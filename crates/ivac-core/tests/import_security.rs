use ivac_core::{
    input::{import_bytes, ImportOptions},
    ErrorKind,
};
use std::io::Write;
#[test]
fn svg_admission_preserves_flat_use_but_refuses_nested_expansion() {
    let direct=br##"<svg xmlns="http://www.w3.org/2000/svg" width="10mm" height="10mm"><defs><path id="p" d="M0 0L1 1"/></defs><use href="#p"/></svg>"##;
    let out = import_bytes("synthetic.svg", direct, &ImportOptions::default()).unwrap();
    assert_eq!(out.segments.len(), 1);
    let nested=br##"<svg xmlns="http://www.w3.org/2000/svg"><defs><path id="p" d="M0 0L1 1"/><g id="g"><use href="#p"/></g></defs><use href="#g"/></svg>"##;
    let err = import_bytes("synthetic.svg", nested, &ImportOptions::default()).unwrap_err();
    assert!(err.message.contains("Nested"));
}
#[test]
fn svgz_is_bounded_before_xml_and_regular_compressed_svg_remains_identical() {
    let svg=b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10mm\" height=\"10mm\"><path d=\"M0 0L1 1\"/></svg>";
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    gz.write_all(svg).unwrap();
    let compressed = gz.finish().unwrap();
    let plain = import_bytes("same.svg", svg, &ImportOptions::default()).unwrap();
    let zipped = import_bytes("same.svg", &compressed, &ImportOptions::default()).unwrap();
    assert_eq!(
        serde_json::to_value(plain).unwrap(),
        serde_json::to_value(zipped).unwrap()
    );
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    gz.write_all(&vec![b' '; 16 * 1024 * 1024 + 1]).unwrap();
    let err =
        import_bytes("cap.svg", &gz.finish().unwrap(), &ImportOptions::default()).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Limit);
}
#[test]
fn svg_deep_nesting_refuses_without_partial_geometry() {
    let svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\">{}<path d=\"M0 0L1 1\"/>{}</svg>",
        "<g>".repeat(65),
        "</g>".repeat(65)
    );
    assert!(import_bytes("deep.svg", svg.as_bytes(), &ImportOptions::default()).is_err());
}
#[test]
fn dxf_nested_chain_is_an_error_not_a_partial_drawing() {
    use dxf::{
        entities::{Entity, EntityType, Insert, Line},
        Block, Drawing, Point,
    };
    let mut d = Drawing::new();
    d.add_entity(Entity::new(EntityType::Line(Line {
        p1: Point::new(0., 0., 0.),
        p2: Point::new(1., 1., 0.),
        ..Default::default()
    })));
    for i in 0..35 {
        let mut b = Block::default();
        b.name = format!("b{i}");
        if i < 34 {
            b.entities.push(Entity::new(EntityType::Insert(Insert {
                name: format!("b{}", i + 1),
                ..Default::default()
            })));
        }
        d.add_block(b);
    }
    d.add_entity(Entity::new(EntityType::Insert(Insert {
        name: "b0".into(),
        ..Default::default()
    })));
    let mut bytes = Vec::new();
    d.save(&mut bytes).unwrap();
    let err = import_bytes("chain.dxf", &bytes, &ImportOptions::default()).unwrap_err();
    assert!(err.message.contains("recursion/expansion"), "{err}");
}

#[test]
fn svg_reference_resolution_and_point_cost_cannot_disagree_with_parser() {
    for svg in [
        r#"<svg xmlns="http://www.w3.org/2000/svg"><path id="same" d="M0 0L1 1"/><path id="same" d="M0 0L2 2"/></svg>"#,
        r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><path id="p" d="M0 0L1 1"/><path id="q" d="M0 0L2 2"/><use href="#p" xlink:href="#q"/></svg>"##,
        r#"<svg xmlns="http://www.w3.org/2000/svg"><defs><marker id="m"><path d="M0 0L1 1"/></marker></defs><path d="M0 0L1 1" marker-mid="url(#m)"/></svg>"#,
    ] {
        assert!(import_bytes("ambiguous.svg", svg.as_bytes(), &ImportOptions::default()).is_err());
    }
    let mut points = String::new();
    for i in 0..100 {
        use std::fmt::Write as _;
        write!(points, "{i},{} ", i % 3).unwrap();
    }
    let uses = "<use href=\"#p\"/>".repeat(2000);
    let svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\"><defs><polyline id=\"p\" points=\"{points}\"/></defs>{uses}</svg>");
    let err = import_bytes("points.svg", svg.as_bytes(), &ImportOptions::default()).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Limit);
}
