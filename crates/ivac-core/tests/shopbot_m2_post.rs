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
    golden(case, &a.gcode);
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
