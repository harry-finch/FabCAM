//! Independent TEST1 `OpenSBP` and continuous straight-segment oracle.
// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::cast_possible_truncation, clippy::float_cmp)]
use ivac_core::geometry::{Point2, Segment};
#[derive(Clone)]
pub struct Move {
    pub a: [f64; 3],
    pub b: [f64; 3],
    pub rapid: bool,
    pub feed: f64,
    pub line: u32,
    pub op_id: u32,
}
pub fn ring(points: &[(f64, f64)]) -> Vec<Segment> {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(a, b)| Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "0", 7))
        .collect()
}
pub fn decode(text: &str) -> Result<Vec<Move>, String> {
    let mut p = [None; 3];
    let mut feed = 0.;
    let mut out = vec![];
    let mut op_id = 0;
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("' OP ") {
            op_id = value.parse().map_err(|_| "invalid_op_id")?;
        }
        if line.is_empty() || line.starts_with('\'') {
            continue;
        }
        let f: Vec<_> = line.split(',').map(str::trim).collect();
        let number = |n: usize| -> Result<f64, String> {
            let v: f64 = f
                .get(n)
                .ok_or("missing_axis")?
                .parse()
                .map_err(|_| "invalid_number")?;
            if !v.is_finite() {
                return Err("nonfinite_output".into());
            }
            Ok(v)
        };
        let old = p;
        match f[0] {
            "MS" => {
                feed = number(1)? * 60.;
                continue;
            }
            "JZ" => p[2] = Some(number(1)?),
            "J2" => {
                p[0] = Some(number(1)?);
                p[1] = Some(number(2)?);
            }
            "J3" | "M3" => {
                for (j, axis) in p.iter_mut().enumerate() {
                    *axis = Some(number(j + 1)?);
                }
            }
            "SA" | "VD" | "END" => continue,
            s if s.starts_with("PAUSE") => continue,
            _ => return Err(format!("unsupported_output:{line}")),
        }
        if let ([Some(x), Some(y), Some(z)], [Some(u), Some(v), Some(w)]) = (old, p) {
            if [x, y, z] != [u, v, w] {
                out.push(Move {
                    a: [x, y, z],
                    b: [u, v, w],
                    rapid: f[0].starts_with('J'),
                    feed,
                    line: i as u32 + 1,
                    op_id,
                });
            }
        }
    }
    Ok(out)
}
pub fn segment_distance(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1]];
    let n = d[0] * d[0] + d[1] * d[1];
    let t = if n == 0. {
        0.
    } else {
        ((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / n
    }
    .clamp(0., 1.);
    (p[0] - a[0] - t * d[0]).hypot(p[1] - a[1] - t * d[1])
}
pub fn rect_edges(lo: [f64; 2], hi: [f64; 2]) -> Vec<([f64; 2], [f64; 2])> {
    let p = [lo, [hi[0], lo[1]], hi, [lo[0], hi[1]]];
    (0..4).map(|i| (p[i], p[(i + 1) % 4])).collect()
}
pub fn clear_edges(
    m: &Move,
    edges: &[([f64; 2], [f64; 2])],
    r0: f64,
    r1: f64,
    eps: f64,
) -> Result<(), String> {
    let velocity = [m.b[0] - m.a[0], m.b[1] - m.a[1]];
    let dr = r1 - r0;
    for (a, b) in edges {
        let e = [b[0] - a[0], b[1] - a[1]];
        let n = e[0] * e[0] + e[1] * e[1];
        if n == 0. {
            return Err("degenerate_oracle_edge".into());
        }
        let u = ((m.a[0] - a[0]) * e[0] + (m.a[1] - a[1]) * e[1]) / n;
        let du = (velocity[0] * e[0] + velocity[1] * e[1]) / n;
        let mut cuts = vec![0., 1.];
        if du != 0. {
            for v in [0., 1.] {
                let t = (v - u) / du;
                if t > 0. && t < 1. {
                    cuts.push(t);
                }
            }
        }
        cuts.sort_by(f64::total_cmp);
        for interval in cuts.windows(2) {
            let mid = (interval[0] + interval[1]) / 2.;
            let projection = u + du * mid;
            let (q, d) = if projection <= 0. {
                ([m.a[0] - a[0], m.a[1] - a[1]], velocity)
            } else if projection >= 1. {
                ([m.a[0] - b[0], m.a[1] - b[1]], velocity)
            } else {
                (
                    [m.a[0] - a[0] - u * e[0], m.a[1] - a[1] - u * e[1]],
                    [velocity[0] - du * e[0], velocity[1] - du * e[1]],
                )
            };
            let rr = (r0 - eps).max(0.);
            let aa = d[0] * d[0] + d[1] * d[1] - dr * dr;
            let bb = 2. * (q[0] * d[0] + q[1] * d[1] - rr * dr);
            let cc = q[0] * q[0] + q[1] * q[1] - rr * rr;
            let mut times = vec![interval[0], interval[1]];
            if aa > 0. {
                let t = -bb / (2. * aa);
                if t > interval[0] && t < interval[1] {
                    times.push(t);
                }
            }
            for t in times {
                if (aa * t + bb) * t + cc < -1e-9 {
                    return Err(format!("continuous_clearance:line={}:t={t}", m.line));
                }
            }
        }
    }
    Ok(())
}
#[test]
fn detects_interior_collision() {
    let m = Move {
        a: [-2., 0., -1.],
        b: [2., 0., -1.],
        rapid: false,
        feed: 1.,
        line: 1,
        op_id: 1,
    };
    assert!(clear_edges(&m, &[([0., -1.], [0., 1.])], 0.5, 0.5, 0.).is_err());
}
#[test]
fn accepts_tangent() {
    let m = Move {
        a: [-2., 2., -1.],
        b: [2., 2., -1.],
        rapid: false,
        feed: 1.,
        line: 1,
        op_id: 1,
    };
    assert!(clear_edges(&m, &[([-4., 1.], [4., 1.])], 1., 1., 0.).is_ok());
}

/// Restrict to the material portion of a feed, including crossing entries.
pub fn below_surface(m: &Move) -> Option<Move> {
    if m.rapid || (m.a[2] > 0. && m.b[2] > 0.) {
        return None;
    }
    let mut cut = m.clone();
    if m.a[2] > 0. || m.b[2] > 0. {
        let t = -m.a[2] / (m.b[2] - m.a[2]);
        let at = std::array::from_fn(|i| m.a[i] + t * (m.b[i] - m.a[i]));
        if m.a[2] > 0. {
            cut.a = at;
        } else {
            cut.b = at;
        }
    }
    Some(cut)
}

#[test]
fn decoder_preserves_unknown_initial_axes_and_rejects_unknown_motion() {
    let moves = decode("JZ,5\nJ2,10,20\nMS,2,2\nM3,10,20,-1\n").unwrap();
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].a, [10., 20., 5.]);
    assert_eq!(moves[0].feed, 120.);
    assert!(decode("M4,10,20,30").is_err());
    assert!(decode("JZ,NaN").is_err());
}
#[test]
fn variable_radius_and_crossing_entry_are_checked() {
    let m = Move {
        a: [0., 2., 1.],
        b: [0., 2., -3.],
        rapid: false,
        feed: 10.,
        line: 1,
        op_id: 1,
    };
    let clipped = below_surface(&m).unwrap();
    assert_eq!(clipped.a, [0., 2., 0.]);
    assert!(clear_edges(&clipped, &[([-1., 0.], [1., 0.])], 0., 3., 0.).is_err());
    assert!(clear_edges(&clipped, &[([-1., 0.], [1., 0.])], 0., 1., 0.).is_ok());
}
