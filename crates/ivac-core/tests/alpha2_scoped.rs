#![allow(clippy::float_cmp, clippy::default_trait_access)]
mod alpha2_support;
#[path = "support/explicit.rs"]
mod fixture_support;
use ivac_core::{
    cancel::CancelToken,
    drawing::{
        api::OperationBinding,
        repair::Repair,
        scoped::{self, ScopedRequest},
        Binding, Drawing, Source, Transform,
    },
    Point2, Segment,
};
fn line(a: (f64, f64), b: (f64, f64)) -> Segment {
    Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "cut", 7)
}
fn fixture() -> scoped::ScopedPreview {
    let d = Drawing::from_sources(vec![
        Source {
            id: 1,
            original: vec![
                line((0., 0.), (10., 0.)),
                line((10., 0.), (10., 10.)),
                line((10., 10.), (0., 10.)),
                line((0., 10.), (0., 0.00005)),
            ],
            legacy_objects: vec![],
            pivot: Point2::new(0., 0.),
            transform: Transform::default(),
            next_entity: 1,
        },
        Source {
            id: 2,
            original: vec![
                line((100., 100.), (110., 100.)),
                line((110., 100.), (110., 110.)),
                line((110., 110.), (100., 110.)),
                line((100., 110.), (100., 100.)),
            ],
            legacy_objects: vec![],
            pivot: Point2::new(0., 0.),
            transform: Transform::default(),
            next_entity: 1,
        },
    ])
    .unwrap();
    let c = &d.contours[0];
    let r = Repair::Gap {
        contour: c.id,
        edge: c.edges.last().unwrap().id,
        connector: true,
        tolerance_mm: 0.001,
    };
    alpha2_support::input(d, r)
}
#[test]
fn v1_stays_strict_v2_is_explicit_and_apply_recomputes() {
    let input = fixture();
    let result = alpha2_support::preview(input.clone()).unwrap();
    assert_eq!(result.geometry.version, 2);
    assert_eq!(result.geometry.drawing.version, 1);
    assert!(result.local_edit_accepted);
    let apply = scoped::evaluate(
        ScopedRequest::Apply {
            version: 2,
            preview: Box::new(input.clone()),
            current: Box::new(input.context.clone()),
        },
        &Default::default(),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(result).unwrap(),
        serde_json::to_value(apply).unwrap()
    );
    let mut stale = input.context.clone();
    stale.project.work_offset.x_mm += 1.;
    assert!(scoped::evaluate(
        ScopedRequest::Apply {
            version: 2,
            preview: Box::new(input),
            current: Box::new(stale)
        },
        &Default::default()
    )
    .unwrap_err()
    .contains("stale"));
}
#[test]
fn stale_document_and_omitted_context_refuse() {
    let input = fixture();
    let mut current = input.context.clone();
    current.document = serde_json::json!({"revision":2});
    assert!(scoped::evaluate(
        ScopedRequest::Apply {
            version: 2,
            preview: Box::new(input.clone()),
            current: Box::new(current)
        },
        &Default::default()
    )
    .is_err());
    let mut json = serde_json::to_value(ScopedRequest::Preview {
        version: 2,
        input: Box::new(input),
    })
    .unwrap();
    json["input"]["context"]
        .as_object_mut()
        .unwrap()
        .remove("project");
    assert!(serde_json::from_value::<ScopedRequest>(json).is_err());
}
#[test]
fn containment_envelopes_fail_closed() {
    let mut input = fixture();
    input.context.drawing.imports[1].transform.scale = 0.1;
    input.context.drawing.imports[1].transform.translate = Point2::new(-8., -8.);
    input.context.project.segments = input.context.drawing.evaluate().unwrap().segments;
    assert!(alpha2_support::preview(input)
        .unwrap_err()
        .contains("proof"));
}
#[test]
fn cancellation_and_budget_have_no_invented_contour() {
    let d = fixture().context.drawing;
    let c = CancelToken::new();
    c.cancel();
    let r = ivac_core::drawing::diagnostics::analyze(&d, &c);
    assert!(!r.complete);
    assert!(r.issues[0].contours.is_empty());
    let mut d = d;
    d.contours[0].edges = vec![d.contours[0].edges[0].clone(); 2001];
    let r = ivac_core::drawing::diagnostics::analyze(&d, &Default::default());
    assert!(!r.complete);
    assert!(r.issues[0].edges.is_empty());
}
#[test]
fn unchanged_placements_preserve_and_changed_or_ambiguous_refuse() {
    use alpha2_support::common;
    use ivac_core::project::{OpKind, OpSource, ProfileParams, TabPlacement, ToolOffset};
    let mut input = fixture();
    let owner = input.context.drawing.contours[1].id;
    let mut params = ivac_core::project::ContourParams::default();
    params.tab_placements = vec![TabPlacement {
        object_id: 2,
        t: 0.25,
        width_override_mm: None,
        height_override_mm: None,
    }];
    let mut op = common::op_single_pass(
        1,
        "held",
        OpKind::Profile {
            offset: ToolOffset::Outside,
            contour: params,
            profile: ProfileParams::default(),
        },
        1,
        -1.,
    );
    op.source = OpSource::Objects {
        ids: vec![2],
        combine: Default::default(),
    };
    input.context.project.operations = vec![op];
    input.context.bindings = vec![OperationBinding {
        operation: 1,
        binding: Binding::Contours { ids: vec![owner] },
    }];
    let saved = serde_json::to_value(&input.context.project.operations).unwrap();
    assert!(alpha2_support::preview(input.clone()).is_ok());
    assert_eq!(
        saved,
        serde_json::to_value(&input.context.project.operations).unwrap()
    );
    let mut wrong_geometry = input.clone();
    wrong_geometry.context.project.segments[4].start.x += 1.;
    assert!(alpha2_support::preview(wrong_geometry).is_err());
    let mut merged_auxiliary = input.clone();
    merged_auxiliary
        .context
        .project
        .segments
        .push(line((110., 100.), (115., 100.)));
    assert!(alpha2_support::preview(merged_auxiliary).is_err());
    let mut shifted = input.clone();
    shifted.context.project.work_offset.x_mm = 1.;
    assert!(alpha2_support::preview(shifted).is_err());
    input.context.bindings[0].binding = Binding::All;
    assert!(alpha2_support::preview(input.clone()).is_err());
    input.context.bindings[0].binding = Binding::Contours {
        ids: vec![input.context.drawing.contours[0].id],
    };
    assert!(alpha2_support::preview(input).is_err());
}
#[test]
fn new_crossing_is_not_relabelled_as_old() {
    let mut auxiliary_crossing = fixture();
    auxiliary_crossing
        .context
        .project
        .segments
        .push(line((-1., 0.000_025), (1., 0.000_025)));
    assert!(alpha2_support::preview(auxiliary_crossing)
        .unwrap_err()
        .contains("Auxiliary"));
    let mut distant_auxiliary = fixture();
    distant_auxiliary
        .context
        .project
        .segments
        .push(line((-100., -100.), (-90., -90.)));
    assert!(alpha2_support::preview(distant_auxiliary.clone()).is_ok());
    distant_auxiliary
        .context
        .project
        .segments
        .last_mut()
        .unwrap()
        .start
        .x = f64::NAN;
    assert!(alpha2_support::preview(distant_auxiliary)
        .unwrap_err()
        .contains("Nonfinite"));
    let mut input = fixture();
    // Move a corner so connecting the open junction would cross an existing edge.
    let c = &mut input.context.drawing.contours[0];
    c.edges.last_mut().unwrap().geometry.end = Point2::new(5., -1.);
    input.context.project.segments = input.context.drawing.evaluate().unwrap().segments;
    if let Repair::Gap { tolerance_mm, .. } = &mut input.repairs[0] {
        *tolerance_mm = 20.;
    }
    assert!(alpha2_support::preview(input).is_err());
}

#[test]
fn original_import_order_and_identity_pivot_are_proven_without_welding() {
    let mut input = fixture();
    input.context.drawing.imports[0].pivot = Point2::new(5., 5.);
    input.context.drawing.imports[0].original.swap(0, 2);
    let d = Drawing::from_sources(input.context.drawing.imports.clone()).unwrap();
    let c = &d.contours[0];
    input.repairs = vec![Repair::Gap {
        contour: c.id,
        edge: c
            .edges
            .iter()
            .find(|e| e.geometry.end.y == 0.00005)
            .unwrap()
            .id,
        connector: true,
        tolerance_mm: 0.001,
    }];
    input.context.project.segments = d.imports.iter().flat_map(|s| s.original.clone()).collect();
    input.context.drawing = d;
    assert!(alpha2_support::preview(input.clone()).is_ok());
    // An authored change cannot be hidden behind unchanged original imports.
    input.context.drawing.contours[1].edges[0].geometry.start.x += 1.;
    assert!(alpha2_support::preview(input).is_err());
}
