//! CPS pipeline integration (cps.7, ivac-yhdf.8): the full
//! request → recorder → JS post → response path, snapshot-pinned for
//! the bundled grbl post and the refs FANUC oracle (skip-if-absent),
//! plus geometry parity and the unqualified two-sided output guard.
//!
//! Snapshots are seahash digests (the full text lives in the JS run —
//! refresh with `IVAC_UPDATE_SNAPSHOTS=1`, which prints digest AND
//! text for hand-review).
#![cfg(feature = "cps")]

mod common;

use std::collections::BTreeMap;

use common::{build_project, closed_circle, closed_rectangle, drill_tool, endmill_tool};
use ivac_core::pipeline::{
    run_pipeline, run_pipeline_two_sided, CpsPostSelection, CpsPostSource, PipelineRequest,
    PipelineResponse, PostProcessorKind,
};
use ivac_core::project::{
    DrillCycle, FlipAxis, FlipRegistration, Op, OpKind, StockConfig, WorkpieceSide,
};

fn stock() -> StockConfig {
    StockConfig {
        origin: [-10.0, -10.0],
        width_mm: 120.0,
        height_mm: 80.0,
        thickness_mm: 12.0,
        ..StockConfig::default()
    }
}

fn selection(source: CpsPostSource) -> CpsPostSelection {
    CpsPostSelection {
        source,
        properties: BTreeMap::new(),
    }
}

fn run_cps(project: ivac_core::project::Project, source: CpsPostSource) -> PipelineResponse {
    run_pipeline(
        PipelineRequest {
            project,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(selection(source)),
        },
        |_, _, _| {},
    )
    .expect("cps pipeline runs")
}

/// Profile around a rectangle plus a drill pattern — exercises linear
/// motion, arcs (corner rounding), a tool change, and canned cycles.
fn fixture_project() -> ivac_core::project::Project {
    let mut segments = closed_rectangle(0.0, 0.0, 60.0, 40.0);
    segments.extend(closed_circle(80.0, 20.0, 2.5));

    let mut profile = Op::default();
    profile.id = 1;
    profile.name = "Profile outer".into();
    profile.params.depth = -3.0;
    // Explicit air clearance: zero at stock top is intentionally refused.
    profile.params.fast_move_z = 5.0;
    profile.params.step = Some(-1.5);
    profile.source = ivac_core::project::OpSource::Layers {
        layers: vec!["0".into()],
        combine: ivac_core::project::SourceCombine::default(),
    };

    let mut drill = Op {
        id: 2,
        name: "Drill pattern".into(),
        kind: OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: 3.0,
                dwell_sec: 0.0,
            },
            chamfer_after_width_mm: None,
            pattern: None,
            spot_first: None,
        },
        tool_id: 2,
        ..Op::default()
    };
    drill.params.depth = -8.0;
    drill.params.fast_move_z = 5.0;

    let mut project = build_project(
        &stock(),
        vec![endmill_tool(1, 6.0), drill_tool(2, 5.0, 118.0)],
        profile,
        segments,
    );
    project.operations.push(drill);
    project
}

/// Historical post-format oracle, with no declared stock/clearance qualification.
/// Known-stock acceptance is tested separately with explicit 5 mm air clearance.
fn legacy_post_fixture() -> ivac_core::project::Project {
    let mut project = fixture_project();
    project.stock = None;
    for op in &mut project.operations {
        op.params.fast_move_z = 0.;
    }
    project
}

fn digest(text: &str) -> u64 {
    seahash::hash(text.as_bytes())
}

fn assert_digest(name: &str, text: &str, expected: u64) {
    let actual = digest(text);
    if std::env::var("IVAC_UPDATE_SNAPSHOTS").is_ok() {
        eprintln!("=== UPDATE DIGEST [{name}] = 0x{actual:016x} ===\n{text}");
        panic!("IVAC_UPDATE_SNAPSHOTS set — paste the digest and rerun without it.");
    }
    assert_eq!(
        actual, expected,
        "[{name}] output drifted (digest 0x{actual:016x}); rerun with IVAC_UPDATE_SNAPSHOTS=1 to review + refresh.\n{text}"
    );
}

#[test]
fn legacy_zero_clearance_is_refused_when_stock_is_declared() {
    let mut project = legacy_post_fixture();
    project.stock = Some(stock());
    let error = run_pipeline(
        PipelineRequest {
            project,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(selection(CpsPostSource::Bundled { id: "grbl".into() })),
        },
        |_, _, _| {},
    )
    .unwrap_err();
    assert!(error.to_string().contains("clearance"));
}

#[test]
fn bundled_grbl_snapshot() {
    let response = run_cps(
        legacy_post_fixture(),
        CpsPostSource::Bundled { id: "grbl".into() },
    );
    assert_eq!(response.output_extension.as_deref(), Some("gcode"));
    assert!(response.gcode.contains("G21"));
    assert!(response.gcode.contains("M30"));
    // Canned cycles expand for GRBL: pecks appear as plain G0/G1.
    assert!(!response.gcode.contains("G83"));
    // Pipeline 55: the two-arc closed circle retains G2 on its second
    // pass instead of incorrectly reversing to G3 as an open contour.
    let circle = response
        .gcode
        .split("(object=1 level=0 pocket=0 segments=2 finish)")
        .nth(1)
        .unwrap()
        .split("(Drill pattern)")
        .next()
        .unwrap();
    assert_eq!(circle.matches("G2 X74.5 I-5.5 F1200").count(), 2);
    assert!(!circle.contains("G3"));
    let old_winding = response.gcode.replace(
        "G1 Z-3. F200\nG2 X74.5 I-5.5 F1200",
        "G1 Z-3. F200\nG3 X74.5 I-5.5 F1200",
    );
    assert_eq!(
        digest(&old_winding),
        0x8a33_59e1_1295_e3c3,
        "reviewed delta must only be second circle pass winding"
    );
    assert_digest("cps_grbl", &response.gcode, 0xfbfa_c637_7915_aeab);
}

#[test]
fn fanuc_refs_snapshot() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../refs/cam-posteditor/src/post-parser/test/test.cps");
    let Ok(script) = std::fs::read_to_string(&path) else {
        eprintln!("SKIP: FANUC oracle not present at {}", path.display());
        return;
    };
    let response = run_cps(
        legacy_post_fixture(),
        CpsPostSource::Inline {
            script,
            filename: Some("fanuc.cps".into()),
        },
    );
    assert_eq!(response.output_extension.as_deref(), Some("nc"));
    // Canned cycles stay canned for FANUC (peck drill → G83).
    assert!(response.gcode.contains("G83"), "{}", response.gcode);
    assert_digest("cps_fanuc", &response.gcode, 0xe140_d3c5_6872_3b27);
}

/// Geometry parity: re-interpreting the bundled post's TEXT lands on
/// the same geometry the IR-derived toolpath reports — total cut
/// length within 0.1% and identical final position. Catches
/// units/scaling regressions without byte-pinning.
#[test]
fn bundled_grbl_geometry_parity() {
    let response = run_cps(
        fixture_project(),
        CpsPostSource::Bundled { id: "grbl".into() },
    );
    let parsed = ivac_core::gcode::preview::interpret(&response.gcode);
    assert!(!parsed.is_empty() && !response.toolpath.is_empty());

    let cut_len = |segments: &[ivac_core::gcode::preview::ToolpathSegment]| -> f64 {
        segments
            .iter()
            .filter(|s| {
                !matches!(
                    s.kind,
                    ivac_core::gcode::preview::MoveKind::Rapid
                        | ivac_core::gcode::preview::MoveKind::Retract
                )
            })
            .map(|s| {
                let dx = s.to.x - s.from.x;
                let dy = s.to.y - s.from.y;
                let dz = s.to.z - s.from.z;
                (dx * dx + dy * dy + dz * dz).sqrt()
            })
            .sum()
    };
    let ir_len = cut_len(&response.toolpath);
    let text_len = cut_len(&parsed);
    let ratio = (ir_len - text_len).abs() / ir_len.max(1e-9);
    assert!(
        ratio < 1e-3,
        "cut-length divergence: IR {ir_len:.3} vs text {text_len:.3}"
    );

    let ir_end = response.toolpath.last().unwrap().to;
    let text_end = parsed.last().unwrap().to;
    assert!(
        (ir_end.x - text_end.x).abs() < 1e-3
            && (ir_end.y - text_end.y).abs() < 1e-3
            && (ir_end.z - text_end.z).abs() < 1e-3,
        "final position diverged: IR {ir_end:?} vs text {text_end:?}"
    );
}

/// Genuine two-sided CPS output remains unqualified (ivac-jtkj). Do not
/// accidentally bypass the work-origin guard while changing CPS execution.
#[test]
fn two_sided_cps_refuses_unqualified_output() {
    let mut project = fixture_project();
    let mut stock = stock();
    stock.flip = Some(FlipRegistration {
        axis: FlipAxis::X,
        ..FlipRegistration::default()
    });
    project.stock = Some(stock);
    project.operations[1].side = WorkpieceSide::Back;

    let result = run_pipeline_two_sided(
        PipelineRequest {
            project,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(selection(CpsPostSource::Bundled { id: "grbl".into() })),
        },
        |_, _, _| {},
    );
    assert!(matches!(
        result,
        Err(ivac_core::pipeline::PipelineError::WorkOrigin(message))
            if message.contains("two-sided CPS output")
    ));
}
