//! Planar surfacing uses the common XYZ emitter; every scanline retracts.
use crate::cam::planar_surface::{planar_raster, PlanarRaster, MAX_SURFACE_LINES};
use crate::cam::setup::Setup;
use crate::gcode::{emit_vcarve_block, PostProcessor};
use crate::geometry::Point2;
use crate::pipeline::{CancelToken, PipelineError, PipelineWarning};
use crate::project::{MachineMode, Op, OpKind, Project, ToolKind, WorkpieceSide};

fn invalid(op: &Op, message: impl Into<String>) -> PipelineError {
    PipelineError::PlanarSurface(op.id, message.into())
}

/// Called by setup resolution BEFORE any cache replay or op emission.
pub(in crate::pipeline) fn validate_planar_surface(
    op: &Op,
    project: &Project,
    setup: &Setup,
) -> Result<(PlanarRaster, usize), PipelineError> {
    let OpKind::PlanarSurface { surface } = &op.kind else {
        unreachable!()
    };
    if op.id == 0 || project.operations.iter().filter(|o| o.id == op.id).count() != 1 {
        return Err(invalid(
            op,
            "Surfacing requires a unique positive operation id.",
        ));
    }
    let tool = project
        .tools
        .iter()
        .find(|t| t.id == op.tool_id)
        .ok_or(PipelineError::UnknownTool(op.id, op.tool_id))?;
    if !tool.diameter.is_finite()
        || !tool.wear_offset_mm.is_finite()
        || tool.diameter <= 0.0
        || tool.diameter - tool.wear_offset_mm <= 0.0
    {
        return Err(invalid(
            op,
            "Surfacing requires a positive physical/effective cutter diameter.",
        ));
    }
    if !matches!(tool.kind, ToolKind::Endmill | ToolKind::Compression)
        || tool.whirl
        || setup.machine.mode != MachineMode::Mill
        || setup.machine.plot_mode_z
    {
        return Err(invalid(
            op,
            "Planar surfacing requires milling mode and a flat endmill without a Whirl overlay.",
        ));
    }
    if op.side != WorkpieceSide::Front
        || project
            .stock
            .as_ref()
            .is_some_and(|s| s.flip.is_some() || s.top_z_mm != 0.0)
        || project.work_offset.z_mm != 0.0
    {
        return Err(invalid(
            op,
            "Planar surfacing V1 requires a single-sided job with Z=0 on the material surface.",
        ));
    }
    let raster = planar_raster(surface, setup.tool.diameter).map_err(|m| invalid(op, m))?;
    if surface.stepover_mm > setup.tool.diameter * 0.95 {
        return Err(invalid(op, "Surfacing stepover must not exceed 95% of the effective cutter diameter (retain overlap between raster passes)."));
    }
    let m = &setup.mill;
    if ![m.depth, m.start_depth, m.step, m.fast_move_z]
        .iter()
        .all(|v| v.is_finite())
        || m.start_depth != 0.0
        || m.depth >= 0.0
        || m.step >= 0.0
        || m.fast_move_z <= 0.0
        || !m.depth_list.is_empty()
        || m.through_depth != 0.0
    {
        return Err(invalid(op, "Use negative depth and stepdown, start depth 0, positive safe Z, and no explicit depth list or through-cut extension."));
    }
    if setup.tool.rate_h == 0 || setup.tool.rate_v == 0 || setup.tool.speed == 0 {
        return Err(invalid(
            op,
            "Set positive surfacing feed, plunge and spindle rates.",
        ));
    }
    if !matches!(op.params.plunge, crate::project::PlungeStrategy::Direct)
        || op.finish_tool_id.is_some()
    {
        return Err(invalid(op, "Surfacing V1 uses direct entry outside the scan edges, one flat cutter and no patterns."));
    }
    if project.stock.as_ref().is_some_and(|s| {
        !s.thickness_mm.is_finite() || s.thickness_mm <= 0.0 || -m.depth >= s.thickness_mm
    }) {
        return Err(invalid(
            op,
            "Surfacing removal must be shallower than the declared material thickness.",
        ));
    }
    let passes = (m.depth / m.step).ceil();
    if !passes.is_finite()
        || passes > crate::cam::MAX_DEPTH_PASSES as f64
        || passes * raster.lines.len() as f64 > MAX_SURFACE_LINES as f64
    {
        return Err(invalid(
            op,
            "Surfacing exceeds the 1000-depth-pass or 100000-total-scanline limit.",
        ));
    }
    if tool
        .flute_length_mm
        .is_some_and(|f| !f.is_finite() || f <= 0.0 || -m.depth > f)
    {
        return Err(invalid(
            op,
            "Surfacing depth exceeds the declared flute length.",
        ));
    }
    let wa = setup.machine.work_area;
    if [wa.x, wa.y, wa.z].iter().all(|v| v.is_finite() && *v > 0.0) {
        let [xmin, ymin, xmax, ymax] = raster.center_bounds;
        if xmin < 0.0
            || ymin < 0.0
            || xmax > wa.x
            || ymax > wa.y
            || -m.depth > wa.z
            || m.fast_move_z > wa.z
        {
            return Err(invalid(op, "The surfacing center travel (including radius and overtravel), depth or safe Z exceeds the configured work area. Adjust the zone or work origin."));
        }
    } else {
        return Err(invalid(
            op,
            "Set a positive XYZ machine work area before surfacing.",
        ));
    }
    Ok((raster, passes as usize))
}

pub(in crate::pipeline) fn run_planar_surface_op<P: PostProcessor>(
    op: &Op,
    project: &Project,
    setup: &Setup,
    post: &mut P,
    last_pos: &mut Point2,
    warnings: &mut Vec<PipelineWarning>,
    cancel: Option<&CancelToken>,
) -> Result<(), PipelineError> {
    let (raster, passes) = validate_planar_surface(op, project, setup)?;
    warnings.push(PipelineWarning::for_op(op.id, "surfacing_edge_overtravel",
        "Surfacing deliberately travels outside its target rectangle. Confirm clear edges, clamps and Z=0 on the material surface; cutter/fixture checks do not prove the full machine is clear."));
    for i in 1..=passes {
        if cancel.is_some_and(CancelToken::is_cancelled) {
            return Err(PipelineError::Cancelled);
        }
        let z = if i == passes {
            setup.mill.depth
        } else {
            (setup.mill.step * i as f64).max(setup.mill.depth)
        };
        let polylines: Vec<_> = raster
            .lines
            .iter()
            .map(|[a, b]| vec![(a[0], a[1], z), (b[0], b[1], z)])
            .collect();
        let mut at_depth = setup.clone();
        // The shared emitter now plunges at the saved plunge feed all the way
        // to this pass depth, then switches to the saved XY cutting feed.
        at_depth.mill.start_depth = z;
        emit_vcarve_block(&at_depth, &polylines, post, last_pos);
    }
    Ok(())
}
