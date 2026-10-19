//! Lot A software-only guard: unsupported motion must not become a successful program.
#![cfg(feature = "cps")]
mod common;
use common::{build_project, closed_rectangle, endmill_tool, op_single_pass, stock_at_origin};
use ivac_core::cancel::CancelToken;
use ivac_core::pipeline::{
    generate_streaming, run_pipeline, CpsPostSelection, CpsPostSource, PipelineError,
    PipelineEvent, PipelineRequest, PostProcessorKind,
};
use ivac_core::project::{ContourParams, OpKind, ProbeAxis, ProfileParams, ToolOffset};
fn request(kind: OpKind) -> PipelineRequest {
    let mut project = build_project(
        &stock_at_origin(40., 40., 10.),
        vec![endmill_tool(1, 3.)],
        op_single_pass(1, "control", kind, 0, -1.),
        closed_rectangle(5., 5., 20., 20.),
    );
    project.operations.push(op_single_pass(
        2,
        "milling",
        OpKind::Profile {
            offset: ToolOffset::Outside,
            contour: ContourParams::default(),
            profile: ProfileParams::default(),
        },
        1,
        -1.,
    ));
    PipelineRequest {
        project,
        post_processor: Some(PostProcessorKind::Cps),
        cps_post: Some(CpsPostSelection {
            source: CpsPostSource::Bundled {
                id: "shopbot".into(),
            },
            properties: std::collections::BTreeMap::default(),
        }),
    }
}
#[test]
fn cps_motion_refuses_after_roundtrip_before_success_event() {
    for kind in [
        OpKind::Homing {
            retract_to_safe_z: true,
        },
        OpKind::Probe {
            axis: ProbeAxis::Z,
            distance_mm: -10.,
            feed_mm_min: 100,
        },
    ] {
        let req = request(kind);
        let req: PipelineRequest =
            serde_json::from_slice(&serde_json::to_vec(&req).unwrap()).unwrap();
        let mut done = false;
        let error = generate_streaming(req.clone(), &CancelToken::new(), &mut |event| {
            if matches!(event, PipelineEvent::Done { .. }) {
                done = true;
            }
        })
        .unwrap_err();
        assert!(matches!(error, PipelineError::UnsupportedCpsMotion(1, _)));
        assert!(!done);
        assert!(error.to_string().contains("No program generated"));
        let mut disabled = req.clone();
        disabled.project.operations[0].enabled = false;
        assert!(run_pipeline(disabled, |_, _, _| {}).is_ok());
        let mut native = req;
        native.post_processor = Some(PostProcessorKind::Linuxcnc);
        native.cps_post = None;
        let output = run_pipeline(native, |_, _, _| {}).unwrap();
        assert!(output.gcode.contains("G28") || output.gcode.contains("G38.2"));
    }
}
#[test]
fn cps_marker_remains_explicitly_skipped_and_pause_still_emits() {
    let marker = run_pipeline(
        request(OpKind::CycleMarker {
            label: "section".into(),
        }),
        |_, _, _| {},
    )
    .unwrap();
    assert!(marker
        .warnings
        .iter()
        .any(|w| w.kind == "cps_unsupported_op"));
    let pause = run_pipeline(
        request(OpKind::Pause {
            message: "inspect".into(),
        }),
        |_, _, _| {},
    )
    .unwrap();
    assert!(pause.gcode.contains("PAUSE"));
}
