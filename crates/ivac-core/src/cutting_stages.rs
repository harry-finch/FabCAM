//! Explicit stage data for the v10 milling contract.
//! Profile/Pocket v10 authors stages explicitly; old role inputs never supply them.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// One authored stage. Missing values are editable, never executable defaults.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CuttingStage {
    pub tool_id: u32,
    #[serde(deserialize_with = "required_option")]
    #[schemars(required, schema_with = "nullable_rate_schema")]
    pub rpm: Option<u32>,
    #[serde(deserialize_with = "required_option")]
    #[schemars(required, schema_with = "nullable_rate_schema")]
    pub feed_mm_min: Option<u32>,
    #[serde(deserialize_with = "required_option")]
    #[schemars(required, schema_with = "nullable_rate_schema")]
    pub plunge_mm_min: Option<u32>,
    /// Positive axial depth removed per pass, in mm.
    #[serde(deserialize_with = "required_option")]
    #[schemars(required, schema_with = "nullable_depth_schema")]
    pub depth_per_pass_mm: Option<f64>,
}

/// Stock reserved by Main and removed by this operation's finishing stage.
/// Retained-for-later stock deliberately does not belong to this structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FinishingStage {
    pub cutting: CuttingStage,
    pub lateral_allowance_mm: f64,
    pub floor_allowance_mm: f64,
}

/// Required semantic discriminator: never an optional flag next to legacy rates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "contract", deny_unknown_fields)]
pub enum MainFinishing {
    #[serde(rename = "main_finishing_v1")]
    V1 {
        main: CuttingStage,
        #[serde(deserialize_with = "required_option")]
        #[schemars(required, schema_with = "nullable_finishing_schema")]
        finishing: Option<FinishingStage>,
    },
}

fn required_option<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    Option::deserialize(deserializer)
}

impl MainFinishing {
    /// Numeric readiness only. Tool/process limits, geometry and reachable
    /// material removal must still be proven by the authoritative CAM pipeline.
    #[must_use]
    pub fn numeric_errors(&self) -> Vec<String> {
        let Self::V1 { main, finishing } = self;
        let mut errors = Vec::new();
        for (label, stage) in std::iter::once(("main", main))
            .chain(finishing.iter().map(|f| ("finishing", &f.cutting)))
        {
            if stage.tool_id == 0 {
                errors.push(format!("{label}: tool_id must be positive"));
            }
            for (field, value) in [
                ("rpm", stage.rpm),
                ("feed_mm_min", stage.feed_mm_min),
                ("plunge_mm_min", stage.plunge_mm_min),
            ] {
                if !value.is_some_and(|v| v > 0) {
                    errors.push(format!("{label}: {field} requires a positive integer"));
                }
            }
            if !stage
                .depth_per_pass_mm
                .is_some_and(|v| v.is_finite() && v > 0.0)
            {
                errors.push(format!(
                    "{label}: depth_per_pass_mm must be finite and positive"
                ));
            }
        }
        if let Some(f) = finishing {
            for (field, value) in [
                ("lateral_allowance_mm", f.lateral_allowance_mm),
                ("floor_allowance_mm", f.floor_allowance_mm),
            ] {
                if !value.is_finite() || value < 0.0 {
                    errors.push(format!("finishing: {field} must be finite and nonnegative"));
                }
            }
        }
        errors
    }
}

#[must_use]
pub fn applies(op: &crate::project::Op) -> bool {
    use crate::project::{OpKind, PocketStrategy};
    matches!(op.kind, OpKind::Profile { .. } | OpKind::Pocket { .. })
        && !matches!(
            op.kind,
            OpKind::Pocket {
                strategy: PocketStrategy::Halfpipe { .. },
                ..
            }
        )
}
impl MainFinishing {
    #[must_use]
    pub fn main(&self) -> &CuttingStage {
        let Self::V1 { main, .. } = self;
        main
    }
    #[must_use]
    pub fn finishing(&self) -> Option<&FinishingStage> {
        let Self::V1 { finishing, .. } = self;
        finishing.as_ref()
    }
}
impl CuttingStage {
    pub fn rates(&self, tool_id: u32) -> Result<(u32, u32, u32), String> {
        if self.tool_id != tool_id {
            return Err("Stage/tool identity mismatch".into());
        }
        match (self.rpm, self.plunge_mm_min, self.feed_mm_min) {
            (Some(r), Some(p), Some(f)) if r > 0 && p > 0 && f > 0 => Ok((r, p, f)),
            _ => Err("Stage requires explicit positive RPM/feed/plunge".into()),
        }
    }
}

/// Execution-local view for existing setup/geometry machinery. The selected
/// stage is explicit; both inert legacy setup triplets carry this same stage.
/// This is never serialized as a project, preset or authored fallback.
pub(crate) fn execution_op(op: &crate::project::Op, stage: &CuttingStage) -> crate::project::Op {
    use crate::cutting_parameters::{CuttingRole, MillingCutting, RoleRates};
    let mut next = op.clone();
    next.tool_id = stage.tool_id;
    next.finish_tool_id = None;
    next.params.stages = None;
    next.params.stage_preset = None;
    next.params.step = stage.depth_per_pass_mm.map(|v| -v);
    next.params.finish_step = None;
    next.params.depth_list.clear();
    next.params.cutting = Some(MillingCutting {
        rates: [CuttingRole::Rough, CuttingRole::Finish]
            .into_iter()
            .map(|role| RoleRates {
                role,
                tool_id: stage.tool_id,
                rpm: stage.rpm,
                feed_mm_min: stage.feed_mm_min,
                plunge_mm_min: stage.plunge_mm_min,
            })
            .collect(),
        ..MillingCutting::default()
    });
    if let Some(pocket) = next.pocket_params_mut() {
        pocket.finish_xy_allowance_mm = None;
        pocket.pocket_nocontour = false;
    }
    next
}

pub(crate) fn validate_structure(op: &crate::project::Op) -> Result<(), String> {
    let stages = op
        .params
        .stages
        .as_ref()
        .ok_or("Profile/Pocket requires the v10 Main/Finishing contract")?;
    if op.params.cutting.is_some()
        || op.params.step.is_some()
        || op.params.finish_step.is_some()
        || !op.params.depth_list.is_empty()
        || op.params.feed_rate_override.is_some()
        || op.params.plunge_rate_override.is_some()
    {
        return Err("Mixed stage and legacy rate/depth contracts are forbidden".into());
    }
    if op.tool_id != stages.main().tool_id
        || op.finish_tool_id != stages.finishing().map(|f| f.cutting.tool_id)
    {
        return Err("Operation and stage tool assignments disagree".into());
    }
    if !op.params.stock_to_leave_mm.is_finite() || op.params.stock_to_leave_mm < 0.0 {
        return Err("Invalid retained-for-later stock".into());
    }
    for stage in std::iter::once(stages.main()).chain(stages.finishing().map(|f| &f.cutting)) {
        if stage.tool_id == 0 {
            return Err("Stage tool_id must be positive".into());
        }
        if stage
            .depth_per_pass_mm
            .is_some_and(|v| !v.is_finite() || v <= 0.0)
        {
            return Err("depth_per_pass_mm must be finite and positive".into());
        }
    }
    if stages.finishing().is_some_and(|f| {
        [f.lateral_allowance_mm, f.floor_allowance_mm]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
    }) {
        return Err("Invalid finishing allowance".into());
    }
    if let Some(snapshot) = &op.params.stage_preset {
        crate::presets::validate_parameters(&snapshot.parameters)?;
    }
    Ok(())
}

pub(crate) fn validate_operation(
    project: &crate::project::Project,
    op: &crate::project::Op,
) -> crate::cutting_parameters::CuttingCheck {
    use crate::cutting_parameters::{validate_role_operation, CuttingCheck};
    use crate::project::{OpKind, PocketStrategy, ToolOffset};
    let mut out = CuttingCheck::default();
    let Some(stages) = &op.params.stages else {
        out.errors.push("Profile/Pocket requires the v10 Main/Finishing contract; recreate legacy cutting parameters explicitly".into());
        return out;
    };
    if op.params.cutting.is_some()
        || op.params.step.is_some()
        || op.params.finish_step.is_some()
        || !op.params.depth_list.is_empty()
    {
        out.errors
            .push("Mixed stage and legacy rate/depth contracts are forbidden".into());
    }
    if op.tool_id != stages.main().tool_id
        || op.finish_tool_id != stages.finishing().map(|f| f.cutting.tool_id)
    {
        out.errors
            .push("Operation and stage tool assignments disagree".into());
    }
    if !op.params.stock_to_leave_mm.is_finite() || op.params.stock_to_leave_mm < 0.0 {
        out.errors
            .push("Retained-for-later stock must be nonnegative".into());
    }
    if op.params.stock_to_leave_mm > 0.0
        && matches!(
            op.kind,
            OpKind::Profile {
                offset: ToolOffset::On | ToolOffset::None,
                ..
            }
        )
    {
        out.errors
            .push("On-contour Profile has no defined side for retained lateral stock".into());
    }
    out.errors.extend(stages.numeric_errors());
    if let Some(snapshot) = &op.params.stage_preset {
        if snapshot.parameters.stages.is_none() {
            out.errors
                .push("Legacy role snapshot on stage parameters".into());
        }
        if let Err(e) = crate::presets::validate_parameters(&snapshot.parameters) {
            out.errors.push(e);
        }
    }
    if let Some(p) = op.pocket_params() {
        if p.pocket_nocontour || p.finish_xy_allowance_mm.is_some() {
            out.errors
                .push("Legacy nocontour/finish allowance is not a v10 finishing toggle".into());
        }
    }
    for (label, stage) in std::iter::once(("Main", stages.main()))
        .chain(stages.finishing().map(|f| ("Finishing", &f.cutting)))
    {
        let view = execution_op(op, stage);
        let check = validate_role_operation(project, &view);
        for (target, messages) in [
            (&mut out.errors, check.errors),
            (&mut out.warnings, check.warnings),
        ] {
            for message in messages {
                let detail = message
                    .trim_start_matches("Rough: ")
                    .trim_start_matches("Finish: ");
                let message = format!("{label}: {detail}");
                if !target.contains(&message) {
                    target.push(message);
                }
            }
        }
    }
    if let Some(f) = stages.finishing() {
        // Bounded finishing output envelope: 0.02 mm per stage, covering the
        // owned emitters' coordinate rounding and 0.01 mm arc linearization.
        // Two stages may deviate in opposite directions. Refuse rather than
        // silently erase a positive allowance. Zero explicitly means a repeat
        // pass and is not a claim that a positive stock band was reserved.
        for (field, value) in [
            ("lateral_allowance_mm", f.lateral_allowance_mm),
            ("floor_allowance_mm", f.floor_allowance_mm),
        ] {
            if value > 0.0 && value < 0.05 {
                out.errors.push(format!(
                    "Finishing {field}: positive allowance must be at least 0.05 mm for qualified output precision; values are not rounded"
                ));
            }
        }
        if matches!(op.kind, OpKind::Profile { .. }) && f.floor_allowance_mm != 0.0 {
            out.errors
                .push("Profile has no defined floor finishing surface".into());
        }
        if matches!(
            op.kind,
            OpKind::Profile {
                offset: ToolOffset::On | ToolOffset::None,
                ..
            }
        ) && f.lateral_allowance_mm != 0.0
        {
            out.errors
                .push("On-contour Profile has no lateral finishing side".into());
        }
        if matches!(
            op.kind,
            OpKind::Pocket {
                strategy: PocketStrategy::Trochoidal { .. },
                ..
            }
        ) {
            out.errors
                .push("Trochoidal finishing coverage is not qualified".into());
        }
        if op.params.depth + f.floor_allowance_mm >= op.params.start_depth {
            out.errors
                .push("Floor allowance leaves no positive main cutting depth".into());
        }
        for stage in [stages.main(), &f.cutting] {
            if let Some(tool) = project.tools.iter().find(|t| t.id == stage.tool_id) {
                if tool.whirl {
                    out.errors
                        .push("Whirling finishing is not qualified".into());
                }
            }
        }
    }
    out
}

fn nullable_rate_schema(
    generator: &mut schemars::gen::SchemaGenerator,
) -> schemars::schema::Schema {
    generator.subschema_for::<Option<u32>>()
}
fn nullable_depth_schema(
    generator: &mut schemars::gen::SchemaGenerator,
) -> schemars::schema::Schema {
    generator.subschema_for::<Option<f64>>()
}
fn nullable_finishing_schema(
    generator: &mut schemars::gen::SchemaGenerator,
) -> schemars::schema::Schema {
    generator.subschema_for::<Option<FinishingStage>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn fixture() -> Value {
        json!({"contract":"main_finishing_v1", "main":{
            "tool_id":1,"rpm":12000,"feed_mm_min":800,"plunge_mm_min":100,
            "depth_per_pass_mm":1.0},"finishing":null})
    }

    #[test]
    fn main_only_round_trip_has_no_finish_values_or_legacy_roles() {
        let v = fixture();
        let parsed: MainFinishing = serde_json::from_value(v.clone()).unwrap();
        assert!(parsed.numeric_errors().is_empty());
        assert_eq!(serde_json::to_value(parsed).unwrap(), v);
    }

    #[test]
    fn refuses_missing_discriminator_legacy_fields_and_fractional_rates() {
        for field in ["contract", "main", "finishing"] {
            let mut v = fixture();
            v.as_object_mut().unwrap().remove(field);
            assert!(serde_json::from_value::<MainFinishing>(v).is_err());
        }
        let mut v = fixture();
        v["rates"] = json!([]);
        assert!(serde_json::from_value::<MainFinishing>(v).is_err());
        for field in ["rpm", "feed_mm_min", "plunge_mm_min"] {
            let mut v = fixture();
            v["main"][field] = json!(10.5);
            assert!(serde_json::from_value::<MainFinishing>(v).is_err());
        }
    }

    #[test]
    fn incomplete_and_invalid_finish_values_refuse_without_mutation() {
        let mut v = fixture();
        v["finishing"] = json!({"cutting":v["main"].clone(),
            "lateral_allowance_mm":-0.1,"floor_allowance_mm":0.2});
        v["main"]["rpm"] = Value::Null;
        let parsed: MainFinishing = serde_json::from_value(v.clone()).unwrap();
        assert_eq!(parsed.numeric_errors().len(), 2);
        assert_eq!(serde_json::to_value(parsed).unwrap(), v);
    }
}
