// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::cast_possible_truncation, clippy::float_cmp)]
use ivac_core::drawing::repair::Repair;
use ivac_core::drawing::{Binding, Drawing, Source, Transform};
use ivac_core::{Point2, Segment};
fn p(x: f64, y: f64) -> Point2 {
    Point2::new(x, y)
}
fn line(a: (f64, f64), b: (f64, f64)) -> Segment {
    Segment::line(p(a.0, a.1), p(b.0, b.1), "cut", 7)
}
fn drawing(groups: Vec<Vec<Segment>>) -> Drawing {
    Drawing::from_sources(
        groups
            .into_iter()
            .enumerate()
            .map(|(i, original)| Source {
                id: i as u32 + 1,
                original,
                legacy_objects: vec![],
                pivot: p(0., 0.),
                transform: Transform::default(),
                next_entity: 1,
            })
            .collect(),
    )
    .unwrap()
}
fn circle(x: f64) -> Vec<Segment> {
    vec![
        Segment::arc(p(x + 2., 0.), p(x - 2., 0.), 1., Some(p(x, 0.)), "arc", 7),
        Segment::arc(p(x - 2., 0.), p(x + 2., 0.), 1., Some(p(x, 0.)), "arc", 7),
    ]
}
fn simplify_shape() -> Vec<Segment> {
    vec![
        line((0., 0.), (5., -0.1)),
        line((5., -0.1), (10., 0.)),
        line((10., 0.), (10., 10.)),
        line((10., 10.), (0., 10.)),
        line((0., 10.), (0., 0.)),
    ]
}
#[test]
fn circles_and_adjacent_line_arcs_are_valid() {
    drawing(vec![circle(0.)]).validate_for_generation().unwrap();
    drawing(vec![vec![
        line((0., 0.), (10., 0.)),
        Segment::arc(p(10., 0.), p(10., 10.), 1., Some(p(10., 5.)), "cut", 7),
        line((10., 10.), (0., 10.)),
        line((0., 10.), (0., 0.)),
    ]])
    .validate_for_generation()
    .unwrap();
}
#[test]
fn simplify_with_distant_curve_and_drill_point() {
    let d = drawing(vec![
        simplify_shape(),
        circle(30.),
        vec![Segment::point(p(40., 0.), "drill", 7)],
    ]);
    let c = &d.contours[0];
    d.preview_repairs(
        &[Repair::Simplify {
            contour: c.id,
            edge: c.edges[0].id,
            deviation_mm: 0.2,
        }],
        &[],
    )
    .unwrap();
}
#[test]
fn simplify_cannot_remove_drill_point_from_sliver() {
    let d = drawing(vec![
        simplify_shape(),
        vec![Segment::point(p(5., -0.05), "drill", 7)],
    ]);
    let c = &d.contours[0];
    assert!(d
        .preview_repairs(
            &[Repair::Simplify {
                contour: c.id,
                edge: c.edges[0].id,
                deviation_mm: 0.2
            }],
            &[]
        )
        .is_err());
}
#[test]
fn tiny_gap_is_not_displayed_as_zero_or_silently_closed() {
    let d = drawing(vec![vec![
        line((0., 0.), (10., 0.)),
        line((10., 0.), (10., 10.)),
        line((10., 10.), (0., 10.)),
        line((0., 10.), (0., 1e-14)),
    ]]);
    assert!(!d.evaluate().unwrap().closed[0]);
    let findings = d.analyze(0.01).unwrap();
    assert!(findings
        .iter()
        .any(|f| f.message.contains("Endpoint gap") && !f.message.contains("0.000000000 mm")));
}
#[test]
fn layer_resolution_preserves_contour_order() {
    let d = drawing(vec![circle(0.), vec![line((10., 0.), (20., 0.))]]);
    let e = d.evaluate().unwrap();
    assert_eq!(
        e.resolve(&Binding::Layers {
            names: vec!["cut".into(), "arc".into()]
        })
        .unwrap(),
        vec![0, 1]
    );
}

#[test]
fn overlapping_crossing_and_tangent_curves_still_refuse() {
    assert!(drawing(vec![circle(0.), vec![line((-3., 0.), (3., 0.))]])
        .validate_for_generation()
        .is_err());
    assert!(drawing(vec![circle(0.), circle(4.)])
        .validate_for_generation()
        .is_err());
    let a = Segment::arc(p(2., 0.), p(-2., 0.), 1., Some(p(0., 0.)), "cut", 7);
    let b = Segment::arc(p(-2., 0.), p(2., 0.), -1., Some(p(0., 0.)), "cut", 7);
    assert!(drawing(vec![vec![a, b]]).validate_for_generation().is_err());
}
#[test]
fn simplify_batches_with_gap_closure_and_contour_deletion() {
    let d = drawing(vec![
        simplify_shape(),
        vec![
            line((20., 0.), (30., 0.)),
            line((30., 0.), (30., 10.)),
            line((30., 10.), (20., 10.)),
            line((20., 10.), (20., 0.01)),
        ],
        vec![line((40., 0.), (40., 0.))],
    ]);
    let a = &d.contours[0];
    let b = &d.contours[1];
    let c = &d.contours[2];
    let repairs = vec![
        Repair::Gap {
            contour: b.id,
            edge: b.edges.last().unwrap().id,
            connector: false,
            tolerance_mm: 0.02,
        },
        Repair::RemoveZeroLine {
            contour: c.id,
            edge: c.edges[0].id,
        },
        Repair::Simplify {
            contour: a.id,
            edge: a.edges[0].id,
            deviation_mm: 0.2,
        },
    ];
    let (next, _) = d.preview_repairs(&repairs, &[]).unwrap();
    assert_eq!(next.contours.len(), 2);
    assert!(next.evaluate().unwrap().closed.iter().all(|v| *v));
}
#[test]
fn boundary_point_is_ambiguous_but_supporting_line_is_not() {
    for (x, ok) in [(5., false), (20., true)] {
        let d = drawing(vec![
            simplify_shape(),
            vec![Segment::point(p(x, 0.), "drill", 7)],
        ]);
        let c = &d.contours[0];
        assert_eq!(
            d.preview_repairs(
                &[Repair::Simplify {
                    contour: c.id,
                    edge: c.edges[0].id,
                    deviation_mm: 0.2
                }],
                &[]
            )
            .is_ok(),
            ok
        );
    }
}
#[test]
fn transform_order_matches_cam_for_all_mirror_combinations() {
    use ivac_core::cam::{
        mirror_segments_x, mirror_segments_y, rotate_segments, scale_segments, translate_segments,
    };
    for mirror_x in [false, true] {
        for mirror_y in [false, true] {
            for scale in [2., -2.] {
                let pivot = p(3., 7.);
                let t = Transform {
                    translate: p(11., -5.),
                    rotation_deg: 30.,
                    scale,
                    mirror_x,
                    mirror_y,
                };
                let mut segments = circle(2.);
                let original = segments.clone();
                scale_segments(&mut segments, pivot, scale);
                if mirror_x {
                    mirror_segments_x(&mut segments, pivot);
                }
                if mirror_y {
                    mirror_segments_y(&mut segments, pivot);
                }
                rotate_segments(&mut segments, pivot, 30_f64.to_radians());
                translate_segments(&mut segments, 11., -5.);
                for (a, b) in original.iter().zip(segments) {
                    assert!(t.point(a.start, pivot).distance(b.start) < 1e-12);
                    assert!(t.point(a.end, pivot).distance(b.end) < 1e-12);
                    assert!(
                        t.point(a.center.unwrap(), pivot)
                            .distance(b.center.unwrap())
                            < 1e-12
                    );
                    assert_eq!(
                        b.bulge,
                        if mirror_x == mirror_y {
                            a.bulge
                        } else {
                            -a.bulge
                        }
                    );
                }
            }
        }
    }
}
#[test]
fn tiny_unclosed_contour_still_protects_drill_point_from_simplification() {
    let mut shape = simplify_shape();
    shape.last_mut().unwrap().end = p(0., 1e-14);
    let d = drawing(vec![shape, vec![Segment::point(p(5., -0.05), "drill", 7)]]);
    assert!(!d.evaluate().unwrap().closed[0]);
    let c = &d.contours[0];
    assert!(d
        .preview_repairs(
            &[Repair::Simplify {
                contour: c.id,
                edge: c.edges[0].id,
                deviation_mm: 0.2
            }],
            &[]
        )
        .is_err());
}

#[test]
fn snap_cannot_collapse_micro_edge() {
    let mut d = drawing(vec![simplify_shape()]);
    let b = d.contours[0].edges[1].geometry.start;
    d.contours[0].edges[0].geometry.start = b;
    d.contours[0].edges[0].geometry.end = p(b.x - 0.01, b.y);
    let c = &d.contours[0];
    let error = d
        .preview_repairs(
            &[Repair::Gap {
                contour: c.id,
                edge: c.edges[0].id,
                connector: false,
                tolerance_mm: 0.02,
            }],
            &[],
        )
        .unwrap_err();
    assert!(error.contains("collapse"), "{error}");
}
#[test]
fn internal_gap_is_not_reported_as_closed() {
    let mut d = drawing(vec![simplify_shape()]);
    d.contours[0].edges[1].geometry.start.x += 1e-12;
    assert!(!d.evaluate().unwrap().closed[0]);
}
#[test]
fn intact_circle_generates_through_authoritative_drawing_api() {
    use ivac_core::drawing::api::{DrawingGenerationRequest, OperationBinding};
    let mut req: DrawingGenerationRequest = serde_json::from_str(include_str!(
        "../../../docs/shopbot-modernization/geom1/square-repaired.request.json"
    ))
    .unwrap();
    let mut d = drawing(vec![circle(0.)]);
    d.imports[0].transform.scale = 10.;
    d.imports[0].transform.translate = p(25., 25.);
    req.drawing = d;
    req.bindings = vec![OperationBinding {
        operation: req.machining.project.operations[0].id,
        binding: Binding::All,
    }];
    let response =
        ivac_core::pipeline::generate_drawing(crate::fixture_support::drawing(req)).unwrap();
    assert!(!response.toolpath.is_empty());
    assert!(!response.gcode.is_empty());
}

#[path = "support/explicit.rs"]
mod fixture_support;
