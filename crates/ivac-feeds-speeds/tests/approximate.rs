//! FS3.9 software qualification, not machining evidence.
use ivac_feeds_speeds::*;
fn pack() -> SourcePack {
    load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v3.json"
    ))
    .unwrap()
}
fn request(p: &SourcePack, d: f64) -> FeedsSpeedsRequest {
    let s = &p.dataset.profile_set.profiles[0];
    FeedsSpeedsRequest {
        schema_version: 1,
        dataset: p.dataset.profile_set.identity.clone(),
        machine: MachineSnapshot {
            identity: Some(Identity {
                id: "machine/fablab-router".into(),
                revision: "1".into(),
            }),
            rpm_min: Some(8000),
            rpm_max: Some(24000),
            max_feed_mm_min: Some(6000),
            max_plunge_mm_min: Some(2000),
        },
        material: MaterialSnapshot {
            identity: Some(taxon_identity("wood.softwood")),
            thickness_mm: Some(12.0),
        },
        tool: ToolSnapshot {
            identity: s.scope.tool.clone(),
            cutting_family: CuttingFamily::FlatEndmill,
            diameter_mm: d,
            flute_count: 1,
            max_rpm: None,
        },
        operation: OperationContext {
            kind: OperationKind::Pocket,
            doc_mm: Some(1.0),
            engagement: Engagement::Nominal { ratio: 0.5 },
            entry: EntryStrategy::Direct,
        },
        aggressiveness: Aggressiveness::Prudent,
    }
}
fn available(result: KnowledgeResult) -> (Box<Recommendation>, KnowledgeExplanation) {
    match result.recommendation {
        FeedsSpeedsRecommendation::Available { recommendation } => {
            (recommendation, result.explanation.unwrap())
        }
        r => panic!("{r:?} {:?}", result.scope_rejections),
    }
}
#[test]
fn six_mm_pocket_normalizes_near_reference_and_keeps_all_transfers_explicit() {
    let p = pack();
    let r = request(&p, 6.0);
    let (a, e) = available(recommend_packs(&[p.clone()], &r).unwrap());
    assert_eq!(a.parameters.rpm, 17000);
    assert_eq!(a.parameters.feed_mm_min, 1530);
    assert_eq!(a.parameters.plunge_mm_min, 765);
    assert_eq!(a.parameters.doc_mm, 1.0);
    assert_eq!(a.source_evidence, Evidence::ReferenceData);
    assert_eq!(e.summary.match_quality, Some(MatchQuality::Close));
    let d = e.records[0].derivation.as_ref().unwrap();
    assert!((d.relative_delta - 0.35 / 6.0).abs() < 1e-12);
    assert!(d.machine_transfer && d.material_transfer && d.slot_basis_for_pocket);
    assert_eq!(a.provenance.profile, p.dataset.profile_set.profiles[0]);
    assert_eq!(d.source_profiles[0], p.dataset.profile_set.profiles[0]);
    assert_eq!(p, pack()); // immutable supplied data
}
#[test]
fn all_six_exact_rows_are_numerically_unchanged() {
    let p = pack();
    let old = load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
    ))
    .unwrap();
    assert_eq!(
        p.dataset.profile_set.profiles,
        old.dataset.profile_set.profiles
    );
    for source in &p.dataset.profile_set.profiles {
        let mut r = request(&p, source.scope.diameter_mm.min);
        r.machine.identity = Some(p.dataset.machines[0].identity.clone());
        r.material.identity = Some(p.dataset.materials[0].identity.clone());
        r.tool.identity = source.scope.tool.clone();
        r.aggressiveness = source.aggressiveness;
        r.operation.kind = OperationKind::FullSlot;
        r.operation.engagement = Engagement::KnownFullWidth;
        r.operation.doc_mm = source.scope.doc_mm.map(|v| v.min);
        let (a, _) = available(recommend_packs(&[p.clone()], &r).unwrap());
        let (b, _) = available(recommend_packs(&[old.clone()], &r).unwrap());
        assert_eq!(a.parameters, b.parameters);
        assert_eq!(a.source_evidence, b.source_evidence);
    }
}
#[test]
fn interpolation_preserves_both_originals_and_is_deterministic() {
    let p = pack();
    let r = request(&p, 4.75);
    let (a, e) = available(recommend_packs(&[p.clone()], &r).unwrap());
    let d = e.records[0].derivation.as_ref().unwrap();
    assert_eq!(d.quality, MatchQuality::Interpolated);
    assert_eq!(d.source_diameters_mm, vec![3.175, 6.35]);
    assert_eq!(d.source_profiles.len(), 2);
    assert!((d.interpolation_weight.unwrap() - (4.75 - 3.175) / (6.35 - 3.175)).abs() < 1e-12);
    assert_eq!(a.source_evidence, Evidence::ReferenceData);
    let mut reordered = p;
    reordered.records.reverse();
    reordered.dataset.profile_set.profiles.reverse();
    let (b, f) = available(recommend_packs(&[reordered], &r).unwrap());
    assert_eq!(a, b);
    assert_eq!(e, f);
}
#[test]
fn small_diameter_extrapolation_is_bounded_and_two_vs_three175_is_refused() {
    let p = pack();
    let mut r = request(&p, 2.2);
    r.tool.identity = p.dataset.profile_set.profiles[3].scope.tool.clone();
    let (a, e) = available(recommend_packs(&[p.clone()], &r).unwrap());
    assert_eq!(e.summary.match_quality, Some(MatchQuality::Extrapolated));
    assert!(a.parameters.feed_mm_min < 1050);
    assert!(a.parameters.plunge_mm_min < 530);
    r.tool.diameter_mm = 2.0;
    let v = recommend_packs(&[p], &r).unwrap();
    assert!(matches!(
        v.recommendation,
        FeedsSpeedsRecommendation::Unavailable { .. }
    ));
    assert!(!v.scope_rejections.is_empty());
}
#[test]
fn two_from_three_has_exact_fifty_percent_gap_and_conservative_factor() {
    let mut p = pack();
    for t in &mut p.dataset.tools {
        if t.diameter_mm == 3.175 {
            t.diameter_mm = 3.0;
        }
    }
    for s in &mut p.dataset.profile_set.profiles {
        if s.scope.diameter_mm.min == 3.175 {
            s.scope.diameter_mm = Range { min: 3.0, max: 3.0 };
        }
    }
    let r = request(&p, 2.0);
    let (a, e) = available(recommend_packs(&[p], &r).unwrap());
    let d = e.records[0].derivation.as_ref().unwrap();
    assert_eq!(d.relative_delta, 0.5);
    assert_eq!(d.quality, MatchQuality::Extrapolated);
    assert!((d.chipload_factor - 4.0 / 9.0).abs() < 1e-12);
    assert!(a.parameters.feed_mm_min <= 466);
}
#[test]
fn larger_diameter_caps_cutting_speed_doc_and_machine_limits() {
    let p = pack();
    let mut r = request(&p, 8.0);
    r.machine.rpm_max = Some(12000);
    r.machine.max_feed_mm_min = Some(500);
    r.machine.max_plunge_mm_min = Some(100);
    let (a, e) = available(recommend_packs(&[p], &r).unwrap());
    assert_eq!(e.summary.match_quality, Some(MatchQuality::Approximate));
    assert!(a.parameters.rpm <= 12000);
    assert!(a.parameters.feed_mm_min <= 500);
    assert!(a.parameters.plunge_mm_min <= 100);
    assert!(a.parameters.doc_mm <= 1.0);
}
#[test]
fn incompatible_family_flutes_operation_machine_and_material_remain_rejected() {
    let p = pack();
    let base = request(&p, 6.0);
    for dimension in 0..5 {
        let mut r = base.clone();
        match dimension {
            0 => r.tool.cutting_family = CuttingFamily::Compression,
            1 => r.tool.flute_count = 2,
            2 => r.operation.kind = OperationKind::Finish,
            3 => r.machine.identity = None,
            _ => r.material.identity = Some(taxon_identity("wood.hardwood")),
        };
        assert!(
            matches!(
                recommend_packs(&[p.clone()], &r).unwrap().recommendation,
                FeedsSpeedsRecommendation::Unavailable { .. }
            ),
            "dimension {dimension}"
        );
    }
}
#[test]
fn exact_and_near_precede_farther_and_bracket_candidates() {
    for d in [6.35, 6.0, 3.175, 3.0] {
        let p = pack();
        let r = request(&p, d);
        let (_, e) = available(recommend_packs(&[p], &r).unwrap());
        assert_eq!(
            e.records[0]
                .derivation
                .as_ref()
                .unwrap()
                .source_profiles
                .len(),
            1
        );
        assert_eq!(e.summary.match_quality, Some(MatchQuality::Close));
    }
}
#[test]
fn policy_permission_and_physical_evidence_cannot_be_broadened() {
    let mut p = pack();
    p.manifest.transformation_permitted = false;
    assert!(validate_source_pack(&p).is_err());
    p = pack();
    p.dataset.profile_set.profiles[0].evidence = Evidence::Tested;
    assert!(validate_source_pack(&p).is_err());
}
#[test]
fn diagnostic_trace_retains_rejections_selection_and_order() {
    let p = pack();
    let r = request(&p, 6.0);
    let t = matching_trace(&[p.clone()], &r).unwrap();
    assert!(t.iter().any(|v| v.selected && v.accepted));
    assert!(t
        .iter()
        .any(|v| !v.accepted && v.rejected_dimensions.iter().any(|d| d.field == "mode")));
    let mut other = p;
    other.records.reverse();
    assert_eq!(t, matching_trace(&[other], &r).unwrap());
}
#[test]
fn machine_transfer_penalty_survives_diameter_adaptation() {
    let p = pack();
    let mut r = request(&p, 6.0);
    r.material.identity = Some(p.dataset.materials[0].identity.clone());
    let mut direct = p.clone();
    direct.manifest.identity.id = "direct-machine".into();
    for profile in &mut direct.dataset.profile_set.profiles {
        profile.scope.machine = IdentitySelector::Exact {
            identity: r.machine.identity.clone().unwrap(),
        };
    }
    let (_, e) = available(recommend_packs(&[p, direct], &r).unwrap());
    assert_eq!(e.summary.selected_pack.id, "direct-machine");
    assert!(!e.records[0].derivation.as_ref().unwrap().machine_transfer);
}
#[test]
fn approximate_disagreement_remains_visible_without_averaging() {
    let a = pack();
    let r = request(&a, 6.0);
    let mut b = a.clone();
    b.manifest.identity.id = "disagreement-fixture".into();
    for p in &mut b.dataset.profile_set.profiles {
        if let CuttingData::ExactValues { feed_mm_min, .. } = &mut p.data {
            *feed_mm_min *= 2.0;
        }
    }
    let (v, e) = available(recommend_packs(&[a, b], &r).unwrap());
    assert_eq!(v.parameters.feed_mm_min, 1530);
    assert!(e.disagreement);
    assert_eq!(e.summary.compatible_sources, 2);
}
#[test]
fn declared_parent_fallback_uses_taxonomy_graph_and_cannot_accept_invented_prefix() {
    let mut p = pack();
    for origin in &mut p.records {
        origin.material_taxon = Some("wood".into());
    }
    let mut r = request(&p, 6.0);
    r.material.identity = Some(taxon_identity("wood.plywood.birch"));
    let (_, e) = available(recommend_packs(&[p.clone()], &r).unwrap());
    assert_eq!(e.records[0].rank[1], 2);
    r.material.identity = Some(taxon_identity("wood.plywood.invented"));
    assert!(matches!(
        recommend_packs(&[p], &r).unwrap().recommendation,
        FeedsSpeedsRecommendation::Unavailable { .. }
    ));
}
#[test]
fn multiple_pack_interpolations_do_not_hide_disagreement() {
    let a = pack();
    let r = request(&a, 5.0);
    let mut b = a.clone();
    b.manifest.identity.id = "interpolation-disagreement".into();
    for p in &mut b.dataset.profile_set.profiles {
        if let CuttingData::ExactValues { feed_mm_min, .. } = &mut p.data {
            *feed_mm_min *= 2.0;
        }
    }
    let (_, e) = available(recommend_packs(&[a, b], &r).unwrap());
    assert_eq!(e.summary.match_quality, Some(MatchQuality::Interpolated));
    assert!(e.disagreement);
    assert_eq!(e.summary.compatible_sources, 2);
    assert_eq!(e.summary.contributors.len(), 4);
}
#[test]
fn no_match_reports_distance_instead_of_already_permitted_transfers() {
    let p = pack();
    let r = request(&p, 2.0);
    let v = recommend_packs(&[p], &r).unwrap();
    assert!(matches!(
        v.recommendation,
        FeedsSpeedsRecommendation::Unavailable { .. }
    ));
    let nearest = &v.scope_rejections[0];
    assert!(nearest
        .mismatches
        .iter()
        .any(|m| m.field == "diameter_proximity" && m.actual.contains("58.75")));
    assert!(!nearest.mismatches.iter().any(|m| matches!(
        m.field.as_str(),
        "machine" | "material" | "tool" | "operation" | "engagement"
    )));
}
#[test]
fn ambiguous_interpolation_anchor_preserves_disagreement_and_determinism() {
    let mut p = pack();
    let mut extra = p.dataset.profile_set.profiles[0].clone();
    extra.identity.id.push_str("-duplicate-anchor");
    if let CuttingData::ExactValues { feed_mm_min, .. } = &mut extra.data {
        *feed_mm_min *= 2.0;
    }
    let mut origin = p.records[0].clone();
    origin.profile = extra.identity.clone();
    origin.source_record_id = extra.identity.id.clone();
    p.records.push(origin);
    p.dataset.profile_set.profiles.push(extra);
    let r = request(&p, 5.0);
    let a = recommend_packs(&[p.clone()], &r).unwrap();
    assert!(a.explanation.as_ref().unwrap().disagreement);
    assert_ne!(
        a.explanation.as_ref().unwrap().summary.match_quality,
        Some(MatchQuality::Interpolated)
    );
    p.records.reverse();
    p.dataset.profile_set.profiles.reverse();
    let b = recommend_packs(&[p], &r).unwrap();
    assert_eq!(a.recommendation, b.recommendation);
    assert_eq!(a.explanation, b.explanation);
}
#[test]
fn router_class_cannot_become_exact_physical_trial_scope() {
    let mut p = pack();
    for origin in &mut p.records {
        origin.adaptation = None;
    }
    for profile in &mut p.dataset.profile_set.profiles {
        profile.category = ProfileCategory::ExactPreset;
        profile.evidence = Evidence::Tested;
        profile.source.evidence_record = Some(EvidenceRecord {
            trial_reference: "SOFTWARE ONLY synthetic attestation".into(),
            validation_criteria_reference: None,
        });
        profile.scope.entry = Some(EntryStrategy::Direct);
        profile.scope.machine = IdentitySelector::Exact {
            identity: Identity {
                id: "machine/fablab-router".into(),
                revision: "1".into(),
            },
        };
    }
    assert!(validate_source_pack(&p)
        .unwrap_err()
        .contains("individual machine"));
}
