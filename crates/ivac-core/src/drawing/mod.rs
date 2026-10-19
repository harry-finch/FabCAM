//! GEOM1 source-preserving geometry contracts. No machine or process rules.
//! Coordinates are local mm; tolerance values are drawing-space mm.
pub mod api;
pub mod repair;
mod topology;

use std::collections::{BTreeMap, BTreeSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cam::chaining::segments_to_objects_indexed;
use crate::geometry::{BBox, Point2, Segment};

pub const CONTRACT: u32 = 1;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct Identity {
    pub import: u32,
    pub entity: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Transform {
    pub translate: Point2,
    pub rotation_deg: f64,
    pub scale: f64,
    pub mirror_x: bool,
    pub mirror_y: bool,
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            translate: Point2::new(0., 0.),
            rotation_deg: 0.,
            scale: 1.,
            mirror_x: false,
            mirror_y: false,
        }
    }
}
impl Transform {
    pub fn validate(&self) -> Result<(), String> {
        if ![
            self.translate.x,
            self.translate.y,
            self.rotation_deg,
            self.scale,
        ]
        .iter()
        .all(|v| v.is_finite())
            || self.scale == 0.
        {
            return Err("Nonfinite or singular drawing transform".into());
        }
        Ok(())
    }
    #[must_use]
    pub fn point(&self, p: Point2, pivot: Point2) -> Point2 {
        let mut x = (p.x - pivot.x) * self.scale;
        let mut y = (p.y - pivot.y) * self.scale;
        if self.mirror_x {
            y = -y;
        }
        if self.mirror_y {
            x = -x;
        }
        let (sin, cos) = self.rotation_deg.to_radians().sin_cos();
        Point2::new(
            pivot.x + x * cos - y * sin + self.translate.x,
            pivot.y + x * sin + y * cos + self.translate.y,
        )
    }
    #[must_use]
    pub fn distance_mm(&self, a: Point2, b: Point2) -> f64 {
        a.distance(b) * self.scale.abs()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub id: u32,
    pub original: Vec<Segment>,
    #[serde(default)]
    pub legacy_objects: Vec<u32>,
    /// Immutable original bbox center, not edited bounds.
    pub pivot: Point2,
    pub transform: Transform,
    pub next_entity: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub id: Identity,
    pub original_indices: Vec<usize>,
    pub geometry: Segment,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Contour {
    pub id: Identity,
    pub parents: Vec<Identity>,
    pub edges: Vec<Edge>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    pub parents: Vec<Identity>,
    pub children: Vec<Identity>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Drawing {
    pub version: u32,
    pub imports: Vec<Source>,
    pub contours: Vec<Contour>,
    pub transitions: Vec<Transition>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Binding {
    Contours {
        ids: Vec<Identity>,
    },
    Unresolved {
        previous: Vec<Identity>,
        reason: String,
    },
    Layers {
        names: Vec<String>,
    },
    All,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Evaluated {
    pub version: u32,
    pub segments: Vec<Segment>,
    /// Same cardinality as segments; each entry is an authoritative stable ID.
    pub membership: Vec<Identity>,
    pub contours: Vec<Identity>,
    pub closed: Vec<bool>,
    pub contour_bounds: Vec<BBox>,
    pub bbox: BBox,
}

fn finite(s: &Segment) -> bool {
    [s.start.x, s.start.y, s.end.x, s.end.y, s.bulge]
        .iter()
        .all(|v| v.is_finite())
        && s.center
            .map_or(true, |p| p.x.is_finite() && p.y.is_finite())
}
impl Drawing {
    /// Seed stable identities from actual chaining provenance, never matching
    /// coordinates back to input. Import boundaries remain distinct.
    pub fn from_sources(mut imports: Vec<Source>) -> Result<Self, String> {
        let mut contours = Vec::new();
        for source in &mut imports {
            if source.original.iter().any(|s| !finite(s)) {
                return Err("Nonfinite source geometry".into());
            }
            source.transform.validate()?;
            let mut next = 1u32;
            for (object, indices) in segments_to_objects_indexed(&source.original) {
                let id = Identity {
                    import: source.id,
                    entity: next,
                };
                next = next.checked_add(1).ok_or("Identity capacity exceeded")?;
                let mut edges = Vec::new();
                for (geometry, index) in object.segments.into_iter().zip(indices) {
                    edges.push(Edge {
                        id: Identity {
                            import: source.id,
                            entity: next,
                        },
                        original_indices: vec![index],
                        geometry,
                    });
                    next = next.checked_add(1).ok_or("Identity capacity exceeded")?;
                }
                contours.push(Contour {
                    id,
                    parents: vec![],
                    edges,
                });
            }
            source.next_entity = next;
        }
        let result = Self {
            version: CONTRACT,
            imports,
            contours,
            transitions: vec![],
        };
        result.validate()?;
        Ok(result)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.version != CONTRACT {
            return Err("Unsupported edited drawing revision".into());
        }
        let mut sources = BTreeMap::new();
        for source in &self.imports {
            source.transform.validate()?;
            if source.id == 0
                || sources.insert(source.id, source).is_some()
                || !source.pivot.x.is_finite()
                || !source.pivot.y.is_finite()
                || source.original.iter().any(|s| !finite(s))
            {
                return Err("Invalid original source identity or geometry".into());
            }
        }
        let mut live = BTreeSet::new();
        let mut provenance = BTreeSet::new();
        for contour in &self.contours {
            let source = sources
                .get(&contour.id.import)
                .ok_or("Missing original import")?;
            if contour.edges.is_empty() {
                return Err("Empty contour".into());
            }
            for id in std::iter::once(contour.id).chain(contour.edges.iter().map(|e| e.id)) {
                if id.import != source.id
                    || id.entity == 0
                    || id.entity >= source.next_entity
                    || !live.insert(id)
                {
                    return Err("Invalid or reused stable identity".into());
                }
            }
            for edge in &contour.edges {
                if !finite(&edge.geometry)
                    || edge
                        .original_indices
                        .iter()
                        .any(|&i| i >= source.original.len() || !provenance.insert((source.id, i)))
                {
                    return Err("Invalid edited edge or provenance".into());
                }
            }
        }
        let mut dead = BTreeSet::new();
        for transition in &self.transitions {
            if transition.parents.is_empty() {
                return Err("Missing lineage parent".into());
            }
            for parent in &transition.parents {
                if live.contains(parent) || !dead.insert(*parent) {
                    return Err("Reused lineage parent".into());
                }
                let source = sources
                    .get(&parent.import)
                    .ok_or("Unknown lineage import")?;
                if parent.entity == 0 || parent.entity >= source.next_entity {
                    return Err("Invalid lineage identity".into());
                }
            }
            for child in &transition.children {
                if transition
                    .parents
                    .iter()
                    .any(|p| p.import != child.import || p.entity >= child.entity)
                {
                    return Err("Invalid lineage order".into());
                }
            }
        }
        let mut children = BTreeSet::new();
        for transition in &self.transitions {
            for child in &transition.children {
                if !children.insert(*child) {
                    return Err("Repeated lineage child".into());
                }
                if self
                    .contours
                    .iter()
                    .any(|c| c.id == *child && c.parents != transition.parents)
                {
                    return Err("Lineage child parents mismatch".into());
                }
                if !self.contours.iter().any(|c| c.id == *child) && !dead.contains(child) {
                    return Err("Missing lineage child".into());
                }
            }
        }
        for contour in &self.contours {
            if !contour.parents.is_empty()
                && !self
                    .transitions
                    .iter()
                    .any(|t| t.children.contains(&contour.id) && t.parents == contour.parents)
            {
                return Err("Unproven contour lineage".into());
            }
        }
        Ok(())
    }
    pub fn evaluate(&self) -> Result<Evaluated, String> {
        self.validate()?;
        let mut segments = Vec::new();
        let mut membership = Vec::new();
        let mut contours = Vec::new();
        let mut closed = Vec::new();
        let mut contour_bounds = Vec::new();
        for contour in &self.contours {
            let source = self
                .imports
                .iter()
                .find(|s| s.id == contour.id.import)
                .ok_or("Missing source")?;
            let first_segment = segments.len();
            contours.push(contour.id);
            closed.push(
                contour.edges.first().unwrap().geometry.start
                    == contour.edges.last().unwrap().geometry.end
                    && contour
                        .edges
                        .windows(2)
                        .all(|w| w[0].geometry.end == w[1].geometry.start),
            );
            for edge in &contour.edges {
                let mut s = edge.geometry.clone();
                s.start = source.transform.point(s.start, source.pivot);
                s.end = source.transform.point(s.end, source.pivot);
                s.center = s.center.map(|p| source.transform.point(p, source.pivot));
                if source.transform.mirror_x != source.transform.mirror_y {
                    s.bulge = -s.bulge;
                }
                if !finite(&s) {
                    return Err("Transform exceeds finite coordinate range".into());
                }
                if edge.geometry.start != edge.geometry.end && s.start == s.end {
                    return Err("Transform loses edge precision".into());
                }
                membership.push(contour.id);
                segments.push(s);
            }
            contour_bounds.push(BBox::from_segments(&segments[first_segment..]));
        }
        let bbox = BBox::from_segments(&segments);
        Ok(Evaluated {
            version: CONTRACT,
            segments,
            membership,
            contours,
            closed,
            contour_bounds,
            bbox,
        })
    }
    pub fn validate_for_generation(&self) -> Result<(), String> {
        topology::validate(self)
    }
    #[must_use]
    pub fn preserve_binding(&self, binding: &Binding) -> Binding {
        match binding {
            Binding::Contours { ids }
                if ids.is_empty()
                    || ids
                        .iter()
                        .any(|id| !self.contours.iter().any(|c| c.id == *id)) =>
            {
                Binding::Unresolved {
                    previous: ids.clone(),
                    reason: "Contour merged, split or deleted — select geometry again".into(),
                }
            }
            _ => binding.clone(),
        }
    }
}

impl Evaluated {
    pub fn resolve(&self, binding: &Binding) -> Result<Vec<usize>, String> {
        match binding {
            Binding::Unresolved { reason, .. } => Err(reason.clone()),
            Binding::Contours { ids } => {
                if ids.is_empty() {
                    return Err("Empty explicit geometry binding".into());
                }
                ids.iter()
                    .map(|id| {
                        self.contours
                            .iter()
                            .position(|c| c == id)
                            .ok_or_else(|| "Missing stable contour — select geometry again".into())
                    })
                    .collect()
            }
            Binding::All => Ok((0..self.contours.len()).collect()),
            Binding::Layers { names } => {
                let layers: BTreeSet<&str> = names.iter().map(String::as_str).collect();
                let selected: BTreeSet<_> = self
                    .segments
                    .iter()
                    .zip(&self.membership)
                    .filter_map(|(s, id)| layers.contains(s.layer.as_ref()).then_some(*id))
                    .collect();
                Ok(self
                    .contours
                    .iter()
                    .enumerate()
                    .filter_map(|(i, id)| selected.contains(id).then_some(i))
                    .collect())
            }
        }
    }
}
