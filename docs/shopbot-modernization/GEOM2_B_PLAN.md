# GEOM2-B — Basic 2D Drawing Tools

2026-10-14 · **PLANNED — NOT IMPLEMENTED · MEDIUM PRIORITY**.
Drawing phase of [GEOM2](GEOM2_PLAN.md), following the shared geometry architecture
and qualified [GEOM2-A](GEOM2_A_PLAN.md). Not a prerequisite for every NEST1 action.

## Bounded goal and dependencies

Create Line, Polyline, Rectangle, Circle and Arc through mouse interaction and exact
numeric dimensions/position. Select/edit entities, assign layers, cancel or commit,
Undo/Redo and save/reopen. Reuse GEOM2-A move/rotate/scale/pivot and qualified snap
behavior for created shapes, not a second transformation implementation.

Practical examples: 120×80 mm plate, Ø6 circle, simple machining contour/reference
line. Drawing a rectangle creates no Pocket automatically; a circle creates no Drill.
Normal CAM creation consumes accepted geometry explicitly. Preserve geometry/source
identity and machining depth/rates/tools; expose deletion/reference conflicts.

## Architecture, UX and exclusions

Use [shared plan](GEOMETRY_PREPARATION_PLAN.md) and [transform/layout contract](NESTING_PLAN.md):
one effective geometry for drawing/CAM, source protection, compatible history/persistence,
no frozen entity schema. Compact contextual 2D toolbar and numeric inspector follow
Professional Workshop UI + CAM UX; preserve keyboard focus, selection and cancellation.
Exclude constraint solvers, full CAD sketching, 3D solids, assembly design, parametric
histories and automatic CAM. GEOM3 utilities remain separately scoped.

## Future acceptance and qualification

Independent dimensions/coordinates and circle/arc centre/radius/sweep/orientation;
degenerate/nonfinite inputs and numerical scale extremes; imported/created mixing,
layers/selection/IDs; reuse of GEOM2-A transforms and snapping; creation/edit/delete
cancel/history/save-reopen; normal CAM source creation and stale output; unchanged
operation rates/tools. Relevant regressions, desktop accessibility/captures and human
review required. Geometry correctness is not physical machining validation.

Open: primitive/arc entry policy, entity/layer ownership, persistent reference scheme,
source/edit composition, operation compatibility/deletion and supported first subset.
Audit before implementation; no importer/schema/UI change now.

**STOP — waiting for human approval before implementation.**
