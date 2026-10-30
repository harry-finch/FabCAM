//! V2 Pocket proof: one outer loop and disjoint single-level islands.
//! Verification-only geometry, independent of the offset kernel. Tangential
//! line/arc joins and polygon corners have analytic reference offsets. A chord
//! enclosure is debited on BOTH source and trajectory; uncertain trimming refuses.
use super::{
    add, cover_lines, decoded_arc_enclosure, distance, flatten, mul, normal, segment_distance, sub,
    topology, Budget, CancelToken, Chord, Point2, Polygon, Segment, VcObject, EPS, FLAT, MAX_PATHS,
    OUTPUT_ERROR, PLAN_ERROR,
};

struct Boundary {
    polygon: Polygon,
    source: Vec<Segment>,
    curved: bool,
}
impl Boundary {
    fn new(obj: &VcObject) -> Result<Self, String> {
        if !obj.closed || obj.segments.len() < 2 || obj.segments.len() > 256 {
            return Err("Pocket finishing requires closed boundaries with 2–256 primitives".into());
        }
        for (i, s) in obj.segments.iter().enumerate() {
            if s.end != obj.segments[(i + 1) % obj.segments.len()].start
                || s.start.distance(s.end) < 0.001
            {
                return Err(
                    "Pocket finishing requires exact, nondegenerate continuity; repair explicitly"
                        .into(),
                );
            }
        }
        topology(&obj.segments)?;
        let mut source = obj.segments.clone();
        if crate::cam::offsets::object_signed_area(obj) < 0. {
            source.reverse();
            for s in &mut source {
                std::mem::swap(&mut s.start, &mut s.end);
                s.bulge = -s.bulge;
            }
        }
        let edges = flatten(&source)?;
        let polygon = Polygon {
            vertices: edges.iter().map(|e| e[0]).collect(),
            edges,
        };
        Ok(Self {
            curved: source.iter().any(|s| s.bulge != 0.),
            polygon,
            source,
        })
    }
    fn reference(&self, inset: f64, island: bool) -> Result<Vec<Chord>, String> {
        if !self.curved {
            return flatten(&self.polygon.reference(inset, island)?);
        }
        let d = if island { -inset } else { inset };
        let mut result = Vec::new();
        for s in &self.source {
            if s.bulge == 0. {
                let n = mul(normal(sub(s.end, s.start)), d / s.start.distance(s.end));
                result.push(Segment::line(add(s.start, n), add(s.end, n), "proof", 7));
            } else {
                let center = add(
                    mul(add(s.start, s.end), 0.5),
                    mul(
                        normal(sub(s.end, s.start)),
                        (1. - s.bulge * s.bulge) / (4. * s.bulge),
                    ),
                );
                let r = center.distance(s.start);
                let next = r - d * s.bulge.signum();
                if next <= PLAN_ERROR {
                    return Err(
                        "Pocket arc offset collapses; choose a smaller cutter/allowance".into(),
                    );
                }
                result.push(Segment::arc(
                    add(center, mul(sub(s.start, center), next / r)),
                    add(center, mul(sub(s.end, center), next / r)),
                    s.bulge,
                    Some(center),
                    "proof",
                    7,
                ));
            }
        }
        // Tangential joins only for the curved family; no invented intersection
        // trimming. Tiny arithmetic residuals stay bounded, never alter source.
        for i in 0..result.len() {
            let j = (i + 1) % result.len();
            if result[i].end.distance(result[j].start) > 1e-8 {
                return Err(
                    "Non-tangential line/arc finishing join needs unqualified trimming".into(),
                );
            }
        }
        topology(&result)?;
        flatten(&result)
    }
}

pub(in crate::pipeline::op_drivers) struct PocketRegion {
    boundaries: Vec<Boundary>,
    outer: usize,
}
impl PocketRegion {
    pub(in crate::pipeline::op_drivers) fn new(
        objects: &[VcObject],
        selected: &[usize],
    ) -> Result<Self, String> {
        if selected.is_empty() || selected.len() > 9 {
            return Err(
                "Pocket finishing supports one region and at most eight simple islands".into(),
            );
        }
        let boundaries: Vec<_> = selected
            .iter()
            .map(|&i| Boundary::new(&objects[i]))
            .collect::<Result<_, _>>()?;
        let sources: Vec<_> = boundaries
            .iter()
            .flat_map(|b| b.source.iter().cloned())
            .collect();
        topology(&sources)?;
        let roots: Vec<_> = (0..boundaries.len())
            .filter(|&i| {
                !(0..boundaries.len())
                    .any(|j| j != i && boundaries[j].polygon.inside(boundaries[i].source[0].start))
            })
            .collect();
        if roots.len() != 1 {
            return Err("Pocket finishing needs one outer boundary; disconnected regions require separate operations".into());
        }
        let outer = roots[0];
        for i in 0..boundaries.len() {
            if i != outer
                && (0..boundaries.len()).any(|j| {
                    j != i
                        && j != outer
                        && boundaries[j].polygon.inside(boundaries[i].source[0].start)
                })
            {
                return Err(
                    "Nested islands beyond one level are not qualified for finishing".into(),
                );
            }
        }
        Ok(Self { boundaries, outer })
    }
    fn edges(&self) -> usize {
        self.boundaries.iter().map(|b| b.polygon.edges.len()).sum()
    }
    fn clearance(&self, p: Point2) -> f64 {
        self.boundaries
            .iter()
            .enumerate()
            .map(|(i, b)| {
                if i == self.outer {
                    b.polygon.clearance(p)
                } else {
                    -b.polygon.clearance(p)
                }
            })
            .fold(f64::INFINITY, f64::min)
    }
    fn reference(&self, inset: f64) -> Result<Vec<Chord>, String> {
        let mut paths = Vec::new();
        for (i, b) in self.boundaries.iter().enumerate() {
            paths.extend(b.reference(inset, i != self.outer)?);
            if paths.len() > MAX_PATHS {
                return Err("Pocket reference proof budget exceeded".into());
            }
        }
        Ok(paths)
    }
    pub(in crate::pipeline::op_drivers) fn safe(
        &self,
        paths: &[Chord],
        inset: f64,
        error: f64,
        cancel: Option<&CancelToken>,
    ) -> Result<(), String> {
        let mut budget = Budget { work: 0, cancel };
        for path in paths {
            budget.use_work(self.edges())?;
            if self.clearance(mul(add(path[0], path[1]), 0.5)) < -FLAT
                || self
                    .boundaries
                    .iter()
                    .flat_map(|b| &b.polygon.edges)
                    .any(|e| segment_distance(*path, *e) < inset - error + 2. * FLAT)
            {
                return Err(
                    "Pocket trajectory gouges a wall, island or retained-stock envelope".into(),
                );
            }
        }
        Ok(())
    }
    pub(in crate::pipeline::op_drivers) fn prove(
        &self,
        radius: f64,
        inset: f64,
        fill: bool,
        paths: &[Chord],
        error: f64,
        cancel: Option<&CancelToken>,
    ) -> Result<(), String> {
        if !radius.is_finite()
            || radius <= 2. * error
            || !inset.is_finite()
            || inset < radius
            || paths.is_empty()
            || paths.len() > MAX_PATHS
        {
            return Err("Pocket cutter/path exceeds proof bounds".into());
        }
        let reference = self.reference(inset)?;
        self.safe(&reference, inset, PLAN_ERROR, cancel)?;
        self.safe(paths, inset, error, cancel)?;
        let mut budget = Budget { work: 0, cancel };
        cover_lines(&reference, paths, error - 3. * FLAT, &mut budget)?;
        if !fill {
            cover_lines(paths, &reference, error - 3. * FLAT, &mut budget)?;
            return Ok(());
        }
        self.fill(
            inset,
            &paths
                .iter()
                .map(|p| (*p, radius + error))
                .collect::<Vec<_>>(),
            &mut budget,
        )
    }
    pub(in crate::pipeline::op_drivers) fn prove_combined_floor(
        &self,
        inset: f64,
        main: &[Chord],
        main_radius: f64,
        finishing: &[Chord],
        finish_radius: f64,
        error: f64,
        cancel: Option<&CancelToken>,
    ) -> Result<(), String> {
        let capsules: Vec<_> = main
            .iter()
            .map(|p| (*p, main_radius + error))
            .chain(finishing.iter().map(|p| (*p, finish_radius + error)))
            .collect();
        if capsules.len() > MAX_PATHS {
            return Err("Pocket combined removal proof budget exceeded".into());
        }
        self.fill(inset, &capsules, &mut Budget { work: 0, cancel })
    }
    fn fill(
        &self,
        inset: f64,
        paths: &[(Chord, f64)],
        budget: &mut Budget<'_>,
    ) -> Result<(), String> {
        let mut b = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for p in &self.boundaries[self.outer].polygon.vertices {
            b[0] = b[0].min(p.x - FLAT);
            b[1] = b[1].min(p.y - FLAT);
            b[2] = b[2].max(p.x + FLAT);
            b[3] = b[3].max(p.y + FLAT);
        }
        let mut pending = vec![(b, (0..paths.len()).collect::<Vec<_>>())];
        while let Some((b, candidates)) = pending.pop() {
            budget.use_work(self.edges() + candidates.len())?;
            let center = Point2::new((b[0] + b[2]) * 0.5, (b[1] + b[3]) * 0.5);
            let halfdiag = (b[2] - b[0]).hypot(b[3] - b[1]) * 0.5;
            if self.clearance(center) + halfdiag + FLAT < inset - EPS {
                continue;
            }
            let candidates: Vec<_> = candidates
                .into_iter()
                .filter(|&i| {
                    let (p, radius) = paths[i];
                    p[0].x.min(p[1].x) - radius <= b[2]
                        && p[0].x.max(p[1].x) + radius >= b[0]
                        && p[0].y.min(p[1].y) - radius <= b[3]
                        && p[0].y.max(p[1].y) + radius >= b[1]
                })
                .collect();
            let corners = [
                Point2::new(b[0], b[1]),
                Point2::new(b[2], b[1]),
                Point2::new(b[2], b[3]),
                Point2::new(b[0], b[3]),
            ];
            if candidates.iter().any(|&i| {
                corners
                    .iter()
                    .all(|c| distance(*c, paths[i].0) <= paths[i].1 - FLAT - EPS)
            }) {
                continue;
            }
            if halfdiag < 1e-6 || pending.len() > 100_000 {
                return Err("Pocket finishing floor coverage is incomplete; select another fill strategy/cutter".into());
            }
            pending.extend(
                [
                    [b[0], b[1], center.x, center.y],
                    [center.x, b[1], b[2], center.y],
                    [b[0], center.y, center.x, b[3]],
                    [center.x, center.y, b[2], b[3]],
                ]
                .map(|tile| (tile, candidates.clone())),
            );
        }
        Ok(())
    }
}

/// Every authored axial level is checked against decoded motions. A full-depth
/// pass alone cannot mask a missing earlier wall/clearing pass. Direct vertical
/// entry and above-stock rapids are independently checked, not credited as fill.
pub(super) fn verify_decoded(
    project: &crate::project::Project,
    op: &crate::project::Op,
    objects: &[VcObject],
    path: &[crate::gcode::preview::ToolpathSegment],
    bindings: &[crate::pipeline::stage_provenance::StageBinding],
    cancel: Option<&CancelToken>,
) -> Result<(), String> {
    use crate::gcode::preview::MoveKind;
    let contract = op.params.stages.as_ref().ok_or("Missing stages")?;
    let scope = contract.pocket_scope().ok_or("Missing explicit scope")?;
    let finish = contract.finishing().ok_or("Missing finishing")?;
    let region = PocketRegion::new(objects, &crate::pipeline::ordered_selection(op, objects))?;
    let mut main_floor = None;
    for binding in bindings.iter().filter(|b| b.operation_id == op.id) {
        let main = binding.stage == "main";
        let depth = op.params.depth - op.params.through_depth.max(0.)
            + if main { finish.floor_allowance_mm } else { 0. };
        let start = if !main && !scope.walls() {
            depth + finish.floor_allowance_mm
        } else {
            op.params.start_depth
        };
        let step = binding
            .parameters
            .depth_per_pass_mm
            .ok_or("Missing axial schedule")?;
        let tool = project
            .tools
            .iter()
            .find(|t| t.id == binding.parameters.tool_id)
            .ok_or("Missing cutter")?;
        let radius = tool.effective_diameter() * 0.5;
        let inset = radius
            + op.params.stock_to_leave_mm
            + if main {
                finish.lateral_allowance_mm
            } else {
                0.
            };
        let reach = tool
            .flute_length_mm
            .ok_or("Known usable flute length required")?;
        if !reach.is_finite() || reach < op.params.start_depth - depth || step > reach {
            return Err("Pocket axial wall/cutter reach is insufficient".into());
        }
        let count = ((start - depth) / step).ceil().max(1.);
        if !count.is_finite() || count > 1000. {
            return Err("Pocket axial proof budget exceeded (1000 levels)".into());
        }
        let mut levels: Vec<_> = (1..=count as usize)
            .map(|n| {
                (
                    (start - step * n as f64).max(depth),
                    Vec::<Segment>::new(),
                    std::collections::BTreeSet::new(),
                )
            })
            .collect();
        let mut all = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        let mut enclosure: f64 = 0.;
        let mut budget = Budget { work: 0, cancel };
        for m in &path[binding.first_segment..binding.end_segment_exclusive] {
            budget.use_work(1)?;
            if m.kind == MoveKind::Rapid {
                if m.from.z.min(m.to.z) < op.params.start_depth - 1e-4
                    && ((m.from.x - m.to.x).abs() > 1e-8
                        || (m.from.y - m.to.y).abs() > 1e-8
                        || m.to.z < m.from.z - 1e-8)
                {
                    return Err(
                        "Pocket rapid enters uncleared material below the authored start surface"
                            .into(),
                    );
                }
                continue;
            }
            if m.from.z.min(m.to.z) > op.params.start_depth + 1e-4 {
                continue;
            }
            if m.from.z.min(m.to.z) < depth - 1e-4 {
                return Err("Pocket decoded path exceeds authored stage depth".into());
            }
            let a = Point2::new(m.from.x, m.from.y);
            let b = Point2::new(m.to.x, m.to.y);
            if (m.from.z - m.to.z).abs() > 1e-4 && a.distance(b) > 1e-8 {
                return Err("Pocket finishing requires proven direct vertical entries; ramp/helix motion refused".into());
            }
            let s = if let Some(arc) = m.arc {
                let c = Point2::new(arc.cx, arc.cy);
                if (a.distance(c) - b.distance(c)).abs() > 0.0003 {
                    return Err("Pocket decoded arc radius is inconsistent".into());
                }
                let sa = (a.y - c.y).atan2(a.x - c.x);
                let ea = (b.y - c.y).atan2(b.x - c.x);
                let sweep = if arc.ccw {
                    (ea - sa).rem_euclid(std::f64::consts::TAU)
                } else {
                    -(sa - ea).rem_euclid(std::f64::consts::TAU)
                };
                let bulge = (sweep / 4.).tan();
                enclosure = enclosure.max(decoded_arc_enclosure(a, b, bulge, c)?);
                Segment::arc(a, b, bulge, Some(c), "decoded", 7)
            } else {
                Segment::line(a, b, "decoded", 7)
            };
            let key = [
                a.x.to_bits(),
                a.y.to_bits(),
                b.x.to_bits(),
                b.y.to_bits(),
                s.bulge.to_bits(),
            ];
            if seen.insert(key) {
                all.push(s.clone());
            }
            if all.len() > MAX_PATHS {
                return Err("Pocket decoded planar proof budget exceeded".into());
            }
            if (m.from.z - m.to.z).abs() < 1e-4 && a.distance(b) > 1e-8 {
                for (z, segments, seen) in &mut levels {
                    budget.use_work(1)?;
                    if (m.from.z - *z).abs() <= 1e-4
                        && (m.to.z - *z).abs() <= 1e-4
                        && seen.insert(key)
                    {
                        segments.push(s.clone());
                    }
                }
            }
        }
        let error = OUTPUT_ERROR - enclosure;
        if error <= 3. * FLAT + EPS {
            return Err("Pocket arc enclosure exhausts output error budget".into());
        }
        region.safe(&flatten(&all)?, inset, error, cancel)?;
        // Identical planar levels share the proof; each level still has its own
        // decoded coverage set and cannot borrow a missing motion from another Z.
        let mut proven = std::collections::BTreeSet::new();
        for (z, segments, keys) in levels {
            if keys.is_empty() {
                return Err(format!("Pocket axial coverage missing at Z={z:.4} mm"));
            }
            let chords = flatten(&segments)?;
            if (z - depth).abs() < 1e-4 {
                if main {
                    main_floor = Some((chords.clone(), radius));
                } else if !scope.floor() {
                    let (prior, prior_radius) = main_floor
                        .as_ref()
                        .ok_or("Main floor not established before wall finishing")?;
                    region.prove_combined_floor(
                        inset,
                        prior,
                        *prior_radius,
                        &chords,
                        radius,
                        error,
                        cancel,
                    )?;
                }
            }
            if proven.insert(keys) {
                if proven.len() > 8 {
                    return Err("Pocket proof exceeds eight distinct planar depth-level geometries per stage".into());
                }
                region.prove(radius, inset, main || scope.floor(), &chords, error, cancel)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    #[test]
    fn decoded_wall_floor_axial_and_motion_mutations_refuse() {
        for scope in ["walls_only", "floor_only", "walls_and_floor"] {
            let mut v: Value = serde_json::from_str(include_str!(
                "../../../../../../tests/fixtures/alpha31/convex-pocket.request.json"
            ))
            .unwrap();
            v["project"]["tools"][0]["flute_length_mm"] = json!(12.);
            let s = &mut v["project"]["operations"][0]["params"]["stages"];
            s["contract"] = json!("pocket_finishing_v2");
            s["finishing"]["scope"] = json!(scope);
            if scope == "walls_only" {
                s["finishing"]["floor_allowance_mm"] = json!(0.);
            }
            if scope == "floor_only" {
                s["finishing"]["lateral_allowance_mm"] = json!(0.);
            }
            let req: crate::pipeline::PipelineRequest = serde_json::from_value(v).unwrap();
            let objects = crate::cam::chaining::segments_to_objects(&req.project.segments);
            let r = crate::pipeline::run_pipeline(req.clone(), |_, _, _| {}).unwrap();
            for mutation in [
                "missing_floor",
                "missing_axial",
                "missing_wall",
                "gouge",
                "rapid",
                "deep",
            ] {
                let mut path = r.toolpath.clone();
                let b = &r.cutting_stages[1];
                let movements = &mut path[b.first_segment..b.end_segment_exclusive];
                let mut changed = false;
                for m in movements
                    .iter_mut()
                    .filter(|m| m.from.z < 0. && m.to.z < 0. && (m.from.x - m.to.x).abs() > 1e-8)
                {
                    match mutation {
                        "missing_floor" => {
                            if m.from.z < -0.49 {
                                m.from.z = 1.;
                                m.to.z = 1.;
                                changed = true;
                            }
                        }
                        "missing_axial" => {
                            if (m.from.z + if scope == "floor_only" { 0.5 } else { 0.2 }).abs()
                                < 0.0001
                            {
                                m.from.z = 1.;
                                m.to.z = 1.;
                                changed = true;
                            }
                        }
                        "missing_wall" => {
                            let polygon = Polygon::new(&objects[0]).unwrap();
                            if polygon.clearance(Point2::new(m.from.x, m.from.y)) < 1.02
                                && polygon.clearance(Point2::new(m.to.x, m.to.y)) < 1.02
                            {
                                m.from.z = 1.;
                                m.to.z = 1.;
                                changed = true;
                            }
                        }
                        "gouge" => {
                            m.from.x -= 30.;
                            m.to.x -= 30.;
                            changed = true;
                        }
                        "rapid" => {
                            m.kind = crate::gcode::preview::MoveKind::Rapid;
                            changed = true;
                        }
                        _ => {
                            m.from.z -= 0.1;
                            m.to.z -= 0.1;
                            changed = true;
                        }
                    }
                }
                assert!(changed);
                assert!(
                    verify_decoded(
                        &req.project,
                        &req.project.operations[0],
                        &objects,
                        &path,
                        &r.cutting_stages,
                        None
                    )
                    .is_err(),
                    "{scope}/{mutation}"
                );
            }
        }
    }
}
