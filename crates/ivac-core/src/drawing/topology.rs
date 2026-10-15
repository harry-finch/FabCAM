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
            if a.kind != SegmentKind::Line || b.kind != SegmentKind::Line {
                // Preserve untouched curves, but do not claim a line/arc
                // intersection proof where their exact bounds overlap.
                return Err("Overlapping curve bounds: repair topology not analyzed".into());
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
            let same = e.membership[i] == e.membership[j];
            let members: Vec<_> = e
                .membership
                .iter()
                .enumerate()
                .filter_map(|(k, id)| (*id == e.membership[i]).then_some(k))
                .collect();
            let adjacent = i.abs_diff(j) == 1
                || (members.first() == Some(&i.min(j)) && members.last() == Some(&i.max(j)));
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

/// Independent point-in-polygon parity witnesses protect entire small islands
/// that could fit in the sliver removed by a simplification without crossing it.
pub fn preserve_containment(before: &Drawing, after: &Drawing) -> Result<(), String> {
    fn inside(p: Point2, edges: &[crate::geometry::Segment]) -> Result<bool, String> {
        let mut result = false;
        for e in edges {
            if (e.start.y > p.y) != (e.end.y > p.y) {
                let side = orient(e.start, e.end, p)?;
                if (side > 0) == (e.end.y > e.start.y) {
                    result = !result;
                }
            }
        }
        Ok(result)
    }
    let a = before.evaluate()?;
    let b = after.evaluate()?;
    if a.segments.iter().any(|s| s.kind != SegmentKind::Line) {
        return Err("Simplification containment with curves/points is not analyzed".into());
    }
    if a.contours != b.contours || a.closed != b.closed {
        return Err("Simplification cannot change contour topology".into());
    }
    for (i, id) in a.contours.iter().enumerate() {
        if !a.closed[i] {
            continue;
        }
        let old: Vec<_> = a
            .segments
            .iter()
            .zip(&a.membership)
            .filter_map(|(s, c)| (c == id).then_some(s.clone()))
            .collect();
        let new: Vec<_> = b
            .segments
            .iter()
            .zip(&b.membership)
            .filter_map(|(s, c)| (c == id).then_some(s.clone()))
            .collect();
        for other in &a.contours {
            if other == id {
                continue;
            }
            let p = a.segments[a.membership.iter().position(|c| c == other).unwrap()].start;
            let q = b.segments[b.membership.iter().position(|c| c == other).unwrap()].start;
            if inside(p, &old)? != inside(q, &new)? {
                return Err("Simplification would change contour containment".into());
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
