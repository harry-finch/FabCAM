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
        other => panic!("{other:?}"),
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
    assert_eq!(e.records[0].rank[1], 2);
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
