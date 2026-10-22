// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::float_cmp, clippy::unnecessary_box_returns)]
#[allow(dead_code)]
#[path = "test1/oracle.rs"]
mod oracle;
// SAFE-Z Z06–Z08 analytical regressions and explicit policy characterizations.
// Passing characterization tests do NOT certify export/streaming safety.
mod common;
use common::*;
use ivac_core::gcode::preview::{MoveKind, Pose3, ToolpathSegment};
use ivac_core::project::*;
use ivac_core::sim::{
    fixture_check::{check_segment_against_fixtures, FixtureCheck},
    holder::HolderProfile,
};

fn fixture(kind: FixtureKind) -> Fixture {
    Fixture {
        id: 1,
        name: "analytical clamp".into(),
        kind,
        origin: (1., 4.),
        z_bottom: 5.,
        z_top: 10.,
        color: 0,
    }
}
fn body() -> HolderProfile {
    let mut t = endmill_tool(1, 2.);
    t.flute_length_mm = Some(0.);
    t.holder = Some(HolderShape::Cylinder {
        diameter_mm: 10.,
        length_mm: 20.,
    });
    HolderProfile::from_tool(&t).unwrap()
}
fn motion(from: [f64; 3], to: [f64; 3]) -> ToolpathSegment {
    ToolpathSegment {
        tool_ref: None,
        from: Pose3 {
            x: from[0],
            y: from[1],
            z: from[2],
        },
        to: Pose3 {
            x: to[0],
            y: to[1],
            z: to[2],
        },
        kind: MoveKind::Rapid,
        gcode_line: 1,
        op_id: 1,
        arc: None,
    }
}
#[test]
fn sloped_rapid_cannot_hide_holder_collision_below_high_endpoint() {
    // At t=0.1 the tip is(1,0,2); the cylinder spans Z2..22 with R5.
    // Witness (1,4,7) is strictly inside both holder and fixture. Neither
    // endpoint tip footprint reaches the fixture. This is an analytic witness,
    // independent of the production sweep/preview implementation.
    for end_z in [11., 20., 40.] {
        for reverse in [false, true] {
            for kind in [
                FixtureKind::Box {
                    width: 1.,
                    depth: 1.,
                },
                FixtureKind::Cylinder { radius: 0.5 },
            ] {
                let mut segment = motion([0., 0., 0.], [10., 0., end_z]);
                if reverse {
                    std::mem::swap(&mut segment.from, &mut segment.to);
                }
                let result =
                    check_segment_against_fixtures(&segment, 1., Some(&body()), &[fixture(kind)]);
                assert!(
                    result
                        .iter()
                        .any(|v| matches!(v, FixtureCheck::Collision { fixture_id: 1, .. })),
                    "end_z={end_z},reverse={reverse}: analytic interior holder witness missed"
                );
            }
        }
    }
}
#[test]
fn distant_or_above_body_fixtures_remain_clear() {
    let segment = motion([0., 0., 0.], [10., 0., 20.]);
    let mut far = fixture(FixtureKind::Cylinder { radius: 0.5 });
    far.origin.1 = 20.;
    let mut high = fixture(FixtureKind::Box {
        width: 1.,
        depth: 1.,
    });
    high.z_bottom = 50.;
    high.z_top = 55.;
    assert!(
        check_segment_against_fixtures(&segment, 1., Some(&body()), &[far, high])
            .iter()
            .all(|v| *v == FixtureCheck::Clear)
    );
}

#[cfg(feature = "cps")]
mod pipeline_checks {
    use super::*;
    use crate::oracle;
    use ivac_core::pipeline::*;
    use std::sync::{Arc, Mutex};
    fn record(name: &str, value: impl serde::Serialize) {
        if let Ok(dir) = std::env::var("SAFE_Z_EVIDENCE") {
            std::fs::write(
                std::path::Path::new(&dir).join(name),
                serde_json::to_vec_pretty(&value).unwrap(),
            )
            .unwrap();
        }
    }
    fn request(cps: bool) -> PipelineRequest {
        let mut req: PipelineRequest = serde_json::from_str(include_str!(
            "../../../tests/fixtures/test1/profile.request.json"
        ))
        .unwrap();
        req.project.segments = closed_rectangle(20., 20., 40., 40.);
        req.project.tools = vec![endmill_tool(1, 6.)];
        req.project.operations[0].params.depth = -1.;
        req.project.operations[0].params.step = Some(-0.5);
        req.project.machine.work_area.z = 0.;
        if !cps {
            req.post_processor = Some(PostProcessorKind::Linuxcnc);
            req.cps_post = None;
        }
        req
    }
    #[derive(Clone)]
    struct Capture(Arc<Mutex<Vec<u8>>>);
    impl std::io::Write for Capture {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    fn rejected(req: PipelineRequest) -> Box<DiagnosticPreview> {
        match run_pipeline(crate::fixture_support::request(req), |_, _, _| {}) {
            Err(PipelineError::FixtureCollision(preview)) => preview,
            other => panic!("expected collision refusal: {other:?}"),
        }
    }
    #[test]
    fn collisions_refuse_output_preserve_preview_and_streams_emit_zero_bytes() {
        for cps in [false, true] {
            let mut req = request(cps);
            req.project.fixtures.push(Fixture {
                id: 1,
                name: "clamp across profile".into(),
                kind: FixtureKind::Box {
                    width: 2.,
                    depth: 2.,
                },
                origin: (30., 17.),
                z_bottom: -2.,
                z_top: 12.,
                color: 0,
            });
            let original = serde_json::to_value(&req).unwrap();
            let preview = rejected(req.clone());
            assert!(!preview.toolpath.is_empty());
            assert!(preview
                .warnings
                .iter()
                .any(|w| w.kind == "fixture_path_collision"));
            assert!(preview
                .warnings
                .iter()
                .any(|w| w.kind == "fixture_clearance_unknown"));
            let err = PipelineError::FixtureCollision(preview)
                .to_structured(None)
                .unwrap();
            let json = serde_json::to_value(&err).unwrap();
            assert!(json["diagnostic_preview"].get("gcode").is_none());
            assert!(json.get("gcode").is_none());
            for distant in [false, true] {
                if distant {
                    req.project.fixtures[0].origin = (90., 90.);
                }
                let bytes = Arc::new(Mutex::new(vec![]));
                assert!(stream_gcode_with_preview(
                    crate::fixture_support::request(req.clone()),
                    Box::new(Capture(bytes.clone()))
                )
                .is_err());
                assert!(stream_gcode_to_writer(
                    crate::fixture_support::request(req.clone()),
                    Box::new(Capture(bytes.clone()))
                )
                .is_err());
                assert!(bytes.lock().unwrap().is_empty());
            }
            req.project.fixtures[0].origin = (30., 17.);
            assert_eq!(serde_json::to_value(&req).unwrap(), original);
            record(&format!("publication-guard-{cps}.json"), json);
        }
    }
    #[test]
    fn fixture_frame_translation_and_cache_mutation_preserve_warning_semantics() {
        let mut req = request(true);
        req.project.fixtures.push(Fixture {
            id: 1,
            name: "clamp".into(),
            kind: FixtureKind::Cylinder { radius: 1. },
            origin: (30., 17.),
            z_bottom: -2.,
            z_top: 12.,
            color: 0,
        });
        let base = rejected(req.clone());
        req.project.work_offset.x_mm = 7.;
        req.project.work_offset.y_mm = -3.;
        let shifted = rejected(req.clone());
        assert_eq!(base.toolpath.len(), shifted.toolpath.len());
        for (a, b) in base.toolpath.iter().zip(&shifted.toolpath) {
            assert!((a.to.x - 7. - b.to.x).abs() < 0.00011);
            assert!((a.to.y + 3. - b.to.y).abs() < 0.00011);
            assert_eq!(a.to.z, b.to.z);
        }
        let kinds = |warnings: &[PipelineWarning]| {
            warnings
                .iter()
                .filter(|w| w.kind.starts_with("fixture_"))
                .map(|w| w.kind.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(kinds(&base.warnings), kinds(&shifted.warnings));
        req.project.fixtures[0].origin = (90., 90.);
        let clear =
            run_pipeline(crate::fixture_support::request(req.clone()), |_, _, _| {}).unwrap();
        assert!(!kinds(&clear.warnings).contains(&"fixture_path_collision".into()));
        req.project.fixtures[0].origin = (30., 17.);
        let restored = rejected(req);
        assert_eq!(kinds(&restored.warnings), kinds(&shifted.warnings));
    }
    #[test]
    fn decoded_two_tool_transitions_clear_stock_but_initial_pose_stays_unknown() {
        let mut req = request(true);
        req.project.tools.push(endmill_tool(2, 4.));
        let mut second = req.project.operations[0].clone();
        second.id = 2;
        second.tool_id = 2;
        second.params.fast_move_z = 8.;
        req.project.operations.push(second);
        let r = run_pipeline(crate::fixture_support::request(req.clone()), |_, _, _| {}).unwrap();
        let moves = oracle::decode(&r.gcode).unwrap();
        assert!(!moves.is_empty());
        assert!(r.gcode.contains("Load tool 2"));
        for (i, m) in moves.iter().enumerate() {
            if m.rapid && (m.a[2] <= 0. || m.b[2] <= 0.) {
                assert_eq!(m.a[0], m.b[0]);
                assert_eq!(m.a[1], m.b[1]);
                assert!(m.b[2] > m.a[2]);
                assert!(i > 0 && !moves[i - 1].rapid);
                assert_eq!(moves[i - 1].b, m.a);
            }
        }
        record(
            "two-tool-transitions.json",
            serde_json::json!({"request":req,"response":r,"initial_pose":"UNKNOWN: operator prerequisite, no first-rapid clearance claim"}),
        );
    }
    #[test]
    fn progress_stream_collision_never_signals_done_and_cancel_never_publishes() {
        let mut req = request(false);
        req.project.fixtures.push(Fixture {
            id: 1,
            name: "clamp".into(),
            kind: FixtureKind::Box {
                width: 1000.,
                depth: 1000.,
            },
            origin: (0., 0.),
            z_bottom: -100.,
            z_top: 100.,
            color: 0,
        });
        let token = CancelToken::new();
        let mut done = false;
        assert!(matches!(
            generate_streaming(
                crate::fixture_support::request(req.clone()),
                &token,
                &mut |event| {
                    if matches!(event, PipelineEvent::Done { .. }) {
                        done = true;
                    }
                }
            ),
            Err(PipelineError::FixtureCollision(_))
        ));
        assert!(!done);
        token.cancel();
        assert!(matches!(
            generate_streaming(crate::fixture_support::request(req), &token, &mut |_| {}),
            Err(PipelineError::Cancelled)
        ));
    }
    #[test]
    fn declared_complete_holder_blocks_where_unknown_body_only_warns() {
        let mut req = request(false);
        req.project.fixtures.push(Fixture {
            id: 1,
            name: "upper clamp".into(),
            kind: FixtureKind::Box {
                width: 2.,
                depth: 2.,
            },
            origin: (30., 21.2),
            z_bottom: 8.,
            z_top: 12.,
            color: 0,
        });
        let unknown =
            run_pipeline(crate::fixture_support::request(req.clone()), |_, _, _| {}).unwrap();
        assert!(unknown
            .warnings
            .iter()
            .any(|w| w.kind == "fixture_clearance_unknown"));
        let tool = &mut req.project.tools[0];
        tool.flute_length_mm = Some(10.);
        tool.stickout_length_mm = Some(0.);
        tool.shank_diameter_mm = Some(6.);
        tool.holder = Some(ivac_core::project::HolderShape::Cylinder {
            diameter_mm: 10.,
            length_mm: 20.,
        });
        let preview = rejected(req);
        assert!(!preview
            .warnings
            .iter()
            .any(|w| w.kind == "fixture_clearance_unknown"));
    }
    #[test]
    fn declared_flute_is_checked_even_when_holder_is_unknown() {
        let mut tool = endmill_tool(1, 2.);
        tool.flute_length_mm = Some(10.);
        let (envelope, complete) = HolderProfile::declared_fixture_envelope(&tool);
        assert!(!complete);
        assert_eq!(envelope.as_ref().unwrap().total_length(), 10.);
        let mut f = fixture(FixtureKind::Cylinder { radius: 0.5 });
        f.z_bottom = 5.;
        f.z_top = 8.;
        f.origin = (0., 0.);
        assert!(check_segment_against_fixtures(
            &motion([-10., 0., 0.], [10., 0., 0.]),
            1.,
            envelope.as_ref(),
            &[f]
        )
        .iter()
        .any(|v| matches!(v, FixtureCheck::Collision { .. })));
    }
    #[test]
    fn shopbot_unqualified_machine_stations_and_parking_stay_refused() {
        for mode in 0..3 {
            let mut req = request(true);
            match mode {
                0 => req.project.machine.park_xy = Some((50., 50.)),
                1 => req.project.machine.park_at_home = true,
                _ => req.project.machine.toolchange_xy = Some((50., 50.)),
            }
            assert!(
                run_pipeline(crate::fixture_support::request(req), |_, _, _| {}).is_err(),
                "mode {mode}"
            );
        }
    }
    #[test]
    fn characterize_partial_body_metadata_is_not_complete_clearance_evidence() {
        let mut req = request(true);
        req.project.tools[0].shank_diameter_mm = Some(6.);
        let profile = HolderProfile::from_tool(&req.project.tools[0]).unwrap();
        assert_eq!(
            profile.total_length(),
            0.,
            "diameter alone establishes no positive body height"
        );
        req.project.fixtures.push(Fixture {
            id: 1,
            name: "high clamp, body length unknown".into(),
            kind: FixtureKind::Box {
                width: 2.,
                depth: 2.,
            },
            origin: (30., 17.),
            z_bottom: 20.,
            z_top: 25.,
            color: 0,
        });
        let r = run_pipeline(crate::fixture_support::request(req.clone()), |_, _, _| {}).unwrap();
        assert!(r
            .warnings
            .iter()
            .any(|w| w.kind == "fixture_clearance_unknown"));
        assert!(!r
            .warnings
            .iter()
            .any(|w| w.kind == "fixture_path_collision"));
        record(
            "partial-body-unknown.json",
            serde_json::json!({"request":req,"response":r,"declared_body_height":profile.total_length(),"verdict":"UNQUALIFIED: diameter-only metadata is not a complete body/holder description"}),
        );
    }
}

#[test]
fn constant_height_holder_crossing_and_output_uncertainty_band() {
    // A wide holder enters a clamp at mid-segment, endpoints clear in XY.
    // Fixture radius0.5 + holder radius5 -> contact at offset5.5mm.
    // This is an analytic boundary test, not a machining clearance margin.
    let segment = motion([-10., 0., 0.], [10., 0., 0.]);
    for (offset, hit) in [(5.5 - 1e-8, true), (5.5 + 1e-8, false)] {
        let mut f = fixture(FixtureKind::Cylinder { radius: 0.5 });
        f.origin = (0., offset);
        let actual = check_segment_against_fixtures(&segment, 1., Some(&body()), &[f]);
        assert_eq!(
            actual
                .iter()
                .any(|v| matches!(v, FixtureCheck::Collision { .. })),
            hit
        );
    }
}

#[path = "support/explicit.rs"]
mod fixture_support;
