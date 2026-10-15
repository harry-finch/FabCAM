# GEOM3 — Advanced Geometry Tools

2026-10-14 · **PLANNED — NOT IMPLEMENTED · FUTURE / OPTIONAL**.
Use stable qualified shared geometry/transform contracts from GEOM1/GEOM2-A
and [shared plan](GEOMETRY_PREPARATION_PLAN.md); chosen utilities may progress
independently when their own prerequisites are qualified, without all GEOM2-B/NEST2.
This is a candidate backlog family, not a commitment to deliver every utility.

## Objective and candidate scope

Selected workshop-demanded utilities: drawing offset, alignment/distribution,
rectangular/circular arrays, trim/extend, advanced simplification, selected
boolean shape edits, justified fillets/chamfers and advanced contour repair.
Audit current import transforms, CAM offsets/source booleans and other reuse before
adding duplicates. Drawing offset/boolean edits are explicit source changes, distinct
from tool compensation or operation-region construction.

Each chosen feature needs its own bounded proposal, topology/tolerance and identity
contract, explicit preview/commit, Undo/Redo, persistence and stale-output checks.
Keep advanced controls contextual/progressively disclosed through existing drawing
and UX5 action infrastructure. No full CAD workspace, solver, solids, assemblies,
parametric history, mechanical simulation or general drafting system implied.

## Qualification and acceptance direction

Select individual tools from real use cases before implementation. For each: independent
analytic expected geometry; holes/islands/open paths/arcs and malformed/degenerate cases;
transform/offset/boolean/refusal boundaries; deterministic output and scale-aware
tolerances; stable layer/entity/operation references; preview/cancel/history/save and
current-output safeguards; existing CAM regressions and desktop/accessibility review.
No screenshot-only proof or automatic repair of unsupported topology.

Drawing mirror is now a GEOM2-A candidate/subphase; GEOM3 must not duplicate it.
Source arrays and NEST1 quantities/instances have distinct ownership; alignment
may serve both through one qualified action/math contract.

Acceptance is feature-specific and approved after relevant shared-foundation evidence. Unselected
candidates remain deferred; no aggregate “advanced geometry complete” claim. Unresolved:
which utilities matter, supported topology, exact/approximate curve handling, budgets,
identity on split/merge and existing-feature overlap. Human review gates each extension.

**STOP — waiting for human approval before implementation.**
