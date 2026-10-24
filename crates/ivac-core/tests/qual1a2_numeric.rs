//! Deterministic numeric contract, integrated in QUAL-1A.4. JSON bytes always cross the real serde reader.
use ivac_core::project::ReliefGrid;

#[test]
fn binary64_midpoints_and_range_are_explicit() {
    for (s, bits) in [
        ("15.122706142614561", 0x402e_3ed3_56eb_1880),
        ("-0", 0x8000_0000_0000_0000),
        ("-1e-999", 0x8000_0000_0000_0000),
        ("5e-324", 1),
        (
            "1.00000000000000011102230246251565404236316680908203125",
            0x3ff0_0000_0000_0000,
        ),
    ] {
        assert_eq!(serde_json::from_str::<f64>(s).unwrap().to_bits(), bits);
    }
    for s in ["1e309", "-1e309", "NaN", "Infinity", "01", "+1"] {
        assert!(serde_json::from_str::<f64>(s).is_err());
    }
    assert_eq!(
        serde_json::from_str::<u64>("9007199254740993").unwrap(),
        9_007_199_254_740_993
    );
    assert!(serde_json::from_str::<u32>("9007199254740993").is_err());
}

#[test]
fn binary32_direct_rounding_and_tagged_relief_boundary_are_distinguished() {
    // Just above the exact f32 midpoint 1 + 2^-24, but rounds to that midpoint
    // as f64. JS Number -> f32 therefore chooses even 1.0; direct f32 RNE is next.
    let lexeme = "1.000000059604644775390625000000000000000000000000001";
    let direct: f32 = serde_json::from_str(lexeme).unwrap();
    assert_eq!(direct.to_bits(), 0x3f80_0001);
    let grid: ReliefGrid =
        serde_json::from_str(&format!("{{\"kind\":\"heightgrid\",\"z\":[{lexeme}]}}")).unwrap();
    let ReliefGrid::Heightgrid { z } = grid else {
        panic!("heightgrid")
    };
    // Serde's internally tagged enum buffers numeric content as f64, matching
    // the existing JS-object relief boundary. This is not direct-f32 rounding.
    assert_eq!(z[0].to_bits(), 0x3f80_0000);
}

/// Expected encodings are from the independent integer/rational RNE oracle in
/// QUAL-1A.2, rechecked in QUAL-1A.4, not from JavaScript or this parser.
#[test]
fn independent_decimal_oracle_including_all_126_dense_coordinates() {
    let rows: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/qual1a4-numeric.json")).unwrap();
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 328);
    assert_eq!(
        rows.iter()
            .filter(|r| r["category"] == "known-coordinate")
            .count(),
        126
    );
    for row in rows {
        let text = row["lexeme"].as_str().unwrap();
        let typed = serde_json::from_str::<f64>(text);
        let reader = serde_json::from_reader::<_, f64>(text.as_bytes());
        if row["classification"] == "finite" {
            let expected = u64::from_str_radix(row["oracle_bits"].as_str().unwrap(), 16).unwrap();
            assert_eq!(typed.unwrap().to_bits(), expected, "{text}");
            assert_eq!(reader.unwrap().to_bits(), expected, "{text}");
            let value: serde_json::Value = serde_json::from_str(text).unwrap();
            assert_eq!(value.as_f64().unwrap().to_bits(), expected, "{text}");
        } else {
            assert!(typed.is_err(), "{text}");
            assert!(reader.is_err(), "{text}");
        }
    }
}
