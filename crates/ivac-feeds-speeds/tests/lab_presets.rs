//! Synthetic SOFTWARE ONLY attestations, never physical machining evidence.
use ivac_feeds_speeds::*;
fn lab_pack() -> SourcePack {
    let mut p = load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
    ))
    .unwrap();
    p.manifest.identity = Identity {
        id: LAB_PACK_ID.into(),
        revision: "0001791792000000".into(),
    };
    p.manifest.schema_version = 2;
    p.manifest.visibility = Visibility::Private;
    p.manifest.distribution = Distribution::LocalOnly;
    p.dataset.profile_set.profiles.truncate(1);
    p.records.truncate(1);
    let profile = &mut p.dataset.profile_set.profiles[0];
    profile.category = ProfileCategory::ExactPreset;
    profile.evidence = Evidence::Tested;
    profile.scope.entry = Some(EntryStrategy::Direct);
    profile.source.evidence_record = Some(EvidenceRecord {
        trial_reference: "SOFTWARE ONLY synthetic trial".into(),
        validation_criteria_reference: None,
    });
    p.records[0].lab = Some(LabPreset {
        name: "SOFTWARE ONLY local preset".into(),
        state: LabState::Tested,
        created_at: "2026-10-12T08:00:00Z".into(),
        updated_at: "2026-10-12T08:00:00Z".into(),
        tested_at: Some("2026-10-12".into()),
        physically_tested: true,
        no_critical_failure: true,
        operator: "Synthetic operator".into(),
        job_reference: "SOFTWARE TEST".into(),
        observations: "No actual machining performed".into(),
        detailed_notes: String::new(),
        finishing_context: String::new(),
        enabled: true,
        deleted: false,
        review: None,
    });
    p
}
fn request(p: &SourcePack) -> FeedsSpeedsRequest {
    let s = &p.dataset.profile_set.profiles[0].scope;
    FeedsSpeedsRequest {
        schema_version: 1,
        dataset: p.dataset.profile_set.identity.clone(),
        machine: MachineSnapshot {
            local_identity: None,
            identity: Some(p.dataset.machines[0].identity.clone()),
            rpm_min: None,
            rpm_max: None,
            max_feed_mm_min: None,
            max_plunge_mm_min: None,
        },
        material: MaterialSnapshot {
            identity: Some(p.dataset.materials[0].identity.clone()),
            thickness_mm: None,
        },
        tool: ToolSnapshot {
            local_identity: None,
            identity: s.tool.clone(),
            cutting_family: s.cutting_family,
            diameter_mm: 6.35,
            flute_count: 1,
            max_rpm: None,
        },
        operation: OperationContext {
            kind: OperationKind::FullSlot,
            doc_mm: Some(2.8),
            engagement: Engagement::KnownFullWidth,
            entry: EntryStrategy::Direct,
        },
        aggressiveness: Aggressiveness::Prudent,
    }
}
#[test]
fn local_capture_round_trip_requires_successful_physical_attestation() {
    let p = lab_pack();
    validate_source_pack(&p).unwrap();
    assert_eq!(
        load_source_pack_json(&serde_json::to_string(&p).unwrap()).unwrap(),
        p
    );
    let mut invalid = p.clone();
    invalid.records[0].lab.as_mut().unwrap().physically_tested = false;
    assert!(validate_source_pack(&invalid).is_err());
    invalid = p.clone();
    invalid.records[0].lab.as_mut().unwrap().no_critical_failure = false;
    assert!(validate_source_pack(&invalid).is_err());
    invalid = p;
    invalid.records[0].lab.as_mut().unwrap().tested_at = None;
    assert!(validate_source_pack(&invalid).is_err());
}
#[test]
fn public_lab_metadata_and_incomplete_validation_are_refused() {
    let mut p = lab_pack();
    p.manifest.distribution = Distribution::Bundled;
    p.manifest.visibility = Visibility::Public;
    assert!(validate_source_pack(&p).is_err());
    p = lab_pack();
    p.records[0].lab.as_mut().unwrap().state = LabState::Validated;
    p.dataset.profile_set.profiles[0].evidence = Evidence::Validated;
    p.dataset.profile_set.profiles[0]
        .source
        .evidence_record
        .as_mut()
        .unwrap()
        .validation_criteria_reference = Some(LAB_VALIDATION_POLICY.into());
    assert!(validate_source_pack(&p).is_err());
    p.records[0].lab.as_mut().unwrap().review = Some(LabReview {
        policy: LAB_VALIDATION_POLICY.into(),
        reviewed_at: "2026-10-12T09:00:00Z".into(),
        notes: "SOFTWARE ONLY synthetic review".into(),
        acceptable_finish_and_behavior: true,
        no_overheating_chatter_or_overload: true,
        suitable_for_lab_use: true,
    });
    validate_source_pack(&p).unwrap();
    p.records[0]
        .lab
        .as_mut()
        .unwrap()
        .review
        .as_mut()
        .unwrap()
        .suitable_for_lab_use = false;
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn local_tested_precedence_uses_existing_resolver_and_disabled_deleted_presets_are_ignored() {
    let p = lab_pack();
    let r = request(&p);
    let public = load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
    ))
    .unwrap();
    let result = recommend_packs(&[public.clone(), p.clone()], &r).unwrap();
    assert_eq!(
        result.explanation.unwrap().summary.selected_pack.id,
        LAB_PACK_ID
    );
    for deleted in [false, true] {
        let mut disabled = p.clone();
        disabled.records[0].lab.as_mut().unwrap().enabled = false;
        disabled.records[0].lab.as_mut().unwrap().deleted = deleted;
        let result = recommend_packs(&[public.clone(), disabled], &r).unwrap();
        assert_eq!(
            result.explanation.unwrap().summary.selected_pack.id,
            public.manifest.identity.id
        );
    }
}
#[test]
fn draft_is_saved_but_never_supplies_recommendations() {
    let mut p = lab_pack();
    let r = request(&p);
    p.dataset.profile_set.profiles[0].category = ProfileCategory::Theoretical;
    p.dataset.profile_set.profiles[0].evidence = Evidence::Theoretical;
    p.dataset.profile_set.profiles[0].scope.engagement = EngagementScope::UnknownSafe;
    p.dataset.profile_set.profiles[0].scope.operation = OperationKind::Profile;
    let l = p.records[0].lab.as_mut().unwrap();
    l.state = LabState::Draft;
    l.enabled = false;
    l.physically_tested = false;
    l.tested_at = None;
    validate_source_pack(&p).unwrap();
    assert!(matches!(
        recommend_packs(&[p], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable { .. }
    ));
}
#[test]
fn history_preserves_prior_evidence_and_rejects_changed_identity() {
    let mut p = lab_pack();
    let previous = p.dataset.profile_set.profiles[0].clone();
    let metadata = p.records[0].lab.clone().unwrap();
    p.records[0].lab_history.push(LabRevision {
        profile: previous,
        metadata,
    });
    validate_source_pack(&p).unwrap();
    p.records[0].lab_history[0].profile.identity.id = "other-preset".into();
    assert!(validate_source_pack(&p).is_err());
}

#[test]
fn malformed_dates_and_unsupported_authoring_schema_are_rejected_without_panics() {
    let mut p = lab_pack();
    p.manifest.schema_version = 1;
    assert!(validate_source_pack(&p).is_err());
    for date in ["2026-02-31", "2026-13-01", "界界界x", "not-a-date"] {
        let mut p = lab_pack();
        p.records[0].lab.as_mut().unwrap().tested_at = Some(date.into());
        assert!(validate_source_pack(&p).is_err());
    }
}

#[test]
fn imported_physical_test_cannot_postdate_its_saved_revision() {
    let mut p = lab_pack();
    p.records[0].lab.as_mut().unwrap().tested_at = Some("2099-01-01".into());
    assert!(validate_source_pack(&p).unwrap_err().contains("test date"));
}

#[test]
fn sorbonne_validated_record_precedes_tested_record_without_ui_ranking() {
    let mut p = lab_pack();
    let r = request(&p);
    let mut profile = p.dataset.profile_set.profiles[0].clone();
    profile.identity.id = "sorbonne-validated-software".into();
    profile.evidence = Evidence::Validated;
    profile
        .source
        .evidence_record
        .as_mut()
        .unwrap()
        .validation_criteria_reference = Some(LAB_VALIDATION_POLICY.into());
    let mut origin = p.records[0].clone();
    origin.profile = profile.identity.clone();
    origin.source_record_id = profile.identity.id.clone();
    let lab = origin.lab.as_mut().unwrap();
    lab.state = LabState::Validated;
    lab.review = Some(LabReview {
        policy: LAB_VALIDATION_POLICY.into(),
        reviewed_at: "2026-10-12T09:00:00Z".into(),
        notes: "SOFTWARE ONLY synthetic review; no physical qualification".into(),
        acceptable_finish_and_behavior: true,
        no_overheating_chatter_or_overload: true,
        suitable_for_lab_use: true,
    });
    p.records.push(origin);
    p.dataset.profile_set.profiles.push(profile);
    let result = recommend_packs(&[p], &r).unwrap();
    let selected = result.explanation.unwrap().summary;
    assert_eq!(selected.evidence, Evidence::Validated);
    assert_eq!(selected.selected_record, "sorbonne-validated-software");
}

#[test]
fn lab_and_reference_identities_coexist_with_lab_priority_and_reference_fallback() {
    let mut lab = lab_pack();
    let mut r = request(&lab);
    let public = load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
    ))
    .unwrap();
    let machine = Identity {
        id: "lab-machine-physical-a".into(),
        revision: "1".into(),
    };
    let tool = Identity {
        id: "lab-tool-physical-a".into(),
        revision: "1".into(),
    };
    let mut machine_row = lab.dataset.machines[0].clone();
    machine_row.identity = machine.clone();
    lab.dataset.machines.push(machine_row);
    let mut tool_row = lab
        .dataset
        .tools
        .iter()
        .find(|t| Some(&t.identity) == r.tool.identity.as_ref())
        .unwrap()
        .clone();
    tool_row.identity = tool.clone();
    lab.dataset.tools.push(tool_row);
    let profile = &mut lab.dataset.profile_set.profiles[0];
    profile.scope.machine = IdentitySelector::Exact {
        identity: machine.clone(),
    };
    profile.scope.tool = Some(tool.clone());
    r.machine.local_identity = Some(machine.clone());
    r.tool.local_identity = Some(tool.clone());
    validate_source_pack(&lab).unwrap();

    for packs in [
        vec![public.clone(), lab.clone()],
        vec![lab.clone(), public.clone()],
    ] {
        let result = recommend_packs(&packs, &r).unwrap();
        let summary = result.explanation.unwrap().summary;
        assert_eq!(summary.selected_pack.id, LAB_PACK_ID);
        assert!(summary.local_only);
        let FeedsSpeedsRecommendation::Available { recommendation } = result.recommendation else {
            panic!("expected lab recommendation")
        };
        assert_eq!(
            recommendation.provenance.request.machine.identity,
            Some(machine.clone())
        );
        assert_eq!(
            recommendation.provenance.request.tool.identity,
            Some(tool.clone())
        );
    }
    // Physical entities with the same official model must not inherit this trial.
    for changed in ["machine", "tool"] {
        let mut other = r.clone();
        let identity = Identity {
            id: "different-physical-entity".into(),
            revision: "1".into(),
        };
        if changed == "machine" {
            other.machine.local_identity = Some(identity);
        } else {
            other.tool.local_identity = Some(identity);
        }
        let result = recommend_packs(&[public.clone(), lab.clone()], &other).unwrap();
        let summary = result.explanation.unwrap().summary;
        assert_eq!(summary.selected_pack, public.manifest.identity);
        assert!(!summary.local_only);
    }
    // Removing the reference leaves the local trial available.
    let mut local_only = r.clone();
    local_only.machine.identity = None;
    local_only.tool.identity = None;
    assert_eq!(
        recommend_packs(&[lab.clone()], &local_only)
            .unwrap()
            .explanation
            .unwrap()
            .summary
            .selected_pack
            .id,
        LAB_PACK_ID
    );
    // Disabling local data leaves the reference usable with both links retained.
    lab.records[0].lab.as_mut().unwrap().enabled = false;
    assert_eq!(
        recommend_packs(&[public.clone(), lab], &r)
            .unwrap()
            .explanation
            .unwrap()
            .summary
            .selected_pack,
        public.manifest.identity
    );
    // Empty physical identities are rejected even when a valid reference exists.
    r.machine.local_identity.as_mut().unwrap().id.clear();
    assert!(matches!(
        recommend_packs(&[public], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::InvalidRequest { .. }
        }
    ));
}

#[test]
fn named_lab_pack_preserves_validation_and_empty_creation_contract() {
    let mut p = lab_pack();
    p.manifest.identity.id = "lab-00000000-0000-4000-8000-000000000001".into();
    p.manifest.name = "Fablab X / Atelier ébénisterie".into();
    assert!(load_source_pack_json(&serde_json::to_string(&p).unwrap()).is_ok());
    p.manifest.visibility = Visibility::Public;
    assert!(load_source_pack_json(&serde_json::to_string(&p).unwrap()).is_err());
    p.manifest.visibility = Visibility::Private;
    p.records[0].lab.as_mut().unwrap().physically_tested = false;
    assert!(load_source_pack_json(&serde_json::to_string(&p).unwrap()).is_err());
    p.records.clear();
    p.dataset.profile_set.profiles.clear();
    assert!(load_source_pack_json(&serde_json::to_string(&p).unwrap()).is_ok());
    p.manifest.identity.revision = "not-a-monotonic-revision".into();
    assert!(load_source_pack_json(&serde_json::to_string(&p).unwrap()).is_err());
}

#[test]
fn named_lab_pack_roundtrip_keeps_priority_without_reclassifying_generic_lab_sources() {
    assert!(!is_lab_pack_id("lab-external-reference"));
    let mut p = lab_pack();
    p.manifest.identity.id = "lab-00000000-0000-4000-8000-000000000001".into();
    let p = load_source_pack_json(&serde_json::to_string(&p).unwrap()).unwrap();
    let public = load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
    ))
    .unwrap();
    let result = recommend_packs(&[public.clone(), p.clone()], &request(&p)).unwrap();
    assert_eq!(
        result.explanation.unwrap().summary.selected_pack.id,
        p.manifest.identity.id
    );
    let mut disabled = p.clone();
    disabled.records[0].lab.as_mut().unwrap().enabled = false;
    let result = recommend_packs(&[public.clone(), disabled], &request(&p)).unwrap();
    assert_eq!(
        result.explanation.unwrap().summary.selected_pack.id,
        public.manifest.identity.id
    );
}

#[test]
fn explicit_tested_validated_and_unavailable_lab_records() {
    let mut p = lab_pack();
    let selected = SelectedRecord {
        pack: p.manifest.identity.clone(),
        record_id: p.records[0].source_record_id.clone(),
        profile: p.records[0].profile.clone(),
    };
    let r = request(&p);
    assert!(matches!(
        recommend_selected_record(&[p.clone()], &r, &selected)
            .unwrap()
            .recommendation,
        FeedsSpeedsRecommendation::Available { .. }
    ));
    p.records[0].lab.as_mut().unwrap().state = LabState::Validated;
    p.dataset.profile_set.profiles[0].evidence = Evidence::Validated;
    p.dataset.profile_set.profiles[0]
        .source
        .evidence_record
        .as_mut()
        .unwrap()
        .validation_criteria_reference = Some(LAB_VALIDATION_POLICY.into());
    p.records[0].lab.as_mut().unwrap().review = Some(LabReview {
        policy: LAB_VALIDATION_POLICY.into(),
        reviewed_at: "2026-10-12T09:00:00Z".into(),
        notes: "SOFTWARE ONLY".into(),
        acceptable_finish_and_behavior: true,
        no_overheating_chatter_or_overload: true,
        suitable_for_lab_use: true,
    });
    assert!(matches!(
        recommend_selected_record(&[p.clone()], &r, &selected)
            .unwrap()
            .recommendation,
        FeedsSpeedsRecommendation::Available { .. }
    ));
    for kind in 0..3 {
        let mut unavailable = p.clone();
        let lab = unavailable.records[0].lab.as_mut().unwrap();
        match kind {
            0 => lab.enabled = false,
            1 => lab.deleted = true,
            _ => lab.state = LabState::Draft,
        }
        assert!(recommend_selected_record(&[unavailable], &r, &selected).is_err());
    }
}
