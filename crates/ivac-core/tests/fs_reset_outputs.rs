//! Fixed, pre-removal explicit v9 programs; no legacy generation path is a test oracle.
use ivac_core::pipeline::{run_pipeline, run_pipeline_two_sided, PipelineRequest};
use serde_json::Value;
#[test]
fn retained_explicit_programs_toolpaths_and_provenance_match_pre_removal() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/fs-reset3-output");
    let hashes: Value = serde_json::from_str(&std::fs::read_to_string(root.join("SHA256SUMS.json")).unwrap()).unwrap();
    let compare = |name: &str, expected: &Value, actual: &Value| {
        for field in ["gcode", "toolpath", "tool_provenance", "gcode_index", "stats"] {
            assert_eq!(expected[field],actual[field],"{name}: {field}");
        }
    };
    for name in hashes.as_object().unwrap().keys() {
        if name.starts_with("shopbot-") && !cfg!(feature="cps") {continue;}
        let fixture: Value = serde_json::from_str(&std::fs::read_to_string(root.join(name)).unwrap()).unwrap();
        let req: PipelineRequest = serde_json::from_value(fixture["request"].clone()).unwrap();
        if fixture.get("two_sided").is_some() {
            let output=serde_json::from_slice::<Value>(&serde_json::to_vec(&run_pipeline_two_sided(req,|_,_,_|{}).unwrap()).unwrap()).unwrap();
            for side in ["front","back"] {compare(name,&fixture["two_sided"][side],&output[side]);}
        } else {
            let output=serde_json::from_slice::<Value>(&serde_json::to_vec(&run_pipeline(req,|_,_,_|{}).unwrap()).unwrap()).unwrap();
            compare(name,&fixture["response"],&output);
        }
    }
}
