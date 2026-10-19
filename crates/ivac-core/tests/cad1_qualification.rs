//! Geometry-only CAD relief through the unchanged generic CAM and `ShopBot` post.
#![cfg(feature = "cps")]
mod common;
use common::{build_project, endmill_tool, op_single_pass, stock_at_origin};
use ivac_core::corner_relief::*;
use ivac_core::pipeline::*;
use ivac_core::project::*;
use ivac_core::{Point2, Segment};

fn fixture(angle: f64, mode: ReliefMode) -> (Vec<Segment>, Point2) {
    let a = angle.to_radians();
    let points = [
        Point2::new(60., 20.),
        Point2::new(100., 20.),
        Point2::new(60. + 40. * a.cos(), 20. + 40. * a.sin()),
    ];
    let segments = (0..3)
        .map(|i| Segment::line(points[i], points[(i + 1) % 3], "0", 7))
        .collect();
    let response = preview(&CornerReliefRequest {
        segments,
        objects: vec![1; 3],
        selected_objects: vec![1],
        selected_corners: vec![CornerRef {
            object_id: 1,
            corner_index: 0,
        }],
        mode,
        removal_side: RemovalSide::Interior,
        tool_diameter_mm: 6.,
    });
    assert!(response.error.is_none(), "{:?}", response.error);
    (response.segments, points[0])
}
fn moves(text: &str) -> Vec<([f64; 3], [f64; 3])> {
    let mut pos = [0., 0., 5.];
    let mut out = Vec::new();
    for line in text.lines() {
        let mut fields = line.split(',');
        let cmd = fields.next().unwrap();
        if ["M3", "J2", "JZ"].contains(&cmd) {
            let v: Vec<f64> = fields.map(|s| s.parse().unwrap()).collect();
            let from = pos;
            match cmd {
                "M3" => pos.copy_from_slice(&v),
                "J2" => {
                    pos[0] = v[0];
                    pos[1] = v[1];
                }
                "JZ" => pos[2] = v[0],
                _ => unreachable!(),
            }
            if cmd == "M3" {
                out.push((from, pos));
            }
        }
    }
    out
}
fn distance_to_line(p: Point2, a: [f64; 3], b: [f64; 3]) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let denom = dx * dx + dy * dy;
    let t = if denom > 0. {
        (((p.x - a[0]) * dx + (p.y - a[1]) * dy) / denom).clamp(0., 1.)
    } else {
        0.
    };
    (a[0] + t * dx - p.x).hypot(a[1] + t * dy - p.y)
}
// Independent analytic region oracle: triangle union the requested radius-3 disk.
// Sample the cutter circumference along each emitted cutting chord. Tolerance
// 0.012 mm includes the existing 0.01 mm post arc linearization, not a fit allowance.
fn assert_sampled_sweep(angle: f64, mode: ReliefMode, parsed: &[([f64; 3], [f64; 3])]) {
    let theta = angle.to_radians();
    let triangle = [
        Point2::new(60., 20.),
        Point2::new(100., 20.),
        Point2::new(60. + 40. * theta.cos(), 20. + 40. * theta.sin()),
    ];
    let bearing = match mode {
        ReliefMode::Dogbone => theta / 2.,
        ReliefMode::TboneIncoming => theta,
        ReliefMode::TboneOutgoing => 0.,
    };
    let center = Point2::new(60. + 3. * bearing.cos(), 20. + 3. * bearing.sin());
    for &(a, b) in parsed {
        if a[2] > -1.9999 || b[2] > -1.9999 {
            continue;
        }
        for step in 0..=4 {
            let t = f64::from(step) / 4.;
            let x = a[0] + t * (b[0] - a[0]) + 5.;
            let y = a[1] + t * (b[1] - a[1]) + 7.;
            for sample in 0..72 {
                let phi = f64::from(sample) * std::f64::consts::TAU / 72.;
                let p = Point2::new(x + 3. * phi.cos(), y + 3. * phi.sin());
                let inside = (0..3).all(|i| {
                    let u = triangle[i];
                    let v = triangle[(i + 1) % 3];
                    ((v.x - u.x) * (p.y - u.y) - (v.y - u.y) * (p.x - u.x)) / u.distance(v)
                        >= -0.012
                });
                assert!(
                    inside || p.distance(center) <= 3.012,
                    "unintended sampled removal at {p:?}, angle={angle}, mode={mode:?}"
                );
            }
        }
    }
}
#[test]
fn exact_diameter_corners_are_reached_in_emitted_profile_and_pocket() {
    for angle in [90., 110., 135.] {
        for mode in [
            ReliefMode::Dogbone,
            ReliefMode::TboneIncoming,
            ReliefMode::TboneOutgoing,
        ] {
            for pocket in [false, true] {
                let (segments, vertex) = fixture(angle, mode);
                let kind = if pocket {
                    OpKind::Pocket {
                        strategy: PocketStrategy::Cascade,
                        contour: ContourParams::default(),
                        pocket: PocketParams {
                            pocket_islands: true,
                            ..Default::default()
                        },
                    }
                } else {
                    OpKind::Profile {
                        offset: ToolOffset::Inside,
                        contour: ContourParams::default(),
                        profile: ProfileParams::default(),
                    }
                };
                let op = op_single_pass(1, "CAD1", kind, 1, -2.);
                let mut project = build_project(
                    &stock_at_origin(150., 100., 10.),
                    vec![endmill_tool(1, 6.)],
                    op,
                    segments,
                );
                project.work_offset.x_mm = 5.;
                project.work_offset.y_mm = 7.;
                let response = run_pipeline(
                    PipelineRequest {
                        project,
                        post_processor: Some(PostProcessorKind::Cps),
                        cps_post: Some(CpsPostSelection {
                            source: CpsPostSource::Bundled {
                                id: "shopbot".into(),
                            },
                            properties: std::collections::BTreeMap::default(),
                        }),
                    },
                    |_, _, _| {},
                )
                .unwrap();
                let parsed = moves(&response.gcode);
                assert_sampled_sweep(angle, mode, &parsed);
                let target = Point2::new(vertex.x - 5., vertex.y - 7.);
                let distance = parsed
                    .iter()
                    .filter(|(a, b)| a[2] < -1.9999 && b[2] < -1.9999)
                    .map(|&(a, b)| distance_to_line(target, a, b))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    distance <= 3.00015,
                    "{angle} {mode:?} pocket={pocket}: vertex gap {}",
                    distance - 3.
                );
                // Every decoded cutting move has its final preview counterpart. Coordinates
                // stay in work space; the frontend maps them back to the drawing.
                for (_, b) in &parsed {
                    assert!(
                        response
                            .toolpath
                            .iter()
                            .any(|m| (m.to.x - b[0]).abs() < 0.0002
                                && (m.to.y - b[1]).abs() < 0.0002
                                && (m.to.z - b[2]).abs() < 0.0002),
                        "missing final preview endpoint"
                    );
                }
                assert!(response
                    .remaining_material
                    .iter()
                    .all(|r| r.status == "unsupported"));
            }
        }
    }
}
