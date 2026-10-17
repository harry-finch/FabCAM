use super::*;
use crate::cam::VcObject;
use crate::geometry::Point2;
use crate::project::{tool::SpindleDirection, CutDirection};
fn p(x: f64, y: f64) -> Point2 {
    Point2::new(x, y)
}
fn square() -> VcObject {
    VcObject::new(
        vec![
            Segment::line(p(0., 0.), p(200., 0.), "0", 7),
            Segment::line(p(200., 0.), p(200., 200.), "0", 7),
            Segment::line(p(200., 200.), p(0., 200.), "0", 7),
            Segment::line(p(0., 200.), p(0., 0.), "0", 7),
        ],
        true,
    )
}
#[test]
fn audit_two_arc_winding_all_spindle_directions() {
    let obj = VcObject::new(
        vec![
            Segment::arc(p(10., 0.), p(-10., 0.), 1., Some(p(0., 0.)), "0", 7),
            Segment::arc(p(-10., 0.), p(10., 0.), 1., Some(p(0., 0.)), "0", 7),
        ],
        true,
    );
    for context in [CutContext::Inner, CutContext::Outer] {
        for spindle in [SpindleDirection::Cw, SpindleDirection::Ccw] {
            for direction in [CutDirection::Climb, CutDirection::Conventional] {
                let mut offset = parallel_offset_inward(&obj, 1.).remove(0);
                enforce_winding(&mut offset, context, direction, spindle);
                let ccw = (context == CutContext::Outer) == (direction == CutDirection::Climb);
                let ccw = ccw != (spindle == SpindleDirection::Ccw);
                assert_eq!(offset_signed_area(&offset) > 0., ccw);
                assert!(
                    (offset_signed_area(&offset).abs() - std::f64::consts::PI * 81.).abs() < 1e-8
                );
            }
        }
    }
}
#[test]
fn audit_bridge_rejects_subsample_notch() {
    let poly = vec![
        p(0., 0.),
        p(40., 0.),
        p(40., 20.),
        p(19., 20.),
        p(19., 5.),
        p(16., 5.),
        p(16., 20.),
        p(0., 20.),
    ];
    assert!(!bridge_stays_inside_polygon(p(0., 10.), p(40., 10.), &poly));
    assert!(bridge_stays_inside_polygon(p(0., 2.), p(40., 2.), &poly));
    assert!(bridge_stays_inside_polygon(p(0., 0.), p(40., 0.), &poly));
}
#[test]
fn audit_island_wall_has_outer_context() {
    let islands = inflate_islands_by_tool_radius(
        &[vec![p(80., 80.), p(120., 80.), p(120., 120.), p(80., 120.)]],
        3.,
    );
    let offsets = pocket_for_object(
        &square(),
        3.,
        false,
        16,
        PocketEmit::Zigzag { angle_deg: 0. },
        &islands,
        3.,
        0.,
        None,
        SpindleDirection::Cw,
    );
    let islands: Vec<_> = offsets
        .iter()
        .filter(|o| o.closed && o.level == 0 && o.is_pocket == 2)
        .collect();
    assert!(!islands.is_empty());
    assert!(islands.iter().all(|o| offset_signed_area(o) < 0.));
}
#[test]
fn audit_approach_projects_onto_long_side() {
    let _ = take_offset_diagnostics();
    let mut offsets = parallel_offset_inward(&square(), 3.);
    rotate_offsets_to_approach_point(&mut offsets, (100., 3.));
    assert!(offsets[0].segments[0].start.distance(p(100., 3.)) < 1e-9);
    assert!(take_offset_diagnostics().approach_point_far.is_empty());
}
#[test]
fn audit_waterline_circle_clearance() {
    let ring: Vec<_> = (0..128)
        .map(|i| {
            let a = i as f64 * std::f64::consts::TAU / 128.;
            p(10. * a.cos(), 10. * a.sin())
        })
        .collect();
    let chains = crate::cam::waterline::clear_level(&[ring], 6., 1.);
    assert!(!chains.is_empty());
    for s in chains.iter().flatten() {
        for q in [s.start, s.end] {
            assert!(q.x.hypot(q.y) <= 7.0001, "{q:?}");
        }
    }
}

#[test]
fn audit_approach_splits_arcs_without_changing_shape_or_tabs() {
    for bulge in [1.0, -1.0, 2.0, -2.0] {
        let start = p(10., 0.);
        let end = p(-10., 0.);
        let obj = VcObject::new(
            vec![
                Segment::arc(start, end, bulge, None, "curved", 4),
                Segment::line(end, start, "curved", 4),
            ],
            true,
        );
        let mut offset = PolylineOffset {
            segments: obj.segments.clone(),
            closed: true,
            level: 0,
            is_pocket: 0,
            layer: "curved".into(),
            color: 4,
            source_object_idx: 17,
            tabs: vec![TabPoint {
                x: 10.,
                y: 0.,
                width_override_mm: Some(2.),
                height_override_mm: Some(1.),
            }],
            is_finish: true,
        };
        let before = offset_signed_area(&offset);
        let (c, _, _, _) = crate::math::bulge_to_arc(start, end, bulge);
        let angle = (start.y - c.y).atan2(start.x - c.x) + 2. * bulge.atan();
        let r = start.distance(c);
        let q = p(c.x + r * angle.cos(), c.y + r * angle.sin());
        rotate_offsets_to_approach_point(std::slice::from_mut(&mut offset), (q.x, q.y));
        assert!(offset.segments[0].start.distance(q) < 1e-8);
        assert!((offset_signed_area(&offset) - before).abs() < 1e-7);
        assert_eq!(offset.segments.len(), 3);
        assert_eq!(offset.source_object_idx, 17);
        assert!(offset.is_finish);
        assert_eq!(offset.tabs[0].width_override_mm, Some(2.));
        for i in 0..offset.segments.len() {
            assert!(
                offset.segments[i]
                    .end
                    .distance(offset.segments[(i + 1) % offset.segments.len()].start)
                    < 1e-8
            );
        }
    }
}

#[test]
fn audit_zigzag_additional_clearance_is_geometric_and_rotation_invariant() {
    let ring: Vec<_> = (0..128)
        .map(|i| {
            let a = i as f64 * std::f64::consts::TAU / 128.;
            p(10. * a.cos(), 10. * a.sin())
        })
        .collect();
    for angle in [0., 17., 45., 90.] {
        let chains = pocket_zigzag_inset(&ring, &[], 0.7, 3., angle);
        assert!(!chains.is_empty());
        for seg in chains.iter().flatten() {
            for q in [seg.start, seg.end] {
                assert!(q.x.hypot(q.y) <= 7.0001);
            }
        }
    }
}

#[test]
fn audit_offset_island_chords_keep_clearance() {
    let boundary = vec![p(-20., -20.), p(20., -20.), p(20., 20.), p(-20., 20.)];
    let hole = vec![p(-2., -2.), p(2., -2.), p(2., 2.), p(-2., 2.)];
    for rings in [
        boundary_offset_inward(&boundary, std::slice::from_ref(&hole), 3.),
        pocket_cascade_with_islands(&boundary, std::slice::from_ref(&hole), 3.),
    ] {
        let mut checked = 0;
        for ring in rings {
            if signed_area(&ring) >= 0. {
                continue;
            }
            for i in 0..ring.len() {
                for j in 0..=20 {
                    let a = ring[i];
                    let b = ring[(i + 1) % ring.len()];
                    let t = j as f64 / 20.;
                    let q = p(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t);
                    let d = (q.x.abs() - 2.).max(0.).hypot((q.y.abs() - 2.).max(0.));
                    assert!(d >= 3. - 1e-6, "island chord clearance={d}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 0);
    }
}

#[test]
fn audit_waterline_nested_holes_and_split_domains_stay_clear() {
    let outer = vec![p(0., 0.), p(40., 0.), p(40., 40.), p(0., 40.)];
    let hole = vec![p(18., -2.), p(22., -2.), p(22., 38.), p(18., 38.)];
    // Geometric erosion splits the center domain when a hole joins a wall.
    let chains = pocket_zigzag_inset(&outer, &[hole], 1., 3., 0.);
    assert!(!chains.is_empty());
    for s in chains.iter().flatten() {
        for j in 0..=20 {
            let t = j as f64 / 20.;
            let q = p(
                s.start.x + (s.end.x - s.start.x) * t,
                s.start.y + (s.end.y - s.start.y) * t,
            );
            assert!(q.x <= 15.0001 || q.x >= 24.9999);
        }
    }
}

#[test]
fn audit_cw_pocket_boundary_context_is_normalized() {
    let mut obj = square();
    obj.segments.reverse();
    for s in &mut obj.segments {
        std::mem::swap(&mut s.start, &mut s.end);
    }
    let offsets = pocket_for_object(
        &obj,
        3.,
        false,
        16,
        PocketEmit::Zigzag { angle_deg: 0. },
        &[],
        3.,
        0.,
        None,
        SpindleDirection::Cw,
    );
    assert!(offsets
        .iter()
        .filter(|o| o.closed && o.level == 0)
        .all(|o| offset_signed_area(o) > 0.));
}

#[test]
fn audit_pocket_wall_directions_m3_m4_climb_conventional() {
    use crate::project::{ContourParams, Op, OpKind, PocketStrategy};
    let islands = inflate_islands_by_tool_radius(
        &[vec![p(80., 80.), p(120., 80.), p(120., 120.), p(80., 120.)]],
        3.,
    );
    let base = pocket_for_object(
        &square(),
        3.,
        false,
        16,
        PocketEmit::Zigzag { angle_deg: 0. },
        &islands,
        3.,
        0.,
        None,
        SpindleDirection::Cw,
    );
    for spindle in [SpindleDirection::Cw, SpindleDirection::Ccw] {
        for direction in [CutDirection::Climb, CutDirection::Conventional] {
            let op = Op {
                kind: OpKind::Pocket {
                    strategy: PocketStrategy::Zigzag { angle_deg: 0.0 },
                    contour: ContourParams {
                        cut_direction: direction,
                        finish_cut_direction: direction,
                        ..Default::default()
                    },
                    pocket: Default::default(),
                },
                ..Default::default()
            };
            let mut offsets = base.clone();
            apply_cut_direction(&mut offsets, &op, false, spindle);
            for o in offsets.iter().filter(|o| o.closed && o.level == 0) {
                let outer = o.is_pocket == 2;
                let expected = (outer == (direction == CutDirection::Climb))
                    != (spindle == SpindleDirection::Ccw);
                assert_eq!(offset_signed_area(o) > 0., expected);
            }
        }
    }
}

#[test]
fn audit_bridge_through_island_vertices_between_samples_is_rejected() {
    let diamond = vec![p(16., 10.), p(17., 11.), p(18., 10.), p(17., 9.)];
    assert!(bridge_crosses_any_island(
        p(0., 10.),
        p(40., 10.),
        std::slice::from_ref(&diamond)
    ));
    assert!(!bridge_crosses_any_island(
        p(0., 2.),
        p(40., 2.),
        &[diamond]
    ));
}
