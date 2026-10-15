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
    if selected.is_empty() {
        return Err("Select closed contours for V-Carve; use Engrave for open paths.".into());
    }
    let mut out = objects.to_vec();
    let mut count = 0;
    for &i in selected {
        let o = &mut out[i];
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
                    "V-Carve contour contains nonfinite, disconnected or zero-length geometry."
                        .into(),
                );
            }
            p.add(s.start.x, s.start.y, s.bulge);
            let pts = if s.kind == SegmentKind::Line || s.bulge.abs() < 1e-12 {
                vec![s.start, s.end]
            } else {
                let (_, _, _, r) = crate::math::bulge_to_arc(s.start, s.end, s.bulge);
                let max_angle = 2. * (1. - (CURVE_ERROR / r).min(1.)).acos();
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
    let edges: Vec<_> = rings
        .iter()
        .flat_map(|r| (0..r.len()).map(|i| (r[i], r[(i + 1) % r.len()])))
        .filter(|(a, b)| a.distance(*b) > TOPOLOGY_EPS)
        .collect();
    if path.len().saturating_mul(edges.len()) > 20_000_000 {
        return Err(
            "V-Carve clearance proof exceeds its budget; simplify the selected geometry.".into(),
        );
    }
    for w in path.windows(2) {
        let (x, y, z) = w[0];
        let (bx, by, bz) = w[1];
        let dx = bx - x;
        let dy = by - y;
        let r0 = tip - z * tan - CLEARANCE_ERROR;
        let dr = (z - bz) * tan;
        for &(a, b) in &edges {
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
                if minimum < -1e-9 {
                    return Err("V-Carve overcut: interpolated cutter envelope crosses a region boundary or island. Refine or simplify the geometry; no output generated.".into());
                }
            }
        }
    }
    Ok(())
}
