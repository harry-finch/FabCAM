use super::test_helpers::*;
use super::*;
use crate::project::{
    DrillCycle, Fixture, FixtureKind, SpotConfig, StockConfig, ToolOffset, WorkpieceSide,
};
fn request(project: Project) -> PipelineRequest {
    PipelineRequest {
        project,
        post_processor: Some(PostProcessorKind::Linuxcnc),
        cps_post: None,
    }
}
#[test]
fn streaming_rejects_two_sided_through_before_writing() {
    let mut a = profile_op(1, 1, ToolOffset::Outside);
    a.params.depth = -12.;
    let mut b = profile_op(2, 1, ToolOffset::Outside);
    b.side = WorkpieceSide::Back;
    let mut p = project_with(vec![a, b], vec![endmill(1, 3.)]);
    p.stock = Some(StockConfig {
        origin: [0., 0.],
        width_mm: 20.,
        height_mm: 20.,
        thickness_mm: 10.,
        top_z_mm: 0.,
        flip: None,
    });
    assert!(matches!(
        prepare_stream(request(p)),
        Err(StreamGcodeError::Pipeline(
            PipelineError::TwoSidedThrough { .. }
        ))
    ));
}
#[test]
fn streaming_fixture_warnings_match_buffered() {
    let mut p = project_with(
        vec![profile_op(1, 1, ToolOffset::Outside)],
        vec![endmill(1, 3.)],
    );
    p.fixtures.push(Fixture {
        id: 1,
        name: "clamp".into(),
        kind: FixtureKind::Cylinder { radius: 3. },
        origin: (10., -1.5),
        z_bottom: -10.,
        z_top: 8.,
        color: 0,
    });
    assert!(matches!(
        run_pipeline(request(p.clone()), |_, _, _| {}),
        Err(PipelineError::FixtureCollision(_))
    ));
    assert!(stream_gcode_with_preview(request(p), Box::new(std::io::sink())).is_err());
}
#[test]
fn cache_key_tracks_spot_tool_entry_position_state_and_topology() {
    let mut op = drill_op(1, 1, DrillCycle::Simple { dwell_sec: 0. });
    if let OpKind::Drill { spot_first, .. } = &mut op.kind {
        *spot_first = Some(SpotConfig {
            spot_tool_id: 2,
            spot_depth_mm: -0.5,
        });
    }
    let mut p = project_with(vec![op], vec![endmill(1, 3.), endmill(2, 2.)]);
    let key = |p: &Project, position: Point2, topology: u64, state: &str| {
        let objects = segments_to_objects(&p.segments);
        let blobs = GlobalKeyBlobs::new(
            &p.machine,
            &p.fixtures,
            &p.text_layers,
            &p.relief_sources,
            &p.work_offset,
        );
        compute_op_cache_key(
            &p.operations[0],
            p,
            &objects,
            &build_tool_index(&p.tools),
            0,
            Some(&blobs),
            Some((topology, position, state.into())),
        )
        .unwrap()
    };
    let base = key(&p, Point2::new(0., 0.), 1, "coolant off");
    assert_ne!(base, key(&p, Point2::new(10., 0.), 1, "coolant off"));
    assert_ne!(base, key(&p, Point2::new(0., 0.), 2, "coolant off"));
    assert_ne!(base, key(&p, Point2::new(0., 0.), 1, "coolant flood"));
    p.tools[1].plunge_rate += 1;
    assert_ne!(base, key(&p, Point2::new(0., 0.), 1, "coolant off"));
}
#[test]
// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
fn grouped_tool_counts_and_spot_round_trip() {
    let mut p = project_with(
        vec![
            profile_op(1, 1, ToolOffset::Outside),
            profile_op(2, 2, ToolOffset::Outside),
            profile_op(3, 1, ToolOffset::Outside),
        ],
        vec![endmill(1, 3.), endmill(2, 3.)],
    );
    p.tools[0].pause = 2;
    p.tools[1].pause = 3;
    p.group_ops_by_tool = true;
    assert_eq!(count_tool_changes(&p), 2);
    assert_eq!(spindle_warmup_seconds(&p), 5.);
    let mut d = drill_op(1, 1, DrillCycle::Simple { dwell_sec: 0. });
    if let OpKind::Drill { spot_first, .. } = &mut d.kind {
        *spot_first = Some(SpotConfig {
            spot_tool_id: 2,
            spot_depth_mm: -0.5,
        });
    }
    p.operations = vec![d];
    assert_eq!(count_tool_changes(&p), 3);
    assert_eq!(spindle_warmup_seconds(&p), 7.);
}
#[test]
// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
fn included_compact_motion_and_unsupported_decimal_modes_are_distinct() {
    assert!(matches!(
        classify_gcode_include_line("G1X10Y2"),
        GcodeIncludeLineClass::Simulated
    ));
    for code in [
        "G43.1 Z10",
        "G92 Z0",
        "G38.2 Z-5",
        "G33 G90 X10",
        "G90 G33 X10",
        "° G43.1Z10",
    ] {
        assert!(
            matches!(
                classify_gcode_include_line(code),
                GcodeIncludeLineClass::Unsimulated(_)
            ),
            "{code}"
        );
    }
    let moves = preview::interpret("G0Z5\nG43.1Z10\nG92Z0\nG38.2Z-20\nG1X10");
    assert_eq!(moves.last().unwrap().to.z, 5.);
}
#[test]
fn timing_respects_override_and_counts_emitted_warmup_once() {
    let mut p = project_with(
        vec![profile_op(1, 1, ToolOffset::Outside)],
        vec![endmill(1, 3.)],
    );
    p.machine.use_kinematic_time_estimate = false;
    p.tools[0].pause = 0;
    let a = run_pipeline(request(p.clone()), |_, _, _| {}).unwrap();
    p.operations[0].params.feed_rate_override = Some(2000);
    let b = run_pipeline(request(p.clone()), |_, _, _| {}).unwrap();
    assert!(b.time_estimate.cut_s < a.time_estimate.cut_s);
    p.tools[0].pause = 2;
    let c = run_pipeline(request(p), |_, _, _| {}).unwrap();
    let emitted = (c.gcode.matches("G4 P2").count() as f64) * 2.;
    assert!(emitted > 0.);
    assert!((c.time_estimate.total_s - b.time_estimate.total_s - emitted).abs() < 1e-6);
}
