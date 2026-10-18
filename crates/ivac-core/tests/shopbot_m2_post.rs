//! M2: actual import -> CAM -> CPS -> OpenSBP, with independent text semantics.
#![cfg(feature = "cps")]
mod common;
use common::{build_project, drill_tool, endmill_tool, op_single_pass, stock_at_origin};
use ivac_core::input::{import_bytes, ImportOptions};
use ivac_core::pipeline::{
    run_pipeline, CpsPostSelection, CpsPostSource, PipelineRequest, PipelineResponse,
    PostProcessorKind,
};
use ivac_core::project::{
    ContourParams, DrillCycle, OpKind, PocketParams, PocketStrategy, ProfileParams, Project,
    ToolChangeStrategy, ToolOffset,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn project(case: &str, format: &str) -> Project {
    let (name, kind) = match case {
        "square" => ("square_100", profile()),
        "circle" => ("circle_d50", profile()),
        "pocket" => (
            "rectangle_with_island",
            OpKind::Pocket {
                strategy: PocketStrategy::Cascade,
                contour: ContourParams::default(),
                pocket: PocketParams {
                    pocket_islands: true,
                    xy_overlap: 0.5,
                    ..Default::default()
                },
            },
        ),
        "drill" | "multiple" => (
            "circle_d50",
            OpKind::Drill {
                cycle: DrillCycle::Peck {
                    peck_step_mm: 2.0,
                    dwell_sec: 0.2,
                },
                chamfer_after_width_mm: None,
                pattern: None,
                spot_first: None,
            },
        ),
        _ => panic!("unknown case"),
    };
    let file = format!("{name}.{format}");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/shopbot-m1")
        .join(&file);
    let segments = import_bytes(
        &file,
        &std::fs::read(path).unwrap(),
        &ImportOptions::default(),
    )
    .unwrap()
    .segments;
    let mut op = op_single_pass(1, case, kind, 1, -5.0);
    op.params.step = Some(-2.0);
    if case == "drill" || case == "multiple" {
        op.params.start_depth = 1.5;
    }
    let tool = if case == "drill" || case == "multiple" {
        drill_tool(1, 6., 118.)
    } else {
        endmill_tool(1, 6.)
    };
    let mut p = build_project(&stock_at_origin(140., 140., 10.), vec![tool], op, segments);
    if case == "multiple" {
        p.tools.push(endmill_tool(2, 6.));
        let mut op = op_single_pass(2, "Circle after drill", profile(), 2, -5.);
        op.params.step = Some(-2.);
        p.operations.push(op);
    }
    p
}
fn profile() -> OpKind {
    OpKind::Profile {
        offset: ToolOffset::Outside,
        contour: ContourParams::default(),
        profile: ProfileParams::default(),
    }
}
fn post(p: Project) -> Result<PipelineResponse, ivac_core::pipeline::PipelineError> {
    run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: BTreeMap::new(),
            }),
        },
        |_, _, _| {},
    )
}
#[derive(Debug)]
struct Move {
    from: [f64; 3],
    to: [f64; 3],
    rapid: bool,
    speed: f64,
}
fn parse(text: &str) -> Vec<Move> {
    assert!(text.is_ascii());
    assert!(text.starts_with("' ivaCAM ShopBot"));
    assert!(text.contains("\nVD,,1\nSA\n"));
    assert!(text.ends_with("END\n"));
    let mut pos = [0., 0., 5.];
    let mut speed = 0.;
    let mut moves = Vec::new();
    for line in text.lines() {
        if line.starts_with("' ")
            || line == "PAUSE"
            || line.starts_with("PAUSE ")
            || line == "VD,,1"
            || line == "SA"
            || line == "END"
        {
            continue;
        }
        let mut fields = line.split(',');
        let cmd = fields.next().unwrap();
        let values: Vec<f64> = fields
            .map(|s| s.parse::<f64>().expect("plain numeric command parameter"))
            .collect();
        assert!(values.iter().all(|v| v.is_finite()));
        let from = pos;
        match cmd {
            "MS" => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0], values[1]);
                assert!(values[0] > 0.);
                speed = values[0];
                continue;
            }
            "JZ" => {
                assert_eq!(values.len(), 1);
                pos[2] = values[0]
            }
            "J2" => {
                assert_eq!(values.len(), 2);
                assert!(pos[2] > 0., "XY jog below stock");
                pos[0] = values[0];
                pos[1] = values[1]
            }
            "M3" => {
                assert_eq!(values.len(), 3);
                assert!(speed > 0.);
                pos.copy_from_slice(&values)
            }
            _ => panic!("undocumented command {line}"),
        }
        moves.push(Move {
            from,
            to: pos,
            rapid: cmd != "M3",
            speed,
        });
    }
    assert!(moves.iter().any(|m| !m.rapid && m.to[2] < 0.));
    assert!((moves.last().unwrap().to[2] - 5.).abs() < 0.0001);
    moves
}
fn golden(case: &str, text: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/shopbot-m2")
        .join(format!("{case}.sbp"));
    if std::env::var_os("IVAC_UPDATE_SHOPBOT_GOLDENS").is_some() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(path).expect("required reviewed SBP golden")
    );
}
fn check(case: &str) {
    let a = post(project(case, "svg")).unwrap();
    let b = post(project(case, "svg")).unwrap();
    let dxf = post(project(case, "dxf")).unwrap();
    assert_eq!(a.output_extension.as_deref(), Some("sbp"));
    assert_eq!(a.gcode, b.gcode);
    assert!(a
        .warnings
        .iter()
        .any(|w| w.message.contains("manual spindle")));
    for text in [&a.gcode, &dxf.gcode] {
        let moves = parse(text);
        let cut: Vec<_> = moves.iter().filter(|m| !m.rapid && m.to[2] < 0.).collect();
        if case == "drill" {
            let tip = -5. - 3. / 59_f64.to_radians().tan();
            let mut bottom = 0_f64;
            let mut retract = true;
            let mut pecks = 0;
            for m in &moves {
                if m.to[2] < 0. {
                    assert!((m.to[0] - 60.).abs() < 0.0001 && (m.to[1] - 60.).abs() < 0.0001);
                    if m.rapid {
                        assert!(m.to[2] >= bottom - 0.0001, "rapid enters uncut drill depth");
                    }
                }
                if m.to[2] >= 1.5 - 0.0001 {
                    retract = true
                }
                if !m.rapid && m.to[2] < bottom - 0.0001 {
                    assert!(retract, "missing full peck retract");
                    assert!(bottom - m.to[2] <= 2.0001);
                    assert!((m.to[0] - 60.).abs() < 0.0001 && (m.to[1] - 60.).abs() < 0.0001);
                    assert!((m.speed - 200. / 60.).abs() < 0.000001);
                    bottom = m.to[2];
                    retract = false;
                    pecks += 1;
                }
            }
            assert!((bottom - tip).abs() < 0.0001 && pecks >= 4);
            assert!(text.contains("PAUSE 0.200"));
        } else {
            if case != "multiple" {
                assert!(
                    moves.iter().all(|m| m.to[2] >= -5.0001),
                    "exported over-depth"
                );
                for z in [-2., -4., -5.] {
                    assert!(cut.iter().any(|m|(m.to[2]-z).abs()<0.0001 && (m.from[2]-z).abs()<0.0001),"missing exported depth pass");
                }
            }
            assert!(cut.iter().any(|m| (m.to[2] + 5.).abs() < 0.0001));
            assert!(cut.iter().any(|m| (m.speed - 20.).abs() < 0.000001));
            if case == "circle" {
                for m in cut.iter().filter(|m| (m.from[2] - m.to[2]).abs() < 0.0001) {
                    let r = (m.to[0] - 60.).hypot(m.to[1] - 60.);
                    assert!((r - 28.).abs() < 0.0202);
                    let mid = [(m.to[0] + m.from[0]) / 2., (m.to[1] + m.from[1]) / 2.];
                    assert!(
                        (mid[0] - 60.).hypot(mid[1] - 60.) >= 28. - 0.0302,
                        "arc chord exceeds tolerance"
                    );
                }
            }
            if case == "pocket" {
                let floor: Vec<_> = cut
                    .iter()
                    .filter(|m| (m.from[2] + 5.).abs() < 0.0001 && (m.to[2] + 5.).abs() < 0.0001)
                    .collect();
                let island_distance = |x: f64, y: f64| {
                    (50. - x)
                        .max(0.)
                        .max(x - 70.)
                        .hypot((50. - y).max(0.).max(y - 70.))
                };
                for m in &cut {
                    if m.from[2] >= 0. {
                        continue;
                    }
                    for t in [0., 0.25, 0.5, 0.75, 1.] {
                        let x = m.from[0] + t * (m.to[0] - m.from[0]);
                        let y = m.from[1] + t * (m.to[1] - m.from[1]);
                        assert!((x - 10.).min(110. - x).min(y - 10.).min(110. - y) >= 2.9798);
                        assert!(
                            island_distance(x, y) >= 2.9798,
                            "exported cut intrudes into island"
                        );
                    }
                }
                let mut checked = 0;
                for x in (14..107).step_by(2) {
                    for y in (14..107).step_by(2) {
                        let x = f64::from(x);
                        let y = f64::from(y);
                        if island_distance(x, y) < 3.5 {
                            continue;
                        }
                        let min_distance = floor
                            .iter()
                            .map(|m| {
                                let dx = m.to[0] - m.from[0];
                                let dy = m.to[1] - m.from[1];
                                let len = dx * dx + dy * dy;
                                let t = if len > 0. {
                                    ((x - m.from[0]) * dx + (y - m.from[1]) * dy) / len
                                } else {
                                    0.
                                };
                                let t = t.clamp(0., 1.);
                                (x - m.from[0] - t * dx).hypot(y - m.from[1] - t * dy)
                            })
                            .fold(f64::INFINITY, f64::min);
                        assert!(
                            min_distance <= 3.0002,
                            "exported floor uncovered at {x},{y}"
                        );
                        checked += 1;
                    }
                }
                assert!(checked > 1000);
            }
            if case == "square" {
                let xs: Vec<_> = cut
                    .iter()
                    .filter(|m| (m.to[2] + 5.).abs() < 0.0001)
                    .map(|m| m.to[0])
                    .collect();
                assert!((xs.iter().copied().fold(f64::INFINITY, f64::min) - 7.).abs() < 0.0001);
                let ys: Vec<_> = cut
                    .iter()
                    .filter(|m| (m.to[2] + 5.).abs() < 0.0001)
                    .map(|m| m.to[1])
                    .collect();
                assert!((ys.iter().copied().fold(f64::INFINITY, f64::min) - 7.).abs() < 0.0001);
                assert!(
                    (ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) - 113.).abs() < 0.0001
                );
                assert!(
                    (xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - 113.).abs() < 0.0001
                );
            }
        }
        if case == "multiple" {
            let first = text
                .find("' drill")
                .or_else(|| text.find("' multiple"))
                .unwrap();
            let second = text.find("' Circle after drill").unwrap();
            assert!(first < second);
            let between = &text[first..second];
            assert!(between.contains("Stop spindle manually") && between.contains("Load tool 2"));
            assert!(text[..second].ends_with("RPM. Wait until ready before OK.\nPAUSE\n"));
        }
    }
    // Validate decoded geometry/coverage before accepting a reference update.
    golden(case, &a.gcode);
}
#[test]
fn square_profile() {
    check("square")
}
#[test]
fn circular_profile() {
    check("circle")
}
#[test]
fn island_pocket() {
    check("pocket")
}
#[test]
fn peck_drilling() {
    check("drill")
}
#[test]
fn multiple_operations() {
    check("multiple")
}
#[test]
fn unsupported_machine_policies_fail() {
    for policy in 0..8 {
        let mut p = project("square", "svg");
        match policy {
            0 => p.machine.tool_change = ToolChangeStrategy::Atc,
            1 => p.machine.tool_change = ToolChangeStrategy::Ignore,
            2 => p.machine.park_xy = Some((0., 0.)),
            3 => p.machine.toolchange_xy = Some((0., 0.)),
            4 => p.machine.use_tool_length_offsets = true,
            6 => p.tools[0].z_shift_mm = Some(1.),
            7 => p.machine.post_change_z = ivac_core::project::PostChangeZStrategy::ManualTouchoff,
            _ => p.tools[0].coolant = ivac_core::project::Coolant::Flood,
        }
        let err = post(p).unwrap_err().to_string();
        assert!(err.contains("ShopBot MVP"), "{err}");
    }
}

#[test]
fn m3_unverified_clearance_refuses_sbp_export() {
    let snapshot: serde_json::Value = serde_json::from_str(include_str!(
        "../../../machines/sorbonne-shopbot-prs.ivac-machine.json"
    ))
    .unwrap();
    let mut p = project("square", "svg");
    let clearance = snapshot["payload"]["fastMoveZ"].as_f64().unwrap();
    assert_eq!(clearance, 0.0, "unverified sentinel, not a safe height");
    for op in &mut p.operations {
        op.params.fast_move_z = clearance;
    }
    // Known stock now fails earlier in the shared milling preflight.
    assert!(matches!(
        post(p.clone()),
        Err(ivac_core::pipeline::PipelineError::UnsafeClearance(1, _))
    ));
    // The existing ShopBot backstop still applies without a stock model.
    p.stock = None;
    let err = post(p).unwrap_err().to_string();
    assert!(
        err.contains("positive authored clearance Z required"),
        "{err}"
    );
}

#[test]
fn m4_lab_spindle_runs_real_cam_and_preserves_manual_default() {
    let p = project("square", "svg");
    let manual = post(p.clone()).unwrap();
    let mut properties = BTreeMap::new();
    properties.insert(
        "labSpindle1Control".into(),
        ivac_core::pipeline::CpsParamValue::Bool(true),
    );
    let lab = run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties,
            }),
        },
        |_, _, _| {},
    )
    .unwrap();
    let movement_lines = |text: &str| {
        text.lines()
            .filter(|l| l.starts_with("M3,") || l.starts_with("J2,") || l.starts_with("MS,"))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(movement_lines(&manual.gcode), movement_lines(&lab.gcode));
    assert!(lab.gcode.contains("TR,18000\nSO,4,1\nSO,1,1\nPAUSE 2.000"));
    assert!(lab.gcode.ends_with("SO,1,0\nEND\n"));
    assert!(!lab.gcode.lines().any(|l| l == "PAUSE"));
    assert!(!manual.gcode.lines().any(|l| l.starts_with("SO,")));
}

#[test]
fn ux1_curved_ramp_exports_instead_of_rejecting_varying_z_arc() {
    let mut p = project("circle", "svg");
    p.operations[0].params.plunge = ivac_core::project::PlungeStrategy::Ramp { angle_deg: 3.0 };
    let result = post(p);
    assert!(
        result.is_ok(),
        "curved ramp must preserve XYZ through ShopBot linearization: {result:?}"
    );
    let result = result.unwrap();
    let movements = parse(&result.gcode);
    assert!(movements.iter().any(|m| !m.rapid
        && m.to[2] < m.from[2]
        && ((m.to[0] - m.from[0]).abs() > 1e-5 || (m.to[1] - m.from[1]).abs() > 1e-5)));
}

fn ux1_fixture(name: &str, kind: OpKind) -> Project {
    let file = format!("{name}.svg");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/shopbot-m1")
        .join(&file);
    let geometry = import_bytes(
        &file,
        &std::fs::read(path).unwrap(),
        &ImportOptions::default(),
    )
    .unwrap();
    let mut op = op_single_pass(1, "UX1 SOFTWARE ONLY", kind, 1, -5.0);
    op.params.step = Some(-2.0);
    op.params.plunge = ivac_core::project::PlungeStrategy::Ramp { angle_deg: 30.0 };
    build_project(
        &stock_at_origin(140., 140., 10.),
        vec![endmill_tool(1, 6.)],
        op,
        geometry.segments,
    )
}
fn ux1_ramp_invariants(response: &PipelineResponse) {
    let moves = parse(&response.gcode);
    let cuts: Vec<_> = moves.iter().filter(|m| !m.rapid).collect();
    assert!(cuts.iter().any(|m| (m.to[2] + 5.).abs() < 0.0001));
    assert!(cuts.iter().all(|m| m.to[2] >= -5.0001));
    let descending: Vec<_> = cuts
        .iter()
        .filter(|m| m.to[2] < m.from[2] && m.to[2] < 0.)
        .collect();
    assert!(!descending.is_empty());
    for m in descending {
        assert!(
            (m.to[0] - m.from[0]).hypot(m.to[1] - m.from[1]) > 0.00005,
            "hidden vertical fallback: {m:?}"
        );
        assert!(
            m.from[2].min(0.) - m.to[2] <= 2.0001,
            "depth increment: {m:?}"
        );
    }
}
#[test]
fn ux1_ramps_qualify_simple_curved_concave_open_and_island_sources() {
    for name in [
        "square_100",
        "circle_d50",
        "concave_L_shape",
        "open_polyline",
    ] {
        let kind = if name == "open_polyline" {
            OpKind::Profile {
                offset: ToolOffset::On,
                contour: ContourParams::default(),
                profile: ProfileParams::default(),
            }
        } else {
            profile()
        };
        let p = ux1_fixture(name, kind);
        let before = serde_json::to_value(&p).unwrap();
        let result = post(p.clone()).unwrap_or_else(|e| panic!("{name}: {e}"));
        ux1_ramp_invariants(&result);
        assert_eq!(serde_json::to_value(&p).unwrap(), before);
        assert_eq!(post(p).unwrap().gcode, result.gcode);
    }
    for name in ["square_100", "rectangle_with_island", "concave_L_shape"] {
        let p = ux1_fixture(
            name,
            OpKind::Pocket {
                strategy: PocketStrategy::Cascade,
                contour: ContourParams::default(),
                pocket: PocketParams {
                    pocket_islands: true,
                    xy_overlap: 0.5,
                    ..Default::default()
                },
            },
        );
        let result = post(p).unwrap_or_else(|e| panic!("pocket {name}: {e}"));
        ux1_ramp_invariants(&result);
        if name == "rectangle_with_island" {
            for m in parse(&result.gcode)
                .into_iter()
                .filter(|m| !m.rapid && m.to[2] < 0.)
            {
                for k in 0..=10 {
                    let t = f64::from(k) / 10.;
                    let x = m.from[0] + (m.to[0] - m.from[0]) * t;
                    let y = m.from[1] + (m.to[1] - m.from[1]) * t;
                    assert!((12.9898..=107.0102).contains(&x) && (12.9898..=107.0102).contains(&y));
                    let dx = (50. - x).max(0.).max(x - 70.);
                    let dy = (50. - y).max(0.).max(y - 70.);
                    assert!(dx.hypot(dy) >= 2.9898, "island cutter clearance at {x},{y}");
                }
            }
        }
    }
}
#[test]
fn ux1_circular_ramp_chords_obey_post_tolerance_and_direction() {
    use ivac_core::project::CutDirection;
    for (offset, radius) in [(ToolOffset::Outside, 28.), (ToolOffset::Inside, 22.)] {
        for direction in [CutDirection::Climb, CutDirection::Conventional] {
            let mut p = project("circle", "svg");
            if let OpKind::Profile { offset: side, .. } = &mut p.operations[0].kind {
                *side = offset;
            }
            p.operations[0].params.plunge =
                ivac_core::project::PlungeStrategy::Ramp { angle_deg: 3.0 };
            let contour = p.operations[0].contour_params_mut().unwrap();
            contour.cut_direction = direction;
            contour.finish_cut_direction = direction;
            let response = post(p).unwrap();
            let moves = parse(&response.gcode);
            let floor: Vec<_> = moves
                .iter()
                .filter(|m| {
                    !m.rapid && (m.from[2] + 5.).abs() < 0.0001 && (m.to[2] + 5.).abs() < 0.0001
                })
                .collect();
            assert!(floor.len() > 20);
            let mut area = 0.;
            for m in &floor {
                area += m.from[0] * m.to[1] - m.to[0] * m.from[1];
                for t in [0., 0.5, 1.] {
                    let x = m.from[0] + (m.to[0] - m.from[0]) * t;
                    let y = m.from[1] + (m.to[1] - m.from[1]) * t;
                    assert!(((x - 60.).hypot(y - 60.) - radius).abs() <= 0.0102);
                }
            }
            assert_eq!(
                area > 0.,
                (direction == CutDirection::Climb) == (offset == ToolOffset::Outside)
            );
            for movement in moves.iter().filter(|m| !m.rapid && m.to[2] < 0.) {
                assert!(
                    response
                        .toolpath
                        .iter()
                        .any(|segment| ux1_preview_distance(segment, movement.to) <= 0.0102),
                    "SBP/IR XYZ disagreement at {:?}",
                    movement.to
                );
            }

            for segment in response
                .toolpath
                .iter()
                .filter(|s| (s.to.z + 5.).abs() < 1e-6)
            {
                assert!(
                    ((segment.to.x - 60.).hypot(segment.to.y - 60.) - radius).abs() <= 0.0102,
                    "preview circle point: {segment:?}"
                );
            }
        }
    }
}

#[test]
fn ux1_unqualified_entry_and_origin_combinations_fail_explicitly() {
    for scenario in [
        "helix",
        "unfittable",
        "ramp-tabs",
        "ramp-leads",
        "short",
        "wcs-z",
        "G55",
    ] {
        let mut p = project("square", "svg");
        p.operations[0].params.plunge = ivac_core::project::PlungeStrategy::Ramp { angle_deg: 3.0 };
        match scenario {
            "helix" | "unfittable" => {
                p.operations[0].params.plunge = ivac_core::project::PlungeStrategy::Helix {
                    angle_deg: 3.,
                    radius_mm: Some(if scenario == "helix" { 5. } else { 500. }),
                }
            }
            "ramp-tabs" => {
                p.operations[0].contour_params_mut().unwrap().tab_mode =
                    ivac_core::project::TabPlacementMode::Auto { count: 2 }
            }
            "ramp-leads" => {
                p.operations[0].contour_params_mut().unwrap().leads.r#in =
                    ivac_core::project::LeadKind::Straight
            }
            "short" => {
                p.segments = common::closed_rectangle(10., 10., 11., 11.);
                p.operations[0].params.plunge =
                    ivac_core::project::PlungeStrategy::Ramp { angle_deg: 0.01 }
            }
            "wcs-x" => p.work_offset.x_mm = 10.,
            "wcs-y" => p.work_offset.y_mm = -5.,
            "wcs-z" => p.work_offset.z_mm = 2.,
            "G55" => p.work_offset.wcs = ivac_core::project::Wcs::G55,
            _ => unreachable!(),
        }
        let before = serde_json::to_value(&p).unwrap();
        let err = post(p.clone()).unwrap_err().to_string();
        assert!(
            err.contains(if scenario == "wcs-z" {
                "work origin"
            } else {
                "ShopBot MVP"
            }),
            "{scenario}: {err}"
        );
        if scenario != "G55" && scenario != "wcs-z" {
            assert!(err.contains("operation 1"), "{scenario}: {err}");
        }
        assert_eq!(serde_json::to_value(&p).unwrap(), before);
    }
}
#[test]
fn ux1_direct_tabs_and_leads_remain_explicit_supported_choices() {
    for tab_type in [
        ivac_core::project::TabType::Rectangle,
        ivac_core::project::TabType::Ramp,
    ] {
        let mut p = project("square", "svg");
        let contour = p.operations[0].contour_params_mut().unwrap();
        contour.tab_mode = ivac_core::project::TabPlacementMode::Auto { count: 2 };
        contour.tabs.height = 1.;
        contour.tabs.width = 10.;
        contour.tabs.tab_type = tab_type;
        contour.leads.r#in = ivac_core::project::LeadKind::Straight;
        contour.leads.out = ivac_core::project::LeadKind::Straight;
        let response = post(p).unwrap();
        let moves = parse(&response.gcode);
        assert!(moves
            .iter()
            .any(|m| !m.rapid && (m.to[2] + 4.).abs() < 0.0001));
        assert!(moves.iter().all(|m| m.to[2] >= -5.0001));
        let mut protected = 0;
        for m in moves.iter().filter(|m| !m.rapid) {
            let dx = m.to[0] - m.from[0];
            let dy = m.to[1] - m.from[1];
            let length2 = dx * dx + dy * dy;
            let t = if length2 > 1e-12 {
                (((15. - m.from[0]) * dx + (7. - m.from[1]) * dy) / length2).clamp(0., 1.)
            } else {
                0.
            };
            let distance = (m.from[0] + t * dx - 15.).hypot(m.from[1] + t * dy - 7.);
            if distance < 1.5 {
                protected += 1;
                assert!(
                    m.from[2] + t * (m.to[2] - m.from[2]) >= -4.0001,
                    "tab center removed: {m:?}"
                );
            }
        }
        assert!(protected > 0, "tab preservation check must be nonvacuous");
    }
}

#[test]
fn ux1_explicit_geometry_and_stock_translation_preserves_entry_preview() {
    let mut base = project("square", "svg");
    base.operations[0].params.plunge = ivac_core::project::PlungeStrategy::Ramp { angle_deg: 3.0 };
    let baseline = post(base.clone()).unwrap();
    let mut moved = base;
    for s in &mut moved.segments {
        s.start.x += 20.;
        s.end.x += 20.;
        s.start.y += 30.;
        s.end.y += 30.;
        if let Some(c) = &mut s.center {
            c.x += 20.;
            c.y += 30.;
        }
    }
    moved.stock.as_mut().unwrap().origin = [20., 30.];
    let shifted = post(moved).unwrap();
    ux1_ramp_invariants(&shifted);
    let a: Vec<_> = parse(&baseline.gcode)
        .into_iter()
        .filter(|m| !m.rapid)
        .collect();
    let b: Vec<_> = parse(&shifted.gcode)
        .into_iter()
        .filter(|m| !m.rapid)
        .collect();
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(b.iter()) {
        assert!((b.to[0] - a.to[0] - 20.).abs() < 0.0002);
        assert!((b.to[1] - a.to[1] - 30.).abs() < 0.0002);
        assert!((b.to[2] - a.to[2]).abs() < 0.0002);
    }
}

// Compare XYZ to analytic arc metadata, not to the preview's coarse display chords.
fn ux1_preview_distance(s: &ivac_core::gcode::preview::ToolpathSegment, p: [f64; 3]) -> f64 {
    let a = [s.from.x, s.from.y, s.from.z];
    let b = [s.to.x, s.to.y, s.to.z];
    let distance = |q: [f64; 3]| (p[0] - q[0]).hypot(p[1] - q[1]).hypot(p[2] - q[2]);
    let endpoints = distance(a).min(distance(b));
    if let Some(arc) = s.arc {
        let theta = |q: [f64; 3]| (q[1] - arc.cy).atan2(q[0] - arc.cx);
        let progress = |t: f64| {
            if arc.ccw {
                (t - theta(a)).rem_euclid(std::f64::consts::TAU)
            } else {
                (theta(a) - t).rem_euclid(std::f64::consts::TAU)
            }
        };
        let total = progress(theta(b));
        if total <= 1e-12 {
            return endpoints;
        }
        let t = progress(theta(p)) / total;
        if t > 1. {
            return endpoints;
        }
        let r = (a[0] - arc.cx).hypot(a[1] - arc.cy);
        let radial = ((p[0] - arc.cx).hypot(p[1] - arc.cy) - r).abs();
        let z = a[2] + (b[2] - a[2]) * t;
        endpoints.min(radial.hypot(p[2] - z))
    } else {
        let v = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let norm = v.iter().map(|x| x * x).sum::<f64>();
        if norm <= 1e-12 {
            return endpoints;
        }
        let t = ((p[0] - a[0]) * v[0] + (p[1] - a[1]) * v[1] + (p[2] - a[2]) * v[2]) / norm;
        let t = t.clamp(0., 1.);
        distance([a[0] + t * v[0], a[1] + t * v[1], a[2] + t * v[2]])
    }
}

// Independent SBP decoder above is the oracle, not the production preview parser.
#[test]
fn ux1_final_preview_matches_emitted_xyz_lines_for_profile_and_pocket() {
    for kind in [
        profile(),
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
    ] {
        for ramp in [false, true] {
            let mut p = ux1_fixture("square_100", kind.clone());
            p.operations[0].params.plunge = if ramp {
                ivac_core::project::PlungeStrategy::Ramp { angle_deg: 30. }
            } else {
                ivac_core::project::PlungeStrategy::Direct
            };
            let before = serde_json::to_value(&p).unwrap();
            let response = post(p.clone()).unwrap();
            let decoded = parse(&response.gcode);
            // First clearance/XY placement starts at an unknown operator XY.
            // All subsequent nonzero moves, including split rapid axes, match.
            let first_cut = decoded.iter().position(|m| !m.rapid).unwrap();
            let expected: Vec<_> = decoded[first_cut..]
                .iter()
                .filter(|m| m.from != m.to)
                .collect();
            let preview = &response.toolpath;
            assert_eq!(preview.len(), expected.len());
            for (segment, movement) in preview.iter().zip(expected) {
                assert_eq!(
                    [segment.from.x, segment.from.y, segment.from.z],
                    movement.from
                );
                assert_eq!([segment.to.x, segment.to.y, segment.to.z], movement.to);
                assert_eq!(
                    segment.kind == ivac_core::gcode::preview::MoveKind::Rapid,
                    movement.rapid
                );
                assert!(
                    segment.arc.is_none(),
                    "machine follows straight SBP XYZ chords"
                );
                assert!(segment.gcode_line > 0);
                assert_eq!(segment.op_id, 1);
                assert_eq!(
                    response.gcode_index.lines_to_segment[segment.gcode_line as usize],
                    preview
                        .iter()
                        .position(|s| std::ptr::eq(s, segment))
                        .unwrap() as u32
                );
                if movement.rapid && movement.to[0..2] != movement.from[0..2] {
                    assert!(movement.from[2] >= 5. && movement.to[2] >= 5.);
                }
            }
            if ramp {
                let diagnostic = response
                    .warnings
                    .iter()
                    .find(|w| w.kind == "entry_representation")
                    .unwrap();
                assert_eq!(diagnostic.params["requested"], "ramp");
                assert_eq!(diagnostic.params["target"], "shopbot");
                assert_eq!(diagnostic.params["preview_source"], "emitted_output");
            }
            assert_eq!(serde_json::to_value(&p).unwrap(), before);
            assert_eq!(post(p).unwrap().gcode, response.gcode);
        }
    }
    let mut circular = project("circle", "svg");
    circular.operations[0].params.plunge =
        ivac_core::project::PlungeStrategy::Ramp { angle_deg: 30. };
    let response = post(circular).unwrap();
    let diagnostic = response
        .warnings
        .iter()
        .find(|w| w.kind == "entry_representation")
        .unwrap();
    assert_eq!(diagnostic.params["resolved"], "linearized_ramp");
    assert!(response
        .toolpath
        .iter()
        .all(|s| s.arc.is_none() && s.gcode_line > 0));
    for s in &response.toolpath {
        assert!(response
            .gcode
            .lines()
            .nth(s.gcode_line as usize - 1)
            .unwrap()
            .starts_with(if s.kind == ivac_core::gcode::preview::MoveKind::Rapid {
                "J"
            } else {
                "M3,"
            }));
    }
}

#[test]
fn ux1_generic_helix_geometry_direction_depth_and_shopbot_refusal_are_distinct() {
    use ivac_core::project::{CutDirection, PlungeStrategy};
    for kind in [
        profile(),
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
    ] {
        let mut winding = Vec::new();
        for direction in [CutDirection::Climb, CutDirection::Conventional] {
            let mut p = ux1_fixture("square_100", kind.clone());
            p.segments = common::closed_rectangle(10., 10., 40., 40.);
            p.operations[0].contour_params_mut().unwrap().cut_direction = direction;
            p.operations[0]
                .contour_params_mut()
                .unwrap()
                .finish_cut_direction = direction;
            p.operations[0].params.plunge = PlungeStrategy::Helix {
                angle_deg: 3.,
                radius_mm: Some(5.),
            };
            let bytes = serde_json::to_vec(&p).unwrap();
            let reloaded: Project = serde_json::from_slice(&bytes).unwrap();
            assert!(matches!(
                reloaded.operations[0].params.plunge,
                PlungeStrategy::Helix {
                    radius_mm: Some(5.),
                    ..
                }
            ));
            let generic = run_pipeline(
                PipelineRequest {
                    project: reloaded.clone(),
                    post_processor: Some(PostProcessorKind::Linuxcnc),
                    cps_post: None,
                },
                |_, _, _| {},
            )
            .unwrap();
            let descent: Vec<_> = generic
                .toolpath
                .iter()
                .filter(|s| s.arc.is_some() && s.to.z < s.from.z)
                .collect();
            assert!(
                !descent.is_empty(),
                "valid local helix geometry must remain machine-independent"
            );
            assert!(descent.iter().all(|s| s.to.z >= -5.0001));
            assert!(descent.iter().any(|s| (s.to.z + 2.).abs() < 0.0001));
            winding.push(descent[0].arc.unwrap().ccw);
            let error = post(reloaded).unwrap_err().to_string();
            assert!(
                error.contains("not qualified") || error.contains("not geometrically realized"),
                "{error}"
            );
            assert_eq!(serde_json::to_vec(&p).unwrap(), bytes);
        }
        assert_ne!(
            winding[0], winding[1],
            "helix follows requested contour winding"
        );
        // The requested large radius cannot fit; do not disguise this as post syntax.
        let mut too_large = ux1_fixture("square_100", kind);
        too_large.segments = common::closed_rectangle(10., 10., 40., 40.);
        too_large.operations[0].params.plunge = PlungeStrategy::Helix {
            angle_deg: 3.,
            radius_mm: Some(500.),
        };
        let generic = run_pipeline(
            PipelineRequest {
                project: too_large.clone(),
                post_processor: Some(PostProcessorKind::Linuxcnc),
                cps_post: None,
            },
            |_, _, _| {},
        )
        .unwrap();
        assert!(generic.gcode.contains("ENTRY: requested Helix; resolved"));
        let err = post(too_large).unwrap_err().to_string();
        assert!(
            err.contains("not geometrically realized") && err.contains("refuses this fallback"),
            "{err}"
        );
    }
}

#[test]
fn ux1_pocket_islands_and_open_profile_helix_never_gain_silent_machine_fallback() {
    use ivac_core::project::PlungeStrategy;
    for radius in [None, Some(5.), Some(500.)] {
        let mut p = project("pocket", "svg");
        p.operations[0].params.plunge = PlungeStrategy::Helix {
            angle_deg: 3.,
            radius_mm: radius,
        };
        let before = serde_json::to_value(&p).unwrap();
        let error = post(p.clone()).unwrap_err().to_string();
        assert!(
            error.contains("operation 1") && error.contains("Operation parameters were kept"),
            "{error}"
        );
        assert_eq!(serde_json::to_value(&p).unwrap(), before);
    }
    let mut p = ux1_fixture(
        "open_polyline",
        OpKind::Profile {
            offset: ToolOffset::On,
            contour: ContourParams::default(),
            profile: ProfileParams::default(),
        },
    );
    p.operations[0].params.plunge = PlungeStrategy::Helix {
        angle_deg: 3.,
        radius_mm: Some(5.),
    };
    let error = post(p).unwrap_err().to_string();
    assert!(
        error.contains("not geometrically realized") && error.contains("refuses this fallback"),
        "{error}"
    );
}

#[test]
fn ux2_work_zero_translates_actual_output_without_changing_intent_or_depths() {
    for case in ["square", "circle", "pocket", "drill"] {
        let base = project(case, "svg");
        let original = post(base.clone()).unwrap();
        let mut shifted = base.clone();
        shifted.work_offset.x_mm = 10.;
        shifted.work_offset.y_mm = -20.;
        let before = serde_json::to_value(&shifted).unwrap();
        let output = post(shifted.clone()).unwrap();
        let a: Vec<_> = parse(&original.gcode)
            .into_iter()
            .filter(|m| !m.rapid)
            .collect();
        let b: Vec<_> = parse(&output.gcode)
            .into_iter()
            .filter(|m| !m.rapid)
            .collect();
        assert_eq!(a.len(), b.len(), "{case}");
        for (left, right) in a.iter().zip(&b) {
            assert!((right.to[0] - left.to[0] + 10.).abs() < 0.0003, "{case}");
            assert!((right.to[1] - left.to[1] - 20.).abs() < 0.0003, "{case}");
            assert_eq!(right.to[2], left.to[2]);
            assert_eq!(right.speed, left.speed);
        }
        assert_eq!(serde_json::to_value(&shifted).unwrap(), before);
        assert_eq!(post(shifted).unwrap().gcode, output.gcode);
        assert_eq!(
            output
                .warnings
                .iter()
                .filter(|w| w.kind == "out_of_stock")
                .count(),
            original
                .warnings
                .iter()
                .filter(|w| w.kind == "out_of_stock")
                .count()
        );
        let native = run_pipeline(
            PipelineRequest {
                project: {
                    let mut p = base;
                    p.work_offset.x_mm = 10.;
                    p.work_offset.y_mm = -20.;
                    p
                },
                post_processor: Some(PostProcessorKind::Linuxcnc),
                cps_post: None,
            },
            |_, _, _| {},
        )
        .unwrap();
        assert!(native.toolpath.iter().any(|s| s.to.z < 0.));
        assert!(native.gcode.contains("G54"));
    }
}

#[test]
fn ux2_fixture_collision_and_pocket_regions_stay_in_the_same_work_frame() {
    use ivac_core::project::{Fixture, FixtureKind};
    let mut p = project("square", "svg");
    // Known flute volume intersects this clamp above the cutting tip.
    p.tools[0].flute_length_mm = Some(20.);
    p.fixtures = vec![Fixture {
        id: 7,
        name: "SOFTWARE ONLY obstacle".into(),
        kind: FixtureKind::Box {
            width: 10.,
            depth: 10.,
        },
        origin: (7., 60.),
        z_bottom: 0.,
        z_top: 10.,
        color: 0xffa050c0,
    }];
    let before = post(p.clone())
        .unwrap_err()
        .to_structured(None)
        .unwrap()
        .diagnostic_preview
        .unwrap();
    assert!(before
        .warnings
        .iter()
        .any(|w| w.kind == "fixture_path_collision" && w.params["fixture_id"] == "7"));
    p.work_offset.x_mm = 10.;
    p.work_offset.y_mm = 20.;
    let after = post(p)
        .unwrap_err()
        .to_structured(None)
        .unwrap()
        .diagnostic_preview
        .unwrap();
    assert_eq!(
        before
            .warnings
            .iter()
            .filter(|w| w.kind == "fixture_path_collision")
            .count(),
        after
            .warnings
            .iter()
            .filter(|w| w.kind == "fixture_path_collision")
            .count()
    );
    let mut pocket = project("pocket", "svg");
    let original = post(pocket.clone()).unwrap();
    pocket.work_offset.x_mm = 10.;
    pocket.work_offset.y_mm = 20.;
    let translated = post(pocket).unwrap();
    assert_eq!(original.regions.len(), translated.regions.len());
    for (a, b) in original.regions.iter().zip(&translated.regions) {
        assert_eq!(a.outer.len(), b.outer.len());
        for (x, y) in a.outer.iter().zip(&b.outer) {
            assert!((x.x - y.x - 10.).abs() < 1e-8 && (x.y - y.y - 20.).abs() < 1e-8);
        }
    }
}

#[test]
fn ux2_invalid_origin_and_obstacle_geometry_fail_before_output() {
    use ivac_core::project::{Fixture, FixtureKind};
    for origin in [f64::NAN, f64::INFINITY] {
        let mut p = project("square", "svg");
        p.work_offset.x_mm = origin;
        assert!(post(p).unwrap_err().to_string().contains("finite"));
    }
    for kind in [
        FixtureKind::Box {
            width: 0.,
            depth: 5.,
        },
        FixtureKind::Cylinder { radius: -1. },
        FixtureKind::Polygon {
            vertices: vec![(0., 0.), (5., 5.), (0., 5.), (5., 0.)],
        },
    ] {
        let mut p = project("square", "svg");
        p.fixtures = vec![Fixture {
            id: 3,
            name: "invalid SOFTWARE ONLY".into(),
            kind,
            origin: (10., 10.),
            z_bottom: 0.,
            z_top: 5.,
            color: 0xffa050c0,
        }];
        assert!(post(p).unwrap_err().to_string().contains("obstacle 3"));
    }
}

#[test]
fn ux2_unqualified_two_sided_shopbot_scaffold_blocks_before_invalid_output() {
    let mut p = project("square", "svg");
    p.stock.as_mut().unwrap().flip = Some(ivac_core::project::FlipRegistration {
        axis: ivac_core::project::FlipAxis::X,
        dowels: None,
    });
    let mut back = p.operations[0].clone();
    back.id = 2;
    back.side = ivac_core::project::WorkpieceSide::Back;
    p.operations.push(back);
    let before = serde_json::to_value(&p).unwrap();
    let err = ivac_core::pipeline::run_pipeline_two_sided(
        PipelineRequest {
            project: p.clone(),
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: BTreeMap::new(),
            }),
        },
        |_, _, _| {},
    )
    .unwrap_err();
    assert!(err.to_string().contains("two-sided CPS"));
    assert_eq!(serde_json::to_value(&p).unwrap(), before);
}

#[test]
fn ux2_polar_drill_center_and_explicit_approach_translate_with_the_drawing() {
    for drill in [false, true] {
        let mut p = project(if drill { "drill" } else { "square" }, "svg");
        if let OpKind::Drill { pattern, .. } = &mut p.operations[0].kind {
            *pattern = Some(ivac_core::project::PatternConfig::Polar {
                count: 2,
                center_x: 80.,
                center_y: 80.,
                angle_step_deg: 180.,
                start_angle_deg: 0.,
            });
        } else {
            p.operations[0].contour_params_mut().unwrap().approach_point = Some((110., 60.));
        }
        let original = post(p.clone()).unwrap();
        p.work_offset.x_mm = 20.;
        p.work_offset.y_mm = 30.;
        let translated = post(p).unwrap();
        let a: Vec<_> = parse(&original.gcode)
            .into_iter()
            .filter(|m| !m.rapid)
            .collect();
        let b: Vec<_> = parse(&translated.gcode)
            .into_iter()
            .filter(|m| !m.rapid)
            .collect();
        assert_eq!(a.len(), b.len());
        for (left, right) in a.iter().zip(b) {
            assert!((right.to[0] - left.to[0] + 20.).abs() < 0.0003);
            assert!((right.to[1] - left.to[1] + 30.).abs() < 0.0003);
            assert_eq!(right.to[2], left.to[2]);
        }
    }
}

// CAM3: sample emitted XYZ on an analytical square, independently of tab emitter.
fn cam3_lowest_z_near(moves: &[Move], point: [f64; 2]) -> f64 {
    moves
        .iter()
        .filter(|m| !m.rapid)
        .filter_map(|m| {
            let dx = m.to[0] - m.from[0];
            let dy = m.to[1] - m.from[1];
            let length2 = dx * dx + dy * dy;
            let t = if length2 > 1e-12 {
                (((point[0] - m.from[0]) * dx + (point[1] - m.from[1]) * dy) / length2)
                    .clamp(0., 1.)
            } else {
                0.
            };
            let distance = (m.from[0] + t * dx - point[0]).hypot(m.from[1] + t * dy - point[1]);
            (distance < 0.01).then_some(m.from[2] + t * (m.to[2] - m.from[2]))
        })
        .fold(f64::INFINITY, f64::min)
}

#[test]
fn cam3_tab_width_and_absolute_height_override_preserve_material() {
    use ivac_core::project::{TabPlacement, TabPlacementMode};
    for height_override in [None, Some(3.)] {
        let mut p = project("square", "svg");
        let contour = p.operations[0].contour_params_mut().unwrap();
        contour.tab_mode = TabPlacementMode::Manual;
        contour.tabs.width = 20.;
        contour.tabs.height = 1.;
        contour.tab_placements = vec![TabPlacement {
            object_id: 1,
            t: 0.125,
            width_override_mm: None,
            height_override_mm: height_override,
        }];
        let response = post(p).unwrap();
        let moves = parse(&response.gcode);
        // Source square starts at (10,10); manual tab at (60,10), projected
        // onto outside cutter center y=7. A 20mm tab spans x=50..70.
        let expected_top = -5. + height_override.unwrap_or(1.);
        for x in [56., 60., 64.] {
            let actual = cam3_lowest_z_near(&moves, [x, 7.]);
            assert!(actual.is_finite(), "missing tab crossing at x={x}");
            assert!(actual >= expected_top - 0.0001,
                "tab removed: override={height_override:?}, x={x}, minimum={actual}, top={expected_top}");
        }
        assert!((cam3_lowest_z_near(&moves, [80., 7.]) + 5.).abs() < 0.0001);
    }
}

#[test]
fn cam3_direct_entry_tab_lead_direction_matrix_has_output_preview_parity() {
    use ivac_core::project::{CutDirection, LeadKind, TabPlacementMode, TabType};
    for case in ["square", "circle"] {
        for offset in [ToolOffset::Outside, ToolOffset::Inside] {
            for direction in [CutDirection::Climb, CutDirection::Conventional] {
                for tab_type in [TabType::Rectangle, TabType::Ramp] {
                    for lead in [LeadKind::Off, LeadKind::Straight, LeadKind::Arc] {
                        let mut p = project(case, "svg");
                        if let OpKind::Profile { offset: side, .. } = &mut p.operations[0].kind {
                            *side = offset;
                        }
                        let contour = p.operations[0].contour_params_mut().unwrap();
                        contour.tab_mode = TabPlacementMode::Auto { count: 2 };
                        contour.tabs.width = 10.;
                        contour.tabs.height = 1.;
                        contour.tabs.tab_type = tab_type;
                        contour.cut_direction = direction;
                        contour.finish_cut_direction = direction;
                        contour.leads.r#in = lead;
                        contour.leads.out = lead;
                        let before = serde_json::to_value(&p).unwrap();
                        let restored: Project = serde_json::from_value(before.clone()).unwrap();
                        let response = post(restored).unwrap();
                        assert_eq!(serde_json::to_value(&p).unwrap(), before);
                        let moves = parse(&response.gcode);
                        assert!(moves
                            .iter()
                            .any(|m| !m.rapid && (m.to[2] + 5.).abs() < 0.0001));
                        assert!(moves.iter().all(|m| m.to[2] >= -5.0001));
                        assert!(moves
                            .iter()
                            .any(|m| !m.rapid && (m.to[2] + 4.).abs() < 0.0001));
                        if case == "square" {
                            let y = if offset == ToolOffset::Outside {
                                7.
                            } else {
                                13.
                            };
                            let minimum = cam3_lowest_z_near(&moves, [15., y]);
                            assert!(minimum.is_finite() && minimum >= -4.0001,
                                "tab center removed: {offset:?}/{direction:?}/{tab_type:?}/{lead:?}: {minimum}");
                        }
                        let area: f64 = moves
                            .iter()
                            .filter(|m| !m.rapid && m.from[2] < 0. && m.to[2] < 0.)
                            .map(|m| {
                                (m.from[0] - 60.) * (m.to[1] - 60.)
                                    - (m.to[0] - 60.) * (m.from[1] - 60.)
                            })
                            .sum();
                        assert_eq!(area > 0., (direction == CutDirection::Climb) == (offset == ToolOffset::Outside),
                            "cut direction changed: {case}/{offset:?}/{direction:?}/{tab_type:?}/{lead:?}");
                        for m in &moves {
                            if m.rapid && (m.from[0] != m.to[0] || m.from[1] != m.to[1]) {
                                assert!(
                                    m.from[2] >= 0. && m.to[2] >= 0.,
                                    "XY rapid in stock: {m:?}"
                                );
                            }
                            if !m.rapid && m.to != m.from {
                                assert!(
                                    response.toolpath.iter().any(|s| [
                                        s.from.x, s.from.y, s.from.z
                                    ]
                                    .iter()
                                    .zip(m.from)
                                    .all(|(a, b)| (a - b).abs() < 0.0001)
                                        && [s.to.x, s.to.y, s.to.z]
                                            .iter()
                                            .zip(m.to)
                                            .all(|(a, b)| (a - b).abs() < 0.0001)),
                                    "emitted move missing from preview: {m:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn cam3_curved_tabs_preserve_center_and_compensated_radius() {
    use ivac_core::project::{TabPlacement, TabPlacementMode, TabType};
    for (mode, tab_type) in [
        (
            ivac_core::project::TabWidthMode::LegacyCenter,
            TabType::Rectangle,
        ),
        (
            ivac_core::project::TabWidthMode::LegacyCenter,
            TabType::Ramp,
        ),
        (
            ivac_core::project::TabWidthMode::MaterialV1,
            TabType::Rectangle,
        ),
        (ivac_core::project::TabWidthMode::MaterialV1, TabType::Ramp),
    ] {
        let mut p = project("circle", "svg");
        let c = p.operations[0].contour_params_mut().unwrap();
        c.tab_mode = TabPlacementMode::Manual;
        c.tabs.width = 12.;
        c.tabs.width_mode = mode;
        c.tabs.height = 1.;
        c.tabs.tab_type = tab_type;
        c.tab_placements = vec![TabPlacement {
            object_id: 1,
            t: 0.125,
            width_override_mm: None,
            height_override_mm: None,
        }];
        let response = post(p).unwrap_or_else(|e| panic!("{mode:?}/{tab_type:?}: {e:?}"));
        let moves = parse(&response.gcode);
        // Circle source is centered (60,60), radius25. The manual eighth
        // perimeter location may traverse either orientation; probe all four
        // diagonal candidates and require at least one protected tab center.
        let protected = [45_f64, 135., 225., 315.]
            .into_iter()
            .filter(|angle| {
                let a = angle.to_radians();
                let point = [60. + 28. * a.cos(), 60. + 28. * a.sin()];
                let low = cam3_lowest_z_near(&moves, point);
                low.is_finite() && low >= -4.0001
            })
            .count();
        assert!(protected > 0, "curved tab not preserved: {tab_type:?}");
        for m in moves.iter().filter(|m| !m.rapid && m.to[2] < 0.) {
            for t in [0., 0.5, 1.] {
                let r = (m.from[0] + t * (m.to[0] - m.from[0]) - 60.)
                    .hypot(m.from[1] + t * (m.to[1] - m.from[1]) - 60.);
                assert!(
                    (r - 28.).abs() <= 0.0102,
                    "curved tab compensation changed: {tab_type:?}, radius={r}"
                );
            }
        }
    }
}

#[test]
fn cam3_tab_at_entry_cannot_be_plunged_through() {
    use ivac_core::project::{TabPlacement, TabPlacementMode};
    for mode in [
        ivac_core::project::TabWidthMode::LegacyCenter,
        ivac_core::project::TabWidthMode::MaterialV1,
    ] {
        let mut p = project("square", "svg");
        let c = p.operations[0].contour_params_mut().unwrap();
        c.tab_mode = TabPlacementMode::Manual;
        c.tabs.width_mode = mode;
        c.tabs.width = 12.;
        c.tabs.height = 1.;
        c.tab_placements = vec![TabPlacement {
            object_id: 1,
            t: 0.,
            width_override_mm: None,
            height_override_mm: None,
        }];
        let response = post(p).unwrap();
        let minimum = cam3_lowest_z_near(&parse(&response.gcode), [10., 7.]);
        assert!(
            minimum.is_finite() && minimum >= -4.0001,
            "entry destroyed tab: {minimum}"
        );
    }
}

#[test]
fn cam4_pocket_residual_tracks_emitted_path_and_work_origin() {
    let mut p = project("square", "svg");
    p.operations[0].kind = OpKind::Pocket {
        strategy: PocketStrategy::Cascade,
        contour: ContourParams::default(),
        pocket: PocketParams {
            pocket_islands: true,
            xy_overlap: 0.5,
            ..Default::default()
        },
    };
    let baseline = post(p.clone()).unwrap();
    let analysis = &baseline.remaining_material[0];
    assert_eq!(analysis.status, "ready", "{}", analysis.reason);
    assert_eq!(analysis.regions.len(), 4);
    assert!(
        (analysis.area_mm2 - (4. - std::f64::consts::PI) * 9.).abs() < 0.2,
        "{}",
        analysis.area_mm2
    );
    p.work_offset.x_mm = 20.;
    p.work_offset.y_mm = 30.;
    let moved = post(p.clone()).unwrap();
    assert!((moved.remaining_material[0].area_mm2 - analysis.area_mm2).abs() < 0.001);
    for (a, b) in analysis
        .regions
        .iter()
        .zip(&moved.remaining_material[0].regions)
    {
        for (x, y) in a.outer.iter().zip(&b.outer) {
            assert!((x.x - y.x - 20.).abs() < 0.0001 && (x.y - y.y - 30.).abs() < 0.0001);
        }
    }
    p.operations[0].params.stock_to_leave_mm = 1.;
    let unsupported = post(p).unwrap();
    assert_eq!(unsupported.remaining_material[0].status, "unsupported");
    assert_eq!(
        unsupported.remaining_material[0].reason,
        "intentional_allowance"
    );
    assert!(unsupported.remaining_material[0].regions.is_empty());
}
