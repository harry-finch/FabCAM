//! V-Carve-only continuous piecewise-linear depth scheduling.
//!
//! Each sweep is min(target, previous cut + step, previous cut + travel*tan(angle)).
//! Insert ALL pairwise intersections of these affine functions on each source
//! segment. Therefore interpolation cannot cross above the depth envelope or
//! exceed the per-pass increment between control points. Reverse at chain ends
//! without retracting into an uncleared entry. The first entry ramp is measured
//! from stock top; subsequent sweeps ramp the additional engagement relative to
//! the already-cleared groove (not a new air-to-full-depth plunge). Finish only when the entire final
//! sweep equals the target, otherwise return an explicit undercut error.
//! Halfpipe deliberately continues to use the legacy vcarve_emit module.
#![allow(clippy::cast_precision_loss)]
/// Scheduling comparison error in axial mm, not a physical tolerance.
pub const DEPTH_EPS: f64 = 1e-7;
const MAX_POINTS: usize = 1_000_000;
const MAX_SWEEPS: usize = 4096;
#[derive(Clone, Copy)]
struct Knot {
    x: f64,
    y: f64,
    target: f64,
    cut: f64,
}
impl Knot {
    fn lerp(self, b: Self, t: f64) -> Self {
        Self {
            x: self.x + t * (b.x - self.x),
            y: self.y + t * (b.y - self.y),
            target: self.target + t * (b.target - self.target),
            cut: self.cut + t * (b.cut - self.cut),
        }
    }
}
// Remove only exact affine redundancy. A repeated epsilon merge could accumulate
// target/envelope drift over thousands of passes; do not spend that error budget.
fn simplify(knots: &mut Vec<Knot>) {
    let mut out: Vec<Knot> = Vec::with_capacity(knots.len());
    for &c in knots.iter() {
        while out.len() >= 2 {
            let a = out[out.len() - 2];
            let b = out[out.len() - 1];
            let dx = c.x - a.x;
            let dy = c.y - a.y;
            let t = if dx.abs() >= dy.abs() {
                (b.x - a.x) / dx
            } else {
                (b.y - a.y) / dy
            };
            let p = a.lerp(c, t);
            if !(0.0..=1.0).contains(&t)
                || p.x != b.x
                || p.y != b.y
                || p.target != b.target
                || p.cut != b.cut
            {
                break;
            }
            out.pop();
        }
        out.push(c);
    }
    *knots = out;
}
/// Coordinates are mm, Z is relative to stock top (negative into stock).
/// Returns one continuous path per chain, beginning at stock top. No hidden
/// vertical entry, no extrapolated XY ramp, no surface jump between chains.
pub fn plan(
    axis: &[(f64, f64, f64)],
    step: f64,
    angle: f64,
) -> Result<Vec<(f64, f64, f64)>, String> {
    crate::profile_scope!("vcarve.plan");
    if axis.len() < 2
        || axis.len() > 20_000
        || !step.is_finite()
        || step <= 0.
        || !angle.is_finite()
        || !(0. ..90.).contains(&angle)
        || angle == 0.
    {
        return Err(
            "Invalid V-Carve chain, pass depth or entry angle (must be between 0 and 90 degrees)."
                .into(),
        );
    }
    let mut knots = Vec::with_capacity(axis.len());
    for &(x, y, z) in axis {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() || z > 0. {
            return Err("Invalid nonfinite or above-stock V-Carve target.".into());
        }
        if let Some(prev) = knots.last() {
            let p: &Knot = prev;
            if (x - p.x).hypot(y - p.y) < DEPTH_EPS {
                return Err(
                    "V-Carve chain has a zero-length edge; no qualified ramp entry.".into(),
                );
            }
        }
        knots.push(Knot {
            x,
            y,
            target: -z,
            cut: 0.,
        });
    }
    let slope = angle.to_radians().tan();
    let mut out = vec![(knots[0].x, knots[0].y, 0.)];
    for _ in 0..MAX_SWEEPS {
        let mut traveled = 0.;
        let mut next = Vec::new();
        for pair in knots.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let length = (b.x - a.x).hypot(b.y - a.y);
            let f = [
                (a.target, b.target),
                (a.cut + step, b.cut + step),
                (
                    a.cut + traveled * slope,
                    b.cut + (traveled + length) * slope,
                ),
                (a.cut, b.cut),
            ];
            let mut ts = vec![0., 1.];
            for i in 0..4 {
                for j in i + 1..4 {
                    let d0 = f[i].0 - f[j].0;
                    let d1 = f[i].1 - f[j].1;
                    if d0 != d1 {
                        let t = d0 / (d0 - d1);
                        if t > 0. && t < 1. {
                            ts.push(t);
                        }
                    }
                }
            }
            ts.sort_by(f64::total_cmp);
            ts.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
            for t in ts {
                if t == 0. && !next.is_empty() {
                    continue;
                }
                let mut p = a.lerp(b, t);
                let old = p.cut;
                p.cut = f[..3]
                    .iter()
                    .map(|&(u, v)| u + t * (v - u))
                    .fold(f64::INFINITY, f64::min)
                    .max(old);
                if !p.cut.is_finite()
                    || p.cut > p.target + DEPTH_EPS
                    || p.cut > old + step + DEPTH_EPS
                {
                    return Err("V-Carve overcut: interpolated pass exceeds its target or pass-depth envelope.".into());
                }
                if p.cut < old - DEPTH_EPS {
                    return Err("V-Carve undercut: non-monotone clearing schedule.".into());
                }
                if next.len() >= MAX_POINTS {
                    return Err("V-Carve undercut: sweep knot budget exceeded.".into());
                }
                next.push(p);
            }
            traveled += length;
        }
        // The final sweep visits every interval at its target, not just its
        // deepest sample. PL functions agree between the union of all knots.
        let complete = next.iter().all(|p| (p.cut - p.target).abs() <= DEPTH_EPS);
        out.extend(next.iter().skip(1).map(|p| (p.x, p.y, -p.cut)));
        if out.len() > MAX_POINTS {
            return Err("V-Carve undercut: path budget exceeded before target completion.".into());
        }
        if complete {
            return Ok(out);
        }
        simplify(&mut next);
        next.reverse();
        knots = next;
    }
    Err(
        "V-Carve undercut: entry/pass budget exhausted before the entire target was reached."
            .into(),
    )
}

#[cfg(test)]
mod audit_tests {
    use super::*;
    #[test]
    fn simplification_keeps_target_and_cut_breakpoints() {
        let a = Knot {
            x: 0.0,
            y: 0.0,
            target: 1.0,
            cut: 0.5,
        };
        let c = Knot { x: 2.0, ..a };
        let b = a.lerp(c, 0.5);
        let mut straight = vec![a, b, c];
        simplify(&mut straight);
        assert_eq!(straight.len(), 2);
        for kink in [
            Knot {
                cut: b.cut + 1e-12,
                ..b
            },
            Knot {
                target: b.target + 1e-12,
                ..b
            },
            Knot { y: 1e-12, ..b },
        ] {
            let mut knots = vec![a, kink, c];
            simplify(&mut knots);
            assert_eq!(knots.len(), 3);
        }
    }
    #[test]
    fn many_passes_remain_bounded_and_reach_target() {
        let path = plan(&[(0.0, 0.0, -2.0), (100.0, 0.0, -2.0)], 0.01, 2.0).unwrap();
        assert!(path.len() < 20000, "{}", path.len());
        assert!((path.last().unwrap().2 + 2.0).abs() <= DEPTH_EPS);
        assert!(path.iter().all(|p| p.2 >= -2.0 - DEPTH_EPS));
    }
}
