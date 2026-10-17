//! Versioned DATA-only pack boundary and deterministic reconciliation. No I/O.
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PACK_SCHEMA_VERSION: u32 = 3;
pub const MAX_PACK_BYTES: usize = 2 * 1024 * 1024;
pub const KNOWLEDGE_POLICY: &str = "fs39-normalized-matching-1";

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsVisibility"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    Private,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsDistribution"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Distribution {
    Bundled,
    LocalOnly,
    ExternalReference,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsPackManifest"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackManifest {
    pub identity: Identity,
    pub schema_version: u32,
    pub name: String,
    pub publisher: String,
    pub source_url: Option<String>,
    pub origin: String,
    pub license: String,
    pub license_url: Option<String>,
    pub attribution: String,
    pub visibility: Visibility,
    pub distribution: Distribution,
    pub redistributable: bool,
    pub transformation_permitted: bool,
    pub default_evidence: Evidence,
    pub notes: String,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsToolClassification"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolClassification {
    pub family: CuttingFamily,
    pub helix: Option<String>,
    pub material: Option<String>,
    pub coating: Option<String>,
    pub shape: Option<String>,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsRecordOrigin"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordOrigin {
    pub profile: Identity,
    pub source_record_id: String,
    pub original_classification: String,
    /// Explicit taxonomy applicability, including descendants. None preserves original identity.
    pub material_taxon: Option<String>,
    pub tool: Option<ToolClassification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lab: Option<LabPreset>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lab_history: Vec<LabRevision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adaptation: Option<AdaptationScope>,
}
pub const LAB_PACK_ID: &str = "sorbonne-fablab";
pub const LAB_VALIDATION_POLICY: &str = "sorbonne-local-validation-1";
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsLabState"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabState {
    Draft,
    Tested,
    Validated,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsLabReview"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabReview {
    pub policy: String,
    pub reviewed_at: String,
    pub notes: String,
    pub acceptable_finish_and_behavior: bool,
    pub no_overheating_chatter_or_overload: bool,
    pub suitable_for_lab_use: bool,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsLabPreset"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabPreset {
    pub name: String,
    pub state: LabState,
    pub created_at: String,
    pub updated_at: String,
    pub tested_at: Option<String>,
    pub physically_tested: bool,
    pub no_critical_failure: bool,
    pub operator: String,
    pub job_reference: String,
    pub observations: String,
    pub detailed_notes: String,
    pub finishing_context: String,
    pub enabled: bool,
    pub deleted: bool,
    pub review: Option<LabReview>,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsLabRevision"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabRevision {
    pub profile: Profile,
    pub metadata: LabPreset,
}
fn valid_lab_date(value: &str) -> bool {
    let s = value.get(..10).unwrap_or("");
    if s.len() != 10 || !s.is_ascii() || s.as_bytes()[4] != b'-' || s.as_bytes()[7] != b'-' {
        return false;
    }
    let Ok(year) = s[..4].parse::<u32>() else {
        return false;
    };
    let Ok(month) = s[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = s[8..10].parse::<u32>() else {
        return false;
    };
    let max = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 0,
    };
    year > 0 && day > 0 && day <= max
}
fn validate_lab(pack: &SourcePack, profile: &Profile, lab: &LabPreset) -> Result<(), String> {
    if pack.manifest.identity.id != LAB_PACK_ID
        || pack.manifest.visibility != Visibility::Private
        || pack.manifest.distribution != Distribution::LocalOnly
    {
        return Err("Lab presets require the private local Sorbonne pack".into());
    }
    if lab.name.trim().is_empty()
        || !valid_lab_date(&lab.created_at)
        || !valid_lab_date(&lab.updated_at)
    {
        return Err("Lab preset name and revision dates are required".into());
    }
    if lab.created_at[..10] > lab.updated_at[..10] {
        return Err("Lab update date precedes creation".into());
    }
    match lab.state {
        LabState::Draft => {
            if profile.evidence != Evidence::Theoretical
                || profile.category != ProfileCategory::Theoretical
                || lab.enabled
                || lab.review.is_some()
            {
                return Err("Draft presets must be inactive, theoretical authoring records".into());
            }
        }
        LabState::Tested | LabState::Validated => {
            if !lab.physically_tested
                || !lab.no_critical_failure
                || lab
                    .tested_at
                    .as_ref()
                    .map_or(true, |s| s.len() != 10 || !valid_lab_date(s))
                || profile.category != ProfileCategory::ExactPreset
            {
                return Err(
                    "Tested presets require dated successful physical use and exact context".into(),
                );
            }
            let tested_at = lab
                .tested_at
                .as_deref()
                .ok_or("Missing physical test date")?;
            if tested_at > &lab.updated_at[..10] {
                return Err("Physical test date follows the saved revision date".into());
            }
            let expected = if lab.state == LabState::Tested {
                Evidence::Tested
            } else {
                Evidence::Validated
            };
            if profile.evidence != expected {
                return Err("Lab workflow and cutting evidence disagree".into());
            }
            if lab.state == LabState::Validated {
                let review = lab
                    .review
                    .as_ref()
                    .ok_or("Validated presets require deliberate review")?;
                if review.policy != LAB_VALIDATION_POLICY
                    || !valid_lab_date(&review.reviewed_at)
                    || review.reviewed_at[..10] < *tested_at
                    || review.reviewed_at[..10] > lab.updated_at[..10]
                    || review.notes.trim().is_empty()
                    || !review.acceptable_finish_and_behavior
                    || !review.no_overheating_chatter_or_overload
                    || !review.suitable_for_lab_use
                    || profile
                        .source
                        .evidence_record
                        .as_ref()
                        .and_then(|r| r.validation_criteria_reference.as_deref())
                        != Some(LAB_VALIDATION_POLICY)
                {
                    return Err("Sorbonne validation review criteria are incomplete".into());
                }
            } else if lab.review.is_some() {
                return Err("Tested preset cannot carry a validated review".into());
            }
        }
    }
    Ok(())
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsSourcePack"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePack {
    pub manifest: PackManifest,
    pub dataset: DatasetDocument,
    pub records: Vec<RecordOrigin>,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsPackCollection"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackCollection {
    pub source_packs: Vec<SourcePack>,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsMaterialTaxon"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialTaxon {
    pub id: String,
    pub parent: Option<String>,
    pub label: String,
}
const MATERIAL_TAXONOMY: &[(&str, Option<&str>, &str)] = &[
    ("wood", None, "Wood"),
    ("wood.mdf", Some("wood"), "MDF"),
    ("wood.plywood", Some("wood"), "Plywood"),
    (
        "wood.plywood.poplar",
        Some("wood.plywood"),
        "Poplar plywood",
    ),
    ("wood.plywood.birch", Some("wood.plywood"), "Birch plywood"),
    ("wood.hardwood", Some("wood"), "Hardwood"),
    ("wood.softwood", Some("wood"), "Softwood"),
    ("plastic", None, "Plastic"),
    ("plastic.pmma", Some("plastic"), "PMMA"),
    ("plastic.pmma.cast", Some("plastic.pmma"), "Cast PMMA"),
    (
        "plastic.pmma.extruded",
        Some("plastic.pmma"),
        "Extruded PMMA",
    ),
    ("plastic.pom", Some("plastic"), "POM"),
    ("plastic.hdpe", Some("plastic"), "HDPE"),
    ("plastic.polycarbonate", Some("plastic"), "Polycarbonate"),
    ("plastic.pvc", Some("plastic"), "PVC"),
    ("metal", None, "Metal"),
    ("metal.aluminium", Some("metal"), "Aluminium"),
    (
        "metal.aluminium.6061",
        Some("metal.aluminium"),
        "Aluminium 6061",
    ),
    (
        "metal.aluminium.7075",
        Some("metal.aluminium"),
        "Aluminium 7075",
    ),
];

pub(crate) fn known_taxon(id: &str) -> bool {
    MATERIAL_TAXONOMY.iter().any(|t| t.0 == id)
}

pub fn material_taxonomy() -> Vec<MaterialTaxon> {
    MATERIAL_TAXONOMY
        .iter()
        .copied()
        .map(|(id, parent, label)| MaterialTaxon {
            id: id.into(),
            parent: parent.map(str::to_owned),
            label: label.into(),
        })
        .collect()
}
pub fn taxon_identity(id: &str) -> Identity {
    Identity {
        id: format!("material/{id}"),
        revision: "1".into(),
    }
}
pub(crate) fn taxon_matches(parent: &str, identity: &Identity) -> bool {
    taxon_distance(parent, identity).is_some()
}
pub(crate) fn taxon_distance(parent: &str, identity: &Identity) -> Option<u32> {
    if identity.revision != "1" {
        return None;
    }
    let id = identity.id.strip_prefix("material/")?;
    let mut taxon = MATERIAL_TAXONOMY.iter().find(|t| t.0 == id)?;
    let mut distance = 0;
    loop {
        if taxon.0 == parent {
            return Some(distance);
        }
        taxon = MATERIAL_TAXONOMY.iter().find(|t| Some(t.0) == taxon.1)?;
        distance += 1;
    }
}

pub fn safe_pack_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 100
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
}
pub fn load_source_pack_json(json: &str) -> Result<SourcePack, String> {
    load_source_pack_json_checked(json).map_err(|e| e.to_string())
}
pub fn load_source_pack_json_checked(json: &str) -> Result<SourcePack, SourcePackError> {
    if json.len() > MAX_PACK_BYTES {
        return Err(SourcePackError::SizeLimit {
            maximum_bytes: MAX_PACK_BYTES,
        });
    }
    let pack: SourcePack =
        serde_json::from_str(json).map_err(|e| SourcePackError::InvalidJson {
            message: e.to_string(),
        })?;
    validate_source_pack_checked(&pack)?;
    Ok(pack)
}
pub fn validate_source_pack(pack: &SourcePack) -> Result<(), String> {
    validate_source_pack_checked(pack).map_err(|e| e.to_string())
}
pub fn validate_source_pack_checked(pack: &SourcePack) -> Result<(), SourcePackError> {
    crate::source_error::check_pack_size(pack)?;
    validate_dataset(&pack.dataset).map_err(|issues| SourcePackError::InvalidDataset { issues })?;
    validate_pack_policy(pack).map_err(|message| SourcePackError::InvalidPack { message })
}
fn validate_pack_policy(pack: &SourcePack) -> Result<(), String> {
    let m = &pack.manifest;
    if !(1..=PACK_SCHEMA_VERSION).contains(&m.schema_version)
        || !safe_pack_id(&m.identity.id)
        || !safe_pack_id(&m.identity.revision)
    {
        return Err("Unsupported source pack schema or unsafe identity/version".into());
    }
    if [&m.name, &m.publisher, &m.origin, &m.license, &m.attribution]
        .iter()
        .any(|v| v.trim().is_empty())
    {
        return Err("Pack identity, provenance, license and attribution are required".into());
    }
    if m.distribution == Distribution::Bundled
        && (m.visibility != Visibility::Public
            || !m.redistributable
            || !m.transformation_permitted
            || m.license_url
                .as_ref()
                .map_or(true, |v| !v.starts_with("https://")))
    {
        return Err(
            "Bundled pack requires public redistribution/transformation rights and license URL"
                .into(),
        );
    }
    if m.distribution == Distribution::ExternalReference
        && !pack.dataset.profile_set.profiles.is_empty()
    {
        return Err("External references cannot supply executable cutting records".into());
    }
    if m.identity.id == LAB_PACK_ID
        && (m.visibility != Visibility::Private
            || m.distribution != Distribution::LocalOnly
            || m.identity.revision.len() != 16
            || !m.identity.revision.bytes().all(|b| b.is_ascii_digit())
            || pack.records.iter().any(|r| r.lab.is_none()))
    {
        return Err(
            "Sorbonne pack requires private local authoring records and a monotonic revision"
                .into(),
        );
    }
    if m.schema_version < 2
        && pack
            .records
            .iter()
            .any(|r| r.lab.is_some() || !r.lab_history.is_empty())
    {
        return Err("Local authoring metadata requires source-pack schema 2".into());
    }
    let profiles: std::collections::BTreeMap<_, _> = pack
        .dataset
        .profile_set
        .profiles
        .iter()
        .map(|p| (&p.identity, p))
        .collect();
    let mut classifications = std::collections::BTreeMap::new();
    for record in &pack.records {
        let profile = profiles
            .get(&record.profile)
            .ok_or("Record profile absent from dataset")?;
        if let (Some(id), Some(classification)) = (&profile.scope.tool, &record.tool) {
            if classifications
                .insert(id, classification)
                .is_some_and(|old| old != classification)
            {
                return Err("Conflicting classifications for the same tool identity".into());
            }
        }
    }
    let mut ids = BTreeSet::new();
    let mut records = BTreeSet::new();
    for r in &pack.records {
        if !ids.insert(&r.profile)
            || !records.insert(&r.source_record_id)
            || r.source_record_id.trim().is_empty()
            || r.original_classification.trim().is_empty()
        {
            return Err("Missing or duplicate record identity/classification".into());
        }
        let p = profiles
            .get(&r.profile)
            .ok_or("Record profile absent from dataset")?;
        validate_adaptation(pack, p, r, &classifications)?;
        if let Some(lab) = &r.lab {
            validate_lab(pack, p, lab)?;
            for previous in &r.lab_history {
                if previous.profile.identity.id != p.identity.id {
                    return Err("Lab history changed stable preset identity".into());
                }
                validate_lab(pack, &previous.profile, &previous.metadata)?;
                let mut previous_dataset = pack.dataset.clone();
                previous_dataset.profile_set.profiles = vec![previous.profile.clone()];
                validate_dataset(&previous_dataset)
                    .map_err(|e| DatasetLoadError::Invalid(e).to_string())?;
            }
        } else if !r.lab_history.is_empty() {
            return Err("History requires local lab metadata".into());
        }
        if r.material_taxon.as_ref().is_some_and(|id| !known_taxon(id)) {
            return Err("Unknown normalized material taxon".into());
        }
        let quantities = normalized_quantities(p);
        if [
            quantities.chipload_mm_tooth,
            quantities.cutting_speed_m_min,
            quantities.doc_ratio,
            quantities.radial_engagement_ratio,
        ]
        .into_iter()
        .flatten()
        .any(|v| !v.is_finite() || v <= 0.0)
        {
            return Err("Unrepresentable normalized physical quantity".into());
        }
        if let Some(t) = &r.tool {
            if p.scope.tool.is_none()
                && [
                    t.helix.as_ref(),
                    t.material.as_ref(),
                    t.coating.as_ref(),
                    t.shape.as_ref(),
                ]
                .into_iter()
                .any(|v| v.is_some())
            {
                return Err("Tool traits require an exact bound tool identity; unknown CAM traits cannot be assumed".into());
            }
            if t.family != p.scope.cutting_family
                || t.helix.as_ref().is_some_and(|v| {
                    !matches!(v.as_str(), "upcut" | "downcut" | "compression" | "straight")
                })
            {
                return Err("Invalid normalized tool classification".into());
            }
        }
        if matches!(p.evidence, Evidence::Tested | Evidence::Validated)
            && matches!(&p.scope.machine, IdentitySelector::Exact {identity} if identity.id=="machine/fablab-router")
        {
            return Err("Physical evidence requires an individual machine identity, not the fablab-router transfer class".into());
        }
        if matches!(p.evidence, Evidence::Tested | Evidence::Validated)
            && (p.category != ProfileCategory::ExactPreset
                || !matches!(p.scope.machine, IdentitySelector::Exact { .. })
                || !matches!(p.scope.material, IdentitySelector::Exact { .. })
                || p.scope.tool.is_none()
                || r.material_taxon.is_some())
        {
            return Err("Tested/Validated records require exact machine/material/tool scope without taxonomy fallback".into());
        }
    }
    if ids.len() != pack.dataset.profile_set.profiles.len() {
        return Err("Every cutting record requires origin metadata".into());
    }
    Ok(())
}
/// Distribution is enforced by the build registry separately; manifest alone grants no bundle entry.
pub fn public_bundle_eligible(pack: &SourcePack) -> bool {
    validate_source_pack(pack).is_ok()
        && pack.manifest.visibility == Visibility::Public
        && pack.manifest.distribution == Distribution::Bundled
        && pack.manifest.redistributable
        && pack.manifest.transformation_permitted
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsNormalizedQuantities"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedQuantities {
    pub chipload_mm_tooth: Option<f64>,
    pub cutting_speed_m_min: Option<f64>,
    pub doc_ratio: Option<f64>,
    pub radial_engagement_ratio: Option<f64>,
    /// The block is normalized; these flags distinguish copied source values.
    pub derived: bool,
    pub chipload_derived: bool,
    pub radial_engagement_derived: bool,
}
pub fn normalized_quantities(p: &Profile) -> NormalizedQuantities {
    let (rpm, chipload, doc, woc) = match p.data {
        CuttingData::ExactValues {
            rpm,
            feed_mm_min,
            doc_mm,
            stepover_ratio,
            ..
        } => (
            rpm,
            p.scope
                .flute_count
                .map(|n| feed_mm_min / (rpm * f64::from(n))),
            doc_mm,
            stepover_ratio,
        ),
        CuttingData::ChiploadTarget {
            rpm,
            chipload_mm_tooth,
            doc_mm,
            stepover_ratio,
            ..
        } => (rpm, Some(chipload_mm_tooth), doc_mm, stepover_ratio),
    };
    let diameter =
        (p.scope.diameter_mm.min == p.scope.diameter_mm.max).then_some(p.scope.diameter_mm.min);
    NormalizedQuantities {
        chipload_mm_tooth: chipload,
        cutting_speed_m_min: diameter.map(|d| std::f64::consts::PI * d * rpm / 1000.0),
        doc_ratio: diameter.map(|d| doc / d),
        radial_engagement_ratio: match p.scope.engagement {
            EngagementScope::FullWidth => Some(1.0),
            _ => woc,
        },
        derived: true,
        chipload_derived: matches!(p.data, CuttingData::ExactValues { .. }) && chipload.is_some(),
        radial_engagement_derived: matches!(p.scope.engagement, EngagementScope::FullWidth),
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsKnowledgeRecord"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeRecord {
    pub pack: Identity,
    pub record_id: String,
    pub profile: Identity,
    pub evidence: Evidence,
    pub local_only: bool,
    #[cfg_attr(feature = "schema", schemars(with = "Vec<u32>"))]
    pub rank: CandidateRank,
    pub normalized: NormalizedQuantities,
    pub origin: RecordOrigin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation: Option<Derivation>,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsKnowledgeContributor"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeContributor {
    pub pack: Identity,
    pub record_id: String,
}
/// Safe to save/share: no original records, source notes, paths or derived source quantities.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsKnowledgeSummary"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeSummary {
    pub policy: String,
    pub engine_version: String,
    pub selected_pack: Identity,
    pub selected_record: String,
    pub evidence: Evidence,
    pub local_only: bool,
    pub compatible_sources: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<KnowledgeContributor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_quality: Option<MatchQuality>,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsKnowledgeExplanation"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeExplanation {
    pub summary: KnowledgeSummary,
    pub records: Vec<KnowledgeRecord>,
    pub reference_chipload_range: Option<Range>,
    pub disagreement: bool,
    pub selection_reason: String,
}
#[derive(Debug, Clone)]
pub struct KnowledgeResult {
    pub recommendation: FeedsSpeedsRecommendation,
    pub explanation: Option<KnowledgeExplanation>,
    pub scope_rejections: Vec<ProfileRejection>,
}
/// All candidates go through the same Engine; no copied math and no averaging.
pub fn recommend_packs(
    packs: &[SourcePack],
    request: &FeedsSpeedsRequest,
) -> Result<KnowledgeResult, String> {
    recommend_packs_checked(packs, request).map_err(|e| e.to_string())
}
pub fn recommend_packs_checked(
    packs: &[SourcePack],
    request: &FeedsSpeedsRequest,
) -> Result<KnowledgeResult, SourcePackError> {
    recommend_packs_traced(packs, request, None)
}

pub(crate) fn recommend_packs_traced(
    packs: &[SourcePack],
    request: &FeedsSpeedsRequest,
    mut trace: Option<&mut Vec<MatchingTrace>>,
) -> Result<KnowledgeResult, SourcePackError> {
    if packs.len() > 32 {
        return Err(SourcePackError::TooManyPacks { maximum: 32 });
    }
    let preflight = Engine::new(ProfileSet {
        schema_version: DOMAIN_SCHEMA_VERSION,
        identity: request.dataset.clone(),
        machine_families: vec![],
        material_families: vec![],
        profiles: vec![],
    })
    .map_err(|issues| SourcePackError::InvalidDataset { issues })?
    .recommend(request);
    if let FeedsSpeedsRecommendation::Unavailable { reason } = preflight {
        if reason != UnavailableReason::MissingCuttingData {
            return Ok(KnowledgeResult {
                recommendation: FeedsSpeedsRecommendation::Unavailable { reason },
                explanation: None,
                scope_rejections: vec![],
            });
        }
    }
    let mut seen = BTreeSet::new();
    let mut candidates = Vec::new();
    let mut refusals = Vec::new();
    for pack in packs {
        validate_source_pack_checked(pack)?;
        if !seen.insert(&pack.manifest.identity.id) {
            return Err(SourcePackError::DuplicatePack {
                id: pack.manifest.identity.id.clone(),
            });
        }
        for origin in &pack.records {
            let eligible = pack.manifest.distribution != Distribution::ExternalReference
                && !origin
                    .lab
                    .as_ref()
                    .is_some_and(|lab| lab.state == LabState::Draft || !lab.enabled || lab.deleted);
            if !eligible && trace.is_none() {
                continue;
            }
            let original = pack
                .dataset
                .profile_set
                .profiles
                .iter()
                .find(|p| p.identity == origin.profile)
                .ok_or_else(|| SourcePackError::InvalidPack {
                    message: "Record profile absent from validated dataset".into(),
                })?;
            let mut p = original.clone();
            let source_set = &pack.dataset.profile_set;
            let mut set = ProfileSet {
                schema_version: source_set.schema_version,
                identity: source_set.identity.clone(),
                machine_families: source_set.machine_families.clone(),
                material_families: source_set.material_families.clone(),
                profiles: Vec::new(),
            };
            let mut r = request.clone();
            r.dataset = set.identity.clone();
            if let (Some(taxon), Some(identity)) = (&origin.material_taxon, &r.material.identity) {
                if taxon_matches(taxon, identity) {
                    p.scope.material = IdentitySelector::Exact {
                        identity: identity.clone(),
                    };
                }
            }
            let mut derivation = None;
            if !crate::matching::matches(&p, &r, &set) {
                if let Some((adapted, detail)) = adapt_profile(pack, origin, original, &p, &r, &set)
                {
                    p = adapted;
                    derivation = Some(detail);
                }
            }
            let taxonomy_mismatch = origin.material_taxon.as_ref().is_some_and(|t| {
                !r.material
                    .identity
                    .as_ref()
                    .is_some_and(|m| taxon_matches(t, m))
            });
            let mut rejected = derived_rejections(pack, origin, &p, &r, &set);
            if taxonomy_mismatch {
                rejected.retain(|m| m.field != "material");
                rejected.push(ScopeMismatch {
                    field: "material".into(),
                    actual: serde_json::to_string(&r.material.identity).map_err(|e| {
                        SourcePackError::InvalidJson {
                            message: e.to_string(),
                        }
                    })?,
                    required: serde_json::to_string(&origin.material_taxon).map_err(|e| {
                        SourcePackError::InvalidJson {
                            message: e.to_string(),
                        }
                    })?,
                });
            }
            set.profiles = vec![p.clone()];
            let engine =
                Engine::new(set).map_err(|issues| SourcePackError::InvalidDataset { issues })?;
            let result = engine.recommend(&r);
            if let Some(rows) = trace.as_deref_mut() {
                rows.push(MatchingTrace {
                    pack: pack.manifest.identity.clone(),
                    record_id: origin.source_record_id.clone(),
                    accepted: eligible
                        && !taxonomy_mismatch
                        && matches!(&result, FeedsSpeedsRecommendation::Available { .. }),
                    selected: false,
                    relative_diameter_delta: (original.scope.diameter_mm.min
                        == original.scope.diameter_mm.max)
                        .then(|| {
                            (original.scope.diameter_mm.min - request.tool.diameter_mm).abs()
                                / request.tool.diameter_mm
                        }),
                    method: derivation.as_ref().map(|d| d.method.clone()),
                    rejected_dimensions: rejected.clone(),
                });
            }
            if !eligible {
                continue;
            }
            if taxonomy_mismatch {
                refusals.push(ProfileRejection {
                    profile: p.identity.clone(),
                    mismatches: rejected,
                });
                continue;
            }
            match result {
                FeedsSpeedsRecommendation::Available { recommendation } => {
                    // Smaller is preferred. Unknown/nonspecific dimensions explicitly cost specificity.
                    let local = pack.manifest.distribution == Distribution::LocalOnly;
                    let preset = match (local, p.category, p.evidence) {
                        (true, ProfileCategory::ExactPreset, Evidence::Validated) => 0,
                        (true, ProfileCategory::ExactPreset, Evidence::Tested) => 1,
                        _ => 2,
                    };
                    let selector_cost = |s: &IdentitySelector| match s {
                        IdentitySelector::Exact { .. } => 0,
                        IdentitySelector::Family { .. } => 1,
                        IdentitySelector::Any => 2,
                    };
                    let mut material_cost = origin.material_taxon.as_ref().map_or(
                        selector_cost(&original.scope.material),
                        |taxon| {
                            r.material
                                .identity
                                .as_ref()
                                .and_then(|m| taxon_distance(taxon, m))
                                .unwrap_or(100)
                        },
                    );
                    if origin.material_taxon.is_none()
                        && derivation.as_ref().is_some_and(|d| d.material_transfer)
                    {
                        // Rank the explicitly permitted transfer scope, not an invented source match.
                        material_cost = origin
                            .adaptation
                            .as_ref()
                            .and_then(|a| {
                                r.material.identity.as_ref().and_then(|m| {
                                    a.material_taxa
                                        .iter()
                                        .filter_map(|t| taxon_distance(t, m))
                                        .min()
                                })
                            })
                            .map_or(material_cost.max(1), |distance| distance + 1);
                    }
                    let rank = CandidateRank {
                        preset_tier: preset,
                        material_cost,
                        quality_cost: derivation.as_ref().map_or(0, |d| d.quality.rank_cost()),
                        machine_transfer: derivation
                            .as_ref()
                            .map_or(0, |d| d.machine_transfer as u32),
                        machine_specificity: selector_cost(&original.scope.machine),
                        nonspecific_tool: u32::from(p.scope.tool.is_none()),
                        diameter_range: u32::from(
                            p.scope.diameter_mm.min != p.scope.diameter_mm.max,
                        ),
                        nonspecific_flutes: u32::from(p.scope.flute_count.is_none()),
                        nonspecific_doc: u32::from(p.scope.doc_mm.is_none()),
                        nonspecific_entry: u32::from(p.scope.entry.is_none()),
                        category_tier: crate::matching::tier(&p).into(),
                        relative_delta_ppm: derivation
                            .as_ref()
                            .map_or(0, |d| (d.relative_delta * 1_000_000.0).round() as u32),
                    };
                    let record = KnowledgeRecord {
                        pack: pack.manifest.identity.clone(),
                        record_id: origin.source_record_id.clone(),
                        profile: origin.profile.clone(),
                        evidence: p.evidence,
                        local_only: local,
                        rank,
                        normalized: normalized_quantities(original),
                        origin: origin.clone(),
                        derivation: derivation.clone(),
                    };
                    candidates.push((record, *recommendation));
                }
                FeedsSpeedsRecommendation::Unavailable { .. } => {
                    if !rejected.is_empty() {
                        refusals.push(ProfileRejection {
                            profile: p.identity.clone(),
                            mismatches: rejected,
                        });
                    }
                }
            }
        }
    }
    add_interpolations(&mut candidates, request);
    if candidates.is_empty() {
        refusals.sort_by(|a, b| {
            a.mismatches
                .len()
                .cmp(&b.mismatches.len())
                .then(a.profile.cmp(&b.profile))
        });
        // Identical profile identities may legitimately occur in independent packs.
        // Collapse identical diagnostics so consumer keyed lists remain unique.
        refusals.dedup_by(|a, b| a.profile == b.profile && a.mismatches == b.mismatches);
        refusals.truncate(3);
        return Ok(KnowledgeResult {
            recommendation: FeedsSpeedsRecommendation::Unavailable {
                reason: UnavailableReason::MissingCuttingData,
            },
            explanation: None,
            scope_rejections: refusals,
        });
    }
    candidates.sort_by(|a, b| {
        a.0.rank
            .cmp(&b.0.rank)
            .then(a.0.pack.cmp(&b.0.pack))
            .then(a.0.record_id.cmp(&b.0.record_id))
    });
    let rank = candidates[0].0.rank;
    candidates.retain(|v| v.0.rank == rank);
    let authored_context = |r: &Recommendation| match r.provenance.profile.data {
        CuttingData::ExactValues {
            rpm,
            doc_mm,
            stepover_ratio,
            ..
        }
        | CuttingData::ChiploadTarget {
            rpm,
            doc_mm,
            stepover_ratio,
            ..
        } => (rpm, doc_mm, stepover_ratio),
    };
    let comparable = candidates.iter().all(|(_, r)| {
        ({
            let (ar, ad, aw) = authored_context(r);
            let (br, bd, bw) = authored_context(&candidates[0].1);
            near(ar, br) && near(ad, bd) && near_option(aw, bw)
        }) && r.provenance.profile.reductions == candidates[0].1.provenance.profile.reductions
            && r.parameters.rpm == candidates[0].1.parameters.rpm
            && near(r.parameters.doc_mm, candidates[0].1.parameters.doc_mm)
            && near_option(
                r.parameters.stepover_ratio,
                candidates[0].1.parameters.stepover_ratio,
            )
            && r.provenance.profile.scope.engagement
                == candidates[0].1.provenance.profile.scope.engagement
    });
    if !comparable {
        return Ok(KnowledgeResult {
            recommendation: FeedsSpeedsRecommendation::Unavailable {
                reason: UnavailableReason::AmbiguousProfiles {
                    identities: candidates.iter().map(|v| v.0.profile.clone()).collect(),
                },
            },
            explanation: None,
            scope_rejections: vec![],
        });
    }
    candidates.sort_by(|a, b| {
        a.1.parameters
            .effective_chipload_mm_tooth
            .total_cmp(&b.1.parameters.effective_chipload_mm_tooth)
            .then(a.0.pack.cmp(&b.0.pack))
            .then(a.0.record_id.cmp(&b.0.record_id))
    });
    let i = match request.aggressiveness {
        Aggressiveness::Prudent => 0,
        Aggressiveness::Normal => (candidates.len() - 1) / 2,
        Aggressiveness::Rapid => candidates.len() - 1,
    };
    let (selected, mut recommendation) = candidates[i].clone();
    // Keep original authored record snapshot rather than its temporary normalized scope.
    let pack = packs
        .iter()
        .find(|p| p.manifest.identity == selected.pack)
        .ok_or_else(|| SourcePackError::Recommendation {
            message: "Missing candidate pack".into(),
        })?;
    recommendation.provenance.profile = pack
        .dataset
        .profile_set
        .profiles
        .iter()
        .find(|p| p.identity == selected.profile)
        .ok_or_else(|| SourcePackError::Recommendation {
            message: "Missing candidate profile".into(),
        })?
        .clone();
    recommendation.provenance.derivation = selected.derivation.clone();
    let range = Range {
        min: candidates[0].1.parameters.effective_chipload_mm_tooth,
        max: candidates[candidates.len() - 1]
            .1
            .parameters
            .effective_chipload_mm_tooth,
    };
    let sources = candidates
        .iter()
        .map(|v| &v.0.pack.id)
        .collect::<BTreeSet<_>>()
        .len() as u32;
    let mut contributors: Vec<KnowledgeContributor> = candidates
        .iter()
        .map(|(record, _)| KnowledgeContributor {
            pack: record.pack.clone(),
            record_id: record.record_id.clone(),
        })
        .collect();
    for (candidate, _) in &candidates {
        if let Some(derivation) = &candidate.derivation {
            let source_pack = packs
                .iter()
                .find(|p| p.manifest.identity == candidate.pack)
                .ok_or_else(|| SourcePackError::Recommendation {
                    message: "Missing candidate source pack".into(),
                })?;
            for source in &derivation.source_profiles {
                if let Some(origin) = source_pack
                    .records
                    .iter()
                    .find(|origin| origin.profile == source.identity)
                {
                    contributors.push(KnowledgeContributor {
                        pack: source_pack.manifest.identity.clone(),
                        record_id: origin.source_record_id.clone(),
                    });
                }
            }
        }
    }
    contributors.sort_by(|a, b| a.pack.cmp(&b.pack).then(a.record_id.cmp(&b.record_id)));
    contributors.dedup_by(|a, b| a.pack == b.pack && a.record_id == b.record_id);
    let summary = KnowledgeSummary {
        policy: KNOWLEDGE_POLICY.into(),
        engine_version: ENGINE_VERSION.into(),
        selected_pack: selected.pack,
        selected_record: selected.record_id,
        evidence: recommendation.source_evidence,
        local_only: selected.local_only,
        compatible_sources: sources,
        match_quality: Some(selected.derivation.as_ref().map_or_else(
            || {
                if selected.evidence == Evidence::Theoretical {
                    MatchQuality::Theoretical
                } else if selected.rank.material_cost > 0 {
                    MatchQuality::Generic
                } else {
                    MatchQuality::Exact
                }
            },
            |d| d.quality,
        )),
        contributors,
    };
    let explanation=KnowledgeExplanation {summary,records:candidates.into_iter().map(|v|v.0).collect(),reference_chipload_range:Some(range),disagreement:range.max>range.min*1.25,selection_reason:format!("{:?}: select same-mode reference or explicit normalized derivation; rank dimensions: local exact trial, material specificity, adaptation quality, machine transfer, machine specificity, tool, diameter, flutes, DOC, entry, category, relative distance",request.aggressiveness)};
    Ok(KnowledgeResult {
        recommendation: FeedsSpeedsRecommendation::Available {
            recommendation: Box::new(recommendation),
        },
        explanation: Some(explanation),
        scope_rejections: vec![],
    })
}

// Absolute tolerance in native units; no fuzzy matching of identity, scopes or safety limits.
fn near(a: f64, b: f64) -> bool {
    a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-9
}
fn near_option(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => near(a, b),
        (None, None) => true,
        _ => false,
    }
}
