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
    if op.is_program_only() || matches!(op.kind, OpKind::DragKnife { .. }) {
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
        OpKind::Pocket { .. } => vec![
            (Rough, op.tool_id),
            (Finish, op.finish_tool_id.unwrap_or(op.tool_id)),
        ],
        OpKind::Profile { .. } | OpKind::Engrave { .. } | OpKind::Chamfer { .. } => {
            vec![(Rough, op.tool_id), (Finish, op.tool_id)]
        }
        _ => vec![(Rough, op.tool_id)],
    }
}

/// Creation only. No generation path calls this helper.
#[must_use]
pub fn seed(op: &Op, tools: &[ToolEntry]) -> MillingCutting {
    MillingCutting {
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
    if op.params.applied_cutting.is_some()
        || op.params.feed_rate_override.is_some()
        || op.params.plunge_rate_override.is_some()
    {
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
    if project.machine.mode != MachineMode::Mill {
        if op.params.cutting.is_some() {
            out.errors
                .push("Milling values cannot be used for this process".into());
        }
        return out;
    }
    if let Err(e) = validate_structure(op) {
        out.errors.push(e);
        return out;
    }
    if matches!(
        op.kind,
        OpKind::Profile { .. } | OpKind::Pocket { .. } | OpKind::Engrave { .. }
    ) && op.params.step.is_none()
        && op.params.depth_list.is_empty()
    {
        out.errors
            .push("Set explicit depth per pass or an explicit depth schedule".into());
    }
    if let Some(pocket) = op.pocket_params() {
        if !pocket.xy_overlap.is_finite() || pocket.xy_overlap <= 0.0 || pocket.xy_overlap >= 100.0
        {
            out.errors.push(
                "Pocket overlap must be explicit, finite and between 0 and 100 percent".into(),
            );
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
            if op.params.step.is_some_and(|s| s.abs() > length)
                || op.params.finish_step.is_some_and(|s| s.abs() > length)
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
    if op.params.depth_list.iter().any(|v| !v.is_finite())
        || op.params.depth_list.windows(2).any(|w| w[1] >= w[0])
    {
        out.errors
            .push("Explicit depths must be finite and strictly descending".into());
    }
    out
}

/// V9 is an explicit native contract; old internal fixtures stay isolated during staged removal.
pub fn validate_project(project: &Project) -> Result<(), crate::pipeline::PipelineError> {
    if project.cutting_version.is_some_and(|v| v != 9) {
        return Err(crate::pipeline::PipelineError::AppliedCutting(
            0,
            "Unsupported cutting contract version".into(),
        ));
    }
    if project.cutting_version != Some(9) {
        return crate::feeds_speeds::validate_applied_project(project);
    }
    for op in project.operations.iter().filter(|o| o.enabled) {
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

/// Resolve only explicit v9 rates. Legacy dispatch remains temporary for internal fixtures.
pub fn saved_rates(op: &Op, tool: &ToolEntry, pass: PassKind) -> Result<(u32, u32, u32), String> {
    if op.params.cutting.is_none() {
        return crate::feeds_speeds::saved_rates(op, tool, pass);
    }
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
    if project.cutting_version != Some(9) {
        return crate::feeds_speeds::effective_machine(project, tool_id);
    }
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
            };
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
    fn invalid_geometry_numeric_and_depth_rejected() {
        for case in 0..5 {
            let mut p = project();
            match case {
                0 => p.tools[0].diameter = f64::NAN,
                1 => p.tools[0].wear_offset_mm = 99.0,
                2 => p.tools[0].flutes = 0,
                3 => p.operations[0].params.step = Some(f64::INFINITY),
                _ => p.tools[0].flute_length_mm = Some(0.1),
            };
            assert!(validate_project(&p).is_err(), "case {case}");
        }
    }
    #[test]
    fn wire_refuses_unknown_fractional_overflow_roles() {
        let p = project();
        let c = serde_json::to_value(p.operations[0].params.cutting.as_ref().unwrap()).unwrap();
        for value in [
            serde_json::json!(1.5),
            serde_json::json!(4294967296_u64),
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
