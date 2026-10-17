//! Synthetic software fixtures only: none of these numbers are material cutting advice.
use ivac_feeds_speeds::*;

fn id(name: &str) -> Identity {
    Identity {
        id: format!("synthetic:{name}"),
        revision: "1".into(),
    }
}

fn exact(identity: Identity) -> IdentitySelector {
    IdentitySelector::Exact { identity }
}

fn range(min: f64, max: f64) -> Range {
    Range { min, max }
}

fn request() -> FeedsSpeedsRequest {
    FeedsSpeedsRequest {
        schema_version: DOMAIN_SCHEMA_VERSION,
        dataset: id("dataset"),
        machine: MachineSnapshot {
            identity: Some(id("machine")),
            rpm_min: Some(100),
            rpm_max: Some(3000),
            max_feed_mm_min: Some(5000),
            max_plunge_mm_min: Some(1000),
        },
        material: MaterialSnapshot {
            identity: Some(id("plywood-software-fixture")),
            thickness_mm: Some(18.0),
        },
        tool: ToolSnapshot {
            identity: Some(id("tool")),
            cutting_family: CuttingFamily::FlatEndmill,
            diameter_mm: 6.0,
            flute_count: 2,
            max_rpm: Some(4000),
        },
        operation: OperationContext {
            kind: OperationKind::Profile,
            doc_mm: Some(2.0),
            engagement: Engagement::Unknown,
            entry: EntryStrategy::Direct,
        },
        aggressiveness: Aggressiveness::Prudent,
    }
}

fn profile(name: &str) -> Profile {
    Profile {
        identity: id(name),
        category: ProfileCategory::Reference,
        evidence: Evidence::ReferenceData,
        source: Source {
            reference: "synthetic arithmetic fixture; no physical source".into(),
            author: "ivaCAM software tests".into(),
            date: "2026-10-12".into(),
            permission_or_license: "original synthetic test data".into(),
            notes: "NOT cutting recommendations".into(),
            evidence_record: None,
        },
        aggressiveness: Aggressiveness::Prudent,
        scope: ProfileScope {
            machine: IdentitySelector::Any,
            material: exact(id("plywood-software-fixture")),
            tool: None,
            cutting_family: CuttingFamily::FlatEndmill,
            diameter_mm: range(3.0, 12.0),
            flute_count: None,
            operation: OperationKind::Profile,
            doc_mm: None,
            engagement: EngagementScope::UnknownSafe,
            entry: None,
            thickness_mm: None,
        },
        data: CuttingData::ChiploadTarget {
            rpm: 1000.0,
            chipload_mm_tooth: 0.1,
            plunge_mm_min: 80.0,
            doc_mm: 2.0,
            stepover_ratio: None,
        },
        reductions: vec![],
    }
}

fn set(profiles: Vec<Profile>) -> ProfileSet {
    ProfileSet {
        schema_version: DOMAIN_SCHEMA_VERSION,
        identity: id("dataset"),
        machine_families: vec![],
        material_families: vec![],
        profiles,
    }
}

fn engine(profiles: Vec<Profile>) -> Engine {
    Engine::new(set(profiles)).unwrap()
}

fn available(result: FeedsSpeedsRecommendation) -> Recommendation {
    match result {
        FeedsSpeedsRecommendation::Available { recommendation } => *recommendation,
        other => panic!("expected available: {other:?}"),
    }
}

fn unavailable(result: FeedsSpeedsRecommendation) -> UnavailableReason {
    match result {
        FeedsSpeedsRecommendation::Unavailable { reason } => reason,
        other => panic!("expected unavailable: {other:?}"),
    }
}

fn full_slot(mut p: Profile) -> Profile {
    p.scope.operation = OperationKind::FullSlot;
    p.scope.engagement = EngagementScope::FullWidth;
    p
}

fn pocket(mut p: Profile) -> Profile {
    p.scope.operation = OperationKind::Pocket;
    p.scope.engagement = EngagementScope::Nominal {
        ratio: range(0.2, 0.6),
    };
    if let CuttingData::ChiploadTarget { stepover_ratio, .. } = &mut p.data {
        *stepover_ratio = Some(0.4);
    }
    p
}

fn local_preset(name: &str, evidence: Evidence) -> Profile {
    let mut p = full_slot(profile(name));
    p.category = ProfileCategory::ExactPreset;
    p.evidence = evidence;
    p.scope.machine = exact(id("machine"));
    p.scope.tool = Some(id("tool"));
    p.scope.flute_count = Some(2);
    p.scope.diameter_mm = range(6.0, 6.0);
    p.scope.doc_mm = Some(range(2.0, 2.0));
    p.scope.entry = Some(EntryStrategy::Direct);
    p.scope.thickness_mm = Some(range(18.0, 18.0));
    p.data = CuttingData::ExactValues {
        rpm: 1000.0,
        feed_mm_min: 200.0,
        plunge_mm_min: 80.0,
        doc_mm: 2.0,
        stepover_ratio: None,
    };
    p.source.evidence_record = Some(EvidenceRecord {
        trial_reference: "synthetic test of a declared label; NO physical trial".into(),
        validation_criteria_reference: Some(
            "synthetic criteria reference; NOT physical qualification".into(),
        ),
    });
    p
}

#[test]
fn fs01_equation_units_and_profile_unknown_engagement() {
    let r = request();
    let result = available(engine(vec![profile("reference")]).recommend(&r));
    let v = &result.parameters;
    assert_eq!((v.rpm, v.feed_mm_min, v.plunge_mm_min), (1000, 200, 80));
    assert_eq!(v.doc_mm, 2.0);
    assert_eq!(v.stepover_ratio, None);
    assert_eq!(v.effective_chipload_mm_tooth, 0.1);
    assert_eq!(
        f64::from(v.feed_mm_min),
        f64::from(v.rpm) * 2.0 * v.effective_chipload_mm_tooth
    );
    assert!(result
        .diagnostics
        .contains(&Diagnostic::UnknownEngagementFullWidthAssumption));
    assert_eq!(result.provenance.request, r);
    assert_eq!(result.provenance.engine_version, ENGINE_VERSION);
}

#[test]
fn fs02_slot_cannot_use_profile_or_low_radial_data() {
    let mut r = request();
    r.operation.kind = OperationKind::FullSlot;
    r.operation.engagement = Engagement::KnownFullWidth;
    assert_eq!(
        unavailable(engine(vec![profile("profile"), pocket(profile("pocket"))]).recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
    let mut p = full_slot(profile("slot"));
    p.reductions.push(ReductionRule {
        identity: id("authored-slot-reduction"),
        source_reference: "synthetic rule".into(),
        chipload_factor: 0.5,
        plunge_factor: 0.75,
        doc_factor: 0.5,
    });
    let result = available(engine(vec![p]).recommend(&r));
    assert_eq!(
        (
            result.parameters.feed_mm_min,
            result.parameters.plunge_mm_min,
            result.parameters.doc_mm
        ),
        (100, 60, 1.0)
    );
    assert_eq!(result.provenance.adjustments.len(), 3);
    assert!(result
        .provenance
        .adjustments
        .iter()
        .all(|a| a.rule == Some(id("authored-slot-reduction"))));
}

#[test]
fn fs02_full_slot_contradiction_is_invalid() {
    for engagement in [
        Engagement::Unknown,
        Engagement::Nominal { ratio: 0.4 },
        Engagement::Nominal { ratio: 1.0 },
    ] {
        let mut r = request();
        r.operation.kind = OperationKind::FullSlot;
        r.operation.engagement = engagement;
        assert!(matches!(
            unavailable(engine(vec![]).recommend(&r)),
            UnavailableReason::InvalidRequest { .. }
        ));
    }
}

#[test]
fn fs03_pocket_preserves_nominal_ratio_and_distinction() {
    let mut r = request();
    r.operation.kind = OperationKind::Pocket;
    r.operation.engagement = Engagement::Nominal { ratio: 0.4 };
    let result = available(engine(vec![pocket(profile("pocket"))]).recommend(&r));
    assert_eq!(result.parameters.stepover_ratio, Some(0.4));
    assert!(result
        .diagnostics
        .contains(&Diagnostic::NominalEngagementNotMaximum));
    assert!(!result
        .diagnostics
        .contains(&Diagnostic::UnknownEngagementFullWidthAssumption));
    r.operation.engagement = Engagement::Unknown;
    assert_eq!(
        unavailable(engine(vec![pocket(profile("pocket"))]).recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
}

#[test]
fn fs04_rpm_clamps_and_chipload_target_uses_final_rpm() {
    let mut r = request();
    r.machine.rpm_max = Some(700);
    let result = available(engine(vec![profile("reference")]).recommend(&r));
    assert_eq!(
        (result.parameters.rpm, result.parameters.feed_mm_min),
        (700, 140)
    );
    assert_eq!(result.parameters.effective_chipload_mm_tooth, 0.1);
    assert!(result
        .provenance
        .adjustments
        .iter()
        .any(|a| a.reason == "machine_rpm_max" && a.before == 1000.0 && a.after == 700.0));
    r.machine.rpm_max = Some(3000);
    r.machine.rpm_min = Some(1500);
    let result = available(engine(vec![profile("reference")]).recommend(&r));
    assert_eq!(
        (result.parameters.rpm, result.parameters.feed_mm_min),
        (1500, 300)
    );
    r.tool.max_rpm = Some(1800);
    r.machine.rpm_min = Some(100);
    r.machine.rpm_max = Some(4000);
    let mut p = profile("reference");
    if let CuttingData::ChiploadTarget { rpm, .. } = &mut p.data {
        *rpm = 2500.0;
    }
    let result = available(engine(vec![p]).recommend(&r));
    assert_eq!(result.parameters.rpm, 1800);
    assert!(result
        .provenance
        .adjustments
        .iter()
        .any(|a| a.reason == "tool_rpm_max"));
}

#[test]
fn fs04_inconsistent_machine_or_tool_limits_refuse() {
    for tool_max in [Some(50), Some(4000)] {
        let mut r = request();
        r.tool.max_rpm = tool_max;
        r.machine.rpm_max = Some(50);
        assert!(matches!(
            unavailable(engine(vec![]).recommend(&r)),
            UnavailableReason::InvalidRequest { .. }
        ));
    }
    let mut r = request();
    r.tool.max_rpm = Some(50);
    assert!(matches!(
        unavailable(engine(vec![]).recommend(&r)),
        UnavailableReason::InvalidRequest { .. }
    ));
}

#[test]
fn fs05_feed_and_plunge_limits_are_independent() {
    let mut r = request();
    r.machine.max_feed_mm_min = Some(125);
    r.machine.max_plunge_mm_min = Some(30);
    let result = available(engine(vec![profile("reference")]).recommend(&r));
    assert_eq!(
        (
            result.parameters.feed_mm_min,
            result.parameters.plunge_mm_min
        ),
        (125, 30)
    );
    assert_eq!(result.parameters.effective_chipload_mm_tooth, 0.0625);
    assert_eq!(result.applicability, Applicability::AdjustedOutsideEvidence);
    assert_eq!(
        result
            .provenance
            .adjustments
            .iter()
            .map(|a| a.reason.as_str())
            .collect::<Vec<_>>(),
        ["machine_feed_max", "machine_plunge_max"]
    );
}

#[test]
fn fs05_unknown_limits_are_not_invented() {
    let mut r = request();
    r.machine.rpm_min = None;
    r.machine.rpm_max = None;
    r.machine.max_feed_mm_min = None;
    r.machine.max_plunge_mm_min = None;
    r.tool.max_rpm = None;
    let result = available(engine(vec![profile("reference")]).recommend(&r));
    assert_eq!(
        result
            .diagnostics
            .iter()
            .filter(|d| matches!(d, Diagnostic::UnknownLimit { .. }))
            .count(),
        5
    );
    assert_eq!(
        (result.parameters.rpm, result.parameters.feed_mm_min),
        (1000, 200)
    );
    assert_eq!(result.provenance.request, r);
}

#[test]
fn fs06_unknown_material_missing_profiles_and_dataset_mismatch() {
    let e = engine(vec![profile("reference")]);
    let mut r = request();
    r.material.identity = None;
    assert_eq!(
        unavailable(e.recommend(&r)),
        UnavailableReason::MissingMaterial
    );
    r.material.identity = Some(id("unknown-grade"));
    assert_eq!(
        unavailable(e.recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
    assert_eq!(
        unavailable(engine(vec![]).recommend(&request())),
        UnavailableReason::MissingCuttingData
    );
    r = request();
    r.dataset.revision = "2".into();
    assert_eq!(
        unavailable(e.recommend(&r)),
        UnavailableReason::DatasetMismatch
    );
}

#[test]
fn fs07_unsupported_families_and_operations() {
    let e = engine(vec![profile("reference")]);
    for family in [
        CuttingFamily::Compression,
        CuttingFamily::VBit,
        CuttingFamily::Other,
    ] {
        let mut r = request();
        r.tool.cutting_family = family;
        assert_eq!(
            unavailable(e.recommend(&r)),
            UnavailableReason::UnsupportedTool
        );
    }
    for kind in [OperationKind::Surfacing, OperationKind::Engraving] {
        let mut r = request();
        r.operation.kind = kind;
        assert_eq!(
            unavailable(e.recommend(&r)),
            UnavailableReason::UnsupportedOperation
        );
    }
    // Non-milling process context is structurally excluded, not interpreted as milling.
    let mut json = serde_json::to_value(request()).unwrap();
    json["process"] = "waterjet".into();
    assert!(serde_json::from_value::<FeedsSpeedsRequest>(json).is_err());
}

#[test]
fn fs08_exact_validated_then_tested_then_reference_precedence() {
    let mut r = request();
    r.operation.kind = OperationKind::FullSlot;
    r.operation.engagement = Engagement::KnownFullWidth;
    let validated = local_preset("validated", Evidence::Validated);
    let tested = local_preset("tested", Evidence::Tested);
    let reference = full_slot(profile("reference"));
    let result =
        available(engine(vec![reference.clone(), tested.clone(), validated.clone()]).recommend(&r));
    assert_eq!(result.provenance.profile.identity, validated.identity);
    assert_eq!(result.provenance.selection_tier, 0);
    assert_eq!(result.source_evidence, Evidence::Validated);
    assert_eq!(result.applicability, Applicability::WithinDeclaredProfile);
    let result = available(engine(vec![reference, tested.clone()]).recommend(&r));
    assert_eq!(result.provenance.profile.identity, tested.identity);
    assert_eq!(result.provenance.selection_tier, 1);
}

#[test]
fn fs08_exact_evidence_never_broadens_to_changed_context() {
    let p = local_preset("validated", Evidence::Validated);
    let mut base = request();
    base.operation.kind = OperationKind::FullSlot;
    base.operation.engagement = Engagement::KnownFullWidth;
    for n in 0..9 {
        let mut r = base.clone();
        match n {
            0 => r.machine.identity = Some(id("other-machine")),
            1 => r.material.identity = Some(id("other-grade")),
            2 => r.tool.identity = Some(id("other-tool")),
            3 => r.tool.flute_count = 3,
            4 => r.tool.diameter_mm = 6.1,
            5 => r.operation.doc_mm = Some(2.1),
            6 => r.operation.entry = EntryStrategy::Ramp,
            7 => r.material.thickness_mm = None,
            _ => r.machine.identity = None,
        }
        assert_eq!(
            unavailable(engine(vec![p.clone()]).recommend(&r)),
            UnavailableReason::MissingCuttingData
        );
    }
}

#[test]
fn fs08_exact_values_keep_chipload_after_rpm_clamp_and_downgrade_applicability() {
    let mut r = request();
    r.operation.kind = OperationKind::FullSlot;
    r.operation.engagement = Engagement::KnownFullWidth;
    r.machine.rpm_max = Some(500);
    let result =
        available(engine(vec![local_preset("validated", Evidence::Validated)]).recommend(&r));
    assert_eq!(
        (result.parameters.rpm, result.parameters.feed_mm_min),
        (500, 100)
    );
    assert_eq!(result.parameters.effective_chipload_mm_tooth, 0.1);
    assert_eq!(result.source_evidence, Evidence::Validated);
    assert_eq!(result.applicability, Applicability::AdjustedOutsideEvidence);
}

#[test]
fn fs09_equal_or_incomparable_scopes_are_ambiguous_in_any_order() {
    let a = profile("a");
    let b = profile("b");
    let result = engine(vec![a.clone(), b.clone()]).recommend(&request());
    assert_eq!(
        result,
        engine(vec![b.clone(), a.clone()]).recommend(&request())
    );
    assert_eq!(
        unavailable(result),
        UnavailableReason::AmbiguousProfiles {
            identities: vec![id("a"), id("b")]
        }
    );
    let mut a = a;
    let mut b = b;
    a.scope.diameter_mm = range(6.0, 6.0);
    b.scope.flute_count = Some(2);
    assert!(matches!(
        unavailable(engine(vec![a.clone(), b.clone()]).recommend(&request())),
        UnavailableReason::AmbiguousProfiles { .. }
    ));
    assert_eq!(
        engine(vec![a.clone(), b.clone()]).recommend(&request()),
        engine(vec![b, a]).recommend(&request())
    );
}

#[test]
fn fs09_strict_containment_selects_unique_maximal_scope() {
    let broad = profile("broad");
    let mut narrow = profile("narrow");
    narrow.scope.diameter_mm = range(6.0, 6.0);
    let mut narrowest = narrow.clone();
    narrowest.identity = id("narrowest");
    narrowest.scope.flute_count = Some(2);
    let result = available(engine(vec![narrow, broad, narrowest.clone()]).recommend(&request()));
    assert_eq!(result.provenance.profile.identity, narrowest.identity);
}

#[test]
fn fs09_ambiguity_in_higher_tier_does_not_fall_through() {
    let mut a = profile("a");
    a.category = ProfileCategory::MachineFamily;
    a.scope.machine = exact(id("machine"));
    let mut b = a.clone();
    b.identity = id("b");
    let result = unavailable(engine(vec![profile("reference"), a, b]).recommend(&request()));
    assert_eq!(
        result,
        UnavailableReason::AmbiguousProfiles {
            identities: vec![id("a"), id("b")]
        }
    );
}

#[test]
fn fs09_all_precedence_tiers_and_mode_are_explicit() {
    let mut theoretical = profile("theory");
    theoretical.category = ProfileCategory::Theoretical;
    theoretical.evidence = Evidence::Theoretical;
    let reference = profile("reference");
    let mut machine = profile("machine");
    machine.category = ProfileCategory::MachineFamily;
    machine.scope.machine = exact(id("machine"));
    assert_eq!(
        available(
            engine(vec![
                theoretical.clone(),
                reference.clone(),
                machine.clone()
            ])
            .recommend(&request())
        )
        .provenance
        .selection_tier,
        2
    );
    assert_eq!(
        available(engine(vec![theoretical.clone(), reference]).recommend(&request()))
            .provenance
            .selection_tier,
        3
    );
    assert_eq!(
        available(engine(vec![theoretical]).recommend(&request()))
            .provenance
            .selection_tier,
        4
    );
    let mut r = request();
    r.aggressiveness = Aggressiveness::Rapid;
    assert_eq!(
        unavailable(engine(vec![machine]).recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
}

#[test]
fn fs09_inclusive_range_boundaries_and_missing_qualifiers() {
    let mut p = profile("reference");
    p.scope.doc_mm = Some(range(1.0, 2.0));
    p.scope.thickness_mm = Some(range(17.0, 18.0));
    let e = engine(vec![p]);
    for (diameter, doc, thickness) in [(3.0, 1.0, 17.0), (12.0, 2.0, 18.0)] {
        let mut r = request();
        r.tool.diameter_mm = diameter;
        r.operation.doc_mm = Some(doc);
        r.material.thickness_mm = Some(thickness);
        available(e.recommend(&r));
    }
    for n in 0..4 {
        let mut r = request();
        match n {
            0 => r.tool.diameter_mm = 12.0001,
            1 => r.operation.doc_mm = None,
            2 => r.material.thickness_mm = None,
            _ => r.operation.doc_mm = Some(0.9999),
        }
        assert_eq!(
            unavailable(e.recommend(&r)),
            UnavailableReason::MissingCuttingData
        );
    }
}

#[test]
fn fs09_families_require_explicit_membership_and_specificity() {
    let mut broad = profile("family");
    broad.scope.material = IdentitySelector::Family {
        id: "wood-fixture-family".into(),
    };
    let narrow = profile("exact-material");
    let mut data = set(vec![broad.clone(), narrow.clone()]);
    data.material_families.push(FamilyMembership {
        id: "wood-fixture-family".into(),
        members: vec![id("plywood-software-fixture"), id("second-material")],
    });
    let e = Engine::new(data.clone()).unwrap();
    assert_eq!(
        available(e.recommend(&request()))
            .provenance
            .profile
            .identity,
        narrow.identity
    );
    let mut r = request();
    r.material.identity = Some(id("second-material"));
    assert_eq!(
        available(e.recommend(&r)).provenance.profile.identity,
        broad.identity
    );
    r.material.identity = Some(id("wood-fixture-family"));
    assert_eq!(
        unavailable(e.recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
    data.material_families.clear();
    assert!(Engine::new(data).is_err());
}

#[test]
fn fs09_family_subset_containment_and_machine_membership() {
    let mut broad = profile("large-family");
    broad.scope.machine = IdentitySelector::Family { id: "large".into() };
    let mut narrow = broad.clone();
    narrow.identity = id("small-family");
    narrow.scope.machine = IdentitySelector::Family { id: "small".into() };
    let mut data = set(vec![broad, narrow.clone()]);
    data.machine_families = vec![
        FamilyMembership {
            id: "large".into(),
            members: vec![id("machine"), id("second-machine")],
        },
        FamilyMembership {
            id: "small".into(),
            members: vec![id("machine")],
        },
    ];
    let e = Engine::new(data).unwrap();
    assert_eq!(
        available(e.recommend(&request()))
            .provenance
            .profile
            .identity,
        narrow.identity
    );
    let mut r = request();
    r.machine.identity = None;
    assert_eq!(
        unavailable(e.recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
}

#[test]
fn fs13_request_profile_set_and_result_round_trip() {
    let data = set(vec![profile("reference")]);
    let r = request();
    let result = Engine::new(data.clone()).unwrap().recommend(&r);
    assert_eq!(
        serde_json::from_str::<ProfileSet>(&serde_json::to_string(&data).unwrap()).unwrap(),
        data
    );
    assert_eq!(
        serde_json::from_str::<FeedsSpeedsRequest>(&serde_json::to_string(&r).unwrap()).unwrap(),
        r
    );
    assert_eq!(
        serde_json::from_str::<FeedsSpeedsRecommendation>(&serde_json::to_string(&result).unwrap())
            .unwrap(),
        result
    );
    let unavailable_result = engine(vec![]).recommend(&r);
    assert_eq!(
        serde_json::from_str::<FeedsSpeedsRecommendation>(
            &serde_json::to_string(&unavailable_result).unwrap()
        )
        .unwrap(),
        unavailable_result
    );
}

#[test]
fn fs13_dataset_validation_rejects_invalid_claims_and_duplicates() {
    for n in 0..13 {
        let mut data = set(vec![profile("reference")]);
        match n {
            0 => data.schema_version += 1,
            1 => data.identity.id.clear(),
            2 => data.profiles.push(data.profiles[0].clone()),
            3 => data.profiles[0].scope.diameter_mm = range(7.0, 6.0),
            4 => data.profiles[0].evidence = Evidence::Validated,
            5 => data.profiles[0].source.permission_or_license.clear(),
            6 => data.profiles[0].scope.material = IdentitySelector::Any,
            7 => data.profiles = vec![local_preset("local", Evidence::Validated)],
            8 => data.profiles[0].scope.cutting_family = CuttingFamily::Compression,
            9 => data.profiles[0].scope.flute_count = Some(0),
            10 => data.profiles[0].scope.operation = OperationKind::Surfacing,
            11 => data.profiles[0].scope.doc_mm = Some(range(3.0, 4.0)),
            _ => data.profiles[0].identity.revision.clear(),
        }
        if n == 7 {
            data.profiles[0].source.evidence_record = None;
        }
        assert!(Engine::new(data).is_err(), "case {n}");
    }
}

#[test]
fn fs13_validation_label_requires_scope_and_record() {
    for n in 0..7 {
        let mut p = local_preset("local", Evidence::Validated);
        match n {
            0 => p.scope.machine = IdentitySelector::Any,
            1 => p.scope.tool = None,
            2 => p.scope.doc_mm = None,
            3 => p.scope.engagement = EngagementScope::UnknownSafe,
            4 => p.scope.entry = Some(EntryStrategy::Unknown),
            5 => {
                p.source
                    .evidence_record
                    .as_mut()
                    .unwrap()
                    .validation_criteria_reference = None
            }
            _ => p.scope.flute_count = None,
        }
        assert!(Engine::new(set(vec![p])).is_err(), "case {n}");
    }
}

#[test]
fn fs13_unknown_fields_and_non_milling_operations_reject_on_deserialize() {
    let mut json = serde_json::to_value(set(vec![profile("reference")])).unwrap();
    json["production_dataset"] = true.into();
    assert!(serde_json::from_value::<ProfileSet>(json).is_err());
    let mut json = serde_json::to_value(request()).unwrap();
    json["operation"]["kind"] = "waterjet".into();
    assert!(serde_json::from_value::<FeedsSpeedsRequest>(json).is_err());
}

#[test]
fn fs15_invalid_request_quantities_and_limits() {
    let e = engine(vec![profile("reference")]);
    for n in 0..13 {
        let mut r = request();
        match n {
            0 => r.tool.flute_count = 0,
            1 => r.tool.diameter_mm = 0.0,
            2 => r.tool.diameter_mm = f64::NAN,
            3 => r.operation.doc_mm = Some(f64::INFINITY),
            4 => r.material.thickness_mm = Some(-1.0),
            5 => r.machine.rpm_max = Some(0),
            6 => r.machine.max_feed_mm_min = Some(0),
            7 => r.machine.max_plunge_mm_min = Some(0),
            8 => r.tool.max_rpm = Some(0),
            9 => r.operation.engagement = Engagement::Nominal { ratio: 0.0 },
            10 => r.operation.engagement = Engagement::Nominal { ratio: 1.1 },
            11 => r.schema_version += 1,
            _ => r.dataset.revision.clear(),
        }
        assert!(
            matches!(
                unavailable(e.recommend(&r)),
                UnavailableReason::InvalidRequest { .. }
            ),
            "case {n}"
        );
    }
}

#[test]
fn fs15_nonfinite_and_bad_profile_data_reject_at_construction() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -1.0] {
        for n in 0..4 {
            let mut p = profile("reference");
            if let CuttingData::ChiploadTarget {
                rpm,
                chipload_mm_tooth,
                plunge_mm_min,
                doc_mm,
                ..
            } = &mut p.data
            {
                match n {
                    0 => *rpm = bad,
                    1 => *chipload_mm_tooth = bad,
                    2 => *plunge_mm_min = bad,
                    _ => *doc_mm = bad,
                }
            }
            assert!(Engine::new(set(vec![p])).is_err());
        }
    }
}

#[test]
fn fs15_rounding_and_representability_are_explicit() {
    let mut p = profile("precision");
    p.data = CuttingData::ChiploadTarget {
        rpm: 1000.5,
        chipload_mm_tooth: 0.1,
        plunge_mm_min: 80.9,
        doc_mm: 2.0,
        stepover_ratio: None,
    };
    let result = available(engine(vec![p]).recommend(&request()));
    assert_eq!(
        (
            result.parameters.rpm,
            result.parameters.feed_mm_min,
            result.parameters.plunge_mm_min
        ),
        (1001, 200, 80)
    );
    assert_eq!(
        result.parameters.effective_chipload_mm_tooth,
        200.0 / (1001.0 * 2.0)
    );
    assert_eq!(
        result
            .provenance
            .adjustments
            .iter()
            .map(|a| a.reason.as_str())
            .collect::<Vec<_>>(),
        ["round_nearest_half_up", "round_down", "round_down"]
    );
    let mut p = profile("too-small");
    if let CuttingData::ChiploadTarget {
        chipload_mm_tooth, ..
    } = &mut p.data
    {
        *chipload_mm_tooth = 0.00001;
    }
    assert_eq!(
        unavailable(engine(vec![p]).recommend(&request())),
        UnavailableReason::UnrepresentableOutput {
            field: "feed_mm_min".into()
        }
    );
}

#[test]
fn fs15_overflow_and_unbounded_large_output_refuse() {
    let mut r = request();
    r.machine.max_feed_mm_min = None;
    for value in [f64::MAX, f64::from(u32::MAX)] {
        let mut p = profile("large");
        if let CuttingData::ChiploadTarget {
            chipload_mm_tooth, ..
        } = &mut p.data
        {
            *chipload_mm_tooth = value;
        }
        assert!(matches!(
            unavailable(engine(vec![p]).recommend(&r)),
            UnavailableReason::UnrepresentableOutput { .. }
        ));
    }
    let mut p = profile("large-rpm");
    if let CuttingData::ChiploadTarget { rpm, .. } = &mut p.data {
        *rpm = f64::from(u32::MAX) + 1.0;
    }
    r.machine.rpm_max = None;
    r.tool.max_rpm = None;
    assert_eq!(
        unavailable(engine(vec![p]).recommend(&r)),
        UnavailableReason::UnrepresentableOutput {
            field: "rpm".into()
        }
    );
}

#[test]
fn fs15_repeated_output_and_input_immutability() {
    let p = profile("reference");
    let data = set(vec![p]);
    let snapshot = data.clone();
    let r = request();
    let request_snapshot = r.clone();
    let e = Engine::new(data.clone()).unwrap();
    let result = e.recommend(&r);
    for _ in 0..20 {
        assert_eq!(e.recommend(&r), result);
    }
    assert_eq!(r, request_snapshot);
    assert_eq!(data, snapshot);
}

#[test]
fn fs15_limit_invariants_over_deterministic_grid() {
    let e = engine(vec![profile("reference")]);
    for rpm_min in [100, 1100] {
        for rpm_max in [1200, 2000] {
            for feed_max in [1, 53, 500] {
                for plunge_max in [1, 37, 100] {
                    let mut r = request();
                    r.machine.rpm_min = Some(rpm_min);
                    r.machine.rpm_max = Some(rpm_max);
                    r.machine.max_feed_mm_min = Some(feed_max);
                    r.machine.max_plunge_mm_min = Some(plunge_max);
                    let v = available(e.recommend(&r)).parameters;
                    assert!(v.rpm >= rpm_min && v.rpm <= rpm_max);
                    assert!(v.feed_mm_min > 0 && v.feed_mm_min <= feed_max);
                    assert!(v.plunge_mm_min > 0 && v.plunge_mm_min <= plunge_max);
                    assert!(
                        v.effective_chipload_mm_tooth.is_finite()
                            && v.effective_chipload_mm_tooth > 0.0
                    );
                    assert_eq!(
                        f64::from(v.feed_mm_min) / (f64::from(v.rpm) * 2.0),
                        v.effective_chipload_mm_tooth
                    );
                }
            }
        }
    }
}

#[test]
fn fs16_aggressiveness_and_evidence_independent_no_hidden_mode_factor() {
    for mode in [
        Aggressiveness::Prudent,
        Aggressiveness::Normal,
        Aggressiveness::Rapid,
    ] {
        let mut p = profile("reference");
        p.aggressiveness = mode;
        let mut r = request();
        r.aggressiveness = mode;
        let result = available(engine(vec![p]).recommend(&r));
        assert_eq!(result.aggressiveness, mode);
        assert_eq!(result.source_evidence, Evidence::ReferenceData);
        assert_eq!(result.parameters.feed_mm_min, 200);
    }
}

#[test]
fn fs16_reduction_expansion_missing_source_and_duplicates_refuse() {
    for n in 0..5 {
        let mut p = profile("reference");
        let mut rule = ReductionRule {
            identity: id("rule"),
            source_reference: "synthetic".into(),
            chipload_factor: 0.5,
            plunge_factor: 1.0,
            doc_factor: 1.0,
        };
        match n {
            0 => rule.chipload_factor = 1.1,
            1 => rule.doc_factor = 0.0,
            2 => rule.plunge_factor = f64::NAN,
            3 => rule.source_reference.clear(),
            _ => {}
        }
        p.reductions.push(rule.clone());
        if n == 4 {
            p.reductions.push(rule);
        }
        assert!(Engine::new(set(vec![p])).is_err());
    }
}

#[test]
fn finish_context_requires_finish_data_and_unknowns_are_visible() {
    let mut r = request();
    r.operation.kind = OperationKind::Finish;
    assert_eq!(
        unavailable(engine(vec![profile("rough")]).recommend(&r)),
        UnavailableReason::MissingCuttingData
    );
    let mut finish = profile("finish");
    finish.scope.operation = OperationKind::Finish;
    r.operation.doc_mm = None;
    r.operation.entry = EntryStrategy::Unknown;
    let result = available(engine(vec![finish]).recommend(&r));
    assert!(result.diagnostics.contains(&Diagnostic::UnknownDoc));
    assert!(result
        .diagnostics
        .contains(&Diagnostic::UnknownEntryStrategy));
}

#[test]
fn request_doc_and_nominal_stepover_cannot_be_silently_increased() {
    let mut r = request();
    r.operation.kind = OperationKind::Pocket;
    r.operation.doc_mm = Some(1.0);
    r.operation.engagement = Engagement::Nominal { ratio: 0.2 };
    let result = available(engine(vec![pocket(profile("pocket"))]).recommend(&r));
    assert_eq!(
        (result.parameters.doc_mm, result.parameters.stepover_ratio),
        (1.0, Some(0.2))
    );
    assert_eq!(result.applicability, Applicability::AdjustedOutsideEvidence);
}

#[test]
fn equivalent_single_member_family_and_exact_scope_are_ambiguous() {
    let mut exact_profile = profile("exact");
    exact_profile.scope.machine = exact(id("machine"));
    let mut family_profile = exact_profile.clone();
    family_profile.identity = id("family");
    family_profile.scope.machine = IdentitySelector::Family {
        id: "singleton".into(),
    };
    let mut data = set(vec![exact_profile, family_profile]);
    data.machine_families.push(FamilyMembership {
        id: "singleton".into(),
        members: vec![id("machine")],
    });
    assert_eq!(
        unavailable(Engine::new(data).unwrap().recommend(&request())),
        UnavailableReason::AmbiguousProfiles {
            identities: vec![id("exact"), id("family")]
        }
    );
}

#[test]
fn nominal_scope_containment_selects_narrower_recipe_at_boundaries() {
    let broad = pocket(profile("broad"));
    let mut narrow = pocket(profile("narrow"));
    narrow.scope.engagement = EngagementScope::Nominal {
        ratio: range(0.3, 0.5),
    };
    let e = engine(vec![broad.clone(), narrow.clone()]);
    let mut r = request();
    r.operation.kind = OperationKind::Pocket;
    for ratio in [0.3, 0.4, 0.5] {
        r.operation.engagement = Engagement::Nominal { ratio };
        assert_eq!(
            available(e.recommend(&r)).provenance.profile.identity,
            narrow.identity
        );
    }
    for ratio in [0.2, 0.6] {
        r.operation.engagement = Engagement::Nominal { ratio };
        assert_eq!(
            available(e.recommend(&r)).provenance.profile.identity,
            broad.identity
        );
    }
}

#[test]
fn family_and_rule_authoring_errors_cannot_bypass_constructor_validation() {
    for n in 0..6 {
        let mut data = set(vec![profile("reference")]);
        let family = FamilyMembership {
            id: "family".into(),
            members: vec![id("machine")],
        };
        match n {
            0 => {
                let mut family = family;
                family.members.clear();
                data.machine_families.push(family);
            }
            1 => {
                data.machine_families.push(family.clone());
                data.machine_families.push(family);
            }
            2 => {
                let mut family = family;
                family.members.push(id("machine"));
                data.machine_families.push(family);
            }
            3 => {
                data.profiles[0].scope.machine = IdentitySelector::Family {
                    id: "unresolved".into(),
                }
            }
            4 => {
                data.profiles[0] = pocket(profile("pocket"));
                if let CuttingData::ChiploadTarget { stepover_ratio, .. } =
                    &mut data.profiles[0].data
                {
                    *stepover_ratio = Some(0.9);
                }
            }
            _ => {
                data.profiles[0] = local_preset("local", Evidence::Tested);
                data.profiles[0].reductions.push(ReductionRule {
                    identity: id("rule"),
                    source_reference: "synthetic".into(),
                    chipload_factor: 0.5,
                    plunge_factor: 1.0,
                    doc_factor: 1.0,
                });
            }
        }
        assert!(Engine::new(data).is_err(), "case {n}");
    }
}

#[test]
fn family_provenance_survives_external_dataset_loss_and_member_reordering() {
    let mut p = profile("family");
    p.scope.material = IdentitySelector::Family {
        id: "material-family".into(),
    };
    let mut data = set(vec![p]);
    data.material_families.push(FamilyMembership {
        id: "material-family".into(),
        members: vec![id("second-material"), id("plywood-software-fixture")],
    });
    let before = Engine::new(data.clone()).unwrap().recommend(&request());
    data.material_families[0].members.reverse();
    assert_eq!(before, Engine::new(data).unwrap().recommend(&request()));
    let decoded: FeedsSpeedsRecommendation =
        serde_json::from_str(&serde_json::to_string(&before).unwrap()).unwrap();
    let recommendation = available(decoded);
    assert_eq!(
        recommendation.provenance.material_family.unwrap().members,
        vec![id("plywood-software-fixture"), id("second-material")]
    );
    assert!(recommendation.provenance.machine_family.is_none());
}

#[test]
fn exact_values_rpm_cap_never_increases_implicit_chipload() {
    let mut p = profile("rpm-coupling");
    p.data = CuttingData::ExactValues {
        rpm: 24000.0,
        feed_mm_min: 2400.0,
        plunge_mm_min: 600.0,
        doc_mm: 1.0,
        stepover_ratio: None,
    };
    let mut r = request();
    r.machine.rpm_max = Some(12000);
    r.tool.max_rpm = None;
    let a = available(engine(vec![p]).recommend(&r));
    assert_eq!(a.parameters.rpm, 12000);
    assert_eq!(a.parameters.feed_mm_min, 1200);
    assert!(a.parameters.effective_chipload_mm_tooth <= 0.05);
    assert_eq!(a.applicability, Applicability::AdjustedOutsideEvidence);
}

#[test]
fn exact_feed_tracks_tool_cap_and_rounding_but_never_increases_at_rpm_floor() {
    let mut p = profile("exact-rpm");
    p.data = CuttingData::ExactValues {
        rpm: 1000.4,
        feed_mm_min: 1000.0,
        plunge_mm_min: 50.0,
        doc_mm: 1.0,
        stepover_ratio: None,
    };
    let mut r = request();
    r.tool.max_rpm = Some(500);
    let a = available(engine(vec![p.clone()]).recommend(&r));
    assert_eq!(a.parameters.feed_mm_min, 499);
    assert_eq!(a.parameters.plunge_mm_min, 24);
    assert!(a.parameters.effective_chipload_mm_tooth <= 1000.0 / (1000.4 * 2.0));
    assert!(a
        .provenance
        .adjustments
        .iter()
        .any(|a| a.reason == "rpm_reduction_preserves_chipload"));
    r.tool.max_rpm = None;
    r.machine.rpm_min = Some(2000);
    let a = available(engine(vec![p]).recommend(&r));
    assert_eq!(a.parameters.feed_mm_min, 1000);
}

#[test]
fn feed_saturation_reports_target_comparison_without_material_threshold_or_rpm_retuning() {
    let mut r = request();
    r.machine.max_feed_mm_min = Some(10);
    let a = available(engine(vec![profile("limited")]).recommend(&r));
    assert_eq!(a.parameters.rpm, 1000);
    assert!(a.diagnostics.contains(&Diagnostic::LowEffectiveChipload {
        chipload_mm_tooth: 0.005,
        target_chipload_mm_tooth: 0.1,
    }));
    assert!(a
        .diagnostics
        .contains(&Diagnostic::GenericProfileDirectEntry));
    let mut explicit = profile("direct");
    explicit.scope.entry = Some(EntryStrategy::Direct);
    let a = available(engine(vec![explicit]).recommend(&r));
    assert!(!a
        .diagnostics
        .contains(&Diagnostic::GenericProfileDirectEntry));
    r.operation.entry = EntryStrategy::Ramp;
    r.machine.max_feed_mm_min = None;
    let a = available(engine(vec![profile("ramp")]).recommend(&r));
    assert!(!a
        .diagnostics
        .contains(&Diagnostic::GenericProfileDirectEntry));
    assert!(!a
        .diagnostics
        .iter()
        .any(|d| matches!(d, Diagnostic::LowEffectiveChipload { .. })));
}

#[test]
fn plunge_scales_down_for_both_data_modes_and_never_up() {
    for exact_mode in [false, true] {
        let mut p = profile("plunge-rpm");
        p.data = if exact_mode {
            CuttingData::ExactValues {
                rpm: 24000.0,
                feed_mm_min: 2400.0,
                plunge_mm_min: 600.0,
                doc_mm: 1.0,
                stepover_ratio: None,
            }
        } else {
            CuttingData::ChiploadTarget {
                rpm: 24000.0,
                chipload_mm_tooth: 0.05,
                plunge_mm_min: 600.0,
                doc_mm: 1.0,
                stepover_ratio: None,
            }
        };
        let mut r = request();
        r.machine.rpm_max = Some(12000);
        r.tool.max_rpm = None;
        let a = available(engine(vec![p.clone()]).recommend(&r));
        assert_eq!(a.parameters.plunge_mm_min, 300);
        assert!(a
            .provenance
            .adjustments
            .iter()
            .any(|x| x.reason == "rpm_reduction_preserves_plunge_per_revolution"));
        r.machine.rpm_max = None;
        r.tool.max_rpm = Some(12000);
        assert_eq!(
            available(engine(vec![p.clone()]).recommend(&r))
                .parameters
                .plunge_mm_min,
            300
        );
        r.tool.max_rpm = None;
        r.machine.rpm_min = Some(30000);
        assert_eq!(
            available(engine(vec![p]).recommend(&r))
                .parameters
                .plunge_mm_min,
            600
        );
    }
}

#[test]
fn exact_rpm_floor_reports_original_chipload_even_with_feed_cap() {
    let mut p = profile("floor-diagnostic");
    p.data = CuttingData::ExactValues {
        rpm: 8000.0,
        feed_mm_min: 1600.0,
        plunge_mm_min: 600.0,
        doc_mm: 1.0,
        stepover_ratio: None,
    };
    let mut r = request();
    r.machine.rpm_min = Some(10000);
    r.machine.rpm_max = None;
    r.tool.max_rpm = None;
    for cap in [None, Some(1000)] {
        r.machine.max_feed_mm_min = cap;
        let a = available(engine(vec![p.clone()]).recommend(&r));
        assert_eq!(a.parameters.feed_mm_min, cap.unwrap_or(1600));
        assert!(a.diagnostics.contains(&Diagnostic::LowEffectiveChipload {
            chipload_mm_tooth: f64::from(cap.unwrap_or(1600)) / 20000.0,
            target_chipload_mm_tooth: 0.1,
        }));
    }
}
