//! Frozen v9 outputs remain immutable. Unaffected families are byte-identical;
//! Main-only differences are restricted to explicit stage/obsolete tag comments.
#[path = "support/explicit.rs"]
mod fixture_support;
use ivac_core::pipeline::{run_pipeline, run_pipeline_two_sided, PipelineRequest};
use serde_json::Value;
#[test]
fn retained_outputs_and_controlled_main_stage_differences() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/fs-reset3-output");
    let hashes: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("SHA256SUMS.json")).unwrap())
            .unwrap();
    let compare = |name: &str, expected: &Value, actual: &Value, changed: bool| {
        if changed {
            assert!(!actual["cutting_stages"].as_array().unwrap().is_empty());
            if name == "dual-pocket.json" {
                // Explicit independent Finishing now clears the whole reachable rectangle.
                // The analytical swept-area oracle in alpha3_v10 checks removal and rates.
                assert_eq!(actual["cutting_stages"].as_array().unwrap().len(), 2);
                assert_eq!(
                    actual["cutting_stages"][0]["parameters"]["feed_mm_min"],
                    811
                );
                assert_eq!(
                    actual["cutting_stages"][1]["parameters"]["feed_mm_min"],
                    413
                );
                assert_eq!(actual["cutting_stages"][1]["parameters"]["tool_id"], 2);
                assert!(
                    actual["toolpath"].as_array().unwrap().len()
                        > expected["toolpath"].as_array().unwrap().len()
                );
            } else {
                assert_eq!(
                    fixture_support::neutral_stage_comments(expected["gcode"].as_str().unwrap()),
                    fixture_support::neutral_stage_comments(actual["gcode"].as_str().unwrap()),
                    "{name}: only reviewed comments may change"
                );
                let movements = |v: &Value| {
                    v["toolpath"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|s| {
                            let mut s = s.clone();
                            s.as_object_mut().unwrap().remove("gcode_line");
                            s
                        })
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    movements(expected),
                    movements(actual),
                    "{name}: same exact decoded movements and tools"
                );
            }
        } else {
            for field in [
                "gcode",
                "toolpath",
                "tool_provenance",
                "gcode_index",
                "stats",
            ] {
                assert_eq!(expected[field], actual[field], "{name}: {field}");
            }
        }
    };
    for name in hashes.as_object().unwrap().keys() {
        if name.starts_with("shopbot-") && !cfg!(feature = "cps") {
            continue;
        }
        let fixture: Value =
            serde_json::from_str(&std::fs::read_to_string(root.join(name)).unwrap()).unwrap();
        let old: PipelineRequest = serde_json::from_value(fixture["request"].clone()).unwrap();
        let changed = old
            .project
            .operations
            .iter()
            .any(ivac_core::cutting_stages::applies);
        let req = fixture_support::reviewed_v10(old);
        let input = serde_json::to_value(&req).unwrap();
        let output = if fixture.get("two_sided").is_some() {
            let out =
                serde_json::to_value(run_pipeline_two_sided(req, |_, _, _| {}).unwrap()).unwrap();
            for side in ["front", "back"] {
                compare(name, &fixture["two_sided"][side], &out[side], changed);
            }
            out
        } else {
            let out = serde_json::to_value(run_pipeline(req, |_, _, _| {}).unwrap()).unwrap();
            compare(name, &fixture["response"], &out, changed);
            out
        };
        if let Some(dir) = std::env::var_os("ALPHA3_DIFFERENTIAL_DIR") {
            let dir = std::path::PathBuf::from(dir);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join(name),serde_json::to_vec_pretty(&serde_json::json!({"historical":fixture,"v10_request":input,"v10_output":output})).unwrap()).unwrap();
        }
    }
}
