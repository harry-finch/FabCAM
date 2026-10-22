//! Workshop attestation and reviewed copies. No matching, ranking or calculation engine.
use crate::cutting_parameters::{validate_operation, MillingCutting, RoleRates};
use crate::project::{Op, Project};
mod specialized;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use specialized::SpecializedParameters;

pub const MAX_COLLECTION_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_PRESETS: usize = 256;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PresetContext {
    pub machine_id: String,
    pub machine_label: String,
    pub material_id: String,
    pub material_label: String,
    /// Canonical relevant setup, tool geometry, strategy and entry snapshot.
    pub signature: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DepthSchedule {
    Step {
        step: Option<f64>,
        finish_step: Option<f64>,
    },
    Explicit {
        depths: Vec<f64>,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PresetParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialized: Option<SpecializedParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_angle_deg: Option<f64>,
    pub rates: Vec<RoleRates>,
    pub schedule: DepthSchedule,
    pub pocket_overlap: Option<f64>,
    pub planar_stepover_mm: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TestedPreset {
    pub id: String,
    pub name: String,
    pub context: PresetContext,
    pub parameters: PresetParameters,
    pub tested: bool,
    pub test_date: Option<String>,
    pub notes: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PresetCollection {
    pub format: String,
    pub version: u32,
    pub id: String,
    pub name: String,
    pub presets: Vec<TestedPreset>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LoadedPresetSnapshot {
    pub collection_id: String,
    pub collection_name: String,
    pub preset_id: String,
    pub preset_name: String,
    pub context: PresetContext,
    pub parameters: PresetParameters,
}

#[must_use]
pub fn safe_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 96
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn text(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max && !value.contains('\0')
}

pub fn parse_collection(json: &str) -> Result<PresetCollection, String> {
    if json.len() > MAX_COLLECTION_BYTES {
        return Err("Collection exceeds 2 MiB".into());
    }
    let collection: PresetCollection = serde_json::from_str(json).map_err(|e| e.to_string())?;
    validate_collection(&collection)?;
    Ok(collection)
}
pub fn validate_collection(c: &PresetCollection) -> Result<(), String> {
    if c.format != "fabcam-preset-collection" || c.version != 1 {
        return Err("Unsupported preset collection format/version".into());
    }
    if !safe_id(&c.id) || !text(&c.name, 160) || c.presets.len() > MAX_PRESETS {
        return Err("Invalid collection ID, name or preset count (maximum 256)".into());
    }
    let mut ids = std::collections::BTreeSet::new();
    for p in &c.presets {
        if !safe_id(&p.id) || !ids.insert(&p.id) || !text(&p.name, 160) || !p.tested {
            return Err(
                "Presets require unique safe IDs, a name and explicit tested attestation".into(),
            );
        }
        if !text(&p.context.machine_id, 256)
            || !text(&p.context.material_id, 256)
            || !text(&p.context.machine_label, 160)
            || !text(&p.context.material_label, 160)
            || !text(&p.context.signature, 65536)
            || p.notes.as_ref().is_some_and(|v| v.len() > 4096)
            || p.test_date.as_ref().is_some_and(|v| v.len() > 32)
        {
            return Err("Invalid or oversized preset context/metadata".into());
        }
        let _: serde_json::Value =
            serde_json::from_str(&p.context.signature).map_err(|e| e.to_string())?;
        validate_parameters(&p.parameters)?;
    }
    Ok(())
}
fn validate_parameters(p: &PresetParameters) -> Result<(), String> {
    if let Some(s) = &p.specialized {
        s.validate()?;
    }
    let mut roles = std::collections::BTreeSet::new();
    if p.rates.is_empty() || p.rates.len() > 5 {
        return Err("Invalid role count".into());
    }
    for r in &p.rates {
        if !roles.insert(r.role)
            || [r.rpm, r.feed_mm_min, r.plunge_mm_min]
                .iter()
                .any(|v| v.map_or(true, |v| v == 0))
        {
            return Err(
                "Preset roles must be unique and complete with positive integer values".into(),
            );
        }
    }
    match &p.schedule {
        DepthSchedule::Step { step, finish_step } => {
            if step
                .iter()
                .chain(finish_step)
                .any(|v| !v.is_finite() || *v >= -1e-9)
            {
                return Err("Invalid preset depth per pass".into());
            }
        }
        DepthSchedule::Explicit { depths } => {
            if depths.is_empty()
                || depths.len() > 4096
                || depths.iter().any(|v| !v.is_finite())
                || depths.windows(2).any(|v| v[1] >= v[0])
            {
                return Err("Invalid explicit depth schedule".into());
            }
        }
    }
    if p.entry_angle_deg
        .is_some_and(|v| !v.is_finite() || !(1.0..=89.0).contains(&v))
    {
        return Err("Entry angle must be finite and within 1–89 degrees".into());
    }
    if p.pocket_overlap
        .is_some_and(|v| !v.is_finite() || v <= 0.0 || v >= 1.0)
        || p.planar_stepover_mm
            .is_some_and(|v| !v.is_finite() || v <= 0.0)
    {
        return Err("Invalid lateral engagement".into());
    }
    Ok(())
}

pub fn parameters(op: &Op) -> Result<PresetParameters, String> {
    use crate::project::OpKind;
    Ok(PresetParameters {
        specialized: SpecializedParameters::capture(op),
        entry_angle_deg: op.params.cutting.as_ref().and_then(|c| c.entry_angle_deg),
        rates: op
            .params
            .cutting
            .as_ref()
            .ok_or("Missing cutting parameters")?
            .rates
            .clone(),
        schedule: if op.params.depth_list.is_empty() {
            DepthSchedule::Step {
                step: op.params.step,
                finish_step: op.params.finish_step,
            }
        } else {
            DepthSchedule::Explicit {
                depths: op.params.depth_list.clone(),
            }
        },
        pocket_overlap: op.pocket_params().map(|p| p.xy_overlap),
        planar_stepover_mm: match &op.kind {
            OpKind::PlanarSurface { surface } => Some(surface.stepover_mm),
            _ => None,
        },
    })
}

/// Stable physical IDs and relevant snapshots, never label/family equivalence.
pub fn context(project: &Project, op: &Op) -> Result<PresetContext, String> {
    let machine_id = project
        .machine
        .physical_id
        .clone()
        .filter(|v| !v.is_empty())
        .ok_or("Set the physical machine identity before saving a tested preset")?;
    let material = project
        .material
        .as_ref()
        .ok_or("Select the exact Stock material before saving a tested preset")?;
    let mut tools = vec![];
    for (role, id) in crate::cutting_parameters::roles(op) {
        let tool = project
            .tools
            .iter()
            .find(|t| t.id == id)
            .ok_or("Assigned tool is missing")?;
        let physical = tool
            .physical_id
            .as_ref()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| format!("Set the physical identity for {role:?} tool {}", tool.name))?;
        let mut geometry = serde_json::to_value(tool).map_err(|e| e.to_string())?;
        if let Some(map) = geometry.as_object_mut() {
            for key in [
                "id",
                "name",
                "speed",
                "feed_rate",
                "plunge_rate",
                "speed_finish",
                "feed_rate_finish",
                "plunge_rate_finish",
                "speed_drill",
                "feed_rate_drill",
                "plunge_rate_drill",
                "max_rpm",
                "default_step",
                "default_xy_overlap",
                "default_peck_step_mm",
                "vcarve_lead_in_angle_deg",
                "last_calibrated",
            ] {
                map.remove(key);
            }
        }
        tools.push(serde_json::json!({"role":role,"physical_id":physical,"geometry":geometry}));
    }
    let mut kind = serde_json::to_value(&op.kind).map_err(|e| e.to_string())?;
    specialized::remove_copied_values(&mut kind);
    if let Some(p) = kind
        .get_mut("pocket")
        .and_then(serde_json::Value::as_object_mut)
    {
        p.remove("xy_overlap");
    }
    if let Some(p) = kind
        .get_mut("surface")
        .and_then(serde_json::Value::as_object_mut)
    {
        p.remove("stepover_mm");
    }
    // Tool numbers are local references; physical context above is authoritative.
    if let Some(p) = kind
        .get_mut("spot_first")
        .and_then(serde_json::Value::as_object_mut)
    {
        p.remove("spot_tool_id");
    }
    let mut machine_setup = serde_json::to_value(&project.machine).map_err(|e| e.to_string())?;
    if let Some(map) = machine_setup.as_object_mut() {
        for key in [
            "name",
            "spindle_rpm_min",
            "spindle_rpm_max",
            "max_feed_mm_min",
        ] {
            map.remove(key);
        }
    }
    let signature=serde_json::to_string(&serde_json::json!({"machine":machine_id,"machine_setup":machine_setup,"mode":project.machine.mode,"material":material,"tools":tools,"operation":kind,"plunge":op.params.plunge})).map_err(|e|e.to_string())?;
    Ok(PresetContext {
        machine_id: machine_id.clone(),
        machine_label: if project.machine.name.trim().is_empty() {
            machine_id.clone()
        } else {
            project.machine.name.clone()
        },
        material_id: material.id.clone(),
        material_label: material.id.clone(),
        signature,
    })
}

pub fn load(
    project: &Project,
    op: &Op,
    collection: &PresetCollection,
    id: &str,
) -> Result<Op, String> {
    validate_collection(collection)?;
    let p = collection
        .presets
        .iter()
        .find(|p| p.id == id)
        .ok_or("Preset not found")?;
    let current = context(project, op)?;
    if current.signature != p.context.signature
        || current.machine_id != p.context.machine_id
        || current.material_id != p.context.material_id
    {
        return Err(
            "Preset context differs: review machine, material, tool geometry and operation".into(),
        );
    }
    let mut next = op.clone();
    let assignments = crate::cutting_parameters::roles(op);
    let mut parameters = p.parameters.clone();
    for r in &mut parameters.rates {
        r.tool_id = assignments
            .iter()
            .find(|(role, _)| *role == r.role)
            .ok_or("Preset role is not supported")?
            .1;
    }
    next.params.cutting = Some(MillingCutting {
        rates: parameters.rates.clone(),
        entry_angle_deg: parameters.entry_angle_deg,
        preset_snapshot: None,
    });
    match &parameters.schedule {
        DepthSchedule::Step { step, finish_step } => {
            next.params.step = *step;
            next.params.finish_step = *finish_step;
            next.params.depth_list.clear();
        }
        DepthSchedule::Explicit { depths } => {
            next.params.depth_list.clone_from(depths);
            next.params.step = None;
            next.params.finish_step = None;
        }
    }
    if let Some(overlap) = parameters.pocket_overlap {
        next.pocket_params_mut()
            .ok_or("Pocket values on non-pocket operation")?
            .xy_overlap = overlap;
    }
    if let Some(step) = parameters.planar_stepover_mm {
        if let crate::project::OpKind::PlanarSurface { surface } = &mut next.kind {
            surface.stepover_mm = step;
        } else {
            return Err("Surfacing values on another operation".into());
        }
    }
    match &parameters.specialized {
        Some(values) => values.apply(&mut next)?,
        None if SpecializedParameters::capture(&next).is_some() => {
            return Err("Preset is missing specialized operation values".into())
        }
        None => {}
    }
    let report = validate_operation(project, &next);
    if !report.errors.is_empty() {
        return Err(report.errors.join("; "));
    }
    next.params.cutting.as_mut().unwrap().preset_snapshot = Some(LoadedPresetSnapshot {
        collection_id: collection.id.clone(),
        collection_name: collection.name.clone(),
        preset_id: p.id.clone(),
        preset_name: p.name.clone(),
        context: current,
        parameters,
    });
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn collection() -> PresetCollection {
        PresetCollection {
            format: "fabcam-preset-collection".into(),
            version: 1,
            id: "workshop".into(),
            name: "Workshop".into(),
            presets: vec![],
        }
    }
    fn example() -> (Project, PresetCollection) {
        let mut p = Project::default();
        p.cutting_version = Some(9);
        p.machine.physical_id = Some("machine-lab".into());
        p.machine.name = "Lab router".into();
        p.material = Some(serde_json::from_str(r#"{"id":"wood-plywood","revision":"1"}"#).unwrap());
        let tool = crate::project::ToolEntry {
            physical_id: Some("tool-lab".into()),
            ..crate::project::ToolEntry::default()
        };
        let mut op = Op::default();
        op.params.step = Some(-1.0);
        op.params.cutting = Some(crate::cutting_parameters::seed(
            &op,
            std::slice::from_ref(&tool),
        ));
        p.tools.push(tool);
        p.operations.push(op);
        let mut c = collection();
        c.presets.push(TestedPreset {
            id: "tested-profile".into(),
            name: "Plywood profile".into(),
            context: context(&p, &p.operations[0]).unwrap(),
            parameters: parameters(&p.operations[0]).unwrap(),
            tested: true,
            test_date: None,
            notes: Some("Private workshop notes".into()),
        });
        (p, c)
    }
    #[test]
    fn specialized_cycles_copy_without_changing_role_assignments() {
        use crate::project::{DrillCycle, OpKind, SpotConfig};
        let (mut p, mut c) = example();
        p.operations[0].kind = OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: 0.7,
                dwell_sec: 0.2,
            },
            spot_first: Some(SpotConfig {
                spot_tool_id: 1,
                spot_depth_mm: -0.3,
            }),
            chamfer_after_width_mm: None,
            pattern: None,
        };
        p.operations[0].params.cutting =
            Some(crate::cutting_parameters::seed(&p.operations[0], &p.tools));
        c.presets[0].context = context(&p, &p.operations[0]).unwrap();
        c.presets[0].parameters = parameters(&p.operations[0]).unwrap();
        let expected = p.operations[0].kind.clone();
        if let OpKind::Drill {
            cycle, spot_first, ..
        } = &mut p.operations[0].kind
        {
            *cycle = DrillCycle::ChipBreak {
                peck_step_mm: 1.2,
                dwell_sec: 0.,
            };
            spot_first.as_mut().unwrap().spot_depth_mm = -0.5;
        }
        let restored = load(&p, &p.operations[0], &c, "tested-profile").unwrap();
        assert_eq!(
            serde_json::to_value(restored.kind).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
        assert_eq!(restored.tool_id, p.operations[0].tool_id);
        let roundtrip = parse_collection(&serde_json::to_string(&c).unwrap()).unwrap();
        assert_eq!(roundtrip, c);
        c.presets[0].parameters.specialized = None;
        assert!(load(&p, &p.operations[0], &c, "tested-profile").is_err());
        c.presets[0].parameters.specialized = Some(SpecializedParameters::Waterline {
            z_step_mm: 1.,
            stepover_mm: 1.,
        });
        assert!(load(&p, &p.operations[0], &c, "tested-profile").is_err());
        c.presets[0].parameters.specialized = Some(SpecializedParameters::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: 0.,
                dwell_sec: 0.,
            },
            spot_depth_mm: Some(-0.3),
            chamfer_width_mm: None,
        });
        assert!(validate_collection(&c).is_err());
    }

    #[test]
    fn exact_reviewed_load_is_a_copy_without_private_notes() {
        let (p, c) = example();
        let before = serde_json::to_string(&p).unwrap();
        let op = load(&p, &p.operations[0], &c, "tested-profile").unwrap();
        assert_eq!(parameters(&op).unwrap(), c.presets[0].parameters);
        assert!(!serde_json::to_string(&op)
            .unwrap()
            .contains("Private workshop notes"));
        assert_eq!(serde_json::to_string(&p).unwrap(), before);
        let mut changed = c.clone();
        changed.presets.clear();
        assert!(op.params.cutting.unwrap().preset_snapshot.is_some());
    }
    #[test]
    fn different_context_limits_or_unattested_data_refused() {
        let (mut p, mut c) = example();
        c.presets[0].tested = false;
        assert!(load(&p, &p.operations[0], &c, "tested-profile").is_err());
        c.presets[0].tested = true;
        p.tools[0].diameter += 1.0;
        assert!(load(&p, &p.operations[0], &c, "tested-profile").is_err());
        p.tools[0].diameter -= 1.0;
        p.tools[0].max_rpm = Some(1);
        assert!(load(&p, &p.operations[0], &c, "tested-profile").is_err());
    }
    #[test]
    fn local_tool_numbers_do_not_establish_physical_compatibility() {
        let (mut p, c) = example();
        p.tools[0].id = 20;
        p.operations[0].tool_id = 20;
        for r in &mut p.operations[0].params.cutting.as_mut().unwrap().rates {
            r.tool_id = 20;
        }
        let next = load(&p, &p.operations[0], &c, "tested-profile").unwrap();
        assert!(next
            .params
            .cutting
            .unwrap()
            .rates
            .iter()
            .all(|r| r.tool_id == 20));
        p.tools[0].physical_id = Some("different-physical-tool".into());
        assert!(load(&p, &p.operations[0], &c, "tested-profile").is_err());
    }
    #[test]
    fn empty_named_collection_roundtrip() {
        let c = collection();
        assert_eq!(
            parse_collection(&serde_json::to_string(&c).unwrap()).unwrap(),
            c
        );
    }
    #[test]
    fn bounded_strict_collection_parser() {
        let mut c = collection();
        c.id = "../escape".into();
        assert!(validate_collection(&c).is_err());
        c = collection();
        c.version = 2;
        assert!(validate_collection(&c).is_err());
        assert!(parse_collection(&" ".repeat(MAX_COLLECTION_BYTES + 1)).is_err());
        assert!(parse_collection("{\"format\":\"source-pack\"}").is_err());
        let mut value = serde_json::to_value(collection()).unwrap();
        value["extra"] = true.into();
        assert!(parse_collection(&value.to_string()).is_err());
    }
}

/// Identical pure validation/copy contract across native, HTTP and WASM.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum PresetCommand {
    ValidateCollection {
        json: String,
    },
    Inspect {
        project: Project,
        op_id: u32,
    },
    Load {
        project: Project,
        op_id: u32,
        collection: PresetCollection,
        preset_id: String,
    },
}
pub fn command(request: PresetCommand) -> Result<serde_json::Value, String> {
    match request {
        PresetCommand::ValidateCollection { json } => {
            serde_json::to_value(parse_collection(&json)?).map_err(|e| e.to_string())
        }
        PresetCommand::Inspect { project, op_id } => {
            let op = project
                .operations
                .iter()
                .find(|o| o.id == op_id)
                .ok_or("Operation not found")?;
            let context = context(&project, op);
            Ok(
                serde_json::json!({"check":validate_operation(&project,op),"parameters":parameters(op)?,"context":context.as_ref().ok(),"context_error":context.as_ref().err()}),
            )
        }
        PresetCommand::Load {
            project,
            op_id,
            collection,
            preset_id,
        } => {
            let op = project
                .operations
                .iter()
                .find(|o| o.id == op_id)
                .ok_or("Operation not found")?;
            serde_json::to_value(load(&project, op, &collection, &preset_id)?)
                .map_err(|e| e.to_string())
        }
    }
}
