//! Bounded per-pocket planar removal diagnostic over FINAL returned movements.
//! No stock simulation, toolpath mutation, cause inference or post syntax here.
use super::RegionPreview;
use crate::cam::{source_combine::polytree_to_regions, VcObject};
use crate::gcode::preview::{MoveKind, ToolpathSegment};
use crate::geometry::{Point2, SegmentKind};
use crate::project::{
    MachineMode, OpKind, PocketStrategy, Project, TabPlacementMode, ToolKind, UnitSystem,
};
use clipper2_rust::{
    boolean_op_tree_d, inflate_paths_d, ClipType, EndType, FillRule, JoinType, PathD, PathsD,
    Point, PolyTreeD,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Numerical model limits in mm/mm²; none is a physical machining tolerance.
const PRECISION: i32 = 5;
const SWEEP_ARC_ERROR_MM: f64 = 0.005;
const MIN_REGION_AREA_MM2: f64 = 0.02;
const DEPTH_EPS_MM: f64 = 0.0001; // emitted coordinate precision, not topology
const MAX_MOVES: usize = 5_000;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemainingMaterial {
    pub op_id: u32,
    /// ready or unsupported; zero area is meaningful only when ready.
    pub status: String,
    pub reason: String,
    pub depth_mm: f64,
    pub area_mm2: f64,
    pub largest_area_mm2: f64,
    pub regions: Vec<RegionPreview>,
}

pub(super) fn analyze(
    project: &Project,
    objects: &[VcObject],
    targets: &[RegionPreview],
    toolpath: &[ToolpathSegment],
    execution_path: bool,
    cache: Option<&crate::pipeline_cache::PipelineCache>,
) -> Vec<RemainingMaterial> {
    project
        .operations
        .iter()
        .filter(|op| op.enabled)
        .map(|op| {
            let depth = op.params.depth - op.params.through_depth.max(0.0);
            let mut result = RemainingMaterial {
                op_id: op.id,
                status: "unsupported".into(),
                reason: String::new(),
                depth_mm: depth,
                area_mm2: 0.,
                largest_area_mm2: 0.,
                regions: Vec::new(),
            };
            let tool = project.tools.iter().find(|tool| tool.id == op.tool_id);
            let pocket = op.pocket_params();
            let contour = op.contour_params();
            let selected = super::ordered_selection(op, objects);
            let moves: Vec<_> = toolpath.iter().filter(|m| m.op_id == op.id).collect();
            let own_targets: Vec<_> = targets
                .iter()
                .filter(|r| r.op_id == op.id)
                .cloned()
                .collect();
            let reason = if !execution_path {
                Some("execution_path_unavailable")
            } else if project.machine.mode != MachineMode::Mill
                || project.machine.unit != UnitSystem::Mm
            {
                Some("metric_milling_only")
            } else if !matches!(
                op.kind,
                OpKind::Pocket {
                    strategy: PocketStrategy::Cascade,
                    ..
                }
            ) {
                Some("cascade_pocket_only")
            } else if !tool.is_some_and(|t| {
                t.kind == ToolKind::Endmill && t.diameter.is_finite() && t.diameter > 0.
            }) || op.finish_tool_id.is_some()
            {
                Some("single_flat_cutter_only")
            } else if contour
                .is_some_and(|c| c.tabs.active || !matches!(c.tab_mode, TabPlacementMode::Off))
            {
                Some("intentional_tabs")
            } else if op.params.stock_to_leave_mm != 0.
                || pocket.is_some_and(|p| p.finish_xy_allowance_mm.unwrap_or(0.) != 0.)
            {
                Some("intentional_allowance")
            } else if !op.params.depth_list.is_empty() || !depth.is_finite() {
                Some("depth_schedule")
            } else if selected.iter().any(|&i| {
                !objects[i].closed
                    || objects[i]
                        .segments
                        .iter()
                        .any(|s| !matches!(s.kind, SegmentKind::Line))
            }) || pocket.is_some_and(|p| p.frame_shape.is_some())
            {
                Some("target_representation")
            } else if moves.iter().any(|m| m.arc.is_some()) {
                Some("analytic_arc_path")
            } else if moves.len() > MAX_MOVES {
                Some("analysis_budget")
            } else if pocket.is_some_and(|p| !p.pocket_islands)
                && own_targets.iter().any(|r| !r.holes.is_empty())
            {
                Some("target_representation")
            } else if own_targets.is_empty() {
                Some("missing_target")
            } else if !project.stock.as_ref().is_some_and(|s| {
                s.top_z_mm == 0. && s.thickness_mm > 0. && depth < 0. && depth >= -s.thickness_mm
            }) {
                Some("stock_depth_context")
            } else {
                None
            };
            if let Some(reason) = reason {
                result.reason = reason.into();
                return result;
            }
            let stock = project.stock.as_ref().expect("qualified stock");
            let bounds = [
                stock.origin[0],
                stock.origin[1],
                stock.origin[0] + stock.width_mm,
                stock.origin[1] + stock.height_mm,
            ];
            if own_targets
                .iter()
                .flat_map(|r| &r.outer)
                .any(|p| p.x < bounds[0] || p.y < bounds[1] || p.x > bounds[2] || p.y > bounds[3])
            {
                result.reason = "target_outside_stock".into();
                return result;
            }
            let radius = tool.expect("qualified tool").diameter * 0.5;
            result.regions = memoized_regions(&own_targets, &moves, radius, depth, cache);
            let areas: Vec<_> = result.regions.iter().map(region_area).collect();
            result.area_mm2 = areas.iter().sum();
            result.largest_area_mm2 = areas.into_iter().fold(0., f64::max);
            result.status = "ready".into();
            result.reason = "resolved_path_at_depth".into();
            result
        })
        .collect()
}

// All eligibility guards run before this function. Stock/operation changes that
// leave the qualified kernel inputs identical may safely reuse only its output.
fn memoized_regions(
    targets: &[RegionPreview],
    moves: &[&ToolpathSegment],
    radius: f64,
    depth: f64,
    cache: Option<&crate::pipeline_cache::PipelineCache>,
) -> Vec<RegionPreview> {
    let key = cache.and_then(|_| {
        crate::profile_measure!(
            "preview.remaining_key",
            remaining_key(targets, moves, radius, depth)
        )
    });
    if let Some(regions) = key.as_ref().and_then(|k| cache?.remaining_memo.get(k)) {
        crate::profile_count!("preview.remaining_cache_hit", 1);
        return regions;
    }
    crate::profile_count!("preview.remaining_cache_miss", 1);
    let result = remaining_regions(targets, moves, radius, depth);
    if let (Some(cache), Some(key)) = (cache, key) {
        cache.remaining_memo.put(key, &result);
    }
    result
}

// Exact binary identity, not a digest: metadata (op/line IDs, feed and strings)
// is absent because the planar kernel reads only target topology and XYZ cuts.
// Ring lengths preserve outer/hole boundaries; full IEEE bits prevent collisions.
fn remaining_key(
    targets: &[RegionPreview],
    moves: &[&ToolpathSegment],
    radius: f64,
    depth: f64,
) -> Option<Vec<u8>> {
    const MAX_KEY_BYTES: usize = 8 * 1024 * 1024;
    if !radius.is_finite() || !depth.is_finite() {
        return None;
    }
    let relevant = |m: &&ToolpathSegment| {
        m.kind != MoveKind::Rapid
            && (m.from.z <= depth + DEPTH_EPS_MM || m.to.z <= depth + DEPTH_EPS_MM)
    };
    let mut words = 4usize.checked_add(
        moves
            .iter()
            .filter(|m| relevant(m))
            .count()
            .checked_mul(6)?,
    )?;
    for r in targets {
        words = words.checked_add(2)?;
        for ring in std::iter::once(&r.outer).chain(r.holes.iter()) {
            words = words
                .checked_add(1)?
                .checked_add(ring.len().checked_mul(2)?)?;
            if ring.iter().any(|p| !p.x.is_finite() || !p.y.is_finite()) {
                return None;
            }
        }
    }
    // Preserve the old nonfinite bypass even for moves excluded by the kernel.
    if moves.iter().any(|m| {
        [m.from.x, m.from.y, m.from.z, m.to.x, m.to.y, m.to.z]
            .iter()
            .any(|n| !n.is_finite())
    }) {
        return None;
    }
    let bytes = words.checked_mul(8)?;
    if bytes > MAX_KEY_BYTES {
        return None;
    }
    let mut key = Vec::with_capacity(bytes);
    let mut word = |n: u64| key.extend_from_slice(&n.to_le_bytes());
    word(u64::from(crate::pipeline_cache::PIPELINE_VERSION));
    word(radius.to_bits());
    word(depth.to_bits());
    word(targets.len() as u64);
    for r in targets {
        word(u64::from(r.op_id)); // output attribution is observable
        word(r.holes.len() as u64);
        for ring in std::iter::once(&r.outer).chain(r.holes.iter()) {
            word(ring.len() as u64);
            for p in ring {
                word(p.x.to_bits());
                word(p.y.to_bits());
            }
        }
    }
    for m in moves.iter().filter(|m| relevant(m)) {
        for n in [m.from.x, m.from.y, m.from.z, m.to.x, m.to.y, m.to.z] {
            word(n.to_bits());
        }
    }
    Some(key)
}

fn ring_path(points: &[Point2], positive: bool) -> PathD {
    let mut path: PathD = points.iter().map(|p| Point::new(p.x, p.y)).collect();
    let signed: f64 = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum();
    if (signed > 0.) != positive {
        path.reverse();
    }
    path
}

// Clip each linear XYZ move to the portion at/below the analysis plane. Earlier
// passes and the upper part of ramp/plunge/retract do not remove this plane.
fn at_depth(m: &ToolpathSegment, depth: f64) -> Option<PathD> {
    if m.kind == MoveKind::Rapid {
        return None;
    }
    let a = m.from.z <= depth + DEPTH_EPS_MM;
    let b = m.to.z <= depth + DEPTH_EPS_MM;
    if !a && !b {
        return None;
    }
    let point = |t: f64| {
        Point::new(
            m.from.x + t * (m.to.x - m.from.x),
            m.from.y + t * (m.to.y - m.from.y),
        )
    };
    let crossing = if a != b {
        ((depth - m.from.z) / (m.to.z - m.from.z)).clamp(0., 1.)
    } else {
        0.
    };
    Some(vec![
        point(if a { 0. } else { crossing }),
        point(if b { 1. } else { crossing }),
    ])
}

fn remaining_regions(
    targets: &[RegionPreview],
    moves: &[&ToolpathSegment],
    radius: f64,
    depth: f64,
) -> Vec<RegionPreview> {
    let mut subject = PathsD::new();
    for target in targets {
        subject.push(ring_path(&target.outer, true));
        subject.extend(target.holes.iter().map(|h| ring_path(h, false)));
    }
    let mut sweep = PathsD::new();
    for path in moves.iter().filter_map(|m| at_depth(m, depth)) {
        // A plunge is a disk, too: Clipper open-path offset needs distinct
        // endpoints, so use a polygonal disk for zero XY travel.
        if path[0] == path[1] {
            // Match the rounded capsule approximation; a fixed vertex count
            // would exceed the declared geometric error on large cutters.
            let count = ((std::f64::consts::PI
                / (1.0 - (SWEEP_ARC_ERROR_MM / radius).min(1.0)).acos())
            .ceil()
            .max(16.0)) as u32;
            sweep.push(
                (0..count)
                    .map(|i| {
                        let angle = std::f64::consts::TAU * f64::from(i) / f64::from(count);
                        Point::new(
                            path[0].x + radius * angle.cos(),
                            path[0].y + radius * angle.sin(),
                        )
                    })
                    .collect(),
            );
        } else {
            sweep.extend(inflate_paths_d(
                &vec![path],
                radius,
                JoinType::Round,
                EndType::Round,
                2.,
                PRECISION,
                SWEEP_ARC_ERROR_MM,
            ));
        }
    }
    let mut tree = PolyTreeD::new();
    boolean_op_tree_d(
        ClipType::Difference,
        FillRule::NonZero,
        &subject,
        &sweep,
        &mut tree,
        PRECISION,
    );
    let op_id = targets[0].op_id;
    polytree_to_regions(&tree, 0, "remaining", 0)
        .into_iter()
        .map(|r| RegionPreview {
            op_id,
            outer: r.boundary,
            holes: r.holes,
        })
        .filter(|r| region_area(r) >= MIN_REGION_AREA_MM2)
        .collect()
}
fn polygon_area(points: &[Point2]) -> f64 {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum::<f64>()
        .abs()
        * 0.5
}
fn region_area(r: &RegionPreview) -> f64 {
    polygon_area(&r.outer) - r.holes.iter().map(|h| polygon_area(h)).sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gcode::preview::Pose3;
    fn square(a: f64, b: f64) -> Vec<Point2> {
        vec![
            Point2::new(a, a),
            Point2::new(b, a),
            Point2::new(b, b),
            Point2::new(a, b),
        ]
    }
    fn movement(a: [f64; 3], b: [f64; 3], kind: MoveKind) -> ToolpathSegment {
        ToolpathSegment {
            from: Pose3 {
                x: a[0],
                y: a[1],
                z: a[2],
            },
            to: Pose3 {
                x: b[0],
                y: b[1],
                z: b[2],
            },
            kind,
            op_id: 1,
            gcode_line: 1,
            arc: None,
        }
    }
    #[test]
    fn sharp_square_corner_matches_analytical_residual() {
        let target = RegionPreview {
            op_id: 1,
            outer: square(0., 20.),
            holes: vec![],
        };
        let mut moves = vec![
            movement([3., 3., -2.], [17., 3., -2.], MoveKind::Cut),
            movement([17., 3., -2.], [17., 17., -2.], MoveKind::Cut),
            movement([17., 17., -2.], [3., 17., -2.], MoveKind::Cut),
            movement([3., 17., -2.], [3., 3., -2.], MoveKind::Cut),
        ];
        for y in [3., 8., 13., 17.] {
            moves.push(movement([3., y, -2.], [17., y, -2.], MoveKind::Cut));
        }
        let refs = moves.iter().collect::<Vec<_>>();
        let result = remaining_regions(&[target], &refs, 3., -2.);
        let area: f64 = result.iter().map(region_area).sum();
        assert_eq!(result.len(), 4);
        assert!(
            (area - (4. - std::f64::consts::PI) * 9.).abs() < 0.15,
            "area={area}"
        );
    }
    #[test]
    fn shallower_and_rapid_paths_do_not_erase_residuals() {
        let target = RegionPreview {
            op_id: 1,
            outer: square(0., 10.),
            holes: vec![],
        };
        let a = movement([0., 5., -1.], [10., 5., -1.], MoveKind::Cut);
        let b = movement([0., 5., -2.], [10., 5., -2.], MoveKind::Rapid);
        let result = remaining_regions(&[target], &[&a, &b], 6., -2.);
        assert!((result.iter().map(region_area).sum::<f64>() - 100.).abs() < 0.001);
    }
    #[test]
    fn islands_are_not_reported_as_remaining() {
        let target = RegionPreview {
            op_id: 1,
            outer: square(0., 10.),
            holes: vec![square(3., 7.)],
        };
        let result = remaining_regions(&[target], &[], 1., -2.);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].holes.len(), 1);
        assert!((region_area(&result[0]) - 84.).abs() < 0.001);
    }
    #[test]
    fn depth_crossing_and_vertical_disk_are_included() {
        let ramp = movement([0., 0., 0.], [10., 0., -4.], MoveKind::Cut);
        let path = at_depth(&ramp, -2.).unwrap();
        assert_eq!(path[0].x, 5.);
        let disk = movement([5., 5., 0.], [5., 5., -2.], MoveKind::Plunge);
        let target = RegionPreview {
            op_id: 1,
            outer: square(0., 10.),
            holes: vec![],
        };
        let result = remaining_regions(&[target], &[&disk], 1., -2.);
        // Circle polygon error is bounded by perimeter * 0.005mm plus
        // boolean rounding: this is an area error, not physical tolerance.
        assert!(
            (result.iter().map(region_area).sum::<f64>() - (100. - std::f64::consts::PI)).abs()
                < 0.032
        );
    }
    #[test]
    fn sufficiently_small_corner_residual_is_filtered_by_declared_area_threshold() {
        let target = RegionPreview {
            op_id: 1,
            outer: square(0., 1.),
            holes: vec![],
        };
        let mut moves = vec![
            movement([0.1, 0.1, -2.], [0.9, 0.1, -2.], MoveKind::Cut),
            movement([0.9, 0.1, -2.], [0.9, 0.9, -2.], MoveKind::Cut),
            movement([0.9, 0.9, -2.], [0.1, 0.9, -2.], MoveKind::Cut),
            movement([0.1, 0.9, -2.], [0.1, 0.1, -2.], MoveKind::Cut),
        ];
        for i in 1..=9 {
            let y = f64::from(i) * 0.1;
            moves.push(movement([0.1, y, -2.], [0.9, y, -2.], MoveKind::Cut));
        }
        let refs = moves.iter().collect::<Vec<_>>();
        let first = remaining_regions(std::slice::from_ref(&target), &refs, 0.1, -2.);
        assert!(first.is_empty());
        let repeated = remaining_regions(&[target], &refs, 0.1, -2.);
        assert_eq!(
            serde_json::to_value(first).unwrap(),
            serde_json::to_value(repeated).unwrap()
        );
    }
    #[test]
    fn concave_target_clips_sweep_and_keeps_only_the_unmachined_arm() {
        let outer = vec![
            Point2::new(0., 0.),
            Point2::new(10., 0.),
            Point2::new(10., 5.),
            Point2::new(5., 5.),
            Point2::new(5., 10.),
            Point2::new(0., 10.),
        ];
        let target = RegionPreview {
            op_id: 1,
            outer,
            holes: vec![],
        };
        let cut = movement([0., 0., -2.], [10., 0., -2.], MoveKind::Cut);
        let result = remaining_regions(&[target], &[&cut], 5., -2.);
        assert!((result.iter().map(region_area).sum::<f64>() - 25.).abs() < 0.01);
        assert!(result
            .iter()
            .flat_map(|r| &r.outer)
            .all(|p| p.x >= -0.00001 && p.x <= 5.00001 && p.y >= 4.99999 && p.y <= 10.00001));
    }
    #[test]
    fn memo_kernel_exact_inputs_include_holes_movements_radius_and_depth() {
        let cache = crate::pipeline_cache::PipelineCache::new(4);
        let target = RegionPreview {
            op_id: 1,
            outer: vec![
                Point2::new(0., 0.),
                Point2::new(10., 0.),
                Point2::new(10., 10.),
                Point2::new(0., 10.),
            ],
            holes: vec![],
        };
        let cut = movement([0., 0., -2.], [10., 0., -2.], MoveKind::Cut);
        let value = |r| serde_json::to_value(r).unwrap();
        for i in 0..5 {
            let mut t = target.clone();
            let mut m = cut.clone();
            let mut radius = 1.;
            let mut depth = -2.;
            match i {
                1 => t.holes.push(vec![
                    Point2::new(4., 4.),
                    Point2::new(5., 4.),
                    Point2::new(5., 5.),
                ]),
                2 => m.to.y = 5.,
                3 => radius = 2.,
                4 => depth = -3.,
                _ => {}
            }
            let expected = value(remaining_regions(&[t.clone()], &[&m], radius, depth));
            assert_eq!(
                value(memoized_regions(
                    &[t.clone()],
                    &[&m],
                    radius,
                    depth,
                    Some(&cache)
                )),
                expected
            );
            assert_eq!(
                value(memoized_regions(&[t], &[&m], radius, depth, Some(&cache))),
                expected
            );
        }
    }
    #[test]
    fn lot7_binary_key_preserves_geometry_topology_and_output_attribution() {
        let mut target = RegionPreview {
            op_id: 1,
            outer: square(0., 10.),
            holes: vec![],
        };
        let mut m = movement([0., 5., -2.], [10., 5., -2.], MoveKind::Cut);
        let a = remaining_key(&[target.clone()], &[&m], 1., -2.).unwrap();
        let json = serde_json::to_vec(&(
            crate::pipeline_cache::PIPELINE_VERSION,
            &[target.clone()],
            &[&m],
            1.,
            -2.,
        ))
        .unwrap();
        assert!(a.len() < json.len());
        m.op_id = 99;
        m.gcode_line = 400;
        assert_eq!(
            Some(a.clone()),
            remaining_key(&[target.clone()], &[&m], 1., -2.)
        );
        m.to.x = 9.;
        assert_ne!(
            Some(a.clone()),
            remaining_key(&[target.clone()], &[&m], 1., -2.)
        );
        m.to.x = 10.;
        target.op_id = 2;
        assert_ne!(
            Some(a.clone()),
            remaining_key(&[target.clone()], &[&m], 1., -2.)
        );
        target.op_id = 1;
        target.holes.push(square(3., 4.));
        assert_ne!(Some(a), remaining_key(&[target], &[&m], 1., -2.));
        m.to.z = f64::NAN;
        assert!(remaining_key(&[], &[&m], 1., -2.).is_none());
    }
}
