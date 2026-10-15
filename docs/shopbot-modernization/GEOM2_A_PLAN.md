# GEOM2-A — Geometry Transformations

2026-10-14 · **PLANNED — NOT IMPLEMENTED · HIGH / MEDIUM PRIORITY**.
Refines former [GEOM2 umbrella](GEOM2_PLAN.md); shared foundation for GEOM2-B and
NEST1. Follow [GEOM1/shared geometry audit](GEOMETRY_PREPARATION_PLAN.md) and
preserve active V-Carve qualification/HARD STOP and existing approvals.

## Objective and candidate scope

Transform selected existing geometry for exact workshop placement/resize and manual
nesting. One selection may contain outer contours, holes, nested paths and disconnected
internal details. A layer or import group is not automatically a logical part.

- Move: mouse drag, absolute XY, relative translation, optional qualified snapping
  and grouped movement; identify which selection reference the numeric XY positions.
- Rotate: arbitrary numeric angle, explicit clockwise/counterclockwise convention,
  quick ±90°, one shared pivot (selection centre, bbox corner, drawing origin or
  custom point). Do not rotate each contour independently when grouping is intended.
- Scale: mm width/height, percentages and proportional default; one factor/pivot
  for the whole selection. A 100×50 mm selection resized to W120 with proportions
  locked becomes 120×60, not separately resized contours.
- Nonuniform X/Y scale: qualify exact supported geometry, bounded documented
  conversion or explicit refusal. Circular arcs/circles can become elliptical;
  current circular/bulge representation cannot silently claim exact ellipse support.
- Drawing mirror: candidate/subphase only after handedness proof; independent of
  physical two-sided stock flip/registration.

## Existing foundation and architecture checkpoint

`frontend/src/lib/state/file-transform.ts` already applies per-import uniform scale,
X/Y mirror, rotate and translate about original file bbox centre. Rust `cam.rs`
has segment translation/rotation/uniform-scale/reflection helpers. This is partial
reuse, not completed selected-entity/group/pivot/dimension editing. Audit current
TS/Rust parity and avoid extending duplicated transform logic independently.

Use one explicit transform order and contract over drawing/source-part/instance/
stock/work/output frames ([layout architecture](NESTING_PLAN.md)). Resolve matrix
versus accepted/baked geometry, entity/group identity, bounds including arc extrema,
source provenance and edit/relief/tab/approach composition before freezing schema.
Preview and CAM must consume the same accepted transformed geometry. Moving drawing
geometry never changes work zero or stock placement implicitly. Preserve compatible
save/reopen and operation references; unsupported reference preservation needs an
explicit repair/refusal workflow, not silent operation loss/rebinding.

## UX and history

Canonical shared Professional Workshop UI + CAM UX: 2D selection/bounds/handles and
compact Transform inspector for position, size, proportions, angle and reference.
Reuse existing tokens/numeric controls and shell; no large permanent CAD toolbar.
Live feedback, precise entry, explicit completion, one history action per completed
transform; cancel restores state. Define transient preview versus live edit invalidation
before implementation. Source files stay untouched. Accepted changes invalidate paths.

## Future qualification and acceptance

Independent mathematical tests for single/group translation; centre/custom/corner
pivots; arbitrary and ±90° rotation; scale percentages/dimensions/proportions; supported
nonuniform scaling/refusal; circles/arcs; reflection and winding; holes/nested contours;
invalid/nonfinite/near-zero/negative factors; large/small coordinates and tolerance
boundaries. Include preview/accepted/CAM geometry equality, grouped relationships,
stable IDs, operation references, cancel/Undo/Redo/save-reopen and stale-output protection.

Acceptance: audit-approved exact/approximate/refused transform matrix, dimensionally
correct grouped transforms, stable authoritative geometry, preserved CAM parameters
and work zero, compatible persistence/history, regression and desktop accessibility
evidence, human gate. No physical fit or full-CAD capability claimed.

Open: representation/ownership, part/group selection, pivot conventions, curve conversion
error budget, negative-scale policy, bounds, source/relief ordering and parity mechanism.
Dedicated architecture review starts this milestone. No production changes now.

**STOP — waiting for human approval before implementation.**
