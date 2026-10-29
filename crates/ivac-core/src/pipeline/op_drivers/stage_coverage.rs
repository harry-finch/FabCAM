//! Conservative coverage certificate for a flat cutter in one axis-aligned rectangle.
//! A tile is accepted only if all corners lie in ONE convex cutter capsule;
//! this proves its entire area is swept, not merely sampled at control points.
// Exact copied coordinates/axis identity are required; fuzzy equality would widen the proof.
#![allow(clippy::float_cmp)]
use crate::cam::{offsets::PolylineOffset, VcObject};
use crate::geometry::Point2;
use crate::pipeline::CancelToken;
const EPS: f64 = 1e-8;
const MAX_WORK: usize = 5_000_000;

pub(super) fn rectangle(obj: &VcObject) -> Result<[f64; 4], String> {
    // The caller requires exactly one selected contour. Unselected containing
    // geometry (notably the synthetic stock outline) is not part of this
    // operation's region. Actual offsets are independently proven below;
    // selected islands/multiple contours remain refused by the caller.
    if !obj.closed || obj.segments.len() != 4 {
        return Err("Finishing proof currently requires one isolated four-edge rectangle".into());
    }
    let mut b = [
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    for (i, s) in obj.segments.iter().enumerate() {
        if s.bulge != 0.0
            || s.end != obj.segments[(i + 1) % 4].start
            || (s.start.x != s.end.x && s.start.y != s.end.y)
        {
            return Err(
                "Finishing proof requires exact axis-aligned straight rectangle edges".into(),
            );
        }
        b[0] = b[0].min(s.start.x);
        b[1] = b[1].min(s.start.y);
        b[2] = b[2].max(s.start.x);
        b[3] = b[3].max(s.start.y);
    }
    if !(b[2] > b[0] && b[3] > b[1]) {
        return Err("Degenerate finishing target".into());
    }
    for (i, s) in obj.segments.iter().enumerate() {
        if obj.segments[..i].iter().any(|p| p.start == s.start) {
            return Err("Repeated rectangle vertex".into());
        }
        if !((s.start.x == b[0] || s.start.x == b[2]) && (s.start.y == b[1] || s.start.y == b[3])) {
            return Err("Unsupported rectangular target".into());
        }
    }
    Ok(b)
}
fn distance(p: Point2, a: Point2, b: Point2) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let l = dx * dx + dy * dy;
    let t = if l == 0.0 {
        0.0
    } else {
        ((p.x - a.x) * dx + (p.y - a.y) * dy) / l
    }
    .clamp(0.0, 1.0);
    (p.x - a.x - t * dx).hypot(p.y - a.y - t * dy)
}
pub(super) fn prove(
    b: [f64; 4],
    radius: f64,
    offsets: &[PolylineOffset],
    fill: bool,
    cancel: Option<&CancelToken>,
) -> Result<(), String> {
    if !b.iter().all(|v| v.is_finite()) || b[0] >= b[2] || b[1] >= b[3] || radius <= EPS {
        return Err("Finishing cutter cannot reach the retained-stock target".into());
    }
    let segments: Vec<_> = offsets.iter().flat_map(|o| &o.segments).collect();
    if segments.len() > 5000
        || segments.iter().any(|s| {
            s.bulge != 0.0
                || ![s.start.x, s.start.y, s.end.x, s.end.y]
                    .iter()
                    .all(|v| v.is_finite())
        })
    {
        return Err("Finishing path exceeds straight-segment proof budget/representation".into());
    }
    if segments.iter().any(|s| {
        [s.start, s.end]
            .iter()
            .any(|p| p.x < b[0] - EPS || p.x > b[2] + EPS || p.y < b[1] - EPS || p.y > b[3] + EPS)
    }) {
        return Err("Finishing path leaves its cutter-safe target".into());
    }
    let corners = [
        Point2::new(b[0], b[1]),
        Point2::new(b[2], b[1]),
        Point2::new(b[2], b[3]),
        Point2::new(b[0], b[3]),
    ];
    // Each full final wall must be present. This also proves the rounded outer
    // cutter band; the residual sharp corners are outside this cutter-reachable target.
    for i in 0..4 {
        if !segments.iter().any(|s| {
            distance(corners[i], s.start, s.end) <= EPS
                && distance(corners[(i + 1) % 4], s.start, s.end) <= EPS
        }) {
            return Err("Final wall coverage not established".into());
        }
    }
    if !fill {
        return Ok(());
    }
    // Axis-aligned capsule interiors contain closed strips. Their exact slab
    // union handles touching swaths without shrinking the physical cutter or
    // claiming that a finite collection of samples proves coverage.
    let strips: Vec<[f64; 4]> = segments
        .iter()
        .filter_map(|s| {
            if s.start.y == s.end.y {
                Some([
                    s.start.x.min(s.end.x),
                    s.start.y - radius,
                    s.start.x.max(s.end.x),
                    s.start.y + radius,
                ])
            } else if s.start.x == s.end.x {
                Some([
                    s.start.x - radius,
                    s.start.y.min(s.end.y),
                    s.start.x + radius,
                    s.start.y.max(s.end.y),
                ])
            } else {
                None
            }
        })
        .collect();
    let mut xs = vec![b[0], b[2]];
    for s in &strips {
        xs.extend([s[0].clamp(b[0], b[2]), s[2].clamp(b[0], b[2])]);
    }
    xs.sort_by(f64::total_cmp);
    xs.dedup();
    let mut work = 0usize;
    let mut union_covers = true;
    for x in xs.windows(2) {
        if crate::pipeline::cancelled(cancel) {
            return Err("Cancelled finishing proof".into());
        }
        work = work
            .checked_add(strips.len())
            .ok_or("Coverage work overflow")?;
        if work > MAX_WORK {
            return Err("Finishing coverage analysis budget exceeded".into());
        }
        let mut ys: Vec<_> = strips
            .iter()
            .filter(|s| s[0] <= x[0] && s[2] >= x[1])
            .map(|s| (s[1], s[3]))
            .collect();
        ys.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut covered_to = b[1];
        for (lo, hi) in ys {
            if lo > covered_to {
                break;
            }
            covered_to = covered_to.max(hi);
        }
        if covered_to < b[3] {
            union_covers = false;
            break;
        }
    }
    if union_covers {
        return Ok(());
    }
    let mut pending = vec![b];
    while let Some(cell) = pending.pop() {
        if crate::pipeline::cancelled(cancel) {
            return Err("Cancelled finishing proof".into());
        }
        let points = [
            Point2::new(cell[0], cell[1]),
            Point2::new(cell[2], cell[1]),
            Point2::new(cell[2], cell[3]),
            Point2::new(cell[0], cell[3]),
        ];
        let mut covered = false;
        for s in &segments {
            work = work.checked_add(4).ok_or("Coverage work overflow")?;
            if work > MAX_WORK {
                return Err("Finishing coverage analysis budget exceeded".into());
            }
            if points
                .iter()
                .all(|p| distance(*p, s.start, s.end) <= radius - EPS)
            {
                covered = true;
                break;
            }
        }
        if !covered {
            let x = (cell[0] + cell[2]) * 0.5;
            let y = (cell[1] + cell[3]) * 0.5;
            if cell[2] - cell[0] < 1e-6 || cell[3] - cell[1] < 1e-6 || pending.len() > 100_000 {
                return Err("Finishing area coverage is incomplete or ambiguous".into());
            }
            pending.extend([
                [cell[0], cell[1], x, y],
                [x, cell[1], cell[2], y],
                [x, y, cell[2], cell[3]],
                [cell[0], y, x, cell[3]],
            ]);
        }
    }
    Ok(())
}

/// Exact rounded-rectangle boundary for an Outside Profile. Each of the four
/// lines and four quarter circles must appear once; no chord approximations or
/// additional segments are credited. This establishes the final offset contour
/// independently of the offset builder's topology labels.
pub(super) fn prove_outside(
    b: [f64; 4],
    distance: f64,
    offsets: &[PolylineOffset],
) -> Result<(), String> {
    if !distance.is_finite() || distance <= 0.0 || offsets.len() != 1 {
        return Err("Outside finishing requires one complete offset contour".into());
    }
    let [x0, y0, x1, y1] = b;
    let p = [
        Point2::new(x0, y0 - distance),
        Point2::new(x1, y0 - distance),
        Point2::new(x1 + distance, y0),
        Point2::new(x1 + distance, y1),
        Point2::new(x1, y1 + distance),
        Point2::new(x0, y1 + distance),
        Point2::new(x0 - distance, y1),
        Point2::new(x0 - distance, y0),
    ];
    let segments = &offsets[0].segments;
    if segments.len() != 8 {
        return Err("Outside finishing contour has unsupported subdivisions".into());
    }
    let close = |a: Point2, b: Point2| (a.x - b.x).hypot(a.y - b.y) <= EPS;
    let mut seen = [false; 8];
    for (k, s) in segments.iter().enumerate() {
        if !close(s.end, segments[(k + 1) % 8].start) {
            return Err("Outside finishing contour is not continuous".into());
        }
        let mut matched = None;
        for i in 0..8 {
            let expected = if i % 2 == 0 {
                0.0
            } else {
                (std::f64::consts::PI / 8.0).tan()
            };
            if close(s.start, p[i])
                && close(s.end, p[(i + 1) % 8])
                && (s.bulge - expected).abs() <= 1e-12
                || close(s.end, p[i])
                    && close(s.start, p[(i + 1) % 8])
                    && (s.bulge + expected).abs() <= 1e-12
            {
                matched = Some(i);
                break;
            }
        }
        let i = matched.ok_or("Outside finishing does not match the required rounded boundary")?;
        if seen[i] {
            return Err("Repeated outside finishing edge".into());
        }
        seen[i] = true;
    }
    if seen.iter().all(|v| *v) {
        Ok(())
    } else {
        Err("Incomplete outside finishing contour".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Segment;
    fn boundary() -> PolylineOffset {
        let p = [
            Point2::new(0., 0.),
            Point2::new(10., 0.),
            Point2::new(10., 10.),
            Point2::new(0., 10.),
        ];
        PolylineOffset {
            segments: (0..4)
                .map(|i| Segment::line(p[i], p[(i + 1) % 4], "0", 7))
                .collect(),
            closed: true,
            level: 0,
            is_pocket: 0,
            layer: "0".into(),
            color: 7,
            source_object_idx: 0,
            tabs: vec![],
            is_finish: false,
        }
    }
    #[test]
    fn wall_ring_is_not_floor_coverage_and_tangent_strips_are_valid() {
        let mut p = boundary();
        assert!(prove([0., 0., 10., 10.], 1., &[p.clone()], false, None).is_ok());
        assert!(prove([0., 0., 10., 10.], 1., &[p.clone()], true, None).is_err());
        for y in [2., 4., 6., 8.] {
            p.segments.push(Segment::line(
                Point2::new(0., y),
                Point2::new(10., y),
                "0",
                7,
            ));
        }
        assert!(prove([0., 0., 10., 10.], 1., &[p.clone()], true, None).is_ok());
        p.segments[4].start.y = 2.0001;
        p.segments[4].end.y = 2.0001;
        assert!(prove([0., 0., 10., 10.], 1., &[p], true, None).is_err());
    }
    #[test]
    fn budget_and_cancellation_refuse_incomplete_proofs() {
        let mut p = boundary();
        let token = CancelToken::new();
        token.cancel();
        assert!(
            prove([0., 0., 10., 10.], 1., &[p.clone()], true, Some(&token))
                .unwrap_err()
                .contains("Cancelled")
        );
        p.segments = vec![p.segments[0].clone(); 5001];
        assert!(prove([0., 0., 10., 10.], 1., &[p], true, None)
            .unwrap_err()
            .contains("budget"));
    }
}
