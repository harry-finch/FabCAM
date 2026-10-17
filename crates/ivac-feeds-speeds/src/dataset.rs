//! Explicit JSON distribution boundary. No filesystem/network access or bundled default.
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const DATASET_FORMAT_VERSION: u32 = 1;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogEntry {
    pub identity: Identity,
    pub label: String,
    pub notes: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolDefinition {
    pub identity: Identity,
    pub label: String,
    pub cutting_family: CuttingFamily,
    pub diameter_mm: f64,
    pub flute_count: u8,
    pub max_rpm: Option<u32>,
    pub notes: String,
}

/// Self-contained authoring document; no implicit inheritance or remote references.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetDocument {
    pub format_version: u32,
    pub description: String,
    pub license: String,
    pub machines: Vec<CatalogEntry>,
    pub materials: Vec<CatalogEntry>,
    pub tools: Vec<ToolDefinition>,
    pub profile_set: ProfileSet,
}

#[derive(Debug)]
pub enum DatasetLoadError {
    Json(serde_json::Error),
    Invalid(Vec<ValidationIssue>),
}

impl std::fmt::Display for DatasetLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(f, "invalid dataset JSON: {error}"),
            Self::Invalid(issues) => {
                write!(f, "invalid dataset: {} issue(s)", issues.len())?;
                for issue in issues.iter().take(3) {
                    write!(f, "; {}: {}", issue.field, issue.message)?;
                }
                Ok(())
            }
        }
    }
}
impl std::error::Error for DatasetLoadError {}

/// Only the loader constructs this validated pair. The caller owns I/O and selection.
#[derive(Debug, Clone)]
pub struct LoadedDataset {
    document: DatasetDocument,
    engine: Engine,
}
impl LoadedDataset {
    pub fn document(&self) -> &DatasetDocument {
        &self.document
    }
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}

/// Reject malformed/unknown fields, unsupported versions, dangling identities and
/// invalid engineering data before exposing an engine. No best-effort partial load.
pub fn load_dataset_json(json: &str) -> Result<LoadedDataset, DatasetLoadError> {
    let document: DatasetDocument = serde_json::from_str(json).map_err(DatasetLoadError::Json)?;
    let engine = validate_dataset(&document).map_err(DatasetLoadError::Invalid)?;
    Ok(LoadedDataset { document, engine })
}

/// Validate an in-memory document without a JSON round trip.
pub fn validate_dataset(document: &DatasetDocument) -> Result<Engine, Vec<ValidationIssue>> {
    let mut issues = crate::validation::set_issues(&document.profile_set);
    let mut issue = |field: String, message: &str| {
        issues.push(ValidationIssue {
            field,
            message: message.into(),
        })
    };
    if document.format_version != DATASET_FORMAT_VERSION {
        issue("format_version".into(), "unsupported dataset format");
    }
    for (field, text) in [
        ("description", &document.description),
        ("license", &document.license),
    ] {
        if text.trim().is_empty() {
            issue(field.into(), "must not be empty");
        }
    }
    let check_catalog =
        |name: &str, entries: Vec<(&Identity, &str)>, issue: &mut dyn FnMut(String, &str)| {
            let mut ids = BTreeSet::new();
            for (i, (identity, label)) in entries.into_iter().enumerate() {
                if identity.id.trim().is_empty()
                    || identity.revision.trim().is_empty()
                    || label.trim().is_empty()
                {
                    issue(
                        format!("{name}[{i}]"),
                        "identity/revision/label must not be empty",
                    );
                }
                if !ids.insert(&identity.id) {
                    issue(format!("{name}[{i}].identity"), "duplicate catalog ID");
                }
            }
        };
    check_catalog(
        "machines",
        document
            .machines
            .iter()
            .map(|v| (&v.identity, v.label.as_str()))
            .collect(),
        &mut issue,
    );
    check_catalog(
        "materials",
        document
            .materials
            .iter()
            .map(|v| (&v.identity, v.label.as_str()))
            .collect(),
        &mut issue,
    );
    check_catalog(
        "tools",
        document
            .tools
            .iter()
            .map(|v| (&v.identity, v.label.as_str()))
            .collect(),
        &mut issue,
    );
    for (i, tool) in document.tools.iter().enumerate() {
        if !tool.diameter_mm.is_finite()
            || tool.diameter_mm <= 0.0
            || tool.flute_count == 0
            || tool.max_rpm == Some(0)
        {
            issue(format!("tools[{i}]"), "invalid geometry or maximum RPM");
        }
    }
    let machine_ids: BTreeSet<_> = document.machines.iter().map(|v| &v.identity).collect();
    let material_ids: BTreeSet<_> = document.materials.iter().map(|v| &v.identity).collect();
    for (name, families, catalog) in [
        (
            "machine_families",
            &document.profile_set.machine_families,
            &machine_ids,
        ),
        (
            "material_families",
            &document.profile_set.material_families,
            &material_ids,
        ),
    ] {
        for (i, family) in families.iter().enumerate() {
            for member in &family.members {
                if !catalog.contains(member) {
                    issue(
                        format!("profile_set.{name}[{i}]"),
                        "family member absent from catalog",
                    );
                }
            }
        }
    }
    for (i, profile) in document.profile_set.profiles.iter().enumerate() {
        for (name, selector, catalog) in [
            ("machine", &profile.scope.machine, &machine_ids),
            ("material", &profile.scope.material, &material_ids),
        ] {
            if let IdentitySelector::Exact { identity } = selector {
                if !catalog.contains(identity) {
                    issue(
                        format!("profile_set.profiles[{i}].scope.{name}"),
                        "identity/revision absent from catalog",
                    );
                }
            }
        }
        if let Some(identity) = &profile.scope.tool {
            match document
                .tools
                .iter()
                .find(|tool| &tool.identity == identity)
            {
                None => issue(
                    format!("profile_set.profiles[{i}].scope.tool"),
                    "identity/revision absent from catalog",
                ),
                Some(tool) => {
                    if tool.cutting_family != profile.scope.cutting_family
                        || profile.scope.flute_count != Some(tool.flute_count)
                        || profile.scope.diameter_mm.min != tool.diameter_mm
                        || profile.scope.diameter_mm.max != tool.diameter_mm
                    {
                        issue(
                            format!("profile_set.profiles[{i}].scope"),
                            "exact tool scope must equal catalog geometry/flutes",
                        );
                    }
                    // A preferred target is not a hard limit. Refuse contradictory
                    // authored data rather than importing a target as a maximum.
                    let rpm = match profile.data {
                        CuttingData::ChiploadTarget { rpm, .. }
                        | CuttingData::ExactValues { rpm, .. } => rpm,
                    };
                    if tool.max_rpm.is_some_and(|max| rpm > f64::from(max)) {
                        issue(
                            format!("profile_set.profiles[{i}].data.rpm"),
                            "target exceeds declared catalog tool maximum",
                        );
                    }
                }
            }
        }
    }
    if !issues.is_empty() {
        return Err(issues);
    }
    Engine::new(document.profile_set.clone())
}
