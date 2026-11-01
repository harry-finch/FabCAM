//! Cutter specifications must never enter machining calculations or machine output.
use ivac_core::pipeline::{run_pipeline, PipelineRequest};

#[test]
fn metadata_preserves_profile_pocket_drill_and_vcarve_outputs_and_preview() {
    for source in [
        include_str!("../../../tests/fixtures/alpha3-v10/profile.request.json"),
        include_str!("../../../tests/fixtures/alpha32/scoped-pocket.request.json"),
        include_str!("../../../tests/fixtures/fs-reset3-output/drill-shared-false.json"),
        include_str!("../../../tests/fixtures/perf-v9/square-medial-linuxcnc.json"),
    ] {
        let value: serde_json::Value = serde_json::from_str(source).unwrap();
        let mut input = value.get("request").unwrap_or(&value).clone();
        input["project"]["cutting_version"] = serde_json::json!(10);
        input["post_processor"] = serde_json::json!("linuxcnc");
        input.as_object_mut().unwrap().remove("cps_post");
        let request: PipelineRequest = serde_json::from_value(input).unwrap();
        let control = run_pipeline(request.clone(), |_, _, _| {}).unwrap();
        assert!(!control.gcode.is_empty());
        for (substrate, coating) in [("carbide", "uncoated"), ("HSS custom", "DLC custom")] {
            let mut changed = request.clone();
            for tool in &mut changed.project.tools {
                tool.substrate = Some(substrate.into());
                tool.coating = Some(coating.into());
            }
            let mut output = run_pipeline(changed, |_, _, _| {}).unwrap();
            // ADR-0025 snapshots correctly retain the new specifications. Only those
            // two authored metadata values differ; all executable and preview fields
            // and all remaining provenance must be byte-for-byte equivalent.
            for binding in &mut output.tool_provenance.bindings {
                assert_eq!(binding.tool.substrate.as_deref(), Some(substrate));
                assert_eq!(binding.tool.coating.as_deref(), Some(coating));
                binding.tool.substrate = None;
                binding.tool.coating = None;
            }
            let actual = serde_json::to_value(output).unwrap();
            let expected = serde_json::to_value(&control).unwrap();
            for (field, expected_value) in expected.as_object().unwrap() {
                assert!(
                    actual[field] == *expected_value,
                    "changed response field: {field}"
                );
            }
        }
    }
}
