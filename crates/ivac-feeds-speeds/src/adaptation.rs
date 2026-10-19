//! FS3.9 bounded adaptations. Policy thresholds are eligibility rules, not physical limits.
use crate::{
    normalized_quantities, CuttingData, Engagement, EngagementScope, Engine, Evidence,
    FeedsSpeedsRecommendation, FeedsSpeedsRequest, Identity, IdentitySelector, KnowledgeRecord,
    OperationKind, Profile, ProfileCategory, ProfileSet, Range, Recommendation, RecordOrigin,
    ScopeMismatch, SourcePack, SourcePackError, ToolClassification, KNOWLEDGE_POLICY,
};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsAdaptationScope"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptationScope {
    pub policy: String,
    pub material_taxa: Vec<String>,
    pub machines: Vec<Identity>,
    /// Explicit interchangeable physical classifications, never inferred from labels.
    pub tools: Vec<Identity>,
    pub slot_to_pocket: bool,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsMatchQuality"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchQuality {
    Close,
    Approximate,
    Extrapolated,
    Interpolated,
    Exact,
    Generic,
    Theoretical,
}
impl MatchQuality {
    /// Explicit policy ordering, independent of enum declaration / serialized names.
    #[must_use]
    pub const fn rank_cost(self) -> u32 {
        match self {
            Self::Exact => 0,
            Self::Close => 1,
            Self::Interpolated => 2,
            Self::Approximate => 3,
            Self::Extrapolated => 4,
            Self::Generic => 5,
            Self::Theoretical => 6,
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsDerivation"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Derivation {
    pub policy: String,
    pub quality: MatchQuality,
    pub method: String,
    pub requested_diameter_mm: f64,
    pub source_diameters_mm: Vec<f64>,
    pub source_profiles: Vec<Profile>,
    pub relative_delta: f64,
    pub interpolation_weight: Option<f64>,
    pub chipload_factor: f64,
    pub doc_factor: f64,
    pub machine_transfer: bool,
    pub material_transfer: bool,
    pub slot_basis_for_pocket: bool,
}

pub(crate) fn validate_adaptation(
    pack: &SourcePack,
    p: &Profile,
    r: &RecordOrigin,
    classifications: &std::collections::BTreeMap<&Identity, &ToolClassification>,
) -> Result<(), String> {
    let Some(a) = &r.adaptation else {
        return Ok(());
    };
    if pack.manifest.schema_version < 3
        || !pack.manifest.transformation_permitted
        || p.evidence != Evidence::ReferenceData
        || p.category != ProfileCategory::Reference
        || a.policy != KNOWLEDGE_POLICY
        || r.lab.is_some()
        || a.material_taxa
            .iter()
            .any(|id| !crate::source_pack::known_taxon(id))
    {
        return Err(
            "Adaptation requires schema 3, permitted transformation and ReferenceData policy scope"
                .into(),
        );
    }
    for id in &a.machines {
        if !pack.dataset.machines.iter().any(|m| &m.identity == id) {
            return Err("Adaptation machine must be an explicit catalog identity".into());
        }
    }
    let original = p
        .scope
        .tool
        .as_ref()
        .and_then(|id| classifications.get(id))
        .copied();
    for id in &a.tools {
        let tool = pack
            .dataset
            .tools
            .iter()
            .find(|t| &t.identity == id)
            .ok_or("Adaptation tool absent from catalog")?;
        let classification = classifications.get(id).copied();
        if classification.is_none()
            || classification != original
            || tool.flute_count != p.scope.flute_count.unwrap_or(0)
        {
            return Err("Adaptation tools require identical confirmed geometry/helix/coating/material and flutes".into());
        }
    }
    Ok(())
}

// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
pub(crate) fn adapt_profile(
    pack: &SourcePack,
    origin: &RecordOrigin,
    source: &Profile,
    normalized: &Profile,
    r: &FeedsSpeedsRequest,
    set: &ProfileSet,
) -> Option<(Profile, Derivation)> {
    // Tested/Validated and exact presets are never generalized. Source ranges lack an anchor.
    if source.category != ProfileCategory::Reference
        || source.evidence != Evidence::ReferenceData
        || !pack.manifest.transformation_permitted
        || !source.reductions.is_empty()
        || source.scope.diameter_mm.min != source.scope.diameter_mm.max
        || source.scope.flute_count != Some(r.tool.flute_count)
    {
        return None;
    }
    let d = source.scope.diameter_mm.min;
    let delta = (d - r.tool.diameter_mm).abs() / r.tool.diameter_mm;
    if delta > 0.5 {
        return None;
    }
    let mut p = normalized.clone();
    let mut machine_transfer = false;
    let mut material_transfer = normalized.scope.material != source.scope.material;
    let mut slot_basis = false;
    let mismatches = crate::matching::mismatches(&p, r, set);
    for mismatch in mismatches {
        match mismatch.field.as_str() {
            "diameter_mm" => {
                p.scope.diameter_mm = Range {
                    min: r.tool.diameter_mm,
                    max: r.tool.diameter_mm,
                }
            }
            "doc_mm" => p.scope.doc_mm = None, // target DOC stays a ceiling below
            "machine"
                if origin.adaptation.as_ref().is_some_and(|a| {
                    r.machine
                        .identity
                        .as_ref()
                        .is_some_and(|id| a.machines.contains(id))
                }) =>
            {
                p.scope.machine = IdentitySelector::Exact {
                    identity: r.machine.identity.clone()?,
                };
                machine_transfer = true;
            }
            "material"
                if origin.adaptation.as_ref().is_some_and(|a| {
                    r.material.identity.as_ref().is_some_and(|id| {
                        a.material_taxa
                            .iter()
                            .any(|t| crate::source_pack::taxon_matches(t, id))
                    })
                }) =>
            {
                p.scope.material = IdentitySelector::Exact {
                    identity: r.material.identity.clone()?,
                };
                material_transfer = true;
            }
            "tool"
                if origin.adaptation.as_ref().is_some_and(|a| {
                    r.tool
                        .identity
                        .as_ref()
                        .is_some_and(|id| a.tools.contains(id))
                }) =>
            {
                p.scope.tool.clone_from(&r.tool.identity);
            }
            "operation" | "engagement"
                if source.scope.operation == OperationKind::FullSlot
                    && source.scope.engagement == EngagementScope::FullWidth
                    && r.operation.kind == OperationKind::Pocket
                    && matches!(r.operation.engagement, Engagement::Nominal { ratio } if ratio > 0.0 && ratio <= 1.0)
                    && origin.adaptation.as_ref().is_some_and(|a| a.slot_to_pocket) =>
            {
                p.scope.operation = OperationKind::Pocket;
                p.scope.engagement = EngagementScope::Nominal {
                    ratio: Range {
                        min: 0.000_001,
                        max: 1.0,
                    },
                };
                slot_basis = true;
            }
            _ => return None,
        }
    }
    let n = normalized_quantities(source);
    let ratio = (r.tool.diameter_mm / d).min(1.0);
    // Continuous bounded policy transition, not a physical calibration model.
    let transition = ((delta - 0.1) / 0.4).clamp(0.0, 1.0);
    let factor = ratio.powf(1.0 + transition);
    let (rpm, plunge, original_doc) = match source.data {
        CuttingData::ExactValues {
            rpm,
            plunge_mm_min,
            doc_mm,
            ..
        }
        | CuttingData::ChiploadTarget {
            rpm,
            plunge_mm_min,
            doc_mm,
            ..
        } => (rpm, plunge_mm_min, doc_mm),
    };
    // Keep source RPM for smaller cutters; cap larger cutter RPM to source cutting speed.
    let rpm = rpm * (d / r.tool.diameter_mm).min(1.0);
    let doc = (n.doc_ratio? * r.tool.diameter_mm * ratio.powf(transition)).min(original_doc);
    p.data = CuttingData::ChiploadTarget {
        rpm,
        chipload_mm_tooth: n.chipload_mm_tooth? * factor,
        plunge_mm_min: plunge * factor,
        doc_mm: doc,
        // No radial chip-thinning uplift or claim of maximum instantaneous engagement.
        stepover_ratio: if slot_basis {
            match r.operation.engagement {
                Engagement::Nominal { ratio } => Some(ratio),
                _ => None,
            }
        } else {
            match source.data {
                CuttingData::ExactValues { stepover_ratio, .. }
                | CuttingData::ChiploadTarget { stepover_ratio, .. } => stepover_ratio,
            }
        },
    };
    Some((
        p,
        Derivation {
            policy: KNOWLEDGE_POLICY.into(),
            quality: if delta <= 0.1 {
                MatchQuality::Close
            } else if delta <= 0.25 {
                MatchQuality::Approximate
            } else {
                MatchQuality::Extrapolated
            },
            method: "bounded_normalized_reference".into(),
            requested_diameter_mm: r.tool.diameter_mm,
            source_diameters_mm: vec![d],
            source_profiles: vec![source.clone()],
            relative_delta: delta,
            interpolation_weight: None,
            chipload_factor: factor,
            doc_factor: doc / original_doc,
            machine_transfer,
            material_transfer,
            slot_basis_for_pocket: slot_basis,
        },
    ))
}

// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
pub(crate) fn add_interpolations(
    candidates: &mut Vec<(KnowledgeRecord, Recommendation)>,
    request: &FeedsSpeedsRequest,
) {
    // Exact/near anchors outrank interpolation. Interpolate only within one authored series.
    let mut pairs = Vec::new();
    for (a, ra) in candidates.iter() {
        for (b, _) in candidates.iter() {
            let (Some(da), Some(db)) = (&a.derivation, &b.derivation) else {
                continue;
            };
            let x = da.source_diameters_mm[0];
            let y = db.source_diameters_mm[0];
            let d = request.tool.diameter_mm;
            // Ambiguous anchor values stay in ordinary candidate reconciliation.
            // Never pick one upper/lower row according to source iteration order.
            let unique_anchor = |diameter: f64| {
                candidates
                    .iter()
                    .filter(|(c, _)| {
                        c.pack == a.pack
                            && c.derivation
                                .as_ref()
                                .is_some_and(|d| d.source_diameters_mm[0] == diameter)
                    })
                    .count()
                    == 1
            };
            if !unique_anchor(x) || !unique_anchor(y) {
                continue;
            }

            let pa = &da.source_profiles[0];
            let pb = &db.source_profiles[0];
            let mut sa = pa.scope.clone();
            let mut sb = pb.scope.clone();
            sa.diameter_mm = Range { min: 1.0, max: 1.0 };
            sb.diameter_mm = sa.diameter_mm;
            // DOC is normalized; tool identity may vary only within declared series.
            sa.doc_mm = None;
            sb.doc_mm = None;
            if a.origin.adaptation.as_ref().is_some_and(|s| {
                pa.scope
                    .tool
                    .as_ref()
                    .is_some_and(|id| s.tools.contains(id))
                    && pb
                        .scope
                        .tool
                        .as_ref()
                        .is_some_and(|id| s.tools.contains(id))
            }) {
                sa.tool = None;
                sb.tool = None;
            }
            if !(x < d && d < y && y / x <= 2.0)
                || a.pack != b.pack
                || a.evidence != b.evidence
                || sa != sb
                || pa.aggressiveness != pb.aggressiveness
                || a.origin.tool != b.origin.tool
                || pa.reductions != pb.reductions
                || a.origin.adaptation != b.origin.adaptation
                || !a.rank.same_context(&b.rank)
            {
                continue;
            }
            let t = (d - x) / (y - x);
            let lerp = |x: f64, y: f64| x + (y - x) * t;
            let na = normalized_quantities(pa);
            let nb = normalized_quantities(pb);
            let (Some(ca), Some(cb), Some(va), Some(vb), Some(za), Some(zb)) = (
                na.chipload_mm_tooth,
                nb.chipload_mm_tooth,
                na.cutting_speed_m_min,
                nb.cutting_speed_m_min,
                na.doc_ratio,
                nb.doc_ratio,
            ) else {
                continue;
            };
            if na.radial_engagement_ratio != nb.radial_engagement_ratio {
                continue;
            }
            let plunge = |p: &Profile| match p.data {
                CuttingData::ExactValues { plunge_mm_min, .. }
                | CuttingData::ChiploadTarget { plunge_mm_min, .. } => plunge_mm_min,
            };
            let mut p = ra.provenance.profile.clone();
            p.data = CuttingData::ChiploadTarget {
                rpm: lerp(va, vb) * 1000.0 / (std::f64::consts::PI * d),
                chipload_mm_tooth: lerp(ca, cb),
                plunge_mm_min: lerp(plunge(pa) / x, plunge(pb) / y) * d,
                doc_mm: lerp(za, zb) * d,
                stepover_ratio: ra.parameters.stepover_ratio,
            };
            let mut set = ProfileSet {
                schema_version: 1,
                identity: request.dataset.clone(),
                machine_families: vec![],
                material_families: vec![],
                profiles: vec![p],
            };
            // Candidate scopes are already normalized to this exact request.
            set.profiles[0].scope.machine = IdentitySelector::Exact {
                identity: request.machine.identity.clone().unwrap_or(Identity {
                    id: "unknown".into(),
                    revision: "1".into(),
                }),
            };
            if request.machine.identity.is_none() {
                set.profiles[0].scope.machine = IdentitySelector::Any;
            }
            if let Ok(engine) = Engine::new(set) {
                if let FeedsSpeedsRecommendation::Available { recommendation } =
                    engine.recommend(request)
                {
                    let mut c = a.clone();
                    let mut detail = da.clone();
                    detail.quality = MatchQuality::Interpolated;
                    detail.method = "normalized_linear_interpolation".into();
                    detail.source_profiles.push(pb.clone());
                    detail.source_diameters_mm.push(y);
                    detail.interpolation_weight = Some(t);
                    detail.relative_delta = 0.0;
                    detail.chipload_factor = 1.0;
                    detail.doc_factor = 1.0;
                    c.derivation = Some(detail);
                    c.rank.quality_cost = MatchQuality::Interpolated.rank_cost();
                    c.rank.relative_delta_ppm = 0;
                    pairs.push((y - x, c, *recommendation));
                }
            }
        }
    }
    pairs.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then(a.1.pack.cmp(&b.1.pack))
            .then(a.1.record_id.cmp(&b.1.record_id))
    });
    let mut added = std::collections::BTreeSet::new();
    for (_, c, r) in pairs {
        if added.insert(c.pack.clone()) {
            candidates.push((c, r));
        }
    }
}

/// Deterministic diagnostic only; never drives application or post-processing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchingTrace {
    pub pack: Identity,
    pub record_id: String,
    pub accepted: bool,
    pub selected: bool,
    pub relative_diameter_delta: Option<f64>,
    pub method: Option<String>,
    pub rejected_dimensions: Vec<ScopeMismatch>,
}
pub fn matching_trace(
    packs: &[SourcePack],
    r: &FeedsSpeedsRequest,
) -> Result<Vec<MatchingTrace>, String> {
    matching_trace_checked(packs, r).map_err(|e| e.to_string())
}
pub fn matching_trace_checked(
    packs: &[SourcePack],
    r: &FeedsSpeedsRequest,
) -> Result<Vec<MatchingTrace>, SourcePackError> {
    let mut trace = Vec::new();
    let result = crate::source_pack::recommend_packs_traced(packs, r, Some(&mut trace))?;
    if let Some(explanation) = result.explanation {
        for row in &mut trace {
            row.selected = explanation
                .summary
                .contributors
                .iter()
                .any(|c| c.pack == row.pack && c.record_id == row.record_id);
        }
    }
    trace.sort_by(|a, b| a.pack.cmp(&b.pack).then(a.record_id.cmp(&b.record_id)));
    Ok(trace)
}

/// Explain failed derived eligibility, rather than listing scopes the declared
/// transfer policy already permits. This function never supplies cutting values.
// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
pub(crate) fn derived_rejections(
    pack: &SourcePack,
    origin: &RecordOrigin,
    p: &Profile,
    r: &FeedsSpeedsRequest,
    set: &ProfileSet,
) -> Vec<ScopeMismatch> {
    let mut rejected = crate::matching::mismatches(p, r, set);
    if p.category != ProfileCategory::Reference
        || p.evidence != Evidence::ReferenceData
        || !pack.manifest.transformation_permitted
        || !p.reductions.is_empty()
        || p.scope.diameter_mm.min != p.scope.diameter_mm.max
    {
        return rejected;
    }
    let slot_transfer = p.scope.operation == OperationKind::FullSlot
        && p.scope.engagement == EngagementScope::FullWidth
        && r.operation.kind == OperationKind::Pocket
        && matches!(r.operation.engagement,Engagement::Nominal{ratio} if ratio>0.0 && ratio<=1.0)
        && origin.adaptation.as_ref().is_some_and(|a| a.slot_to_pocket);
    rejected.retain(|m| match m.field.as_str() {
        "doc_mm" | "diameter_mm" => false,
        "machine" => !origin.adaptation.as_ref().is_some_and(|a| {
            r.machine
                .identity
                .as_ref()
                .is_some_and(|id| a.machines.contains(id))
        }),
        "material" => !origin.adaptation.as_ref().is_some_and(|a| {
            r.material.identity.as_ref().is_some_and(|id| {
                a.material_taxa
                    .iter()
                    .any(|t| crate::source_pack::taxon_matches(t, id))
            })
        }),
        "tool" => !origin.adaptation.as_ref().is_some_and(|a| {
            r.tool
                .identity
                .as_ref()
                .is_some_and(|id| a.tools.contains(id))
        }),
        "operation" | "engagement" => !slot_transfer,
        _ => true,
    });
    let d = p.scope.diameter_mm.min;
    let delta = (d - r.tool.diameter_mm).abs() / r.tool.diameter_mm;
    if delta > 0.5 {
        rejected.push(ScopeMismatch {
            field: "diameter_proximity".into(),
            actual: format!("\"{:.2} %\"", delta * 100.0),
            required: format!("\"≤50 % · Ø{d} mm source\""),
        });
    }
    rejected
}
