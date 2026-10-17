//! Conservative line topology validator. Uncertain predicates refuse a repair.
use super::Drawing;
use crate::geometry::{Point2, SegmentKind};

pub(super) fn orient(a: Point2, b: Point2, c: Point2) -> Result<i8, String> {
    let u = (b.x - a.x) * (c.y - a.y);
    let v = (b.y - a.y) * (c.x - a.x);
    let cross = u - v;
    if a == c || b == c || (a.x == b.x && b.x == c.x) || (a.y == b.y && b.y == c.y) {
        return Ok(0);
    }
    // Roundoff guard for the two products and coordinate subtractions. This
    // is numerical predicate uncertainty, never a geometric repair tolerance.
    let bound = 32. * f64::EPSILON * (u.abs() + v.abs());
    if !cross.is_finite() || cross.abs() <= bound {
        // Exact dyadic representation: every finite f64 multiplied by 2^1074
        // is an integer. This fallback adds no geometric epsilon or rounding.
        fn integer(v: f64) -> num_bigint::BigInt {
            let bits = v.to_bits();
            let exponent = ((bits >> 52) & 0x7ff) as usize;
            let fraction = bits & ((1u64 << 52) - 1);
            let mantissa = if exponent == 0 {
                fraction
            } else {
                fraction | (1u64 << 52)
            };
            let n = num_bigint::BigInt::from(mantissa) << exponent.saturating_sub(1);
            if bits >> 63 == 1 {
                -n
            } else {
                n
            }
        }
        if ![a.x, a.y, b.x, b.y, c.x, c.y].iter().all(|n| n.is_finite()) {
            return Err("Nonfinite topology input".into());
        }
        let ax = integer(a.x);
        let ay = integer(a.y);
        let value = (integer(b.x) - &ax) * (integer(c.y) - &ay)
            - (integer(b.y) - &ay) * (integer(c.x) - &ax);
        return Ok(match value.sign() {
            num_bigint::Sign::Minus => -1,
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => 1,
        });
    }
    Ok(if cross > 0. { 1 } else { -1 })
}
fn overlap(a: f64, b: f64, c: f64, d: f64) -> bool {
    a.min(b) <= c.max(d) && c.min(d) <= a.max(b)
}
fn on(a: Point2, b: Point2, p: Point2) -> bool {
    overlap(a.x, b.x, p.x, p.x) && overlap(a.y, b.y, p.y, p.y)
}
/// Check complete evaluated drawing. Unsupported curves are explicit, never
/// treated as empty obstacles. A fixed work budget bounds pathological inputs.
pub fn validate(d: &Drawing) -> Result<(), String> {
    let e = d.evaluate()?;
    if e.segments.len() > 2000 {
        return Err("Topology analysis budget exceeded (2000 segments); no repair accepted".into());
    }
    for s in &e.segments {
        if matches!(s.kind, SegmentKind::Arc | SegmentKind::Circle) {
            let v = cavalier_contours::polyline::PlineVertex::new(s.start.x, s.start.y, s.bulge);
            if s.start == s.end || s.bulge == 0.0 || v.bulge_is_zero() {
                return Err("Degenerate or near-flat curve intersection is not qualified".into());
            }
            let (center, radius, _, _) = crate::math::bulge_to_arc(s.start, s.end, s.bulge);
            if !center.x.is_finite()
                || !center.y.is_finite()
                || !(radius * radius).is_finite()
                || s.center
                    .is_some_and(|c| c.distance(center) > CURVE_INCIDENCE_MM)
            {
                return Err("Inconsistent or numerically unbounded curve geometry".into());
            }
        }
        if s.kind != SegmentKind::Line {
            continue;
        }
        if s.start == s.end {
            return Err("Repair would retain a zero-length line".into());
        }
    }
    // Sort on minimum X to prune separated pairs; bounded even on dense overlaps.
    let bounds: Vec<_> = e
        .segments
        .iter()
        .map(|s| crate::geometry::BBox::from_segments(std::slice::from_ref(s)))
        .collect();
    let mut order: Vec<_> = (0..e.segments.len()).collect();
    order.sort_by(|&a, &b| bounds[a].min_x.total_cmp(&bounds[b].min_x));
    for (oi, &i) in order.iter().enumerate() {
        let a = &e.segments[i];
        for &j in &order[oi + 1..] {
            let b = &e.segments[j];
            if bounds[j].min_x > bounds[i].max_x {
                break;
            }
            if !overlap(
                bounds[i].min_y,
                bounds[i].max_y,
                bounds[j].min_y,
                bounds[j].max_y,
            ) {
                continue;
            }
            if a.kind == SegmentKind::Point || b.kind == SegmentKind::Point {
                continue;
            }
            let same = e.membership[i] == e.membership[j];
            let adjacent = same
                && (i.abs_diff(j) == 1
                    || (e.membership[..i.min(j)].last() != Some(&e.membership[i])
                        && e.membership.get(i.max(j) + 1) != Some(&e.membership[i])));
            if a.kind != SegmentKind::Line || b.kind != SegmentKind::Line {
                validate_curve_pair(a, b, adjacent)?;
                continue;
            }
            let o1 = orient(a.start, a.end, b.start)?;
            let o2 = orient(a.start, a.end, b.end)?;
            let o3 = orient(b.start, b.end, a.start)?;
            let o4 = orient(b.start, b.end, a.end)?;
            let intersects = (o1 * o2 < 0 && o3 * o4 < 0)
                || (o1 == 0 && on(a.start, a.end, b.start))
                || (o2 == 0 && on(a.start, a.end, b.end))
                || (o3 == 0 && on(b.start, b.end, a.start))
                || (o4 == 0 && on(b.start, b.end, a.end));
            if !intersects {
                continue;
            }
            let shared = [a.start, a.end]
                .iter()
                .any(|p| *p == b.start || *p == b.end);
            if same && shared && adjacent {
                // Two straight segments sharing only their end are allowed;
                // collinear backtracking/overlap is not.
                if o1 != 0 || o2 != 0 {
                    continue;
                }
                let positive_x = a.start.x.max(a.end.x).min(b.start.x.max(b.end.x))
                    - a.start.x.min(a.end.x).max(b.start.x.min(b.end.x));
                let positive_y = a.start.y.max(a.end.y).min(b.start.y.max(b.end.y))
                    - a.start.y.min(a.end.y).max(b.start.y.min(b.end.y));
                if positive_x <= 0. && positive_y <= 0. {
                    continue;
                }
            }
            return Err("Crossing, overlap or touching contours require explicit review".into());
        }
    }
    Ok(())
}

/// Numerical intersection/endpoint incidence tolerance in drawing mm, not a
/// user repair tolerance. Does not weld endpoints or change exact closure.
const CURVE_INCIDENCE_MM: f64 = 1e-9;
fn validate_curve_pair(
    a: &crate::Segment,
    b: &crate::Segment,
    adjacent: bool,
) -> Result<(), String> {
    use cavalier_contours::polyline::{pline_seg_intr, PlineSegIntr, PlineVertex};
    let vertex = |s: &crate::Segment| PlineVertex::new(s.start.x, s.start.y, s.bulge);
    for s in [a, b] {
        if s.kind != SegmentKind::Line
            && (s.start == s.end || s.bulge == 0. || vertex(s).bulge_is_zero())
        {
            return Err("Degenerate or near-flat curve intersection is not qualified".into());
        }
    }
    let intr = pline_seg_intr(
        vertex(a),
        PlineVertex::new(a.end.x, a.end.y, 0.),
        vertex(b),
        PlineVertex::new(b.end.x, b.end.y, 0.),
        CURVE_INCIDENCE_MM,
    );
    let endpoint = |q: cavalier_contours::core::math::Vector2<f64>| {
        let q = Point2::new(q.x, q.y);
        adjacent
            && [a.start, a.end]
                .iter()
                .any(|p| p.distance(q) <= CURVE_INCIDENCE_MM)
            && [b.start, b.end]
                .iter()
                .any(|p| p.distance(q) <= CURVE_INCIDENCE_MM)
    };
    let safe = match intr {
        PlineSegIntr::NoIntersect => true,
        PlineSegIntr::OneIntersect { point } | PlineSegIntr::TangentIntersect { point } => {
            endpoint(point)
        }
        PlineSegIntr::TwoIntersects { point1, point2 } => endpoint(point1) && endpoint(point2),
        PlineSegIntr::OverlappingArcs { .. } | PlineSegIntr::OverlappingLines { .. } => false,
    };
    if safe {
        Ok(())
    } else {
        Err("Crossing, overlap or touching curves require explicit review".into())
    }
}

/// Independent point-in-polygon parity witnesses protect entire small islands
/// that could fit in the sliver removed by a simplification without crossing it.
pub fn preserve_containment(before: &Drawing, after: &Drawing) -> Result<(), String> {
    fn inside(p: Point2, edges: &[crate::geometry::Segment]) -> Result<bool, String> {
        let mut result = false;
        for e in edges {
            let side = orient(e.start, e.end, p)?;
            // Only the finite edge is ambiguous, not its infinite support line.
            if side == 0 && on(e.start, e.end, p) {
                return Err("Point lies directly on contour edge; containment ambiguous".into());
            }
            if (e.start.y > p.y) != (e.end.y > p.y) && (side > 0) == (e.end.y > e.start.y) {
                result = !result;
            }
        }
        Ok(result)
    }
    let a = before.evaluate()?;
    let b = after.evaluate()?;
    if a.contours != b.contours {
        return Err("Simplification cannot change contour identities".into());
    }
    for id in &a.contours {
        let edges = |e: &super::Evaluated| {
            e.segments
                .iter()
                .zip(&e.membership)
                .filter_map(|(s, c)| (c == id).then_some(s.clone()))
                .collect::<Vec<_>>()
        };
        let mut old = edges(&a);
        let mut new = edges(&b);
        if old == new {
            continue;
        }
        if old.iter().chain(&new).any(|s| s.kind != SegmentKind::Line) {
            return Err("Simplification of curved contours is not analyzed".into());
        }
        // An open chain gets a virtual closing chord for this proof only.
        // Its endpoints do not move during simplification. This protects the
        // swept sliver (including drill points) even for microscopic gaps,
        // without declaring the drawing closed or modifying its geometry.
        for ring in [&mut old, &mut new] {
            let first = ring.first().ok_or("Empty simplification contour")?.start;
            let last = ring.last().ok_or("Empty simplification contour")?.end;
            if first != last {
                ring.push(crate::Segment::line(last, first, "proof", 7));
            }
        }
        // All surviving witnesses, including points and arc endpoints; final
        // topology validation also rejects crossing any other curve/line.
        for ((p, owner), (q, next_owner)) in a
            .segments
            .iter()
            .zip(&a.membership)
            .filter(|(_, owner)| *owner != id)
            .zip(
                b.segments
                    .iter()
                    .zip(&b.membership)
                    .filter(|(_, owner)| *owner != id),
            )
        {
            if owner != next_owner || p != q {
                return Err("Unrelated geometry changed during simplification proof".into());
            }
            for (p, q) in [(p.start, q.start), (p.end, q.end)] {
                if inside(p, &old)? != inside(q, &new)? {
                    return Err("Simplification would change contour containment".into());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_fallback_resolves_cancelled_products() {
        let n = 134_217_728.;
        assert_eq!(
            orient(
                Point2::new(0., 0.),
                Point2::new(n, n - 1.),
                Point2::new(n - 1., n - 2.)
            )
            .unwrap(),
            -1
        );
        assert_eq!(
            orient(
                Point2::new(0., 0.),
                Point2::new(1., 1.),
                Point2::new(2., 2.)
            )
            .unwrap(),
            0
        );
        assert_eq!(
            orient(
                Point2::new(0., 0.),
                Point2::new(1e-200, 0.),
                Point2::new(0., 1e-200)
            )
            .unwrap(),
            1
        );
    }
}
