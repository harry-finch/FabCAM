use ivac_feeds_speeds::*;
use serde_json::{json, Value};
const DATA: &str = include_str!("../../../data/feeds-speeds/sienci-router-reference.v1.json");
fn request(doc: &DatasetDocument, index: usize) -> FeedsSpeedsRequest {
    let profile = &doc.profile_set.profiles[index];
    let tool = doc
        .tools
        .iter()
        .find(|v| Some(&v.identity) == profile.scope.tool.as_ref())
        .unwrap();
    let doc_mm = match profile.data {
        CuttingData::ExactValues { doc_mm, .. } => doc_mm,
        _ => unreachable!(),
    };
    FeedsSpeedsRequest {
        schema_version: DOMAIN_SCHEMA_VERSION,
        dataset: doc.profile_set.identity.clone(),
        machine: MachineSnapshot {
            identity: Some(doc.machines[0].identity.clone()),
            rpm_min: None,
            rpm_max: None,
            max_feed_mm_min: None,
            max_plunge_mm_min: None,
        },
        material: MaterialSnapshot {
            identity: Some(doc.materials[0].identity.clone()),
            thickness_mm: None,
        },
        tool: ToolSnapshot {
            identity: Some(tool.identity.clone()),
            cutting_family: tool.cutting_family,
            diameter_mm: tool.diameter_mm,
            flute_count: tool.flute_count,
            max_rpm: tool.max_rpm,
        },
        operation: OperationContext {
            kind: OperationKind::FullSlot,
            doc_mm: Some(doc_mm),
            engagement: Engagement::KnownFullWidth,
            entry: EntryStrategy::Unknown,
        },
        aggressiveness: profile.aggressiveness,
    }
}
fn available(engine: &Engine, r: &FeedsSpeedsRequest) -> Box<Recommendation> {
    match engine.recommend(r) {
        FeedsSpeedsRecommendation::Available { recommendation } => recommendation,
        other => panic!("{other:?}"),
    }
}
fn unavailable(engine: &Engine, r: &FeedsSpeedsRequest) {
    assert_eq!(
        engine.recommend(r),
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::MissingCuttingData
        }
    );
}
fn reject(mutator: impl FnOnce(&mut Value)) {
    let mut data: Value = serde_json::from_str(DATA).unwrap();
    mutator(&mut data);
    assert!(load_dataset_json(&data.to_string()).is_err());
}
#[test]
fn source_rows_transcribed_without_derating_or_chipload_recomputation() {
    let loaded = load_dataset_json(DATA).unwrap();
    let doc = loaded.document();
    assert_eq!(doc.profile_set.profiles.len(), 6);
    let expected = [
        (17000, 1620, 810, 2.8),
        (17000, 2160, 1080, 4.1),
        (17000, 2700, 1350, 5.5),
        (22000, 1050, 530, 4.3),
        (22000, 1400, 700, 6.4),
        (22000, 1750, 880, 8.5),
    ];
    for (i, (rpm, feed, plunge, depth)) in expected.into_iter().enumerate() {
        let r = request(doc, i);
        let recommendation = available(loaded.engine(), &r);
        assert_eq!(
            (
                recommendation.parameters.rpm,
                recommendation.parameters.feed_mm_min,
                recommendation.parameters.plunge_mm_min,
                recommendation.parameters.doc_mm
            ),
            (rpm, feed, plunge, depth)
        );
        assert_eq!(recommendation.source_evidence, Evidence::ReferenceData);
        assert_eq!(
            recommendation.applicability,
            Applicability::WithinDeclaredProfile
        );
        assert_eq!(
            recommendation.provenance.profile,
            doc.profile_set.profiles[i]
        );
        assert!(recommendation
            .diagnostics
            .contains(&Diagnostic::UnknownEntryStrategy));
        assert_eq!(
            recommendation.parameters.effective_chipload_mm_tooth,
            f64::from(feed) / f64::from(rpm)
        );
    }
}
#[test]
fn explicit_roundtrip_retains_catalog_and_source_snapshots() {
    let d = load_dataset_json(DATA).unwrap();
    let encoded = serde_json::to_string(d.document()).unwrap();
    assert_eq!(
        load_dataset_json(&encoded).unwrap().document(),
        d.document()
    );
    let r = request(d.document(), 0);
    let a = available(d.engine(), &r);
    let restored: Recommendation =
        serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
    assert_eq!(*a, restored);
}
#[test]
fn no_shopbot_or_unknown_machine_transfer() {
    let d = load_dataset_json(DATA).unwrap();
    let mut r = request(d.document(), 0);
    r.machine.identity = Some(Identity {
        id: "lab-shopbot".into(),
        revision: "1".into(),
    });
    unavailable(d.engine(), &r);
    r.machine.identity = None;
    unavailable(d.engine(), &r);
}
#[test]
fn no_silent_metric_diameter_material_or_tool_transfer() {
    let d = load_dataset_json(DATA).unwrap();
    let r = request(d.document(), 0);
    let mut edited = r.clone();
    edited.tool.diameter_mm = 6.0;
    unavailable(d.engine(), &edited);
    edited = r.clone();
    edited.tool.flute_count = 2;
    unavailable(d.engine(), &edited);
    edited = r.clone();
    edited.tool.identity = None;
    unavailable(d.engine(), &edited);
    edited = r;
    edited.material.identity = Some(Identity {
        id: "mdf".into(),
        revision: "1".into(),
    });
    unavailable(d.engine(), &edited);
}
#[test]
fn no_pocket_finish_or_unknown_doc_transfer() {
    let d = load_dataset_json(DATA).unwrap();
    let r = request(d.document(), 0);
    for kind in [
        OperationKind::Pocket,
        OperationKind::Finish,
        OperationKind::Profile,
    ] {
        let mut e = r.clone();
        e.operation.kind = kind;
        unavailable(d.engine(), &e);
    }
    let mut e = r.clone();
    e.operation.doc_mm = None;
    unavailable(d.engine(), &e);
    e = r;
    e.operation.doc_mm = Some(3.0);
    unavailable(d.engine(), &e);
}
#[test]
fn external_reference_never_becomes_locally_validated_after_clamp() {
    let d = load_dataset_json(DATA).unwrap();
    let mut r = request(d.document(), 0);
    r.machine.rpm_max = Some(16000);
    let a = available(d.engine(), &r);
    assert_eq!(a.parameters.feed_mm_min, 1620);
    assert_eq!(a.parameters.rpm, 16000);
    assert_eq!(a.source_evidence, Evidence::ReferenceData);
    assert_eq!(a.applicability, Applicability::AdjustedOutsideEvidence);
    assert!(!a.provenance.adjustments.is_empty());
}
#[test]
fn schema_versions_unknown_fields_duplicate_fields_and_units_refused() {
    reject(|v| v["format_version"] = json!(2));
    reject(|v| v["profile_set"]["schema_version"] = json!(2));
    reject(|v| v["profile_set"]["profiles"][0]["data"]["feed_inches_min"] = json!(64));
    reject(|v| v["unexpected"] = json!(true));
    assert!(load_dataset_json(&DATA.replacen(
        "\"format_version\": 1",
        "\"format_version\": 1, \"format_version\": 1",
        1
    ))
    .is_err());
    assert!(load_dataset_json("{truncated").is_err());
}
#[test]
fn dangling_revisions_and_family_members_refused() {
    reject(|v| {
        v["profile_set"]["profiles"][0]["scope"]["material"]["identity"]["revision"] = json!("2")
    });
    reject(|v| v["profile_set"]["profiles"][0]["scope"]["tool"]["id"] = json!("absent"));
    reject(|v| {
        v["profile_set"]["machine_families"] =
            json!([{"id":"lab","members":[{"id":"absent","revision":"1"}]}])
    });
}
#[test]
fn catalog_duplicates_geometry_conflicts_and_hard_limit_conflicts_refused() {
    reject(|v| {
        let entry = v["materials"][0].clone();
        v["materials"].as_array_mut().unwrap().push(entry);
    });
    reject(|v| v["tools"][0]["diameter_mm"] = json!(6));
    reject(|v| v["tools"][0]["flute_count"] = json!(0));
    reject(|v| v["tools"][0]["max_rpm"] = json!(15000));
}
#[test]
fn incomplete_data_provenance_or_unearned_evidence_refused_atomically() {
    reject(|v| {
        v["profile_set"]["profiles"][0]["data"]
            .as_object_mut()
            .unwrap()
            .remove("plunge_mm_min");
    });
    reject(|v| v["profile_set"]["profiles"][0]["source"]["permission_or_license"] = json!(""));
    reject(|v| v["profile_set"]["profiles"][0]["evidence"] = json!("validated"));
    reject(|v| v["profile_set"]["profiles"][0]["data"]["feed_mm_min"] = json!(-1));
}

#[test]
fn local_trial_records_are_distinct_from_external_references() {
    // Synthetic authoring proof only: this is never distributed as cutting data.
    for (evidence, criteria) in [
        ("tested", Value::Null),
        ("validated", json!("synthetic:criteria")),
    ] {
        let mut data: Value = serde_json::from_str(DATA).unwrap();
        let p = &mut data["profile_set"]["profiles"][0];
        p["category"] = json!("exact_preset");
        p["evidence"] = json!(evidence);
        p["scope"]["entry"] = json!("direct");
        p["source"]["evidence_record"] =
            json!({"trial_reference":"synthetic:trial","validation_criteria_reference":criteria});
        let d = load_dataset_json(&data.to_string()).unwrap();
        let mut r = request(d.document(), 0);
        r.operation.entry = EntryStrategy::Direct;
        let a = available(d.engine(), &r);
        assert_eq!(
            a.source_evidence,
            if evidence == "tested" {
                Evidence::Tested
            } else {
                Evidence::Validated
            }
        );
        assert_eq!(
            a.provenance
                .profile
                .source
                .evidence_record
                .as_ref()
                .unwrap()
                .trial_reference,
            "synthetic:trial"
        );
    }
}

#[test]
fn missing_data_explains_actual_scope_without_relaxing_matching() {
    let loaded = load_dataset_json(DATA).unwrap();
    let mut r = request(loaded.document(), 0);
    assert!(loaded.engine().scope_rejections(&r).is_empty());
    r.operation.doc_mm = Some(1.0);
    assert_eq!(
        loaded.engine().recommend(&r),
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::MissingCuttingData
        }
    );
    let rows = loaded.engine().scope_rejections(&r);
    assert!(!rows.is_empty());
    for row in &rows {
        assert_eq!(row.mismatches.len(), 1);
        let mismatch = &row.mismatches[0];
        assert_eq!(mismatch.field, "doc_mm");
        assert_eq!(
            serde_json::from_str::<Value>(&mismatch.actual).unwrap(),
            json!(1.0)
        );
        assert_eq!(
            serde_json::from_str::<Value>(&mismatch.required).unwrap(),
            json!({"min":2.8,"max":2.8})
        );
    }
    r.machine.identity = None;
    assert!(loaded
        .engine()
        .scope_rejections(&r)
        .iter()
        .all(|row| row.mismatches.iter().any(|m| m.field == "machine")));
    r.operation.kind = OperationKind::Pocket;
    assert!(loaded
        .engine()
        .scope_rejections(&r)
        .iter()
        .all(|row| row.mismatches.iter().any(|m| m.field == "operation")));
    assert_eq!(
        loaded.engine().scope_rejections(&r),
        loaded.engine().scope_rejections(&r)
    );
}
