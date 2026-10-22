//! FS-RESET-2: software-only source-bound output and rejection regressions.
use super::test_helpers::*;
use super::*;
use crate::cutting_parameters::{self as cutting, CuttingRole};
use crate::project::{
    DrillCycle, OpKind, OpSource, PocketStrategy, SpotConfig, ToolKind, ToolOffset,
};

fn explicit(mut project: Project) -> Project {
    project.cutting_version = Some(9);
    for op in &mut project.operations {
        if !op.is_program_only() {
            op.params.cutting = Some(cutting::seed(op, &project.tools));
        }
    }
    project
}
fn request(project: Project) -> PipelineRequest {
    PipelineRequest {
        project,
        post_processor: Some(PostProcessorKind::Linuxcnc),
        cps_post: None,
    }
}
fn set_role(op: &mut Op, role: CuttingRole, rpm: u32, feed: u32, plunge: u32) {
    let r = op
        .params
        .cutting
        .as_mut()
        .unwrap()
        .rates
        .iter_mut()
        .find(|r| r.role == role)
        .unwrap();
    r.rpm = Some(rpm);
    r.feed_mm_min = Some(feed);
    r.plunge_mm_min = Some(plunge);
}
#[test]
fn v9_standard_valid_inputs_keep_byte_identical_programs() {
    let mut pocket = pocket_op(1, 1, OpSource::All);
    pocket.pocket_params_mut().unwrap().xy_overlap = 0.5;
    let mut engrave = profile_op(1, 1, ToolOffset::On);
    engrave.kind = OpKind::Engrave {
        contour: Default::default(),
    };
    let mut chamfer = profile_op(1, 1, ToolOffset::On);
    chamfer.kind = OpKind::Chamfer {
        width_mm: 0.5,
        finish_pass: true,
    };
    for (name, op, tool) in [
        (
            "profile",
            profile_op(1, 1, ToolOffset::Outside),
            endmill(1, 3.0),
        ),
        ("pocket", pocket, endmill(1, 3.0)),
        ("engrave", engrave, endmill(1, 3.0)),
        ("chamfer", chamfer, vbit()),
    ] {
        let p = project_with(vec![op], vec![tool]);
        let baseline = run_pipeline(request(p.clone()), |_, _, _| {}).unwrap();
        let current = run_pipeline(request(explicit(p)), |_, _, _| {}).unwrap();
        assert_eq!(baseline.gcode, current.gcode, "{name}");
        assert_eq!(baseline.toolpath.len(), current.toolpath.len(), "{name}");
        assert_eq!(
            current.tool_provenance.authority,
            provenance::Authority::Resolved
        );
    }
}
#[cfg(feature = "cps")]
#[test]
fn v9_test1_shopbot_profile_and_vcarve_match_established_sources() {
    for (name, json, golden) in [
        (
            "profile",
            include_str!("../../../../tests/fixtures/test1/profile.request.json"),
            include_str!("../../../../tests/fixtures/test1/profile.sbp"),
        ),
        (
            "vcarve",
            include_str!("../../../../tests/fixtures/test1/vcarve.request.json"),
            include_str!("../../../../tests/fixtures/test1/vcarve.sbp"),
        ),
    ] {
        let mut req: PipelineRequest = serde_json::from_str(json).unwrap();
        let baseline = run_pipeline(req.clone(), |_, _, _| {}).unwrap();
        req.project = explicit(req.project);
        let current = run_pipeline(req, |_, _, _| {}).unwrap();
        assert_eq!(
            current.gcode, baseline.gcode,
            "{name}: unchanged source output"
        );
        // Historical fixture header build strings may differ; compare executable commands.
        let commands = |s: &str| {
            s.lines()
                .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('\''))
                .map(str::trim)
                .collect::<Vec<_>>()
                .join("\n")
        };
        assert_eq!(
            commands(&current.gcode),
            commands(golden),
            "{name}: established output fixture"
        );
    }
}
#[test]
fn v9_dual_pocket_uses_only_actual_secondary_finish_values() {
    let mut op = pocket_op(1, 1, OpSource::All);
    op.finish_tool_id = Some(2);
    op.pocket_params_mut().unwrap().xy_overlap = 0.5;
    let mut p = explicit(project_with(
        vec![op],
        vec![endmill(1, 3.0), endmill(2, 2.0)],
    ));
    set_role(&mut p.operations[0], CuttingRole::Rough, 11000, 811, 111);
    set_role(&mut p.operations[0], CuttingRole::Finish, 13000, 413, 113);
    p.machine.tool_change = crate::project::ToolChangeStrategy::Atc;
    p.tools[1].speed = 99999;
    p.tools[1].feed_rate_finish = Some(99999);
    let resp = run_pipeline(request(p), |_, _, _| {}).unwrap();
    for word in ["S11000", "S13000", "F811", "F413", "T2"] {
        assert!(resp.gcode.contains(word), "missing {word}: {}", resp.gcode);
    }
    assert!(!resp.gcode.contains("99999"));
    assert_eq!(
        resp.tool_provenance
            .bindings
            .iter()
            .map(|b| b.project_tool_id)
            .collect::<std::collections::BTreeSet<_>>(),
        [1, 2].into()
    );
}
#[test]
fn v9_drill_spot_and_chamfer_are_independent_even_with_the_same_tool() {
    for shared in [true, false] {
        let mut drill = drill_op(
            1,
            1,
            DrillCycle::Peck {
                peck_step_mm: 1.0,
                dwell_sec: 0.0,
            },
        );
        drill.kind = OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: 1.0,
                dwell_sec: 0.0,
            },
            spot_first: Some(SpotConfig {
                spot_tool_id: if shared { 1 } else { 2 },
                spot_depth_mm: -0.3,
            }),
            chamfer_after_width_mm: Some(0.3),
            pattern: None,
        };
        drill.finish_tool_id = Some(if shared { 1 } else { 3 });
        let mut tools = vec![vbit()];
        if !shared {
            for id in [2, 3] {
                let mut t = vbit();
                t.id = id;
                tools.push(t);
            }
        }
        let mut p = explicit(project_with_segments(
            closed_circle(Point2::new(10., 10.), 3.),
            vec![drill],
            tools,
        ));
        set_role(&mut p.operations[0], CuttingRole::Drill, 11000, 810, 110);
        set_role(&mut p.operations[0], CuttingRole::Spot, 12000, 820, 120);
        set_role(&mut p.operations[0], CuttingRole::Chamfer, 13000, 830, 130);
        let resp = run_pipeline(request(p), |_, _, _| {}).unwrap();
        for word in ["S11000", "S12000", "S13000", "F110", "F120", "F830"] {
            assert!(
                resp.gcode.contains(word),
                "shared={shared}: missing {word}: {}",
                resp.gcode
            );
        }
        assert!(!resp.gcode.contains("S18000"), "live default leaked");
        assert_eq!(
            resp.tool_provenance.bindings.len(),
            if shared { 1 } else { 3 }
        );
    }
}
#[test]
fn v9_missing_secondary_values_and_known_limits_refuse_without_clamping() {
    let op = profile_op(1, 1, ToolOffset::Outside);
    let p = explicit(project_with(vec![op], vec![endmill(1, 3.)]));
    for case in 0..6 {
        let mut bad = p.clone();
        match case {
            0 => bad.operations[0].params.cutting.as_mut().unwrap().rates[1].rpm = None,
            1 => bad.machine.spindle_rpm_max = Some(1000),
            2 => bad.machine.max_feed_mm_min = Some(10),
            3 => bad.tools[0].max_rpm = Some(1000),
            4 => bad.tools[0].flute_length_mm = Some(0.1),
            _ => bad.tools[0].kind = ToolKind::Drill,
        }
        assert!(
            run_pipeline(request(bad), |_, _, _| {}).is_err(),
            "case {case}"
        );
    }
    assert!(run_pipeline(request(p), |_, _, _| {}).is_ok());
}
#[test]
fn v9_chamfer_reach_and_peck_defaults_are_not_silently_substituted() {
    let mut op = drill_op(
        1,
        1,
        DrillCycle::Peck {
            peck_step_mm: 0.,
            dwell_sec: 0.,
        },
    );
    let mut tool = vbit();
    tool.default_peck_step_mm = Some(1.0);
    let mut p = explicit(project_with_segments(
        closed_circle(Point2::new(10., 10.), 3.),
        vec![op.clone()],
        vec![tool.clone()],
    ));
    assert!(run_pipeline(request(p.clone()), |_, _, _| {}).is_err());
    op.kind = OpKind::Chamfer {
        width_mm: 10.,
        finish_pass: false,
    };
    op.params.cutting = Some(cutting::seed(&op, std::slice::from_ref(&tool)));
    p.operations[0] = op;
    assert!(run_pipeline(request(p), |_, _, _| {}).is_err());
}
#[test]
fn v9_role_matrix_uses_only_consumed_specialized_sets() {
    let mut op = profile_op(1, 1, ToolOffset::On);
    for kind in [
        OpKind::Engrave {
            contour: Default::default(),
        },
        OpKind::Chamfer {
            width_mm: 0.5,
            finish_pass: false,
        },
        OpKind::Pocket {
            strategy: PocketStrategy::Halfpipe {
                profile: crate::project::HalfpipeProfile::CircularArc { radius_mm: 1.5 },
            },
            contour: Default::default(),
            pocket: Default::default(),
        },
        OpKind::TSlot {
            contour: Default::default(),
        },
        OpKind::Dovetail {
            contour: Default::default(),
        },
        OpKind::VCarve {
            carve: Default::default(),
        },
        OpKind::Helix,
    ] {
        op.kind = kind;
        assert_eq!(cutting::roles(&op), vec![(CuttingRole::Rough, 1)]);
    }
}
#[test]
fn v9_public_contract_refuses_legacy_and_duplicate_ids() {
    let p = project_with(
        vec![profile_op(1, 1, ToolOffset::Outside)],
        vec![endmill(1, 3.)],
    );
    assert!(cutting::require_current(&p).is_err());
    let mut p = explicit(p);
    assert!(cutting::require_current(&p).is_ok());
    p.tools.push(p.tools[0].clone());
    assert!(run_pipeline(request(p), |_, _, _| {}).is_err());
}

fn specialized_projects() -> Vec<(&'static str, Project)> {
    use crate::cam::{planar_surface::PlanarSurfaceParams, surface_mill::ScanDirection};
    use crate::project::{FormProfileSample, HalfpipeProfile, ReliefGrid, ReliefSource};
    let mut cases = vec![];
    let mut op = profile_op(1, 1, ToolOffset::On);
    op.kind = OpKind::PlanarSurface {
        surface: PlanarSurfaceParams {
            origin: [5., 5.],
            width_mm: 10.,
            height_mm: 10.,
            margin_mm: 0.,
            overtravel_mm: 0.,
            stepover_mm: 1.5,
            direction: ScanDirection::AlongX,
        },
    };
    cases.push((
        "planar",
        project_with(vec![op.clone()], vec![endmill(1, 3.)]),
    ));
    op.kind = OpKind::Thread {
        pitch_mm: 1.,
        internal: true,
        climb: false,
        radial_passes: 2,
        start_angle_rad: 0.,
        thread_depth_mm: Some(0.6),
    };
    cases.push((
        "thread",
        project_with_segments(
            closed_circle(Point2::new(10., 10.), 6.),
            vec![op.clone()],
            vec![endmill(1, 3.)],
        ),
    ));
    let mut form = endmill(1, 8.);
    form.kind = ToolKind::FormProfile;
    form.form_profile_mm = vec![
        FormProfileSample { z_mm: 0., r_mm: 4. },
        FormProfileSample { z_mm: 2., r_mm: 4. },
        FormProfileSample { z_mm: 2., r_mm: 2. },
        FormProfileSample { z_mm: 8., r_mm: 2. },
    ];
    for (name, kind) in [
        (
            "tslot",
            OpKind::TSlot {
                contour: Default::default(),
            },
        ),
        (
            "dovetail",
            OpKind::Dovetail {
                contour: Default::default(),
            },
        ),
    ] {
        op.kind = kind;
        cases.push((name, project_with(vec![op.clone()], vec![form.clone()])));
    }
    let mut ball = endmill(1, 4.);
    ball.kind = ToolKind::BallNose;
    ball.flute_length_mm = Some(20.);
    op = pocket_op(1, 1, OpSource::All);
    op.pocket_params_mut().unwrap().xy_overlap = 0.5;
    if let OpKind::Pocket { strategy, .. } = &mut op.kind {
        *strategy = PocketStrategy::Halfpipe {
            profile: HalfpipeProfile::CircularArc { radius_mm: 2. },
        };
    }
    cases.push((
        "halfpipe",
        project_with(vec![op.clone()], vec![ball.clone()]),
    ));
    op.kind = OpKind::ReliefMill {
        source_id: 7,
        z_min_mm: -5.,
        z_max_mm: 0.,
        invert: false,
        scallop_height_mm: 0.,
        stepover_mm: Some(2.),
        scan_direction: ScanDirection::AlongX,
        along_step_mm: 1.,
    };
    let mut p = project_with(vec![op.clone()], vec![ball]);
    p.segments.clear();
    p.relief_sources = vec![ReliefSource {
        id: 7,
        name: "software ramp".into(),
        origin: Point2::new(0., 0.),
        cell: 2.,
        cols: 6,
        rows: 6,
        grid: ReliefGrid::Grayscale {
            brightness: (0..36).map(|i| (i % 6) as f32 / 5.).collect(),
        },
    }];
    cases.push(("relief", p));
    op.kind = OpKind::WaterlineRough {
        source_id: 7,
        z_step_mm: 2.,
        stepover_mm: 1.5,
        floor_z_mm: 0.,
    };
    let mut p = project_with(vec![op], vec![endmill(1, 3.)]);
    p.segments.clear();
    p.relief_sources = vec![ReliefSource {
        id: 7,
        name: "software basin".into(),
        origin: Point2::new(0., 0.),
        cell: 2.,
        cols: 9,
        rows: 9,
        grid: ReliefGrid::Heightgrid {
            z: (0..81)
                .map(|i| {
                    let x = i % 9;
                    let y = i / 9;
                    -6. * x.min(y).min(8 - x).min(8 - y) as f32 / 4.
                })
                .collect(),
        },
    }];
    cases.push(("waterline", p));
    cases
}
#[test]
fn v9_specialized_valid_inputs_keep_program_and_toolpath() {
    for (name, p) in specialized_projects() {
        let old = run_pipeline(request(p.clone()), |_, _, _| {})
            .unwrap_or_else(|e| panic!("legacy {name}: {e}"));
        let new = run_pipeline(request(explicit(p)), |_, _, _| {})
            .unwrap_or_else(|e| panic!("v9 {name}: {e}"));
        assert!(!new.toolpath.is_empty(), "{name}");
        assert_eq!(old.gcode, new.gcode, "{name}");
        assert_eq!(
            serde_json::to_value(old.toolpath).unwrap(),
            serde_json::to_value(new.toolpath).unwrap(),
            "{name}"
        );
        assert_eq!(
            new.tool_provenance.authority,
            provenance::Authority::Resolved,
            "{name}"
        );
    }
}
#[test]
fn v9_specialized_invalid_inputs_cannot_bypass_mandatory_restrictions() {
    for (name, p) in specialized_projects() {
        let mut p = explicit(p);
        match name {
            "planar" => {
                if let OpKind::PlanarSurface { surface } = &mut p.operations[0].kind {
                    surface.stepover_mm = 4.;
                }
            }
            "thread" => {
                if let OpKind::Thread { pitch_mm, .. } = &mut p.operations[0].kind {
                    *pitch_mm = 0.;
                }
            }
            "tslot" | "dovetail" => p.tools[0].form_profile_mm.clear(),
            "halfpipe" => p.tools[0].flute_length_mm = None,
            "relief" | "waterline" => p.tools[0].flute_length_mm = Some(0.5),
            _ => unreachable!(),
        }
        assert!(run_pipeline(request(p), |_, _, _| {}).is_err(), "{name}");
    }
}
#[test]
fn v9_two_sided_preserves_both_programs_and_refuses_invalid_back() {
    use crate::project::{FlipAxis, FlipRegistration, StockConfig, WorkpieceSide};
    let front = profile_op(1, 1, ToolOffset::Outside);
    let mut back = front.clone();
    back.id = 2;
    back.side = WorkpieceSide::Back;
    let mut p = project_with(vec![front, back], vec![endmill(1, 3.)]);
    p.stock = Some(StockConfig {
        origin: [0., 0.],
        width_mm: 30.,
        height_mm: 30.,
        thickness_mm: 20.,
        top_z_mm: 0.,
        flip: Some(FlipRegistration {
            axis: FlipAxis::X,
            dowels: None,
        }),
    });
    let old = run_pipeline_two_sided(request(p.clone()), |_, _, _| {}).unwrap();
    let mut p = explicit(p);
    let new = run_pipeline_two_sided(request(p.clone()), |_, _, _| {}).unwrap();
    assert_eq!(old.front.gcode, new.front.gcode);
    assert_eq!(old.back.unwrap().gcode, new.back.as_ref().unwrap().gcode);
    assert_eq!(
        new.front.tool_provenance.authority,
        provenance::Authority::Resolved
    );
    assert_eq!(
        new.back.unwrap().tool_provenance.authority,
        provenance::Authority::Resolved
    );
    let mut dowels = p.clone();
    dowels.stock.as_mut().unwrap().flip.as_mut().unwrap().dowels =
        Some(crate::project::DowelPinConfig {
            diameter_mm: 4.,
            count: 2,
            margin_mm: 3.,
        });
    assert!(run_pipeline_two_sided(request(dowels), |_, _, _| {})
        .unwrap_err()
        .to_string()
        .contains("explicit operation-owned"));
    p.operations[1].params.cutting.as_mut().unwrap().rates[1].rpm = None;
    assert!(run_pipeline_two_sided(request(p), |_, _, _| {}).is_err());
}
#[test]
fn v9_streaming_preserves_bytes_and_preflight_rejects_missing_values() {
    #[derive(Clone)]
    struct Sink(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);
    impl std::io::Write for Sink {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut p = explicit(project_with(
        vec![profile_op(1, 1, ToolOffset::Outside)],
        vec![endmill(1, 3.)],
    ));
    let expected = run_pipeline(request(p.clone()), |_, _, _| {})
        .unwrap()
        .gcode;
    let sink = Sink(Default::default());
    stream_gcode_to_writer(request(p.clone()), Box::new(sink.clone())).unwrap();
    assert_eq!(*sink.0.lock().unwrap(), expected.as_bytes());
    p.operations[0].params.cutting.as_mut().unwrap().rates[1].feed_mm_min = None;
    let sink = Sink(Default::default());
    assert!(stream_gcode_to_writer(request(p), Box::new(sink.clone())).is_err());
    assert!(sink.0.lock().unwrap().is_empty());
}
