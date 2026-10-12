use crate::{matching, validation, *};

/// Validated immutable supplied data. Deserialization alone does not validate data;
/// construction always passes through [`Engine::new`].
#[derive(Debug, Clone)]
pub struct Engine {
    profiles: ProfileSet,
}

impl Engine {
    pub fn new(profiles: ProfileSet) -> Result<Self, Vec<ValidationIssue>> {
        let issues = validation::set_issues(&profiles);
        if issues.is_empty() {
            Ok(Self { profiles })
        } else {
            Err(issues)
        }
    }

    /// Pure proposal: neither request nor dataset is mutated or externally read.
    pub fn recommend(&self, request: &FeedsSpeedsRequest) -> FeedsSpeedsRecommendation {
        match self.propose(request) {
            Ok(recommendation) => FeedsSpeedsRecommendation::Available {
                recommendation: Box::new(recommendation),
            },
            Err(reason) => FeedsSpeedsRecommendation::Unavailable { reason },
        }
    }

    /// Closest refused scopes for explanation only. This does not select a
    /// recommendation, change precedence or relax any applicability criterion.
    pub fn scope_rejections(&self, r: &FeedsSpeedsRequest) -> Vec<ProfileRejection> {
        let mut rows: Vec<_> = self
            .profiles
            .profiles
            .iter()
            .map(|p| ProfileRejection {
                profile: p.identity.clone(),
                mismatches: matching::mismatches(p, r, &self.profiles),
            })
            .collect();
        let Some(minimum) = rows.iter().map(|p| p.mismatches.len()).min() else {
            return vec![];
        };
        if minimum == 0 {
            return vec![];
        }
        rows.retain(|p| p.mismatches.len() == minimum);
        rows.sort_by(|a, b| a.profile.cmp(&b.profile));
        rows.truncate(3);
        rows
    }

    fn propose(&self, r: &FeedsSpeedsRequest) -> Result<Recommendation, UnavailableReason> {
        let issues = validation::request_issues(r);
        if !issues.is_empty() {
            return Err(UnavailableReason::InvalidRequest { issues });
        }
        if r.dataset != self.profiles.identity {
            return Err(UnavailableReason::DatasetMismatch);
        }
        if r.tool.cutting_family != CuttingFamily::FlatEndmill {
            return Err(UnavailableReason::UnsupportedTool);
        }
        if !matches!(
            r.operation.kind,
            OperationKind::Profile
                | OperationKind::Pocket
                | OperationKind::FullSlot
                | OperationKind::Finish
        ) {
            return Err(UnavailableReason::UnsupportedOperation);
        }
        if r.material.identity.is_none() {
            return Err(UnavailableReason::MissingMaterial);
        }
        let candidates: Vec<_> = self
            .profiles
            .profiles
            .iter()
            .filter(|p| matching::matches(p, r, &self.profiles))
            .collect();
        let best_tier = candidates
            .iter()
            .map(|p| matching::tier(p))
            .min()
            .ok_or(UnavailableReason::MissingCuttingData)?;
        let top: Vec<_> = candidates
            .into_iter()
            .filter(|p| matching::tier(p) == best_tier)
            .collect();
        let maximal: Vec<_> = top
            .iter()
            .copied()
            .filter(|p| {
                !top.iter()
                    .any(|other| matching::strictly_narrower(other, p, &self.profiles))
            })
            .collect();
        let profile = if maximal.len() == 1 {
            maximal[0]
        } else {
            let mut identities: Vec<_> = maximal.iter().map(|p| p.identity.clone()).collect();
            identities.sort();
            return Err(UnavailableReason::AmbiguousProfiles { identities });
        };
        calculate(r, profile, best_tier, &self.profiles)
    }
}

fn change(
    adjustments: &mut Vec<Adjustment>,
    field: &str,
    reason: &str,
    before: f64,
    after: f64,
    rule: Option<&Identity>,
) -> f64 {
    if before != after {
        adjustments.push(Adjustment {
            field: field.into(),
            reason: reason.into(),
            before,
            after,
            rule: rule.cloned(),
        });
    }
    after
}

fn ceiling(
    value: f64,
    limit: Option<u32>,
    field: &str,
    reason: &str,
    a: &mut Vec<Adjustment>,
) -> f64 {
    match limit {
        Some(limit) => change(a, field, reason, value, value.min(f64::from(limit)), None),
        None => value,
    }
}

fn integer(value: f64, field: &str) -> Result<u32, UnavailableReason> {
    if !value.is_finite() || value < 1.0 || value > f64::from(u32::MAX) {
        return Err(UnavailableReason::UnrepresentableOutput {
            field: field.into(),
        });
    }
    Ok(value as u32)
}

fn calculate(
    r: &FeedsSpeedsRequest,
    p: &Profile,
    tier: u8,
    set: &ProfileSet,
) -> Result<Recommendation, UnavailableReason> {
    let mut adjustments = Vec::new();
    let mut diagnostics = Vec::new();
    for (field, bound) in [
        ("machine.rpm_min", r.machine.rpm_min),
        ("machine.rpm_max", r.machine.rpm_max),
        ("machine.max_feed_mm_min", r.machine.max_feed_mm_min),
        ("machine.max_plunge_mm_min", r.machine.max_plunge_mm_min),
        ("tool.max_rpm", r.tool.max_rpm),
    ] {
        if bound.is_none() {
            diagnostics.push(Diagnostic::UnknownLimit {
                field: field.into(),
            });
        }
    }
    match r.operation.engagement {
        Engagement::Unknown => diagnostics.push(Diagnostic::UnknownEngagementFullWidthAssumption),
        Engagement::Nominal { .. } => diagnostics.push(Diagnostic::NominalEngagementNotMaximum),
        Engagement::KnownFullWidth => {}
    }
    if r.operation.entry == EntryStrategy::Unknown {
        diagnostics.push(Diagnostic::UnknownEntryStrategy);
    }
    if r.operation.doc_mm.is_none() {
        diagnostics.push(Diagnostic::UnknownDoc);
    }
    let (target_rpm, mut target_rate, mut plunge, mut doc, mut stepover) = match p.data {
        CuttingData::ChiploadTarget {
            rpm,
            chipload_mm_tooth,
            plunge_mm_min,
            doc_mm,
            stepover_ratio,
        } => (
            rpm,
            chipload_mm_tooth,
            plunge_mm_min,
            doc_mm,
            stepover_ratio,
        ),
        CuttingData::ExactValues {
            rpm,
            feed_mm_min,
            plunge_mm_min,
            doc_mm,
            stepover_ratio,
        } => (rpm, feed_mm_min, plunge_mm_min, doc_mm, stepover_ratio),
    };
    for rule in &p.reductions {
        target_rate = change(
            &mut adjustments,
            "chipload_mm_tooth",
            "profile_reduction",
            target_rate,
            target_rate * rule.chipload_factor,
            Some(&rule.identity),
        );
        plunge = change(
            &mut adjustments,
            "plunge_mm_min",
            "profile_reduction",
            plunge,
            plunge * rule.plunge_factor,
            Some(&rule.identity),
        );
        doc = change(
            &mut adjustments,
            "doc_mm",
            "profile_reduction",
            doc,
            doc * rule.doc_factor,
            Some(&rule.identity),
        );
    }
    if let Some(request_doc) = r.operation.doc_mm {
        doc = change(
            &mut adjustments,
            "doc_mm",
            "request_doc_ceiling",
            doc,
            doc.min(request_doc),
            None,
        );
    }
    if let (Some(target), Engagement::Nominal { ratio }) = (stepover, r.operation.engagement) {
        stepover = Some(change(
            &mut adjustments,
            "stepover_ratio",
            "request_nominal_ceiling",
            target,
            target.min(ratio),
            None,
        ));
    }
    // Positive half-ties round upward for RPM. Hard integer bounds follow rounding.
    let mut rpm = change(
        &mut adjustments,
        "rpm",
        "round_nearest_half_up",
        target_rpm,
        target_rpm.round(),
        None,
    );
    rpm = ceiling(
        rpm,
        r.machine.rpm_max,
        "rpm",
        "machine_rpm_max",
        &mut adjustments,
    );
    rpm = ceiling(rpm, r.tool.max_rpm, "rpm", "tool_rpm_max", &mut adjustments);
    if let Some(min) = r.machine.rpm_min {
        rpm = change(
            &mut adjustments,
            "rpm",
            "machine_rpm_min",
            rpm,
            rpm.max(f64::from(min)),
            None,
        );
    }
    let rpm = integer(rpm, "rpm")?;
    // Target data keeps chipload at the final RPM. Exact data preserves its feed.
    let mut feed = match p.data {
        CuttingData::ChiploadTarget { .. } => {
            f64::from(rpm) * f64::from(r.tool.flute_count) * target_rate
        }
        CuttingData::ExactValues { .. } => target_rate,
    };
    // Detect overflow before a clamp can hide a nonfinite intermediate.
    for (field, value) in [
        ("feed_mm_min", feed),
        ("plunge_mm_min", plunge),
        ("doc_mm", doc),
        ("chipload_or_feed", target_rate),
    ] {
        if !value.is_finite() || value <= 0.0 {
            return Err(UnavailableReason::UnrepresentableOutput {
                field: field.into(),
            });
        }
    }
    feed = ceiling(
        feed,
        r.machine.max_feed_mm_min,
        "feed_mm_min",
        "machine_feed_max",
        &mut adjustments,
    );
    plunge = ceiling(
        plunge,
        r.machine.max_plunge_mm_min,
        "plunge_mm_min",
        "machine_plunge_max",
        &mut adjustments,
    );
    // Floor feed/plunge to avoid increasing the supplied targets at application precision.
    feed = change(
        &mut adjustments,
        "feed_mm_min",
        "round_down",
        feed,
        feed.floor(),
        None,
    );
    plunge = change(
        &mut adjustments,
        "plunge_mm_min",
        "round_down",
        plunge,
        plunge.floor(),
        None,
    );
    let feed = integer(feed, "feed_mm_min")?;
    let plunge = integer(plunge, "plunge_mm_min")?;
    let chipload = f64::from(feed) / (f64::from(rpm) * f64::from(r.tool.flute_count));
    let applicability = if adjustments.is_empty() {
        Applicability::WithinDeclaredProfile
    } else {
        Applicability::AdjustedOutsideEvidence
    };
    Ok(Recommendation {
        parameters: RecommendedParameters {
            rpm,
            feed_mm_min: feed,
            plunge_mm_min: plunge,
            doc_mm: doc,
            stepover_ratio: stepover,
            effective_chipload_mm_tooth: chipload,
        },
        aggressiveness: r.aggressiveness,
        source_evidence: p.evidence,
        applicability,
        diagnostics,
        provenance: Provenance {
            engine_version: ENGINE_VERSION.into(),
            schema_version: DOMAIN_SCHEMA_VERSION,
            dataset: r.dataset.clone(),
            profile: p.clone(),
            machine_family: family_snapshot(&p.scope.machine, &set.machine_families),
            material_family: family_snapshot(&p.scope.material, &set.material_families),
            request: r.clone(),
            selection_tier: tier,
            adjustments,
        },
    })
}

fn family_snapshot(
    selector: &IdentitySelector,
    families: &[FamilyMembership],
) -> Option<FamilyMembership> {
    let IdentitySelector::Family { id } = selector else {
        return None;
    };
    let mut family = families.iter().find(|f| &f.id == id)?.clone();
    family.members.sort();
    Some(family)
}
