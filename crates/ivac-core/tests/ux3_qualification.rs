//! UX3 decoded `ShopBot` coverage, multipass, clearance and refusal proofs.
#![cfg(feature = "cps")]
// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::float_cmp)]
mod common;
use common::{build_project, endmill_tool, op_single_pass, stock_at_origin};
use ivac_core::cam::planar_surface::{planar_raster, PlanarSurfaceParams};
use ivac_core::cam::surface_mill::ScanDirection;
use ivac_core::gcode::preview::MoveKind;
use ivac_core::pipeline::*;
use ivac_core::project::*;
use std::collections::BTreeMap;

fn fixture(direction: ScanDirection) -> Project {
    let surface = PlanarSurfaceParams {
        origin: [20.123, 30.456],
        width_mm: 41.123,
        height_mm: 23.789,
        margin_mm: 1.234,
        overtravel_mm: 1.111,
        stepover_mm: 2.4,
        direction,
    };
    let mut op = op_single_pass(1, "Surfacing", OpKind::PlanarSurface { surface }, 1, -1.3);
    op.params.step = Some(-0.5);
    let mut stock = stock_at_origin(41.123, 23.789, 15.);
    stock.origin = [20.123, 30.456];
    let mut tool = endmill_tool(1, 6.);
    tool.flute_length_mm = Some(10.);
    let mut p = build_project(&stock, vec![tool], op, vec![]);
    p.work_offset.x_mm = 5.;
    p.work_offset.y_mm = 7.;
    p
}
fn run(p: Project) -> Result<PipelineResponse, PipelineError> {
    run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: BTreeMap::new(),
            }),
        }),
        |_, _, _| {},
    )
}
#[derive(Debug)]
struct Move {
    from: [f64; 3],
    to: [f64; 3],
    rapid: bool,
}
fn decode(text: &str) -> Vec<Move> {
    let mut position = [0., 0., 5.];
    let mut moves = vec![];
    for line in text.lines() {
        let mut fields = line.split(',');
        let cmd = fields.next().unwrap_or_default();
        if !["M3", "J2", "JZ", "MZ"].contains(&cmd) {
            continue;
        }
        let values: Vec<f64> = fields.map(|v| v.parse().unwrap()).collect();
        let from = position;
        match cmd {
            "M3" => position.copy_from_slice(&values),
            "J2" => {
                position[0] = values[0];
                position[1] = values[1];
            }
            "JZ" | "MZ" => position[2] = values[0],
            _ => unreachable!(),
        }
        moves.push(Move {
            from,
            to: position,
            rapid: cmd.starts_with('J'),
        });
    }
    moves
}
fn distance(q: [f64; 2], m: &Move) -> f64 {
    let v = [m.to[0] - m.from[0], m.to[1] - m.from[1]];
    let length = v[0] * v[0] + v[1] * v[1];
    let t = if length == 0. {
        0.
    } else {
        (((q[0] - m.from[0]) * v[0] + (q[1] - m.from[1]) * v[1]) / length).clamp(0., 1.)
    };
    (q[0] - m.from[0] - t * v[0]).hypot(q[1] - m.from[1] - t * v[1])
}
#[test]
fn decoded_shopbot_covers_every_target_edge_and_corner_in_xy_origins_and_both_directions() {
    for direction in [ScanDirection::AlongX, ScanDirection::AlongY] {
        let p = fixture(direction);
        let OpKind::PlanarSurface { surface } = &p.operations[0].kind else {
            unreachable!()
        };
        let raster = planar_raster(surface, 6.).unwrap();
        let response = run(p).unwrap();
        assert!(response
            .warnings
            .iter()
            .any(|w| w.kind == "surfacing_edge_overtravel"));
        assert!(
            !response.warnings.iter().any(|w| [
                "op_source_empty",
                "out_of_stock",
                "out_of_work_area"
            ]
            .contains(&w.kind.as_str())),
            "{:?}",
            response.warnings
        );
        let moves = decode(&response.gcode);
        let final_cuts: Vec<_> = moves
            .iter()
            .filter(|m| !m.rapid && (m.from[2] + 1.3).abs() < 1e-6 && (m.to[2] + 1.3).abs() < 1e-6)
            .collect();
        let [xmin, ymin, xmax, ymax] = raster.target_bounds;
        for ix in 0..=80 {
            for iy in 0..=80 {
                let q = [
                    xmin + (xmax - xmin) * f64::from(ix) / 80. - 5.,
                    ymin + (ymax - ymin) * f64::from(iy) / 80. - 7.,
                ];
                assert!(
                    final_cuts.iter().any(|m| distance(q, m) <= 3.),
                    "uncovered {q:?}"
                );
            }
        }
        // Preview is reconstructed from the exact final program, in WCS.
        for m in moves.iter().filter(|m| !m.rapid && m.from != m.to) {
            assert!(
                response.toolpath.iter().any(|s| s.op_id == 1
                    && (s.to.x - m.to[0]).abs() < 1e-6
                    && (s.to.y - m.to[1]).abs() < 1e-6
                    && (s.to.z - m.to[2]).abs() < 1e-6),
                "preview lacks {m:?}"
            );
        }
    }
}
#[test]
fn depths_are_bounded_and_every_xy_rapid_occurs_at_safe_z() {
    let response = run(fixture(ScanDirection::AlongX)).unwrap();
    let moves = decode(&response.gcode);
    let mut depths: Vec<f64> = moves
        .iter()
        .filter(|m| !m.rapid && m.to[2] < 0.)
        .map(|m| m.to[2])
        .collect();
    depths.sort_by(f64::total_cmp);
    depths.dedup();
    assert_eq!(depths, vec![-1.3, -1., -0.5]);
    for m in &moves {
        if m.rapid && (m.from[0] != m.to[0] || m.from[1] != m.to[1]) {
            assert!(m.from[2] >= 5. && m.to[2] >= 5., "unsafe {m:?}");
        }
        if !m.rapid {
            assert!(m.to[2] >= -1.3);
        }
    }
    assert!(response.toolpath.iter().any(|s| s.kind == MoveKind::Cut));
}
#[test]
fn invalid_tool_depth_schedule_work_area_and_unsupported_context_refuse_output() {
    for case in 0..17 {
        let mut p = fixture(ScanDirection::AlongX);
        match case {
            0 => p.tools[0].kind = ToolKind::BallNose,
            1 => p.tools[0].whirl = true,
            2 => p.operations[0].params.depth = 0.,
            3 => p.operations[0].params.step = Some(1.),
            4 => p.operations[0].params.fast_move_z = 0.,
            5 => p.operations[0].params.start_depth = -0.1,
            6 => p.machine.work_area.x = 10.,
            7 => p.machine.mode = MachineMode::Laser,
            8 => p.stock.as_mut().unwrap().top_z_mm = 1.,
            9 => p.operations[0].side = WorkpieceSide::Back,
            10 => p.operations[0].params.depth_list = vec![-1.],
            11 => p.tools[0].flute_length_mm = Some(1.),
            12 => p.tools[0].diameter = -1.,
            13 => p.tools[0].wear_offset_mm = 7.,
            14 => p.machine.work_area.x = f64::INFINITY,
            16 => p.machine.plot_mode_z = true,
            15 => {
                let OpKind::PlanarSurface { surface } = &mut p.operations[0].kind else {
                    unreachable!()
                };
                surface.stepover_mm = 5.8;
            }
            _ => unreachable!(),
        }
        assert!(run(p).is_err(), "case {case} unexpectedly emitted");
    }
}
#[test]
fn planned_edge_travel_does_not_exempt_cutting_below_stock() {
    let mut p = fixture(ScanDirection::AlongX);
    p.stock.as_mut().unwrap().thickness_mm = 1.;
    let result = run(p);
    assert!(
        result.is_err()
            || result
                .unwrap()
                .warnings
                .iter()
                .any(|w| w.kind == "out_of_stock")
    );
}
#[test]
fn manual_rates_persist_and_drive_shopbot_without_a_recommendation() {
    let mut p = crate::fixture_support::project(fixture(ScanDirection::AlongX));
    for rate in &mut p.operations[0].params.cutting.as_mut().unwrap().rates {
        rate.rpm = Some(16000);
        rate.feed_mm_min = Some(1800);
        rate.plunge_mm_min = Some(300);
    }
    let roundtrip: Project = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
    let response = run(roundtrip).unwrap();
    assert!(response.gcode.contains("16000 RPM"));
    let speeds: Vec<Vec<f64>> = response
        .gcode
        .lines()
        .filter_map(|line| line.strip_prefix("MS,"))
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();
    assert!(speeds.contains(&vec![30., 30.]));
    assert!(speeds.contains(&vec![5., 5.]));
}

#[test]
fn fixture_on_extended_edge_travel_is_still_a_collision() {
    let mut p = fixture(ScanDirection::AlongX);
    p.fixtures.push(Fixture {
        id: 1,
        name: "Edge clamp".into(),
        kind: FixtureKind::Box {
            width: 2.,
            depth: 2.,
        },
        origin: (15., 30.),
        z_bottom: -2.,
        z_top: 4.,
        color: 0xffa0_50c0,
    });
    // ADR-0023 (approved 2026-10-18): a detected fixture collision blocks the
    // executable program; the warning survives only in the diagnostic preview.
    match run(p) {
        Err(PipelineError::FixtureCollision(preview)) => {
            assert!(!preview.toolpath.is_empty());
            assert!(preview
                .warnings
                .iter()
                .any(|w| w.kind == "fixture_path_collision"));
        }
        other => panic!("expected a blocking fixture collision, got {other:?}"),
    }
}
#[test]
fn standalone_generic_output_and_disabled_invalid_op_are_supported() {
    let p = fixture(ScanDirection::AlongY);
    assert!(run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p.clone(),
            post_processor: Some(PostProcessorKind::Hpgl),
            cps_post: None
        }),
        |_, _, _| {}
    )
    .is_err());
    let response = run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p.clone(),
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        }),
        |_, _, _| {},
    )
    .unwrap();
    assert!(response
        .toolpath
        .iter()
        .any(|s| s.op_id == 1 && s.to.z < 0.));
    let mut mixed = p.clone();
    let mut disabled = p.operations[0].clone();
    disabled.id = 2;
    disabled.enabled = false;
    disabled.params.depth = 0.;
    mixed.operations.push(disabled);
    assert!(run(mixed).is_ok());
}

#[test]
fn contour_jobs_retain_their_existing_work_origin_warning() {
    let mut p = fixture(ScanDirection::AlongX);
    p.segments = common::closed_rectangle(20., 30., 40., 40.);
    p.operations.push(op_single_pass(
        2,
        "Profile",
        OpKind::Profile {
            offset: ToolOffset::On,
            profile: ProfileParams::default(),
            contour: ContourParams::default(),
        },
        1,
        -0.5,
    ));
    let response = run(p).unwrap();
    assert!(response
        .warnings
        .iter()
        .any(|w| w.kind == "stock_origin_outside_geometry_bbox"));
}

#[path = "support/explicit.rs"]
mod fixture_support;
