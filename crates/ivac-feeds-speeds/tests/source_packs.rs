use ivac_feeds_speeds::*;
const PACK: &str = include_str!("../../../data/feeds-speeds/packs/sienci-longmill.v1.json");
fn pack() -> SourcePack {
    load_source_pack_json(PACK).unwrap()
}
fn request(p: &SourcePack) -> FeedsSpeedsRequest {
    let profile = &p.dataset.profile_set.profiles[0];
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
            identity: profile.scope.tool.clone(),
            cutting_family: CuttingFamily::FlatEndmill,
            diameter_mm: 6.35,
            flute_count: 1,
            max_rpm: None,
        },
        operation: OperationContext {
            kind: OperationKind::FullSlot,
            doc_mm: Some(2.8),
            engagement: Engagement::KnownFullWidth,
            entry: EntryStrategy::Unknown,
        },
        aggressiveness: Aggressiveness::Prudent,
    }
}
fn available(result: KnowledgeResult) -> (Box<Recommendation>, KnowledgeExplanation) {
    match result.recommendation {
        FeedsSpeedsRecommendation::Available { recommendation } => {
            (recommendation, result.explanation.unwrap())
        }
        other @ FeedsSpeedsRecommendation::Unavailable { .. } => panic!("{other:?}"),
    }
}
fn synthetic(id: &str, feed: f64, taxon: Option<&str>) -> SourcePack {
    let mut p = pack();
    p.manifest.identity.id = id.into();
    p.manifest.distribution = Distribution::LocalOnly;
    p.manifest.name = "Synthetic software fixture — not machining data".into();
    for profile in &mut p.dataset.profile_set.profiles {
        if let CuttingData::ExactValues { feed_mm_min, .. } = &mut profile.data {
            *feed_mm_min = feed;
        }
    }
    if let Some(taxon) = taxon {
        for r in &mut p.records {
            r.material_taxon = Some(taxon.into());
        }
    }
    p
}
#[test]
fn manifest_versions_and_required_license() {
    let mut p = pack();
    assert!(public_bundle_eligible(&p));
    p.manifest.schema_version = 99;
    assert!(validate_source_pack(&p).is_err());
    p = pack();
    p.manifest.license.clear();
    assert!(validate_source_pack(&p).is_err());
    p = pack();
    p.manifest.attribution.clear();
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn classification_and_private_not_bundle() {
    let mut p = pack();
    p.manifest.distribution = Distribution::LocalOnly;
    assert!(!public_bundle_eligible(&p));
    p.manifest.distribution = Distribution::Bundled;
    p.manifest.visibility = Visibility::Private;
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn identities_and_record_origins_are_required() {
    let mut p = pack();
    p.records.pop();
    assert!(validate_source_pack(&p).is_err());
    p = pack();
    p.records.push(p.records[0].clone());
    assert!(validate_source_pack(&p).is_err());
    p = pack();
    p.manifest.identity.id = "../outside".into();
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn schema_and_size_fail_closed() {
    assert!(load_source_pack_json("{}").is_err());
    assert!(load_source_pack_json(&" ".repeat(MAX_PACK_BYTES + 1)).is_err());
    let mut v: serde_json::Value = serde_json::from_str(PACK).unwrap();
    v["executable"] = "evil".into();
    assert!(load_source_pack_json(&v.to_string()).is_err());
}
#[test]
fn source_record_and_original_values_survive() {
    let p = pack();
    let (r, e) = available(recommend_packs(&[p.clone()], &request(&p)).unwrap());
    assert_eq!(r.provenance.profile, p.dataset.profile_set.profiles[0]);
    assert_eq!(e.records[0].origin, p.records[0]);
    assert_eq!(e.summary.selected_pack, p.manifest.identity);
    assert_eq!(r.parameters.feed_mm_min, 1620);
}
#[test]
fn derived_quantities_have_units_and_keep_original() {
    let p = pack();
    let q = normalized_quantities(&p.dataset.profile_set.profiles[0]);
    assert!((q.chipload_mm_tooth.unwrap() - 1620.0 / 17000.0).abs() < 1e-12);
    assert!((q.doc_ratio.unwrap() - 2.8 / 6.35).abs() < 1e-12);
    assert!(
        (q.cutting_speed_m_min.unwrap() - std::f64::consts::PI * 6.35 * 17000.0 / 1000.0).abs()
            < 1e-12
    );
    assert_eq!(q.radial_engagement_ratio, Some(1.0));
    assert!(q.derived);
}
#[test]
fn generic_material_fallback_and_exact_preference() {
    let p = pack();
    let mut r = request(&p);
    r.material.identity = Some(taxon_identity("plastic.pmma.cast"));
    let generic = synthetic("generic", 800.0, Some("plastic"));
    let exact = synthetic("exact", 1000.0, Some("plastic.pmma.cast"));
    let (v, e) = available(recommend_packs(&[generic.clone()], &r).unwrap());
    assert_eq!(v.parameters.feed_mm_min, 800);
    assert_eq!(e.records[0].rank.material_cost, 2);
    let (v, e) = available(recommend_packs(&[generic, exact], &r).unwrap());
    assert_eq!(v.parameters.feed_mm_min, 1000);
    assert_eq!(e.summary.selected_pack.id, "exact");
}
#[test]
fn composite_sienci_is_not_all_mdf() {
    let p = pack();
    let mut r = request(&p);
    r.material.identity = Some(taxon_identity("wood.mdf"));
    assert!(matches!(
        recommend_packs(&[p], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable { .. }
    ));
}
#[test]
fn unknown_material_taxonomy_and_tool_traits_are_rejected() {
    let mut p = pack();
    p.records[0].material_taxon = Some("invented".into());
    assert!(validate_source_pack(&p).is_err());
    p = pack();
    p.records[0].tool.as_mut().unwrap().family = CuttingFamily::VBit;
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn disagreement_selects_authored_record_never_average() {
    let p = pack();
    let r = request(&p);
    let a = synthetic("a", 1000.0, None);
    let b = synthetic("b", 2000.0, None);
    let (v, e) = available(recommend_packs(&[b, a], &r).unwrap());
    assert_eq!(v.parameters.feed_mm_min, 1000);
    assert!(e.disagreement);
    assert_eq!(e.summary.compatible_sources, 2);
    assert_eq!(v.source_evidence, Evidence::ReferenceData);
}
#[test]
fn normal_and_rapid_stay_in_authored_same_mode_records() {
    for mode in [Aggressiveness::Normal, Aggressiveness::Rapid] {
        let p = pack();
        let mut r = request(&p);
        r.aggressiveness = mode;
        r.operation.doc_mm = Some(if mode == Aggressiveness::Normal {
            4.1
        } else {
            5.5
        });
        let a = synthetic("a", 1000.0, None);
        let b = synthetic("b", 2000.0, None);
        let c = synthetic("c", 3000.0, None);
        let (v, _) = available(recommend_packs(&[c, b, a], &r).unwrap());
        assert_eq!(
            v.parameters.feed_mm_min,
            if mode == Aggressiveness::Normal {
                2000
            } else {
                3000
            }
        );
    }
}
#[test]
fn noncomparable_rpm_is_ambiguous_even_when_clamps_equal() {
    let p = pack();
    let mut r = request(&p);
    r.machine.rpm_max = Some(10000);
    let a = synthetic("a", 1000.0, None);
    let mut b = synthetic("b", 2000.0, None);
    for p in &mut b.dataset.profile_set.profiles {
        if let CuttingData::ExactValues { rpm, .. } = &mut p.data {
            *rpm = 18000.0;
        }
    }
    assert!(matches!(
        recommend_packs(&[a, b], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::AmbiguousProfiles { .. }
        }
    ));
}
#[test]
fn deterministic_under_input_reordering() {
    let p = pack();
    let r = request(&p);
    let a = synthetic("a", 1000.0, None);
    let b = synthetic("b", 2000.0, None);
    let first = recommend_packs(&[a.clone(), b.clone()], &r).unwrap();
    let second = recommend_packs(&[b, a], &r).unwrap();
    assert_eq!(first.recommendation, second.recommendation);
    assert_eq!(first.explanation, second.explanation);
}
#[test]
fn duplicate_pack_ids_refused() {
    let p = pack();
    assert!(recommend_packs(&[p.clone(), p.clone()], &request(&p)).is_err());
}
#[test]
fn exact_local_tested_preset_precedes_reference() {
    let p = pack();
    let mut r = request(&p);
    r.operation.entry = EntryStrategy::Direct;
    let mut tested = synthetic("tested", 1200.0, None);
    for p in &mut tested.dataset.profile_set.profiles {
        p.evidence = Evidence::Tested;
        p.category = ProfileCategory::ExactPreset;
        p.scope.entry = Some(EntryStrategy::Direct);
        p.source.evidence_record = Some(EvidenceRecord {
            trial_reference: "synthetic test trial fixture — not physical evidence".into(),
            validation_criteria_reference: None,
        });
    }
    let (v, e) = available(recommend_packs(&[p, tested], &r).unwrap());
    assert_eq!(v.parameters.feed_mm_min, 1200);
    assert_eq!(e.summary.evidence, Evidence::Tested);
}
#[test]
fn tested_cannot_use_material_fallback() {
    let mut p = synthetic("tested", 1200.0, Some("wood"));
    p.dataset.profile_set.profiles[0].evidence = Evidence::Tested;
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn missing_and_invalid_context_keep_reasons() {
    let p = pack();
    let mut r = request(&p);
    r.tool.flute_count = 0;
    assert!(matches!(
        recommend_packs(&[p.clone()], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::InvalidRequest { .. }
        }
    ));
    r = request(&p);
    r.material.identity = None;
    assert!(matches!(
        recommend_packs(&[p], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::MissingMaterial
        }
    ));
}
#[test]
fn bundle_registry_is_explicit_and_contains_only_licensed_public_packs() {
    let registry: serde_json::Value = serde_json::from_str(include_str!(
        "../../../frontend/public/feeds-speeds/registry.v1.json"
    ))
    .unwrap();
    assert_eq!(registry["packs"].as_array().unwrap().len(), 1);
    let p = pack();
    assert_eq!(registry["packs"][0]["id"], p.manifest.identity.id);
    assert!(public_bundle_eligible(&p));
    assert!(p.manifest.attribution.contains("Sienci"));
    let mut local = p;
    local.manifest.distribution = Distribution::LocalOnly;
    assert!(!public_bundle_eligible(&local));
}

#[test]
fn source_chipload_is_distinguished_from_derived_chipload() {
    let mut p = pack().dataset.profile_set.profiles[0].clone();
    assert!(normalized_quantities(&p).chipload_derived);
    p.data = CuttingData::ChiploadTarget {
        rpm: 17000.0,
        chipload_mm_tooth: 0.08,
        plunge_mm_min: 500.0,
        doc_mm: 2.8,
        stepover_ratio: None,
    };
    let q = normalized_quantities(&p);
    assert_eq!(q.chipload_mm_tooth, Some(0.08));
    assert!(!q.chipload_derived);
    assert!(q.radial_engagement_derived);
}

#[test]
fn floating_point_roundoff_does_not_create_false_ambiguity_but_real_doc_difference_does() {
    let mut a = synthetic("roundoff-a", 1000.0, None);
    let mut b = synthetic("roundoff-b", 1000.0, None);
    for (pack, doc) in [(&mut a, 0.3), (&mut b, 0.1 + 0.2)] {
        for profile in &mut pack.dataset.profile_set.profiles {
            profile.scope.doc_mm = None;
            if let CuttingData::ExactValues { doc_mm, .. } = &mut profile.data {
                *doc_mm = doc;
            }
        }
    }
    let r = request(&a);
    available(recommend_packs(&[a.clone(), b.clone()], &r).unwrap());
    for profile in &mut b.dataset.profile_set.profiles {
        if let CuttingData::ExactValues { doc_mm, .. } = &mut profile.data {
            *doc_mm = 0.30001;
        }
    }
    assert!(matches!(
        recommend_packs(&[a, b], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable {
            reason: UnavailableReason::AmbiguousProfiles { .. }
        }
    ));
}

#[test]
fn typed_errors_preserve_legacy_api_and_rank_array_contract() {
    assert!(matches!(
        load_source_pack_json_checked("{"),
        Err(SourcePackError::InvalidJson { .. })
    ));
    let mut p = pack();
    p.dataset.tools[0].flute_count = 0;
    assert!(matches!(
        validate_source_pack_checked(&p),
        Err(SourcePackError::InvalidDataset { .. })
    ));
    let p = pack();
    let (_, e) = available(recommend_packs_checked(&[p.clone()], &request(&p)).unwrap());
    let rank = e.records[0].rank;
    let json = serde_json::to_value(rank).unwrap();
    assert_eq!(json.as_array().unwrap().len(), 12);
    assert_eq!(serde_json::from_value::<CandidateRank>(json).unwrap(), rank);
    assert!(serde_json::from_str::<CandidateRank>("[0,1]").is_err());
    let qualities = [
        MatchQuality::Exact,
        MatchQuality::Close,
        MatchQuality::Interpolated,
        MatchQuality::Approximate,
        MatchQuality::Extrapolated,
        MatchQuality::Generic,
        MatchQuality::Theoretical,
    ];
    assert!(qualities
        .windows(2)
        .all(|q| q[0].rank_cost() < q[1].rank_cost()));
}

fn selection(p: &SourcePack) -> SelectedRecord {
    SelectedRecord {
        pack: p.manifest.identity.clone(),
        record_id: p.records[0].source_record_id.clone(),
        profile: p.records[0].profile.clone(),
    }
}
#[test]
fn explicit_selection_pins_original_even_when_recommended_prefers_another() {
    let a = synthetic("a", 1000.0, None);
    let b = synthetic("b", 2000.0, None);
    let r = request(&a);
    let chosen = selection(&b);
    let packs = [a, b.clone()];
    let (_, auto) = available(recommend_packs(&packs, &r).unwrap());
    assert_eq!(auto.summary.selected_pack.id, "a");
    let (manual, info) = available(recommend_selected_record(&packs, &r, &chosen).unwrap());
    assert_eq!(manual.parameters.feed_mm_min, 2000);
    assert_eq!(info.summary.selected_pack, b.manifest.identity);
    assert_eq!(info.records.len(), 1);
    assert_eq!(info.records[0].origin, b.records[0]);
    assert!(info.records[0].derivation.is_none());
}
#[test]
fn selected_single_source_equals_recommended_and_does_not_mutate_request() {
    let p = pack();
    let r = request(&p);
    let before = serde_json::to_value(&r).unwrap();
    let auto = recommend_packs(&[p.clone()], &r).unwrap();
    let explicit = recommend_selected_record(&[p.clone()], &r, &selection(&p)).unwrap();
    assert_eq!(
        serde_json::to_value(auto.recommendation).unwrap(),
        serde_json::to_value(explicit.recommendation).unwrap()
    );
    assert_eq!(before, serde_json::to_value(r).unwrap());
}
#[test]
fn selected_unknown_revision_identity_and_ineligible_never_fall_back() {
    let p = pack();
    for dimension in 0..5 {
        let mut selected = selection(&p);
        match dimension {
            0 => selected.record_id.push_str("missing"),
            1 => selected.pack.revision.push_str("missing"),
            2 => selected.profile.revision.push_str("missing"),
            3 => selected.pack.id.push_str("missing"),
            _ => selected.profile.id.push_str("missing"),
        }
        assert!(recommend_selected_record(&[p.clone()], &request(&p), &selected).is_err());
    }
    let mut external = p.clone();
    external.manifest.distribution = Distribution::ExternalReference;
    assert!(recommend_selected_record(&[external], &request(&p), &selection(&p)).is_err());
}
#[test]
fn selected_incompatible_context_never_adapts_or_substitutes() {
    let p = pack();
    for dimension in 0..5 {
        let mut r = request(&p);
        match dimension {
            0 => r.machine.identity.as_mut().unwrap().id.push_str("missing"),
            1 => r.material.identity = Some(taxon_identity("metal.aluminium.6061")),
            2 => {
                r.tool.identity = Some(Identity {
                    id: "missing".into(),
                    revision: "1".into(),
                })
            }
            3 => r.tool.diameter_mm = 100.0,
            _ => r.tool.flute_count = 8,
        }
        let result = recommend_selected_record(&[p.clone()], &r, &selection(&p)).unwrap();
        assert!(
            matches!(
                result.recommendation,
                FeedsSpeedsRecommendation::Unavailable { .. }
            ),
            "dimension {dimension}"
        );
        assert!(result.explanation.is_none());
    }
}
