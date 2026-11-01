//! Bounded polygon finishing proof, independent of the offset builder.
//!
//! The reference is an analytical parallel-edge/round-join construction, used
//! only for verification, never emitted as a replacement toolpath. Intersecting,
//! collapsed or trimmed references refuse. Source primitives are never changed.
//! A finite chord enclosure accounts for arc sagitta; continuous capsule interval
//! coverage and convex tile containment prove coverage between samples as well.
#[cfg(test)]
mod perf1;
mod pocket;
use crate::cam::{offsets::PolylineOffset, VcObject};
use crate::geometry::{Point2, Segment, SegmentKind};
use crate::pipeline::CancelToken;
pub(super) use pocket::PocketRegion;

const FLAT: f64 = 0.0001;
const EPS: f64 = 1e-8;
pub(super) const PLAN_ERROR: f64 = 0.003;
pub(super) const OUTPUT_ERROR: f64 = 0.02;
const MAX_WORK: usize = 5_000_000;
const MAX_PATHS: usize = 5_000;
type Chord = [Point2; 2];

struct Budget<'a> {
    work: usize,
    cancel: Option<&'a CancelToken>,
}
impl Budget<'_> {
    fn use_work(&mut self, n: usize) -> Result<(), String> {
        self.work = self
            .work
            .checked_add(n)
            .ok_or("Finishing proof work overflow")?;
        #[cfg(test)]
        perf1::record_work(n, self.work);
        if self.work > MAX_WORK {
            return Err(
                "Finishing polygon proof budget exceeded; reduce contour/path complexity".into(),
            );
        }
        if crate::pipeline::cancelled(self.cancel) {
            return Err("Finishing proof cancelled".into());
        }
        Ok(())
    }
}
fn add(a: Point2, b: Point2) -> Point2 {
    Point2::new(a.x + b.x, a.y + b.y)
}
fn sub(a: Point2, b: Point2) -> Point2 {
    Point2::new(a.x - b.x, a.y - b.y)
}
fn mul(a: Point2, f: f64) -> Point2 {
    Point2::new(a.x * f, a.y * f)
}
fn dot(a: Point2, b: Point2) -> f64 {
    a.x * b.x + a.y * b.y
}
fn cross(a: Point2, b: Point2) -> f64 {
    a.x * b.y - a.y * b.x
}
fn normal(a: Point2) -> Point2 {
    Point2::new(-a.y, a.x)
}
fn distance(p: Point2, c: Chord) -> f64 {
    let d = sub(c[1], c[0]);
    let q = dot(d, d);
    let t = if q == 0. {
        0.
    } else {
        (dot(sub(p, c[0]), d) / q).clamp(0., 1.)
    };
    p.distance(add(c[0], mul(d, t)))
}
fn segment_intersects(a: Chord, b: Chord) -> bool {
    let u = sub(a[1], a[0]);
    let v = sub(b[1], b[0]);
    let w = sub(b[0], a[0]);
    let den = cross(u, v);
    if den != 0. {
        let t = cross(w, v) / den;
        let s = cross(w, u) / den;
        if (0. ..=1.).contains(&t) && (0. ..=1.).contains(&s) {
            return true;
        }
    }
    false
}
fn segment_distance(a: Chord, b: Chord) -> f64 {
    if segment_intersects(a, b) {
        return 0.;
    }
    distance(a[0], b)
        .min(distance(a[1], b))
        .min(distance(b[0], a))
        .min(distance(b[1], a))
}
fn topology(segments: &[Segment]) -> Result<(), String> {
    let drawing = crate::drawing::Drawing::from_sources(vec![crate::drawing::Source {
        id: 1,
        original: segments.to_vec(),
        legacy_objects: vec![],
        pivot: Point2::new(0., 0.),
        transform: crate::drawing::Transform::default(),
        next_entity: 1,
    }])?;
    drawing.validate_for_generation()
}

pub(super) struct Polygon {
    vertices: Vec<Point2>,
    edges: Vec<Chord>,
}
impl Polygon {
    pub(super) fn new(obj: &VcObject) -> Result<Self, String> {
        if !obj.closed || obj.segments.len() < 3 || obj.segments.len() > 256 {
            return Err("Finishing needs one closed polygon with 3–256 straight edges; islands/multiple regions are unsupported".into());
        }
        for (i, s) in obj.segments.iter().enumerate() {
            if s.kind != SegmentKind::Line || s.bulge != 0. {
                return Err("Finishing source curves are not qualified; use a straight-edge polygon or disable Finishing".into());
            }
            if s.end != obj.segments[(i + 1) % obj.segments.len()].start {
                return Err("Finishing requires exact endpoint continuity; preview an explicit drawing repair".into());
            }
            if ![s.start.x, s.start.y, s.end.x, s.end.y]
                .iter()
                .all(|v| v.is_finite() && v.abs() <= 1_000_000.)
                || s.start.distance(s.end) < 0.001
            {
                return Err("Finishing polygon has nonfinite, out-of-range or near-degenerate edges (<0.001 mm)".into());
            }
        }
        topology(&obj.segments).map_err(|e| format!("Finishing source topology: {e}"))?;
        let mut vertices: Vec<_> = obj.segments.iter().map(|s| s.start).collect();
        if crate::cam::offsets::object_signed_area(obj) < 0. {
            vertices.reverse();
        }
        let edges = (0..vertices.len())
            .map(|i| [vertices[i], vertices[(i + 1) % vertices.len()]])
            .collect();
        Ok(Self { vertices, edges })
    }
    fn inside(&self, p: Point2) -> bool {
        crate::geometry::point_in_polygon(&self.vertices, p.x, p.y)
    }
    fn clearance(&self, p: Point2) -> f64 {
        let d = self
            .edges
            .iter()
            .map(|e| distance(p, *e))
            .fold(f64::INFINITY, f64::min);
        if self.inside(p) {
            d
        } else {
            -d
        }
    }
    fn reference(&self, inset: f64, outside: bool) -> Result<Vec<Segment>, String> {
        let n = self.vertices.len();
        let d = if outside { -inset } else { inset };
        let dirs: Vec<_> = self
            .edges
            .iter()
            .map(|e| mul(sub(e[1], e[0]), 1. / e[0].distance(e[1])))
            .collect();
        let mut joins = Vec::new();
        for i in 0..n {
            let u = dirs[(i + n - 1) % n];
            let v = dirs[i];
            let p = self.vertices[i];
            let turn = cross(u, v);
            let cosine = dot(u, v);
            if turn.abs() < 1e-10 || 1. + cosine < 1e-8 {
                return Err("Finishing reference has an ambiguous/near-collinear corner; simplify explicitly or disable Finishing".into());
            }
            if turn * d > 0. {
                let at = add(p, mul(add(normal(u), normal(v)), d / (1. + cosine)));
                joins.push((at, at, 0.));
            } else {
                joins.push((
                    add(p, mul(normal(u), d)),
                    add(p, mul(normal(v), d)),
                    (turn.atan2(cosine) / 4.).tan(),
                ));
            }
        }
        let mut result = Vec::new();
        for i in 0..n {
            let (a, b, bulge) = joins[i];
            if bulge != 0. {
                result.push(Segment::arc(
                    a,
                    b,
                    bulge,
                    Some(self.vertices[i]),
                    "proof",
                    7,
                ));
            }
            let to = joins[(i + 1) % n].0;
            if dot(sub(to, b), dirs[i]) <= 0.001 {
                return Err("Finishing offset collapses or requires unqualified trimming at a narrow corner; reduce cutter/allowance or disable Finishing".into());
            }
            result.push(Segment::line(b, to, "proof", 7));
        }
        topology(&result)
            .map_err(|e| format!("Finishing offset crosses/touches or splits: {e}"))?;
        Ok(result)
    }
    fn safe(
        &self,
        paths: &[Chord],
        inset: f64,
        outside: bool,
        error: f64,
        budget: &mut Budget<'_>,
    ) -> Result<(), String> {
        for path in paths {
            budget.use_work(self.edges.len())?;
            let middle = mul(add(path[0], path[1]), 0.5);
            if self.inside(middle) == outside
                || self
                    .edges
                    .iter()
                    .any(|e| segment_distance(*path, *e) < inset - error + FLAT)
            {
                return Err("Finishing trajectory crosses its permitted cutter/retained-stock envelope; offset or emitted connector is unsafe".into());
            }
        }
        Ok(())
    }
    /// Prove reference-wall coverage and, for a Pocket, the full eroded region.
    /// Boundary coverage + coverage of E proves the reachable opening E ⊕ disk(r),
    /// within the declared wall error: points of the opening outside E project
    /// onto its boundary. No claim is made for inaccessible sharp corners.
    pub(super) fn prove(
        &self,
        radius: f64,
        inset: f64,
        outside: bool,
        pocket: bool,
        paths: &[Chord],
        error: f64,
        cancel: Option<&CancelToken>,
    ) -> Result<(), String> {
        let mut budget = Budget { work: 0, cancel };
        budget.use_work(1)?;
        if !radius.is_finite()
            || radius <= error * 2.
            || !inset.is_finite()
            || inset < radius
            || paths.is_empty()
            || paths.len() > MAX_PATHS
        {
            return Err("Finishing cutter/path range exceeds the bounded polygon proof".into());
        }
        let reference = flatten(&self.reference(inset, outside)?)?;
        self.safe(&reference, inset, outside, PLAN_ERROR, &mut budget)?;
        self.safe(paths, inset, outside, error, &mut budget)?;
        cover_lines(&reference, paths, error - 2. * FLAT, &mut budget)?;
        if !pocket {
            cover_lines(paths, &reference, error - 2. * FLAT, &mut budget)?;
        }
        if pocket {
            self.fill(inset, radius + error, paths, &mut budget)?;
        }
        Ok(())
    }
    fn fill(
        &self,
        inset: f64,
        radius: f64,
        paths: &[Chord],
        budget: &mut Budget<'_>,
    ) -> Result<(), String> {
        let mut b = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for p in &self.vertices {
            b[0] = b[0].min(p.x);
            b[1] = b[1].min(p.y);
            b[2] = b[2].max(p.x);
            b[3] = b[3].max(p.y);
        }
        let mut pending = vec![(b, (0..paths.len()).collect::<Vec<_>>())];
        while let Some((b, candidates)) = pending.pop() {
            budget.use_work(self.edges.len() + candidates.len())?;
            let candidates: Vec<_> = candidates
                .into_iter()
                .filter(|&i| {
                    let p = paths[i];
                    p[0].x.min(p[1].x) - radius <= b[2]
                        && p[0].x.max(p[1].x) + radius >= b[0]
                        && p[0].y.min(p[1].y) - radius <= b[3]
                        && p[0].y.max(p[1].y) + radius >= b[1]
                })
                .collect();
            let center = Point2::new((b[0] + b[2]) * 0.5, (b[1] + b[3]) * 0.5);
            let halfdiag = (b[2] - b[0]).hypot(b[3] - b[1]) * 0.5;
            // Signed distance is 1-Lipschitz. The WHOLE tile is outside E.
            if self.clearance(center) + halfdiag < inset - EPS {
                continue;
            }
            let corners = [
                Point2::new(b[0], b[1]),
                Point2::new(b[2], b[1]),
                Point2::new(b[2], b[3]),
                Point2::new(b[0], b[3]),
            ];
            if candidates.iter().any(|&i| {
                corners
                    .iter()
                    .all(|c| distance(*c, paths[i]) <= radius - FLAT - EPS)
            }) {
                continue;
            }
            if halfdiag < 1e-6 || pending.len() > 100_000 {
                return Err(format!("Finishing floor coverage is incomplete near ({:.4}, {:.4}) mm (nearest path {:.6} mm); choose another fill strategy/cutter", center.x, center.y, paths.iter().map(|p|distance(center,*p)).fold(f64::INFINITY,f64::min)));
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

pub(super) fn flatten(segments: &[Segment]) -> Result<Vec<Chord>, String> {
    let mut out = Vec::new();
    for s in segments {
        if ![s.start.x, s.start.y, s.end.x, s.end.y, s.bulge]
            .iter()
            .all(|v| v.is_finite())
            || [s.start.x, s.start.y, s.end.x, s.end.y]
                .iter()
                .any(|v| v.abs() > 1_000_000.)
        {
            return Err("Finishing proof requires finite coordinates within ±1000000 mm".into());
        }
        if out.len() >= MAX_PATHS {
            return Err("Finishing path proof budget exceeded (5000 chords)".into());
        }
        if s.bulge == 0. {
            out.push([s.start, s.end]);
            continue;
        }
        let v = sub(s.end, s.start);
        let length = s.start.distance(s.end);
        if !s.bulge.is_finite() || length == 0. {
            return Err("Invalid finishing arc".into());
        }
        let center = add(
            mul(add(s.start, s.end), 0.5),
            mul(normal(v), (1. - s.bulge * s.bulge) / (4. * s.bulge)),
        );
        let radius = center.distance(s.start);
        let sweep = 4. * s.bulge.atan();
        let count = (sweep.abs() / (2. * (1. - (FLAT / radius).min(1.)).acos()))
            .ceil()
            .max(1.);
        if !count.is_finite() || count > MAX_PATHS as f64 || out.len() + count as usize > MAX_PATHS
        {
            return Err("Finishing arc proof budget exceeded".into());
        }
        let start = (s.start.y - center.y).atan2(s.start.x - center.x);
        let mut from = s.start;
        for i in 1..=count as usize {
            let angle = start + sweep * i as f64 / count;
            let to = if i == count as usize {
                s.end
            } else {
                add(
                    center,
                    Point2::new(radius * angle.cos(), radius * angle.sin()),
                )
            };
            out.push([from, to]);
            from = to;
        }
    }
    if out.len() > MAX_PATHS {
        return Err("Finishing path proof budget exceeded (5000 chords)".into());
    }
    Ok(out)
}
pub(super) fn planned(offsets: &[PolylineOffset]) -> Result<Vec<Chord>, String> {
    let count = offsets
        .iter()
        .try_fold(0usize, |n, p| n.checked_add(p.segments.len()))
        .ok_or("Finishing segment count overflow")?;
    if count > MAX_PATHS {
        return Err("Finishing path proof budget exceeded (5000 segments)".into());
    }
    let segments: Vec<_> = offsets
        .iter()
        .flat_map(|p| p.segments.iter().cloned())
        .collect();
    flatten(&segments)
}

// Exact segment/capsule intersection represented as intervals on the queried
// segment: rectangular tube + two endpoint disks. Closed union, no grid samples.
fn capsule_intervals(a: Chord, b: Chord, r: f64, out: &mut Vec<[f64; 2]>) {
    let d = sub(a[1], a[0]);
    let v = sub(b[1], b[0]);
    let l = b[0].distance(b[1]);
    if l > 0. {
        let u = mul(v, 1. / l);
        let w = sub(a[0], b[0]);
        let mut lo: f64 = 0.;
        let mut hi: f64 = 1.;
        for (base, slope, min, max) in [
            (dot(w, u), dot(d, u), 0., l),
            (cross(w, u), cross(d, u), -r, r),
        ] {
            if slope == 0. {
                if base < min || base > max {
                    hi = -1.;
                    break;
                }
            } else {
                let p = (min - base) / slope;
                let q = (max - base) / slope;
                lo = lo.max(p.min(q));
                hi = hi.min(p.max(q));
            }
        }
        if lo <= hi {
            out.push([lo, hi]);
        }
    }
    for p in b {
        let w = sub(a[0], p);
        let aa = dot(d, d);
        let bb = dot(w, d);
        let cc = dot(w, w) - r * r;
        if aa == 0. {
            if cc <= 0. {
                out.push([0., 1.]);
            }
            continue;
        }
        // Orthogonal-distance form avoids cancellation of two huge quadratic
        // terms when a long edge intersects a sub-millimetre endpoint disk.
        let perpendicular = cross(w, d).abs() / aa.sqrt();
        if perpendicular <= r {
            let half = ((r - perpendicular) * (r + perpendicular) / aa).sqrt();
            let lo = (-bb / aa - half).max(0.);
            let hi = (-bb / aa + half).min(1.);
            if lo <= hi {
                out.push([lo, hi]);
            }
        }
    }
}
fn cover_lines(
    target: &[Chord],
    paths: &[Chord],
    r: f64,
    budget: &mut Budget<'_>,
) -> Result<(), String> {
    for a in target {
        budget.use_work(paths.len())?;
        let mut intervals = Vec::new();
        for b in paths {
            capsule_intervals(*a, *b, r - EPS, &mut intervals);
        }
        intervals.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let mut end = 0.;
        for i in intervals {
            if i[0] > end {
                break;
            }
            end = end.max(i[1]);
        }
        if end < 1. {
            return Err("Finishing wall coverage is incomplete; compensated contour differs from the reachable reference".into());
        }
    }
    Ok(())
}

// See the decoded gate below. Kept separate so a test can compare the bound
// against analytic points rather than against the production chord flattener.
fn decoded_arc_enclosure(a: Point2, b: Point2, bulge: f64, center: Point2) -> Result<f64, String> {
    if bulge == 0. || !bulge.is_finite() {
        return Err("Finishing decoded arc has an ambiguous sweep".into());
    }
    let reconstructed = add(
        mul(add(a, b), 0.5),
        mul(normal(sub(b, a)), (1. - bulge * bulge) / (4. * bulge)),
    );
    let bound = 2. * reconstructed.distance(center) + EPS;
    if !bound.is_finite() {
        return Err("Finishing decoded arc has a nonfinite enclosure".into());
    }
    Ok(bound)
}

/// Final publication gate over the owned decoded program, after actual stage and
/// cutter binding. Uses only the existing response fields; no persisted receipt
/// is accepted as a certificate. Main-only and accepted rectangles are unchanged.
pub(in crate::pipeline) fn verify_output(
    project: &crate::project::Project,
    objects: &[VcObject],
    path: &[crate::gcode::preview::ToolpathSegment],
    stages: &[crate::pipeline::stage_provenance::StageBinding],
    cancel: Option<&CancelToken>,
) -> Result<(), crate::pipeline::PipelineError> {
    use crate::gcode::preview::MoveKind;
    use crate::project::{OpKind, ToolOffset};
    for op in project.operations.iter().filter(|op| {
        op.enabled
            && op
                .params
                .stages
                .as_ref()
                .is_some_and(|s| s.finishing().is_some())
    }) {
        let fail = |e: String| {
            if crate::pipeline::cancelled(cancel) {
                crate::pipeline::PipelineError::Cancelled
            } else {
                crate::pipeline::PipelineError::FinishingProof(op.id, e)
            }
        };
        if op
            .params
            .stages
            .as_ref()
            .is_some_and(|s| s.pocket_scope().is_some())
        {
            pocket::verify_decoded(project, op, objects, path, stages, cancel).map_err(fail)?;
            continue;
        }
        let selected = crate::pipeline::ordered_selection(op, objects);
        if selected.len() != 1 {
            return Err(fail("Finishing requires one selected contour; islands and multiple regions are unsupported".into()));
        }
        let object = &objects[selected[0]];
        if super::stage_coverage::rectangle(object).is_ok() {
            continue;
        }
        let polygon = Polygon::new(object).map_err(fail)?;
        let pocket = matches!(op.kind, OpKind::Pocket { .. });
        let outside = matches!(
            op.kind,
            OpKind::Profile {
                offset: ToolOffset::Outside,
                ..
            }
        );
        let finish = op
            .params
            .stages
            .as_ref()
            .and_then(crate::cutting_stages::MainFinishing::finishing);
        let finish = finish.expect("filtered finishing");
        for binding in stages.iter().filter(|s| s.operation_id == op.id) {
            let main = binding.stage == "main";
            let depth = op.params.depth - op.params.through_depth.max(0.)
                + if main { finish.floor_allowance_mm } else { 0. };
            let tool = project
                .tools
                .iter()
                .find(|t| t.id == binding.parameters.tool_id)
                .ok_or_else(|| fail("Missing actual stage cutter".into()))?;
            let radius = tool.effective_diameter() * 0.5;
            let inset = radius
                + op.params.stock_to_leave_mm
                + if main {
                    finish.lateral_allowance_mm
                } else {
                    0.
                };
            let mut all = Vec::new();
            let mut final_depth = Vec::new();
            let mut seen = std::collections::BTreeSet::new();
            let mut seen_final = std::collections::BTreeSet::new();
            let mut scan_budget = Budget { work: 0, cancel };
            let mut arc_enclosure: f64 = 0.;
            for m in &path[binding.first_segment..binding.end_segment_exclusive] {
                scan_budget.use_work(1).map_err(fail)?;
                if m.kind == MoveKind::Rapid || m.from.z.min(m.to.z) > op.params.start_depth + 1e-4
                {
                    continue;
                }
                if m.from.z.min(m.to.z) < depth - 1e-4 {
                    return Err(fail(
                        "Finishing decoded path exceeds its authored stage depth".into(),
                    ));
                }
                let a = Point2::new(m.from.x, m.from.y);
                let b = Point2::new(m.to.x, m.to.y);
                let s = if let Some(arc) = m.arc {
                    let start = (a.y - arc.cy).atan2(a.x - arc.cx);
                    let end = (b.y - arc.cy).atan2(b.x - arc.cx);
                    let angle = if arc.ccw {
                        (end - start).rem_euclid(std::f64::consts::TAU)
                    } else {
                        -(start - end).rem_euclid(std::f64::consts::TAU)
                    };
                    if (a.distance(Point2::new(arc.cx, arc.cy))
                        - b.distance(Point2::new(arc.cx, arc.cy)))
                    .abs()
                        > 0.0003
                    {
                        return Err(fail("Finishing decoded arc radius is inconsistent".into()));
                    }
                    let bulge = (angle / 4.).tan();
                    // Output rounding can put the last decoded endpoint slightly
                    // off the ArcXY circle. Our bulge primitive then has another
                    // center. Both parameterizations start at `a` and have the
                    // same sweep: their distance is at most 2*|center delta| at
                    // EVERY angle. Deduct that enclosure from the existing
                    // output budget, never silently replace the analytic motion.
                    let enclosure = decoded_arc_enclosure(a, b, bulge, Point2::new(arc.cx, arc.cy))
                        .map_err(fail)?;
                    arc_enclosure = arc_enclosure.max(enclosure);
                    Segment::arc(a, b, bulge, Some(Point2::new(arc.cx, arc.cy)), "decoded", 7)
                } else {
                    Segment::line(a, b, "decoded", 7)
                };
                let key = [
                    s.start.x.to_bits(),
                    s.start.y.to_bits(),
                    s.end.x.to_bits(),
                    s.end.y.to_bits(),
                    s.bulge.to_bits(),
                ];
                if m.from.z <= depth + 1e-4 && m.to.z <= depth + 1e-4 && seen_final.insert(key) {
                    final_depth.push(s.clone());
                }
                if seen.insert(key) {
                    all.push(s);
                }
                if all.len() > MAX_PATHS {
                    return Err(fail("Finishing decoded proof budget exceeded (5000 distinct planar movements per stage)".into()));
                }
            }
            let output_error = OUTPUT_ERROR - arc_enclosure;
            if output_error <= 2. * FLAT + EPS {
                return Err(fail(
                    "Finishing decoded arc uncertainty exhausts the output precision budget".into(),
                ));
            }
            let all = flatten(&all).map_err(fail)?;
            let final_depth = flatten(&final_depth).map_err(fail)?;
            let mut budget = Budget { work: 0, cancel };
            polygon
                .safe(&all, inset, outside, output_error, &mut budget)
                .map_err(fail)?;
            polygon
                .prove(
                    radius,
                    inset,
                    outside,
                    pocket,
                    &final_depth,
                    output_error,
                    cancel,
                )
                .map_err(fail)?;
        }
    }
    Ok(())
}

pub(in crate::pipeline) fn needs_decoded_gate(
    project: &crate::project::Project,
    objects: &[VcObject],
) -> bool {
    project
        .operations
        .iter()
        .filter(|op| {
            op.enabled
                && op
                    .params
                    .stages
                    .as_ref()
                    .is_some_and(|s| s.finishing().is_some())
        })
        .any(|op| {
            let selected = crate::pipeline::ordered_selection(op, objects);
            op.params
                .stages
                .as_ref()
                .is_some_and(|s| s.pocket_scope().is_some())
                || selected.len() != 1
                || super::stage_coverage::rectangle(&objects[selected[0]]).is_err()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn polygon() -> Polygon {
        let points = [
            Point2::new(0., 0.),
            Point2::new(20., 0.),
            Point2::new(24., 12.),
            Point2::new(5., 20.),
        ];
        let segments: Vec<_> = (0..points.len())
            .map(|i| Segment::line(points[i], points[(i + 1) % points.len()], "0", 7))
            .collect();
        Polygon::new(&crate::cam::chaining::segments_to_objects(&segments)[0]).unwrap()
    }
    #[test]
    fn wall_is_not_floor_and_incomplete_or_shifted_paths_refuse() {
        let p = polygon();
        let wall = flatten(&p.reference(1., false).unwrap()).unwrap();
        p.prove(1., 1., false, false, &wall, PLAN_ERROR, None)
            .unwrap();
        let error = p
            .prove(1., 1., false, true, &wall, PLAN_ERROR, None)
            .unwrap_err();
        assert!(error.contains("floor coverage"));
        assert!(p
            .prove(
                1.,
                1.,
                false,
                false,
                &wall[..wall.len() - 1],
                PLAN_ERROR,
                None
            )
            .unwrap_err()
            .contains("wall coverage"));
        let moved: Vec<_> = wall
            .iter()
            .map(|c| {
                [
                    add(c[0], Point2::new(0., -0.04)),
                    add(c[1], Point2::new(0., -0.04)),
                ]
            })
            .collect();
        assert!(p
            .prove(1., 1., false, false, &moved, OUTPUT_ERROR, None)
            .unwrap_err()
            .contains("envelope"));
    }
    #[test]
    fn interval_union_is_continuous_not_endpoint_only() {
        let a = [Point2::new(0., 0.), Point2::new(10., 0.)];
        let b = [
            [Point2::new(0., 0.), Point2::new(4., 0.)],
            [Point2::new(6., 0.), Point2::new(10., 0.)],
        ];
        let mut budget = Budget {
            work: 0,
            cancel: None,
        };
        assert!(cover_lines(&[a], &b, 0.5, &mut budget).is_err());
        cover_lines(&[a], &b, 1.01, &mut budget).unwrap();
    }
    #[test]
    fn cancellation_and_work_ceiling_refuse_without_relaxation() {
        let token = CancelToken::new();
        token.cancel();
        let p = polygon();
        let path = flatten(&p.reference(1., false).unwrap()).unwrap();
        assert!(p
            .prove(1., 1., false, false, &path, PLAN_ERROR, Some(&token))
            .unwrap_err()
            .contains("cancelled"));
        let mut budget = Budget {
            work: MAX_WORK,
            cancel: None,
        };
        assert!(budget.use_work(1).unwrap_err().contains("budget"));
        let mut budget = Budget {
            work: usize::MAX,
            cancel: None,
        };
        assert!(budget.use_work(1).unwrap_err().contains("overflow"));
    }
    #[test]
    fn rounded_arc_endpoint_enclosure_contains_the_entire_analytic_sweep() {
        let a = Point2::new(11., 20.);
        let center = Point2::new(10., 20.);
        let angle: f64 = 0.2;
        let b = Point2::new(10. + 1.0002 * angle.cos(), 20. + 1.0002 * angle.sin());
        let bulge = (angle / 4.).tan();
        let bound = decoded_arc_enclosure(a, b, bulge, center).unwrap();
        assert!(bound > 0.0002 && bound < 0.003);
        let c = add(
            mul(add(a, b), 0.5),
            mul(normal(sub(b, a)), (1. - bulge * bulge) / (4. * bulge)),
        );
        for k in 0..=100 {
            let t = angle * f64::from(k) / 100.;
            let v = sub(a, c);
            let converted = add(
                c,
                Point2::new(v.x * t.cos() - v.y * t.sin(), v.x * t.sin() + v.y * t.cos()),
            );
            let exact = Point2::new(center.x + t.cos(), center.y + t.sin());
            assert!(converted.distance(exact) <= bound);
        }
        assert!(decoded_arc_enclosure(a, b, 0., center).is_err());
        assert!(decoded_arc_enclosure(a, b, f64::NAN, center).is_err());
    }
    #[test]
    fn decoded_gate_rejects_missing_floor_gouge_and_wrong_depth() {
        let request: crate::pipeline::PipelineRequest = serde_json::from_str(include_str!(
            "../../../../../tests/fixtures/alpha31/convex-pocket.request.json"
        ))
        .unwrap();
        let objects = crate::cam::chaining::segments_to_objects(&request.project.segments);
        let response = crate::pipeline::run_pipeline(request.clone(), |_, _, _| {}).unwrap();
        for mode in ["missing", "gouge", "deep"] {
            let mut path = response.toolpath.clone();
            let stage = &response.cutting_stages[1];
            for m in &mut path[stage.first_segment..stage.end_segment_exclusive] {
                if m.from.z < 0. && m.to.z < 0. {
                    match mode {
                        "missing" => {
                            m.from.z = m.from.z.max(-0.4);
                            m.to.z = m.to.z.max(-0.4);
                        }
                        "gouge" => {
                            m.from.x -= 3.;
                            m.to.x -= 3.;
                        }
                        _ => {
                            m.from.z -= 0.1;
                            m.to.z -= 0.1;
                        }
                    }
                }
            }
            assert!(
                verify_output(
                    &request.project,
                    &objects,
                    &path,
                    &response.cutting_stages,
                    None
                )
                .is_err(),
                "{mode}"
            );
        }
    }
    #[test]
    fn exact_source_continuity_topology_and_tiny_edges_are_required() {
        let mut segments = vec![
            Segment::line(Point2::new(0., 0.), Point2::new(20., 0.), "0", 7),
            Segment::line(Point2::new(20., 0.), Point2::new(5., 20.), "0", 7),
            Segment::line(Point2::new(5., 20.), Point2::new(0., 0.000_001), "0", 7),
        ];
        let obj = crate::cam::chaining::segments_to_objects(&segments).remove(0);
        assert!(Polygon::new(&obj).err().unwrap().contains("continuity"));
        segments[2].end = Point2::new(0., 0.);
        segments[0].kind = SegmentKind::Arc;
        let obj = crate::cam::chaining::segments_to_objects(&segments).remove(0);
        assert!(Polygon::new(&obj).err().unwrap().contains("curves"));
    }
}
