//! Explicit v10 stage dispatch. Geometry tags never choose authored stage rates.
use crate::cam::{setup::Setup, VcObject};
use crate::cutting_stages::execution_op;
use crate::gcode::{emit_polylines_block, PostProcessor};
use crate::geometry::Point2;
use crate::pipeline::{CancelToken, PipelineError, PipelineWarning};
use crate::project::{Op, Project};

#[allow(clippy::too_many_arguments)]
pub(super) fn run<P: PostProcessor>(
    op: &Op,
    project: &Project,
    objects: &[VcObject],
    _setup: &Setup,
    post: &mut P,
    last_pos: &mut Point2,
    warnings: &mut Vec<PipelineWarning>,
    cancel: Option<&CancelToken>,
) -> Result<(usize, usize, bool), PipelineError> {
    let stages = op
        .params
        .stages
        .as_ref()
        .ok_or_else(|| PipelineError::AppliedCutting(op.id, "Missing stage contract".into()))?;
    // Plan both stages before emitting either. Failed finishing never publishes a
    // seemingly complete main-only program.
    let mut plans = Vec::new();
    for (name, stage, finishing) in std::iter::once(("main", stages.main(), false))
        .chain(stages.finishing().map(|f| ("finishing", &f.cutting, true)))
    {
        if crate::pipeline::cancelled(cancel) {
            return Err(PipelineError::Cancelled);
        }
        let mut view = execution_op(op, stage);
        if let Some(c) = view.contour_params_mut() {
            let direction = if finishing {
                c.finish_cut_direction
            } else {
                c.cut_direction
            };
            c.cut_direction = direction;
            c.finish_cut_direction = direction;
        }
        if !finishing {
            if let Some(f) = stages.finishing() {
                view.params.stock_to_leave_mm =
                    op.params.stock_to_leave_mm + f.lateral_allowance_mm;
                view.params.depth = op.params.depth + f.floor_allowance_mm;
            }
        }
        let mut setup =
            crate::pipeline::setup_resolver::synthesize_op_setup(&view, project, warnings)?;
        crate::pipeline::setup_resolver::resolve_auto_helix_radius(
            &view, objects, &mut setup, warnings,
        );
        let (mut offsets, closed) =
            super::build_op_offsets(&view, project, objects, &setup, warnings, cancel)?;
        if stages.finishing().is_some() {
            use crate::project::{
                LeadKind, OpKind, PlungeStrategy, TabPlacementMode, ToolKind, ToolOffset,
            };
            let fail = |e: String| {
                if crate::pipeline::cancelled(cancel) {
                    PipelineError::Cancelled
                } else {
                    PipelineError::FinishingProof(op.id, e)
                }
            };
            let selected = crate::pipeline::ordered_selection(op, objects);
            if selected.len() != 1
                || op.pattern().is_some()
                || !matches!(op.params.plunge, PlungeStrategy::Direct)
            {
                return Err(fail(
                    "Finishing proof requires one selected unpatterned contour (no islands) with direct entry"
                        .into(),
                ));
            }
            let c = op
                .contour_params()
                .ok_or_else(|| fail("Missing contour".into()))?;
            if c.tabs.active
                || !matches!(c.tab_mode, TabPlacementMode::Off)
                || !c.tab_placements.is_empty()
                || c.approach_point.is_some()
                || !matches!(c.leads.r#in, LeadKind::Off)
                || !matches!(c.leads.out, LeadKind::Off)
            {
                return Err(fail("Finishing with authored holding/approach/leads is not yet proven; placements were preserved".into()));
            }
            let tool = project
                .tools
                .iter()
                .find(|t| t.id == stage.tool_id)
                .ok_or(PipelineError::UnknownTool(op.id, stage.tool_id))?;
            if tool.kind != ToolKind::Endmill
                || op.profile_params().is_some_and(|p| p.helix || p.overcut)
            {
                return Err(fail(
                    "Finishing proof requires a flat endmill and ordinary depth passes".into(),
                ));
            }
            let pocket = matches!(op.kind, OpKind::Pocket { .. });
            let radius = tool.effective_diameter() * 0.5;
            let inset = radius + view.params.stock_to_leave_mm;
            if let Ok(bounds) = super::stage_coverage::rectangle(&objects[selected[0]]) {
                if matches!(
                    op.kind,
                    OpKind::Profile {
                        offset: ToolOffset::Outside,
                        ..
                    }
                ) {
                    super::stage_coverage::prove_outside(bounds, inset, &offsets).map_err(fail)?;
                } else if pocket
                    || matches!(
                        op.kind,
                        OpKind::Profile {
                            offset: ToolOffset::Inside,
                            ..
                        }
                    )
                {
                    let inside = [
                        bounds[0] + inset,
                        bounds[1] + inset,
                        bounds[2] - inset,
                        bounds[3] - inset,
                    ];
                    super::stage_coverage::prove(inside, radius, &offsets, pocket, cancel)
                        .map_err(fail)?;
                } else {
                    return Err(fail(
                    "On-contour finishing is not qualified; choose an explicit compensated side"
                        .into(),
                ));
                }
            } else {
                let outside = matches!(
                    op.kind,
                    OpKind::Profile {
                        offset: ToolOffset::Outside,
                        ..
                    }
                );
                if !outside
                    && !pocket
                    && !matches!(
                        op.kind,
                        OpKind::Profile {
                            offset: ToolOffset::Inside,
                            ..
                        }
                    )
                {
                    return Err(fail("On-contour finishing is not qualified; choose an explicit compensated side".into()));
                }
                let polygon =
                    super::stage_polygon::Polygon::new(&objects[selected[0]]).map_err(fail)?;
                let paths = super::stage_polygon::planned(&offsets).map_err(fail)?;
                polygon
                    .prove(
                        radius,
                        inset,
                        outside,
                        pocket,
                        &paths,
                        super::stage_polygon::PLAN_ERROR,
                        cancel,
                    )
                    .map_err(fail)?;
            }
            if !pocket {
                let f = stages.finishing().expect("checked");
                let main_tool = project
                    .tools
                    .iter()
                    .find(|t| t.id == stages.main().tool_id)
                    .ok_or(PipelineError::UnknownTool(op.id, stages.main().tool_id))?;
                let finish_tool = project
                    .tools
                    .iter()
                    .find(|t| t.id == f.cutting.tool_id)
                    .ok_or(PipelineError::UnknownTool(op.id, f.cutting.tool_id))?;
                // Equal authored effective cutters: do not infer equivalence with an epsilon.
                #[allow(clippy::float_cmp)]
                if main_tool.effective_diameter() != finish_tool.effective_diameter()
                    || f.lateral_allowance_mm > radius
                {
                    return Err(fail("Profile finishing band requires equal cutter diameters and allowance no larger than radius".into()));
                }
            }
        }
        for offset in &mut offsets {
            offset.is_finish = false;
        }
        plans.push((name, setup, offsets, closed));
    }
    let mut count = 0;
    let mut closed_count = 0;
    post.raw(&format!("; OP {}", op.id));
    let mut swapped = false;
    for (name, setup, offsets, closed) in plans {
        if name == "finishing" && setup.tool.number != stages.main().tool_id {
            let tool = project.tools.iter().find(|t| t.id == setup.tool.number);
            crate::pipeline::emit_toolchange_envelope(
                post,
                &project.machine,
                &setup,
                tool,
                setup.tool.number,
                false,
                Some(setup.tool.speed),
                Some((setup.tool.rate_h, setup.tool.rate_v)),
            );
            swapped = true;
        }
        post.raw(&format!("; FABSTAGE_1 {} {name}", op.id));
        count += offsets.len();
        closed_count = closed_count.max(closed);
        emit_polylines_block(&setup, &offsets, post, last_pos);
        post.raw(&format!("; FABSTAGE_1 {} end", op.id));
    }
    Ok((closed_count, count, swapped))
}
