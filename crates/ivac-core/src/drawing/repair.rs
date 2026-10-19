//! Bounded, explicit line-chain repairs. Every preview evaluates a cloned batch.
use super::{Binding, Contour, Drawing, Edge, Identity, Transition};
use crate::geometry::{Point2, Segment, SegmentKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Repair {
    RemoveZeroLine {
        contour: Identity,
        edge: Identity,
    },
    /// Move the tail endpoint to the next start, or append a connector.
    Gap {
        contour: Identity,
        edge: Identity,
        connector: bool,
        tolerance_mm: f64,
    },
    /// Endpoint orientation is explicit; reverse whole chains if requested.
    Join {
        a: Identity,
        b: Identity,
        reverse_a: bool,
        reverse_b: bool,
        connector: bool,
        tolerance_mm: f64,
    },
    RemoveRepeatedLoop {
        contour: Identity,
    },
    RemoveDuplicate {
        remove: Identity,
        keep: Identity,
    },
    /// One original interior vertex; previously edited edges are protected.
    Simplify {
        contour: Identity,
        edge: Identity,
        deviation_mm: f64,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Finding {
    pub contour: Identity,
    pub message: String,
    pub repair: Option<Repair>,
}
fn reverse(c: &mut Contour) {
    c.edges.reverse();
    for e in &mut c.edges {
        std::mem::swap(&mut e.geometry.start, &mut e.geometry.end);
        e.geometry.bulge = -e.geometry.bulge;
    }
}
fn straight(c: &Contour) -> bool {
    c.edges
        .iter()
        .all(|e| e.geometry.kind == SegmentKind::Line && e.geometry.bulge == 0.)
}
fn connected(c: &Contour) -> bool {
    c.edges
        .windows(2)
        .all(|p| p[0].geometry.end == p[1].geometry.start)
}
fn tolerance(n: f64) -> Result<(), String> {
    if !n.is_finite() || n < 0. {
        Err("Tolerance must be finite and nonnegative mm".into())
    } else {
        Ok(())
    }
}
fn same(a: &Contour, b: &Contour) -> bool {
    if a.id.import != b.id.import || a.edges.len() != b.edges.len() {
        return false;
    }
    let exact = |a: &Contour, b: &Contour| {
        a.edges
            .iter()
            .zip(&b.edges)
            .all(|(a, b)| a.geometry == b.geometry)
    };
    let mut reverse_b = b.clone();
    reverse(&mut reverse_b);
    exact(a, b) || exact(a, &reverse_b)
}
fn repeated_loop(c: &Contour) -> bool {
    let half = c.edges.len() / 2;
    if half < 2 || c.edges.len() % 2 != 0 || !straight(c) || !connected(c) {
        return false;
    }
    let a = Contour {
        edges: c.edges[..half].to_vec(),
        ..c.clone()
    };
    let b = Contour {
        edges: c.edges[half..].to_vec(),
        ..c.clone()
    };
    a.edges[0].geometry.start == a.edges.last().unwrap().geometry.end && same(&a, &b)
}
impl Drawing {
    pub fn analyze(&self, join_mm: f64) -> Result<Vec<Finding>, String> {
        self.validate()?;
        tolerance(join_mm)?;
        if self.contours.iter().map(|c| c.edges.len()).sum::<usize>() > 2000 {
            return Err("Analysis budget exceeded (2000 segments); analysis incomplete".into());
        }
        let mut out = vec![];
        for c in &self.contours {
            let source = self
                .imports
                .iter()
                .find(|s| s.id == c.id.import)
                .ok_or("Missing import")?;
            if !straight(c) {
                out.push(Finding {
                    contour: c.id,
                    message: "Points/curves retained; automatic repair not analyzed".into(),
                    repair: None,
                });
                continue;
            }
            if repeated_loop(c) {
                out.push(Finding { contour:c.id, message:"Exact repeated closed loop; existing explicit selection will require review".into(), repair:Some(Repair::RemoveRepeatedLoop{contour:c.id}) });
            }
            for (i, e) in c.edges.iter().enumerate() {
                let next = &c.edges[(i + 1) % c.edges.len()];
                let gap = source
                    .transform
                    .distance_mm(e.geometry.end, next.geometry.start);
                if gap > 0. {
                    out.push(Finding {
                        contour: c.id,
                        message: format!(
                            "Endpoint gap {} mm{} — endpoints remain distinct until explicitly repaired",
                            if gap < 1e-9 { format!("{gap:.3e}") } else { format!("{gap:.9}") },
                            if i + 1 == c.edges.len() {
                                " (open contour)"
                            } else {
                                ""
                            }
                        ),
                        repair: if gap <= join_mm && c.edges.len() > 1 {
                            Some(Repair::Gap {
                                contour: c.id,
                                edge: e.id,
                                connector: false,
                                tolerance_mm: join_mm,
                            })
                        } else {
                            None
                        },
                    });
                }
                if e.geometry.start == e.geometry.end {
                    out.push(Finding {
                        contour: c.id,
                        message: "Zero-length line; review intent".into(),
                        repair: Some(Repair::RemoveZeroLine {
                            contour: c.id,
                            edge: e.id,
                        }),
                    });
                }
            }
        }
        for (i, a) in self.contours.iter().enumerate() {
            for b in self.contours.iter().skip(i + 1) {
                if straight(a) && same(a, b) {
                    out.push(Finding {
                        contour: b.id,
                        message: "Exact duplicate line path (same import/layer)".into(),
                        repair: Some(Repair::RemoveDuplicate {
                            remove: b.id,
                            keep: a.id,
                        }),
                    });
                }
            }
        }
        // Unique compatible open endpoints only, using actual source-space
        // deltas converted to drawing mm. No nearest-neighbor intent guessing.
        let mut endpoints = Vec::new();
        for c in &self.contours {
            if !straight(c)
                || !connected(c)
                || c.edges[0].geometry.start == c.edges.last().unwrap().geometry.end
            {
                continue;
            }
            endpoints.push((
                c.id,
                true,
                c.edges[0].geometry.start,
                c.edges[0].geometry.layer.clone(),
            ));
            endpoints.push((
                c.id,
                false,
                c.edges.last().unwrap().geometry.end,
                c.edges[0].geometry.layer.clone(),
            ));
        }
        let mut pairs = Vec::new();
        let mut counts = vec![0usize; endpoints.len()];
        for i in 0..endpoints.len() {
            for j in i + 1..endpoints.len() {
                let a = &endpoints[i];
                let b = &endpoints[j];
                if a.0 == b.0 || a.0.import != b.0.import || a.3 != b.3 {
                    continue;
                }
                if self.join_distance(a.0, a.2, b.2, join_mm).is_ok() {
                    pairs.push((i, j));
                    counts[i] += 1;
                    counts[j] += 1;
                }
            }
        }
        for (i, j) in pairs {
            let a = &endpoints[i];
            let b = &endpoints[j];
            let unique = counts[i] == 1 && counts[j] == 1;
            out.push(Finding {
                contour: a.0,
                message: if unique {
                    format!("Join open contours {} and {}", a.0.entity, b.0.entity)
                } else {
                    "Competing endpoint candidates — review required".into()
                },
                repair: unique.then_some(Repair::Join {
                    a: a.0,
                    b: b.0,
                    reverse_a: a.1,
                    reverse_b: !b.1,
                    connector: true,
                    tolerance_mm: join_mm,
                }),
            });
        }
        if let Err(message) = super::topology::validate(self) {
            if let Some(c) = self.contours.first() {
                out.push(Finding {
                    contour: c.id,
                    message: format!("Topology: {message}"),
                    repair: None,
                });
            }
        }
        Ok(out)
    }
    /// Candidates are checked by the same acceptance path as an explicit request.
    pub fn simplification_findings(&self, mm: f64) -> Result<Vec<Finding>, String> {
        tolerance(mm)?;
        let mut out = Vec::new();
        if self.contours.iter().map(|c| c.edges.len()).sum::<usize>() > 200 {
            if let Some(c) = self.contours.first() {
                out.push(Finding {
                    contour: c.id,
                    message: "Simplification not analyzed: 200-segment work budget exceeded".into(),
                    repair: None,
                });
            }
            return Ok(out);
        }
        for c in &self.contours {
            for edge in c.edges.iter().take(c.edges.len().saturating_sub(1)) {
                let repair = Repair::Simplify {
                    contour: c.id,
                    edge: edge.id,
                    deviation_mm: mm,
                };
                if self
                    .preview_repairs(std::slice::from_ref(&repair), &[])
                    .is_ok()
                {
                    out.push(Finding { contour: c.id, message: format!("Simplify original vertex (maximum deviation {mm} mm; one per contour per batch)"), repair: Some(repair) });
                    break;
                }
            }
        }
        Ok(out)
    }
    pub fn preview_repairs(
        &self,
        repairs: &[Repair],
        bindings: &[Binding],
    ) -> Result<(Self, Vec<Binding>), String> {
        self.validate()?;
        let mut next = self.clone();
        // Repeated simplifications on the same contour need a cumulative checker;
        // do not pretend per-step error budgets are a batch-wide proof.
        let mut simplified = std::collections::BTreeSet::new();
        for repair in repairs {
            if let Repair::Simplify { contour, .. } = repair {
                if !simplified.insert(*contour) {
                    return Err("Only one simplification per contour per batch is qualified".into());
                }
            }
            let before_step = matches!(repair, Repair::Simplify { .. }).then(|| next.clone());
            next.apply_repair(repair)?;
            if let Some(before_step) = before_step {
                super::topology::preserve_containment(&before_step, &next)?;
            }
        }
        next.validate()?;
        super::topology::validate(&next)?;
        next.evaluate()?;
        Ok((
            next.clone(),
            bindings.iter().map(|b| next.preserve_binding(b)).collect(),
        ))
    }
    fn allocate(&mut self, import: u32) -> Result<Identity, String> {
        let s = self
            .imports
            .iter_mut()
            .find(|s| s.id == import)
            .ok_or("Missing import")?;
        let id = Identity {
            import,
            entity: s.next_entity,
        };
        s.next_entity = s
            .next_entity
            .checked_add(1)
            .ok_or("Identity capacity exceeded")?;
        Ok(id)
    }
    fn contour_index(&self, id: Identity) -> Result<usize, String> {
        self.contours
            .iter()
            .position(|c| c.id == id)
            .ok_or_else(|| "Contour changed — analyze again".into())
    }
    fn join_distance(&self, id: Identity, a: Point2, b: Point2, mm: f64) -> Result<(), String> {
        tolerance(mm)?;
        let s = self
            .imports
            .iter()
            .find(|s| s.id == id.import)
            .ok_or("Missing import")?;
        let d = s.transform.distance_mm(a, b);
        if !d.is_finite() || d > mm {
            return Err("Gap exceeds drawing-space join tolerance".into());
        }
        Ok(())
    }
    fn apply_repair(&mut self, repair: &Repair) -> Result<(), String> {
        match repair {
            Repair::RemoveZeroLine { contour, edge } => {
                let ci = self.contour_index(*contour)?;
                let ei = self.contours[ci]
                    .edges
                    .iter()
                    .position(|e| e.id == *edge)
                    .ok_or("Edge changed")?;
                let g = &self.contours[ci].edges[ei].geometry;
                if g.kind != SegmentKind::Line || g.start != g.end || g.bulge != 0. {
                    return Err(
                        "Only an exact zero-length LINE can be removed; POINTs are preserved"
                            .into(),
                    );
                }
                self.contours[ci].edges.remove(ei);
                if self.contours[ci].edges.is_empty() {
                    self.contours.remove(ci);
                    self.transitions.push(Transition {
                        parents: vec![*contour],
                        children: vec![],
                    });
                }
            }
            Repair::Gap {
                contour,
                edge,
                connector,
                tolerance_mm,
            } => {
                let ci = self.contour_index(*contour)?;
                let c = &self.contours[ci];
                if !straight(c) || c.edges.len() < 2 {
                    return Err("Gap repair requires a line chain".into());
                }
                let i = c
                    .edges
                    .iter()
                    .position(|e| e.id == *edge)
                    .ok_or("Edge changed")?;
                let a = c.edges[i].geometry.end;
                let b = c.edges[(i + 1) % c.edges.len()].geometry.start;
                self.join_distance(*contour, a, b, *tolerance_mm)?;
                if a == b {
                    return Err("Endpoints already coincide".into());
                }
                if !connector && self.contours[ci].edges[i].geometry.start == b {
                    return Err("Snapping this gap would collapse the edge; use a connector or review the micro-edge".into());
                }
                // Competing endpoint candidates are not resolved by nearest-distance guessing.
                let others = self
                    .contours
                    .iter()
                    .filter(|c| c.id.import == contour.import)
                    .flat_map(|c| c.edges.iter())
                    .filter(|e| {
                        e.id != *edge
                            && e.id
                                != self.contours[ci].edges[(i + 1) % self.contours[ci].edges.len()]
                                    .id
                    });
                for e in others {
                    if self
                        .join_distance(*contour, a, e.geometry.start, *tolerance_mm)
                        .is_ok()
                        || self
                            .join_distance(*contour, a, e.geometry.end, *tolerance_mm)
                            .is_ok()
                    {
                        return Err("Competing endpoint — explicit geometry review required".into());
                    }
                }
                if *connector {
                    let id = self.allocate(contour.import)?;
                    let layer = self.contours[ci].edges[i].geometry.layer.clone();
                    let color = self.contours[ci].edges[i].geometry.color;
                    self.contours[ci].edges.insert(
                        i + 1,
                        Edge {
                            id,
                            original_indices: vec![],
                            geometry: Segment::line(a, b, layer, color),
                        },
                    );
                } else {
                    self.contours[ci].edges[i].geometry.end = b;
                }
            }
            Repair::Join {
                a,
                b,
                reverse_a,
                reverse_b,
                connector,
                tolerance_mm,
            } => {
                if a == b || a.import != b.import {
                    return Err("Join requires two contours in the same import".into());
                }
                let ai = self.contour_index(*a)?;
                let bi = self.contour_index(*b)?;
                let mut ca = self.contours[ai].clone();
                let mut cb = self.contours[bi].clone();
                if !straight(&ca) || !straight(&cb) || !connected(&ca) || !connected(&cb) {
                    return Err("Join requires continuous line chains".into());
                }
                if *reverse_a {
                    reverse(&mut ca);
                }
                if *reverse_b {
                    reverse(&mut cb);
                }
                let last = ca.edges.last().unwrap().geometry.clone();
                let first = cb.edges[0].geometry.clone();
                if last.layer != first.layer {
                    return Err("Cross-layer join is not qualified".into());
                }
                if ca.edges[0].geometry.start == last.end
                    || first.start == cb.edges.last().unwrap().geometry.end
                {
                    return Err("Closed contours cannot be joined as open paths".into());
                }
                self.join_distance(*a, last.end, first.start, *tolerance_mm)?;
                // Recheck ambiguity on the current batch state, not just at analysis time.
                for other in &self.contours {
                    if other.id == *a || other.id == *b || other.id.import != a.import {
                        continue;
                    }
                    for edge in &other.edges {
                        if edge.geometry.layer != last.layer {
                            continue;
                        }
                        for point in [edge.geometry.start, edge.geometry.end] {
                            if self
                                .join_distance(*a, last.end, point, *tolerance_mm)
                                .is_ok()
                                || self
                                    .join_distance(*a, first.start, point, *tolerance_mm)
                                    .is_ok()
                            {
                                return Err(
                                    "Competing endpoint — join requires explicit review".into()
                                );
                            }
                        }
                    }
                }
                if *connector && last.end != first.start {
                    let id = self.allocate(a.import)?;
                    ca.edges.push(Edge {
                        id,
                        original_indices: vec![],
                        geometry: Segment::line(last.end, first.start, last.layer, last.color),
                    });
                } else {
                    ca.edges.last_mut().unwrap().geometry.end = first.start;
                }
                ca.edges.extend(cb.edges);
                let id = self.allocate(a.import)?;
                let parents = vec![*a, *b];
                ca.id = id;
                ca.parents.clone_from(&parents);
                self.contours.retain(|c| c.id != *a && c.id != *b);
                self.contours.insert(ai.min(bi), ca);
                self.transitions.push(Transition {
                    parents,
                    children: vec![id],
                });
            }
            Repair::RemoveRepeatedLoop { contour } => {
                let ci = self.contour_index(*contour)?;
                if !repeated_loop(&self.contours[ci]) {
                    return Err("Contour is not an exact repeated closed loop".into());
                }
                let id = self.allocate(contour.import)?;
                let c = &mut self.contours[ci];
                c.edges.truncate(c.edges.len() / 2);
                c.id = id;
                c.parents = vec![*contour];
                self.transitions.push(Transition {
                    parents: vec![*contour],
                    children: vec![id],
                });
            }
            Repair::RemoveDuplicate { remove, keep } => {
                if remove == keep {
                    return Err("Choose a distinct duplicate survivor".into());
                }
                let i = self.contour_index(*remove)?;
                let k = self.contour_index(*keep)?;
                if !straight(&self.contours[i]) || !same(&self.contours[i], &self.contours[k]) {
                    return Err("Paths are not exact same-import/layer duplicates".into());
                }
                self.contours.remove(i);
                self.transitions.push(Transition {
                    parents: vec![*remove],
                    children: vec![],
                });
            }
            Repair::Simplify {
                contour,
                edge,
                deviation_mm,
            } => {
                tolerance(*deviation_mm)?;
                let ci = self.contour_index(*contour)?;
                let c = &self.contours[ci];
                if !straight(c) || !connected(c) {
                    return Err("Simplification requires a continuous line chain".into());
                }
                let i = c
                    .edges
                    .iter()
                    .position(|e| e.id == *edge)
                    .ok_or("Edge changed")?;
                if i + 1 >= c.edges.len() {
                    return Err("Protected endpoint".into());
                }
                let source = self
                    .imports
                    .iter()
                    .find(|s| s.id == contour.import)
                    .unwrap();
                for e in &c.edges[i..=i + 1] {
                    if e.original_indices.len() != 1 {
                        return Err("Previously simplified or added edges are protected".into());
                    }
                    let original = &source.original[e.original_indices[0]];
                    let mut reversed = original.clone();
                    std::mem::swap(&mut reversed.start, &mut reversed.end);
                    reversed.bulge = -reversed.bulge;
                    if e.geometry != *original && e.geometry != reversed {
                        return Err(
                            "Previously modified edges are protected from simplification".into(),
                        );
                    }
                }
                if c.edges[i].geometry.layer != c.edges[i + 1].geometry.layer
                    || c.edges[i].geometry.color != c.edges[i + 1].geometry.color
                {
                    return Err("Simplification cannot merge different primitive semantics".into());
                }
                let a = c.edges[i].geometry.start;
                let p = c.edges[i].geometry.end;
                let b = c.edges[i + 1].geometry.end;
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let l = dx.hypot(dy);
                if l == 0. || !l.is_finite() {
                    return Err("Degenerate simplification chord".into());
                }
                let projection = ((p.x - a.x) * dx + (p.y - a.y) * dy) / (l * l);
                let deviation = ((p.x - a.x) * dy - (p.y - a.y) * dx).abs() / l;
                let scale = self
                    .imports
                    .iter()
                    .find(|s| s.id == contour.import)
                    .unwrap()
                    .transform
                    .scale
                    .abs();
                let u = Point2::new(p.x - a.x, p.y - a.y);
                let v = Point2::new(b.x - p.x, b.y - p.y);
                let cosine = (u.x * v.x + u.y * v.y) / (u.x.hypot(u.y) * v.x.hypot(v.y));
                let rounding_mm = if super::topology::orient(a, p, b)? == 0 {
                    0.
                } else {
                    64. * f64::EPSILON
                        * [
                            a.x.abs(),
                            a.y.abs(),
                            p.x.abs(),
                            p.y.abs(),
                            b.x.abs(),
                            b.y.abs(),
                            l,
                        ]
                        .into_iter()
                        .fold(1., f64::max)
                        * scale
                };
                // Projection covers the replacement chord, proving both Hausdorff
                // directions; protect turns greater than five degrees.
                if !(0. ..=1.).contains(&projection)
                    || !deviation.is_finite()
                    || !rounding_mm.is_finite()
                    || deviation * scale + rounding_mm > *deviation_mm
                    || !cosine.is_finite()
                    || cosine - 64. * f64::EPSILON < 5f64.to_radians().cos()
                {
                    return Err("Deviation or protected-corner bound exceeded".into());
                }
                let second = self.contours[ci].edges.remove(i + 1);
                let first = &mut self.contours[ci].edges[i];
                first.geometry.end = b;
                first.original_indices.extend(second.original_indices);
            }
        }
        // Whole-batch topology validation follows; endpoint distance is not a proof.
        Ok(())
    }
}
