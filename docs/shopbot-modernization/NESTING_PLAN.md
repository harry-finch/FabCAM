# Geometry transformation and sheet nesting — shared architecture plan

2026-10-14 · **PLANNED — NOT IMPLEMENTED**. Documentation checkpoint ivac-ms20.
Extends [geometry preparation](GEOMETRY_PREPARATION_PLAN.md), not a new CAD workspace.
GEOM2-A/B and NEST1/2 identifiers have no conflict in the current roadmap.

## Dependencies and scheduling

GEOM1 → reliable prepared sources → GEOM2-A transform foundation.
GEOM2-A → GEOM2-B basic authoring, and separately → NEST1 manual layout → NEST2
automatic true-shape optimization. NEST1 does not require every drawing primitive.
GEOM3 optional utilities can progress independently when a chosen feature's dependencies
are qualified. This graph is not a forced execution order or permission to start work.
Current V-Carve human gate ivac-1b75 and approved priorities remain first; exact ordering
against two-sided/inlays/other future work stays open. Prefer GEOM1 before serious WJ.

## Existing capabilities and reuse limits

Current per-import TS FileTransform and Rust segment helpers provide translation,
rotation, uniform scale and reflection, not arbitrary grouped entity/part-instance
editing. ImportEntry IDs, combined geometry, normal source resolution, history,
stock/WCS and generation identity are reuse seams. Source containment “nesting” in
chaining/source_combine is contour topology, not sheet placement. Preliminary source
search found no dedicated sheet instance/true-shape optimizer; begin implementation
with deeper architecture/capability audit rather than claim inherited full nesting.

Reuse current stock dimensions/origin and fixture/work-area information where appropriate,
but sheet ownership is unresolved. Existing selected operation arrays/other transforms,
if applicable, must be audited before duplicates; they are not assumed to be part instances.

## Proposed ownership and evaluation — unfrozen

Source geometry → editable drawing/logical part → instance transform → resolved
positioned geometry → normal CAM source resolution → toolpaths → capability/post/output.
Persistent source edits and non-destructive instance placement share math, not ownership.
A source part may include outer outline, holes, pockets, engraving and disconnected
details. One instance has stable identity and placement; changing it does not mutate
the source. Prefer reference plus transform over unnecessary destructive copies,
but decide matrices versus baked/accepted geometry only after auditing consumers.

Resolve explicit drawing, source-part, instance, sheet/stock, work and output frames,
unit/angle/pivot conventions and transform order. Reuse CAM2/UX2 WCS normalization;
no extra hidden transform or implicit zero/stock changes. Verify reflection/handedness,
winding, holes, inside/outside compensation, cutting direction and arc geometry.
Drawing mirror is not physical two-sided flip. Audit existing frontend/Rust duplication;
one authoritative contract and accepted geometry must agree across preview and CAM.

Stable document/entity/part/instance/selection/operation references must survive changes
or produce explicit compatibility/refusal. Audit multi-import renumbering, source reload,
cleanup split/merge, relief/tab/approach attachments and quantities. GEOM1 cannot silently
alter valid open paths or corrupt instance identity; nesting validates its own required
piece outlines. Editing source versus placing instance versus modifying CAM parameters
must remain visible distinct contexts.

## Stock, CAM and clearance boundary

Decide whether a sheet is current stock or a linked layout concept; no competing
stock model. Margins/usable bounds/keep-outs/grain and stock orientation need explicit
ownership. Decide operation-to-instance mapping at its own architecture checkpoint:
source pocket/profile intent applied to quantities must reach each transformed source
through normal generic CAM, not translated finished code. Preserve work origins,
compensation, ordering, machine limits, warning/freshness and final output/preview parity.

Geometric overlap/spacing checks and machining clearance differ. Milling adds cutter
radius/compensation, leads/tabs/hold-down; WJ adds kerf, lead/pierce clearance, quality
and order. Placement-level infrastructure is generic; process constraints are explicit
adapters/requirements to audit, not hidden constants or a claimed safe machining verdict.
Fixtures remain obstacles with current uncertainty; no relaxed collision/export gates.
No ProtoMAX output, new CAM computation or FS coupling introduced here.

## Shared UX, lifecycle and open review

Professional Workshop UI + CAM UX govern compact selection/handles, numeric position/
angle, snapping/history and contextual sheet/placement inspector. No second drawing
editor or dashboard. UX5 Add instance/Duplicate/Rotate90/Arrange/Show warnings use
existing registry/owners only after implementation. No new commands in this plan.

Every source/instance/layout edit has explicit Apply/cancel, atomic Undo/Redo, compatible
save/reopen and deterministic accepted resolution. Source or placement changes make
previous generation stale. Search/view/optimization proposal does not silently alter
the job. No schema, library or optimizer selected; no external source overwrite.

Dedicated GEOM2-A and NEST1 architecture reviews must resolve representation, grouping,
matrix/baking, nonuniform arcs, references, sheet linkage, CAM binding, collision shapes,
process-clearance interface and tolerance/performance budgets. NEST2 begins with
algorithm/license/Rust integration audit. See [NEST1](NEST1_PLAN.md), [NEST2](NEST2_PLAN.md).
No new ADR freezes these decisions now; human scope/design approval precedes code.

**STOP — waiting for human approval before implementation.**
