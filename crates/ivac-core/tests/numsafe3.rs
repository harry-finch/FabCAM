//! Bounded semantic/resource regressions. No machine execution.
#![allow(clippy::float_cmp)]
mod common;
use ivac_core::cam::{
    offsets::{pocket_zigzag, pocket_zigzag_bounded},
    surface::SurfaceField,
    surface_mill::{
        drop_cutter_cancellable, surface_mill_cancellable, ScanDirection, SurfaceMillParams,
    },
    thread::helix_waypoints_checked,
    waterline::checked_z_levels,
};
use ivac_core::cancel::CancelToken;
use ivac_core::geometry::Point2;
use ivac_core::project::tool::SpindleDirection;

#[test]
fn pipeline_refuses_thread_budget_and_recovers_without_cached_partial_success() {
    use common::{build_project, closed_circle, endmill_tool, op_single_pass, stock_at_origin};
    use ivac_core::pipeline::{run_pipeline, PipelineRequest, PostProcessorKind};
    use ivac_core::project::OpKind;
    let generate = |pitch, passes| {
        let op = op_single_pass(
            1,
            "thread",
            OpKind::Thread {
                pitch_mm: pitch,
                internal: true,
                climb: true,
                radial_passes: passes,
                start_angle_rad: 0.,
                thread_depth_mm: Some(0.5),
            },
            1,
            -2.,
        );
        run_pipeline(
            PipelineRequest {
                project: build_project(
                    &stock_at_origin(100., 100., 18.),
                    vec![endmill_tool(1, 2.)],
                    op,
                    closed_circle(30., 30., 5.),
                ),
                post_processor: Some(PostProcessorKind::Linuxcnc),
                cps_post: None,
            },
            |_, _, _| {},
        )
    };
    let baseline = generate(1., 1).unwrap().gcode;
    assert!(generate(1e-300, 1)
        .unwrap_err()
        .to_string()
        .contains("1000000"));
    assert!(generate(1., u32::MAX)
        .unwrap_err()
        .to_string()
        .contains("1000000"));
    assert_eq!(generate(1., 1).unwrap().gcode, baseline);
}

#[test]
fn waterline_boundary_and_finite_stall_refuse_without_large_allocation() {
    assert_eq!(checked_z_levels(0., -1000., 1.).unwrap().len(), 1000);
    assert!(checked_z_levels(0., -1001., 1.).is_err());
    assert!(checked_z_levels(-1e20, -2e20, 1.).is_err());
    assert!(checked_z_levels(0., -1., 1e-300).is_err());
    assert_eq!(checked_z_levels(10., 0., 3.).unwrap(), vec![7., 4., 1., 0.]);
}

#[test]
fn thread_tiny_pitch_refuses_and_cancel_does_not_poison_regeneration() {
    let token = CancelToken::new();
    let generate = |pitch, cancel| {
        helix_waypoints_checked(
            Point2::new(0., 0.),
            2.,
            0.,
            -2.,
            pitch,
            true,
            true,
            1.,
            64,
            0.,
            SpindleDirection::Cw,
            cancel,
        )
    };
    assert!(generate(1e-300, None).is_err());
    let baseline = generate(1., None).unwrap();
    assert_eq!(baseline.len(), 130);
    assert_eq!(baseline[128].2, -2.);
    token.cancel();
    assert_eq!(generate(1., Some(&token)).unwrap_err(), "cancelled");
    assert_eq!(generate(1., None).unwrap(), baseline);
}

#[test]
fn relief_footprint_work_and_sampling_refuse_independently() {
    let tiny_cell = SurfaceField::new(Point2::new(0., 0.), 0.0001, 2, 2, vec![-1.; 4]);
    assert!(drop_cutter_cancellable(&tiny_cell, 3., 3., None).is_err());
    let field = SurfaceField::new(Point2::new(0., 0.), 1., 101, 101, vec![-1.; 10201]);
    // About 31k footprint entries, but >300M field×footprint work.
    assert!(drop_cutter_cancellable(&field, 100., 100., None)
        .unwrap_err()
        .contains("evaluations"));
    let shifted = SurfaceField::new(Point2::new(1e16, 0.), 1., 10, 2, vec![-1.; 20]);
    let params = SurfaceMillParams {
        tool_radius_mm: 0.1,
        corner_radius_mm: 0.1,
        scallop_height_mm: 0.1,
        stepover_mm: Some(1.),
        along_step_mm: 0.25,
        direction: ScanDirection::AlongX,
        z_floor_mm: -1.,
        z_top_mm: 0.,
    };
    assert!(surface_mill_cancellable(&shifted, &params, None).is_err());
    let normal = SurfaceField::new(Point2::new(0., 0.), 1., 10, 2, vec![-1.; 20]);
    let baseline = surface_mill_cancellable(&normal, &params, None)
        .unwrap()
        .unwrap();
    let token = CancelToken::new();
    token.cancel();
    assert!(surface_mill_cancellable(&normal, &params, Some(&token))
        .unwrap()
        .is_none());
    assert_eq!(
        surface_mill_cancellable(&normal, &params, None)
            .unwrap()
            .unwrap(),
        baseline
    );
}

#[test]
fn bounded_clearing_matches_legacy_geometry_and_refuses_partial_result() {
    let boundary = vec![
        Point2::new(0., 0.),
        Point2::new(10., 0.),
        Point2::new(10., 10.),
        Point2::new(0., 10.),
    ];
    let baseline = pocket_zigzag(&boundary, &[], 1., 1.);
    let exact_points: usize = baseline.iter().map(|chain| chain.len() + 1).sum();
    let mut remaining = exact_points;
    let bounded =
        pocket_zigzag_bounded(&boundary, &[], 1., 1., Some(&mut remaining), None).unwrap();
    assert_eq!(remaining, 0);
    assert_eq!(
        serde_json::to_value(bounded).unwrap(),
        serde_json::to_value(&baseline).unwrap()
    );
    let mut too_small = exact_points - 1;
    assert!(pocket_zigzag_bounded(&boundary, &[], 1., 1., Some(&mut too_small), None).is_err());
    let token = CancelToken::new();
    token.cancel();
    assert!(pocket_zigzag_bounded(&boundary, &[], 1., 1., None, Some(&token)).is_err());
    assert_eq!(
        serde_json::to_value(pocket_zigzag(&boundary, &[], 1., 1.)).unwrap(),
        serde_json::to_value(baseline).unwrap()
    );
}

#[test]
fn relief_retained_points_are_bounded_before_scanline_materialization() {
    let field = SurfaceField::new(Point2::new(0., 0.), 1., 400, 400, vec![-1.; 160_000]);
    let params = SurfaceMillParams {
        tool_radius_mm: 0.1,
        corner_radius_mm: 0.1,
        scallop_height_mm: 0.1,
        stepover_mm: Some(0.5),
        along_step_mm: 0.25,
        direction: ScanDirection::AlongX,
        z_floor_mm: -1.,
        z_top_mm: 0.,
    };
    assert!(surface_mill_cancellable(&field, &params, None)
        .unwrap_err()
        .contains("retained XYZ"));
}

#[test]
fn representative_relief_remains_within_initial_budgets() {
    // 100mm-class imported surface at the UI's usual 256-cell resolution.
    let field = SurfaceField::new(Point2::new(0., 0.), 0.4, 256, 256, vec![-1.; 65_536]);
    let params = SurfaceMillParams {
        tool_radius_mm: 3.,
        corner_radius_mm: 3.,
        scallop_height_mm: 0.1,
        stepover_mm: None,
        along_step_mm: 0.4,
        direction: ScanDirection::AlongX,
        z_floor_mm: -1.,
        z_top_mm: 0.,
    };
    let paths = surface_mill_cancellable(&field, &params, None)
        .unwrap()
        .unwrap();
    assert!(!paths.is_empty());
    assert!(paths.iter().map(Vec::len).sum::<usize>() < 1_000_000);
    assert!(paths.iter().flatten().all(|p| p.2 >= -1. && p.2 <= 0.));
}

#[test]
fn thread_whirling_obeys_remaining_operation_budget() {
    use ivac_core::gcode::face_mill_overlay::{
        apply_whirl_to_polyline, apply_whirl_to_polyline_checked, WhirlParams, WhirlState,
    };
    let points = [(0., 0., 0.), (1., 0., -1.)];
    let params = WhirlParams {
        radius: 0.1,
        stepover: 0.1,
        osc: 0.,
        climb: true,
    };
    assert!(
        apply_whirl_to_polyline_checked(&points, params, &mut WhirlState::default(), 10, None,)
            .is_err()
    );
    let baseline = apply_whirl_to_polyline(&points, params, &mut WhirlState::default());
    assert_eq!(
        apply_whirl_to_polyline_checked(
            &points,
            params,
            &mut WhirlState::default(),
            baseline.len(),
            None,
        )
        .unwrap(),
        baseline
    );
}
