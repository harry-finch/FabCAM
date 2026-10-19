//! Independent analytical references for the GEOM1 contract. No CAM snapshots.
// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::float_cmp)]
use ivac_core::drawing::{Binding, Contour, Drawing, Identity, Source, Transform, Transition};
use ivac_core::geometry::{Point2, Segment};
fn line(a: (f64, f64), b: (f64, f64)) -> Segment {
    Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "cut", 7)
}
fn source(id: u32, segments: Vec<Segment>) -> Source {
    Source {
        id,
        original: segments,
        legacy_objects: vec![],
        pivot: Point2::new(0., 0.),
        transform: Transform::default(),
        next_entity: 1,
    }
}
fn two() -> Drawing {
    Drawing::from_sources(vec![
        source(1, vec![line((0., 0.), (1., 0.)), line((3., 0.), (4., 0.))]),
        source(2, vec![line((0., 0.), (1., 0.))]),
    ])
    .unwrap()
}
#[test]
fn identical_imports_have_distinct_identity_and_exact_provenance() {
    let d = two();
    let e = d.evaluate().unwrap();
    assert_eq!(e.contours.len(), 3);
    assert_ne!(e.contours[0], e.contours[2]);
    assert_eq!(d.contours[0].edges[0].original_indices, vec![0]);
    assert_eq!(d.contours[1].edges[0].original_indices, vec![1]);
    assert_eq!(d.contours[2].edges[0].original_indices, vec![0]);
}
#[test]
fn reversed_chaining_retains_source_ordinals() {
    let d = Drawing::from_sources(vec![source(
        1,
        vec![line((0., 0.), (1., 0.)), line((2., 0.), (1., 0.))],
    )])
    .unwrap();
    assert_eq!(d.contours.len(), 1);
    assert_eq!(d.contours[0].edges[1].geometry.start, Point2::new(1., 0.));
    assert_eq!(d.contours[0].edges[1].original_indices, vec![1]);
    assert_eq!(d.imports[0].original[1].start, Point2::new(2., 0.));
}
#[test]
fn unique_modification_preserves_binding_and_original() {
    let mut d = two();
    let id = d.contours[0].id;
    let b = Binding::Contours { ids: vec![id] };
    d.contours[0].edges[0].geometry.end.x = 1.1;
    assert_eq!(d.preserve_binding(&b), b);
    assert_eq!(d.imports[0].original[0].end.x, 1.);
    assert_eq!(d.evaluate().unwrap().resolve(&b).unwrap(), vec![0]);
}
#[test]
fn deletion_does_not_retarget_to_the_next_import() {
    let mut d = two();
    let deleted = d.contours.remove(0);
    let other = d.contours[1].id;
    d.transitions.push(Transition {
        parents: vec![deleted.id],
        children: vec![],
    });
    assert!(matches!(
        d.preserve_binding(&Binding::Contours {
            ids: vec![deleted.id]
        }),
        Binding::Unresolved { .. }
    ));
    let e = d.evaluate().unwrap();
    assert_eq!(
        e.resolve(&Binding::Contours { ids: vec![other] }).unwrap(),
        vec![1]
    );
    assert!(e
        .resolve(&Binding::Contours {
            ids: vec![deleted.id]
        })
        .is_err());
}
#[test]
fn merge_requires_review_even_if_both_parents_were_selected() {
    let mut d = two();
    let b = d.contours.remove(1);
    let a = d.contours.remove(0);
    let parents = vec![a.id, b.id];
    let id = Identity {
        import: 1,
        entity: d.imports[0].next_entity,
    };
    d.imports[0].next_entity += 1;
    let mut edges = a.edges;
    edges.extend(b.edges);
    d.contours.insert(
        0,
        Contour {
            id,
            parents: parents.clone(),
            edges,
        },
    );
    d.transitions.push(Transition {
        parents: parents.clone(),
        children: vec![id],
    });
    d.validate().unwrap();
    assert!(matches!(
        d.preserve_binding(&Binding::Contours { ids: parents }),
        Binding::Unresolved { .. }
    ));
}
#[test]
fn split_does_not_automatically_select_children() {
    let mut d = Drawing::from_sources(vec![source(
        1,
        vec![line((0., 0.), (1., 0.)), line((1., 0.), (2., 0.))],
    )])
    .unwrap();
    let old = d.contours.remove(0);
    let parent = old.id;
    let mut children = vec![];
    for edge in old.edges {
        let id = Identity {
            import: 1,
            entity: d.imports[0].next_entity,
        };
        d.imports[0].next_entity += 1;
        d.contours.push(Contour {
            id,
            parents: vec![parent],
            edges: vec![edge],
        });
        children.push(id);
    }
    d.transitions.push(Transition {
        parents: vec![parent],
        children,
    });
    d.validate().unwrap();
    assert!(matches!(
        d.preserve_binding(&Binding::Contours { ids: vec![parent] }),
        Binding::Unresolved { .. }
    ));
}
#[test]
fn evaluated_coordinates_use_frozen_pivot_and_similarity_transform() {
    let mut d = Drawing::from_sources(vec![source(1, vec![line((11., 20.), (12., 20.))])]).unwrap();
    d.imports[0].pivot = Point2::new(10., 20.);
    d.imports[0].transform = Transform {
        translate: Point2::new(100., 200.),
        rotation_deg: 90.,
        scale: 2.,
        mirror_x: false,
        mirror_y: false,
    };
    let e = d.evaluate().unwrap();
    assert!((e.segments[0].start.x - 110.).abs() < 1e-12);
    assert!((e.segments[0].start.y - 222.).abs() < 1e-12);
    d.contours[0].edges[0].geometry.end.x = 13.;
    assert_eq!(d.evaluate().unwrap().segments[0].start, e.segments[0].start);
}
#[test]
fn distance_thresholds_are_visible_mm_independent_of_rotation_translation_mirror() {
    for scale in [0.001, 1., 1000.] {
        let t = Transform {
            scale,
            rotation_deg: 37.,
            translate: Point2::new(1e9, -1e9),
            mirror_x: true,
            mirror_y: false,
        };
        for (gap, expected) in [(0.099_999, false), (0.100_001, true)] {
            let d = t.distance_mm(Point2::new(0., 0.), Point2::new(gap / scale, 0.));
            assert_eq!(d > 0.1, expected);
            assert!((d - gap).abs() < 1e-14);
        }
    }
    assert_eq!(
        Transform::default().distance_mm(Point2::new(0., 0.), Point2::new(0.1, 0.)),
        0.1
    );
}
#[test]
fn invalid_revision_identity_transform_and_unresolved_binding_refuse() {
    let mut d = two();
    d.version = 2;
    assert!(d.evaluate().is_err());
    d.version = 1;
    d.imports[0].transform.scale = 0.;
    assert!(d.evaluate().is_err());
    d.imports[0].transform.scale = 1.;
    d.contours[1].id = d.contours[0].id;
    assert!(d.evaluate().is_err());
    let d = two();
    assert!(d
        .evaluate()
        .unwrap()
        .resolve(&Binding::Unresolved {
            previous: vec![],
            reason: "review".into()
        })
        .is_err());
    assert!(serde_json::from_str::<Transform>(r#"{"translate":{"x":0,"y":0},"rotation_deg":0,"scale":1,"mirror_x":false,"mirror_y":false,"scale_y":2}"#).is_err());
}

#[test]
fn gap_preview_is_nonmutating_and_matches_inclusive_visible_tolerance() {
    use ivac_core::drawing::repair::Repair;
    let d = Drawing::from_sources(vec![source(
        1,
        vec![
            line((0., 0.), (1., 0.)),
            line((1., 0.), (1., 1.)),
            line((1., 1.), (0., 1.)),
            line((0., 1.), (0., 0.1)),
        ],
    )])
    .unwrap();
    let id = d.contours[0].id;
    let edge = d.contours[0].edges.last().unwrap().id;
    let r = Repair::Gap {
        contour: id,
        edge,
        connector: false,
        tolerance_mm: 0.1,
    };
    let (after, _) = d.preview_repairs(&[r], &[]).unwrap();
    assert_eq!(d.contours[0].edges.last().unwrap().geometry.end.y, 0.1);
    assert_eq!(after.contours[0].edges.last().unwrap().geometry.end.y, 0.);
    assert!(after.evaluate().unwrap().closed[0]);
    assert!(d
        .preview_repairs(
            &[Repair::Gap {
                contour: id,
                edge,
                connector: false,
                tolerance_mm: 0.099_999
            }],
            &[]
        )
        .is_err());
}
#[test]
fn crossing_result_is_refused_even_within_join_tolerance() {
    use ivac_core::drawing::repair::Repair;
    let d = Drawing::from_sources(vec![source(
        1,
        vec![
            line((0., 0.), (2., 2.)),
            line((2., 2.), (0., 2.)),
            line((0., 2.), (2., 0.)),
            line((2., 0.), (0.1, 0.)),
        ],
    )])
    .unwrap();
    let c = &d.contours[0];
    assert!(d
        .preview_repairs(
            &[Repair::Gap {
                contour: c.id,
                edge: c.edges.last().unwrap().id,
                connector: true,
                tolerance_mm: 0.1
            }],
            &[]
        )
        .is_err());
}

mod common;
#[test]
fn edited_generation_uses_authoritative_geometry_and_refuses_old_request_shape() {
    use common::{build_project, op_single_pass, stock_at_origin, vbit_tool};
    use ivac_core::drawing::api::*;
    use ivac_core::pipeline::*;
    use ivac_core::project::*;
    let segments = vec![
        line((0., 0.), (20., 0.)),
        line((20., 0.), (20., 10.)),
        line((20., 10.), (0., 10.)),
        line((0., 10.), (0., 0.)),
    ];
    let drawing = Drawing::from_sources(vec![source(1, segments.clone())]).unwrap();
    let op = op_single_pass(
        1,
        "GEOM1 software reference",
        OpKind::VCarve {
            carve: VCarveParams::default(),
        },
        1,
        -1.,
    );
    let project = build_project(
        &stock_at_origin(30., 20., 10.),
        vec![vbit_tool(1, 6., 90., 0.)],
        op,
        segments,
    );
    let legacy = PipelineRequest {
        project,
        post_processor: None,
        cps_post: None,
    };
    let expected = run_pipeline(legacy.clone(), |_, _, _| {}).unwrap();
    let mut request = DrawingGenerationRequest {
        version: 1,
        drawing,
        bindings: vec![OperationBinding {
            operation: 1,
            binding: Binding::All,
        }],
        machining: legacy,
    };
    // Caller display/legacy coordinates cannot override the authoritative drawing.
    request.machining.project.segments = vec![line((900., 900.), (901., 900.))];
    assert!(
        serde_json::from_value::<PipelineRequest>(serde_json::to_value(&request).unwrap()).is_err()
    );
    let actual = generate_drawing(request.clone()).unwrap();
    assert_eq!(actual.gcode, expected.gcode);
    request.bindings[0].binding = Binding::Unresolved {
        previous: vec![],
        reason: "deleted".into(),
    };
    assert!(generate_drawing(request).is_err());
}

#[test]
fn legacy_conversion_proves_membership_and_preserves_unrelated_import() {
    use ivac_core::drawing::api::*;
    let mut a = source(1, vec![line((0., 0.), (1., 0.))]);
    a.legacy_objects = vec![1];
    let mut b = source(2, vec![line((10., 0.), (11., 0.))]);
    b.legacy_objects = vec![1];
    let r = evaluate(DrawingRequest::Start {
        version: 1,
        imports: vec![a.clone(), b],
        legacy_offset: Point2::new(3., 4.),
        legacy_text: vec![],
        legacy_auxiliary: vec![],
        tolerance_mm: Some(0.1),
        simplification_mm: None,
    })
    .unwrap();
    assert_eq!(
        r.legacy_contours,
        vec![
            Some(Identity {
                import: 1,
                entity: 1
            }),
            Some(Identity {
                import: 2,
                entity: 1
            })
        ]
    );
    let mut touching = a.clone();
    touching.id = 2;
    let r = evaluate(DrawingRequest::Start {
        version: 1,
        imports: vec![a, touching],
        legacy_offset: Point2::new(0., 0.),
        legacy_text: vec![],
        legacy_auxiliary: vec![],
        tolerance_mm: None,
        simplification_mm: None,
    })
    .unwrap();
    assert!(r.legacy_contours.iter().all(Option::is_none));
}

#[test]
fn snap_and_connector_are_distinct_and_join_requires_binding_review() {
    use ivac_core::drawing::repair::Repair;
    let d = Drawing::from_sources(vec![source(
        1,
        vec![line((0., 0.), (1., 0.)), line((1.08, 0.), (2., 0.))],
    )])
    .unwrap();
    assert_eq!(d.contours.len(), 2);
    let a = d.contours[0].id;
    let b = d.contours[1].id;
    let repair = |connector| Repair::Join {
        a,
        b,
        reverse_a: false,
        reverse_b: false,
        connector,
        tolerance_mm: 0.1,
    };
    let (snap, bindings) = d
        .preview_repairs(&[repair(false)], &[Binding::Contours { ids: vec![a] }])
        .unwrap();
    assert_eq!(snap.contours[0].edges.len(), 2);
    assert_eq!(snap.contours[0].edges[0].geometry.end.x, 1.08);
    assert!(matches!(bindings[0], Binding::Unresolved { .. }));
    let (connector, _) = d.preview_repairs(&[repair(true)], &[]).unwrap();
    assert_eq!(connector.contours[0].edges.len(), 3);
    assert_eq!(connector.contours[0].edges[0].geometry.end.x, 1.);
    assert_eq!(connector.contours[0].edges[1].geometry.start.x, 1.);
}

#[test]
fn scaled_gap_repairs_preserve_independent_import_and_pivot() {
    use ivac_core::drawing::repair::Repair;
    for scale in [0.001, 1., 1000.] {
        for rotation in [0., 37., 90.] {
            for mirror in [false, true] {
                let mut s = source(
                    1,
                    vec![
                        line((0., 0.), (1. / scale, 0.)),
                        line((1. / scale, 0.), (1. / scale, 1. / scale)),
                        line((1. / scale, 1. / scale), (0., 1. / scale)),
                        line((0., 1. / scale), (0., 0.1 / scale)),
                    ],
                );
                s.pivot = Point2::new(0.5 / scale, 0.5 / scale);
                s.transform = Transform {
                    scale,
                    rotation_deg: rotation,
                    mirror_x: mirror,
                    translate: Point2::new(10., 20.),
                    ..Default::default()
                };
                let other = source(2, vec![line((30., 30.), (31., 30.))]);
                let d = Drawing::from_sources(vec![s, other]).unwrap();
                let c = &d.contours[0];
                let r = |mm| Repair::Gap {
                    contour: c.id,
                    edge: c.edges.last().unwrap().id,
                    connector: true,
                    tolerance_mm: mm,
                };
                assert!(d.preview_repairs(&[r(0.099_999)], &[]).is_err());
                let (a, _) = d.preview_repairs(&[r(0.100_001)], &[]).unwrap();
                let e = a.evaluate().unwrap();
                assert!(e.closed[0]);
                assert_eq!(a.imports[0].pivot, d.imports[0].pivot);
                assert_eq!(e.segments.last(), d.evaluate().unwrap().segments.last());
                assert_eq!(a.contours[1].id, d.contours[1].id);
                assert_eq!(a.imports[0].original, d.imports[0].original);
            }
        }
    }
}
#[test]
fn direct_ambiguous_join_refuses_without_relying_on_ui_analysis() {
    use ivac_core::drawing::repair::Repair;
    let d = Drawing::from_sources(vec![source(
        1,
        vec![
            line((0., 0.), (1., 0.)),
            line((1.08, 0.), (2., 0.)),
            line((1.04, 0.04), (1.04, 1.)),
        ],
    )])
    .unwrap();
    assert_eq!(d.contours.len(), 3);
    let r = Repair::Join {
        a: d.contours[0].id,
        b: d.contours[1].id,
        reverse_a: false,
        reverse_b: false,
        connector: true,
        tolerance_mm: 0.1,
    };
    assert!(d
        .preview_repairs(&[r], &[])
        .unwrap_err()
        .contains("Competing"));
}
#[test]
fn exact_duplicate_closed_paths_can_be_removed_but_not_rebound() {
    use ivac_core::drawing::repair::Repair;
    let square = vec![
        line((0., 0.), (2., 0.)),
        line((2., 0.), (2., 2.)),
        line((2., 2.), (0., 2.)),
        line((0., 2.), (0., 0.)),
    ];
    let mut both = square.clone();
    both.extend(square);
    let d = Drawing::from_sources(vec![source(1, both)]).unwrap();
    assert_eq!(d.contours.len(), 1);
    let deleted = d.contours[0].id;
    let (a, b) = d
        .preview_repairs(
            &[Repair::RemoveRepeatedLoop { contour: deleted }],
            &[Binding::Contours { ids: vec![deleted] }],
        )
        .unwrap();
    assert_eq!(a.evaluate().unwrap().segments.len(), 4);
    assert!(matches!(b[0], Binding::Unresolved { .. }));
    assert_eq!(a.imports[0].original.len(), 8);
}
#[test]
fn zero_line_removal_preserves_intentional_point() {
    use ivac_core::drawing::repair::Repair;
    let d = Drawing::from_sources(vec![source(
        1,
        vec![
            line((0., 0.), (0., 0.)),
            Segment::point(Point2::new(10., 10.), "drill", 3),
        ],
    )])
    .unwrap();
    let zero = d
        .contours
        .iter()
        .find(|c| c.edges[0].geometry.kind == ivac_core::geometry::SegmentKind::Line)
        .unwrap();
    let point = d
        .contours
        .iter()
        .find(|c| c.edges[0].geometry.kind == ivac_core::geometry::SegmentKind::Point)
        .unwrap();
    assert!(d
        .preview_repairs(
            &[Repair::RemoveZeroLine {
                contour: point.id,
                edge: point.edges[0].id
            }],
            &[]
        )
        .is_err());
    let (a, _) = d
        .preview_repairs(
            &[Repair::RemoveZeroLine {
                contour: zero.id,
                edge: zero.edges[0].id,
            }],
            &[],
        )
        .unwrap();
    assert_eq!(a.contours[0].id, point.id);
    assert_eq!(a.evaluate().unwrap().segments[0], point.edges[0].geometry);
}
#[test]
fn simplification_has_continuous_bound_and_protects_already_modified_edges() {
    use ivac_core::drawing::repair::Repair;
    let d = Drawing::from_sources(vec![source(
        1,
        vec![
            line((0., 0.), (1., 0.01)),
            line((1., 0.01), (2., 0.)),
            line((2., 0.), (3., 0.)),
        ],
    )])
    .unwrap();
    let c = &d.contours[0];
    let r = |mm| Repair::Simplify {
        contour: c.id,
        edge: c.edges[0].id,
        deviation_mm: mm,
    };
    assert!(d.preview_repairs(&[r(0.0099)], &[]).is_err());
    let (a, _) = d.preview_repairs(&[r(0.010_001)], &[]).unwrap();
    assert_eq!(a.contours[0].edges.len(), 2);
    assert_eq!(a.contours[0].edges[0].geometry, line((0., 0.), (2., 0.)));
    assert!(a.preview_repairs(&[r(0.1)], &[]).is_err());
    assert!(d.preview_repairs(&[r(0.01), r(0.01)], &[]).is_err());
}
#[test]
fn simplification_refuses_contained_island_in_removed_sliver() {
    use ivac_core::drawing::repair::Repair;
    let mut segments = vec![
        line((0., 0.), (1., 0.01)),
        line((1., 0.01), (2., 0.)),
        line((2., 0.), (2., -1.)),
        line((2., -1.), (0., -1.)),
        line((0., -1.), (0., 0.)),
    ];
    segments.extend([
        line((0.9, 0.004), (1.1, 0.004)),
        line((1.1, 0.004), (1.1, 0.006)),
        line((1.1, 0.006), (0.9, 0.006)),
        line((0.9, 0.006), (0.9, 0.004)),
    ]);
    let d = Drawing::from_sources(vec![source(1, segments)]).unwrap();
    let c = &d.contours[0];
    let e = d
        .preview_repairs(
            &[Repair::Simplify {
                contour: c.id,
                edge: c.edges[0].id,
                deviation_mm: 0.010_001,
            }],
            &[],
        )
        .unwrap_err();
    assert!(e.contains("containment"), "{e}");
}

#[cfg(feature = "cps")]
#[test]
fn ui_saved_repair_matches_generated_decoded_program_and_resolved_preview() {
    use ivac_core::drawing::api::DrawingGenerationRequest;
    use ivac_core::gcode::preview::MoveKind;
    use ivac_core::pipeline::{generate_drawing, run_pipeline};
    let request: DrawingGenerationRequest = serde_json::from_str(include_str!(
        "../../../docs/shopbot-modernization/geom1/square-repaired.request.json"
    ))
    .unwrap();
    let response = generate_drawing(request.clone()).unwrap();
    // ADR-0025 adds only `' FABTOOL/1` provenance comments; every other byte
    // still equals the saved GEOM1 software reference.
    let (markers, rest): (Vec<&str>, Vec<&str>) = response
        .gcode
        .split_inclusive('\n')
        .partition(|l| l.starts_with("' FABTOOL/1 "));
    assert_eq!(
        markers,
        ["' FABTOOL/1 REQUEST 0 1\n", "' FABTOOL/1 ACTIVE 0 1\n"]
    );
    assert_eq!(
        rest.concat(),
        include_str!("../../../docs/shopbot-modernization/geom1/square-repaired-software-only.sbp")
    );
    let mut reference = request.machining.clone();
    reference.project.segments = request.drawing.evaluate().unwrap().segments;
    assert_eq!(
        response.gcode,
        run_pipeline(reference, |_, _, _| {}).unwrap().gcode
    );
    let mut pos: [Option<f64>; 3] = [None, None, None];
    let mut decoded = vec![];
    for (line, text) in response.gcode.lines().enumerate() {
        let mut fields = text.split(',');
        let cmd = fields.next().unwrap_or("");
        if !["M3", "J2", "JZ"].contains(&cmd) {
            continue;
        }
        let values: Vec<f64> = fields.map(|s| s.parse().unwrap()).collect();
        let old = pos;
        match cmd {
            "M3" => {
                for i in 0..3 {
                    pos[i] = Some(values[i]);
                }
            }
            "J2" => {
                pos[0] = Some(values[0]);
                pos[1] = Some(values[1]);
            }
            "JZ" => pos[2] = Some(values[0]),
            _ => unreachable!(),
        }
        if let ([Some(x), Some(y), Some(z)], [Some(a), Some(b), Some(c)]) = (old, pos) {
            if (x - a).abs() + (y - b).abs() + (z - c).abs() < 1e-10 {
                continue;
            }
            if cmd == "J2" {
                assert!(z > 0. && c > 0.);
            }
            decoded.push(([x, y, z], [a, b, c], cmd == "M3", line + 1));
        }
    }
    assert_eq!(decoded.len(), response.toolpath.len());
    for ((a, b, cut, line), s) in decoded.iter().zip(&response.toolpath) {
        assert_eq!(*line, s.gcode_line as usize);
        for (actual, expected) in a
            .iter()
            .zip([s.from.x, s.from.y, s.from.z])
            .chain(b.iter().zip([s.to.x, s.to.y, s.to.z]))
        {
            assert!((actual - expected).abs() < 1e-9);
        }
        assert_eq!(*cut, !matches!(s.kind, MoveKind::Rapid));
        assert_eq!(s.op_id, 1);
    }
    // Also exercise explicit stable selection rather than only dynamic Layers.
    let mut explicit = request;
    explicit.bindings[0].binding = Binding::Contours {
        ids: vec![explicit.drawing.contours[0].id],
    };
    assert_eq!(generate_drawing(explicit).unwrap().gcode, response.gcode);
}

#[cfg(feature = "cps")]
#[test]
fn edited_sources_fail_explicitly_for_empty_or_synthetic_or_fuzzy_ambiguous_scope() {
    use ivac_core::drawing::api::DrawingGenerationRequest;
    use ivac_core::pipeline::generate_drawing;
    let base: DrawingGenerationRequest = serde_json::from_str(include_str!(
        "../../../docs/shopbot-modernization/geom1/square-repaired.request.json"
    ))
    .unwrap();
    let mut missing = base.clone();
    missing.bindings[0].binding = Binding::Layers {
        names: vec!["deleted".into()],
    };
    assert!(generate_drawing(missing)
        .unwrap_err()
        .to_string()
        .contains("no remaining geometry"));
    let mut stock = base.clone();
    stock.bindings[0].binding = Binding::Layers {
        names: vec!["__stock_outline".into()],
    };
    assert!(generate_drawing(stock)
        .unwrap_err()
        .to_string()
        .contains("Synthetic stock"));
    let mut fuzzy = base;
    // Remove the 0.08 connector and leave a 0.001 gap; legacy fuzzy closure
    // would call this closed. Exact authoritative membership must not silently
    // turn the old intent into an open toolpath.
    let c = &mut fuzzy.drawing.contours[0];
    c.edges.pop();
    c.edges.last_mut().unwrap().geometry.end.y = 10.001;
    assert!(generate_drawing(fuzzy)
        .unwrap_err()
        .to_string()
        .contains("fuzzy chaining"));
}

#[test]
fn deleting_an_earlier_import_cannot_redirect_an_unrelated_operation() {
    use common::{build_project, op_single_pass, stock_at_origin, vbit_tool};
    use ivac_core::drawing::{api::*, repair::Repair};
    use ivac_core::pipeline::*;
    use ivac_core::project::*;
    let rectangle = vec![
        line((0., 0.), (20., 0.)),
        line((20., 0.), (20., 10.)),
        line((20., 10.), (0., 10.)),
        line((0., 10.), (0., 0.)),
    ];
    let mut second = source(2, rectangle);
    second.transform = Transform {
        rotation_deg: 37.,
        translate: Point2::new(30., 40.),
        mirror_y: true,
        ..Default::default()
    };
    let d = Drawing::from_sources(vec![
        source(1, vec![line((-50., -50.), (-50., -50.))]),
        second,
    ])
    .unwrap();
    let target = d.contours[1].id;
    let original = d.evaluate().unwrap();
    let zero = &d.contours[0];
    let (accepted, b) = d
        .preview_repairs(
            &[Repair::RemoveZeroLine {
                contour: zero.id,
                edge: zero.edges[0].id,
            }],
            &[Binding::Contours { ids: vec![target] }],
        )
        .unwrap();
    assert_eq!(b[0], Binding::Contours { ids: vec![target] });
    let unrelated: Vec<_> = original
        .segments
        .iter()
        .zip(&original.membership)
        .filter_map(|(s, id)| (*id == target).then_some(s.clone()))
        .collect();
    assert_eq!(accepted.evaluate().unwrap().segments, unrelated);
    let op = op_single_pass(
        1,
        "Unrelated import reference",
        OpKind::VCarve {
            carve: VCarveParams::default(),
        },
        1,
        -1.,
    );
    let mut project = build_project(
        &stock_at_origin(100., 100., 10.),
        vec![vbit_tool(1, 6., 90., 0.)],
        op,
        unrelated,
    );
    project.work_offset.x_mm = 5.;
    project.work_offset.y_mm = 7.;
    let machining = PipelineRequest {
        project,
        post_processor: None,
        cps_post: None,
    };
    let expected = run_pipeline(machining.clone(), |_, _, _| {}).unwrap();
    let result = generate_drawing(DrawingGenerationRequest {
        version: 1,
        drawing: accepted,
        bindings: vec![OperationBinding {
            operation: 1,
            binding: b[0].clone(),
        }],
        machining,
    })
    .unwrap();
    assert_eq!(result.gcode, expected.gcode);
}
