//! Explicit protocol v2. Receipts authorize local editing, never machining.
use super::{
    api::{self, DrawingRequest, DrawingResponse, OperationBinding},
    diagnostics::{self, TopologyReport},
    repair::Repair,
    Binding, Drawing, Identity,
};
use crate::{cancel::CancelToken, geometry::SegmentKind, project::Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RepairContext {
    pub drawing: Drawing,
    pub project: Project,
    pub bindings: Vec<OperationBinding>,
    /// Complete frontend document context for stale rejection; never accepted
    /// as proof of ownership or serialized in the portable response.
    pub document: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScopedPreview {
    pub context: RepairContext,
    pub repairs: Vec<Repair>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScopedRequest {
    Analyze {
        version: u32,
        input: Box<DrawingRequest>,
    },
    Preview {
        version: u32,
        input: Box<ScopedPreview>,
    },
    Apply {
        version: u32,
        preview: Box<ScopedPreview>,
        current: Box<RepairContext>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Continuity {
    pub contour: Identity,
    pub exact: bool,
    pub max_gap_mm: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScopedResponse {
    #[serde(flatten)]
    pub geometry: DrawingResponse,
    pub topology: TopologyReport,
    pub continuity: Vec<Continuity>,
    /// Explicitly local edit acceptance. No persisted CAM certificate.
    pub local_edit_accepted: bool,
}
fn same<T: Serialize>(a: &T, b: &T) -> Result<bool, String> {
    Ok(serde_json::to_value(a).map_err(|e| e.to_string())?
        == serde_json::to_value(b).map_err(|e| e.to_string())?)
}
fn response(
    mut geometry: DrawingResponse,
    accepted: bool,
    cancel: &CancelToken,
) -> Result<ScopedResponse, String> {
    if cancel.is_cancelled() {
        return Err("Drawing request cancelled".into());
    }
    geometry.version = 2;
    // Keep v1 findings exclusively for explicit repair proposals. V2 topology
    // owns global issues, including budgets; never attach those to contour 1.
    geometry.findings.retain(|f| {
        !f.message.starts_with("Topology:")
            && !f.message.starts_with("Simplification not analyzed:")
    });
    let topology = diagnostics::analyze(&geometry.drawing, cancel);
    let continuity = geometry
        .drawing
        .contours
        .iter()
        .map(|c| {
            let s = geometry
                .drawing
                .imports
                .iter()
                .find(|s| s.id == c.id.import)
                .expect("validated source");
            let gap = c
                .edges
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    s.transform.distance_mm(
                        e.geometry.end,
                        c.edges[(i + 1) % c.edges.len()].geometry.start,
                    )
                })
                .fold(0f64, f64::max);
            if !gap.is_finite() {
                return Err("Continuity measurement exceeds finite numeric range".to_string());
            }
            Ok(Continuity {
                contour: c.id,
                exact: c.edges.iter().enumerate().all(|(i, e)| {
                    e.geometry.end == c.edges[(i + 1) % c.edges.len()].geometry.start
                }),
                max_gap_mm: gap,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(ScopedResponse {
        geometry,
        topology,
        continuity,
        local_edit_accepted: accepted,
    })
}
pub fn evaluate(req: ScopedRequest, cancel: &CancelToken) -> Result<ScopedResponse, String> {
    if cancel.is_cancelled() {
        return Err("Drawing request cancelled".into());
    }
    let version = match &req {
        ScopedRequest::Analyze { version, .. }
        | ScopedRequest::Preview { version, .. }
        | ScopedRequest::Apply { version, .. } => *version,
    };
    if version != 2 {
        return Err("Scoped drawing requires explicit protocol v2".into());
    }
    match req {
        ScopedRequest::Analyze { input, .. } => {
            if matches!(*input, DrawingRequest::Preview { .. }) {
                return Err("Use v2 Preview with complete context".into());
            }
            response(api::evaluate(*input)?, false, cancel)
        }
        ScopedRequest::Preview { input, .. } => preview(&input, cancel),
        ScopedRequest::Apply {
            preview: input,
            current,
            ..
        } => {
            if !same(&input.context, current.as_ref())? {
                return Err("Drawing preview is stale; analyze and preview again".into());
            }
            // Re-run geometry, topology and placement proofs, not an echoed token.
            preview(
                &ScopedPreview {
                    context: *current,
                    repairs: input.repairs,
                },
                cancel,
            )
        }
    }
}

/// Recognize two exact layouts, never a fuzzy coordinate match: the current
/// evaluated drawing, or an untouched identity-transformed original import.
/// Return the actual baseline world geometry and membership in project order.
fn source_baseline(
    ctx: &RepairContext,
    evaluated: super::Evaluated,
) -> Result<(Drawing, super::Evaluated, Vec<Identity>), String> {
    let count = evaluated.segments.len();
    if !ctx.project.text_layers.is_empty()
        || ctx.project.segments.len() > 2000
        || ctx.project.segments.len() < count
    {
        return Err(
            "Source proof requires bounded matching drawing geometry and no live text".into(),
        );
    }
    let prefix = &ctx.project.segments[..count];
    if same(&evaluated.segments, &prefix.to_vec())? {
        let owners = evaluated.membership.clone();
        return Ok((ctx.drawing.clone(), evaluated, owners));
    }
    let d = &ctx.drawing;
    if !d.transitions.is_empty()
        || d.imports
            .iter()
            .any(|s| s.transform != super::Transform::default())
        || d.imports
            .iter()
            .try_fold(0usize, |n, s| n.checked_add(s.original.len()))
            != Some(count)
    {
        return Err(
            "Unsupported drawing source ordering or transform; source proof refused".into(),
        );
    }
    let canonical = Drawing::from_sources(d.imports.clone())?;
    if !same(&canonical.contours, &d.contours)? {
        return Err("Original imports do not represent the current edited drawing".into());
    }
    let raw: Vec<_> = d.imports.iter().flat_map(|s| s.original.clone()).collect();
    if !same(&raw, &prefix.to_vec())? {
        return Err("Actual project geometry disagrees with drawing source".into());
    }
    let mut owners = vec![];
    for source in &d.imports {
        for ordinal in 0..source.original.len() {
            let matches: Vec<_> = d
                .contours
                .iter()
                .filter(|c| {
                    c.id.import == source.id
                        && c.edges.iter().any(|e| e.original_indices == [ordinal])
                })
                .map(|c| c.id)
                .collect();
            if matches.len() != 1 {
                return Err("Original segment ownership is ambiguous".into());
            }
            owners.push(matches[0]);
        }
    }
    // The legacy frontend leaves identity-transformed coordinates untouched.
    // Use that exact baseline, avoiding subtract/add pivot rounding. This is
    // local proof data only: drawing-v1 evaluation semantics stay unchanged.
    let mut actual = canonical;
    for source in &mut actual.imports {
        source.pivot = crate::geometry::Point2::new(0., 0.);
    }
    let actual_evaluated = actual.evaluate()?;
    Ok((actual, actual_evaluated, owners))
}

fn preview(input: &ScopedPreview, cancel: &CancelToken) -> Result<ScopedResponse, String> {
    let before = &input.context.drawing;
    before.validate()?;
    if input.repairs.is_empty() || input.repairs.len() > 2000 {
        return Err("Select 1–2000 explicit repairs".into());
    }
    let pre = diagnostics::analyze(before, cancel);
    if !pre.complete {
        return Err("Incomplete or unsupported baseline topology; scoped proof refused".into());
    }
    let mut next = before.clone();
    let mut affected = BTreeSet::new();
    for r in &input.repairs {
        if cancel.is_cancelled() {
            return Err("Drawing request cancelled".into());
        }
        let Repair::Gap { contour, .. } = r else {
            return Err("Scoped v2 currently qualifies explicit line-gap repairs only".into());
        };
        affected.insert(*contour);
        next.apply_repair(r)?;
    }
    next.validate()?;
    let post = diagnostics::analyze(&next, cancel);
    if !post.complete {
        return Err("Incomplete or unsupported resulting topology; scoped proof refused".into());
    }
    // A retained defect must involve unchanged contours and exactly the same
    // ordered edge identities/reason. Never accept a moved existing crossing.
    for issue in &post.issues {
        if issue.contours.iter().any(|c| affected.contains(c)) || !pre.issues.contains(issue) {
            return Err(
                "Repair introduces or retains a topology defect on affected geometry".into(),
            );
        }
    }
    if before.contours.len() != next.contours.len() {
        return Err("Scoped repair changed contour membership".into());
    }
    for (a, b) in before.contours.iter().zip(&next.contours) {
        if a.id != b.id || (!affected.contains(&a.id) && !same(a, b)?) {
            return Err("Unmodified contour identity or geometry changed".into());
        }
    }
    // Sufficient, intentionally conservative containment proof: the swept
    // bounding envelope of each changed contour is disjoint from every other
    // before/after envelope,
    // including points, curves and other changed contours. This rules out
    // nesting and adjacency changes. Nested/ambiguous edits fail closed.
    let ctx = &input.context;
    let (actual_before, a, project_owners) = source_baseline(ctx, before.evaluate()?)?;
    let b = next.evaluate()?;
    let actual_pre = diagnostics::analyze(&actual_before, cancel);
    if !actual_pre.complete
        || post
            .issues
            .iter()
            .any(|issue| !actual_pre.issues.contains(issue))
    {
        return Err("Actual source topology proof is incomplete or introduces a new defect".into());
    }
    for change in api::drawing_changes(&actual_before, &next)? {
        let owner = before
            .contours
            .iter()
            .find(|c| c.edges.iter().any(|e| e.id == change.id));
        if owner.is_some_and(|c| !affected.contains(&c.id)) {
            return Err("Unmodified world geometry changed during source evaluation".into());
        }
    }
    let auxiliary: Vec<_> = ctx.project.segments[a.segments.len()..]
        .iter()
        .map(|s| {
            if !super::finite(s) {
                return Err("Nonfinite auxiliary geometry".to_string());
            }
            super::topology::validate_segment(s)?;
            Ok(crate::geometry::BBox::from_segments(std::slice::from_ref(
                s,
            )))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let disjoint = |x: crate::geometry::BBox, y: crate::geometry::BBox| {
        x.max_x < y.min_x || y.max_x < x.min_x || x.max_y < y.min_y || y.max_y < x.min_y
    };
    for (i, id) in a
        .contours
        .iter()
        .enumerate()
        .filter(|(_, id)| affected.contains(id))
    {
        let old = a.contour_bounds[i];
        let new = b.contour_bounds[i];
        let min_x = old.min_x.min(new.min_x);
        let min_y = old.min_y.min(new.min_y);
        let max_x = old.max_x.max(new.max_x);
        let max_y = old.max_y.max(new.max_y);
        let envelope = crate::geometry::BBox {
            min_x,
            min_y,
            max_x,
            max_y,
        };
        if auxiliary.iter().any(|other| !disjoint(envelope, *other)) {
            return Err(
                "Auxiliary geometry intersects the repair envelope; source proof refused".into(),
            );
        }
        for (j, other) in b.contour_bounds.iter().enumerate() {
            let prior = a.contour_bounds[j];
            if j != i
                && !(max_x < other.min_x.min(prior.min_x)
                    || other.max_x.max(prior.max_x) < min_x
                    || max_y < other.min_y.min(prior.min_y)
                    || other.max_y.max(prior.max_y) < min_y)
            {
                return Err(format!("Containment/adjacency proof is ambiguous for contour {}:{}; overlapping contour envelopes require review",id.import,id.entity));
            }
        }
        if before.contours[i]
            .edges
            .iter()
            .any(|e| e.geometry.kind != SegmentKind::Line)
        {
            return Err("Scoped curve repair is unsupported".into());
        }
    }
    let mut op_ids = BTreeSet::new();
    if ctx.bindings.len() != ctx.project.operations.len() {
        return Err("Incomplete operation binding context".into());
    }
    for op in &ctx.project.operations {
        if !op_ids.insert(op.id) {
            return Err("Duplicate operation identity".into());
        }
        let bindings: Vec<_> = ctx
            .bindings
            .iter()
            .filter(|x| x.operation == op.id)
            .collect();
        if bindings.len() != 1 {
            return Err("Missing or ambiguous operation binding".into());
        }
        if op.contour_params().is_some_and(|c| {
            c.tabs.active || !c.tab_placements.is_empty() || c.approach_point.is_some()
        }) {
            let Binding::Contours { ids } = &bindings[0].binding else {
                return Err(format!(
                    "Operation {} placements require explicit proven contour ownership",
                    op.id
                ));
            };
            if ids.len() != 1 || ids.iter().any(|id| affected.contains(id)) {
                return Err(format!(
                    "Operation {} placements depend on changed or ambiguous geometry",
                    op.id
                ));
            }
            a.resolve(&bindings[0].binding)?;
            // No reassignment: the actual operation source must resolve to the
            // same unique contour in the before view. Layer/All are not proofs.
            let crate::project::OpSource::Objects { ids: source, .. } = &op.source else {
                return Err("Placement source ownership is not explicit".into());
            };
            let index = a
                .contours
                .iter()
                .position(|id| id == &ids[0])
                .ok_or("Missing placement owner")?;
            if source.as_slice()
                != [u32::try_from(index + 1).map_err(|_| "Contour capacity exceeded")?]
            {
                return Err("Placement source and stable owner disagree".into());
            }
            // Bind ownership to the actual geometry supplied to CAM, not only
            // a caller's stable-ID claim. This bounded slice supports unshifted
            // imported geometry, with optional disjoint auxiliary segments.
            if ctx.project.work_offset.x_mm != 0. || ctx.project.work_offset.y_mm != 0. {
                return Err("Placement ownership requires zero XY work offset".into());
            }
            if auxiliary
                .iter()
                .any(|other| !disjoint(a.contour_bounds[index], *other))
            {
                return Err("Auxiliary geometry makes placement ownership ambiguous".into());
            }
            let objects = crate::cam::chaining::segments_to_objects_indexed(&ctx.project.segments);
            let actual = objects
                .get(index)
                .ok_or("Missing actual placement source")?;
            if actual.1.len() != project_owners.iter().filter(|id| **id == ids[0]).count()
                || actual
                    .1
                    .iter()
                    .any(|&i| project_owners.get(i) != Some(&ids[0]))
            {
                return Err(
                    "Placement source merges or disagrees with its claimed contour owner".into(),
                );
            }
            if let Some(c) = op.contour_params() {
                if c.tab_placements.iter().any(|p| {
                    p.object_id != source[0] || !p.t.is_finite() || !(0.0..1.0).contains(&p.t)
                }) || c
                    .approach_point
                    .is_some_and(|(x, y)| !x.is_finite() || !y.is_finite())
                {
                    return Err(
                        "Authored placement does not belong to the unchanged contour".into(),
                    );
                }
            }
        }
    }
    let bindings = ctx
        .bindings
        .iter()
        .map(|x| next.preserve_binding(&x.binding))
        .collect::<Vec<_>>();
    let binding_indices = bindings.iter().map(|bnd| b.resolve(bnd).ok()).collect();
    let changes = api::drawing_changes(before, &next)?;
    let findings = next.analyze(0.1)?;
    let geometry = DrawingResponse {
        version: 2,
        drawing: next,
        evaluated: b,
        findings,
        bindings,
        legacy_contours: vec![],
        changes,
        binding_indices,
    };
    response(geometry, true, cancel)
}
