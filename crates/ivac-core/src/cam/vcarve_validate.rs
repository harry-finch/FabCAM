//! V-Carve-specific input and continuous cutter-envelope checks.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use super::VcObject;
use crate::{Point2, Segment, SegmentKind};
use cavalier_contours::polyline::{PlineSource, PlineSourceMut, Polyline};
/// Sagitta of source arc chords, mm. Separate from axis sample spacing (0.1 mm).
pub const CURVE_ERROR: f64 = 0.002;
/// Polygon/offset predicate allowance, mm (Clipper's existing 1e-4 mm grid).
pub const CLEARANCE_ERROR: f64 = 0.0002;
const TOPOLOGY_EPS: f64 = 1e-7;
/// Clone only the caller's objects; other operations keep their original curves.
pub fn prepared(objects: &[VcObject], selected: &[usize]) -> Result<Vec<VcObject>, String> {
    crate::profile_scope!("vcarve.prepare");
    if selected.is_empty() {
        return Err("Select closed contours for V-Carve; use Engrave for open paths.".into());
    }
    let mut out = objects.to_vec();
    let mut count = 0;
    for &i in selected {
        let o = out
            .get_mut(i)
            .ok_or_else(|| format!("V-Carve selected object index {i} is out of bounds."))?;
        if !o.closed || o.segments.len() < 2 {
            return Err("V-Carve requires closed regions; open paths need Engrave.".into());
        }
        let mut p = Polyline::new_closed();
        let mut lines = Vec::new();
        for (k, s) in o.segments.iter().enumerate() {
            if ![s.start.x, s.start.y, s.end.x, s.end.y, s.bulge]
                .iter()
                .all(|v| v.is_finite() && v.abs() < 1e6)
                || s.start.distance(s.end) < TOPOLOGY_EPS
                || s.end.distance(o.segments[(k + 1) % o.segments.len()].start) > TOPOLOGY_EPS
            {
                return Err(
                    "V-Carve contour contains nonfinite, disconnected, zero-length or out-of-budget geometry (coordinates must be below 1000000 mm in magnitude)."
                        .into(),
                );
            }
            p.add(s.start.x, s.start.y, s.bulge);
            let pts = if s.kind == SegmentKind::Line
                || s.bulge.abs() < 1e-12
                || s.start.distance(s.end) * 0.5 * s.bulge.abs() <= CURVE_ERROR
            {
                vec![s.start, s.end]
            } else {
                let (_, _, _, r) = crate::math::bulge_to_arc(s.start, s.end, s.bulge);
                if !r.is_finite() || r <= 0.0 {
                    return Err("Invalid V-Carve arc radius.".into());
                }
                // Equivalent sagitta bound without cancellation in 1-error/r.
                let max_angle = 4. * (CURVE_ERROR / (2. * r)).min(0.5).sqrt().asin();
                let n = (4. * s.bulge.atan().abs() / max_angle).ceil().max(8.) as usize;
                if n > 10000 {
                    return Err("V-Carve curve exceeds the flattening budget.".into());
                }
                super::segment_to_points(s, n.div_ceil(8))
            };
            for w in pts.windows(2) {
                lines.push(Segment::line(w[0], w[1], s.layer.clone(), s.color));
            }
        }
        let mut intersects = false;
        p.visit_self_intersects(&mut |_| {
            intersects = true;
        });
        if intersects || !p.area().is_finite() || p.area().abs() < TOPOLOGY_EPS {
            return Err("V-Carve requires simple nonzero-area contours; self-intersections are unsupported.".into());
        }
        count += lines.len();
        if count > 10000 {
            return Err("V-Carve exceeds the 10000-edge geometry budget.".into());
        }
        o.segments = lines;
    }
    Ok(out)
}
/// Prove disk clearance throughout a linearly interpolated XYZ segment.
/// Projection onto each boundary edge is affine. Split where its nearest point
/// changes (endpoint/interior). On each interval distance²-radius² is quadratic;
/// checking endpoints and its stationary minimum bounds the ENTIRE interval.
/// This is not an endpoint-only or regularly sampled clearance check.
pub fn check_envelope(
    path: &[(f64, f64, f64)],
    rings: &[&[Point2]],
    tip: f64,
    tan: f64,
) -> Result<(), String> {
    check_envelope_impl(path, rings, tip, tan, true)
}
fn check_envelope_impl(
    path: &[(f64, f64, f64)],
    rings: &[&[Point2]],
    tip: f64,
    tan: f64,
    use_index: bool,
) -> Result<(), String> {
    crate::profile_scope!("vcarve.clearance");
    let edges: Vec<_> = rings
        .iter()
        .flat_map(|r| (0..r.len()).map(|i| (r[i], r[(i + 1) % r.len()])))
        .filter(|(a, b)| a.distance(*b) > TOPOLOGY_EPS)
        .collect();
    if !tip.is_finite()
        || !tan.is_finite()
        || tip < 0.0
        || tan <= 0.0
        || path
            .iter()
            .any(|&(x, y, z)| ![x, y, z].iter().all(|v| v.is_finite()))
        || rings
            .iter()
            .flat_map(|r| r.iter())
            .any(|p| !p.x.is_finite() || !p.y.is_finite())
    {
        return Err("Invalid nonfinite V-Carve clearance input.".into());
    }
    let index = super::spatial::SegmentBoxIndex::new(&edges);
    let mut candidates = Vec::new();
    let mut proofs = 0_usize;
    let mut broad_work = 0_usize;
    for w in path.windows(2) {
        let (x, y, z) = w[0];
        let (bx, by, bz) = w[1];
        let dx = bx - x;
        let dy = by - y;
        let r0 = tip - z * tan - CLEARANCE_ERROR;
        let dr = (z - bz) * tan;
        if ![dx, dy, r0, dr].iter().all(|v| v.is_finite()) {
            return Err("Nonfinite V-Carve envelope coefficients.".into());
        }
        let radius = r0.max(r0 + dr).max(0.0);
        if !radius.is_finite() {
            return Err("Invalid V-Carve envelope radius.".into());
        }
        // Expand the broad phase for coordinate arithmetic roundoff; the
        // narrow phase below retains the original clearance budget/polynomial.
        let pad = 1e-7
            + 64.0 * f64::EPSILON * x.abs().max(y.abs()).max(bx.abs()).max(by.abs()).max(radius);
        if use_index {
            broad_work = broad_work.saturating_add(index.candidates(
                Point2::new(x.min(bx) - radius - pad, y.min(by) - radius - pad),
                Point2::new(x.max(bx) + radius + pad, y.max(by) + radius + pad),
                &mut candidates,
            ));
        } else {
            candidates.clear();
            candidates.extend(0..edges.len());
        }
        if broad_work > 50_000_000 {
            return Err(
                "V-Carve clearance spatial query budget exceeded; simplify the selected geometry."
                    .into(),
            );
        }
        proofs = proofs.saturating_add(candidates.len());
        if proofs > 20_000_000 {
            return Err(
                "V-Carve clearance proof exceeds its budget; simplify the selected geometry."
                    .into(),
            );
        }
        for &edge in &candidates {
            let (a, b) = edges[edge];
            let ex = b.x - a.x;
            let ey = b.y - a.y;
            let len = ex * ex + ey * ey;
            let u = ((x - a.x) * ex + (y - a.y) * ey) / len;
            let du = (dx * ex + dy * ey) / len;
            let mut ts = vec![0., 1.];
            if du != 0. {
                for bound in [0., 1.] {
                    let t = (bound - u) / du;
                    if t > 0. && t < 1. {
                        ts.push(t);
                    }
                }
            }
            if dr != 0. {
                let t = -r0 / dr;
                if t > 0. && t < 1. {
                    ts.push(t);
                }
            }
            ts.sort_by(f64::total_cmp);
            for range in ts.windows(2) {
                let mid = (range[0] + range[1]) * 0.5;
                let projection = u + mid * du;
                let (qx, qy, qdx, qdy) = if projection <= 0. {
                    (x - a.x, y - a.y, dx, dy)
                } else if projection >= 1. {
                    (x - b.x, y - b.y, dx, dy)
                } else {
                    (
                        x - a.x - u * ex,
                        y - a.y - u * ey,
                        dx - du * ex,
                        dy - du * ey,
                    )
                };
                let (rr, rd) = if r0 + mid * dr > 0. {
                    (r0, dr)
                } else {
                    (0., 0.)
                };
                let aa = qdx * qdx + qdy * qdy - rd * rd;
                let bb = 2. * (qx * qdx + qy * qdy - rr * rd);
                let cc = qx * qx + qy * qy - rr * rr;
                let eval = |t: f64| (aa * t + bb) * t + cc;
                let mut minimum = eval(range[0]).min(eval(range[1]));
                if aa > 0. {
                    let t = -bb / (2. * aa);
                    if t > range[0] && t < range[1] {
                        minimum = minimum.min(eval(t));
                    }
                }
                if ![aa, bb, cc, minimum].iter().all(|v| v.is_finite()) {
                    return Err("Nonfinite V-Carve clearance proof.".into());
                }
                if minimum < -1e-9 {
                    return Err("V-Carve overcut: interpolated cutter envelope crosses a region boundary or island. Refine or simplify the geometry; no output generated.".into());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod audit_tests {
    use super::*;
    #[test]
    fn prepared_checks_indices_and_nearly_flat_large_radius_arcs() {
        let points = [
            Point2::new(0.0, 0.0),
            Point2::new(100_000.0, 0.0),
            Point2::new(100_000.0, 10.0),
            Point2::new(0.0, 10.0),
        ];
        let mut segments: Vec<_> = (0..4)
            .map(|i| Segment::line(points[i], points[(i + 1) % 4], "0", 7))
            .collect();
        segments[0] = Segment::arc(points[0], points[1], 1e-12, None, "0", 7);
        let objects = vec![VcObject::new(segments, true)];
        assert!(prepared(&objects, &[1])
            .unwrap_err()
            .contains("out of bounds"));
        assert_eq!(prepared(&objects, &[0]).unwrap()[0].segments.len(), 4);
    }
    #[test]
    fn broad_phase_agrees_with_all_edge_analytical_proof() {
        let outer = [
            Point2::new(-20.0, -20.0),
            Point2::new(20.0, -20.0),
            Point2::new(20.0, 20.0),
            Point2::new(-20.0, 20.0),
        ];
        let island = [
            Point2::new(-1.0, -1.0),
            Point2::new(1.0, -1.0),
            Point2::new(1.0, 1.0),
            Point2::new(-1.0, 1.0),
        ];
        for i in -25..25 {
            for depth in [0.0, 0.5, 2.0, 5.0] {
                let path = [
                    (-24.0, f64::from(i), -depth),
                    (24.0, f64::from(i) + 0.3, -depth * 0.4),
                ];
                assert_eq!(
                    check_envelope_impl(&path, &[&outer, &island], 0.1, 0.7, true),
                    check_envelope_impl(&path, &[&outer, &island], 0.1, 0.7, false)
                );
            }
        }
    }
    #[test]
    fn distant_edges_do_not_consume_the_analytical_pair_budget() {
        let ring: Vec<_> = (0..5000)
            .map(|i| {
                let a = f64::from(i) * std::f64::consts::TAU / 5000.0;
                Point2::new(100.0 * a.cos(), 100.0 * a.sin())
            })
            .collect();
        let path: Vec<_> = (0..5000)
            .map(|i| (f64::from(i) * 0.0001, 0.0, -0.5))
            .collect();
        assert!(check_envelope(&path, &[&ring], 0.0, 1.0).is_ok());
    }
}
