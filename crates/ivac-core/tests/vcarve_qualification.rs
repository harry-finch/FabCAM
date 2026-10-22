//! Independent software references. Synthetic settings, not machining advice.
// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::float_cmp)]
mod common;
use common::*;
use ivac_core::cam::vcarve_plan::plan;
use ivac_core::pipeline::*;
use ivac_core::project::*;
use ivac_core::{Point2, Segment};
fn rectangle_project(medial: bool) -> Project {
    let p = [(0., 0.), (20., 0.), (20., 10.), (0., 10.)];
    let segments = (0..4)
        .map(|i| {
            Segment::line(
                Point2::new(p[i].0, p[i].1),
                Point2::new(p[(i + 1) % 4].0, p[(i + 1) % 4].1),
                "0",
                7,
            )
        })
        .collect();
    let mut op = op_single_pass(
        1,
        "VCARVE SOFTWARE ONLY",
        OpKind::VCarve {
            carve: VCarveParams {
                full_medial_axis: medial,
                ..Default::default()
            },
        },
        1,
        -1.,
    );
    op.params.step = Some(-0.2);
    build_project(
        &stock_at_origin(30., 20., 10.),
        vec![vbit_tool(1, 6., 90., 0.)],
        op,
        segments,
    )
}
fn generic(project: Project) -> Result<PipelineResponse, PipelineError> {
    run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project,
            post_processor: None,
            cps_post: None,
        }),
        |_, _, _| {},
    )
}
#[test]
fn a1_perimeter_honors_pass_step_and_depth_limited_radius() {
    let r = generic(rectangle_project(false)).unwrap();
    for level in [-0.2_f64, -0.4, -0.6, -0.8, -1.] {
        assert!(
            r.toolpath.iter().any(|s| (s.to.z - level).abs() < 0.00011),
            "missing pass depth {level}"
        );
    }
    // At depth 1 a 90-degree sharp bit engages radius 1, not radius 3.
    assert!(r
        .toolpath
        .iter()
        .any(|s| (s.to.x - 1.).abs() < 0.00011 && (s.to.z + 1.).abs() < 0.00011));
}
#[test]
fn a2_interpolated_ramp_never_exceeds_local_target() {
    let path = plan(
        &[(0., 0., -1.), (1., 0., -0.01), (10., 0., -0.01)],
        0.5,
        10.,
    )
    .unwrap();
    for w in path.windows(2) {
        for j in 0..=100 {
            let t = f64::from(j) / 100.;
            let x = w[0].0 + t * (w[1].0 - w[0].0);
            let z = w[0].2 + t * (w[1].2 - w[0].2);
            let target = if x <= 1. { -1. + 0.99 * x } else { -0.01 };
            assert!(z >= target - 1e-7, "overcut x={x} z={z} target={target}");
        }
    }
}
#[test]
fn a3_short_chain_reaches_target_at_both_ends() {
    let path = plan(&[(0., 0., -1.), (1., 0., -1.)], 0.5, 10.).unwrap();
    for x in [0., 1.] {
        assert!(
            path.iter()
                .any(|p| (p.0 - x).abs() < 1e-7 && (p.2 + 1.).abs() < 1e-7),
            "undercut at {x}"
        );
    }
}
#[test]
fn reference_cones_caps_and_flat_tips() {
    for (angle, tip, cap, expected) in [
        (90., 0., 2., 2.),
        (60., 0., 2., 3.464_101_615_137_755),
        (90., 1., 2., 1.5),
        (60., 1., 2., 2.598_076_211_353_316),
    ] {
        let mut p = rectangle_project(false);
        p.tools[0] = vbit_tool(1, 8., angle, tip);
        p.operations[0].params.depth = -10.;
        p.operations[0]
            .vcarve_params_mut()
            .unwrap()
            .carve_max_width_mm = Some(cap);
        let r = generic(p).unwrap();
        let min = r.toolpath.iter().map(|s| s.to.z).fold(0., f64::min);
        assert!((min + expected).abs() < 0.00011, "{angle} tip={tip}: {min}");
    }
}
#[test]
fn continuous_clearance_catches_between_sample_collision() {
    use ivac_core::cam::vcarve_validate::check_envelope;
    // Both endpoints have >1 mm clearance to the island. Its midpoint does not.
    let island = [
        Point2::new(4.9, -0.05),
        Point2::new(5.1, -0.05),
        Point2::new(5.1, 0.05),
        Point2::new(4.9, 0.05),
    ];
    assert!(
        check_envelope(&[(0., 0., -1.), (10., 0., -1.)], &[&island], 0., 1.)
            .unwrap_err()
            .contains("overcut")
    );
    // Variable radius tangent to a horizontal boundary is valid throughout.
    let edge = [Point2::new(0., 0.), Point2::new(10., 0.)];
    assert!(check_envelope(&[(0., 1., -1.), (10., 2., -2.)], &[&edge], 0., 1.).is_ok());
}
#[test]
fn invalid_inputs_and_physical_limits_fail_closed() {
    for variant in 0..12 {
        let mut p = rectangle_project(false);
        match variant {
            0 => p.tools[0].tip_angle_deg = 0.,
            1 => p.tools[0].tip_diameter = Some(8.),
            2 => p.tools[0].flute_length_mm = Some(0.5),
            3 => p.tools[0].vcarve_lead_in_angle_deg = Some(90.),
            4 => p.operations[0].params.start_depth = -1.,
            5 => {
                p.operations[0]
                    .vcarve_params_mut()
                    .unwrap()
                    .multi_pass_refine = true;
            }
            6 => {
                p.segments.pop();
            }
            7 => p.tools[0].kind = ToolKind::Endmill,
            9 => p.operations[0].params.through_depth = 0.2,
            10 => p.operations[0].params.depth_list = vec![-0.2, -1.],
            11 => p.operations[0].params.finish_step = Some(-0.1),
            _ => {
                p.operations[0]
                    .vcarve_params_mut()
                    .unwrap()
                    .carve_max_width_mm = Some(-1.);
            }
        }
        assert!(generic(p).is_err(), "variant {variant}");
    }
    assert!(plan(&[(0., 0., -1.), (0., 0., -1.)], 0.1, 10.).is_err());
    assert!(plan(&[(0., 0., -1.), (1., 0., f64::NAN)], 0.1, 10.).is_err());
    assert!(plan(&[(0., 0., -1.), (1e-5, 0., -1.)], 0.1, 1.)
        .unwrap_err()
        .contains("undercut"));
}
#[test]
fn medial_axis_is_variable_finite_deterministic_and_does_not_clear_plateau() {
    let p = rectangle_project(true);
    let r = generic(p.clone()).unwrap();
    let again = generic(p).unwrap();
    assert_eq!(r.gcode, again.gcode);
    assert!(r.toolpath.iter().any(|s| s.to.z < 0. && s.to.z > -0.9));
    assert!(r
        .toolpath
        .iter()
        .all(|s| [s.to.x, s.to.y, s.to.z].iter().all(|v| v.is_finite())));
    assert!(r.warnings.iter().any(|w| w.kind == "vcarve_scope"));
    // Independent swept-cone surface coverage oracle, continuous along each
    // linear motion. A capped medial skeleton must NOT masquerade as a pocket.
    let covered = |x: f64, y: f64| {
        r.toolpath
            .iter()
            .filter(|s| s.from.z <= 0. && s.to.z <= 0.)
            .any(|s| {
                let dx = s.to.x - s.from.x;
                let dy = s.to.y - s.from.y;
                let a = s.from.x - x;
                let b = s.from.y - y;
                let radius = -s.from.z; // sharp 90-degree reference cone
                let dr = s.from.z - s.to.z;
                let q2 = dx * dx + dy * dy - dr * dr;
                let q1 = 2. * (a * dx + b * dy - radius * dr);
                let q0 = a * a + b * b - radius * radius;
                let at = |t: f64| (q2 * t + q1) * t + q0;
                let mut minimum = at(0.).min(at(1.));
                if q2 > 0. {
                    minimum = minimum.min(at((-q1 / (2. * q2)).clamp(0., 1.)));
                }
                minimum <= 1e-8
            })
    };
    assert!(covered(10., 5.), "middle spine must cut");
    assert!(
        !covered(5., 2.),
        "capped medial axis leaves material away from its spines"
    );
    let mut covered_cells = 0;
    for row in 0..20 {
        for col in 0..40 {
            if covered(f64::from(col) * 0.5 + 0.25, f64::from(row) * 0.5 + 0.25) {
                covered_cells += 1;
            }
        }
    }
    println!("Full medial axis surface coverage: {covered_cells}/800 cell centres (0.5 mm grid); not an area bound");
    assert!(covered_cells > 0 && covered_cells < 800);
    // Independent rectangle clearance, at 21 points along every cutting chord.
    for s in r.toolpath.iter().filter(|s| s.from.z <= 0. && s.to.z <= 0.) {
        for j in 0..=20 {
            let t = f64::from(j) / 20.;
            let x = s.from.x + t * (s.to.x - s.from.x);
            let y = s.from.y + t * (s.to.y - s.from.y);
            let z = s.from.z + t * (s.to.z - s.from.z);
            assert!(-z <= x.min(20. - x).min(y).min(10. - y) + 0.0004);
        }
    }
}
fn ring(points: &[(f64, f64)]) -> Vec<Segment> {
    (0..points.len())
        .map(|i| {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "0", 7)
        })
        .collect()
}
#[test]
fn topology_matrix_and_winding() {
    let shapes = vec![
        ("acute triangle", vec![(0., 0.), (20., 0.), (3., 6.)]),
        (
            "nonconvex L",
            vec![
                (0., 0.),
                (20., 0.),
                (20., 4.),
                (4., 4.),
                (4., 20.),
                (0., 20.),
            ],
        ),
        (
            "thin slot",
            vec![(0., 0.), (10., 0.), (10., 0.1), (0., 0.1)],
        ),
        (
            "near collinear",
            vec![(0., 0.), (10., 0.000_001), (20., 0.), (20., 10.), (0., 10.)],
        ),
    ];
    for (name, pts) in shapes {
        for reverse in [false, true] {
            let mut pts = pts.clone();
            if reverse {
                pts.reverse();
            }
            let mut p = rectangle_project(true);
            p.segments = ring(&pts);
            let r = generic(p);
            println!(
                "matrix {name} reverse={reverse}: {}",
                r.as_ref()
                    .map_or_else(std::string::ToString::to_string, |r| format!(
                        "{} moves",
                        r.toolpath.len()
                    ))
            );
            // These reference geometries must stay supported, not silently regress to refusal.
            assert!(r.is_ok(), "{name}: {r:?}");
            // Independent oracle includes interpolated points, not only vertices.
            if let Ok(r) = r {
                for s in r.toolpath.iter().filter(|s| s.from.z <= 0. && s.to.z <= 0.) {
                    for j in 0..=10 {
                        let t = f64::from(j) / 10.;
                        let x = s.from.x + t * (s.to.x - s.from.x);
                        let y = s.from.y + t * (s.to.y - s.from.y);
                        let z = s.from.z + t * (s.to.z - s.from.z);
                        let clearance = (0..pts.len())
                            .map(|i| {
                                let a = pts[i];
                                let b = pts[(i + 1) % pts.len()];
                                let dx = b.0 - a.0;
                                let dy = b.1 - a.1;
                                let u = (((x - a.0) * dx + (y - a.1) * dy) / (dx * dx + dy * dy))
                                    .clamp(0., 1.);
                                (x - a.0 - u * dx).hypot(y - a.1 - u * dy)
                            })
                            .fold(f64::INFINITY, f64::min);
                        assert!(-z <= clearance + 0.0004, "{name} overcut");
                    }
                }
            } else {
                assert!(r.unwrap_err().to_string().contains("V-Carve"));
            }
        }
    }
    for pts in [
        vec![(0., 0.), (10., 10.), (0., 10.), (10., 0.)],
        vec![(0., 0.), (1., 0.), (2., 0.)],
    ] {
        let mut p = rectangle_project(true);
        p.segments = ring(&pts);
        assert!(generic(p).is_err());
    }
}
#[test]
fn circle_islands_and_disconnected_regions_are_explicit() {
    for medial in [false, true] {
        for case in 0..3 {
            let mut p = rectangle_project(medial);
            if case == 0 {
                let mut a = Segment::line(Point2::new(5., 10.), Point2::new(15., 10.), "0", 7);
                a.kind = ivac_core::SegmentKind::Arc;
                a.bulge = 1.;
                let mut b = a.clone();
                b.start = a.end;
                b.end = a.start;
                p.segments = vec![a, b];
            } else if case == 1 {
                p.segments = ring(&[(0., 0.), (20., 0.), (20., 20.), (0., 20.)]);
                p.segments
                    .extend(ring(&[(7., 7.), (13., 7.), (13., 13.), (7., 13.)]));
            } else {
                p.segments
                    .extend(ring(&[(30., 0.), (40., 0.), (40., 10.), (30., 10.)]));
            }
            let r = generic(p);
            println!(
                "matrix case={case} medial={medial}: {}",
                r.as_ref()
                    .map_or_else(std::string::ToString::to_string, |r| format!(
                        "{} moves",
                        r.toolpath.len()
                    ))
            );
            assert!(r.is_ok(), "reference case={case} medial={medial}: {r:?}");
            // Pipeline 50 fixes the old 0.25 mm island-join overcut. Qualify
            // the newly supported perimeter fixture against raw geometry,
            // including interpolated XYZ points (90° sharp bit: radius=-Z).
            if case == 1 && !medial {
                let response = r.as_ref().unwrap();
                for segment in response
                    .toolpath
                    .iter()
                    .filter(|s| s.from.z <= 0. && s.to.z <= 0.)
                {
                    for j in 0..=100 {
                        let t = f64::from(j) / 100.;
                        let x = segment.from.x + t * (segment.to.x - segment.from.x);
                        let y = segment.from.y + t * (segment.to.y - segment.from.y);
                        let z = segment.from.z + t * (segment.to.z - segment.from.z);
                        let outer = x.min(y).min(20. - x).min(20. - y);
                        let island = (7. - x)
                            .max(0.)
                            .max(x - 13.)
                            .hypot((7. - y).max(0.).max(y - 13.));
                        assert!(
                            -z <= outer.min(island) + 0.0002,
                            "island perimeter overcut at {x},{y},{z}"
                        );
                    }
                }
                for around_island in [false, true] {
                    assert!(
                        response
                            .toolpath
                            .iter()
                            .any(|s| (s.to.z + 1.).abs() < 0.0001
                                && ((s.to.x > 5. && s.to.x < 15. && s.to.y > 5. && s.to.y < 15.)
                                    == around_island)),
                        "both perimeter loops must reach the target depth"
                    );
                }
            }
            if let Ok(r) = r {
                assert!(!r.toolpath.is_empty());
            } else {
                assert!(r.unwrap_err().to_string().contains("V-Carve"));
            }
        }
    }
}
#[cfg(feature = "cps")]
fn shopbot(project: Project) -> Result<PipelineResponse, PipelineError> {
    run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: std::collections::BTreeMap::default(),
            }),
        }),
        |_, _, _| {},
    )
}
#[cfg(feature = "cps")]
#[test]
fn a4_decoded_output_preview_generic_parity_and_persistence() {
    use ivac_core::gcode::preview::MoveKind;
    for (medial, island) in [(false, false), (true, false), (false, true)] {
        let mut p = rectangle_project(medial);
        if island {
            p.segments = ring(&[(0., 0.), (20., 0.), (20., 20.), (0., 20.)]);
            p.segments
                .extend(ring(&[(7., 7.), (13., 7.), (13., 13.), (7., 13.)]));
        }
        p.work_offset.x_mm = 5.;
        p.work_offset.y_mm = 7.;
        let restored: Project = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        let native = generic(p.clone()).unwrap();
        let r = shopbot(p).unwrap();
        assert_eq!(r.gcode, shopbot(restored).unwrap().gcode);
        // Independent parser: no production decoder calls or invented startup XY.
        let mut pos = [None, None, None];
        let mut feed = 0.;
        let mut decoded = Vec::new();
        for (line, text) in r.gcode.lines().enumerate() {
            let mut f = text.split(',');
            let cmd = f.next().unwrap_or("");
            if !["MS", "M3", "J2", "JZ"].contains(&cmd) {
                continue;
            }
            let v: Vec<f64> = f.map(|s| s.parse().unwrap()).collect();
            if cmd == "MS" {
                assert_eq!(v[0], v[1]);
                feed = v[0];
                continue;
            }
            let old = pos;
            match cmd {
                "M3" => {
                    for i in 0..3 {
                        pos[i] = Some(v[i]);
                    }
                }
                "J2" => {
                    pos[0] = Some(v[0]);
                    pos[1] = Some(v[1]);
                }
                "JZ" => pos[2] = Some(v[0]),
                _ => unreachable!(),
            }
            if let ([Some(x), Some(y), Some(z)], [Some(bx), Some(by), Some(bz)]) = (old, pos) {
                if (x - bx).abs() + (y - by).abs() + (z - bz).abs() < 1e-10 {
                    continue;
                }
                if cmd == "M3" {
                    assert!(feed > 0.);
                    assert!((feed - 20.).abs() < 1e-6 || (feed - 200. / 60.).abs() < 1e-6);
                }
                if cmd == "J2" {
                    assert!(z > 0. && bz > 0., "unsafe inter-region travel");
                }
                decoded.push(([x, y, z], [bx, by, bz], cmd == "M3", line + 1));
            }
        }
        assert_eq!(decoded.len(), r.toolpath.len());
        for ((a, b, cut, line), s) in decoded.iter().zip(&r.toolpath) {
            assert_eq!(*line, s.gcode_line as usize);
            for (u, v) in a.iter().zip([s.from.x, s.from.y, s.from.z]) {
                assert!((u - v).abs() < 1e-9);
            }
            for (u, v) in b.iter().zip([s.to.x, s.to.y, s.to.z]) {
                assert!((u - v).abs() < 1e-9);
            }
            assert_eq!(*cut, !matches!(s.kind, MoveKind::Rapid));
            assert!(s.arc.is_none());
            assert_eq!(s.op_id, 1);
        }
        let actual: Vec<_> = decoded
            .iter()
            .filter(|(_, _, cut, _)| *cut)
            .map(|(_, b, _, _)| *b)
            .collect();
        let intended: Vec<_> = native
            .toolpath
            .iter()
            .filter(|s| {
                !matches!(s.kind, MoveKind::Rapid)
                    && (s.to.x - s.from.x).abs()
                        + (s.to.y - s.from.y).abs()
                        + (s.to.z - s.from.z).abs()
                        > 1e-10
            })
            .map(|s| [s.to.x, s.to.y, s.to.z])
            .collect();
        assert_eq!(actual.len(), intended.len());
        for (a, b) in actual.iter().zip(intended) {
            for i in 0..3 {
                assert!((a[i] - b[i]).abs() < 0.00011, "resolved/generic mismatch");
            }
        }
        assert!((actual.iter().map(|p| p[2]).fold(0., f64::min) + 1.).abs() < 0.00011);
    }
}

#[path = "support/explicit.rs"]
mod fixture_support;
