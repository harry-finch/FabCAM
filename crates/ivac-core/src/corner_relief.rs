//! Explicit CAD corner relief, in millimetres. No operations, toolpaths or posts.
//! A relief is the union of the removal-side wedge and a cutter-radius disk
//! through its vertex. Exact circular segments replace only the chosen corner.
use crate::{Point2, Segment, SegmentKind};
use cavalier_contours::polyline::{PlineContainsResult, PlineSource, PlineSourceMut, Polyline};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::{PI, TAU};

// Endpoint/topology tolerance, not fit allowance or output rounding.
const EPS: f64 = 1e-7;
const MAX_SEGMENTS: usize = 512;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReliefMode {
    Dogbone,
    TboneIncoming,
    TboneOutgoing,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RemovalSide {
    Interior,
    Exterior,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct CornerRef {
    pub object_id: u32,
    pub corner_index: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CornerReliefRequest {
    pub segments: Vec<Segment>,
    /// One stable object tag per source segment. Zero/unassigned cannot be edited.
    pub objects: Vec<u32>,
    pub selected_objects: Vec<u32>,
    pub selected_corners: Vec<CornerRef>,
    pub mode: ReliefMode,
    pub removal_side: RemovalSide,
    pub tool_diameter_mm: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CornerCandidate {
    pub reference: CornerRef,
    pub vertex: Point2,
    pub center: Option<Point2>,
    pub reason: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CornerReliefResponse {
    pub candidates: Vec<CornerCandidate>,
    pub segments: Vec<Segment>,
    pub objects: Vec<u32>,
    pub error: Option<String>,
}
#[derive(Clone)]
struct Cut {
    before: Point2,
    after: Point2,
    center: Point2,
    arcs: Vec<Segment>,
}
fn sub(a: Point2, b: Point2) -> Point2 {
    Point2::new(a.x - b.x, a.y - b.y)
}
fn dot(a: Point2, b: Point2) -> f64 {
    a.x * b.x + a.y * b.y
}
fn cross(a: Point2, b: Point2) -> f64 {
    a.x * b.y - a.y * b.x
}
fn add_scaled(a: Point2, b: Point2, t: f64) -> Point2 {
    Point2::new(a.x + b.x * t, a.y + b.y * t)
}
fn unit(a: Point2) -> Point2 {
    let d = a.x.hypot(a.y);
    Point2::new(a.x / d, a.y / d)
}
fn pline(s: &[Segment]) -> Polyline<f64> {
    let mut p = Polyline::new_closed();
    for s in s {
        p.add(s.start.x, s.start.y, s.bulge);
    }
    p
}
fn simple(s: &[Segment]) -> bool {
    if s.len() < 2 || !pline(s).area().is_finite() || pline(s).area().abs() < EPS * EPS {
        return false;
    }
    let mut hit = false;
    pline(s).visit_self_intersects(&mut |_| {
        hit = true;
    });
    !hit
}
fn ordered(mut s: Vec<Segment>) -> Result<Vec<Segment>, String> {
    if s.len() < 2 {
        return Err("A closed contour is required".into());
    }
    if s.iter().any(|s| {
        ![s.start.x, s.start.y, s.end.x, s.end.y, s.bulge]
            .iter()
            .all(|v| v.is_finite())
            || s.start.distance(s.end) <= EPS
            || !matches!(
                s.kind,
                SegmentKind::Line | SegmentKind::Arc | SegmentKind::Circle
            )
    }) {
        return Err("Invalid or degenerate contour segments".into());
    }
    let mut out = vec![s.remove(0)];
    while !s.is_empty() {
        let at = out.last().unwrap().end;
        let matches: Vec<usize> = s
            .iter()
            .enumerate()
            .filter(|(_, s)| s.start.distance(at) <= EPS || s.end.distance(at) <= EPS)
            .map(|(i, _)| i)
            .collect();
        if matches.len() != 1 {
            return Err("Open, branched or disconnected contour".into());
        }
        let mut next = s.remove(matches[0]);
        if next.start.distance(at) > EPS {
            std::mem::swap(&mut next.start, &mut next.end);
            next.bulge = -next.bulge;
        }
        next.start = at;
        out.push(next);
    }
    if out.last().unwrap().end.distance(out[0].start) > EPS || !simple(&out) {
        return Err("Open or self-intersecting contour".into());
    }
    Ok(out)
}
fn cut(
    s: &[Segment],
    i: usize,
    r: f64,
    mode: ReliefMode,
    side: RemovalSide,
) -> Result<Cut, String> {
    let prev = &s[(i + s.len() - 1) % s.len()];
    let next = &s[i];
    let v = next.start;
    if prev.kind != SegmentKind::Line || next.kind != SegmentKind::Line {
        return Err("Select a corner between two straight edges".into());
    }
    let u = unit(sub(prev.start, v));
    let w = unit(sub(next.end, v));
    let sign = pline(s).area().signum()
        * match side {
            RemovalSide::Interior => 1.,
            RemovalSide::Exterior => -1.,
        };
    if -cross(u, w) * sign <= 1e-6 {
        return Err("No compatible internal corner on this removal side".into());
    }
    if dot(u, w) > 1e-8 {
        return Err("Acute corner: exact-diameter relief needs an isolated plunge or an enlarged access slot".into());
    }
    let bis = sub(Point2::new(-u.x, -u.y), w);
    if bis.x.hypot(bis.y) < 1e-6 {
        return Err("Nearly straight corner".into());
    }
    let axis = match mode {
        ReliefMode::Dogbone => unit(Point2::new(u.x + w.x, u.y + w.y)),
        ReliefMode::TboneIncoming => u,
        ReliefMode::TboneOutgoing => w,
    };
    let offset = Point2::new(axis.x * r, axis.y * r);
    let center = add_scaled(v, axis, r);
    let a = (2. * dot(offset, u)).max(0.);
    let b = (2. * dot(offset, w)).max(0.);
    if a >= v.distance(prev.start) - EPS || b >= v.distance(next.end) - EPS {
        return Err("Cutter relief exceeds an adjacent edge".into());
    }
    let before = add_scaled(v, u, a);
    let after = add_scaled(v, w, b);
    let a0 = (before.y - center.y).atan2(before.x - center.x);
    let a1 = (after.y - center.y).atan2(after.x - center.x);
    let sweep = if sign > 0. {
        (a1 - a0).rem_euclid(TAU)
    } else {
        -(a0 - a1).rem_euclid(TAU)
    };
    if sweep.abs() < 1e-6 || sweep.abs() > TAU - 1e-6 {
        return Err("Degenerate relief arc".into());
    }
    // Existing consumers expect arcs no larger than a semicircle.
    let count = if sweep.abs() > PI + EPS { 2 } else { 1 };
    let mut arcs = Vec::new();
    let mut start = before;
    for k in 1..=count {
        let angle = a0 + sweep * f64::from(k) / f64::from(count);
        let end = if k == count {
            after
        } else {
            Point2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
        };
        arcs.push(Segment::arc(
            start,
            end,
            (sweep / f64::from(count) / 4.).tan(),
            Some(center),
            next.layer.clone(),
            next.color,
        ));
        start = end;
    }
    Ok(Cut {
        before,
        after,
        center,
        arcs,
    })
}
fn splice(s: &[Segment], cuts: &BTreeMap<usize, Cut>) -> Result<Vec<Segment>, String> {
    let mut out = Vec::new();
    for (i, seg) in s.iter().enumerate() {
        if let Some(c) = cuts.get(&i) {
            out.extend(c.arcs.clone());
        }
        let mut edge = seg.clone();
        if let Some(c) = cuts.get(&i) {
            edge.start = c.after;
        }
        if let Some(c) = cuts.get(&((i + 1) % s.len())) {
            edge.end = c.before;
        }
        if dot(sub(edge.end, edge.start), sub(seg.end, seg.start)) <= EPS * EPS {
            return Err("Selected reliefs overlap on an edge".into());
        }
        out.push(edge);
    }
    if !simple(&out) {
        return Err("Relief would intersect another edge".into());
    }
    Ok(out)
}
fn topology(before: &BTreeMap<u32, Vec<Segment>>, after: &BTreeMap<u32, Vec<Segment>>) -> bool {
    let ids: Vec<_> = before.keys().copied().collect();
    for (i, a) in ids.iter().enumerate() {
        for b in &ids[i + 1..] {
            let old = pline(&before[a]).contains(&pline(&before[b]));
            if matches!(
                old,
                PlineContainsResult::Intersected | PlineContainsResult::InvalidInput
            ) || old != pline(&after[a]).contains(&pline(&after[b]))
            {
                return false;
            }
        }
    }
    true
}
/// Preview is pure and atomic. On failure no modified geometry is returned.
#[must_use]
pub fn preview(req: &CornerReliefRequest) -> CornerReliefResponse {
    let mut result = CornerReliefResponse {
        candidates: Vec::new(),
        segments: req.segments.clone(),
        objects: req.objects.clone(),
        error: None,
    };
    let fail = |mut out: CornerReliefResponse, msg: &str| {
        out.error = Some(msg.into());
        out
    };
    if !req.tool_diameter_mm.is_finite() || req.tool_diameter_mm <= EPS * 100. {
        return fail(result, "A finite positive cutter diameter is required");
    }
    if req.segments.len() != req.objects.len() || req.segments.len() > MAX_SEGMENTS {
        return fail(
            result,
            "Invalid geometry tags or geometry exceeds CAD1 budget",
        );
    }
    let mut grouped: BTreeMap<u32, Vec<Segment>> = BTreeMap::new();
    for (s, id) in req.segments.iter().zip(&req.objects) {
        grouped.entry(*id).or_default().push(s.clone());
    }
    let mut contours = BTreeMap::new();
    for (id, s) in &grouped {
        match ordered(s.clone()) {
            Ok(s) => {
                contours.insert(*id, s);
            }
            Err(reason) => {
                return fail(result, &format!("Object {id}: {reason}"));
            }
        }
    }
    let selected: BTreeSet<_> = req.selected_objects.iter().copied().collect();
    if selected.contains(&0) || selected.iter().any(|id| !contours.contains_key(id)) {
        return fail(result, "Select assigned closed contours");
    }
    let mut valid: BTreeMap<CornerRef, Cut> = BTreeMap::new();
    for id in &selected {
        let s = &contours[id];
        for i in 0..s.len() {
            let reference = CornerRef {
                object_id: *id,
                corner_index: i,
            };
            let built =
                cut(s, i, req.tool_diameter_mm / 2., req.mode, req.removal_side).and_then(|c| {
                    let replaced = splice(s, &BTreeMap::from([(i, c.clone())]))?;
                    let mut after = contours.clone();
                    after.insert(*id, replaced);
                    if !topology(&contours, &after) {
                        return Err("Relief intersects or changes another contour/island".into());
                    }
                    Ok(c)
                });
            let (center, reason) = match built {
                Ok(c) => {
                    let center = c.center;
                    valid.insert(reference.clone(), c);
                    (Some(center), None)
                }
                Err(e) => (None, Some(e)),
            };
            result.candidates.push(CornerCandidate {
                reference,
                vertex: s[i].start,
                center,
                reason,
            });
        }
    }
    let unique: BTreeSet<_> = req.selected_corners.iter().cloned().collect();
    if unique.len() != req.selected_corners.len() {
        return fail(result, "Duplicate corner selection");
    }
    let mut cuts: BTreeMap<u32, BTreeMap<usize, Cut>> = BTreeMap::new();
    for key in &req.selected_corners {
        let Some(c) = valid.get(key) else {
            return fail(
                result,
                "Selected corner is unavailable; review the selection",
            );
        };
        cuts.entry(key.object_id)
            .or_default()
            .insert(key.corner_index, c.clone());
    }
    let mut after = contours.clone();
    for (id, cuts) in &cuts {
        match splice(&contours[id], cuts) {
            Ok(s) => {
                after.insert(*id, s);
            }
            Err(e) => return fail(result, &e),
        }
    }
    if !topology(&contours, &after) {
        return fail(result, "Selected reliefs collide with another contour");
    }
    // Keep unmodified source ordering and object tag cardinality; only replace
    // edited objects at their first occurrence, without introducing new IDs.
    let mut seen = BTreeSet::new();
    let mut segments = Vec::new();
    let mut objects = Vec::new();
    for (s, id) in req.segments.iter().zip(&req.objects) {
        if cuts.contains_key(id) {
            if seen.insert(*id) {
                for s in &after[id] {
                    segments.push(s.clone());
                    objects.push(*id);
                }
            }
        } else {
            segments.push(s.clone());
            objects.push(*id);
        }
    }
    result.segments = segments;
    result.objects = objects;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn polygon(points: &[(f64, f64)]) -> CornerReliefRequest {
        let segments = points
            .iter()
            .enumerate()
            .map(|(i, &(x, y))| {
                let next = points[(i + 1) % points.len()];
                Segment::line(Point2::new(x, y), Point2::new(next.0, next.1), "0", 7)
            })
            .collect();
        CornerReliefRequest {
            segments,
            objects: vec![1; points.len()],
            selected_objects: vec![1],
            selected_corners: vec![CornerRef {
                object_id: 1,
                corner_index: 0,
            }],
            mode: ReliefMode::Dogbone,
            removal_side: RemovalSide::Interior,
            tool_diameter_mm: 6.,
        }
    }
    fn square() -> CornerReliefRequest {
        polygon(&[(0., 0.), (40., 0.), (40., 40.), (0., 40.)])
    }
    #[test]
    fn right_angle_exact_disk_and_shared_corner() {
        for mode in [
            ReliefMode::Dogbone,
            ReliefMode::TboneIncoming,
            ReliefMode::TboneOutgoing,
        ] {
            for d in [2., 6., 6.35, 12.] {
                let mut q = square();
                q.mode = mode;
                q.tool_diameter_mm = d;
                let out = preview(&q);
                assert!(out.error.is_none(), "{:?}", out.error);
                let c = out.candidates[0].center.unwrap();
                let r = d / 2.;
                assert!((c.distance(Point2::new(0., 0.)) - r).abs() < 1e-10);
                match mode {
                    ReliefMode::Dogbone => {
                        assert!((c.x - r / 2_f64.sqrt()).abs() < 1e-10);
                        assert!((c.y - c.x).abs() < 1e-10);
                    }
                    ReliefMode::TboneIncoming => assert!((c.y - r).abs() < 1e-10),
                    ReliefMode::TboneOutgoing => assert!((c.x - r).abs() < 1e-10),
                }
                for arc in out.segments.iter().filter(|s| s.kind == SegmentKind::Arc) {
                    let (_, _, _, actual) =
                        crate::math::bulge_to_arc(arc.start, arc.end, arc.bulge);
                    assert!((actual - r).abs() < 1e-9);
                }
            }
        }
    }
    #[test]
    fn nonorthogonal_corners_are_exact_and_reachable_by_normal_offset() {
        for angle in [30_f64, 60., 89., 90., 110., 135., 160.] {
            for mode in [
                ReliefMode::Dogbone,
                ReliefMode::TboneIncoming,
                ReliefMode::TboneOutgoing,
            ] {
                let a = angle.to_radians();
                let mut q = polygon(&[(0., 0.), (50., 0.), (50. * a.cos(), 50. * a.sin())]);
                q.mode = mode;
                let out = preview(&q);
                if angle < 90. {
                    assert!(out.error.is_some());
                    assert!(out.candidates[0].reason.as_ref().unwrap().contains("Acute"));
                    continue;
                }
                assert!(out.error.is_none(), "{angle} {mode:?}: {:?}", out.error);
                let c = out.candidates[0].center.unwrap();
                assert!((c.distance(Point2::new(0., 0.)) - 3.).abs() < 1e-9);
                let offsets = crate::cam::offsets::parallel_offset_inward(
                    &crate::cam::VcObject::new(out.segments, true),
                    3.,
                );
                let min = offsets
                    .iter()
                    .flat_map(|o| &o.segments)
                    .map(|s| s.start.distance(c).min(s.end.distance(c)))
                    .fold(f64::INFINITY, f64::min);
                assert!(min < 1e-5, "{angle} {mode:?}: center missing by {min}");
            }
        }
    }
    #[test]
    fn batch_and_reversed_winding() {
        let mut q = square();
        q.selected_corners = (0..4)
            .map(|i| CornerRef {
                object_id: 1,
                corner_index: i,
            })
            .collect();
        let a = preview(&q);
        assert!(a.error.is_none());
        assert_eq!(a.segments.len(), 8);
        q.segments.reverse();
        for s in &mut q.segments {
            std::mem::swap(&mut s.start, &mut s.end);
        }
        let b = preview(&q);
        assert!(b.error.is_none());
        assert!((pline(&a.segments).area() + pline(&b.segments).area()).abs() < 1e-8);
    }
    #[test]
    fn unrelated_arc_and_nested_island_are_preserved() {
        let mut q = square();
        q.segments[2] = Segment::arc(
            Point2::new(40., 40.),
            Point2::new(0., 40.),
            1.,
            Some(Point2::new(20., 40.)),
            "0",
            7,
        );
        let arc = q.segments[2].clone();
        let island = polygon(&[(15., 15.), (20., 15.), (20., 20.), (15., 20.)]);
        q.segments.extend(island.segments.clone());
        q.objects.extend([2; 4]);
        let result = preview(&q);
        assert!(result.error.is_none(), "{:?}", result.error);
        assert!(result.segments.contains(&arc));
        let retained: Vec<_> = result
            .segments
            .iter()
            .zip(&result.objects)
            .filter(|(_, id)| **id == 2)
            .map(|(s, _)| s.clone())
            .collect();
        assert_eq!(retained, island.segments);
        assert_eq!(
            result.objects.iter().copied().collect::<BTreeSet<_>>(),
            BTreeSet::from([1, 2])
        );
    }
    #[test]
    fn translation_rotation_and_mirror_preserve_radius_and_area() {
        let base = square();
        let original = preview(&base);
        for mirror in [false, true] {
            let mut q = base.clone();
            if mirror {
                crate::cam::mirror_segments_x(&mut q.segments, Point2::new(0., 0.));
            }
            crate::cam::rotate_segments(&mut q.segments, Point2::new(0., 0.), 0.73);
            crate::cam::translate_segments(&mut q.segments, 17., -29.);
            let result = preview(&q);
            assert!(result.error.is_none());
            assert!(
                (pline(&result.segments).area().abs() - pline(&original.segments).area().abs())
                    .abs()
                    < 1e-8
            );
            assert!(
                (result.candidates[0]
                    .center
                    .unwrap()
                    .distance(result.candidates[0].vertex)
                    - 3.)
                    .abs()
                    < 1e-10
            );
        }
    }
    #[test]
    fn invalid_and_conflicting_inputs_are_atomic() {
        for d in [0., -1., f64::NAN, f64::INFINITY, 100.] {
            let mut q = square();
            q.tool_diameter_mm = d;
            let a = preview(&q);
            assert!(a.error.is_some());
            assert_eq!(a.segments, q.segments);
        }
        let mut q = polygon(&[(0., 0.), (6., 0.), (6., 20.), (0., 20.)]);
        q.selected_corners.push(CornerRef {
            object_id: 1,
            corner_index: 1,
        });
        let out = preview(&q);
        assert!(out.error.is_some());
        assert_eq!(out.segments, q.segments);
        let q = polygon(&[(0., 0.), (20., 20.), (0., 20.), (20., 0.)]);
        assert!(preview(&q).error.is_some());
    }
    #[test]
    fn concave_exterior_corner_and_neighbor_refusal() {
        let mut q = polygon(&[
            (0., 0.),
            (30., 0.),
            (30., 10.),
            (10., 10.),
            (10., 30.),
            (0., 30.),
        ]);
        q.removal_side = RemovalSide::Exterior;
        q.selected_corners[0].corner_index = 3;
        assert!(preview(&q).error.is_none());
        let mut q = square();
        let small = polygon(&[(-1., 1.), (1., 1.), (1., 3.), (-1., 3.)]);
        q.objects.extend(vec![2; 4]);
        q.segments.extend(small.segments);
        assert!(preview(&q).error.is_some());
    }
}
