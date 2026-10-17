use serde::{Deserialize, Serialize};

/// Immutable identity: labels never participate in matching.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsIdentity"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub id: String,
    pub revision: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsAggressiveness"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Aggressiveness {
    Prudent,
    Normal,
    Rapid,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsEvidence"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    Theoretical,
    ReferenceData,
    Tested,
    Validated,
}

/// Cutting family, deliberately distinct from CAM geometry families.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsCuttingFamily"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CuttingFamily {
    FlatEndmill,
    Compression,
    VBit,
    Other,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsOperationKind"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    Profile,
    Pocket,
    FullSlot,
    Finish,
    Surfacing,
    Engraving,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsEntryStrategy"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryStrategy {
    Direct,
    Ramp,
    Helix,
    Unknown,
}

/// Nominal stepover is not a bound on instantaneous physical engagement.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsEngagement"))]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Engagement {
    KnownFullWidth,
    Nominal { ratio: f64 },
    Unknown,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsMachineSnapshot"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MachineSnapshot {
    pub identity: Option<Identity>,
    pub rpm_min: Option<u32>,
    pub rpm_max: Option<u32>,
    pub max_feed_mm_min: Option<u32>,
    pub max_plunge_mm_min: Option<u32>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsMaterialSnapshot"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialSnapshot {
    /// Grade/condition are part of immutable material identity.
    pub identity: Option<Identity>,
    pub thickness_mm: Option<f64>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsToolSnapshot"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolSnapshot {
    pub identity: Option<Identity>,
    pub cutting_family: CuttingFamily,
    pub diameter_mm: f64,
    pub flute_count: u8,
    pub max_rpm: Option<u32>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsOperationContext"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationContext {
    pub kind: OperationKind,
    pub doc_mm: Option<f64>,
    pub engagement: Engagement,
    pub entry: EntryStrategy,
}

/// Immutable milling-only context. There is no generic process parameter bag.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsFeedsSpeedsRequest"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedsSpeedsRequest {
    pub schema_version: u32,
    pub dataset: Identity,
    pub machine: MachineSnapshot,
    pub material: MaterialSnapshot,
    pub tool: ToolSnapshot,
    pub operation: OperationContext,
    pub aggressiveness: Aggressiveness,
}

/// Inclusive range with explicit units in its containing field name.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsRange"))]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsIdentitySelector"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum IdentitySelector {
    Any,
    Exact { identity: Identity },
    Family { id: String },
}

/// Authored unknown-safe data is a separate scope, not an assumed low WOC.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsEngagementScope"))]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EngagementScope {
    FullWidth,
    Nominal { ratio: Range },
    UnknownSafe,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsProfileScope"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileScope {
    pub machine: IdentitySelector,
    pub material: IdentitySelector,
    pub tool: Option<Identity>,
    pub cutting_family: CuttingFamily,
    pub diameter_mm: Range,
    pub flute_count: Option<u8>,
    pub operation: OperationKind,
    pub doc_mm: Option<Range>,
    pub engagement: EngagementScope,
    pub entry: Option<EntryStrategy>,
    pub thickness_mm: Option<Range>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsProfileCategory"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileCategory {
    ExactPreset,
    MachineFamily,
    Reference,
    Theoretical,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsEvidenceRecord"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRecord {
    pub trial_reference: String,
    pub validation_criteria_reference: Option<String>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsSource"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub reference: String,
    pub author: String,
    pub date: String,
    pub permission_or_license: String,
    pub notes: String,
    pub evidence_record: Option<EvidenceRecord>,
}

/// Each profile supplies its own mode/operation-specific values. No hidden factors.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsCuttingData"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CuttingData {
    ChiploadTarget {
        rpm: f64,
        chipload_mm_tooth: f64,
        plunge_mm_min: f64,
        doc_mm: f64,
        stepover_ratio: Option<f64>,
    },
    ExactValues {
        rpm: f64,
        feed_mm_min: f64,
        plunge_mm_min: f64,
        doc_mm: f64,
        stepover_ratio: Option<f64>,
    },
}

/// Named empirical reductions apply only within the enclosing profile's scope.
/// FS1 allows factors (0,1]; expansions require a separately qualified policy.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsReductionRule"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReductionRule {
    pub identity: Identity,
    pub source_reference: String,
    pub chipload_factor: f64,
    pub plunge_factor: f64,
    pub doc_factor: f64,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsProfile"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub identity: Identity,
    pub category: ProfileCategory,
    pub evidence: Evidence,
    pub source: Source,
    pub aggressiveness: Aggressiveness,
    pub scope: ProfileScope,
    pub data: CuttingData,
    /// Ordered; order is retained in provenance. Empty for exact presets.
    pub reductions: Vec<ReductionRule>,
}

/// Explicit membership, never inferred from names or supplied as a wildcard.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsFamilyMembership"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMembership {
    pub id: String,
    pub members: Vec<Identity>,
}

/// FS1 in-memory domain input, not the FS2 distribution/authoring file format.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsProfileSet"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileSet {
    pub schema_version: u32,
    pub identity: Identity,
    pub machine_families: Vec<FamilyMembership>,
    pub material_families: Vec<FamilyMembership>,
    pub profiles: Vec<Profile>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsValidationIssue"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsUnavailableReason"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum UnavailableReason {
    InvalidRequest { issues: Vec<ValidationIssue> },
    DatasetMismatch,
    MissingMaterial,
    UnsupportedTool,
    UnsupportedOperation,
    MissingCuttingData,
    AmbiguousProfiles { identities: Vec<Identity> },
    UnrepresentableOutput { field: String },
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsApplicability"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Applicability {
    WithinDeclaredProfile,
    AdjustedOutsideEvidence,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsDiagnostic"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Diagnostic {
    UnknownLimit {
        field: String,
    },
    UnknownEngagementFullWidthAssumption,
    NominalEngagementNotMaximum,
    UnknownEntryStrategy,
    UnknownDoc,
    /// Feed ceiling reduced chipload below the selected target; not a material rubbing threshold.
    LowEffectiveChipload {
        chipload_mm_tooth: f64,
        target_chipload_mm_tooth: f64,
    },
    GenericProfileDirectEntry,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsAdjustment"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adjustment {
    pub field: String,
    pub reason: String,
    pub before: f64,
    pub after: f64,
    pub rule: Option<Identity>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsRecommendedParameters"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecommendedParameters {
    pub rpm: u32,
    pub feed_mm_min: u32,
    pub plunge_mm_min: u32,
    pub doc_mm: f64,
    pub stepover_ratio: Option<f64>,
    pub effective_chipload_mm_tooth: f64,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsProvenance"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub engine_version: String,
    pub schema_version: u32,
    pub dataset: Identity,
    /// Full selected profile preserves source, scope, data and rule revisions.
    pub profile: Profile,
    /// Selected family membership snapshots keep matching explainable offline.
    pub machine_family: Option<FamilyMembership>,
    pub material_family: Option<FamilyMembership>,
    pub request: FeedsSpeedsRequest,
    pub selection_tier: u8,
    pub adjustments: Vec<Adjustment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation: Option<crate::Derivation>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsRecommendation"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recommendation {
    pub parameters: RecommendedParameters,
    pub aggressiveness: Aggressiveness,
    pub source_evidence: Evidence,
    pub applicability: Applicability,
    pub diagnostics: Vec<Diagnostic>,
    pub provenance: Provenance,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsFeedsSpeedsRecommendation"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum FeedsSpeedsRecommendation {
    Available { recommendation: Box<Recommendation> },
    Unavailable { reason: UnavailableReason },
}

/// Presentation-only explanation of a refused scope; values are JSON-encoded.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsScopeMismatch"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeMismatch {
    pub field: String,
    pub actual: String,
    pub required: String,
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "FsProfileRejection"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileRejection {
    pub profile: Identity,
    pub mismatches: Vec<ScopeMismatch>,
}
