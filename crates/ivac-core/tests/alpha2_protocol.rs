//! Existing reader boundaries, before any protocol changes.
use ivac_core::drawing::api::{evaluate, DrawingRequest};
use ivac_core::drawing::repair::Finding;
#[test]
fn v1_finding_cannot_represent_global_diagnostic_without_invented_contour() {
    assert!(serde_json::from_value::<Finding>(serde_json::json!({
        "contour": null, "message":"Diagnostic budget exceeded", "repair":null
    }))
    .is_err());
}
#[test]
fn old_request_refuses_new_placement_context_instead_of_ignoring_it() {
    assert!(serde_json::from_value::<DrawingRequest>(serde_json::json!({
        "kind":"start", "version":1, "imports":[],"legacy_offset":{"x":0.,"y":0.},
        "placement_context":[]
    }))
    .is_err());
}
#[test]
fn old_engine_refuses_protocol_two() {
    let r = serde_json::from_value::<DrawingRequest>(serde_json::json!({
        "kind":"start", "version":2, "imports":[],"legacy_offset":{"x":0.,"y":0.}
    }))
    .unwrap();
    assert_eq!(
        evaluate(r).unwrap_err(),
        "Unsupported GEOM1 protocol revision"
    );
}
