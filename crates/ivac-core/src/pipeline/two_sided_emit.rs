//! Two-sided (flip-stock) two-program emission (rt1.11.3.1, Phase 2b).
//!
//! A two-sided job cuts the front face, the operator flips the stock about a
//! single axis, then cuts the back face. This orchestrator turns ONE project
//! into TWO programs without touching the single-program pipeline: it splits
//! the op list by [`WorkpieceSide`], runs the existing [`run_pipeline`] once
//! per side, and composes the results.
//!
//! ## Coordinate model (see bd rt1.11 part B + the guard convention)
//!
//! * **XY** — back-side geometry is mirrored about the stock centre-line for
//!   the flip axis ([`crate::cam::flip::flip_segments_xy`]) so that, once the
//!   operator physically turns the stock over, the mirrored toolpath lands on
//!   the correct back-side features.
//! * **Z** — a `Back` op's depth schedule is authored *own-face-relative*
//!   (0 = the back face, negative = into the stock — the same numeric form a
//!   `Front` op uses for the top face; this is exactly what the conflict
//!   guard's `op_removal_mm` assumes). The back program therefore emits those
//!   depths *verbatim*, and the header instructs the operator to **re-zero Z
//!   to the new top (back) face**. Real stock thickness varies ±0.5 mm, so a
//!   physical re-zero — not a computed datum shift — is the reliable anchor.
//!   `flip_z` is deliberately NOT applied to emitted cut depths: with the
//!   physical re-zero it would double-transform. (It remains the sim/preview
//!   frame relationship, wired in the Phase-3 dual-surface preview.)
//!
//! ## Dowel registration
//!
//! The front program drills auto-placed dowel holes on the flip-axis
//! centre-line (the mirror-invariant line, so the same holes register the
//! stock in both setups); the back program references those positions in its
//! header rather than re-drilling them. Placement is stock-relative
//! ([`DowelPinConfig`]: `count`, `margin_mm`), spread toward the two ends and
//! inset from the edges so they sit outside a centred part.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{
    run_pipeline, warnings, PipelineError, PipelineRequest, PipelineResponse, PostProcessorKind,
};
use crate::cam::chaining::{classify_containment, segments_to_objects};
use crate::cam::flip::{flip_point_xy, flip_segments_xy};
use crate::geometry::{Point2, Segment, SegmentKind};
use crate::project::{
    DowelPinConfig, DrillCycle, FlipAxis, Op, OpKind, OpSource, Project, SourceCombine,
    StockConfig, ToolKind, WorkpieceSide,
};

/// Synthetic layer the injected dowel-drill op targets. Underscore-prefixed
/// like the text-layer render pool (`__text_<id>`) so it can't collide with an
/// imported layer name.
const DOWEL_LAYER: &str = "__dowels";

/// Result of a two-sided emission: the front program plus, for a genuine
/// two-sided job, the back program. `back` is `None` for a single-sided
/// project — the orchestrator then returns the front field unchanged from the
/// ordinary single-program path, so single-sided callers see byte-identical
/// output.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct TwoSidedResponse {
    /// The front program (and, in a two-sided job, the one that drills the
    /// dowel registration holes).
    pub front: PipelineResponse,
    /// The back program — present only when the job is two-sided (stock flip
    /// registration set and at least one enabled `Back` op). Its geometry is
    /// mirrored and its header carries the flip + Z-re-zero instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back: Option<PipelineResponse>,
}

/// Whether `project` is a genuine two-sided job: it declares a stock flip
/// registration AND has at least one enabled `Back` op. A project with `Back`
/// ops but no flip registration (or vice-versa) is treated as single-sided —
/// the orchestrator can't mirror without an axis, and the front-only path
/// reproduces today's behaviour.
fn two_sided_registration(project: &Project) -> Option<(&StockConfig, FlipAxis)> {
    let stock = project.stock.as_ref()?;
    let flip = stock.flip.as_ref()?;
    let has_back = project
        .operations
        .iter()
        .any(|o| o.enabled && o.side == WorkpieceSide::Back);
    has_back.then_some((stock, flip.axis))
}

/// Run the two-sided pipeline: one program per stock face.
///
/// For a single-sided project (no flip registration, or no enabled `Back` op)
/// this is exactly [`run_pipeline`] wrapped as `TwoSidedResponse { front, back:
/// None }` — output is byte-identical to the single-program path. HPGL (a
/// pen-plotter dialect with no Z / milling) is always treated as single-sided.
///
/// For a two-sided project it:
/// 1. runs the conflict guard on the WHOLE project (opposing-overlap detection
///    needs both sides' extents, so it must precede the per-side split);
/// 2. emits the front program from the `Front` ops plus an injected dowel-drill
///    op;
/// 3. emits the back program from the `Back` ops with their XY mirrored about
///    the stock centre-line;
/// 4. prepends flip / re-zero / dowel instructions to each program header.
///
/// # Errors
///
/// Propagates any [`PipelineError`] from either per-side run, and
/// [`PipelineError::TwoSidedThrough`] from the whole-project conflict guard
/// when a front op would sever the stock before the flip.
pub fn run_pipeline_two_sided<F: Fn(&str, f64, &str)>(
    req: PipelineRequest,
    progress: F,
) -> Result<TwoSidedResponse, PipelineError> {
    let post_kind = req.post_processor.unwrap_or_default();

    // HPGL or non-two-sided → the ordinary single program, unchanged.
    let Some((stock, axis)) = two_sided_registration(&req.project) else {
        return Ok(TwoSidedResponse {
            front: run_pipeline(req, &progress)?,
            back: None,
        });
    };
    if post_kind == PostProcessorKind::Hpgl {
        return Ok(TwoSidedResponse {
            front: run_pipeline(req, &progress)?,
            back: None,
        });
    }

    // Existing two-sided headers use native G-code comments and unshifted
    // registration references. Do not present those as qualified CPS/translated
    // machine output. Keep authored flip/origin intent; fail before any output.
    if post_kind == PostProcessorKind::Cps
        || req.project.work_offset.x_mm != 0.
        || req.project.work_offset.y_mm != 0.
    {
        return Err(PipelineError::WorkOrigin("two-sided CPS output or translated XY registration is not qualified; use a single-sided job or an unshifted native setup".into()));
    }
    // These drivers read independent origins/heightfields, not mirrored segments.
    // Refuse until their back-face transform has a qualified adapter.
    if req.project.operations.iter().any(|op| {
        op.enabled
            && op.side == WorkpieceSide::Back
            && matches!(
                op.kind,
                OpKind::PlanarSurface { .. }
                    | OpKind::ReliefMill { .. }
                    | OpKind::WaterlineRough { .. }
            )
    }) {
        return Err(PipelineError::Fixture("back-face surfacing/heightfield transforms are not qualified; no two-sided programs emitted".into()));
    }
    let stock = stock.clone();
    let dowels = stock.flip.as_ref().and_then(|f| f.dowels.clone());
    if req.project.cutting_version == Some(9) && dowels.is_some() {
        return Err(PipelineError::Fixture("Automatic dowel drilling has no explicit operation-owned cutting values and is not qualified for v9. Prepare registration with explicit Drill operations and disable automatic holes.".into()));
    }
    let dowel_centres = dowels
        .as_ref()
        .map(|d| dowel_centres(&stock, axis, d))
        .transpose()?
        .unwrap_or_default();

    // (1) Correctness gate on the UNSPLIT project: opposing-overlap detection
    // compares a Front and a Back footprint, so both sides must be present.
    // A FrontSever refuses the whole job here, before either program emits.
    // The per-side sub-runs' own `two_sided_guard` then no-ops (front: no Back
    // op; back: no Front op). Runs before dowel injection so the registration
    // through-holes never count as a "front sever".
    let mut overlap_warnings = Vec::new();
    {
        let mut objects = segments_to_objects(&req.project.segments);
        classify_containment(&mut objects);
        warnings::two_sided_guard(&req.project, &objects, &mut overlap_warnings)?;
    }

    // (2) Front program: keep Front ops, prepend the dowel-drill op.
    let mut front_project = req.project.clone();
    front_project
        .operations
        .retain(|o| o.side == WorkpieceSide::Front);
    if !dowel_centres.is_empty() {
        if let Some(d) = dowels.as_ref() {
            inject_dowel_op(&mut front_project, &stock, &dowel_centres, d)?;
        }
    }
    let mut front = run_pipeline(
        PipelineRequest {
            project: front_project,
            post_processor: Some(post_kind),
            cps_post: req.cps_post.clone(),
        },
        &progress,
    )?;
    // Attach the whole-project overlap warnings to the front program (the one
    // the operator reads first).
    front.warnings.extend(overlap_warnings);
    front.gcode = prepend_lines(&front.gcode, &front_header(&dowel_centres));

    // (3) Back program: keep Back ops, mirror their XY about the stock centre.
    let mut back_project = req.project.clone();
    back_project
        .operations
        .retain(|o| o.side == WorkpieceSide::Back);
    flip_segments_xy(&mut back_project.segments, axis, &stock);
    let mut back = run_pipeline(
        PipelineRequest {
            project: back_project,
            post_processor: Some(post_kind),
            cps_post: req.cps_post.clone(),
        },
        &progress,
    )
    .map_err(|error| match error {
        PipelineError::FixtureCollision(mut preview) => {
            preview.side = Some(WorkpieceSide::Back);
            PipelineError::FixtureCollision(preview)
        }
        other => other,
    })?;
    back.gcode = prepend_lines(&back.gcode, &back_header(axis, &stock, &dowel_centres));

    Ok(TwoSidedResponse {
        front,
        back: Some(back),
    })
}

/// Auto-place dowel-hole centres on the flip-axis centre-line — the line the
/// mirror leaves invariant, so a hole drilled from the front registers the
/// same point on the back after the flip. Holes are spread evenly toward the
/// two ends of that line, inset `margin_mm` from the edges so they clear a
/// centred part.
fn dowel_centres(
    stock: &StockConfig,
    axis: FlipAxis,
    cfg: &DowelPinConfig,
) -> Result<Vec<Point2>, PipelineError> {
    let span = match axis {
        FlipAxis::X => stock.width_mm,
        FlipAxis::Y => stock.height_mm,
    };
    let across = match axis {
        FlipAxis::X => stock.height_mm,
        FlipAxis::Y => stock.width_mm,
    };
    let radius = cfg.diameter_mm * 0.5;
    if !(2..=1024).contains(&cfg.count)
        || ![
            span,
            across,
            cfg.diameter_mm,
            cfg.margin_mm,
            stock.origin[0],
            stock.origin[1],
        ]
        .iter()
        .all(|v| v.is_finite())
        || cfg.diameter_mm <= 0.
        || cfg.margin_mm < radius
        || across < cfg.diameter_mm
        || span - 2.0 * cfg.margin_mm < cfg.diameter_mm * f64::from(cfg.count - 1)
    {
        return Err(PipelineError::Fixture("dowel registration requires 2..1024 distinct nonoverlapping holes contained in stock; review count, diameter and margin".into()));
    }
    let count = cfg.count;
    let margin = cfg.margin_mm;
    let (ox, oy) = (stock.origin[0], stock.origin[1]);
    let cx = ox + stock.width_mm / 2.0;
    let cy = oy + stock.height_mm / 2.0;

    // `span` runs ALONG the flip line: X for an X-axis flip (mirror-Y line
    // y = cy), Y for a Y-axis flip (mirror-X line x = cx).
    let (lo, hi) = match axis {
        FlipAxis::X => (ox + margin, ox + stock.width_mm - margin),
        FlipAxis::Y => (oy + margin, oy + stock.height_mm - margin),
    };
    let centres: Vec<_> = (0..count)
        .map(|i| {
            let t = f64::from(i) / f64::from(count - 1);
            let along = lo + t * (hi - lo);
            match axis {
                FlipAxis::X => Point2::new(along, cy),
                FlipAxis::Y => Point2::new(cx, along),
            }
        })
        .collect();
    if centres.iter().any(|p| !p.x.is_finite() || !p.y.is_finite())
        || centres
            .windows(2)
            .any(|p| (p[1].x - p[0].x).hypot(p[1].y - p[0].y) < cfg.diameter_mm)
    {
        return Err(PipelineError::Fixture(
            "dowel centres lose finite/distinct spacing at these stock coordinates".into(),
        ));
    }
    Ok(centres)
}

/// Inject a dowel-drill op (targeting a synthetic point-geometry layer) as the
/// FIRST op of the front program, so the registration holes are drilled before
/// anything else moves. The holes go through the stock (+0.5 mm) so a pin can
/// pass through into the fixture. An unambiguous existing matching drill is
/// required; physical tip geometry and authored drilling rates are never invented.
// exact equality is the contract (parsed words, configured/identity values); a tolerance would change behaviour.
#[allow(clippy::float_cmp)]
fn inject_dowel_op(
    project: &mut Project,
    stock: &StockConfig,
    centres: &[Point2],
    cfg: &DowelPinConfig,
) -> Result<(), PipelineError> {
    let candidates: Vec<_> = project
        .tools
        .iter()
        .filter(|t| {
            t.kind == ToolKind::Drill
                && t.effective_diameter() == cfg.diameter_mm
                && t.tip_angle_deg.is_finite()
                && t.tip_angle_deg > 0.
                && t.tip_angle_deg <= 180.
                && t.speed_drill.is_some_and(|v| v > 0)
                && t.feed_rate_drill.is_some_and(|v| v > 0)
                && t.plunge_rate_drill.is_some_and(|v| v > 0)
        })
        .collect();
    if candidates.len() != 1 {
        return Err(PipelineError::Fixture("dowels require one matching declared Drill with explicit drilling RPM/feed/plunge and valid tip geometry".into()));
    }
    let tool_id = candidates[0].id;
    let op_id = project
        .operations
        .iter()
        .map(|o| o.id)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| PipelineError::Fixture("dowel operation ID exhausted".into()))?;
    // Point geometry for each hole, on the synthetic dowel layer.
    let layer: Arc<str> = Arc::from(DOWEL_LAYER);
    for c in centres {
        project.segments.push(Segment {
            kind: SegmentKind::Point,
            start: *c,
            end: *c,
            bulge: 0.0,
            center: None,
            layer: Arc::clone(&layer),
            color: 7,
        });
    }

    let mut params = crate::project::OpParams::mill_default();
    params.start_depth = 0.0;
    // Through the stock into the fixture so the pin passes fully.
    params.depth = -(stock.thickness_mm + 0.5);
    params.fast_move_z = 5.0;

    let peck = cfg.diameter_mm.max(1.0);
    let dowel_op = Op {
        id: op_id,
        name: "Dowel registration holes".to_string(),
        enabled: true,
        kind: OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: peck,
                dwell_sec: 0.0,
            },
            chamfer_after_width_mm: None,
            pattern: None,
            spot_first: None,
        },
        tool_id,
        finish_tool_id: None,
        source: OpSource::Layers {
            layers: vec![DOWEL_LAYER.to_string()],
            combine: SourceCombine::default(),
        },
        params,
        group: None,
        pin_order: true,
        side: WorkpieceSide::Front,
    };
    project.operations.insert(0, dowel_op);
    Ok(())
}

/// Front-program header comment block.
fn front_header(dowel_centres: &[Point2]) -> Vec<String> {
    let mut lines = vec![
        "===== TWO-SIDED JOB: FRONT PROGRAM =====".to_string(),
        "Run this program FIRST.".to_string(),
    ];
    if dowel_centres.is_empty() {
        lines.push("No dowel registration configured — align the flip by hand.".to_string());
    } else {
        lines.push(format!(
            "Drills {} dowel registration hole(s) — leave the pins in for the back program.",
            dowel_centres.len()
        ));
    }
    lines.push("Do NOT unclamp the stock until the back program is set up.".to_string());
    lines
}

/// Back-program header comment block: flip axis, re-zero, dowel references.
fn back_header(axis: FlipAxis, stock: &StockConfig, dowel_centres: &[Point2]) -> Vec<String> {
    let axis_name = match axis {
        FlipAxis::X => "X",
        FlipAxis::Y => "Y",
    };
    let mut lines = vec![
        "===== TWO-SIDED JOB: BACK PROGRAM =====".to_string(),
        format!("1. FLIP the stock about the {axis_name} axis."),
    ];
    if dowel_centres.is_empty() {
        lines.push("2. Re-register the stock against your fixture.".to_string());
    } else {
        // Mirror the centres for reference (identity for on-axis holes, but
        // computed explicitly to document the front↔back mapping).
        let refs: Vec<String> = dowel_centres
            .iter()
            .map(|c| {
                let m = flip_point_xy(*c, axis, stock);
                format!("({:.2}, {:.2})", m.x, m.y)
            })
            .collect();
        lines.push(format!(
            "2. Seat the stock on the dowel pins at: {}.",
            refs.join(", ")
        ));
    }
    lines.push(
        "3. RE-ZERO Z to the new top (back) face before running — stock thickness varies."
            .to_string(),
    );
    lines
}

/// Prepend `;`-comment lines to a program. LinuxCNC / GRBL both treat a
/// leading `;` line as a full-line comment anywhere in the file, so a header
/// block at the very top is safe.
fn prepend_lines(gcode: &str, lines: &[String]) -> String {
    let mut out = String::with_capacity(gcode.len() + lines.len() * 48);
    for l in lines {
        out.push_str("; ");
        out.push_str(l);
        out.push('\n');
    }
    out.push_str(gcode);
    out
}

#[cfg(test)]
// Test fixtures assert exact literal values and bounded indices.
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::pipeline::test_helpers::{endmill, pocket_op};
    use crate::pipeline::{run_pipeline, PipelineError};
    use crate::project::{FlipRegistration, Op, OpSource, Project, StockConfig, WorkpieceSide};

    /// `w × h` stock, 10 mm thick, top z = 0, X-axis flip (mirrors Y), with a
    /// 2-pin dowel registration.
    fn flip_stock(w: f64, h: f64) -> StockConfig {
        StockConfig {
            origin: [0.0, 0.0],
            width_mm: w,
            height_mm: h,
            thickness_mm: 10.0,
            top_z_mm: 0.0,
            flip: Some(FlipRegistration {
                axis: FlipAxis::X,
                dowels: Some(DowelPinConfig {
                    diameter_mm: 4.0,
                    count: 2,
                    margin_mm: 2.0,
                }),
            }),
        }
    }

    fn pocket(id: u32, side: WorkpieceSide, depth: f64) -> Op {
        let mut op = pocket_op(id, 1, OpSource::All);
        op.params.depth = depth;
        op.side = side;
        op
    }

    /// Largest Y coordinate emitted in `gcode` (naive `Y<num>` scan — enough
    /// to tell which half of the stock a program cuts in).
    fn max_y(gcode: &str) -> f64 {
        let mut m = f64::MIN;
        for tok in gcode.split_whitespace() {
            if let Some(rest) = tok.strip_prefix('Y') {
                if let Ok(v) = rest.parse::<f64>() {
                    m = m.max(v);
                }
            }
        }
        m
    }

    fn two_sided(w: f64, h: f64, ops: Vec<Op>) -> Project {
        let mut drill = endmill(2, 4.0);
        drill.kind = ToolKind::Drill;
        drill.tip_angle_deg = 118.0;
        drill.speed_drill = Some(3000);
        drill.feed_rate_drill = Some(200);
        drill.plunge_rate_drill = Some(100);
        let mut project =
            crate::pipeline::test_helpers::project_with(ops, vec![endmill(1, 3.0), drill]);
        project.stock = Some(flip_stock(w, h));
        project
            .stock
            .as_mut()
            .unwrap()
            .flip
            .as_mut()
            .unwrap()
            .dowels = None;
        project
    }

    fn run2(project: Project) -> Result<TwoSidedResponse, PipelineError> {
        run_pipeline_two_sided(
            crate::fixture_support::request(PipelineRequest {
                project,
                post_processor: Some(PostProcessorKind::Linuxcnc),
                cps_post: None,
            }),
            |_, _, _| {},
        )
    }

    /// A single-sided project (no Back op) emits exactly ONE program, and the
    /// front is byte-identical to the ordinary single-program pipeline.
    #[test]
    fn single_sided_emits_one_program_byte_identical() {
        let ops = vec![pocket(1, WorkpieceSide::Front, -3.0)];
        let mut project = crate::pipeline::test_helpers::project_with(ops, vec![endmill(1, 3.0)]);
        // Stock present but NO flip registration → single-sided.
        project.stock = Some(StockConfig {
            origin: [0.0, 0.0],
            width_mm: 20.0,
            height_mm: 20.0,
            thickness_mm: 10.0,
            top_z_mm: 0.0,
            flip: None,
        });
        let single = run_pipeline(
            crate::fixture_support::request(PipelineRequest {
                project: project.clone(),
                post_processor: Some(PostProcessorKind::Linuxcnc),
                cps_post: None,
            }),
            |_, _, _| {},
        )
        .expect("single-program run");
        let two = run2(project).expect("two-sided run on a single-sided project");
        assert!(
            two.back.is_none(),
            "single-sided job must not emit a back program"
        );
        assert_eq!(
            two.front.gcode, single.gcode,
            "single-sided front must be byte-identical to the single-program path"
        );
    }

    /// A two-sided project emits two programs, each with its own header; the
    /// back header names the flip axis and the Z re-zero.
    #[test]
    fn two_sided_emits_two_programs_with_headers() {
        let project = two_sided(
            20.0,
            40.0,
            vec![
                pocket(1, WorkpieceSide::Front, -3.0),
                pocket(2, WorkpieceSide::Back, -3.0),
            ],
        );
        let two = run2(project).expect("two-sided job emits");
        let back = two.back.expect("two-sided job has a back program");
        assert!(two.front.gcode.contains("FRONT PROGRAM"));
        assert!(back.gcode.contains("BACK PROGRAM"));
        assert!(
            back.gcode.contains("FLIP the stock about the X axis"),
            "back header must state the flip axis"
        );
        assert!(
            back.gcode.contains("RE-ZERO Z"),
            "back header must instruct the Z re-zero"
        );
    }

    /// The back program's geometry is mirrored about the stock centre-line: a
    /// feature in the bottom half of the stock (Y 0..20) machines in the top
    /// half (Y 20..40) of the back program.
    #[test]
    fn back_geometry_is_mirrored() {
        // 20×40 stock; closed_square(20) fills the bottom half (Y 0..20).
        let project = two_sided(
            20.0,
            40.0,
            vec![
                pocket(1, WorkpieceSide::Front, -3.0),
                pocket(2, WorkpieceSide::Back, -3.0),
            ],
        );
        let two = run2(project).expect("two-sided job emits");
        let back = two.back.expect("back program");
        let front_max = max_y(&two.front.gcode);
        let back_max = max_y(&back.gcode);
        assert!(
            front_max <= 20.5,
            "front cuts the bottom half only (max Y {front_max})"
        );
        assert!(
            back_max >= 35.0,
            "back is mirrored into the top half (max Y {back_max})"
        );
    }

    /// Dowel holes are drilled in the front program and only referenced in the
    /// back header (the back must not re-drill them).
    #[test]
    fn automatic_dowels_refuse_without_authored_roles() {
        let mut project = two_sided(
            20.0,
            40.0,
            vec![
                pocket(1, WorkpieceSide::Front, -3.0),
                pocket(2, WorkpieceSide::Back, -3.0),
            ],
        );
        project
            .stock
            .as_mut()
            .unwrap()
            .flip
            .as_mut()
            .unwrap()
            .dowels = flip_stock(20., 40.).flip.unwrap().dowels;
        let err = run2(project).unwrap_err();
        assert!(
            err.to_string().contains("Automatic dowel drilling"),
            "{err}"
        );
    }

    /// The conflict guard still fires on the WHOLE project before the split: a
    /// front op cutting clean through the stock refuses both programs.
    #[test]
    fn front_through_cut_still_refuses() {
        let project = two_sided(
            20.0,
            20.0,
            vec![
                pocket(1, WorkpieceSide::Front, -12.0),
                pocket(2, WorkpieceSide::Back, -3.0),
            ],
        );
        match run2(project) {
            Err(PipelineError::TwoSidedThrough { op_id, .. }) => assert_eq!(op_id, 1),
            other => panic!("expected TwoSidedThrough refuse, got {other:?}"),
        }
    }

    /// Opposing overlapping cuts warn (non-fatal) and the warning rides on the
    /// front program the operator reads first.
    #[test]
    fn opposing_overlap_warns_on_front() {
        let project = two_sided(
            20.0,
            20.0,
            vec![
                pocket(1, WorkpieceSide::Front, -6.0),
                pocket(2, WorkpieceSide::Back, -6.0),
            ],
        );
        let two = run2(project).expect("overlapping job still emits");
        assert!(
            two.front
                .warnings
                .iter()
                .any(|w| w.kind == "two_sided_overlap"),
            "the whole-project overlap warning must ride on the front program"
        );
    }
    #[test]
    fn lot7_dowels_refuse_bad_layout_or_invented_tools() {
        let stock = flip_stock(20., 40.);
        let mut cfg = stock.flip.as_ref().unwrap().dowels.clone().unwrap();
        assert_eq!(dowel_centres(&stock, FlipAxis::X, &cfg).unwrap().len(), 2);
        for count in [0, 1, 1025] {
            cfg.count = count;
            assert!(dowel_centres(&stock, FlipAxis::X, &cfg).is_err());
        }
        cfg.count = 2;
        for margin in [0., 10., 11., f64::NAN] {
            cfg.margin_mm = margin;
            assert!(dowel_centres(&stock, FlipAxis::X, &cfg).is_err());
        }
        cfg.margin_mm = 2.;
        let mut project = two_sided(
            20.,
            40.,
            vec![
                pocket(1, WorkpieceSide::Front, -3.),
                pocket(2, WorkpieceSide::Back, -3.),
            ],
        );
        project.tools.retain(|t| t.kind != ToolKind::Drill);
        let before = project.segments.len();
        assert!(inject_dowel_op(&mut project, &stock, &[Point2::new(2., 20.)], &cfg).is_err());
        assert_eq!(before, project.segments.len());
    }
    #[test]
    fn lot7_registration_keeps_real_drill_geometry_and_rates() {
        let mut p = two_sided(
            20.,
            40.,
            vec![
                pocket(1, WorkpieceSide::Front, -3.),
                pocket(2, WorkpieceSide::Back, -3.),
            ],
        );
        let mut stock = p.stock.clone().unwrap();
        stock.flip.as_mut().unwrap().dowels = flip_stock(20., 40.).flip.unwrap().dowels;
        let cfg = stock.flip.as_ref().unwrap().dowels.as_ref().unwrap();
        let centres = dowel_centres(&stock, FlipAxis::X, cfg).unwrap();
        inject_dowel_op(&mut p, &stock, &centres, cfg).unwrap();
        assert_eq!(p.tools.len(), 2);
        assert_eq!(p.operations[0].tool_id, 2);
        assert_eq!(p.tools[1].tip_angle_deg, 118.);
        assert_eq!(p.tools[1].speed_drill, Some(3000));
        let mut huge = stock.clone();
        huge.origin = [1e308, 0.];
        assert!(dowel_centres(&huge, FlipAxis::X, cfg).is_err());
        let mut back = pocket(2, WorkpieceSide::Back, -3.);
        back.kind = OpKind::PlanarSurface {
            surface: crate::cam::planar_surface::PlanarSurfaceParams {
                origin: [0., 0.],
                width_mm: 20.,
                height_mm: 40.,
                margin_mm: 0.,
                overtravel_mm: 0.,
                stepover_mm: 1.,
                direction: crate::cam::surface_mill::ScanDirection::AlongX,
            },
        };
        assert!(matches!(
            run2(two_sided(
                20.,
                40.,
                vec![pocket(1, WorkpieceSide::Front, -3.), back]
            )),
            Err(PipelineError::Fixture(_))
        ));
    }
}
