//! Explicit trial application. Never calls the recommendation resolver.
use super::{
    cam_request, saved_rates, AppliedCutting, AppliedRateSlot, CuttingOrigin, FsCamRequest,
    FsCamResponse, FsPass, RateOrigins,
};
use crate::project::{Op, OpKind, PassKind, Project, ToolKind};
use ivac_feeds_speeds as fs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrialTarget {
    pub machine: fs::Identity,
    pub material: fs::Identity,
    pub tool: fs::Identity,
    pub tool_id: u32,
    pub operation: fs::OperationKind,
    pub diameter_mm: f64,
    pub flute_count: u8,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrialValues {
    pub rpm: u32,
    pub feed_mm_min: u32,
    pub plunge_mm_min: u32,
    pub doc_mm: f64,
    pub stepover_ratio: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TrialKind {
    TheoreticalUntested,
}
/// Portable allowlist: no source bodies, labels, notes, URLs or filesystem paths.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExperimentalRecipe {
    pub kind: TrialKind,
    pub source: fs::SelectedRecord,
    pub target: TrialTarget,
    pub applied_values: TrialValues,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrialPreview {
    pub operation: Op,
    pub context_differences: Vec<String>,
}
fn identity(id: &fs::Identity) -> Result<(), String> {
    // IDs are opaque, but local paths and arbitrary prose are not portable identities.
    for s in [&id.id, &id.revision] {
        if s.is_empty()
            || s.len() > 256
            || s.starts_with('/')
            || s.contains('\\')
            || s.contains(':')
            || s.chars().any(char::is_whitespace)
        {
            return Err(
                "Trial identities must be nonempty portable identifiers, not paths or notes".into(),
            );
        }
    }
    Ok(())
}
fn target(project: &Project, op: &Op) -> Result<TrialTarget, String> {
    let bindings = project
        .fs_context
        .as_ref()
        .map_or(&[][..], |c| c.tools.as_slice());
    let unique: std::collections::BTreeSet<_> = bindings.iter().map(|b| b.tool_id).collect();
    if unique.len() != bindings.len()
        || project.tools.iter().filter(|t| t.id == op.tool_id).count() != 1
    {
        return Err("Ambiguous tool identity/bindings; repair the project before trial use".into());
    }

    if op.finish_tool_id.is_some_and(|id| id != op.tool_id) {
        return Err(
            "Trial recipes support one cutter only; remove the distinct finish tool".into(),
        );
    }
    let r = cam_request(
        project,
        op,
        fs::Identity {
            id: "trial".into(),
            revision: "1".into(),
        },
        fs::Aggressiveness::Prudent,
    )?;
    let issues = fs::request_issues(&r);
    if !issues.is_empty() {
        return Err(format!("Invalid trial setup: {issues:?}"));
    }
    let tool = project
        .tools
        .iter()
        .find(|t| t.id == op.tool_id)
        .ok_or("Assigned cutter missing")?;
    if tool.kind != ToolKind::Endmill || tool.whirl {
        return Err("Trial recipes require a standard flat endmill without whirling".into());
    }
    let out = TrialTarget {
        machine: r
            .machine
            .local_identity
            .or(r.machine.identity)
            .ok_or("Assign a machine identity before trial use")?,
        material: r
            .material
            .identity
            .ok_or("Select a material before trial use")?,
        tool: r
            .tool
            .local_identity
            .or(r.tool.identity)
            .ok_or("Assign a cutter identity before trial use")?,
        tool_id: op.tool_id,
        operation: r.operation.kind,
        diameter_mm: r.tool.diameter_mm,
        flute_count: r.tool.flute_count,
    };
    for id in [&out.machine, &out.material, &out.tool] {
        identity(id)?;
    }
    if !out.diameter_mm.is_finite() || out.diameter_mm <= 0.0 || out.flute_count == 0 {
        return Err("Trial cutter diameter and flute count must be positive".into());
    }
    Ok(out)
}
fn validate_values(project: &Project, op: &Op, v: &TrialValues) -> Result<(), String> {
    let tool = project
        .tools
        .iter()
        .find(|t| t.id == op.tool_id)
        .ok_or("Assigned cutter missing")?;
    let binding = project
        .fs_context
        .as_ref()
        .and_then(|c| c.tools.iter().find(|b| b.tool_id == op.tool_id));
    let min = project
        .machine
        .spindle_rpm_min
        .filter(|v| *v > 0)
        .ok_or("Set the machine minimum RPM before trial use")?;
    let max = project
        .machine
        .spindle_rpm_max
        .filter(|v| *v > 0)
        .ok_or("Set the machine maximum RPM before trial use")?;
    let tool_max = binding
        .and_then(|b| b.max_rpm)
        .filter(|v| *v > 0)
        .ok_or("Set the cutter maximum RPM before trial use")?;
    let feed_max = project
        .machine
        .max_feed_mm_min
        .filter(|v| *v > 0)
        .ok_or("Set the machine feed ceiling before trial use")?;
    let flute = tool
        .flute_length_mm
        .filter(|v| v.is_finite() && *v > 0.0)
        .ok_or("Set the cutter flute length before trial use")?;
    if min > max || v.rpm < min || v.rpm > max.min(tool_max) {
        return Err(
            "Trial RPM is outside machine/cutter limits; edit the Draft explicitly (no clamping)"
                .into(),
        );
    }
    // The existing pipeline applies this ceiling to both rates. It is not evidence
    // of an independently qualified axial limit.
    if v.feed_mm_min == 0
        || v.plunge_mm_min == 0
        || v.feed_mm_min > feed_max
        || v.plunge_mm_min > feed_max
    {
        return Err("Trial feed/plunge must be positive and within the configured feed ceiling (no clamping)".into());
    }
    if !v.doc_mm.is_finite() || v.doc_mm <= 0.0 || v.doc_mm > flute {
        return Err(
            "Trial depth/pass must be positive and no greater than cutter flute length".into(),
        );
    }
    if v.stepover_ratio
        .is_some_and(|s| !s.is_finite() || s <= 0.0 || s >= 1.0)
        || (matches!(op.kind, OpKind::Pocket { .. }) && v.stepover_ratio.is_none())
    {
        return Err(
            "Pocket trial requires an explicit stepover strictly between 0 and 100%".into(),
        );
    }
    Ok(())
}
fn rate(v: f64) -> Result<u32, String> {
    if !v.is_finite() || v <= 0.0 || v > f64::from(u32::MAX) || v.fract() != 0.0 {
        return Err("Trial RPM/feed/plunge must be positive integer values; fractional values are not rounded".into());
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(v as u32)
}
/// Invoked again for direct API/loaded projects, independent of installed packs.
pub(super) fn validate_saved(project: &Project, op: &Op, a: &AppliedCutting) -> Result<(), String> {
    let Some(recipe) = &a.experimental_recipe else {
        return Ok(());
    };
    let current = target(project, op)?;
    if current != recipe.target {
        return Err("Experimental target changed; review and apply the trial again".into());
    }
    identity(&recipe.source.pack)?;
    identity(&recipe.source.profile)?;
    identity(&fs::Identity {
        id: recipe.source.record_id.clone(),
        revision: "1".into(),
    })?;
    if !fs::is_lab_pack_id(&recipe.source.pack.id) || a.proposal.is_some() || a.knowledge.is_some()
    {
        return Err(
            "Experimental provenance must identify a local Draft, not a recommendation".into(),
        );
    }
    validate_values(project, op, &recipe.applied_values)?;
    if a.slots.len() != 2
        || op.params.feed_rate_override.is_some()
        || op.params.plunge_rate_override.is_some()
    {
        return Err("Experimental operations require exactly two explicit cutting roles without rate overrides".into());
    }
    let r = cam_request(
        project,
        op,
        recipe.source.pack.clone(),
        fs::Aggressiveness::Prudent,
    )?;
    for pass in [PassKind::Rough, PassKind::Finish] {
        let tool = project
            .tools
            .iter()
            .find(|t| t.id == op.tool_id)
            .ok_or("Assigned cutter missing")?;
        let (rpm, plunge_mm_min, feed_mm_min) = saved_rates(op, tool, pass)?;
        let stepover_ratio = op.pocket_params().map(|p| 1.0 - p.xy_overlap);
        validate_values(
            project,
            op,
            &TrialValues {
                rpm,
                feed_mm_min,
                plunge_mm_min,
                doc_mm: r
                    .operation
                    .doc_mm
                    .ok_or("Explicit trial depth/pass required")?,
                stepover_ratio,
            },
        )?;
    }
    Ok(())
}
fn selected_draft<'a>(
    collection: &'a fs::PackCollection,
    selected: &fs::SelectedRecord,
) -> Result<&'a fs::Profile, String> {
    let matches: Vec<_> = collection
        .source_packs
        .iter()
        .filter(|p| p.manifest.identity == selected.pack)
        .collect();
    if matches.len() != 1 {
        return Err("Draft source missing, ambiguous or revised; refresh the preview".into());
    }
    let pack = matches[0];
    fs::validate_source_pack(pack)?;
    let record = pack
        .records
        .iter()
        .find(|r| r.source_record_id == selected.record_id && r.profile == selected.profile)
        .ok_or("Draft record missing or revised")?;
    let lab = record
        .lab
        .as_ref()
        .ok_or("Only local Draft recipes support trial use")?;
    if lab.state != fs::LabState::Draft || lab.deleted || lab.enabled {
        return Err("Only inactive, non-deleted local Drafts support trial use".into());
    }
    let profile = pack
        .dataset
        .profile_set
        .profiles
        .iter()
        .find(|p| p.identity == selected.profile)
        .ok_or("Draft profile missing")?;
    if profile.scope.cutting_family != fs::CuttingFamily::FlatEndmill
        || !profile.reductions.is_empty()
    {
        return Err("Trial requires a flat-endmill recipe without automatic reductions".into());
    }
    Ok(profile)
}
fn context_differences(profile: &fs::Profile, target: &TrialTarget) -> Vec<String> {
    let name = |id: &fs::Identity| format!("{} @{}", id.id, id.revision);
    let selector = |s: &fs::IdentitySelector| match s {
        fs::IdentitySelector::Any => "Unspecified".to_owned(),
        fs::IdentitySelector::Exact { identity } => name(identity),
        fs::IdentitySelector::Family { id } => format!("Family: {id}"),
    };
    vec![
        format!(
            "Machine — source: {}; target: {}",
            selector(&profile.scope.machine),
            name(&target.machine)
        ),
        format!(
            "Material — source: {}; target: {}",
            selector(&profile.scope.material),
            name(&target.material)
        ),
        format!(
            "Cutter — source: {}; target: {}",
            profile
                .scope
                .tool
                .as_ref()
                .map_or_else(|| "Unspecified".to_owned(), name),
            name(&target.tool)
        ),
        format!(
            "Diameter — source: {}–{} mm; target: {} mm",
            profile.scope.diameter_mm.min, profile.scope.diameter_mm.max, target.diameter_mm
        ),
        format!(
            "Flutes — source: {}; target: {}",
            profile
                .scope
                .flute_count
                .map_or_else(|| "Unspecified".into(), |n| n.to_string()),
            target.flute_count
        ),
    ]
}
/// Explicit preview/apply path; Draft eligibility in Suggest is never changed.
pub(super) fn trial(request: &FsCamRequest) -> Result<FsCamResponse, String> {
    let selected = request
        .selected_record
        .as_ref()
        .ok_or("Select a local Draft for trial use")?;
    let collection: fs::PackCollection =
        serde_json::from_str(&request.dataset_json).map_err(|e| e.to_string())?;
    let profile = selected_draft(&collection, selected)?;
    let op = request
        .project
        .operations
        .iter()
        .find(|o| o.id == request.op_id)
        .ok_or("Operation missing")?;
    let target = target(&request.project, op)?;
    if profile.scope.operation != target.operation {
        return Err(
            "Draft process differs from the target; author an explicit recipe for this operation"
                .into(),
        );
    }
    let fs::CuttingData::ExactValues {
        rpm,
        feed_mm_min,
        plunge_mm_min,
        doc_mm,
        stepover_ratio,
    } = profile.data
    else {
        return Err(
            "Author an explicit feed in the Draft; trial use does not calculate chipload targets"
                .into(),
        );
    };
    let values = TrialValues {
        rpm: rate(rpm)?,
        feed_mm_min: rate(feed_mm_min)?,
        plunge_mm_min: rate(plunge_mm_min)?,
        doc_mm,
        stepover_ratio,
    };
    validate_values(&request.project, op, &values)?;
    if !op.params.depth_list.is_empty() {
        return Err("Remove the explicit depth schedule before applying a trial depth/pass".into());
    }
    let mut next = op.clone();
    next.params.step = Some(-doc_mm);
    next.params.feed_rate_override = None;
    next.params.plunge_rate_override = None;
    if let Some(p) = next.pocket_params_mut() {
        p.xy_overlap = 1.0 - stepover_ratio.ok_or("Explicit pocket stepover required")?;
    }
    let differences = context_differences(profile, &target);
    next.params.applied_cutting = Some(AppliedCutting {
        slots: [FsPass::Rough, FsPass::Finish]
            .into_iter()
            .map(|pass| AppliedRateSlot {
                tool_id: op.tool_id,
                pass,
                rpm: values.rpm,
                feed_mm_min: values.feed_mm_min,
                plunge_mm_min: values.plunge_mm_min,
                origins: RateOrigins {
                    rpm: CuttingOrigin::Manual,
                    feed: CuttingOrigin::Manual,
                    plunge: CuttingOrigin::Manual,
                },
            })
            .collect(),
        proposal: None,
        knowledge: None,
        context_key: String::new(),
        doc_origin: CuttingOrigin::Manual,
        stepover_origin: stepover_ratio.map(|_| CuttingOrigin::Manual),
        experimental_recipe: Some(ExperimentalRecipe {
            kind: TrialKind::TheoreticalUntested,
            source: selected.clone(),
            target,
            applied_values: values,
        }),
    });
    validate_saved(
        &request.project,
        &next,
        next.params
            .applied_cutting
            .as_ref()
            .ok_or("Missing trial values")?,
    )?;
    if request.apply && !request.trial_acknowledged {
        return Err(
            "Acknowledge that trial values are theoretical and untested before applying".into(),
        );
    }
    Ok(FsCamResponse {
        recommendation: fs::FeedsSpeedsRecommendation::Unavailable {
            reason: fs::UnavailableReason::MissingCuttingData,
        },
        knowledge: None,
        scope_rejections: vec![],
        applied_op: request.apply.then(|| next.clone()),
        trial: Some(TrialPreview {
            operation: next,
            context_differences: differences,
        }),
    })
}
