//! Explicit milling values and CAM readiness, independent of recommendation data.
use crate::project::{MachineMode, Op, OpKind, PassKind, Project, ToolEntry};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CuttingRole {
    Rough,
    Finish,
    Drill,
    Spot,
    Chamfer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RoleRates {
    pub role: CuttingRole,
    pub tool_id: u32,
    pub rpm: Option<u32>,
    pub feed_mm_min: Option<u32>,
    pub plunge_mm_min: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MillingCutting {
    pub rates: Vec<RoleRates>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_angle_deg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preset_snapshot: Option<crate::presets::LoadedPresetSnapshot>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CuttingCheck {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Actual driver roles. Separate secondary roles even when the cutter is shared.
#[must_use]
pub fn roles(op: &Op) -> Vec<(CuttingRole, u32)> {
    use CuttingRole::{Chamfer, Drill, Finish, Rough, Spot};
    if op.is_program_only()
        || matches!(
            op.kind,
            OpKind::DragKnife { .. } | OpKind::RasterEngrave { .. }
        )
    {
        return vec![];
    }
    match &op.kind {
        OpKind::Drill {
            spot_first,
            chamfer_after_width_mm,
            ..
        } => {
            let mut result = vec![(Drill, op.tool_id)];
            if let Some(s) = spot_first {
                result.push((Spot, s.spot_tool_id));
            }
            if chamfer_after_width_mm.is_some() {
                result.push((Chamfer, op.finish_tool_id.unwrap_or(op.tool_id)));
            }
            result
        }
        OpKind::Pocket {
            strategy: crate::project::PocketStrategy::Halfpipe { .. },
            ..
        }
        | OpKind::Chamfer {
            finish_pass: false, ..
        } => vec![(Rough, op.tool_id)],
        OpKind::Pocket { pocket, .. } if pocket.pocket_nocontour => vec![(Rough, op.tool_id)],
        OpKind::Pocket { .. } => vec![
            (Rough, op.tool_id),
            (Finish, op.finish_tool_id.unwrap_or(op.tool_id)),
        ],
        OpKind::Profile { .. } | OpKind::Chamfer { .. } => {
            vec![(Rough, op.tool_id), (Finish, op.tool_id)]
        }
        _ => vec![(Rough, op.tool_id)],
    }
}

#[must_use]
pub fn uses_entry_angle(op: &Op) -> bool {
    matches!(
        op.kind,
        OpKind::VCarve { .. }
            | OpKind::Pocket {
                strategy: crate::project::PocketStrategy::Halfpipe { .. },
                ..
            }
    ) || op.drill_chamfer_after_width_mm().is_some()
}

/// Creation only. No generation path calls this helper.
#[must_use]
pub fn seed(op: &Op, tools: &[ToolEntry]) -> MillingCutting {
    MillingCutting {
        entry_angle_deg: uses_entry_angle(op).then(|| {
            tools
                .iter()
                .find(|t| t.id == op.finish_tool_id.unwrap_or(op.tool_id))
                .and_then(|t| t.vcarve_lead_in_angle_deg)
                .unwrap_or(10.0)
        }),
        rates: roles(op)
            .into_iter()
            .map(|(role, tool_id)| {
                let values = tools.iter().find(|t| t.id == tool_id).map(|t| {
                    let pass = match role {
                        CuttingRole::Finish => PassKind::Finish,
                        CuttingRole::Drill | CuttingRole::Spot | CuttingRole::Chamfer => {
                            PassKind::Drill
                        }
                        CuttingRole::Rough => PassKind::Rough,
                    };
                    crate::project::resolve_tool_rates(t, pass)
                });
                RoleRates {
                    role,
                    tool_id,
                    rpm: values.and_then(|v| (v.0 > 0).then_some(v.0)),
                    plunge_mm_min: values.and_then(|v| (v.1 > 0).then_some(v.1)),
                    feed_mm_min: values.and_then(|v| (v.2 > 0).then_some(v.2)),
                }
            })
            .collect(),
        preset_snapshot: None,
    }
}

pub fn rates(op: &Op, role: CuttingRole, tool_id: u32) -> Result<(u32, u32, u32), String> {
    let cutting = op
        .params
        .cutting
        .as_ref()
        .ok_or("Missing explicit cutting parameters")?;
    let mut slots = cutting
        .rates
        .iter()
        .filter(|s| s.role == role && s.tool_id == tool_id);
    let slot = slots
        .next()
        .ok_or_else(|| format!("Missing {role:?} role for tool {tool_id}"))?;
    if slots.next().is_some() {
        return Err(format!("Duplicate {role:?} role"));
    }
    match (slot.rpm, slot.plunge_mm_min, slot.feed_mm_min) {
        (Some(r), Some(p), Some(f)) if r > 0 && p > 0 && f > 0 => Ok((r, p, f)),
        _ => Err(format!(
            "{role:?}: spindle, feed and plunge must be explicit positive integers"
        )),
    }
}

/// Structural persistence permits missing values, but never ambiguous roles.
pub fn validate_structure(op: &Op) -> Result<(), String> {
    let c = op
        .params
        .cutting
        .as_ref()
        .ok_or("Missing explicit cutting parameters")?;
    let expected = roles(op);
    let mut actual: Vec<_> = c.rates.iter().map(|s| (s.role, s.tool_id)).collect();
    actual.sort_unstable();
    let mut sorted = expected;
    sorted.sort_unstable();
    if actual != sorted {
        return Err("Cutting roles are missing, duplicated or assigned to a different tool".into());
    }
    if op.params.feed_rate_override.is_some() || op.params.plunge_rate_override.is_some() {
        return Err("Legacy cutting overrides are not part of the v9 contract".into());
    }
    Ok(())
}

pub fn validate_limits(
    project: &Project,
    tool: &ToolEntry,
    values: (u32, u32, u32),
) -> Result<(), String> {
    let (rpm, plunge, feed) = values;
    let m = &project.machine;
    if rpm == 0 || plunge == 0 || feed == 0 {
        return Err("Spindle, feed and plunge must be positive".into());
    }
    if m.spindle_rpm_min
        .zip(m.spindle_rpm_max)
        .is_some_and(|(a, b)| a > b)
        || m.spindle_rpm_min == Some(0)
        || m.spindle_rpm_max == Some(0)
        || tool.max_rpm == Some(0)
        || m.max_feed_mm_min == Some(0)
    {
        return Err("Conflicting or zero machine/tool limits".into());
    }
    if m.spindle_rpm_min.is_some_and(|v| rpm < v) {
        return Err(format!("Spindle {rpm} rpm is below the machine minimum"));
    }
    if m.spindle_rpm_max.is_some_and(|v| rpm > v) {
        return Err(format!("Spindle {rpm} rpm exceeds the machine maximum"));
    }
    if tool.max_rpm.is_some_and(|v| rpm > v) {
        return Err(format!(
            "Spindle {rpm} rpm exceeds tool {} maximum",
            tool.id
        ));
    }
    if m.max_feed_mm_min.is_some_and(|v| feed > v || plunge > v) {
        return Err(format!(
            "Feed {feed} / plunge {plunge} mm/min exceeds the machine maximum"
        ));
    }
    Ok(())
}

/// General constraints apply regardless of any workshop attestation.
#[must_use]
pub fn validate_operation(project: &Project, op: &Op) -> CuttingCheck {
    let mut out = CuttingCheck::default();
    if op.is_program_only() {
        return out;
    }
    if op.params.feed_rate_override.is_some() || op.params.plunge_rate_override.is_some() {
        out.errors
            .push("Legacy cutting overrides are unsupported in v9".into());
    }
    if project.machine.mode != MachineMode::Mill {
        if op.params.cutting.is_some() {
            out.errors
                .push("Milling values cannot be used for this process".into());
        }
        return out;
    }
    if matches!(
        op.kind,
        OpKind::DragKnife { .. } | OpKind::RasterEngrave { .. }
    ) {
        out.errors
            .push("Select the matching non-milling machine process for this operation".into());
        return out;
    }
    if let Err(e) = validate_structure(op) {
        out.errors.push(e);
        return out;
    }
    if matches!(
        op.kind,
        OpKind::Profile { .. }
            | OpKind::Pocket { .. }
            | OpKind::Engrave { .. }
            | OpKind::Chamfer { .. }
            | OpKind::VCarve { .. }
            | OpKind::PlanarSurface { .. }
    ) && op.params.step.is_none()
        && op.params.depth_list.is_empty()
    {
        out.errors
            .push("Set explicit depth per pass or an explicit depth schedule".into());
    }
    if let Some(pocket) = op.pocket_params() {
        if !pocket.xy_overlap.is_finite() || pocket.xy_overlap <= 0.0 || pocket.xy_overlap >= 1.0 {
            out.errors.push(
                "Pocket overlap must be explicit, finite and between 0 and 100 percent".into(),
            );
        }
    }
    if uses_entry_angle(op) {
        let angle = op.params.cutting.as_ref().and_then(|c| c.entry_angle_deg);
        let max = if op.drill_chamfer_after_width_mm().is_some() {
            45.0
        } else {
            89.0
        };
        if !angle.is_some_and(|v| v.is_finite() && v >= 1.0 && v <= max) {
            out.errors.push(format!(
                "Set an explicit entry angle between 1 and {max} degrees"
            ));
        }
    }
    for (role, id) in roles(op) {
        let Some(tool) = project.tools.iter().find(|t| t.id == id) else {
            out.errors.push(format!("{role:?}: tool {id} is missing"));
            continue;
        };
        if !tool.kind.compatible_with_mode(MachineMode::Mill) {
            out.errors
                .push(format!("{role:?}: incompatible cutter process"));
        }
        if !tool.diameter.is_finite()
            || !tool.wear_offset_mm.is_finite()
            || tool.diameter <= 0.0
            || tool.diameter - tool.wear_offset_mm <= 0.0
            || tool.flutes == 0
        {
            out.errors
                .push(format!("{role:?}: invalid diameter, wear or flute count"));
        }
        out.errors
            .extend(validate_specialized_values(op, tool, role));
        match rates(op, role, id).and_then(|v| validate_limits(project, tool, v)) {
            Ok(()) => {}
            Err(e) => out.errors.push(format!("{role:?}: {e}")),
        }
        if project.machine.spindle_rpm_min.is_none()
            || project.machine.spindle_rpm_max.is_none()
            || project.machine.max_feed_mm_min.is_none()
            || tool.max_rpm.is_none()
            || tool.flute_length_mm.is_none()
        {
            out.warnings.push(format!("{role:?}: some machine/tool limits are unknown; verify spindle, feed and usable flute length"));
        }
        if let Some(length) = tool.flute_length_mm {
            if !length.is_finite() || length <= 0.0 {
                out.errors
                    .push(format!("{role:?}: invalid usable flute length"));
            }
            let reach = axial_reach(op, tool, role);
            if reach.is_some_and(|depth| depth > length + 1e-9) {
                out.errors.push(format!(
                    "{role:?}: requested cutting depth exceeds usable flute length"
                ));
            }
            if matches!(role, CuttingRole::Rough | CuttingRole::Finish)
                && !matches!(op.kind, OpKind::TSlot { .. } | OpKind::Dovetail { .. })
                && (op.params.step.is_some_and(|s| s.abs() > length)
                    || op.params.finish_step.is_some_and(|s| s.abs() > length))
            {
                out.errors.push(format!(
                    "{role:?}: depth per pass exceeds usable flute length"
                ));
            }
        }
    }
    for (name, value) in [
        ("depth", op.params.depth),
        ("start depth", op.params.start_depth),
        ("clearance", op.params.fast_move_z),
        ("stock to leave", op.params.stock_to_leave_mm),
    ] {
        if !value.is_finite() {
            out.errors.push(format!("{name} must be finite"));
        }
    }
    if let Some(s) = op.params.step {
        if !s.is_finite() || s >= -1e-9 {
            out.errors
                .push("Depth per pass must be finite and positive".into());
        }
    }
    if op
        .params
        .finish_step
        .is_some_and(|s| !s.is_finite() || s >= -1e-9)
    {
        out.errors
            .push("Finish depth per pass must be finite and positive".into());
    }
    if !op.params.through_depth.is_finite() || op.params.through_depth < 0.0 {
        out.errors
            .push("Through depth must be finite and nonnegative".into());
    }
    if op
        .params
        .depth_list
        .first()
        .is_some_and(|v| *v >= op.params.start_depth)
    {
        out.errors
            .push("Explicit depth schedule must start below the start depth".into());
    }
    if op.params.depth_list.iter().any(|v| !v.is_finite())
        || op.params.depth_list.windows(2).any(|w| w[1] >= w[0])
    {
        out.errors
            .push("Explicit depths must be finite and strictly descending".into());
    }
    out
}

fn axial_reach(op: &Op, tool: &ToolEntry, role: CuttingRole) -> Option<f64> {
    if role == CuttingRole::Spot {
        return if let OpKind::Drill {
            spot_first: Some(spot),
            ..
        } = op.kind
        {
            Some(-spot.spot_depth_mm)
        } else {
            None
        };
    }
    let width = if role == CuttingRole::Chamfer {
        op.drill_chamfer_after_width_mm()
    } else if let OpKind::Chamfer { width_mm, .. } = op.kind {
        Some(width_mm)
    } else {
        None
    };
    if let Some(w) = width {
        return Some(
            crate::cam::chamfer::chamfer_depth_capped(
                w,
                tool.tip_angle_deg,
                tool.effective_diameter(),
                tool.tip_diameter.unwrap_or(0.0),
            )
            .z
            .abs(),
        );
    }
    match op.kind {
        OpKind::Profile { .. }
        | OpKind::Pocket { .. }
        | OpKind::Engrave { .. }
        | OpKind::Drill { .. }
        | OpKind::PlanarSurface { .. }
        | OpKind::Thread { .. } => Some(op.params.depth_list.iter().fold(
            (-op.params.depth + op.params.through_depth).max(0.0),
            |d, z| d.max(-z),
        )),
        OpKind::ReliefMill { z_min_mm, .. } => Some(-z_min_mm),
        _ => None,
    }
}

fn validate_specialized_values(op: &Op, tool: &ToolEntry, role: CuttingRole) -> Vec<String> {
    use crate::project::{DrillCycle, HalfpipeProfile, PocketStrategy, ToolKind};
    let mut errors = vec![];
    let positive = |v: f64| v.is_finite() && v > 0.0;
    if matches!(
        op.kind,
        OpKind::Profile { .. }
            | OpKind::Pocket { .. }
            | OpKind::Thread { .. }
            | OpKind::WaterlineRough { .. }
    ) && tool.kind == ToolKind::Drill
    {
        errors
            .push("This operation requires a side-cutting milling tool, not an axial drill".into());
    }
    if matches!(op.kind, OpKind::TSlot { .. } | OpKind::Dovetail { .. })
        && (tool.kind != ToolKind::FormProfile || tool.form_profile_mm.len() < 2)
    {
        errors.push("Undercut operations require an explicit form-profile cutter".into());
    }
    let width = if role == CuttingRole::Chamfer {
        op.drill_chamfer_after_width_mm()
    } else if let OpKind::Chamfer { width_mm, .. } = op.kind {
        Some(width_mm)
    } else {
        None
    };
    if let Some(w) = width {
        let tip = tool.tip_diameter.unwrap_or(0.0);
        if tool.kind != ToolKind::VBit
            || !positive(w)
            || !tip.is_finite()
            || tip < 0.0
            || tip >= tool.effective_diameter()
            || !(1.0..=179.0).contains(&tool.tip_angle_deg)
            || w > (tool.effective_diameter() - tip) * 0.5
        {
            errors.push("Chamfer requires a valid V-bit and an explicit width within its physical reach; values are not clamped".into());
        }
    }
    if let OpKind::VCarve { carve } = &op.kind {
        let tip = tool.tip_diameter.unwrap_or(0.0);
        if tool.kind != ToolKind::VBit
            || !tip.is_finite()
            || tip < 0.0
            || tip >= tool.effective_diameter()
            || !(1.0..=179.0).contains(&tool.tip_angle_deg)
            || op.params.start_depth != 0.0
            || op.params.depth > 0.0
            || op.params.through_depth != 0.0
            || !op.params.depth_list.is_empty()
            || op.params.finish_step.is_some()
            || carve.multi_pass_refine
            || carve
                .carve_max_width_mm
                .is_some_and(|r| !positive(r) || r > tool.effective_diameter() * 0.5)
            || carve
                .source_inset_mm
                .is_some_and(|v| !v.is_finite() || v < 0.0)
        {
            errors.push("V-Carve requires valid V-bit geometry, stock-top start, explicit passes and a radius cap within the cutter; alternate schedules/refine are unsupported".into());
        }
    }
    if let OpKind::PlanarSurface { surface } = &op.kind {
        if !matches!(tool.kind, ToolKind::Endmill | ToolKind::Compression)
            || tool.whirl
            || !positive(surface.stepover_mm)
            || surface.stepover_mm > tool.effective_diameter() * 0.95
            || op.params.start_depth != 0.0
            || op.params.depth >= 0.0
            || op.params.through_depth != 0.0
            || !op.params.depth_list.is_empty()
            || op.finish_tool_id.is_some()
            || !matches!(op.params.plunge, crate::project::PlungeStrategy::Direct)
        {
            errors.push("Planar surfacing requires a flat cutter, one explicit depth-step schedule, direct entry and stepover at most 95% of cutter diameter".into());
        }
    }
    match &op.kind {
        OpKind::Drill {
            cycle, spot_first, ..
        } => {
            let (peck, dwell) = match *cycle {
                DrillCycle::Simple { dwell_sec } => (None, dwell_sec),
                DrillCycle::Peck {
                    peck_step_mm,
                    dwell_sec,
                }
                | DrillCycle::ChipBreak {
                    peck_step_mm,
                    dwell_sec,
                } => (Some(peck_step_mm), dwell_sec),
            };
            if peck.is_some_and(|v| !positive(v)) || !dwell.is_finite() || dwell < 0.0 {
                errors.push(
                    "Drill cycle requires positive explicit peck depth and nonnegative dwell"
                        .into(),
                );
            }
            if spot_first.is_some_and(|v| {
                !v.spot_depth_mm.is_finite() || v.spot_depth_mm >= op.params.start_depth
            }) {
                errors.push("Spot depth must be finite and below the start depth".into());
            }
        }
        OpKind::Pocket {
            strategy: PocketStrategy::Halfpipe { profile },
            ..
        } => {
            if op.finish_tool_id.is_some() {
                errors.push("Halfpipe uses one cutter; remove the unused finish tool".into());
            }
            if tool.flute_length_mm.is_none() {
                errors.push("Halfpipe requires an explicit usable flute length; no tool-radius rating is inferred".into());
            }
            if op.params.step.is_some_and(|v| v.abs() < 0.05) {
                errors.push("Halfpipe depth per pass must be at least 0.05 mm".into());
            }
            let valid = match profile {
                HalfpipeProfile::CircularArc { radius_mm } => {
                    tool.kind == ToolKind::BallNose
                        && positive(*radius_mm)
                        && (tool.effective_diameter() * 0.5 - radius_mm).abs() <= radius_mm * 0.10
                }
                HalfpipeProfile::VBottom { included_angle_deg } => {
                    tool.kind == ToolKind::VBit
                        && (1.0..=179.0).contains(included_angle_deg)
                        && (tool.tip_angle_deg - included_angle_deg).abs() <= 1e-6
                }
            };
            if !valid {
                errors
                    .push("Halfpipe profile requires matching ball-nose or V-bit geometry".into());
            }
        }
        OpKind::ReliefMill {
            z_min_mm,
            z_max_mm,
            scallop_height_mm,
            stepover_mm,
            along_step_mm,
            ..
        } => {
            if !matches!(tool.kind, ToolKind::BallNose | ToolKind::BullNose)
                || !z_min_mm.is_finite()
                || !z_max_mm.is_finite()
                || z_min_mm >= z_max_mm
                || (stepover_mm.is_none() && !positive(*scallop_height_mm))
                || !positive(*along_step_mm)
                || stepover_mm.is_some_and(|v| !positive(v))
            {
                errors.push("Relief requires a ball/bull-nose, descending finite depth range and positive sampling/engagement".into());
            }
            if tool.kind == ToolKind::BullNose
                && !tool
                    .corner_radius_mm
                    .is_some_and(|r| positive(r) && r <= tool.effective_diameter() * 0.5)
            {
                errors.push(
                    "Bull-nose corner radius must be explicit and within cutter radius".into(),
                );
            }
        }
        OpKind::WaterlineRough {
            z_step_mm,
            stepover_mm,
            floor_z_mm,
            ..
        } => {
            if !positive(*z_step_mm)
                || !positive(*stepover_mm)
                || !floor_z_mm.is_finite()
                || *floor_z_mm > 0.0
            {
                errors.push(
                    "Waterline requires explicit positive depth/side steps and a nonpositive floor"
                        .into(),
                );
            }
        }
        OpKind::Thread {
            pitch_mm,
            radial_passes,
            thread_depth_mm,
            ..
        } => {
            if !positive(*pitch_mm)
                || *radial_passes == 0
                || thread_depth_mm.is_some_and(|v| !positive(v))
            {
                errors.push("Thread requires positive pitch, radial pass count and any explicit thread depth".into());
            }
        }
        _ => {}
    }
    errors
}

/// Required by every public generation adapter before any output file is staged.
pub fn require_current(project: &Project) -> Result<(), String> {
    if project.cutting_version != Some(9) {
        return Err(
            "Generation requires explicit cutting contract v9; legacy projects are unsupported"
                .into(),
        );
    }
    Ok(())
}

/// Every generation path requires the explicit native v9 contract.
pub fn validate_project(project: &Project) -> Result<(), crate::pipeline::PipelineError> {
    require_current(project).map_err(|e| crate::pipeline::PipelineError::AppliedCutting(0, e))?;
    let mut tool_ids = std::collections::BTreeSet::new();
    let mut op_ids = std::collections::BTreeSet::new();
    if project
        .tools
        .iter()
        .any(|t| t.id == 0 || !tool_ids.insert(t.id))
        || project
            .operations
            .iter()
            .any(|o| o.id == 0 || !op_ids.insert(o.id))
    {
        return Err(crate::pipeline::PipelineError::AppliedCutting(
            0,
            "Tool and operation IDs must be positive and unique".into(),
        ));
    }
    for op in project.operations.iter().filter(|o| o.enabled) {
        for (_, id) in roles(op) {
            if !project.tools.iter().any(|t| t.id == id) {
                return Err(crate::pipeline::PipelineError::UnknownTool(op.id, id));
            }
        }
        let report = validate_operation(project, op);
        if !report.errors.is_empty() {
            return Err(crate::pipeline::PipelineError::AppliedCutting(
                op.id,
                report.errors.join("; "),
            ));
        }
    }
    Ok(())
}

/// Resolve only explicit v9 milling rates. Generation never reads creation defaults.
pub fn saved_rates(op: &Op, tool: &ToolEntry, pass: PassKind) -> Result<(u32, u32, u32), String> {
    let role = match pass {
        PassKind::Drill => CuttingRole::Drill,
        PassKind::Rough => CuttingRole::Rough,
        PassKind::Finish => CuttingRole::Finish,
    };
    let expected = roles(op);
    // A single-role driver fills both inert Setup triplets with that one role.
    // A dual-tool Pocket's primary Setup never emits the secondary finish ring.
    let selected = if expected.contains(&(role, tool.id)) {
        role
    } else if let Some((r, _)) = expected.iter().find(|(_, id)| *id == tool.id) {
        *r
    } else {
        return Err("Tool has no active role".into());
    };
    rates(op, selected, tool.id)
}

#[must_use]
pub fn effective_machine(project: &Project, tool_id: u32) -> crate::project::MachineConfig {
    let mut m = project.machine.clone();
    if let Some(max) = project
        .tools
        .iter()
        .find(|t| t.id == tool_id)
        .and_then(|t| t.max_rpm)
    {
        m.spindle_rpm_max = Some(m.spindle_rpm_max.map_or(max, |v| v.min(max)));
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    fn project() -> Project {
        let tools = vec![ToolEntry::default()];
        let mut op = Op::default();
        op.params.step = Some(-1.0);
        op.params.cutting = Some(seed(&op, &tools));
        Project {
            cutting_version: Some(9),
            tools,
            operations: vec![op],
            ..Project::default()
        }
    }
    #[test]
    fn explicit_rates_survive_default_changes() {
        let mut p = project();
        let before = p.operations[0].params.cutting.clone();
        p.tools[0].feed_rate = 99999;
        p.tools[0].speed = 1;
        assert!(validate_operation(&p, &p.operations[0]).errors.is_empty());
        assert_eq!(before, p.operations[0].params.cutting);
        assert_ne!(
            saved_rates(&p.operations[0], &p.tools[0], PassKind::Rough)
                .unwrap()
                .0,
            1
        );
    }
    #[test]
    fn incomplete_is_editable_but_cannot_generate() {
        let mut p = project();
        p.operations[0].params.cutting.as_mut().unwrap().rates[0].rpm = None;
        assert!(validate_structure(&p.operations[0]).is_ok());
        let json = serde_json::to_string(&p).unwrap();
        let restored: Project = serde_json::from_str(&json).unwrap();
        assert!(validate_project(&restored).is_err());
        assert!(json.contains("\"rpm\":null"));
    }
    #[test]
    fn no_duplicate_missing_stray_or_misassigned_role() {
        let p = project();
        for change in 0..4 {
            let mut op = p.operations[0].clone();
            let c = op.params.cutting.as_mut().unwrap();
            match change {
                0 => {
                    c.rates.pop();
                }
                1 => c.rates.push(c.rates[0].clone()),
                2 => c.rates[0].role = CuttingRole::Spot,
                _ => c.rates[0].tool_id = 999,
            }
            assert!(validate_structure(&op).is_err());
        }
    }
    #[test]
    fn known_limits_reject_without_mutation_unknowns_disclosed() {
        let mut p = project();
        let original = p.operations[0].params.cutting.clone();
        assert!(!validate_operation(&p, &p.operations[0]).warnings.is_empty());
        p.tools[0].max_rpm = Some(1);
        assert!(validate_project(&p).is_err());
        p.tools[0].max_rpm = None;
        p.machine.max_feed_mm_min = Some(1);
        assert!(validate_project(&p).is_err());
        p.machine.max_feed_mm_min = None;
        p.machine.spindle_rpm_min = Some(u32::MAX);
        assert!(validate_project(&p).is_err());
        assert_eq!(original, p.operations[0].params.cutting);
    }
    #[test]
    fn zero_known_ratings_refuse_instead_of_becoming_unknown() {
        for case in 0..4 {
            let mut p = project();
            let original = p.operations[0].params.cutting.clone();
            match case {
                0 => p.machine.spindle_rpm_min = Some(0),
                1 => p.machine.spindle_rpm_max = Some(0),
                2 => p.machine.max_feed_mm_min = Some(0),
                _ => p.tools[0].max_rpm = Some(0),
            }
            assert!(validate_operation(&p, &p.operations[0])
                .errors
                .iter()
                .any(|e| e.contains("zero machine/tool limits")));
            assert!(validate_project(&p).is_err());
            assert_eq!(original, p.operations[0].params.cutting);
        }
        let p = project();
        assert!(validate_project(&p).is_ok());
        assert!(!validate_operation(&p, &p.operations[0]).warnings.is_empty());
    }
    #[test]
    fn invalid_geometry_numeric_and_depth_rejected() {
        for case in 0..5 {
            let mut p = project();
            match case {
                0 => p.tools[0].diameter = f64::NAN,
                1 => p.tools[0].wear_offset_mm = 99.0,
                2 => p.tools[0].flutes = 0,
                3 => p.operations[0].params.step = Some(f64::INFINITY),
                _ => p.tools[0].flute_length_mm = Some(0.1),
            }
            assert!(validate_project(&p).is_err(), "case {case}");
        }
    }
    #[test]
    fn pocket_overlap_is_a_fraction_and_cannot_inherit_defaults() {
        let mut p = project();
        p.operations[0].kind =
            serde_json::from_str(r#"{"type":"pocket","strategy":"cascade"}"#).unwrap();
        p.operations[0].pocket_params_mut().unwrap().xy_overlap = 0.5;
        assert!(validate_project(&p).is_ok());
        for overlap in [0.0, 1.0, 1.5, f64::NAN] {
            p.operations[0].pocket_params_mut().unwrap().xy_overlap = overlap;
            assert!(validate_project(&p).is_err());
        }
    }
    #[test]
    fn wire_refuses_unknown_fractional_overflow_roles() {
        let p = project();
        let c = serde_json::to_value(p.operations[0].params.cutting.as_ref().unwrap()).unwrap();
        for value in [
            serde_json::json!(1.5),
            serde_json::json!(4_294_967_296_u64),
            serde_json::json!(-1),
        ] {
            let mut c = c.clone();
            c["rates"][0]["rpm"] = value;
            assert!(serde_json::from_value::<MillingCutting>(c).is_err());
        }
        let mut c = c;
        c["surprise"] = serde_json::json!(1);
        assert!(serde_json::from_value::<MillingCutting>(c).is_err());
    }
}
