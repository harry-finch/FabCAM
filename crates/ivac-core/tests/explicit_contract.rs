//! Current wire contract: no source context or saved recommendation can bypass v9.
use ivac_core::pipeline::PipelineRequest;
use ivac_core::project::MaterialIdentity;
use serde_json::{json, Value};

fn request() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/cli-v9/profile.request.json"
    ))
    .unwrap()
}

#[test]
fn explicit_request_round_trips_without_legacy_state() {
    let req: PipelineRequest = serde_json::from_value(request()).unwrap();
    ivac_core::cutting_parameters::validate_project(&req.project).unwrap();
    let encoded = serde_json::to_value(&req).unwrap();
    assert_eq!(encoded["project"]["cutting_version"], 9);
    assert!(encoded["project"].get("fs_context").is_none());
    assert!(encoded["project"]["operations"][0]["params"]
        .get("applied_cutting")
        .is_none());
    let decoded: PipelineRequest = serde_json::from_value(encoded).unwrap();
    ivac_core::cutting_parameters::validate_project(&decoded.project).unwrap();
}

#[test]
fn rejects_obsolete_or_unknown_request_fields_instead_of_ignoring_them() {
    for field in ["fs_context", "unknown_cutting_context"] {
        let mut req = request();
        req["project"][field] = json!({});
        let error = serde_json::from_value::<PipelineRequest>(req).unwrap_err();
        assert!(error.to_string().contains(field));
    }
    for (field, value) in [
        ("applied_cutting", json!({"slots": []})),
        ("applied_cutting", Value::Null),
        ("fs_full_width", json!(false)),
        ("unknown_rate", json!(100)),
    ] {
        let mut req = request();
        req["project"]["operations"][0]["params"][field] = value;
        let error = serde_json::from_value::<PipelineRequest>(req).unwrap_err();
        assert!(error.to_string().contains(field));
    }
}

#[test]
fn material_identity_preserves_exact_revision_and_refuses_private_extensions() {
    let identity: MaterialIdentity =
        serde_json::from_value(json!({"id":"private-material", "revision":"07"})).unwrap();
    assert_eq!(identity.revision, "07");
    assert_eq!(
        serde_json::to_value(identity).unwrap(),
        json!({"id":"private-material", "revision":"07"})
    );
    assert!(serde_json::from_value::<MaterialIdentity>(
        json!({"id":"material", "revision":"1", "recommendation":{}})
    )
    .is_err());
}
