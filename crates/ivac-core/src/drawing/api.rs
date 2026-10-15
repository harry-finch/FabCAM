//! Explicit transport contract. Old engines lack these endpoints/exports and
//! must refuse; the frontend must never fall back to legacy generation.
use super::repair::{Finding, Repair};
use super::{Binding, Drawing, Evaluated, Source};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DrawingRequest {
    Start {
        version: u32,
        imports: Vec<Source>,
        legacy_offset: crate::geometry::Point2,
        #[serde(default)]
        legacy_text: Vec<crate::project::TextLayer>,
        #[serde(default)]
        legacy_auxiliary: Vec<crate::geometry::Segment>,
        #[serde(default)]
        tolerance_mm: Option<f64>,
        #[serde(default)]
        simplification_mm: Option<f64>,
    },
    Evaluate {
        version: u32,
        drawing: Drawing,
        #[serde(default)]
        tolerance_mm: Option<f64>,
        #[serde(default)]
        simplification_mm: Option<f64>,
    },
    Preview {
        version: u32,
        drawing: Drawing,
        repairs: Vec<Repair>,
        bindings: Vec<Binding>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DrawingResponse {
    pub version: u32,
    pub drawing: Drawing,
    pub evaluated: Evaluated,
    pub findings: Vec<Finding>,
    pub bindings: Vec<Binding>,
    pub legacy_contours: Vec<Option<super::Identity>>,
    pub changes: Vec<DrawingChange>,
    pub binding_indices: Vec<Option<Vec<usize>>>,
}
pub fn evaluate(req: DrawingRequest) -> Result<DrawingResponse, String> {
    let mut legacy_contours = vec![];
    let mut changes = vec![];
    let revision = match &req {
        DrawingRequest::Start { version, .. }
        | DrawingRequest::Evaluate { version, .. }
        | DrawingRequest::Preview { version, .. } => *version,
    };
    if revision != super::CONTRACT {
        return Err("Unsupported GEOM1 protocol revision".into());
    }
    let (v, d, bindings, tolerance, simplification) = match req {
        DrawingRequest::Start {
            version,
            imports,
            tolerance_mm,
            simplification_mm,
            legacy_offset,
            legacy_text,
            legacy_auxiliary,
        } => {
            let drawing = Drawing::from_sources(imports)?;
            legacy_contours = legacy_map(&drawing, legacy_offset, &legacy_text, &legacy_auxiliary)?;
            (
                version,
                drawing,
                vec![],
                tolerance_mm.unwrap_or(0.1),
                simplification_mm,
            )
        }
        DrawingRequest::Evaluate {
            version,
            drawing,
            tolerance_mm,
            simplification_mm,
        } => (
            version,
            drawing,
            vec![],
            tolerance_mm.unwrap_or(0.1),
            simplification_mm,
        ),
        DrawingRequest::Preview {
            version,
            drawing,
            repairs,
            bindings,
        } => {
            let (d, b) = drawing.preview_repairs(&repairs, &bindings)?;
            changes = drawing_changes(&drawing, &d)?;
            (version, d, b, 0.1, None)
        }
    };
    if v != super::CONTRACT {
        return Err("Unsupported GEOM1 protocol revision".into());
    }
    let evaluated = d.evaluate()?;
    let mut findings = d.analyze(tolerance)?;
    if let Some(mm) = simplification {
        findings.extend(d.simplification_findings(mm)?);
    }
    let binding_indices = bindings.iter().map(|b| evaluated.resolve(b).ok()).collect();
    Ok(DrawingResponse {
        version: v,
        drawing: d,
        evaluated,
        findings,
        bindings,
        binding_indices,
        legacy_contours,
        changes,
    })
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OperationBinding {
    pub operation: u32,
    pub binding: Binding,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DrawingGenerationRequest {
    pub version: u32,
    pub drawing: Drawing,
    pub bindings: Vec<OperationBinding>,
    pub machining: crate::pipeline::PipelineRequest,
}

/// Prove old UI object tags and the actual legacy CAM chain agree on the
/// identical source ordinals. A geometric coincidence is never sufficient.
fn legacy_map(
    d: &Drawing,
    offset: crate::geometry::Point2,
    text: &[crate::project::TextLayer],
    auxiliary: &[crate::geometry::Segment],
) -> Result<Vec<Option<super::Identity>>, String> {
    if !offset.x.is_finite() || !offset.y.is_finite() {
        return Err("Nonfinite legacy work offset".into());
    }
    let mut segments = vec![];
    let mut owners = vec![];
    let mut ui_ids = vec![];
    let mut id_offset = 0u32;
    for source in &d.imports {
        for (ordinal, raw) in source.original.iter().enumerate() {
            let mut s = raw.clone();
            s.start = source.transform.point(s.start, source.pivot);
            s.end = source.transform.point(s.end, source.pivot);
            s.start.x -= offset.x;
            s.start.y -= offset.y;
            s.end.x -= offset.x;
            s.end.y -= offset.y;
            s.center = s.center.map(|p| {
                let mut q = source.transform.point(p, source.pivot);
                q.x -= offset.x;
                q.y -= offset.y;
                q
            });
            if source.transform.mirror_x != source.transform.mirror_y {
                s.bulge = -s.bulge;
            }
            segments.push(s);
            owners.push(
                d.contours
                    .iter()
                    .find(|c| {
                        c.id.import == source.id
                            && c.edges
                                .iter()
                                .any(|e| e.original_indices.contains(&ordinal))
                    })
                    .map(|c| c.id),
            );
            ui_ids.push(
                source
                    .legacy_objects
                    .get(ordinal)
                    .copied()
                    .filter(|id| *id > 0)
                    .and_then(|id| id.checked_add(id_offset)),
            );
        }
        id_offset = id_offset
            .checked_add(source.legacy_objects.iter().copied().max().unwrap_or(0))
            .ok_or("Legacy ID capacity exceeded")?;
    }
    for raw in auxiliary {
        let mut s = raw.clone();
        s.start.x -= offset.x;
        s.start.y -= offset.y;
        s.end.x -= offset.x;
        s.end.y -= offset.y;
        s.center = s
            .center
            .map(|p| crate::geometry::Point2::new(p.x - offset.x, p.y - offset.y));
        owners.push(None);
        ui_ids.push(None);
        segments.push(s);
    }
    for layer in text {
        let mut layer = layer.clone();
        layer.origin.0 -= offset.x;
        layer.origin.1 -= offset.y;
        let rendered = crate::input::text::render_text_layer(&layer).map_err(|e| e.to_string())?;
        owners.extend(std::iter::repeat(None).take(rendered.len()));
        ui_ids.extend(std::iter::repeat(None).take(rendered.len()));
        segments.extend(rendered);
    }
    Ok(crate::cam::chaining::segments_to_objects_indexed(&segments)
        .into_iter()
        .enumerate()
        .map(|(index, (_, indices))| {
            let first = indices.first().and_then(|&i| owners[i]);
            let matches = first.is_some()
                && indices
                    .iter()
                    .all(|&i| owners[i] == first && ui_ids[i] == u32::try_from(index + 1).ok())
                && indices.len() == owners.iter().filter(|id| **id == first).count();
            if matches {
                first
            } else {
                None
            }
        })
        .collect())
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DrawingChange {
    pub id: super::Identity,
    pub before: Option<crate::geometry::Segment>,
    pub after: Option<crate::geometry::Segment>,
}
fn drawing_changes(before: &Drawing, after: &Drawing) -> Result<Vec<DrawingChange>, String> {
    fn edges(
        d: &Drawing,
    ) -> Result<std::collections::BTreeMap<super::Identity, crate::geometry::Segment>, String> {
        let evaluated = d.evaluate()?;
        Ok(d.contours
            .iter()
            .flat_map(|c| c.edges.iter().map(|e| e.id))
            .zip(evaluated.segments)
            .collect())
    }
    let a = edges(before)?;
    let b = edges(after)?;
    let keys: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).copied().collect();
    Ok(keys
        .into_iter()
        .filter(|id| a.get(id) != b.get(id))
        .map(|id| DrawingChange {
            id,
            before: a.get(&id).cloned(),
            after: b.get(&id).cloned(),
        })
        .collect())
}
