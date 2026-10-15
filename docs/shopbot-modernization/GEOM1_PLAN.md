# GEOM1 — Drawing Cleanup & Repair

2026-10-14 · **PLANNED — NOT IMPLEMENTED · HIGH PRIORITY**.
Depends on the audit and common contracts in [shared plan](GEOMETRY_PREPARATION_PLAN.md).
After active V-Carve review and preserved approvals; preferred before serious WJ work.

## Objective and initial supported scope to qualify

Prepare imperfect SVG/DXF for machining: select → analyze → diagnose → set tolerances
→ preview → select corrections → explicitly Apply → Undo/Redo. Detection is separate
from permission to alter drawing geometry. All initial repair modes require proof:

- Isolated geometry: distinguish points, zero-length edges, short valid edges, drill
  locations and intentional open paths. No automatic deletion of meaningful points.
- Duplicates: identical/reversed segments and paths, coincident vertices and repeated
  zero-length edges, with explicit tolerances and cross-layer/provenance policy.
  Coincident or overlapping geometry can be intentional; review before removal.
- Join open paths, highest initial priority: actual endpoint distance in mm, not pixels.
  A 0.080 mm gap may be proposed under 0.100 mm tolerance, not automatically joined.
  Ambiguous competing candidates or unrelated contours require resolution/refusal.
  Snap endpoints, add connector and close contour are distinct explicit actions;
  disclose moved points/new edges and curve effects.
- Simplify: controlled geometric deviation for near-collinear/vectorized/micro-edge
  polylines and oversampled curves, preserving dimensions, acute corners and topology.
  Short-edge deletion alone is not a simplification error guarantee.
- Self-intersections: distinguish crossing, tangent, overlap, near miss and expected
  separate-shape intersections. Initial support may diagnose only; complex automatic
  topology repair deferred.
- Contour state: open, closed, nearly closed, invalid or ambiguous, interpreted with
  operation context where available. Open is not inherently invalid.

## Architecture, UX and boundaries

Use shared purpose-specific units/tolerances, stable identity and authoritative geometry.
Diagnostic records identify affected entities, proposal, tolerance, ambiguity and
operation impact. Counts are useful summaries, not frozen UI or invented PASS counts.
Preview original versus removed/added/moved/simplified geometry and unresolved findings;
choose corrections independently, cancel without mutation, Apply as coherent batch.
Surface reference/relief/tab conflicts before commit and stale machining results afterward.
Use canonical shared UI/CAM UX skills and Drawing/Geometry context, not giant cards
or per-operation repair controls. Post-import review is optional and proportionate.

Exclude silent normalization/repair, external source overwrite, advanced all-topology
healing, complete CAD editing, CAM/post rules and machine-specific cleanup. Analyze
what survives importer normalization first; never claim recovery of discarded entities.

## Future qualification and acceptance

| Matrix | Planned independent checks |
| --- | --- |
| Join | Exact coincidence, gaps just below/at/above tolerance, competing candidates, reversed paths and open/closed outcomes; snap versus connector versus closure |
| Identity/topology | Nested contours, coincident/overlapping/reversed segments, duplicate paths, distinct layers, intentional intersections and source references |
| Degeneracy | Zero-length edges, isolated points versus legitimate drill points, invalid/nonfinite inputs and very short valid features |
| Shape preservation | Nearly collinear vertices, acute corners, arcs/curves, deviation bound, self-crossing/tangent/near cases, large coordinates and tiny features |
| Lifecycle | Repeated-run determinism, preview/cancel nonmutation, one batch Undo/Redo, save/reopen, operations/tab/relief references and stale paths/collision diagnostics |

Acceptance: verified scope and tolerance contracts; accurate diagnostics; no ambiguous
silent repair; independently bounded simplification and joining; explicit preview/Apply;
stable references or explained refusal; history/persistence/current-output protection;
relevant regression tests and desktop/accessibility evidence; human software review.
Numerical proof does not establish physical machining tolerance.

Open decisions: supported repair subset, candidate selection and ambiguity, cross-layer
duplicate intent, curve snapping strategy, simplification metric/budget, importer loss,
preview proposal invalidation and identity remapping. Resolve in architecture audit first.

**STOP — waiting for human approval before implementation.**
