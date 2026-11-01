//! SAFETY-FIX-1 containment only. Never changes authored machining intent.
use super::{PipelineError, PipelineWarning};
use crate::cam::VcObject;
use crate::project::{
    LeadKind, Op, OpKind, OpSource, PlungeStrategy, PocketStrategy, Project, TabWidthMode,
    WorkpieceSide,
};

fn refuse(op: &Op, reason: &'static str, message: &'static str) -> PipelineError {
    PipelineError::MotionGuard(op.id, reason, message)
}

pub(super) fn validate_parameters(project: &Project) -> Result<(), PipelineError> {
    let two_sided = project.stock.as_ref().is_some_and(|s| s.flip.is_some())
        && project
            .operations
            .iter()
            .any(|o| o.enabled && o.side == WorkpieceSide::Back);
    if two_sided
        && project.stock.as_ref().is_some_and(|s| {
            !s.thickness_mm.is_finite() || s.thickness_mm <= 0. || s.top_z_mm != 0.
        })
    {
        return Err(PipelineError::MotionGuard(0, "two_sided_depth", "Two-sided holding requires positive known thickness and the qualified own-face Z-zero frame"));
    }
    for op in project.operations.iter().filter(|o| o.enabled) {
        // These standard emitters share the loop-only entry planner. Thread's
        // intrinsic machining helix and Drill cycles do not use that planner.
        if op.contour_params().is_some()
            && (matches!(op.params.plunge, PlungeStrategy::Helix { .. })
                || op.profile_params().is_some_and(|p| p.helix)
                || matches!(
                    op.kind,
                    OpKind::Pocket {
                        strategy: PocketStrategy::Trochoidal { .. },
                        ..
                    }
                ))
        {
            return Err(refuse(op, "helical_entry", "Helical entry/descent cannot be proven clear of retained geometry. Use a separately supported Direct or Ramp setup; no strategy was substituted"));
        }
        if op.profile_params().is_some_and(|p| p.overcut) {
            return Err(refuse(op, "legacy_overcut", "Legacy Profile overcut requires renewed winding and retained-side geometric qualification. Explicit CAD geometry is unaffected"));
        }
        if two_sided && op.side == WorkpieceSide::Back {
            let uses_text = !project.text_layers.is_empty()
                && match &op.source {
                    OpSource::Layers { layers, .. } => project
                        .text_layers
                        .iter()
                        .any(|t| layers.contains(&format!("__text_{}", t.id))),
                    OpSource::All | OpSource::Objects { .. } => true,
                };
            if uses_text {
                return Err(refuse(
                    op,
                    "back_text",
                    "Back editable text requires qualified mirroring before two-sided generation",
                ));
            }
            if op
                .contour_params()
                .is_some_and(|c| c.approach_point.is_some())
            {
                return Err(refuse(
                    op,
                    "back_approach",
                    "Authored Back approach requires qualified coordinate transformation",
                ));
            }
            if matches!(op.kind, OpKind::RasterEngrave { .. }) {
                return Err(refuse(
                    op,
                    "back_source",
                    "Back raster source placement requires qualified coordinate transformation",
                ));
            }
            if op.pattern().is_some() {
                return Err(refuse(
                    op,
                    "back_pattern",
                    "Back pattern placement requires qualified coordinate transformation",
                ));
            }
        }
    }
    Ok(())
}

/// Bound actual own-face reach, including depths synthesized outside common params.
pub(super) fn removal_mm(project: &Project, op: &Op) -> Result<f64, PipelineError> {
    let unbounded = match op.kind {
        OpKind::Pocket {
            strategy: PocketStrategy::Halfpipe { .. },
            ..
        } => !op.params.depth.is_finite() || op.params.depth >= -1e-9,
        OpKind::VCarve { .. } => !op.params.depth.is_finite() || op.params.depth >= 0.,
        _ => false,
    };
    if unbounded {
        return Err(refuse(
            op,
            "two_sided_depth",
            "Two-sided specialized removal requires an explicit qualified depth bound",
        ));
    }
    let setup = super::setup_resolver::synthesize_op_setup(op, project, &mut Vec::new())?;
    let depth = op
        .params
        .depth_list
        .iter()
        .copied()
        .fold(setup.mill.depth.min(setup.mill.start_depth), f64::min);
    let mut reach = (-depth).max(0.) + op.params.through_depth.max(0.);
    if let OpKind::Drill {
        spot_first,
        chamfer_after_width_mm,
        ..
    } = &op.kind
    {
        if let Some(spot) = spot_first {
            reach = reach.max((-spot.spot_depth_mm).max(0.) + op.params.through_depth.max(0.));
        }
        if let Some(width) = chamfer_after_width_mm.filter(|w| *w > 0.) {
            let id = op.finish_tool_id.unwrap_or(op.tool_id);
            let tool = project
                .tools
                .iter()
                .find(|t| t.id == id)
                .ok_or(PipelineError::UnknownTool(op.id, id))?;
            let cone = crate::cam::chamfer::chamfer_depth_capped(
                width,
                tool.tip_angle_deg,
                tool.effective_diameter(),
                tool.tip_diameter.unwrap_or(0.),
            );
            reach = reach.max(-cone.z);
        }
    }
    if !reach.is_finite() {
        return Err(refuse(
            op,
            "two_sided_depth",
            "Two-sided effective removal depth could not be proven",
        ));
    }
    Ok(reach)
}

/// Retained material, not an authored mode flag. Only the established ordinary
/// closed Profile emitter is accepted; Pocket/specialized holding is unproven.
/// 0.05 mm is a conservative positive-material proof margin above chord/output
/// precision, not a statement of physical holding strength.
pub(super) fn effective_holding(
    project: &Project,
    op: &Op,
    objects: &[VcObject],
    thickness: f64,
) -> bool {
    const MATERIAL_MARGIN_MM: f64 = 0.05;
    let Some(c) = op.contour_params() else {
        return false;
    };
    let uses_text = !project.text_layers.is_empty()
        && match &op.source {
            OpSource::Layers { layers, .. } => project
                .text_layers
                .iter()
                .any(|t| layers.contains(&format!("__text_{}", t.id))),
            OpSource::All | OpSource::Objects { .. } => true,
        };
    if uses_text
        || !matches!(op.kind, OpKind::Profile { .. })
        || op
            .params
            .stages
            .as_ref()
            .is_some_and(|s| s.finishing().is_some())
        || !matches!(op.params.plunge, PlungeStrategy::Direct)
        || !matches!(c.leads.r#in, LeadKind::Off)
        || !matches!(c.leads.out, LeadKind::Off)
    {
        return false;
    }
    let view = op.params.stages.as_ref().map_or_else(
        || op.clone(),
        |s| crate::cutting_stages::execution_op(op, s.main()),
    );
    let Ok(setup) = super::setup_resolver::synthesize_op_setup(&view, project, &mut Vec::new())
    else {
        return false;
    };
    let Ok((offsets, _)) = super::offset_builder::build_op_offsets(
        &view,
        project,
        objects,
        &setup,
        &mut Vec::<PipelineWarning>::new(),
        None,
    ) else {
        return false;
    };
    let floor = view
        .params
        .depth_list
        .iter()
        .copied()
        .fold(view.params.depth, f64::min)
        - view.params.through_depth.max(0.);
    setup.tabs.active
        && setup.tabs.width_mode == TabWidthMode::MaterialV1
        && offsets.len() == 1
        && offsets.iter().all(|path| {
            path.closed
                && !path.segments.is_empty()
                && path.tabs.iter().any(|tab| {
                    let width = tab.radius(setup.tabs.width) * 2.;
                    let material_width = width;
                    width.is_finite()
                        && material_width >= MATERIAL_MARGIN_MM
                        && tab.lift(setup.tabs.height.abs()).is_finite()
                        && floor + tab.lift(setup.tabs.height.abs()) + thickness
                            >= MATERIAL_MARGIN_MM
                })
        })
}
