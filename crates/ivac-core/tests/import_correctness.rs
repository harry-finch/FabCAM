//! DXF OCS / BLOCK / unit and SVG unit import contracts against analytically
//! computed expectations (`tests/fixtures/import-correctness/generate.py`
//! writes the fixtures and `expected.json` independently of this importer).
use std::path::PathBuf;

use ivac_core::geometry::{Point2, Segment, SegmentKind};
use ivac_core::input::{import_path, ImportOptions};
use serde_json::Value;

/// Comparison tolerance for analytically computed coordinates (mm).
const TOL: f64 = 1e-6;
/// The importer's 0.01 mm chord budget for sampled curves (mm).
const CURVE_TOL: f64 = 0.0101;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/import-correctness")
}

fn xy(v: &Value) -> Point2 {
    Point2::new(v[0].as_f64().unwrap(), v[1].as_f64().unwrap())
}

fn near(a: Point2, b: Point2, tol: f64) -> bool {
    a.distance(b) <= tol
}

/// Consume the imported segments in order against the expected primitives.
fn check(chains: &[Value], segs: &[Segment]) -> Result<(), String> {
    let mut i = 0;
    let at = |i: usize| segs.get(i).ok_or_else(|| format!("missing segment {i}"));
    for prim in chains {
        if let Some(l) = prim.get("line") {
            let s = at(i)?;
            if s.kind != SegmentKind::Line
                || !near(s.start, xy(&l[0]), TOL)
                || !near(s.end, xy(&l[1]), TOL)
            {
                return Err(format!("segment {i}: expected line {l}, got {s:?}"));
            }
            i += 1;
        } else if let Some(a) = prim.get("arc") {
            let (c, start, end) = (xy(&a["center"]), xy(&a["start"]), xy(&a["end"]));
            let ccw = a["ccw"].as_bool().unwrap();
            let r = c.distance(start);
            if !near(at(i)?.start, start, TOL) {
                return Err(format!(
                    "segment {i}: arc start {:?} != {start:?}",
                    at(i)?.start
                ));
            }
            loop {
                let s = at(i)?;
                let centered = s.center.is_some_and(|sc| near(sc, c, TOL));
                if s.kind != SegmentKind::Arc || !centered || (c.distance(s.end) - r).abs() > TOL {
                    return Err(format!("segment {i}: expected arc about {c:?}, got {s:?}"));
                }
                if (s.bulge > 0.0) != ccw {
                    return Err(format!(
                        "segment {i}: wrong arc direction (bulge {})",
                        s.bulge
                    ));
                }
                i += 1;
                if near(s.end, end, TOL) {
                    break;
                }
            }
        } else if let Some(cl) = prim.get("circle") {
            let (c, r) = (xy(&cl["center"]), cl["r"].as_f64().unwrap());
            for _ in 0..2 {
                let s = at(i)?;
                let centered = s.center.is_some_and(|sc| near(sc, c, TOL));
                if s.kind != SegmentKind::Circle
                    || !centered
                    || (c.distance(s.start) - r).abs() > TOL
                {
                    return Err(format!(
                        "segment {i}: expected circle about {c:?} r {r}, got {s:?}"
                    ));
                }
                i += 1;
            }
        } else if let Some(cv) = prim.get("curve") {
            let (c, u, v) = (xy(&cv["center"]), xy(&cv["u"]), xy(&cv["v"]));
            let det = u.x * v.y - u.y * v.x;
            if !near(at(i)?.start, xy(&cv["start"]), CURVE_TOL) {
                return Err(format!("segment {i}: curve start mismatch"));
            }
            loop {
                let s = at(i)?;
                for p in [s.start, s.end] {
                    let (dx, dy) = (p.x - c.x, p.y - c.y);
                    let (a, b) = ((dx * v.y - dy * v.x) / det, (u.x * dy - u.y * dx) / det);
                    if (a.hypot(b) - 1.0).abs() > 1e-6 {
                        return Err(format!("segment {i}: {p:?} is not on the expected ellipse"));
                    }
                }
                i += 1;
                if near(s.end, xy(&cv["end"]), CURVE_TOL) {
                    break;
                }
            }
        }
    }
    if i == segs.len() {
        Ok(())
    } else {
        Err(format!("{} unexpected extra segment(s)", segs.len() - i))
    }
}

#[test]
fn import_contracts_match_analytic_expectations() {
    let cases: Vec<Value> =
        serde_json::from_str(&std::fs::read_to_string(dir().join("expected.json")).unwrap())
            .unwrap();
    let mut failures = Vec::new();
    for case in &cases {
        let name = case["name"].as_str().unwrap();
        let result = import_path(
            &dir().join(case["file"].as_str().unwrap()),
            &ImportOptions::default(),
        );
        let outcome = match (result, case.get("error").and_then(Value::as_str)) {
            (Err(e), Some(expected))
                if e.message.to_lowercase().contains(&expected.to_lowercase()) =>
            {
                Ok(())
            }
            (Err(e), _) => Err(format!("unexpected error: {}", e.message)),
            (Ok(_), Some(_)) => Err("expected a refusal".into()),
            (Ok(out), None) => {
                let warned = case.get("warning").and_then(Value::as_str);
                let geometry = if let Some(chains) = case.get("chains").and_then(Value::as_array) {
                    check(chains, &out.segments)
                } else {
                    let width = out.bbox.max_x - out.bbox.min_x;
                    let want = case["width_mm"].as_f64().unwrap();
                    if (width - want).abs() <= 1e-3 {
                        Ok(())
                    } else {
                        Err(format!("width {width} mm != {want} mm"))
                    }
                };
                geometry.and_then(|()| match warned {
                    Some(w) if !out.warnings.iter().any(|x| x.to_lowercase().contains(w)) => {
                        Err(format!("missing '{w}' warning: {:?}", out.warnings))
                    }
                    None if !out.warnings.is_empty() => {
                        Err(format!("unexpected warnings {:?}", out.warnings))
                    }
                    _ => Ok(()),
                })
            }
        };
        if let Err(why) = outcome {
            failures.push(format!("{name}: {why}"));
        }
    }
    assert!(cases.len() >= 38, "fixture set shrank: {}", cases.len());
    assert!(
        failures.is_empty(),
        "{} failure(s):\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// An explicit import scale is the user's unit decision: no ambiguity warning.
#[test]
fn explicit_scale_silences_unit_ambiguity() {
    for file in ["dxf_units_absent.dxf", "svg_px.svg"] {
        let opts = ImportOptions {
            scale: 0.264_583_333,
            ..ImportOptions::default()
        };
        let out = import_path(&dir().join(file), &opts).unwrap();
        assert!(out.warnings.is_empty(), "{file}: {:?}", out.warnings);
    }
}

/// ADR-0026 unit contract: status, declared source, explicit scale and the
/// refusal that prevents converting a physical SVG twice.
#[test]
fn unit_status_and_explicit_scale_contract() {
    use ivac_core::input::{checked_import_scale, ImportUnitStatus as S};
    let import = |file: &str, scale: f64| {
        import_path(
            &dir().join(file),
            &ImportOptions {
                scale,
                ..ImportOptions::default()
            },
        )
    };
    let width = |o: &ivac_core::ImportOutput| o.bbox.max_x - o.bbox.min_x;
    for (file, status, source) in [
        ("svg_mm.svg", S::Physical, "width='100mm', height='50mm'"),
        ("svg_px.svg", S::Assumed, "width='96px', height='96px'"),
        ("svg_unitless.svg", S::Assumed, "width='96', height='96'"),
        (
            "svg_viewbox_only.svg",
            S::Assumed,
            "width='absent', height='absent'",
        ),
        ("dxf_units_inch.dxf", S::Declared, "$INSUNITS = Inches"),
        (
            "dxf_units_absent.dxf",
            S::Assumed,
            "$INSUNITS = absent or unitless",
        ),
        (
            "dxf_units_unitless.dxf",
            S::Assumed,
            "$INSUNITS = absent or unitless",
        ),
    ] {
        let out = import(file, 0.0).unwrap();
        assert_eq!(
            (out.unit_status, out.unit_source.as_str()),
            (status, source),
            "{file}"
        );
    }
    // Explicit choices on ambiguous sources: 96 px at 96 dpi is one inch.
    let px96 = import("svg_px.svg", 25.4 / 96.).unwrap();
    assert_eq!(px96.unit_status, S::Explicit);
    assert!((width(&px96) - 25.4).abs() < 1e-9 && px96.warnings.is_empty());
    let pt72 = import("svg_illustrator_pt_as_px.svg", 25.4 / 72.).unwrap();
    assert!(
        (width(&pt72) - 210.0).abs() < 0.01,
        "A4 width {}",
        width(&pt72)
    );
    let inch = import("dxf_units_absent.dxf", 25.4).unwrap();
    assert_eq!(inch.unit_status, S::Explicit);
    assert!((width(&inch) - 25.4).abs() < 1e-9);
    // A physical SVG never takes a second factor.
    let err = import("svg_mm.svg", 25.4 / 96.).unwrap_err();
    assert!(err.message.contains("twice"), "{}", err.message);
    assert!((width(&import("svg_mm.svg", 0.0).unwrap()) - 100.0).abs() < 1e-9);
    // Transport validation of explicit scales.
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, 1e-7, 1e7] {
        assert!(checked_import_scale(bad).is_err(), "{bad}");
    }
    for good in [1e-6, 25.4 / 96., 1.0, 25.4, 1e6] {
        assert_eq!(
            checked_import_scale(good).unwrap().to_bits(),
            good.to_bits()
        );
    }
}

/// Sources saved before 2026-10-20 have no unit fields; they read back as
/// `unknown` with their stored scale untouched.
#[test]
fn legacy_saved_source_reads_as_unknown_units() {
    let legacy = serde_json::json!({
        "filename": "old.svg", "format": "svg", "segments": [], "layers": [],
        "bbox": {"min_x": 0.0, "min_y": 0.0, "max_x": 10.0, "max_y": 5.0},
        "unit_scale": 1.0, "warnings": []
    });
    let out: ivac_core::ImportOutput = serde_json::from_value(legacy).unwrap();
    assert_eq!(out.unit_status, ivac_core::input::ImportUnitStatus::Unknown);
    assert_eq!(out.unit_source, "");
    assert!((out.unit_scale - 1.0).abs() < f64::EPSILON);
}

/// The frontend groups importer warnings by these exact shapes
/// (`frontend/src/lib/state/import-units.ts`); keep them in sync.
#[test]
fn warning_texts_match_the_frontend_classifier() {
    let solid = "0\nSOLID\n8\nHATCHING\n10\n0.0\n20\n0.0\n30\n0.0\n11\n1.0\n21\n0.0\n31\n0.0\n12\n0.0\n22\n1.0\n32\n0.0\n13\n1.0\n23\n1.0\n33\n0.0\n";
    let dxf = format!(
        "0\nSECTION\n2\nHEADER\n9\n$INSUNITS\n70\n4\n0\nENDSEC\n0\nSECTION\n2\nENTITIES\n{solid}{solid}\
         0\nINSERT\n8\n0\n2\nGHOST\n10\n0.0\n20\n0.0\n30\n0.0\n\
         0\nLINE\n8\n0\n10\n0.0\n20\n0.0\n30\n0.0\n11\n5.0\n21\n0.0\n31\n0.0\n0\nENDSEC\n0\nEOF\n"
    );
    let out =
        ivac_core::input::import_bytes("w.dxf", dxf.as_bytes(), &ImportOptions::default()).unwrap();
    assert_eq!(
        out.warnings,
        [
            "unsupported entity SOLID on layer 'HATCHING'",
            "unsupported entity SOLID on layer 'HATCHING'",
            "INSERT references missing block 'GHOST'",
        ]
    );
    assert_eq!(
        out.segments.len(),
        1,
        "the supported LINE is still imported"
    );
}
