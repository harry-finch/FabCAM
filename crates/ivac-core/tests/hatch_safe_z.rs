//! Targeted post-audit regressions; no machine execution.
// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::float_cmp)]
mod common;
use common::{build_project, closed_rectangle, endmill_tool, op_single_pass, stock_at_origin};
use ivac_core::input::{import_bytes, ImportOptions};
use ivac_core::pipeline::{
    run_pipeline, stream_gcode_to_writer, PipelineRequest, PostProcessorKind,
};
use ivac_core::project::{ContourParams, OpKind, ProfileParams, ToolOffset};
use std::sync::{Arc, Mutex};

fn request(top: f64, clearance: f64) -> PipelineRequest {
    let mut stock = stock_at_origin(100., 100., 18.);
    stock.top_z_mm = top;
    let mut op = op_single_pass(
        1,
        "SAFE-Z witness",
        OpKind::Profile {
            offset: ToolOffset::Outside,
            contour: ContourParams::default(),
            profile: ProfileParams::default(),
        },
        1,
        top - 1.,
    );
    op.params.start_depth = top;
    op.params.step = Some(-1.);
    op.params.fast_move_z = clearance;
    PipelineRequest {
        project: build_project(
            &stock,
            vec![endmill_tool(1, 6.)],
            op,
            closed_rectangle(20., 20., 40., 40.),
        ),
        post_processor: Some(PostProcessorKind::Linuxcnc),
        cps_post: None,
    }
}

#[test]
fn hatch_block_is_refused_instead_of_imported_at_definition_coordinates() {
    let bytes =
        include_bytes!("../../../docs/releases/consolidation-2026-10-18-evidence/block-hatch.dxf");
    let result = import_bytes("block.dxf", bytes, &ImportOptions::default());
    assert!(
        result.is_err(),
        "misplaced BLOCK HATCH must not be a successful drawing"
    );
    let message = result.unwrap_err().to_string();
    assert!(
        message.contains("HATCH") && message.contains("BLOCK"),
        "{message}"
    );
}

#[test]
fn known_stock_refuses_clearance_at_or_below_top() {
    for clearance in [-5., 0., 5., 18., 18.000_001] {
        for work_area_known in [false, true] {
            let mut req = request(18., clearance);
            if !work_area_known {
                req.project.machine.work_area.z = 0.;
            }
            let result = run_pipeline(req, |_, _, _| {});
            assert!(result.is_err(), "top18/clearance{clearance} must refuse");
            assert!(result.unwrap_err().to_string().contains("clearance"));
        }
    }
}

#[derive(Clone)]
struct Capture(Arc<Mutex<Vec<u8>>>);
impl std::io::Write for Capture {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn invalid_clearance_stream_writes_no_bytes() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let result = stream_gcode_to_writer(request(18., 5.), Box::new(Capture(bytes.clone())));
    assert!(
        result.is_err(),
        "invalid clearance must fail before streaming"
    );
    assert!(bytes.lock().unwrap().is_empty());
}

#[test]
fn top_level_hatch_stays_supported_but_unused_block_is_also_refused() {
    let fixture = std::str::from_utf8(include_bytes!(
        "../../../docs/releases/consolidation-2026-10-18-evidence/block-hatch.dxf"
    ))
    .unwrap();
    let start = fixture.find("0\nHATCH\n").unwrap();
    let end = fixture.find("0\nENDBLK\n").unwrap();
    let hatch = &fixture[start..end];
    let top = format!("0\nSECTION\n2\nENTITIES\n{hatch}0\nENDSEC\n0\nEOF\n");
    let out = import_bytes("top.dxf", top.as_bytes(), &ImportOptions::default()).unwrap();
    assert_eq!(out.segments.len(), 4);
    assert_eq!((out.bbox.min_x, out.bbox.max_x), (0., 10.));
    let no_insert = &fixture[..fixture.find("0\nINSERT\n").unwrap()];
    let unused = format!("{no_insert}0\nENDSEC\n0\nEOF\n");
    assert!(import_bytes("unused.dxf", unused.as_bytes(), &ImportOptions::default()).is_err());
    // A HATCH string in a layer/value must not be mistaken for an entity.
    let top = top.replace("8\n0\n", "8\nBLOCK\n");
    assert!(import_bytes("layer.dxf", top.as_bytes(), &ImportOptions::default()).is_ok());
    // ASCII DXF entity keywords are still guarded with a legacy-encoded name.
    let mut legacy = fixture.as_bytes().to_vec();
    let name = fixture.find("HATCH_BLOCK").unwrap();
    legacy[name] = 0xe9;
    assert!(
        import_bytes("legacy.dxf", &legacy, &ImportOptions::default())
            .unwrap_err()
            .to_string()
            .contains("HATCH")
    );
}

#[test]
fn finite_and_disabled_and_unknown_stock_contracts() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(run_pipeline(request(18., value), |_, _, _| {}).is_err());
        assert!(run_pipeline(request(value, 23.), |_, _, _| {}).is_err());
    }
    let mut req = request(18., 23.);
    let mut disabled = req.project.operations[0].clone();
    disabled.id = 2;
    disabled.enabled = false;
    disabled.params.fast_move_z = 5.;
    req.project.operations.push(disabled);
    assert!(run_pipeline(req.clone(), |_, _, _| {}).is_ok());
    req.project.stock = None;
    req.project.operations[0].params.fast_move_z = 5.;
    assert!(
        run_pipeline(req, |_, _, _| {}).is_ok(),
        "unknown stock is not a new global prohibition"
    );
}

#[test]
fn stock_edit_is_revalidated_even_after_successful_cached_generation() {
    let req = request(0., 5.);
    let before = serde_json::to_value(&req.project).unwrap();
    assert!(run_pipeline(req.clone(), |_, _, _| {}).is_ok());
    assert!(run_pipeline(req.clone(), |_, _, _| {}).is_ok());
    let mut changed = req.clone();
    changed.project.stock.as_mut().unwrap().top_z_mm = 18.;
    let error = run_pipeline(changed, |_, _, _| {}).unwrap_err();
    assert!(error.to_string().contains("clearance"));
    assert!(error.to_structured(None).is_some());
    assert_eq!(before, serde_json::to_value(&req.project).unwrap());
    assert!(run_pipeline(req, |_, _, _| {}).is_ok());
}

#[cfg(feature = "cps")]
fn shopbot(mut req: PipelineRequest) -> PipelineRequest {
    use ivac_core::pipeline::{CpsPostSelection, CpsPostSource};
    req.post_processor = Some(PostProcessorKind::Cps);
    req.cps_post = Some(CpsPostSelection {
        source: CpsPostSource::Bundled {
            id: "shopbot".into(),
        },
        properties: std::collections::BTreeMap::default(),
    });
    req
}

#[cfg(feature = "cps")]
#[test]
fn shopbot_decoded_lateral_rapids_stay_above_stock_with_two_clearances() {
    for top in [0., 18.] {
        let mut req = shopbot(request(top, top + 5.));
        req.project.work_offset.x_mm = 10.;
        req.project.work_offset.y_mm = 10.;
        let mut next = req.project.operations[0].clone();
        next.id = 2;
        next.params.fast_move_z = top + 8.;
        next.tool_id = 2;
        req.project.tools.push(endmill_tool(2, 6.));
        req.project.operations.push(next);
        let r = run_pipeline(req, |_, _, _| {}).unwrap();
        // Independent minimal OpenSBP state oracle: initial XY deliberately unknown.
        let mut z: Option<f64> = None;
        let mut lateral_count = 0;
        for line in r.gcode.lines() {
            let fields: Vec<_> = line.split(',').collect();
            match fields[0] {
                "JZ" => {
                    z = Some(fields[1].parse().unwrap());
                    assert!(z.unwrap() > top);
                }
                "J2" => {
                    assert!(z.is_some_and(|v| v > top));
                    lateral_count += 1;
                }
                "J3" => {
                    let next: f64 = fields[3].parse().unwrap();
                    assert!(z.is_some_and(|v| v > top) && next > top);
                    z = Some(next);
                    lateral_count += 1;
                }
                "M3" => z = Some(fields[3].parse().unwrap()),
                _ => {}
            }
        }
        assert!(lateral_count >= 2);
        // Known-position preview rapids must either be in air or vertical upward retracts.
        for s in r
            .toolpath
            .iter()
            .filter(|s| s.kind == ivac_core::gcode::preview::MoveKind::Rapid)
        {
            let vertical = s.from.x == s.to.x && s.from.y == s.to.y;
            assert!(
                (vertical && s.to.z >= s.from.z && s.to.z > top)
                    || (s.from.z > top && s.to.z > top)
            );
        }
    }
}

#[cfg(feature = "cps")]
#[test]
fn shopbot_invalid_stock_clearance_and_existing_z_origin_refuse() {
    assert!(run_pipeline(shopbot(request(18., 5.)), |_, _, _| {})
        .unwrap_err()
        .to_string()
        .contains("clearance"));
    let mut req = shopbot(request(18., 23.));
    req.project.work_offset.z_mm = 18.;
    assert!(run_pipeline(req, |_, _, _| {})
        .unwrap_err()
        .to_string()
        .contains("numeric Z"));
}

#[cfg(feature = "cps")]
#[test]
fn shopbot_parking_stays_explicitly_unsupported() {
    let mut req = shopbot(request(18., 23.));
    req.project.machine.park_xy = Some((80., 80.));
    assert!(run_pipeline(req, |_, _, _| {})
        .unwrap_err()
        .to_string()
        .contains("parking"));
}

#[test]
fn generic_parking_and_stream_preview_retain_safe_air_traversals() {
    use ivac_core::pipeline::stream_gcode_with_preview;
    let mut req = request(18., 23.);
    req.project.machine.park_xy = Some((80., 80.));
    let out = run_pipeline(req.clone(), |_, _, _| {}).unwrap();
    for segment in out
        .toolpath
        .iter()
        .filter(|s| s.kind == ivac_core::gcode::preview::MoveKind::Rapid)
    {
        if segment.from.x != segment.to.x || segment.from.y != segment.to.y {
            assert!(segment.from.z > 18. && segment.to.z > 18.);
        }
    }
    let bytes = Arc::new(Mutex::new(Vec::new()));
    assert!(stream_gcode_with_preview(req, Box::new(Capture(bytes.clone()))).is_ok());
    assert!(!bytes.lock().unwrap().is_empty());
    let bytes = Arc::new(Mutex::new(Vec::new()));
    assert!(stream_gcode_with_preview(request(18., 5.), Box::new(Capture(bytes.clone()))).is_err());
    assert!(bytes.lock().unwrap().is_empty());
}

#[test]
fn header_only_job_cannot_bypass_stock_guard() {
    let mut req = request(18., 23.);
    req.project.operations[0].kind = OpKind::Pause {
        message: "inspect".into(),
    };
    assert!(run_pipeline(req, |_, _, _| {})
        .unwrap_err()
        .to_string()
        .contains("clearance"));
}

#[test]
fn inch_serialization_guard_and_shared_process_boundary() {
    use ivac_core::project::{MachineMode, UnitSystem};
    let mut req = request(18., 18.001);
    req.project.machine.unit = UnitSystem::Inch;
    assert!(run_pipeline(req.clone(), |_, _, _| {}).is_err());
    req.project.operations[0].params.fast_move_z = 23.;
    assert!(run_pipeline(req, |_, _, _| {}).is_ok());
    let mut req = request(18., 5.);
    req.project.machine.mode = MachineMode::Laser;
    assert!(
        run_pipeline(req, |_, _, _| {}).is_ok(),
        "milling guard must not reinterpret laser height"
    );
}

#[cfg(feature = "cps")]
#[test]
fn supported_pocket_and_drill_keep_air_clearance() {
    use ivac_core::project::{DrillCycle, PocketParams, PocketStrategy};
    for kind in [
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
        OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: 0.5,
                dwell_sec: 0.,
            },
            chamfer_after_width_mm: None,
            pattern: None,
            spot_first: None,
        },
    ] {
        let mut req = shopbot(request(0., 5.));
        req.project.operations[0].kind = kind;
        let out = run_pipeline(req, |_, _, _| {}).unwrap();
        for s in out
            .toolpath
            .iter()
            .filter(|s| s.kind == ivac_core::gcode::preview::MoveKind::Rapid)
        {
            if s.from.x != s.to.x || s.from.y != s.to.y {
                assert!(s.from.z > 0. && s.to.z > 0.);
            }
        }
    }
}

#[test]
fn custom_identity_z_rounding_must_not_erase_clearance() {
    use ivac_core::gcode::post_profile::{AxesConfig, PostProfile};
    let mut req = request(18., 18.0002);
    let mut profile = PostProfile::linuxcnc_default();
    profile.axes = Some(AxesConfig::default()); // standard identity Z, three decimals
    req.project.machine.post_profile = Some(profile);
    assert!(run_pipeline(req.clone(), |_, _, _| {}).is_err());
    req.project.operations[0].params.fast_move_z = 23.;
    assert!(run_pipeline(req, |_, _, _| {}).is_ok());
}
