#![allow(clippy::float_cmp, clippy::default_trait_access)]
#[path = "support/explicit.rs"]
mod fixture_support;
// Source-bound counterexamples: v1 behavior retained; v2 fixes qualified separately.
mod alpha2_support;
use ivac_core::drawing::repair::Repair;
use ivac_core::drawing::{Drawing, Source, Transform};
use ivac_core::{Point2, Segment};
fn line(a: (f64, f64), b: (f64, f64)) -> Segment {
    Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "synthetic", 7)
}
fn drawing() -> Drawing {
    let groups = vec![
        vec![
            line((0., 0.), (10., 0.)),
            line((10., 0.), (10., 10.)),
            line((10., 10.), (0., 10.)),
            line((0., 10.), (0., 0.00005)),
        ],
        vec![
            line((100., 100.), (110., 110.)),
            line((100., 110.), (110., 100.)),
        ],
    ];
    Drawing::from_sources(
        groups
            .into_iter()
            .zip(1..)
            .map(|(original, id)| Source {
                id,
                original,
                legacy_objects: vec![],
                pivot: Point2::new(0., 0.),
                transform: Transform::default(),
                next_entity: 1,
            })
            .collect(),
    )
    .unwrap()
}
#[test]
fn topology_finding_must_not_blame_unrelated_first_contour() {
    let d = drawing();
    // V1 deliberately retains its old global behavior; the same regression
    // now exercises the explicitly selected V2 diagnostics.
    let report = ivac_core::drawing::diagnostics::analyze(&d, &Default::default());
    assert!(report.complete);
    assert_eq!(report.issues.len(), 1);
    assert!(!report.issues[0].contours.contains(&d.contours[0].id));
    assert_eq!(report.issues[0].edges.len(), 2);
}
#[test]
fn safe_gap_repair_should_not_be_blocked_by_distant_unchanged_crossing() {
    let d = drawing();
    let c = &d.contours[0];
    let r = Repair::Gap {
        contour: c.id,
        edge: c.edges.last().unwrap().id,
        connector: true,
        tolerance_mm: 0.001,
    };
    let after = alpha2_support::preview(alpha2_support::input(d, r))
        .expect("Independent explicit repair should be editable, without granting CAM eligibility");
    assert!(
        after.geometry.drawing.validate_for_generation().is_err(),
        "Unrelated crossing must still block CAM"
    );
}

#[test]
fn exact_closure_and_fuzzy_membership_are_distinct_at_all_reviewed_scales() {
    for scale in [0.001, 1., 1000.] {
        let mut d = drawing();
        d.contours.truncate(1);
        d.imports[0].transform.scale = scale;
        d.imports[0].transform.rotation_deg = 37.;
        d.imports[0].transform.translate = Point2::new(50., -90.);
        let c = &d.contours[0];
        assert!(!d.evaluate().unwrap().closed[0]);
        for connector in [true, false] {
            let r = Repair::Gap {
                contour: c.id,
                edge: c.edges.last().unwrap().id,
                connector,
                tolerance_mm: 0.0001 * scale,
            };
            let scoped = alpha2_support::preview(alpha2_support::input(d.clone(), r.clone()))
                .expect("V2 must prove the transformed isolated gap as well");
            assert!(scoped.geometry.evaluated.closed[0]);
            let (after, _) = d.preview_repairs(&[r], &[]).unwrap();
            assert!(after.evaluate().unwrap().closed[0]);
            assert_eq!(after.contours[0].id, c.id);
            assert_eq!(d.contours[0].edges.last().unwrap().geometry.end.y, 0.00005);
        }
    }
}
#[test]
fn crossing_overlap_and_touch_remain_generation_refusals() {
    for (a, b) in [
        (line((0., 0.), (10., 10.)), line((0., 10.), (10., 0.))),
        (line((0., 0.), (10., 0.)), line((2., 0.), (8., 0.))),
        (line((0., 0.), (10., 0.)), line((5., 0.), (5., 5.))),
    ] {
        let d = Drawing::from_sources(
            vec![a, b]
                .into_iter()
                .zip(1..)
                .map(|(s, id)| Source {
                    id,
                    original: vec![s],
                    legacy_objects: vec![],
                    pivot: Point2::new(0., 0.),
                    transform: Transform::default(),
                    next_entity: 1,
                })
                .collect(),
        )
        .unwrap();
        assert!(d.validate_for_generation().is_err());
        let report = ivac_core::drawing::diagnostics::analyze(&d, &Default::default());
        assert!(report.complete);
        assert!(!report.issues.is_empty());
    }
}
