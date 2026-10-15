//! Qualified V-Carve driver: source regions -> target depth envelopes ->
//! V-Carve-only passes/entries -> normal generic XYZ emission.
use crate::cam::setup::Setup;
use crate::cam::source_combine::combine_source_regions;
use crate::cam::{vcarve_plan, vcarve_validate, VcObject};
use crate::gcode::{emit_vcarve_block, PostProcessor};
use crate::geometry::Point2;
use crate::pipeline::{
    cancelled, effective_step, ordered_selection, source_combine_mode, CancelToken, PipelineError,
    PipelineWarning,
};
use crate::project::{MachineMode, Op, Project, ToolKind};

pub(in crate::pipeline) fn vcarve_would_emit(op: &Op, objects: &[VcObject]) -> bool {
    !combine_source_regions(
        objects,
        &ordered_selection(op, objects),
        source_combine_mode(op),
    )
    .is_empty()
}
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(in crate::pipeline) fn run_vcarve_op<P: PostProcessor>(
    op: &Op,
    project: &Project,
    objects: &[VcObject],
    setup: &Setup,
    post: &mut P,
    last_pos: &mut Point2,
    warnings: &mut Vec<PipelineWarning>,
    cancel: Option<&CancelToken>,
) -> Result<(), PipelineError> {
    crate::profile_scope!("vcarve.driver");
    let invalid = |s: &str| PipelineError::VCarve(op.id, s.into());
    let tool = project
        .tools
        .iter()
        .find(|t| t.id == op.tool_id)
        .ok_or(PipelineError::UnknownTool(op.id, op.tool_id))?;
    let diameter = tool.effective_diameter();
    let tip = tool.tip_diameter.unwrap_or(0.);
    if tool.kind != ToolKind::VBit || !matches!(project.machine.mode, MachineMode::Mill) {
        return Err(invalid("Select a V-bit in milling mode."));
    }
    if !diameter.is_finite()
        || diameter <= 0.
        || !tip.is_finite()
        || tip < 0.
        || tip >= diameter
        || !tool.tip_angle_deg.is_finite()
        || !(1. ..=179.).contains(&tool.tip_angle_deg)
    {
        return Err(invalid("Invalid V-bit geometry: finite positive diameter, tip diameter from zero to less than the cutting diameter, and included angle 1–179° required."));
    }
    if !op.params.depth.is_finite()
        || op.params.depth > 0.
        || !op.params.start_depth.is_finite()
        || op.params.start_depth != 0.
        || !setup.mill.fast_move_z.is_finite()
        || setup.mill.fast_move_z <= 0.
    {
        return Err(invalid("V-Carve requires stock-top start depth 0, a nonpositive maximum depth and positive clearance. Staged starts are not qualified."));
    }
    if setup.tool.rate_h == 0 || setup.tool.rate_v == 0 || setup.tool.speed == 0 {
        return Err(invalid("Set positive feed, plunge and spindle rates."));
    }
    if op.params.through_depth != 0.
        || !op.params.depth_list.is_empty()
        || op.params.finish_step.is_some()
    {
        return Err(invalid("V-Carve uses maximum depth and step per pass. Clear Through depth, Depth list and Finish step; these alternate schedules are not qualified."));
    }
    let dpp = effective_step(op, tool).map(f64::abs).unwrap_or(1.);
    if !dpp.is_finite() || dpp <= 0. {
        return Err(invalid("Set a finite positive depth per pass."));
    }
    let angle = tool.vcarve_lead_in_angle_deg.unwrap_or(10.);
    if !angle.is_finite() || angle <= 0. || angle >= 90. {
        return Err(invalid(
            "V-Carve entry angle must be between 0 and 90 degrees.",
        ));
    }
    let params = op.vcarve_params().expect("VCarve driver");
    if params.multi_pass_refine {
        return Err(invalid("The legacy refine-pass option is not implemented. Disable it; normal depth passes already reach the qualified target."));
    }
    if params
        .carve_max_width_mm
        .is_some_and(|c| !c.is_finite() || c <= 0.)
        || params
            .source_inset_mm
            .is_some_and(|c| !c.is_finite() || c < 0.)
    {
        return Err(invalid("Radius cap and source inset must be finite and nonnegative (radius cap strictly positive)."));
    }
    let tan = (tool.tip_angle_deg.to_radians() * 0.5).tan();
    let tip_r = tip * 0.5;
    let reach = diameter * 0.5;
    let r_cap = params.carve_max_width_mm.unwrap_or(reach).min(reach);
    let max_depth = if op.params.depth < 0. {
        -op.params.depth
    } else {
        f64::INFINITY
    };
    let z_limit = ((r_cap - tip_r) / tan).min(max_depth);
    if z_limit <= 0. {
        return Err(invalid(
            "V-Carve radius cap is at or below the flat tip: no material can be removed.",
        ));
    }
    if tool
        .flute_length_mm
        .is_some_and(|f| !f.is_finite() || f <= 0. || z_limit > f)
    {
        return Err(invalid("Requested carving depth exceeds the authored cutting length; reduce maximum depth or select another bit."));
    }
    let selected = ordered_selection(op, objects);
    let prepared = vcarve_validate::prepared(objects, &selected).map_err(|e| invalid(&e))?;
    let regions = crate::profile_measure!(
        "vcarve.regions",
        combine_source_regions(&prepared, &selected, source_combine_mode(op))
    );
    if regions.is_empty() {
        return Err(invalid(
            "No closed carving region remains after geometry combination.",
        ));
    }
    let mut polylines = Vec::new();
    let mut capped = false;
    let mut total = 0;
    for region in &regions {
        if cancelled(cancel) {
            return Err(PipelineError::Cancelled);
        }
        let inset = params.source_inset_mm.unwrap_or(0.);
        let storage;
        let boundary = if inset > 0. {
            let rings =
                crate::cam::offsets::boundary_offset_inward(&region.boundary, &region.holes, inset);
            if rings.len() != 1 || !region.holes.is_empty() {
                return Err(invalid("Source inset collapses or splits the region, or contains islands; this inlay topology is not qualified."));
            }
            storage = rings[0].clone();
            storage.as_slice()
        } else {
            region.boundary.as_slice()
        };
        let holes = &region.holes;
        let mut targets = Vec::new();
        if params.full_medial_axis {
            let vc = crate::cam::vcarve::VcRegion {
                outer: boundary.to_vec(),
                holes: holes.clone(),
            };
            // Keep the existing axis builder, but do not silently prune reachable
            // small features. Every retained chain must complete or fail the op.
            let axes = crate::cam::geometry_cache::medial_axis_cached(&vc, cancel);
            if cancelled(cancel) {
                return Err(PipelineError::Cancelled);
            }
            if axes.is_empty() {
                return Err(invalid("No medial axis could be resolved at the 0.1 mm sampling spacing. Geometry is unsupported; no output generated."));
            }
            for axis in axes.iter() {
                let (path, limited, zero) = crate::profile_measure!(
                    "vcarve.target_depth",
                    crate::cam::vcarve::polyline_to_z(
                        axis,
                        tool.tip_angle_deg.to_radians(),
                        tip_r,
                        Some(r_cap),
                        Some(-z_limit),
                    )
                );
                capped |= limited;
                if zero {
                    return Err(invalid("Undercut: a medial-axis chain is narrower than the flat tip and cannot be carved with this bit."));
                }
                let mut target = Vec::new();
                for (x, y, z, _) in path {
                    if target
                        .last()
                        .is_some_and(|&(px, py, _): &(f64, f64, f64)| (x - px).hypot(y - py) < 1e-7)
                    {
                        continue;
                    }
                    target.push((x, y, z));
                }
                if target.len() < 2 {
                    return Err(invalid(
                        "Undercut: a degenerate medial chain cannot be completed.",
                    ));
                }
                targets.push(target);
            }
        } else {
            // The XY offset must correspond to the actual depth after caps.
            let radius = tip_r + z_limit * tan;
            capped = z_limit < (r_cap - tip_r) / tan;
            let rings = crate::cam::offsets::boundary_offset_inward(boundary, holes, radius);
            if rings.is_empty() {
                return Err(invalid("No perimeter fits the selected radius/depth. Reduce the radius cap or choose Full medial axis."));
            }
            for ring in rings {
                let mut target: Vec<_> = ring.iter().map(|p| (p.x, p.y, -z_limit)).collect();
                target.dedup_by(|a, b| (a.0 - b.0).hypot(a.1 - b.1) < 1e-7);
                if target.len() < 3 {
                    return Err(invalid("Degenerate perimeter after V-bit offset."));
                }
                if (target[0].0 - target.last().unwrap().0)
                    .hypot(target[0].1 - target.last().unwrap().1)
                    > 1e-7
                {
                    target.push(target[0]);
                }
                targets.push(target);
            }
        }
        let mut rings = vec![boundary];
        rings.extend(holes.iter().map(Vec::as_slice));
        for target in targets {
            crate::profile_count!("vcarve.target_points", target.len());
            vcarve_validate::check_envelope(&target, &rings, tip_r, tan)
                .map_err(|e| invalid(&e))?;
            let path = vcarve_plan::plan(&target, dpp, angle).map_err(|e| invalid(&e))?;
            crate::profile_count!("vcarve.planned_points", path.len());
            total += path.len();
            if total > 1_000_000 {
                return Err(invalid("Undercut: operation exceeds the one-million-waypoint budget before completion."));
            }
            polylines.push(path);
        }
    }
    if polylines.is_empty() {
        return Err(invalid("Undercut: no qualified cutting path."));
    }
    if capped {
        warnings.push(PipelineWarning::for_op(op.id,"vcarve_target_capped","V-Carve reaches the explicitly capped target; capped regions are not fully cleared. Reduce radius limits only if that is your intended geometry."));
    }
    warnings.push(PipelineWarning::for_op(op.id,"vcarve_scope",if params.full_medial_axis {"Full medial axis: completed sampled spine paths (0.1 mm boundary spacing), not a complete-area pocket or exact swept-volume simulation."} else {"Perimeter: V-bit contour groove at constant final depth; the interior is not cleared."}));
    // Shared emitter is unchanged: the plan begins at Z=0 and contains its own
    // qualified continuous entry. Between separate chains it retracts to safe Z.
    crate::profile_measure!(
        "vcarve.emit",
        emit_vcarve_block(setup, &polylines, post, last_pos)
    );
    Ok(())
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::geometry::{Point2, Segment};
    use crate::pipeline::test_helpers::vbit;
    use crate::pipeline::{run_pipeline, PipelineRequest};
    use crate::project::MachineConfig;
    use crate::project::{Op, OpKind, OpParams, OpSource, Project};

    /// `VCarve` op produces a non-empty toolpath whose deepest cutting
    /// move sits well below `start_depth - 0.1` — proves the medial
    /// axis ratchet actually plunges into the slot rather than just
    /// tracing the boundary at z=0.
    #[test]
    fn vcarve_op_emits_cutting_moves_below_start_depth() {
        let op = Op {
            id: 7,
            name: "Carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams::default(),
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -10.0,
                start_depth: 0.0,
                step: Some(-1.0),
                fast_move_z: 5.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        let project = Project {
            segments: vec![
                Segment::line(Point2::new(0.0, 0.0), Point2::new(20.0, 0.0), "0", 7),
                Segment::line(
                    Point2::new(20.0, 0.0),
                    Point2::new(10.0, 17.320_508),
                    "0",
                    7,
                ),
                Segment::line(Point2::new(10.0, 17.320_508), Point2::new(0.0, 0.0), "0", 7),
            ],
            machine: MachineConfig::default(),
            tools: vec![vbit()],
            operations: vec![op],
            fixtures: Vec::default(),
            text_layers: Vec::default(),
            work_offset: crate::project::WorkOffset::default(),
            stock: None,
            relief_sources: Vec::new(),
            group_ops_by_tool: false,
            material: None,
            fs_context: None,
        };
        let resp = run_pipeline(
            PipelineRequest {
                project,
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .expect("pipeline ran");
        assert!(!resp.gcode.is_empty(), "gcode should not be empty");
        let any_deep = resp
            .toolpath
            .iter()
            .any(|s| s.to.z < -0.1 && !matches!(s.kind, crate::gcode::preview::MoveKind::Rapid));
        assert!(
            any_deep,
            "expected at least one cutting move below start_depth - 0.1; got {} toolpath segs",
            resp.toolpath.len()
        );
    }

    /// User report: V-Carve op pointed at an open polyline (e.g. a
    /// single-line text layer) silently produced no toolpath because
    /// `combine_source_regions` returns empty. Now warns instead.
    #[test]
    fn vcarve_op_rejects_no_closed_region() {
        let op = Op {
            id: 7,
            name: "Carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams::default(),
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -3.0,
                start_depth: 0.0,
                step: Some(-1.0),
                fast_move_z: 5.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        // A single LINE segment doesn't form a closed contour. No
        // region → expect the warning.
        let project = Project {
            segments: vec![Segment::line(
                Point2::new(0.0, 0.0),
                Point2::new(50.0, 0.0),
                "0",
                7,
            )],
            machine: MachineConfig::default(),
            tools: vec![vbit()],
            operations: vec![op],
            fixtures: Vec::default(),
            text_layers: Vec::default(),
            work_offset: crate::project::WorkOffset::default(),
            stock: None,
            relief_sources: Vec::new(),
            group_ops_by_tool: false,
            material: None,
            fs_context: None,
        };
        let resp = run_pipeline(
            PipelineRequest {
                project,
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .expect_err("unsupported carve must fail, not return successful empty output");
        assert!(resp.to_string().contains("closed"), "{resp}");
    }

    /// Tool-reach clamp: a 6mm V-bit physically can't
    /// engage past r = 3mm. For a 30x30 square (incircle radius 15mm)
    /// the medial axis hits r = 15 — without the clamp the depth math
    /// would dive to z = -15 / tan(30°) ≈ -26mm regardless of the bit's
    /// 3mm reach. The clamp keeps z above ≈ -5.2mm (3 / tan(30°)).
    /// Exercises `full_medial_axis` = true so the medial-axis path runs.
    #[test]
    fn vcarve_op_respects_tool_reach() {
        let op = Op {
            id: 7,
            name: "Carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams {
                    full_medial_axis: true,
                    ..Default::default()
                },
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -50.0, // very deep so the tool-reach cap is the limiting factor
                start_depth: 0.0,
                step: Some(-1.0),
                fast_move_z: 5.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        // 30x30 closed square — incircle radius 15mm.
        let project = Project {
            segments: vec![
                Segment::line(Point2::new(0.0, 0.0), Point2::new(30.0, 0.0), "0", 7),
                Segment::line(Point2::new(30.0, 0.0), Point2::new(30.0, 30.0), "0", 7),
                Segment::line(Point2::new(30.0, 30.0), Point2::new(0.0, 30.0), "0", 7),
                Segment::line(Point2::new(0.0, 30.0), Point2::new(0.0, 0.0), "0", 7),
            ],
            machine: MachineConfig::default(),
            tools: vec![vbit()],
            operations: vec![op],
            fixtures: Vec::default(),
            text_layers: Vec::default(),
            work_offset: crate::project::WorkOffset::default(),
            stock: None,
            relief_sources: Vec::new(),
            group_ops_by_tool: false,
            material: None,
            fs_context: None,
        };
        let resp = run_pipeline(
            PipelineRequest {
                project,
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .expect("pipeline ran");
        let z_min = resp.toolpath.iter().map(|s| s.to.z).fold(0.0_f64, f64::min);
        // vbit() default is diameter 6.35mm, tip 60° → tool_reach_r = 3.175,
        // tan(30°) ≈ 0.5774, z_min_expected ≈ -5.50mm. The cone-floor
        // depth could only go that deep with the clamp; without it, the
        // medial-axis radius of ~15mm produces z ≈ -26mm.
        assert!(
            z_min > -10.0,
            "with tool-reach clamp, z_min should be > -10mm; got {z_min}",
        );
        assert!(
            resp.warnings
                .iter()
                .any(|w| w.kind == "vcarve_target_capped"),
            "tool-reach cap should mark depth_limited",
        );
    }

    /// Default V-Carve (`full_medial_axis` = false) traces ONLY a
    /// perimeter loop on a 30×30 square — no spine cuts through the
    /// interior. Compare with `vcarve_op_respects_tool_reach` above
    /// which exercises the medial-axis branch on the same geometry.
    #[test]
    fn vcarve_op_default_is_perimeter_only() {
        let op = Op {
            id: 7,
            name: "Carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams::default(),
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -10.0,
                start_depth: 0.0,
                step: Some(-1.0),
                fast_move_z: 5.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        // 30x30 closed square — incircle radius 15mm. Boundary inset by
        // R = 3.175 mm (vbit tool reach) → expect a ~23.65×23.65 square
        // loop centred on (15, 15).
        let project = Project {
            segments: vec![
                Segment::line(Point2::new(0.0, 0.0), Point2::new(30.0, 0.0), "0", 7),
                Segment::line(Point2::new(30.0, 0.0), Point2::new(30.0, 30.0), "0", 7),
                Segment::line(Point2::new(30.0, 30.0), Point2::new(0.0, 30.0), "0", 7),
                Segment::line(Point2::new(0.0, 30.0), Point2::new(0.0, 0.0), "0", 7),
            ],
            machine: MachineConfig::default(),
            tools: vec![vbit()],
            operations: vec![op],
            fixtures: Vec::default(),
            text_layers: Vec::default(),
            work_offset: crate::project::WorkOffset::default(),
            stock: None,
            relief_sources: Vec::new(),
            group_ops_by_tool: false,
            material: None,
            fs_context: None,
        };
        let resp = run_pipeline(
            PipelineRequest {
                project,
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .expect("pipeline ran");
        // All cut points should be in a thin annular band near the
        // boundary — no point should be deep inside the square (which
        // is where the medial-axis spine would land). The inset is
        // ≤ 5 mm so points beyond (5, 25) on both axes signal a spine
        // cut that shouldn't exist in perimeter mode.
        let cut_pts: Vec<_> = resp
            .toolpath
            .iter()
            .filter(|s| !matches!(s.kind, crate::gcode::preview::MoveKind::Rapid))
            .map(|s| (s.to.x, s.to.y, s.to.z))
            .collect();
        assert!(!cut_pts.is_empty(), "perimeter pass should emit cut moves");
        for (x, y, _z) in &cut_pts {
            // Distance from boundary <= ~5 mm tolerance. The boundary
            // is the [0..30] square; the closest edge gives the
            // distance.
            let d = x.min(30.0 - x).min(*y).min(30.0 - y);
            assert!(
                d < 5.0,
                "perimeter mode emitted a cut at ({x:.2}, {y:.2}) which is {d:.2} mm from any edge — looks like a spine cut",
            );
        }
        // The perimeter should sit at the expected z depth (≈ -5.5 mm).
        let z_min = cut_pts.iter().map(|(_, _, z)| *z).fold(0.0_f64, f64::min);
        assert!(
            z_min < -3.0 && z_min > -8.0,
            "expected perimeter depth around -5.5 mm; got z_min = {z_min}",
        );
    }

    /// A Plug-side V-Carve with `source_inset_mm = 1.0` emits a
    /// perimeter loop further from the original boundary than the
    /// Pocket-side default. We pair them on the same 30×30 square and
    /// verify the plug's outer extent is ≈ 1 mm narrower than the
    /// pocket's — that's the geometric clearance an inlay needs.
    #[test]
    fn vcarve_inlay_plug_offsets_by_source_inset() {
        let make_op = |source_inset: Option<f64>| Op {
            id: 7,
            name: "Carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams {
                    source_inset_mm: source_inset,
                    ..Default::default()
                },
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -3.0,
                start_depth: 0.0,
                step: Some(-1.0),
                fast_move_z: 5.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        let square = vec![
            Segment::line(Point2::new(0.0, 0.0), Point2::new(30.0, 0.0), "0", 7),
            Segment::line(Point2::new(30.0, 0.0), Point2::new(30.0, 30.0), "0", 7),
            Segment::line(Point2::new(30.0, 30.0), Point2::new(0.0, 30.0), "0", 7),
            Segment::line(Point2::new(0.0, 30.0), Point2::new(0.0, 0.0), "0", 7),
        ];
        let extent_x = |resp: &crate::pipeline::PipelineResponse| -> (f64, f64) {
            let xs: Vec<f64> = resp
                .toolpath
                .iter()
                .filter(|s| !matches!(s.kind, crate::gcode::preview::MoveKind::Rapid))
                .map(|s| s.to.x)
                .collect();
            (
                xs.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                xs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            )
        };
        let run = |op: Op| -> crate::pipeline::PipelineResponse {
            let project = Project {
                segments: square.clone(),
                machine: MachineConfig::default(),
                tools: vec![vbit()],
                operations: vec![op],
                fixtures: Vec::default(),
                text_layers: Vec::default(),
                work_offset: crate::project::WorkOffset::default(),
                stock: None,
                relief_sources: Vec::new(),
                group_ops_by_tool: false,
                material: None,
                fs_context: None,
            };
            run_pipeline(
                PipelineRequest {
                    project,
                    post_processor: None,
                    cps_post: None,
                },
                |_, _, _| {},
            )
            .expect("pipeline ran")
        };
        let pocket = run(make_op(None));
        let plug = run(make_op(Some(1.0)));
        let (pmin, pmax) = extent_x(&pocket);
        let (qmin, qmax) = extent_x(&plug);
        // Plug perimeter is offset inward by 1mm MORE than pocket.
        assert!(
            (qmin - pmin - 1.0).abs() < 0.2,
            "plug x_min {qmin:.3} should be ~1mm inside pocket x_min {pmin:.3}",
        );
        assert!(
            (pmax - qmax - 1.0).abs() < 0.2,
            "plug x_max {qmax:.3} should be ~1mm inside pocket x_max {pmax:.3}",
        );
    }

    /// Full-medial-axis V-Carve where the effective r-cap drops
    /// at or below the V-bit's flat tip must not silently emit a Z=0
    /// toolpath (the cutter would walk the medial axis without engaging,
    /// removing nothing — looking like a successful cut). Verify a
    /// `vcarve_below_tip_radius` warning fires and NO cutting moves
    /// are emitted.
    #[test]
    fn vcarve_full_medial_axis_rejects_below_tip_radius() {
        // Custom V-bit with a deliberately fat flat tip — tip_diameter
        // 1.0 mm ⇒ tip_radius 0.5 mm.
        let mut fat_tip = crate::pipeline::test_helpers::vbit();
        fat_tip.tip_diameter = Some(1.0);
        let op = Op {
            id: 7,
            name: "Carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams {
                    // r-cap 0.4 < tip_radius 0.5 → every chain's Z
                    // collapses to 0.
                    carve_max_width_mm: Some(0.4),
                    full_medial_axis: true,
                    ..Default::default()
                },
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -3.0,
                start_depth: 0.0,
                step: Some(-1.0),
                fast_move_z: 5.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        let project = Project {
            segments: vec![
                Segment::line(Point2::new(0.0, 0.0), Point2::new(30.0, 0.0), "0", 7),
                Segment::line(Point2::new(30.0, 0.0), Point2::new(30.0, 30.0), "0", 7),
                Segment::line(Point2::new(30.0, 30.0), Point2::new(0.0, 30.0), "0", 7),
                Segment::line(Point2::new(0.0, 30.0), Point2::new(0.0, 0.0), "0", 7),
            ],
            machine: MachineConfig::default(),
            tools: vec![fat_tip],
            operations: vec![op],
            fixtures: Vec::default(),
            text_layers: Vec::default(),
            work_offset: crate::project::WorkOffset::default(),
            stock: None,
            relief_sources: Vec::new(),
            group_ops_by_tool: false,
            material: None,
            fs_context: None,
        };
        let resp = run_pipeline(
            PipelineRequest {
                project,
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .expect_err("unsupported carve must fail, not return successful empty output");
        assert!(resp.to_string().contains("flat tip"), "{resp}");
    }

    #[test]
    fn vcarve_op_round_trips_through_serde_json() {
        let op = Op {
            id: 11,
            name: "Sign carve".into(),
            enabled: true,
            kind: OpKind::VCarve {
                carve: crate::project::VCarveParams {
                    carve_max_width_mm: Some(4.0),
                    multi_pass_refine: true,
                    full_medial_axis: false,
                    source_inset_mm: None,
                },
            },
            tool_id: 1,
            finish_tool_id: None,
            source: OpSource::All,
            params: OpParams {
                depth: -8.0,
                start_depth: 0.0,
                step: Some(-0.8),
                fast_move_z: 6.0,
                ..OpParams::default()
            },
            group: None,
            pin_order: false,
            side: crate::project::WorkpieceSide::Front,
        };
        let json = serde_json::to_string(&op).expect("serialize");
        let back: Op = serde_json::from_str(&json).expect("deserialize");
        let OpKind::VCarve { carve } = &back.kind else {
            panic!("expected VCarve kind, got {:?}", back.kind);
        };
        assert_eq!(carve.carve_max_width_mm, Some(4.0));
        assert!(carve.multi_pass_refine);
        assert_eq!(back.params.depth, -8.0);
    }
}
