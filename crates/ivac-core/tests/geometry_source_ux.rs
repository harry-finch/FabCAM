//! Audit fixtures for existing interpretation. No geometry algorithm changes.
use ivac_core::cam::chaining::{classify_containment, segments_to_objects};
use ivac_core::cam::source_combine::{combine_source_regions, CombinedRegion};
use ivac_core::cam::VcObject;
use ivac_core::geometry::{Point2, Segment};
use ivac_core::input::{dxf_in::import_dxf_bytes, svg_in::import_svg_bytes, ImportOptions};
use ivac_core::project::SourceCombine;

#[test]
fn svg_named_groups_remain_one_synthetic_layer() {
    let bytes =
        br#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
      <g id="Outer"><rect x="10" y="10" width="40" height="40"/></g>
      <g id="Inner"><rect x="23" y="23" width="14" height="14"/></g>
    </svg>"#;
    let out = import_svg_bytes("groups.svg".into(), bytes, &ImportOptions::default()).unwrap();
    assert_eq!(out.layers.len(), 1);
    assert_eq!(out.layers[0].name, "svg");
    assert!(out.segments.iter().all(|s| s.layer.as_ref() == "svg"));
    assert_eq!(out.object_meta.len(), 2);
}

#[test]
fn dxf_named_layers_and_import_layer_filter_are_preserved() {
    // Self-contained ASCII DXF: does not depend on the optional refs/ corpus.
    let bytes = b"0\nSECTION\n2\nENTITIES\n0\nLINE\n8\nCuts\n10\n0\n20\n0\n11\n10\n21\n0\n0\nLINE\n8\nMarking\n10\n0\n20\n5\n11\n10\n21\n5\n0\nENDSEC\n0\nEOF\n";
    let out = import_dxf_bytes("layers.dxf".into(), bytes, &ImportOptions::default()).unwrap();
    assert_eq!(out.segments.len(), 2);
    assert!(out.layers.iter().any(|l| l.name == "Cuts"));
    assert!(out.layers.iter().any(|l| l.name == "Marking"));
    let selected = import_dxf_bytes(
        "layers.dxf".into(),
        bytes,
        &ImportOptions {
            select_layers: vec!["Cuts".into()],
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(selected.segments.len(), 1);
    assert_eq!(selected.segments[0].layer.as_ref(), "Cuts");
}

fn square(side: f64, x: f64, y: f64) -> Vec<Segment> {
    let p = [(x, y), (x + side, y), (x + side, y + side), (x, y + side)];
    (0..4)
        .map(|i| {
            let a = p[i];
            let b = p[(i + 1) % 4];
            Segment::line(Point2::new(a.0, a.1), Point2::new(b.0, b.1), "audit", 7)
        })
        .collect()
}
fn objects(shapes: Vec<Vec<Segment>>) -> Vec<VcObject> {
    let mut result = segments_to_objects(&shapes.concat());
    classify_containment(&mut result);
    result
}
fn area(r: &CombinedRegion) -> f64 {
    let poly = |p: &[Point2]| {
        (0..p.len())
            .map(|i| {
                let a = p[i];
                let b = p[(i + 1) % p.len()];
                a.x * b.y - b.x * a.y
            })
            .sum::<f64>()
            .abs()
            * 0.5
    };
    poly(&r.boundary) - r.holes.iter().map(|p| poly(p)).sum::<f64>()
}

#[test]
fn difference_preserves_explicit_operand_order() {
    let o = objects(vec![square(10., 0., 0.), square(4., 3., 3.)]);
    let forward = combine_source_regions(&o, &[0, 1], SourceCombine::Difference);
    assert_eq!(forward.len(), 1);
    assert!((area(&forward[0]) - 84.).abs() < 1e-6);
    assert!(combine_source_regions(&o, &[1, 0], SourceCombine::Difference).is_empty());
}

#[test]
fn shared_auto_kernel_uses_selected_nesting_depth_not_svg_fill_rules() {
    let o = objects(vec![
        square(20., 0., 0.),
        square(12., 4., 4.),
        square(4., 8., 8.),
    ]);
    let regions = combine_source_regions(&o, &[0, 1, 2], SourceCombine::Auto);
    assert_eq!(regions.len(), 2);
    assert_eq!(regions.iter().map(|r| r.holes.len()).sum::<usize>(), 1);
    let total: f64 = regions.iter().map(area).sum();
    assert!((total - (400. - 144. + 16.)).abs() < 1e-6);
    let outer_only = combine_source_regions(&o, &[0], SourceCombine::Auto);
    assert!(outer_only[0].holes.is_empty());
}

#[test]
fn all_region_modes_ignore_open_contours_and_missing_indices() {
    let mut o = objects(vec![square(10., 0., 0.)]);
    o.push(VcObject::new(
        vec![Segment::line(
            Point2::new(30., 0.),
            Point2::new(40., 0.),
            "audit",
            7,
        )],
        false,
    ));
    for mode in [
        SourceCombine::Auto,
        SourceCombine::Union,
        SourceCombine::Difference,
        SourceCombine::Intersection,
        SourceCombine::Xor,
        SourceCombine::None,
    ] {
        let regions = combine_source_regions(&o, &[1, 99, 0], mode);
        assert_eq!(regions.len(), 1);
        assert!((area(&regions[0]) - 100.).abs() < 1e-6);
    }
}

#[test]
fn separate_contours_does_not_preserve_nested_holes() {
    let o = objects(vec![square(10., 0., 0.), square(4., 3., 3.)]);
    let regions = combine_source_regions(&o, &[0, 1], SourceCombine::None);
    assert_eq!(regions.len(), 2);
    assert!(regions.iter().all(|r| r.holes.is_empty()));
    assert!((regions.iter().map(area).sum::<f64>() - 116.).abs() < 1e-6);
}

#[test]
fn xor_preserves_parity_after_empty_intermediate() {
    let a = VcObject::new(square(10., 0., 0.), true);
    for n in 2..=4 {
        let o = vec![a.clone(); n];
        let selected: Vec<_> = (0..n).collect();
        let regions = combine_source_regions(&o, &selected, SourceCombine::Xor);
        assert_eq!(regions.len(), n % 2);
        if n % 2 == 1 {
            assert!((area(&regions[0]) - 100.).abs() < 1e-6);
        }
    }
    let b = VcObject::new(square(4., 20., 20.), true);
    let regions = combine_source_regions(&[a.clone(), a, b], &[0, 1, 2], SourceCombine::Xor);
    assert_eq!(regions.len(), 1);
    assert!((area(&regions[0]) - 16.).abs() < 1e-6);
}
