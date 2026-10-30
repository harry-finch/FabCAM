//! Real HTTP extraction from original numeric lexemes; no typed input shortcut.
use super::*;
use axum::http::Request;
use tower::ServiceExt;

#[tokio::test]
async fn alpha3_http_matches_complete_v10_stage_output_and_refuses_v9() {
    let mut value: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/alpha3-v10/profile.request.json"
    ))
    .unwrap();
    value["post_processor"] = serde_json::json!("linuxcnc");
    value["cps_post"] = serde_json::Value::Null;
    let op = &mut value["project"]["operations"][0];
    op["finish_tool_id"] = serde_json::json!(1);
    op["params"]["stages"]["finishing"] = serde_json::json!({
        "cutting":op["params"]["stages"]["main"],
        "lateral_allowance_mm":0.1,"floor_allowance_mm":0.0});
    for version in [10, 9] {
        value["project"]["cutting_version"] = serde_json::json!(version);
        let request = Request::builder()
            .method("POST")
            .uri("/generate")
            .header("content-type", "application/json")
            .body(Body::from(value.to_string()))
            .unwrap();
        let response = Router::new()
            .route("/generate", post(generate))
            .with_state(Arc::new(AppState::default()))
            .oneshot(request)
            .await
            .unwrap();
        if version == 10 {
            assert_eq!(response.status(), StatusCode::OK);
            let bytes = axum::body::to_bytes(response.into_body(), 16 * 1024 * 1024)
                .await
                .unwrap();
            let actual: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let expected =
                run_pipeline(serde_json::from_value(value.clone()).unwrap(), |_, _, _| {}).unwrap();
            assert_eq!(actual, serde_json::to_value(expected).unwrap());
        } else {
            assert!(response.status().is_client_error());
        }
    }
}

#[tokio::test]
async fn numeric_text_reaches_authoritative_http_inspector() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/perf-v9/square-medial-linuxcnc.json"
    ))
    .unwrap();
    let mut project = fixture["project"].clone();
    project["cutting_version"] = 10.into();
    let id = project["operations"][0]["id"].clone();
    project["operations"][0]["params"]["cutting"]["entry_angle_deg"] =
        serde_json::json!("__LEXEME__");
    let template = serde_json::json!({"action":"inspect","project":project,"op_id":id}).to_string();
    for (lexeme, expected) in [
        ("15.122706142614561", Some(0x402e_3ed3_56eb_1880)),
        ("-0.0", Some(0x8000_0000_0000_0000)),
        ("5e-324", Some(1)),
        ("1e309", None),
        ("01", None),
    ] {
        let request = Request::builder()
            .method("POST")
            .uri("/presets/command")
            .header("content-type", "application/json")
            .body(Body::from(template.replace("\"__LEXEME__\"", lexeme)))
            .unwrap();
        let response = Router::new()
            .route("/presets/command", post(preset_command_handler))
            .oneshot(request)
            .await
            .unwrap();
        if let Some(bits) = expected {
            assert_eq!(response.status(), StatusCode::OK);
            let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
                .await
                .unwrap();
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(
                value["parameters"]["entry_angle_deg"]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                bits
            );
            // An inspected invalid angle is not a successful CAM validation.
            if lexeme != "15.122706142614561" {
                assert!(!value["check"]["errors"].as_array().unwrap().is_empty());
            }
        } else {
            assert!(response.status().is_client_error());
        }
    }
}

#[tokio::test]
async fn corrected_http_generation_matches_buffered_pipeline_and_refuses_invalid_values() {
    let fixture = include_str!("../../../tests/fixtures/perf-v9/square-medial-linuxcnc.json");
    for invalid in [false, true] {
        let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        value["project"]["cutting_version"] = 10.into();
        value["project"]["operations"][0]["params"]["cutting"]["entry_angle_deg"] =
            serde_json::json!("__LEXEME__");
        let body = value.to_string().replace(
            "\"__LEXEME__\"",
            if invalid {
                "1e309"
            } else {
                "15.122706142614561"
            },
        );
        let request = Request::builder()
            .method("POST")
            .uri("/generate")
            .header("content-type", "application/json")
            .body(Body::from(body.clone()))
            .unwrap();
        let response = Router::new()
            .route("/generate", post(generate))
            .with_state(Arc::new(AppState::default()))
            .oneshot(request)
            .await
            .unwrap();
        if invalid {
            assert!(response.status().is_client_error());
        } else {
            assert_eq!(response.status(), StatusCode::OK);
            let bytes = axum::body::to_bytes(response.into_body(), 16 * 1024 * 1024)
                .await
                .unwrap();
            let actual: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let parsed: PipelineRequest = serde_json::from_str(&body).unwrap();
            let expected = run_pipeline(parsed, |_, _, _| {}).unwrap();
            assert_eq!(actual, serde_json::to_value(expected).unwrap());
        }
    }
}

#[tokio::test]
async fn alpha31_http_polygon_finishing_and_coverage_refusal_match_core() {
    for fixture in [
        include_str!("../../../tests/fixtures/alpha31/convex-pocket.request.json"),
        include_str!("../../../tests/fixtures/alpha31/concave-pocket.request.json"),
    ] {
        let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        for refuse in [false, true] {
            if refuse {
                value["project"]["operations"][0]["params"]["stock_to_leave_mm"] =
                    serde_json::json!(30.);
            }
            let expected =
                run_pipeline(serde_json::from_value(value.clone()).unwrap(), |_, _, _| {});
            let request = Request::builder()
                .method("POST")
                .uri("/generate")
                .header("content-type", "application/json")
                .body(Body::from(value.to_string()))
                .unwrap();
            let response = Router::new()
                .route("/generate", post(generate))
                .with_state(Arc::new(AppState::default()))
                .oneshot(request)
                .await
                .unwrap();
            if refuse {
                assert!(expected.is_err());
                assert!(response.status().is_client_error());
            } else {
                assert_eq!(response.status(), StatusCode::OK);
                let bytes = axum::body::to_bytes(response.into_body(), 16 * 1024 * 1024)
                    .await
                    .unwrap();
                let actual: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                assert_eq!(actual, serde_json::to_value(expected.unwrap()).unwrap());
            }
        }
    }
}
