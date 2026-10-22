//! Independent witnesses for the lot-8 report, not implementation mirrors.
use super::test_helpers::{
    closed_square, closed_square_offset, endmill, pocket_op, project_with_segments,
};
use super::*;
use crate::cam::chaining::{classify_containment, segments_to_objects};
use crate::cam::offsets::PolylineOffset;
use crate::cam::source_combine::FrameShape;
use crate::geometry::{Point2, Segment};
use crate::project::{OpSource, PocketStrategy, SourceCombine, TabPlacement, TabPlacementMode};
fn offsets(p: &crate::project::Project) -> Result<Vec<PolylineOffset>, PipelineError> {
    let fixture = crate::fixture_support::project(p.clone());
    let p = &fixture;
    let mut objects = segments_to_objects(&p.segments);
    classify_containment(&mut objects);
    let setup = super::setup_resolver::synthesize_op_setup(&p.operations[0], p, &mut Vec::new())?;
    super::offset_builder::build_op_offsets(
        &p.operations[0],
        p,
        &objects,
        &setup,
        &mut Vec::new(),
        None,
    )
    .map(|v| v.0)
}
fn tabbed_pocket(frame: bool) -> crate::project::Project {
    let mut op = pocket_op(1, 1, OpSource::All);
    let c = op.contour_params_mut().unwrap();
    c.tabs.active = true;
    c.tabs.width = 4.;
    c.tabs.height = 1.;
    c.tab_mode = TabPlacementMode::Manual;
    c.tab_placements = vec![TabPlacement {
        object_id: 1,
        t: 0.125,
        width_override_mm: None,
        height_override_mm: None,
    }];
    if frame {
        let p = op.pocket_params_mut().unwrap();
        p.frame_shape = Some(FrameShape::Rectangle);
        p.frame_padding_mm = Some(8.);
    }
    project_with_segments(closed_square(20.), vec![op], vec![endmill(1, 2.)])
}
#[test]
fn lot8_frame_preserves_manual_holding_tab() {
    let result = offsets(&tabbed_pocket(true)).unwrap();
    assert!(result
        .iter()
        .flat_map(|o| &o.tabs)
        .any(|t| (t.x - 10.).abs() < 1e-4 && t.y.abs() < 3.));
}
#[test]
fn lot8_disjoint_union_keeps_secondary_object_tabs() {
    let mut p = tabbed_pocket(false);
    p.segments.extend(closed_square_offset(20., 40., 0.));
    p.operations[0].source = OpSource::Objects {
        ids: vec![1, 2],
        combine: SourceCombine::Union,
    };
    p.operations[0].contour_params_mut().unwrap().tab_placements[0].object_id = 2;
    let result = offsets(&p).unwrap();
    assert!(result
        .iter()
        .flat_map(|o| &o.tabs)
        .any(|t| (t.x - 50.).abs() < 1e-4 && t.y.abs() < 3.));
}
#[test]
fn lot8_open_inner_stroke_does_not_change_annular_pocket_or_preview() {
    let mut segments = closed_square(40.);
    segments.extend(super::test_helpers::closed_circle(
        Point2::new(20., 20.),
        5.,
    ));
    for strategy in [
        PocketStrategy::Cascade,
        PocketStrategy::Spiral,
        PocketStrategy::Zigzag { angle_deg: 0. },
    ] {
        let mut op = pocket_op(1, 1, OpSource::All);
        if let crate::project::OpKind::Pocket {
            strategy: s,
            pocket,
            ..
        } = &mut op.kind
        {
            *s = strategy;
            pocket.pocket_islands = false;
        }
        let mut p = project_with_segments(segments.clone(), vec![op], vec![endmill(1, 2.)]);
        let before = serde_json::to_value(offsets(&p).unwrap()).unwrap();
        p.segments.push(Segment::line(
            Point2::new(5., 5.),
            Point2::new(10., 5.),
            "0",
            7,
        ));
        assert_eq!(before, serde_json::to_value(offsets(&p).unwrap()).unwrap());
        let mut objects = segments_to_objects(&p.segments);
        classify_containment(&mut objects);
        let regions = super::regions::build_region_previews(&p, &objects);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].holes.len(), 1);
    }
}
#[test]
fn lot8_boolean_erased_holding_contour_refuses_instead_of_retargeting() {
    let mut p = tabbed_pocket(false);
    p.segments.extend(closed_square_offset(20., 10., 0.));
    p.operations[0].source = OpSource::Objects {
        ids: vec![1, 2],
        combine: SourceCombine::Union,
    };
    // Centre remains on the union's bottom edge, but its source contour was
    // changed. A whole-span lineage adapter is not qualified for that change.
    assert!(matches!(offsets(&p), Err(PipelineError::Drawing(_))));
}
fn asymmetric_arcs() -> Vec<Segment> {
    let center = Point2::new(10., 20.);
    let a = Point2::new(12., 20.);
    let b = Point2::new(9., 20. + 3.0_f64.sqrt());
    vec![
        Segment::arc(
            a,
            b,
            (std::f64::consts::PI / 6.).tan(),
            Some(center),
            "0",
            7,
        ),
        Segment::arc(
            b,
            a,
            (std::f64::consts::PI / 3.).tan(),
            Some(center),
            "0",
            7,
        ),
    ]
}
#[test]
fn lot8_analytic_arc_circle_is_not_a_lens_or_multiple_winding() {
    use crate::cam::offsets::small_circle_drill;
    use crate::cam::VcObject;
    let segments = asymmetric_arcs();
    let circle = VcObject::new(segments.clone(), true);
    let drill = small_circle_drill(&circle, 2.).unwrap();
    let mut uncached = segments.clone();
    for s in &mut uncached {
        s.center = None;
    }
    assert!(small_circle_drill(&VcObject::new(uncached, true), 2.).is_some());
    let mut stale_center = segments.clone();
    stale_center[0].center = Some(Point2::new(0., 0.));
    assert!(small_circle_drill(&VcObject::new(stale_center, true), 2.).is_none());
    assert!(drill.segments[0].start.distance(Point2::new(10., 20.)) < 1e-12);
    let mut doubled = segments.clone();
    doubled.extend(segments.clone());
    assert!(small_circle_drill(&VcObject::new(doubled, true), 2.).is_none());
    let mut lens = segments.clone();
    lens[1].bulge = -(std::f64::consts::PI / 6.).tan();
    assert!(small_circle_drill(&VcObject::new(lens, true), 2.).is_none());
    let cw = segments
        .iter()
        .rev()
        .map(|s| Segment::arc(s.end, s.start, -s.bulge, s.center, "0", 7))
        .collect();
    assert!(small_circle_drill(&VcObject::new(cw, true), 2.).is_some());
}
#[test]
fn lot8_drill_arc_bounds_are_centered_and_concave_outside_reference_refuses() {
    use super::test_helpers::drill_op;
    use crate::project::DrillCycle;
    let op = drill_op(1, 1, DrillCycle::Simple { dwell_sec: 0. });
    let mut p = project_with_segments(asymmetric_arcs(), vec![op], vec![endmill(1, 2.)]);
    // Hole radius 2 > tool radius 1: exercise bbox-centre fallback, not the
    // small-circle substitution. Arc extrema must restore the true centre.
    let result = offsets(&p).unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].segments[0].start.distance(Point2::new(10., 20.)) < 1e-12);
    let pts = [
        (0., 0.),
        (10., 0.),
        (10., 2.),
        (2., 2.),
        (2., 8.),
        (10., 8.),
        (10., 10.),
        (0., 10.),
    ];
    p.segments = pts
        .iter()
        .zip(pts.iter().cycle().skip(1))
        .take(pts.len())
        .map(|(a, b)| Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "0", 7))
        .collect();
    assert!(matches!(offsets(&p), Err(PipelineError::Drawing(_))));
}
#[test]
fn lot8_uniform_tab_phase_avoids_every_rectangle_corner_without_changing_spacing() {
    let pts = vec![
        Point2::new(0., 0.),
        Point2::new(30., 0.),
        Point2::new(30., 20.),
        Point2::new(0., 20.),
    ];
    let phase = crate::cam::tabs::corner_safe_tab_phase(&pts, 4, 10., 5.000_001).unwrap();
    for i in 0..4 {
        let t = (f64::from(i) * 25. + phase).rem_euclid(100.);
        for corner in [0., 30., 50., 80.] {
            let distance = (t - corner).abs();
            assert!(distance.min(100. - distance) >= 5.);
        }
    }
    let tiny = vec![
        Point2::new(0., 0.),
        Point2::new(5., 0.),
        Point2::new(5., 5.),
        Point2::new(0., 5.),
    ];
    assert!(crate::cam::tabs::corner_safe_tab_phase(&tiny, 1, 12., 6.000_001).is_none());
}
#[test]
// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
fn lot8_frame_tab_survives_final_resolved_native_cut_height() {
    let mut p = tabbed_pocket(true);
    p.segments = closed_square_offset(20., 20., 20.);
    p.operations[0].params.depth = -4.;
    p.operations[0].params.step = Some(-2.);
    let response = run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        }),
        |_, _, _| {},
    )
    .unwrap();
    // Select the actual nearest horizontal wall pass: the existing hole
    // erosion includes its declared sagitta/grid pad, so nominal y=19 is not
    // necessarily an emitted coordinate. This is a witness-selection window,
    // not a relaxed Z/holding-height assertion.
    let wall_y = response
        .toolpath
        .iter()
        .filter(|m| {
            m.kind == crate::gcode::preview::MoveKind::Cut
                && m.from.y == m.to.y
                && m.from.y < 20.
                && m.from.x.min(m.to.x) <= 30.
                && m.from.x.max(m.to.x) >= 30.
        })
        .map(|m| m.from.y)
        .max_by(f64::total_cmp)
        .unwrap();
    assert!(wall_y <= 19.0001 && wall_y > 18.);
    let mut witnesses = 0;
    for m in &response.toolpath {
        if m.kind != crate::gcode::preview::MoveKind::Cut
            || (m.from.y - wall_y).abs() > 1e-9
            || (m.to.y - wall_y).abs() > 1e-9
        {
            continue;
        }
        if m.from.x.min(m.to.x) <= 30.
            && m.from.x.max(m.to.x) >= 30.
            && (m.to.x - m.from.x).abs() > 1e-9
        {
            let t = (30. - m.from.x) / (m.to.x - m.from.x);
            let z = m.from.z + t * (m.to.z - m.from.z);
            assert!(z >= -3.0001, "tab bridge cut through at {z}");
            witnesses += 1;
        }
    }
    assert!(witnesses > 0, "no emitted cut crossed the holding bridge");
}

#[cfg(feature = "cps")]
#[test]
fn lot8_same_tool_finish_rpm_change_is_refused_not_silently_ignored_by_shopbot() {
    let mut p = project_with_segments(
        closed_square(40.),
        vec![pocket_op(1, 1, OpSource::All)],
        vec![endmill(1, 2.)],
    );
    p.tools[0].speed = 18000;
    p.tools[0].speed_finish = Some(12000);
    let result = run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: BTreeMap::default(),
            }),
        }),
        |_, _, _| {},
    );
    let error = result.unwrap_err().to_string();
    assert!(error.contains("unsupported record spindleSpeed"), "{error}");
}

#[test]
fn lot8_frame_preview_uses_effective_radius_like_the_path_builder() {
    let mut op = pocket_op(1, 1, OpSource::All);
    op.pocket_params_mut().unwrap().frame_shape = Some(FrameShape::Rectangle);
    let mut tool = endmill(1, 4.);
    tool.wear_offset_mm = 2.;
    let p = project_with_segments(closed_square(20.), vec![op], vec![tool]);
    let mut objects = segments_to_objects(&p.segments);
    classify_containment(&mut objects);
    let regions = super::regions::build_region_previews(&p, &objects);
    assert_eq!(regions.len(), 1);
    let min = regions[0]
        .outer
        .iter()
        .map(|p| p.x)
        .min_by(f64::total_cmp)
        .unwrap();
    assert!(
        (min + 1.).abs() < 1e-4,
        "frame used nominal rather than effective radius: {min}"
    );
}
#[test]
fn lot8_impossible_auto_corner_phase_warns_without_erasing_tabs_or_manual_intent() {
    let mut p = tabbed_pocket(false);
    p.segments = closed_square(5.);
    p.operations[0].contour_params_mut().unwrap().tabs.width = 12.;
    let objects = segments_to_objects(&p.segments);
    let mut warnings = Vec::new();
    let manual = super::tabs::build_op_tabs_by_object(&p.operations[0], &objects, &mut warnings);
    assert_eq!(manual[&0].len(), 1);
    assert!((manual[&0][0].x - 2.5).abs() < 1e-12);
    assert!(!warnings
        .iter()
        .any(|w| w.kind == "tabs_corner_clearance_unavailable"));
    p.operations[0].contour_params_mut().unwrap().tab_mode = TabPlacementMode::Auto { count: 1 };
    let automatic = super::tabs::build_op_tabs_by_object(&p.operations[0], &objects, &mut warnings);
    assert_eq!(automatic[&0].len(), 1);
    assert!(warnings
        .iter()
        .any(|w| w.kind == "tabs_corner_clearance_unavailable"));
}
