//! Material-width tab envelopes. Width describes a retained disc around the
//! projected tab anchor; its Minkowski sum with the cutter disc is protected.
//! Rectangle transitions are analytic line/circle intersections. Ramp emission
//! uses an upper envelope over entire intervals (not just endpoint samples).
use super::*;
use crate::cam::offsets::TabPoint;

// Endpoint uplift budget in mm. Total interpolated excess is <= twice this
// budget before XY/rounding effects; it never permits cutting below the bridge.
const RAMP_EXCESS_MM: f64 = 0.005;
// Covers the current 0.01 mm path linearization budget plus 0.0001 mm
// coordinate rounding. This enlarges retention; it never relaxes XY fitting.

fn point(seg: &Segment, t: f64) -> Point2 {
    if t <= 0. {
        return seg.start;
    }
    if t >= 1. {
        return seg.end;
    }
    if matches!(seg.kind, SegmentKind::Arc | SegmentKind::Circle) && seg.bulge != 0. {
        let (c, _, _, r) = math::bulge_to_arc(seg.start, seg.end, seg.bulge);
        let a = (seg.start.y - c.y).atan2(seg.start.x - c.x) + t * 4. * seg.bulge.atan();
        Point2::new(c.x + r * a.cos(), c.y + r * a.sin())
    } else {
        Point2::new(
            seg.start.x + t * (seg.end.x - seg.start.x),
            seg.start.y + t * (seg.end.y - seg.start.y),
        )
    }
}

fn length(seg: &Segment) -> f64 {
    if seg.bulge != 0. {
        let (_, _, _, r) = math::bulge_to_arc(seg.start, seg.end, seg.bulge);
        r * (4. * seg.bulge.atan()).abs()
    } else {
        seg.start.distance(seg.end)
    }
}

fn crossings(seg: &Segment, anchor: Point2, radius: f64, breaks: &mut Vec<f64>) {
    if seg.bulge == 0. {
        let dx = seg.end.x - seg.start.x;
        let dy = seg.end.y - seg.start.y;
        let a = dx * dx + dy * dy;
        if a == 0. {
            return;
        }
        let x = seg.start.x - anchor.x;
        let y = seg.start.y - anchor.y;
        let b = x * dx + y * dy;
        let disc = b * b - a * (x * x + y * y - radius * radius);
        if disc >= 0. {
            for t in [(-b - disc.sqrt()) / a, (-b + disc.sqrt()) / a] {
                if t > 0. && t < 1. {
                    breaks.push(t);
                }
            }
        }
    } else {
        let (c, _, _, r) = math::bulge_to_arc(seg.start, seg.end, seg.bulge);
        let d = c.distance(anchor);
        if d == 0. || r == 0. {
            return;
        }
        let cosine = (d * d + r * r - radius * radius) / (2. * d * r);
        if !(-1. ..=1.).contains(&cosine) {
            return;
        }
        let phi = (anchor.y - c.y).atan2(anchor.x - c.x);
        let alpha = cosine.acos();
        let start = (seg.start.y - c.y).atan2(seg.start.x - c.x);
        let sweep = 4. * seg.bulge.atan();
        for a in [phi - alpha, phi + alpha] {
            let delta = if sweep > 0. {
                (a - start).rem_euclid(std::f64::consts::TAU)
            } else {
                (start - a).rem_euclid(std::f64::consts::TAU)
            };
            let t = delta / sweep.abs();
            if t > 0. && t < 1. {
                breaks.push(t);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn emit<P: PostProcessor>(
    segments: &[Segment],
    tabs: &[TabPoint],
    tabs_z: f64,
    cut_z: f64,
    width: f64,
    tool_radius: f64,
    ramp: Option<f64>,
    rate_v: u32,
    rate_h: u32,
    post: &mut P,
) {
    let tool_radius = tool_radius + RETENTION_MARGIN_MM;
    let slope = ramp.map(|a| a.to_radians().tan());
    let lift = (tabs_z - cut_z).abs();
    let height = |p: Point2, margin: f64| {
        tabs.iter()
            .filter(|t| t.radius(width) > 0.)
            .fold(cut_z, |z, t| {
                let top = t.lift(lift);
                let distance = (p.x - t.x).hypot(p.y - t.y);
                let radius = t.radius(width) + tool_radius;
                let h = match slope {
                    Some(s) => (top - s * (distance - radius).max(0.) + margin).clamp(0., top),
                    None => {
                        if distance <= radius {
                            top
                        } else {
                            0.
                        }
                    }
                };
                z.max(cut_z + h)
            })
    };
    for seg in segments {
        if matches!(seg.kind, SegmentKind::Point) {
            continue;
        }
        let mut breaks = vec![0., 1.];
        for tab in tabs.iter().filter(|t| t.radius(width) > 0.) {
            let c = Point2::new(tab.x, tab.y);
            let r = tab.radius(width) + tool_radius;
            crossings(seg, c, r, &mut breaks);
            if let Some(s) = slope {
                crossings(seg, c, r + tab.lift(lift) / s, &mut breaks);
            }
        }
        breaks.sort_by(f64::total_cmp);
        breaks.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
        for pair in breaks.windows(2) {
            let lo = pair[0];
            let hi = pair[1];
            let len = length(seg) * (hi - lo);
            let mid = height(point(seg, (lo + hi) * 0.5), 0.);
            let varying = slope.is_some()
                && tabs.iter().any(|tab| {
                    let p = point(seg, (lo + hi) * 0.5);
                    let d = (p.x - tab.x).hypot(p.y - tab.y);
                    let r = tab.radius(width) + tool_radius;
                    d > r && d < r + tab.lift(lift) / slope.unwrap()
                });
            let mut count = if varying {
                (len * slope.unwrap() / (2. * RAMP_EXCESS_MM))
                    .ceil()
                    .max(1.) as usize
            } else {
                1
            };
            if varying && seg.bulge != 0. {
                let (_, _, _, radius) = math::bulge_to_arc(seg.start, seg.end, seg.bulge);
                let max_angle = 2. * (1. - (0.005 / radius).min(1.)).acos();
                count = count
                    .max(((hi - lo) * (4. * seg.bulge.atan()).abs() / max_angle).ceil() as usize);
            }
            for k in 0..count {
                let a = lo + (hi - lo) * k as f64 / count as f64;
                let b = lo + (hi - lo) * (k + 1) as f64 / count as f64;
                let p = point(seg, a);
                let q = point(seg, b);
                // A slope-L Lipschitz height field is bounded above by linear
                // interpolation of endpoints raised by L*arc_length/2.
                let margin = if varying {
                    slope.unwrap() * len / count as f64 * 0.5
                } else {
                    0.
                };
                let z0 = if varying { height(p, margin) } else { mid };
                let z1 = if varying { height(q, margin) } else { mid };
                post.feedrate(rate_v);
                post.linear(None, None, Some(z0));
                post.feedrate(rate_h);
                if seg.bulge == 0. || varying {
                    post.linear(Some(q.x), Some(q.y), Some(z1));
                } else {
                    let (c, _, _, _) = math::bulge_to_arc(seg.start, seg.end, seg.bulge);
                    if seg.bulge > 0. {
                        post.arc_ccw(
                            Some(q.x),
                            Some(q.y),
                            Some(z1),
                            Some(c.x - p.x),
                            Some(c.y - p.y),
                        );
                    } else {
                        post.arc_cw(
                            Some(q.x),
                            Some(q.y),
                            Some(z1),
                            Some(c.x - p.x),
                            Some(c.y - p.y),
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod audit_11_12 {
    use super::*;
    #[test]
    fn retained_width_survives_cutter_sweep_for_rectangle_and_ramp() {
        for ramp in [None, Some(30.)] {
            for reverse in [false, true] {
                let (a, b) = if reverse { (20., -20.) } else { (-20., 20.) };
                let seg = Segment::line(Point2::new(a, 0.), Point2::new(b, 0.), "0", 0);
                let tab = TabPoint {
                    x: 0.,
                    y: 0.,
                    width_override_mm: Some(6.),
                    height_override_mm: Some(1.),
                };
                let mut post = crate::gcode::linuxcnc::Post::new();
                post.move_to(Some(a), Some(0.), Some(-5.));
                emit(
                    &[seg],
                    &[tab],
                    -4.,
                    -5.,
                    10.,
                    3.,
                    ramp,
                    100,
                    1000,
                    &mut post,
                );
                let moves = crate::gcode::preview::interpret(&post.finish());
                for m in moves
                    .iter()
                    .filter(|m| m.kind != crate::gcode::preview::MoveKind::Rapid)
                {
                    for k in 0..=20 {
                        let t = k as f64 / 20.;
                        let x = m.from.x + t * (m.to.x - m.from.x);
                        let z = m.from.z + t * (m.to.z - m.from.z);
                        // A radius-3 cutter can touch retained x in [-3,3]
                        // exactly when its centre lies inside [-6,6].
                        if x.abs() < 6. - 1e-5 {
                            assert!(z >= -4.0001, "{ramp:?}: x={x} z={z}");
                        }
                    }
                }
                assert!(moves.iter().any(|m| m.to.x.abs() > 10. && m.to.z < -4.9));
            }
        }
    }
    #[test]
    fn circle_tabs_split_arcs_and_reach_final_floor() {
        let a = Point2::new(100., 0.);
        let b = Point2::new(-100., 0.);
        let c = Point2::new(0., 0.);
        let segs = [
            Segment::arc(a, b, 1., Some(c), "0", 0),
            Segment::arc(b, a, 1., Some(c), "0", 0),
        ];
        let tabs = [
            TabPoint {
                x: 0.,
                y: 100.,
                width_override_mm: None,
                height_override_mm: None,
            },
            TabPoint {
                x: 0.,
                y: -100.,
                width_override_mm: None,
                height_override_mm: None,
            },
        ];
        let mut post = crate::gcode::linuxcnc::Post::new();
        post.move_to(Some(100.), Some(0.), Some(-5.));
        emit(&segs, &tabs, -4., -5., 6., 3., None, 100, 1000, &mut post);
        let moves = crate::gcode::preview::interpret(&post.finish());
        assert!(
            moves
                .iter()
                .filter(|m| m.to.z < -4.99 && m.from.z < -4.99)
                .count()
                > 10
        );
        assert!(
            moves
                .iter()
                .any(|m| m.to.y.abs() > 99.9 && m.to.z >= -4.0001),
            "{}",
            post.finish()
        );
        assert!(post.finish().contains("G3"));
    }
}

#[cfg(test)]
mod curved_retention {
    use super::*;
    #[test]
    fn clockwise_counterclockwise_overlapping_and_seam_tabs_keep_material() {
        for bulge in [-1., 1.] {
            for ramp in [None, Some(30.)] {
                let a = Point2::new(100., 0.);
                let b = Point2::new(-100., 0.);
                let c = Point2::new(0., 0.);
                let segments = [
                    Segment::arc(a, b, bulge, Some(c), "0", 0),
                    Segment::arc(b, a, bulge, Some(c), "0", 0),
                ];
                let tabs: Vec<_> = [(100., 0.), (0., 100.), (-1., 100.)]
                    .into_iter()
                    .map(|(x, y)| TabPoint {
                        x,
                        y,
                        width_override_mm: None,
                        height_override_mm: None,
                    })
                    .collect();
                let mut post = crate::gcode::linuxcnc::Post::new();
                post.move_to(Some(100.), Some(0.), Some(-4.));
                emit(
                    &segments, &tabs, -4., -5., 6., 3., ramp, 100, 1000, &mut post,
                );
                let moves = crate::gcode::preview::interpret(&post.finish());
                for m in moves
                    .iter()
                    .filter(|m| m.kind != crate::gcode::preview::MoveKind::Rapid)
                {
                    for k in 0..=50 {
                        let t = f64::from(k) / 50.;
                        let x = m.from.x + t * (m.to.x - m.from.x);
                        let y = m.from.y + t * (m.to.y - m.from.y);
                        let z = m.from.z + t * (m.to.z - m.from.z);
                        if tabs
                            .iter()
                            .any(|tab| (x - tab.x).hypot(y - tab.y) < 6. - 0.0001)
                        {
                            assert!(z >= -4.0001, "bulge={bulge} ramp={ramp:?} ({x},{y},{z})");
                        }
                    }
                }
                assert!(moves.iter().any(|m| m.to.z < -4.99));
            }
        }
    }
}
