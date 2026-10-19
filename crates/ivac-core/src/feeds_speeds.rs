//! Deliberate milling proposal/application adapter. Generation only reads saved rates.
use crate::project::{MachineMode, Op, OpKind, PassKind, Project, ToolEntry, ToolKind};
use ivac_feeds_speeds as fs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FsContext {
    pub machine_identity: Option<fs::Identity>,
    pub tools: Vec<FsToolBinding>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FsToolBinding {
    pub tool_id: u32,
    pub identity: fs::Identity,
    pub max_rpm: Option<u32>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FsPass {
    Rough,
    Finish,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CuttingOrigin {
    Recommendation,
    Manual,
    LegacySnapshot,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RateOrigins {
    pub rpm: CuttingOrigin,
    pub feed: CuttingOrigin,
    pub plunge: CuttingOrigin,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AppliedRateSlot {
    pub tool_id: u32,
    pub pass: FsPass,
    pub rpm: u32,
    pub feed_mm_min: u32,
    pub plunge_mm_min: u32,
    pub origins: RateOrigins,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AppliedCutting {
    pub slots: Vec<AppliedRateSlot>,
    /// Optional evidence snapshot; manually entered operation rates need no proposal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposal: Option<fs::Recommendation>,
    /// Saved native context after Apply; never used to overwrite parameters.
    pub context_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knowledge: Option<fs::KnowledgeSummary>,
    pub doc_origin: CuttingOrigin,
    pub stepover_origin: Option<CuttingOrigin>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FsCamRequest {
    pub project: Project,
    pub op_id: u32,
    pub dataset_json: String,
    pub aggressiveness: fs::Aggressiveness,
    /// False previews only; true explicitly applies. Never called by generation.
    pub apply: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCamResponse {
    pub recommendation: fs::FeedsSpeedsRecommendation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knowledge: Option<fs::KnowledgeExplanation>,
    /// Native op with authoritative geometry/rates only on explicit Apply.
    pub applied_op: Option<Op>,
    /// Optional transient explanation; never persisted with cutting parameters.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope_rejections: Vec<fs::ProfileRejection>,
}

pub fn cam_request(
    project: &Project,
    op: &Op,
    dataset: fs::Identity,
    aggressiveness: fs::Aggressiveness,
) -> Result<fs::FeedsSpeedsRequest, String> {
    if project.machine.mode != MachineMode::Mill {
        return Err("Feeds & speeds requires milling mode".into());
    }
    let tool = project
        .tools
        .iter()
        .find(|t| t.id == op.tool_id)
        .ok_or("Operation tool is missing")?;
    let binding = project
        .fs_context
        .as_ref()
        .and_then(|c| c.tools.iter().find(|t| t.tool_id == tool.id));
    let (kind, engagement) = match &op.kind {
        OpKind::Profile { .. } if op.params.fs_full_width => {
            (fs::OperationKind::FullSlot, fs::Engagement::KnownFullWidth)
        }
        OpKind::Profile { .. } => (fs::OperationKind::Profile, fs::Engagement::Unknown),
        OpKind::Pocket { pocket, .. } => {
            let ratio = 1.0
                - if pocket.xy_overlap > 0.0 {
                    pocket.xy_overlap
                } else {
                    tool.default_xy_overlap.filter(|v| *v > 0.0).unwrap_or(0.5)
                };
            (fs::OperationKind::Pocket, fs::Engagement::Nominal { ratio })
        }
        _ => return Err("This operation is not supported by the milling assistant".into()),
    };
    let doc_mm = if op.params.depth_list.is_empty() {
        crate::pipeline::effective_step(op, tool).ok().map(f64::abs)
    } else {
        let mut previous = op.params.start_depth;
        let mut maximum: f64 = 0.0;
        for depth in &op.params.depth_list {
            let increment = previous - depth;
            if !increment.is_finite() || increment <= 0.0 {
                return Err("Depth list must descend in finite steps".into());
            }
            maximum = maximum.max(increment);
            previous = *depth;
        }
        Some(maximum)
    };
    let entry = match op.params.plunge {
        crate::project::PlungeStrategy::Direct => fs::EntryStrategy::Direct,
        crate::project::PlungeStrategy::Ramp { .. } => fs::EntryStrategy::Ramp,
        crate::project::PlungeStrategy::Helix { .. } => fs::EntryStrategy::Helix,
    };
    Ok(fs::FeedsSpeedsRequest {
        schema_version: fs::DOMAIN_SCHEMA_VERSION,
        dataset,
        machine: fs::MachineSnapshot {
            identity: project
                .fs_context
                .as_ref()
                .and_then(|v| v.machine_identity.clone()),
            rpm_min: project.machine.spindle_rpm_min,
            rpm_max: project.machine.spindle_rpm_max,
            max_feed_mm_min: project.machine.max_feed_mm_min,
            // XY feed limit is not evidence of an axial plunge limit.
            max_plunge_mm_min: None,
        },
        material: fs::MaterialSnapshot {
            identity: project.material.clone(),
            thickness_mm: project.stock.as_ref().map(|v| v.thickness_mm),
        },
        tool: fs::ToolSnapshot {
            identity: binding.map(|b| b.identity.clone()),
            cutting_family: if tool.kind == ToolKind::Endmill {
                fs::CuttingFamily::FlatEndmill
            } else {
                fs::CuttingFamily::Other
            },
            diameter_mm: tool.effective_diameter(),
            flute_count: tool.flutes,
            max_rpm: binding.and_then(|b| b.max_rpm),
        },
        operation: fs::OperationContext {
            kind,
            doc_mm,
            engagement,
            entry,
        },
        aggressiveness,
    })
}

/// Stable native context snapshot; includes source/geometry and relevant tools/limits.
/// Dataset availability/revision never participates in generation.
#[must_use]
pub fn context_key(project: &Project, op: &Op) -> String {
    let mut params = op.params.clone();
    params.applied_cutting = None;
    let tools: Vec<_> = project
        .tools
        .iter()
        .filter(|t| t.id == op.tool_id || Some(t.id) == op.finish_tool_id)
        .collect();
    serde_json::to_string(&(
        op.tool_id,
        op.finish_tool_id,
        &op.source,
        &op.kind,
        params,
        &project.machine,
        &project.material,
        &project.fs_context,
        &project.stock,
        tools,
    ))
    .expect("serializable context")
}

/// Applied slots replace tool defaults completely; legacy overrides only apply when absent.
pub fn saved_rates(op: &Op, tool: &ToolEntry, pass: PassKind) -> Result<(u32, u32, u32), String> {
    if let Some(applied) = &op.params.applied_cutting {
        let role = match pass {
            PassKind::Rough => FsPass::Rough,
            PassKind::Finish => FsPass::Finish,
            PassKind::Drill => {
                return Err("Applied milling rates cannot be used for drilling".into())
            }
        };
        let matches: Vec<_> = applied
            .slots
            .iter()
            .filter(|s| s.tool_id == tool.id && s.pass == role)
            .collect();
        if matches.len() != 1 {
            return Err(
                "Applied rate role missing or duplicated; apply or edit all tool roles explicitly"
                    .into(),
            );
        }
        let s = matches[0];
        if s.rpm == 0 || s.feed_mm_min == 0 || s.plunge_mm_min == 0 {
            return Err("Applied RPM/feed/plunge must be positive".into());
        }
        return Ok((s.rpm, s.plunge_mm_min, s.feed_mm_min));
    }
    let (rpm, plunge, feed) = crate::project::resolve_tool_rates(tool, pass);
    Ok((
        rpm,
        op.params.plunge_rate_override.unwrap_or(plunge),
        op.params.feed_rate_override.unwrap_or(feed),
    ))
}

// owned argument is the transport/command contract.
#[allow(clippy::needless_pass_by_value)]
pub fn feeds_speeds(request: FsCamRequest) -> Result<FsCamResponse, String> {
    if request.dataset_json.len() > 16 * 1024 * 1024 {
        return Err("Enabled source data exceeds 16 MiB".into());
    }
    let op = request
        .project
        .operations
        .iter()
        .find(|o| o.id == request.op_id)
        .ok_or("Operation missing")?;
    let value: serde_json::Value =
        serde_json::from_str(&request.dataset_json).map_err(|e| e.to_string())?;
    let (r, recommendation, scope_rejections, knowledge) = if value.get("source_packs").is_some() {
        let collection: fs::PackCollection =
            serde_json::from_value(value).map_err(|e| e.to_string())?;
        let r = cam_request(
            &request.project,
            op,
            fs::Identity {
                id: "cutting-knowledge".into(),
                revision: "1".into(),
            },
            request.aggressiveness,
        )?;
        let result = fs::recommend_packs(&collection.source_packs, &r)?;
        (
            r,
            result.recommendation,
            result.scope_rejections,
            result.explanation,
        )
    } else {
        let loaded = fs::load_dataset_json(&request.dataset_json).map_err(|e| e.to_string())?;
        let r = cam_request(
            &request.project,
            op,
            loaded.document().profile_set.identity.clone(),
            request.aggressiveness,
        )?;
        let recommendation = loaded.engine().recommend(&r);
        let rejections = if matches!(
            recommendation,
            fs::FeedsSpeedsRecommendation::Unavailable {
                reason: fs::UnavailableReason::MissingCuttingData
            }
        ) {
            loaded.engine().scope_rejections(&r)
        } else {
            vec![]
        };
        (r, recommendation, rejections, None)
    };
    let applied_op = if request.apply {
        let fs::FeedsSpeedsRecommendation::Available {
            recommendation: proposal,
        } = &recommendation
        else {
            return Err("No matching recommendation to apply".into());
        };
        let mut next = op.clone();
        // An explicit schedule wins in CAM. Never silently replace it with a step.
        if !next.params.depth_list.is_empty()
            && r.operation.doc_mm != Some(proposal.parameters.doc_mm)
        {
            return Err("Explicit depth schedule differs from recommended DOC; edit the schedule explicitly before Apply".into());
        }
        next.params.step = Some(-proposal.parameters.doc_mm);
        if let Some(ratio) = proposal.parameters.stepover_ratio {
            if let Some(pocket) = next.pocket_params_mut() {
                pocket.xy_overlap = 1.0 - ratio;
            }
        }
        let mut slots = Vec::new();
        let mut ids = vec![op.tool_id];
        if let Some(id) = op.finish_tool_id.filter(|id| *id != op.tool_id) {
            ids.push(id);
        }
        for id in ids {
            let tool = request
                .project
                .tools
                .iter()
                .find(|t| t.id == id)
                .ok_or("Finish tool missing")?;
            for (pass, role) in [
                (PassKind::Rough, FsPass::Rough),
                (PassKind::Finish, FsPass::Finish),
            ] {
                let main = id == op.tool_id
                    && (!matches!(op.kind, OpKind::Pocket { .. }) || role == FsPass::Rough);
                if !main {
                    if let Some(previous) =
                        op.params.applied_cutting.as_ref().and_then(|a| {
                            a.slots.iter().find(|s| s.tool_id == id && s.pass == role)
                        })
                    {
                        slots.push(previous.clone());
                        continue;
                    }
                }
                // Explicit Apply snapshots unqualified secondary roles; they are
                // labelled legacy/manual, never promoted to recommendation evidence.
                let (rpm, plunge, feed) = if op.params.applied_cutting.is_none() {
                    saved_rates(op, tool, pass)?
                } else {
                    crate::project::resolve_tool_rates(tool, pass)
                };
                let origin = if main {
                    CuttingOrigin::Recommendation
                } else {
                    CuttingOrigin::LegacySnapshot
                };
                slots.push(AppliedRateSlot {
                    tool_id: id,
                    pass: role,
                    rpm: if main { proposal.parameters.rpm } else { rpm },
                    feed_mm_min: if main {
                        proposal.parameters.feed_mm_min
                    } else {
                        feed
                    },
                    plunge_mm_min: if main {
                        proposal.parameters.plunge_mm_min
                    } else {
                        plunge
                    },
                    origins: RateOrigins {
                        rpm: origin,
                        feed: origin,
                        plunge: origin,
                    },
                });
            }
        }
        next.params.feed_rate_override = None;
        next.params.plunge_rate_override = None;
        let key = context_key(&request.project, &next);
        next.params.applied_cutting = Some(AppliedCutting {
            slots,
            proposal: if knowledge.as_ref().is_some_and(|k| k.summary.local_only) {
                None
            } else {
                Some((**proposal).clone())
            },
            knowledge: knowledge.as_ref().map(|k| k.summary.clone()),
            context_key: key,
            doc_origin: CuttingOrigin::Recommendation,
            stepover_origin: proposal
                .parameters
                .stepover_ratio
                .map(|_| CuttingOrigin::Recommendation),
        });
        let mut checked = request.project.clone();
        if let Some(target) = checked.operations.iter_mut().find(|o| o.id == next.id) {
            *target = next.clone();
        }
        validate_applied_project(&checked).map_err(|e| e.to_string())?;
        Some(next)
    } else {
        None
    };
    Ok(FsCamResponse {
        recommendation,
        knowledge,
        applied_op,
        scope_rejections,
    })
}

pub(crate) fn effective_machine(project: &Project, tool_id: u32) -> crate::project::MachineConfig {
    let mut machine = project.machine.clone();
    if let Some(max) = project
        .fs_context
        .as_ref()
        .and_then(|c| c.tools.iter().find(|b| b.tool_id == tool_id))
        .and_then(|b| b.max_rpm)
    {
        machine.spindle_rpm_max = Some(machine.spindle_rpm_max.map_or(max, |m| m.min(max)));
    }
    machine
}

pub(crate) fn validate_applied_project(
    project: &Project,
) -> Result<(), crate::pipeline::PipelineError> {
    use crate::pipeline::PipelineError;
    if let Some(context) = &project.fs_context {
        let mut ids = std::collections::BTreeSet::new();
        for binding in &context.tools {
            if !ids.insert(binding.tool_id) || binding.max_rpm == Some(0) {
                return Err(PipelineError::AppliedCutting(
                    0,
                    "Duplicate tool bindings or invalid tool maximum RPM".into(),
                ));
            }
        }
    }
    for op in project
        .operations
        .iter()
        .filter(|o| o.enabled && o.params.applied_cutting.is_some())
    {
        if project.machine.mode != MachineMode::Mill
            || !matches!(
                op.kind,
                OpKind::Profile { .. } | OpKind::Pocket { .. } | OpKind::PlanarSurface { .. }
            )
        {
            return Err(PipelineError::AppliedCutting(
                op.id,
                "Saved milling rates are incompatible with this process/operation".into(),
            ));
        }
        let mut ids = vec![op.tool_id];
        if let Some(id) = op.finish_tool_id.filter(|id| *id != op.tool_id) {
            ids.push(id);
        }
        for id in ids {
            let tool = project
                .tools
                .iter()
                .find(|t| t.id == id)
                .ok_or(PipelineError::UnknownTool(op.id, id))?;
            if tool.kind != ToolKind::Endmill
                && !(matches!(op.kind, OpKind::PlanarSurface { .. })
                    && tool.kind == ToolKind::Compression)
            {
                return Err(PipelineError::AppliedCutting(
                    op.id,
                    "Review applied values after changing cutter family".into(),
                ));
            }
            for pass in [PassKind::Rough, PassKind::Finish] {
                saved_rates(op, tool, pass).map_err(|e| PipelineError::AppliedCutting(op.id, e))?;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FsLabCapture {
    pub request: fs::FeedsSpeedsRequest,
    pub rpm: u32,
    pub feed_mm_min: u32,
    pub plunge_mm_min: u32,
    pub machine_name: String,
    pub tool_name: String,
    pub operation_name: String,
    pub finishing_context: String,
}
/// Read-only capture of the same rates and context consumed by generation.
pub fn capture_lab_preset(project: &Project, op_id: u32) -> Result<FsLabCapture, String> {
    let op = project
        .operations
        .iter()
        .find(|op| op.id == op_id)
        .ok_or("Operation is missing")?;
    let request = cam_request(
        project,
        op,
        fs::Identity {
            id: fs::LAB_PACK_ID.into(),
            revision: "capture".into(),
        },
        op.params
            .applied_cutting
            .as_ref()
            .and_then(|a| a.proposal.as_ref())
            .map_or(fs::Aggressiveness::Prudent, |p| p.aggressiveness),
    )?;
    if request.machine.identity.is_none()
        || request.material.identity.is_none()
        || request.tool.identity.is_none()
    {
        return Err("Set stable machine, material and tool identities in Machine, Stock and Tool library before saving a reusable lab preset".into());
    }
    let tool = project
        .tools
        .iter()
        .find(|t| t.id == op.tool_id)
        .ok_or("Tool is missing")?;
    let pass = if matches!(op.kind, OpKind::Profile { .. }) {
        PassKind::Finish
    } else {
        PassKind::Rough
    };
    let (rpm, plunge_mm_min, feed_mm_min) = saved_rates(op, tool, pass)?;
    let finishing_context = if matches!(op.kind, OpKind::Pocket { .. }) {
        let finish_tool = project
            .tools
            .iter()
            .find(|t| t.id == op.finish_tool_id.unwrap_or(op.tool_id))
            .ok_or("Finish tool is missing")?;
        let rates = saved_rates(op, finish_tool, PassKind::Finish)?;
        format!("Primary pocket roughing preset; secondary finish context only: tool #{}; RPM {}; plunge {}; feed {}. Radial finish engagement is not inferred.", finish_tool.id, rates.0, rates.1, rates.2)
    } else {
        "Profile cutting role; no separate finish engagement inferred".into()
    };
    Ok(FsLabCapture {
        request,
        rpm,
        feed_mm_min,
        plunge_mm_min,
        machine_name: project.machine.name.clone(),
        tool_name: tool.name.clone(),
        operation_name: op.name.clone(),
        finishing_context,
    })
}

pub fn validate_source_pack_json(json: &str) -> Result<fs::SourcePack, String> {
    fs::load_source_pack_json(json)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::pipeline::{run_pipeline, PipelineRequest};
    const DATA: &str = include_str!("../../../data/feeds-speeds/sienci-router-reference.v1.json");
    fn fixture() -> Project {
        let d = fs::load_dataset_json(DATA).unwrap();
        let doc = d.document();
        let mut tool = ToolEntry::default();
        tool.id = 1;
        tool.kind = ToolKind::Endmill;
        tool.diameter = 6.35;
        tool.flutes = 1;
        let mut op = Op::default();
        op.params.depth = -2.8;
        op.params.step = Some(-2.8);
        op.params.fs_full_width = true;
        Project {
            tools: vec![tool],
            operations: vec![op],
            material: Some(doc.materials[0].identity.clone()),
            fs_context: Some(FsContext {
                machine_identity: Some(doc.machines[0].identity.clone()),
                tools: vec![FsToolBinding {
                    tool_id: 1,
                    identity: doc.tools[0].identity.clone(),
                    max_rpm: None,
                }],
            }),
            ..Project::default()
        }
    }
    #[test]
    fn lab_capture_reads_manual_resolved_rates_without_mutating_operation() {
        let mut project = fixture();
        project.operations[0].params.feed_rate_override = Some(4000);
        project.operations[0].params.plunge_rate_override = Some(900);
        let before = serde_json::to_string(&project).unwrap();
        let captured = capture_lab_preset(&project, project.operations[0].id).unwrap();
        assert_eq!(captured.feed_mm_min, 4000);
        assert_eq!(captured.plunge_mm_min, 900);
        assert_eq!(captured.request.tool.diameter_mm, 6.35);
        assert_eq!(captured.request.operation.doc_mm, Some(2.8));
        assert_eq!(
            captured.request.operation.engagement,
            fs::Engagement::KnownFullWidth
        );
        assert_eq!(serde_json::to_string(&project).unwrap(), before);
    }
    #[test]
    fn lab_capture_preserves_unknown_engagement_and_rejects_missing_identity() {
        let mut project = fixture();
        project.operations[0].params.fs_full_width = false;
        assert_eq!(
            capture_lab_preset(&project, project.operations[0].id)
                .unwrap()
                .request
                .operation
                .engagement,
            fs::Engagement::Unknown
        );
        project.fs_context = None;
        assert!(capture_lab_preset(&project, project.operations[0].id).is_err());
    }
    #[test]
    fn lab_capture_uses_applied_manual_slot_instead_of_original_recommendation() {
        let mut project = fixture();
        let mut op = action(&project, true).applied_op.unwrap();
        op.params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots
            .iter_mut()
            .find(|s| s.pass == FsPass::Finish)
            .unwrap()
            .feed_mm_min = 4000;
        op.params.applied_cutting.as_mut().unwrap().slots[0]
            .origins
            .feed = CuttingOrigin::Manual;
        project.operations[0] = op;
        let c = capture_lab_preset(&project, project.operations[0].id).unwrap();
        assert_eq!(c.feed_mm_min, 4000);
        assert_eq!(c.rpm, 17000);
    }
    fn action(p: &Project, apply: bool) -> FsCamResponse {
        feeds_speeds(FsCamRequest {
            project: p.clone(),
            op_id: 1,
            dataset_json: DATA.into(),
            aggressiveness: fs::Aggressiveness::Prudent,
            apply,
        })
        .unwrap()
    }
    fn apply(p: &mut Project) {
        p.operations[0] = action(p, true).applied_op.unwrap();
    }
    #[test]
    fn approximate_proposal_apply_and_reopen_keep_resolved_values_authoritative() {
        let mut p = fixture();
        p.tools[0].diameter = 6.0;
        p.material = Some(fs::taxon_identity("wood.softwood"));
        p.fs_context.as_mut().unwrap().machine_identity = Some(fs::Identity {
            id: "machine/fablab-router".into(),
            revision: "1".into(),
        });
        p.operations[0].kind = OpKind::Pocket {
            strategy: crate::project::PocketStrategy::Cascade,
            contour: crate::project::ContourParams::default(),
            pocket: crate::project::PocketParams::default(),
        };
        p.operations[0].params.step = Some(-1.0);
        let pack = fs::load_source_pack_json(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v3.json"
        ))
        .unwrap();
        let data = serde_json::to_string(&fs::PackCollection {
            source_packs: vec![pack],
        })
        .unwrap();
        let before = serde_json::to_string(&p).unwrap();
        let preview = feeds_speeds(FsCamRequest {
            project: p.clone(),
            op_id: 1,
            dataset_json: data.clone(),
            aggressiveness: fs::Aggressiveness::Prudent,
            apply: false,
        })
        .unwrap();
        assert!(preview.applied_op.is_none());
        assert_eq!(serde_json::to_string(&p).unwrap(), before);
        let applied = feeds_speeds(FsCamRequest {
            project: p.clone(),
            op_id: 1,
            dataset_json: data,
            aggressiveness: fs::Aggressiveness::Prudent,
            apply: true,
        })
        .unwrap();
        p.operations[0] = applied.applied_op.unwrap();
        p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots[0]
            .feed_mm_min = 1200;
        p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots[0]
            .origins
            .feed = CuttingOrigin::Manual;
        let restored: Project = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        let rates =
            saved_rates(&restored.operations[0], &restored.tools[0], PassKind::Rough).unwrap();
        assert_eq!(rates, (17000, 765, 1200));
        let mut private = fs::load_source_pack_json(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v3.json"
        ))
        .unwrap();
        private.manifest.distribution = fs::Distribution::LocalOnly;
        for profile in &mut private.dataset.profile_set.profiles {
            profile.source.notes = "PRIVATE-DERIVED-SENTINEL".into();
        }
        let result = feeds_speeds(FsCamRequest {
            project: p,
            op_id: 1,
            dataset_json: serde_json::to_string(&fs::PackCollection {
                source_packs: vec![private],
            })
            .unwrap(),
            aggressiveness: fs::Aggressiveness::Prudent,
            apply: true,
        })
        .unwrap();
        let applied = result.applied_op.unwrap();
        let json = serde_json::to_string(&applied).unwrap();
        assert!(!json.contains("PRIVATE-DERIVED-SENTINEL"));
        assert!(applied.params.applied_cutting.unwrap().proposal.is_none());
    }
    #[test]
    fn private_pack_apply_saves_values_and_ids_without_record_body() {
        let p = fixture();
        let mut pack = fs::load_source_pack_json(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
        ))
        .unwrap();
        pack.manifest.distribution = fs::Distribution::LocalOnly;
        for profile in &mut pack.dataset.profile_set.profiles {
            profile.source.notes = "PRIVATE-DATA-SENTINEL".into();
        }
        let result = feeds_speeds(FsCamRequest {
            project: p.clone(),
            op_id: 1,
            dataset_json: serde_json::to_string(&fs::PackCollection {
                source_packs: vec![pack],
            })
            .unwrap(),
            aggressiveness: fs::Aggressiveness::Prudent,
            apply: true,
        })
        .unwrap();
        let next = result.applied_op.unwrap();
        let json = serde_json::to_string(&next).unwrap();
        assert!(!json.contains("PRIVATE-DATA-SENTINEL"));
        assert!(next
            .params
            .applied_cutting
            .as_ref()
            .unwrap()
            .proposal
            .is_none());
        assert!(
            next.params
                .applied_cutting
                .as_ref()
                .unwrap()
                .knowledge
                .as_ref()
                .unwrap()
                .local_only
        );
        // No packs or engine call on reopen; saved roles remain authoritative.
        let restored: Op = serde_json::from_str(&json).unwrap();
        assert_eq!(
            saved_rates(&restored, &p.tools[0], PassKind::Finish).unwrap(),
            (17000, 810, 1620)
        );
    }
    #[test]
    fn public_pack_migration_preserves_single_source_values() {
        let p = fixture();
        let legacy = action(&p, false);
        let pack = fs::load_source_pack_json(include_str!(
            "../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
        ))
        .unwrap();
        let result = feeds_speeds(FsCamRequest {
            project: p,
            op_id: 1,
            dataset_json: serde_json::to_string(&fs::PackCollection {
                source_packs: vec![pack],
            })
            .unwrap(),
            aggressiveness: fs::Aggressiveness::Prudent,
            apply: false,
        })
        .unwrap();
        let fs::FeedsSpeedsRecommendation::Available { recommendation: a } = legacy.recommendation
        else {
            panic!("legacy unavailable")
        };
        let fs::FeedsSpeedsRecommendation::Available { recommendation: b } = result.recommendation
        else {
            panic!("pack unavailable")
        };
        assert_eq!(a.parameters, b.parameters);
        assert!(result.knowledge.is_some());
    }
    #[test]
    fn rejected_scope_is_transient_and_old_responses_remain_readable() {
        let mut p = fixture();
        p.operations[0].params.step = Some(-1.0);
        let before = serde_json::to_string(&p).unwrap();
        let response = action(&p, false);
        assert!(response.applied_op.is_none());
        assert_eq!(serde_json::to_string(&p).unwrap(), before);
        assert!(!response.scope_rejections.is_empty());
        assert!(response
            .scope_rejections
            .iter()
            .all(|row| row.mismatches.iter().any(|m| m.field == "doc_mm")));
        let mut old = serde_json::to_value(&response).unwrap();
        old.as_object_mut().unwrap().remove("scope_rejections");
        let restored: FsCamResponse = serde_json::from_value(old).unwrap();
        assert!(restored.scope_rejections.is_empty());
    }

    #[test]
    fn preview_is_immutable_apply_is_explicit_and_roundtrip_offline() {
        let mut p = fixture();
        let before = serde_json::to_string(&p).unwrap();
        let preview = action(&p, false);
        assert!(preview.applied_op.is_none());
        assert_eq!(serde_json::to_string(&p).unwrap(), before);
        apply(&mut p);
        let restored: Project = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        let op = &restored.operations[0];
        assert_eq!(op.params.step, Some(-2.8));
        assert_eq!(
            saved_rates(op, &restored.tools[0], PassKind::Finish).unwrap(),
            (17000, 810, 1620)
        );
        assert_eq!(
            op.params.applied_cutting.as_ref().unwrap().context_key,
            context_key(&restored, op)
        );
    }
    #[test]
    fn manual_slots_survive_tool_data_changes_and_context_becomes_stale() {
        let mut p = fixture();
        apply(&mut p);
        let slot = &mut p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots[1];
        slot.feed_mm_min = 1500;
        slot.origins.feed = CuttingOrigin::Manual;
        p.tools[0].speed = 999;
        p.tools[0].feed_rate = 999;
        assert_eq!(
            saved_rates(&p.operations[0], &p.tools[0], PassKind::Finish).unwrap(),
            (17000, 810, 1500)
        );
        assert_ne!(
            p.operations[0]
                .params
                .applied_cutting
                .as_ref()
                .unwrap()
                .context_key,
            context_key(&p, &p.operations[0])
        );
    }
    #[test]
    fn missing_duplicate_new_tool_and_wrong_process_roles_refuse() {
        let mut p = fixture();
        apply(&mut p);
        p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots
            .pop();
        assert!(validate_applied_project(&p).is_err());
        p = fixture();
        apply(&mut p);
        let slot = p.operations[0]
            .params
            .applied_cutting
            .as_ref()
            .unwrap()
            .slots[0]
            .clone();
        p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots
            .push(slot);
        assert!(validate_applied_project(&p).is_err());
        p = fixture();
        apply(&mut p);
        p.machine.mode = MachineMode::Drag;
        assert!(validate_applied_project(&p).is_err());
        p = fixture();
        apply(&mut p);
        p.operations[0].tool_id = 2;
        assert!(validate_applied_project(&p).is_err());
    }
    #[test]
    fn native_geometry_context_uses_depth_schedule_and_nominal_overlap() {
        let mut p = fixture();
        let dataset = fs::load_dataset_json(DATA)
            .unwrap()
            .document()
            .profile_set
            .identity
            .clone();
        p.machine.max_feed_mm_min = Some(9000);
        p.operations[0].params.depth_list = vec![-1.0, -2.5, -2.8];
        p.operations[0].params.step = Some(-9.0);
        let r = cam_request(
            &p,
            &p.operations[0],
            dataset.clone(),
            fs::Aggressiveness::Prudent,
        )
        .unwrap();
        assert_eq!(r.operation.doc_mm, Some(1.5));
        assert_eq!(r.machine.max_plunge_mm_min, None);
        assert_eq!(r.machine.max_feed_mm_min, Some(9000));
        p.operations[0].params.fs_full_width = false;
        let r = cam_request(&p, &p.operations[0], dataset, fs::Aggressiveness::Prudent).unwrap();
        assert_eq!(r.operation.engagement, fs::Engagement::Unknown);
        assert_eq!(r.operation.kind, fs::OperationKind::Profile);
    }
    #[test]
    fn legacy_serialization_omits_fs_fields_and_inheritance_survives() {
        let p = Project {
            tools: vec![ToolEntry::default()],
            operations: vec![Op::default()],
            ..Project::default()
        };
        let encoded = serde_json::to_string(&p).unwrap();
        assert!(!encoded.contains("applied_cutting"));
        assert!(!encoded.contains("fs_context"));
        assert!(!encoded.contains("fs_full_width"));
        assert_eq!(
            saved_rates(&p.operations[0], &p.tools[0], PassKind::Finish).unwrap(),
            crate::project::resolve_tool_rates(&p.tools[0], PassKind::Finish)
        );
    }
    #[test]
    fn pipeline_uses_saved_manual_values_and_warns_on_changed_context() {
        use crate::geometry::{Point2, Segment};
        let mut p = fixture();
        p.segments = vec![
            Segment::line(Point2::new(0.0, 0.0), Point2::new(20.0, 0.0), "0", 7),
            Segment::line(Point2::new(20.0, 0.0), Point2::new(20.0, 20.0), "0", 7),
            Segment::line(Point2::new(20.0, 20.0), Point2::new(0.0, 20.0), "0", 7),
            Segment::line(Point2::new(0.0, 20.0), Point2::new(0.0, 0.0), "0", 7),
        ];
        apply(&mut p);
        let slot = &mut p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots[1];
        slot.feed_mm_min = 1500;
        slot.origins.feed = CuttingOrigin::Manual;
        p.tools[0].feed_rate = 999;
        p.tools[0].feed_rate_finish = Some(998);
        let response = run_pipeline(
            PipelineRequest {
                project: p,
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .unwrap();
        assert!(response.gcode.contains("F1500"));
        assert!(!response.gcode.contains("F998"));
        assert!(response
            .warnings
            .iter()
            .any(|w| w.kind == "feeds_speeds_stale"));
    }

    #[test]
    fn manual_only_values_roundtrip_and_generate_without_recommendation_or_data() {
        use crate::geometry::{Point2, Segment};
        let mut p = fixture();
        p.segments = vec![
            Segment::line(Point2::new(0.0, 0.0), Point2::new(20.0, 0.0), "0", 7),
            Segment::line(Point2::new(20.0, 0.0), Point2::new(20.0, 20.0), "0", 7),
            Segment::line(Point2::new(20.0, 20.0), Point2::new(0.0, 20.0), "0", 7),
            Segment::line(Point2::new(0.0, 20.0), Point2::new(0.0, 0.0), "0", 7),
        ];
        apply(&mut p);
        let generate = |project| {
            run_pipeline(
                PipelineRequest {
                    project,
                    post_processor: None,
                    cps_post: None,
                },
                |_, _, _| {},
            )
            .unwrap()
        };
        let before = generate(p.clone());
        let block = p.operations[0].params.applied_cutting.as_mut().unwrap();
        block.proposal = None;
        block.context_key.clear();
        for slot in &mut block.slots {
            slot.origins = RateOrigins {
                rpm: CuttingOrigin::Manual,
                feed: CuttingOrigin::Manual,
                plunge: CuttingOrigin::Manual,
            };
        }
        p.material = None;
        p.fs_context = None;
        p.tools[0].speed = 999;
        p.tools[0].feed_rate = 998;
        p.tools[0].plunge_rate = 997;
        let json = serde_json::to_string(&p).unwrap();
        assert!(!json.contains("provenance"));
        let restored: Project = serde_json::from_str(&json).unwrap();
        assert!(restored.operations[0]
            .params
            .applied_cutting
            .as_ref()
            .unwrap()
            .proposal
            .is_none());
        let after = generate(restored);
        assert_eq!(
            before.gcode, after.gcode,
            "Evidence metadata must not change resolved machine output"
        );
        assert!(!after
            .warnings
            .iter()
            .any(|w| w.kind == "feeds_speeds_stale"));
    }

    #[test]
    fn section_metadata_reads_active_profile_pass_and_final_constraints() {
        let mut p = fixture();
        apply(&mut p);
        let slot = &mut p.operations[0]
            .params
            .applied_cutting
            .as_mut()
            .unwrap()
            .slots[1];
        slot.rpm = 13000;
        slot.feed_mm_min = 1500;
        slot.plunge_mm_min = 600;
        p.machine.spindle_rpm_max = Some(12000);
        p.machine.max_feed_mm_min = Some(1400);
        p.fs_context.as_mut().unwrap().tools[0].max_rpm = Some(11000);
        let index = p.tools.iter().map(|tool| (tool.id, tool)).collect();
        let ctx = crate::pipeline::cps_ctx::build_section_ctx(
            &p.operations[0],
            &p,
            &index,
            &crate::cam::setup::Setup::default(),
        );
        assert_eq!(ctx.tool.spindle_rpm, 11000);
        let number = |key: &str| {
            ctx.params
                .iter()
                .find_map(|(name, value)| match value {
                    crate::pipeline::CpsParamValue::Number(n) if name == key => Some(*n),
                    _ => None,
                })
                .unwrap()
        };
        assert_eq!(number("operation:tool_feedCutting"), 1400.0);
        assert_eq!(number("operation:tool_feedPlunge"), 600.0);
        assert_eq!(
            saved_rates(&p.operations[0], &p.tools[0], PassKind::Finish).unwrap(),
            (13000, 600, 1500)
        );
    }
    #[test]
    fn explicit_apply_can_resolve_a_changed_tool_without_live_inheritance() {
        let mut p = fixture();
        apply(&mut p);
        let mut new_tool = p.tools[0].clone();
        new_tool.id = 2;
        p.tools.push(new_tool);
        p.operations[0].tool_id = 2;
        let mut binding = p.fs_context.as_ref().unwrap().tools[0].clone();
        binding.tool_id = 2;
        p.fs_context.as_mut().unwrap().tools.push(binding);
        assert!(validate_applied_project(&p).is_err());
        apply(&mut p);
        assert!(validate_applied_project(&p).is_ok());
        assert!(p.operations[0]
            .params
            .applied_cutting
            .as_ref()
            .unwrap()
            .slots
            .iter()
            .all(|s| s.tool_id == 2));
    }
    #[test]
    fn pocket_uses_nominal_overlap_and_keeps_secondary_pass_origins() {
        let mut p = fixture();
        let mut data: serde_json::Value = serde_json::from_str(DATA).unwrap();
        let profile = &mut data["profile_set"]["profiles"][0];
        profile["scope"]["operation"] = serde_json::json!("pocket");
        profile["scope"]["engagement"] =
            serde_json::json!({"kind":"nominal","ratio":{"min":0.4,"max":0.4}});
        profile["data"]["stepover_ratio"] = serde_json::json!(0.4);
        profile["source"]["author"] = serde_json::json!("Original synthetic software fixture");
        p.operations[0].kind = OpKind::Pocket {
            strategy: crate::project::PocketStrategy::Spiral,
            contour: crate::project::ContourParams::default(),
            pocket: crate::project::PocketParams::default(),
        };
        p.operations[0].pocket_params_mut().unwrap().xy_overlap = 0.6;
        let d = fs::load_dataset_json(&data.to_string()).unwrap();
        let r = cam_request(
            &p,
            &p.operations[0],
            d.document().profile_set.identity.clone(),
            fs::Aggressiveness::Prudent,
        )
        .unwrap();
        assert_eq!(
            r.operation.engagement,
            fs::Engagement::Nominal { ratio: 0.4 }
        );
        let result = feeds_speeds(FsCamRequest {
            project: p.clone(),
            op_id: 1,
            dataset_json: data.to_string(),
            aggressiveness: fs::Aggressiveness::Prudent,
            apply: true,
        })
        .unwrap();
        let op = result.applied_op.unwrap();
        let block = op.params.applied_cutting.as_ref().unwrap();
        assert!(matches!(
            block.slots[0].origins.feed,
            CuttingOrigin::Recommendation
        ));
        assert!(matches!(
            block.slots[1].origins.feed,
            CuttingOrigin::LegacySnapshot
        ));
        assert_eq!(op.pocket_params().unwrap().xy_overlap, 0.6);
    }
}
