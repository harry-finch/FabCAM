//! Scaffold-ownership context types — the structured data
//! [`crate::gcode::PostProcessor`] hooks receive when a post owns the
//! program envelope itself (`capabilities().owns_scaffold`, i.e. the
//! `.cps` recorder) instead of having the pipeline emit dialect
//! scaffolding through `emit_program_begin`/`emit_toolchange_envelope`.
//!
//! Everything here is plain data and compiles in every build — the
//! `cps` feature gates the recorder that consumes it, not the seam.

use std::collections::HashMap;

use crate::cam::setup::Setup;
use crate::pipeline::CpsParamValue;
use crate::project::{
    Coolant, MachineConfig, Op, OpKind, PassKind, Project, SpindleDirection, ToolEntry, ToolKind,
    UnitSystem, Wcs,
};

/// What a post implementation takes over from the pipeline. Returned by
/// [`crate::gcode::PostProcessor::capabilities`]; the default (all
/// `false`) keeps every existing dialect byte-identical.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PostCaps {
    /// The post owns program begin/end, tool-change envelopes, group
    /// markers and program-only scaffolding — the pipeline calls the
    /// structured hooks instead of emitting dialect text.
    pub owns_scaffold: bool,
}

/// Program-level context for [`crate::gcode::PostProcessor::begin_program`].
#[derive(Debug, Clone)]
pub struct ProgramCtx<'a> {
    pub unit: UnitSystem,
    pub wcs: Wcs,
    /// Program-wide rapid/clearance Z (mm).
    pub fast_move_z: f64,
    pub machine: &'a MachineConfig,
    /// Numeric-friendly default ("1001") — FANUC-style posts validate
    /// `getAsInt(programName)`. A user-facing knob can land later
    /// without touching this seam.
    pub program_name: String,
    pub program_comment: String,
}

/// Per-operation context for [`crate::gcode::PostProcessor::begin_section`].
/// One section per op — Fusion's model — even when the tool repeats.
#[derive(Debug, Clone)]
pub struct SectionCtx {
    pub op_id: u32,
    pub op_name: String,
    /// Autodesk-style strategy slug (`"contour2d"`, `"drill"`, …).
    pub strategy: &'static str,
    /// Op group name; replaces the `; === GROUP ===` marker lines the
    /// pipeline emits for dialect posts.
    pub group: Option<String>,
    pub tool: CpsToolInfo,
    pub wcs: Wcs,
    /// Section parameter stream with Autodesk names
    /// (`operation-comment`, `operation:tool_feedCutting`, …), in
    /// replay order.
    pub params: Vec<(String, CpsParamValue)>,
}

/// Mid-section tool swap (dual-tool rough→finish, drill→chamfer) for
/// [`crate::gcode::PostProcessor::mid_section_toolchange`].
#[derive(Debug, Clone)]
pub struct SectionToolCtx {
    pub tool: CpsToolInfo,
    pub rate_h: u32,
    pub rate_v: u32,
}

/// Tool description in post-facing terms (mm / radians / rpm).
#[derive(Debug, Clone)]
pub struct CpsToolInfo {
    pub project_tool_id: u32,
    /// `ToolEntry.id` — the project-level tool key (what T-words use).
    pub number: u32,
    pub description: String,
    pub kind: ToolKind,
    pub diameter: f64,
    /// Ball nose → radius, bull nose → its fillet radius, else 0.
    pub corner_radius: f64,
    /// V-bits / engravers: half the tip angle, radians. Else 0.
    pub taper_angle: f64,
    pub flutes: u32,
    /// Clamped to the machine's RPM window.
    pub spindle_rpm: u32,
    pub spindle_clockwise: bool,
    pub coolant: Coolant,
}

/// Program-flow events for [`crate::gcode::PostProcessor::program_event`]
/// (program-only ops that are not motion).
#[derive(Debug, Clone)]
pub enum ProgramEventCtx {
    /// Operator stop (Pause op). `optional` mirrors the machine's
    /// M0-vs-M1 choice.
    Stop { message: String, optional: bool },
    /// Verbatim lines (GcodeInclude), already variable-expanded.
    PassThrough { lines: Vec<String> },
}

pub(crate) fn build_program_ctx<'a>(project: &'a Project, header_setup: &Setup) -> ProgramCtx<'a> {
    ProgramCtx {
        unit: header_setup.machine.unit,
        wcs: header_setup.wcs,
        fast_move_z: header_setup.mill.fast_move_z,
        machine: &project.machine,
        program_name: "1001".to_string(),
        program_comment: "ivaCAM".to_string(),
    }
}

pub(crate) fn build_section_ctx(
    op: &Op,
    project: &Project,
    tool_index: &HashMap<u32, &ToolEntry>,
    header_setup: &Setup,
) -> SectionCtx {
    let tool_entry = tool_index.get(&op.tool_id).copied();
    let pass = op_pass_kind(op);
    let resolved = super::setup_resolver::synthesize_op_setup(op, project, &mut Vec::new()).ok();
    let (rpm, rate_v, rate_h) = resolved.map_or((0, 0, 0), |s| {
        if pass == PassKind::Finish {
            (
                s.tool.speed_finish,
                s.tool.rate_v_finish,
                s.tool.rate_h_finish,
            )
        } else {
            (s.tool.speed, s.tool.rate_v, s.tool.rate_h)
        }
    });
    // Authored milling intent travels to posts as transient section metadata, not
    // machine-specific geometry or new persisted operation fields.
    let entry = if matches!(
        op.kind,
        OpKind::Pocket {
            strategy: crate::project::PocketStrategy::Trochoidal { .. },
            ..
        }
    ) {
        "helix"
    } else {
        match op.params.plunge {
            crate::project::PlungeStrategy::Direct => "direct",
            crate::project::PlungeStrategy::Ramp { .. } => "ramp",
            crate::project::PlungeStrategy::Helix { .. } => "helix",
        }
    };
    let contour = op.contour_params();
    let has_tabs = contour.is_some_and(|c| {
        c.tabs.active || !matches!(c.tab_mode, crate::project::TabPlacementMode::Off)
    });
    let has_leads = contour.is_some_and(|c| {
        c.leads.r#in != crate::project::LeadKind::Off
            || c.leads.out != crate::project::LeadKind::Off
    });
    let path_helix = match &op.kind {
        OpKind::Profile { profile, .. } => profile.helix,
        _ => false,
    };
    let params = vec![
        (
            "ivacam:entry-start-z".into(),
            CpsParamValue::Number(op.params.start_depth),
        ),
        (
            "ivacam:wcs-x".into(),
            CpsParamValue::Number(project.work_offset.x_mm),
        ),
        (
            "ivacam:wcs-y".into(),
            CpsParamValue::Number(project.work_offset.y_mm),
        ),
        (
            "ivacam:wcs-z".into(),
            CpsParamValue::Number(project.work_offset.z_mm),
        ),
        (
            "ivacam:entry-strategy".into(),
            CpsParamValue::Text(entry.into()),
        ),
        (
            "ivacam:entry-has-tabs".into(),
            CpsParamValue::Bool(has_tabs),
        ),
        (
            "ivacam:entry-has-leads".into(),
            CpsParamValue::Bool(has_leads),
        ),
        ("ivacam:path-helix".into(), CpsParamValue::Bool(path_helix)),
        (
            "ivacam:uses-static-tool-z-shift".to_string(),
            CpsParamValue::Bool(
                project
                    .tools
                    .iter()
                    .any(|t| t.z_shift_mm.unwrap_or(0.0) != 0.0),
            ),
        ),
        (
            "operation-comment".to_string(),
            CpsParamValue::Text(op.name.clone()),
        ),
        (
            "operation:tool_feedCutting".to_string(),
            CpsParamValue::Number(f64::from(rate_h)),
        ),
        (
            "operation:tool_feedPlunge".to_string(),
            CpsParamValue::Number(f64::from(rate_v)),
        ),
        (
            "operation:tool_feedEntry".to_string(),
            CpsParamValue::Number(f64::from(rate_h)),
        ),
        (
            "operation:tool_feedExit".to_string(),
            CpsParamValue::Number(f64::from(rate_h)),
        ),
    ];
    SectionCtx {
        op_id: op.id,
        op_name: op.name.clone(),
        strategy: op_strategy_slug(&op.kind),
        group: op
            .group
            .as_deref()
            .filter(|g| !g.is_empty())
            .map(str::to_owned),
        tool: cps_tool_info(tool_entry, op.tool_id, rpm),
        wcs: header_setup.wcs,
        params,
    }
}

pub(crate) fn build_section_tool_ctx(
    machine: &MachineConfig,
    new_tool: Option<&ToolEntry>,
    new_tool_id: u32,
    target_speed: Option<u32>,
    rates: Option<(u32, u32)>,
) -> SectionToolCtx {
    let rpm = super::setup_resolver::clamp_rpm_silent(
        target_speed
            .or_else(|| new_tool.map(|t| t.speed))
            .unwrap_or(0),
        machine,
    );
    let (rate_h, rate_v) =
        rates.unwrap_or_else(|| new_tool.map_or((0, 0), |t| (t.feed_rate, t.plunge_rate)));
    SectionToolCtx {
        rate_h,
        rate_v,
        tool: cps_tool_info(new_tool, new_tool_id, rpm),
    }
}

/// Post-facing tool info from a library entry. A missing entry (the
/// pipeline errors with `UnknownTool` moments later) still yields a
/// well-formed placeholder so the hook signature stays simple.
fn cps_tool_info(entry: Option<&ToolEntry>, fallback_id: u32, rpm: u32) -> CpsToolInfo {
    let Some(t) = entry else {
        return CpsToolInfo {
            project_tool_id: fallback_id,
            number: fallback_id,
            description: String::new(),
            kind: ToolKind::Endmill,
            diameter: 0.0,
            corner_radius: 0.0,
            taper_angle: 0.0,
            flutes: 0,
            spindle_rpm: rpm,
            spindle_clockwise: true,
            coolant: Coolant::Off,
        };
    };
    let corner_radius = match t.kind {
        ToolKind::BallNose => t.effective_diameter() / 2.0,
        ToolKind::BullNose => t.corner_radius_mm.unwrap_or(0.0).max(0.0),
        _ => 0.0,
    };
    let taper_angle = match t.kind {
        ToolKind::VBit | ToolKind::Engraver => (t.tip_angle_deg / 2.0).to_radians(),
        _ => 0.0,
    };
    CpsToolInfo {
        project_tool_id: t.id,
        number: t.id,
        description: t.name.clone(),
        kind: t.kind,
        diameter: t.effective_diameter(),
        corner_radius,
        taper_angle,
        flutes: u32::from(t.flutes),
        spindle_rpm: rpm,
        spindle_clockwise: t.spindle_direction == SpindleDirection::Cw,
        coolant: t.coolant,
    }
}

/// Rate set the section parameters advertise: drill ops use the drill
/// overrides, everything else the rough set (what the drivers command
/// for the primary cut).
fn op_pass_kind(op: &Op) -> PassKind {
    if matches!(op.kind, OpKind::Drill { .. }) {
        PassKind::Drill
    } else if matches!(op.kind, OpKind::Profile { .. }) {
        PassKind::Finish
    } else {
        PassKind::Rough
    }
}

/// Autodesk-style strategy slug for `Section.strategy`. Informational
/// for posts (they branch on cycles/parameters far more than on this),
/// so nearest-neighbour naming is fine.
fn op_strategy_slug(kind: &OpKind) -> &'static str {
    match kind {
        OpKind::Profile { .. } => "contour2d",
        OpKind::Pocket { .. } => "pocket2d",
        OpKind::Drill { .. } => "drill",
        OpKind::Thread { .. } => "thread",
        OpKind::Chamfer { .. } => "chamfer2d",
        OpKind::VCarve { .. } => "vcarve",
        OpKind::Engrave { .. } | OpKind::RasterEngrave { .. } => "engrave",
        OpKind::DragKnife { .. } => "trace",
        OpKind::TSlot { .. } | OpKind::Dovetail { .. } => "slot",
        OpKind::PlanarSurface { .. } => "planar_surface",
        OpKind::ReliefMill { .. } => "parallel",
        OpKind::WaterlineRough { .. } => "contour",
        // Program-only kinds never build a section; anything new lands
        // here until it gets a dedicated slug.
        _ => "milling",
    }
}

#[cfg(test)]
mod lot8_tests {
    use super::*;
    #[test]
    fn lot8_cps_reports_the_same_effective_diameter_as_path_compensation() {
        let mut tool = crate::pipeline::test_helpers::endmill(1, 6.);
        tool.kind = ToolKind::BallNose;
        tool.wear_offset_mm = 0.2;
        let info = cps_tool_info(Some(&tool), 1, 12000);
        assert!((info.diameter - 5.8).abs() < 1e-12);
        assert!((info.corner_radius - 2.9).abs() < 1e-12);
        assert_eq!(info.spindle_rpm, 12000);
        assert!((tool.diameter - 6.).abs() < 1e-12);
    }
}
