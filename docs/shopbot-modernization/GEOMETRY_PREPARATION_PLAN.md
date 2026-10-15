# Drawing cleanup and lightweight geometry editing — shared plan

2026-10-15 · **GEOM1 ARCHITECTURE APPROVED; BOUNDED IMPLEMENTATION IN REVIEW**.
User selected GEOM1 after accepted V-Carve/UX6-B software gates. See
[the concrete audit](GEOM1_AUDIT.md) and approved ADR-0021, plus [results](GEOM1_RESULTS.md). The original placement
text below is historical; GEOM2/3 and nesting remain unstarted. Documentation checkpoint ivac-gpn7.
Future family within the existing CAD/geometry-preparation track; CAD1 is retained:
[GEOM1](GEOM1_PLAN.md) HIGH, [GEOM2-A](GEOM2_A_PLAN.md) HIGH/MEDIUM,
[GEOM2-B](GEOM2_B_PLAN.md) MEDIUM and [GEOM3](GEOM3_PLAN.md) FUTURE/OPTIONAL.
GEOM2 is refined into A/B; [NEST1](NEST1_PLAN.md) MEDIUM and
[NEST2](NEST2_PLAN.md) FUTURE/ADVANCED add separate layout milestones. No GEOM identifier conflict found in the
current roadmap. These names do not renumber existing milestones or add a full CAD track.

## Placement and approval

Keep current [V-Carve software review](VCARVE_QUALIFICATION.md), human gate
ivac-1b75, and its HARD STOP first. Propose GEOM1 after that geometry-sensitive
qualification, at the next separately approved geometry-preparation slot, and before
serious ProtoMAX integration. Cleanup improves Profile/Pocket/V-Carve source quality.
Do not displace approved work or invent an order against two-sided/inlay/other future
priorities; scheduling among them requires human approval. GEOM2 follows a stable
shared editing architecture; GEOM3 features are individually justified later.

FabCAM should prepare and repair geometry for machining without replacing a full
CAD application. No implementation, importer rewrite, project migration, new palette
commands, CAM/post changes, machine control or automatic publication in this plan.

## Current source-backed foundation

Paths below are repository-relative. This is preliminary inventory, not a complete
architecture qualification; each implementation starts with a deeper capability audit.

| Seam | Reuse and limits |
| --- | --- |
| `crates/ivac-core/src/geometry.rs`, `input.rs` | KEEP Point/Line/Arc/Circle, segment layers, bbox and import object metadata; not a full editable source-document model |
| `input/svg_in.rs` | KEEP usvg paths and applied group transforms; curves flattened with importer settings; current output uses common `svg` layer. Original SVG groups/primitive identity are not a preserved editable tree |
| `input/dxf_in.rs` | KEEP layer/unit conversion, line/polyline/arc/circle/point and other entity conversion; block transforms/text have distinct handling. Some short lines already filtered by importer MIN_DIST; analysis cannot diagnose information already discarded |
| `frontend/src/lib/state/file-transform.ts`, `import-ops.ts` | KEEP source-preserving per-import transforms and combined input; combineImports namespaces object IDs by preceding imports' maximum IDs, so topology/cardinality edits need explicit reference strategy |
| `project.svelte.ts::transformedImport`, [ADR-0013](adr/ADR-0013-persistent-corner-relief.md) | KEEP one common effective geometry for rendering/CAM, original source plus bounded accepted corner-relief result; audit edit/relief ordering and invalidation rather than grow a speculative feature tree |
| `crates/ivac-core/src/cam/chaining.rs`, `cam/source_combine.rs` | KEEP chaining/containment and operation-region booleans; their machining resolution is not permanent drawing repair or a completed drawing editor |
| `frontend/src/lib/state/commands.ts`, `history.ts`, `generation-context.ts` | KEEP explicit undoable mutation and authoritative CAM-input freshness; no separate canvas-only geometry or ready state |
| `frontend/src/lib/state/palette-catalog.ts`, `palette-dispatch.ts` | KEEP established UX5 registry/owner dispatch; integrate implemented geometry commands later |

Existing import transforms, text geometry, corner relief, selection and source booleans
are reusable capabilities, not GEOM1/2 completion. Audit arrays/mirror/offset and other
utilities before duplicating them. SVG/DXF do not expose equivalent entity/group/curve
semantics. Record actual unit fallback, open/closed reconstruction, layer/group loss,
curve approximation and point handling, with explicit limits; no importer change now.

## Shared architecture direction — not a frozen schema

External source documents → editable drawing geometry → CAM source/region resolution
→ generated paths → machine capability resolution → output. Distinguish external
files, editable entities, layers/groups, selected sources, boolean/containment machining
regions and generated paths. An operation's Union is not a permanent merge of shapes.

Prefer original import preservation and one authoritative effective drawing; audit
editable copies versus explicit edits/modifiers/source-edit separation before choosing.
All 2D/3D/CAM/selection/bounds consumers must agree. GEOM1/2 share infrastructure;
do not build competing entity stores or a canvas-only repair. Resolve stable source,
entity/layer/selection IDs and explicit remapping on join/split/delete/duplicate, including
multi-import references. Never silently bind an existing operation to a different shape.
Audit tab/approach/relief attachment and file-watcher/reimport conflict behavior.

Analyze and preview are read-only; Apply uses one coherent batch history action,
updates authoritative source resolution and invalidates current paths/diagnostics.
Undo/Redo restores affected references; save/reopen preserves edits compatibly.
Cancel changes nothing. Keep operation depth/feed/tool values unchanged unless
separately edited. Original SVG/DXF files are never overwritten without an explicit
export/overwrite workflow. Any future persisted model/version needs its own audit,
ADR and compatibility approval; no schema or algorithm is selected here.

## Tolerance and diagnostic policy

Name coordinate precision, join distance, simplification deviation, closure tolerance,
intersection/topology tolerance and toolpath approximation separately, with units and
geometry-scale assumptions. Test boundary inclusivity explicitly; no global epsilon,
viewport-pixel repair threshold or automatic machining-tolerance redefinition.
Curve/chord/output precision and physical tolerances are distinct. Reuse numerical
guidance in the canonical geometry skill; qualify large coordinates/small features.

Separate geometric findings from operation-specific incompatibility. Open lines,
short valid edges and drilling points are not automatically errors. Pocket closure
requirements must not trigger silent source repair. Automatic post-import analysis
may offer “Review drawing issues”; it never implies automatic modification. Severity
depends on actual issue and intended use; harmless findings are not export blockers.

## UX, palette and future waterjet

Canonical [$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md)
governs visuals; [$cam-ux-ui](../../../.agents/skills/cam-ux-ui/SKILL.md) adds CAM rules.
Use a Drawing/Geometry menu, compact 2D/contextual tools or post-import review entry,
not individual Pocket/Profile inspectors or a separate full CAD workspace. Preserve
shell density, numeric precision, selection/focus and contextual disclosure.
Original/removed/added/moved/simplified/unresolved repair geometry needs restrained
semantic legend and accessible text; never resemble a generated machining path.

UX5 is already implemented in its documented scope. Future Analyze/Join/Remove
duplicates/Simplify/Create rectangle/Create circle/Draw polyline commands reuse its
existing owner dispatch and availability. No command placeholders become executable
before the feature exists; no second keyboard or mutation handler.

GEOM1 remains process-independent: continuous contour and open/closed semantics,
duplicates/zero-length/micro-segment diagnostics, stable topology and reliable
tolerances are shared with future WJ. Entry/exit eligibility can use resulting geometry,
but pierce/quality/kerf/machine rules stay in waterjet CAM. Prefer GEOM1 before serious
ProtoMAX integration; no milling FS dependency or waterjet-specific cleanup engine.
See [future process assessment](FUTURE_PROCESSES.md).

## Common exclusions, evidence and unresolved decisions

Exclude 3D solids, full sketch constraint solvers, assemblies, parametric feature
histories, complex dimension-driven drafting, mechanical simulation and automated
part redesign. GEOM3 is not authorization to expand into these domains.

Each milestone needs independent geometric reference checks, targeted/regression
tests, lifecycle/source-reference/freshness proof, realistic desktop accessibility
review/screenshots and human approval before continuation. Screenshots alone prove
neither topology nor repair correctness. No software/physical PASS claimed here.

Open: representation and source/edit ordering, retained import provenance, stable IDs
through topology changes, relief/tab/source-watch conflict, tolerance defaults and
limits, cross-layer joins, curve-preserving repair/simplification, serialization/version
compatibility, diagnostic budgets and final feature-by-feature GEOM3 selection.

**STOP — waiting for human approval before implementing any geometry-editing milestone.**

## GEOM2 refinement and nesting dependency update

GEOM2-A supplies shared qualified grouped transforms; GEOM2-B reuses them for
created primitives. NEST1 depends on GEOM2-A and reliable GEOM1 source/preconditions,
not every GEOM2-B primitive. NEST2 builds on NEST1 sheets/instances/constraints;
GEOM3 chosen tools can progress independently when justified. See [nesting architecture](NESTING_PLAN.md).

Explicitly distinguish drawing/source edits from source-part definition and placed
instances. Nonuniform circular-curve scaling needs exact support, controlled conversion
or refusal; no silent approximation. Current per-import transforms are partial reuse.
Stable parts/instances/operations and current stock/WCS need audit before schema choice.
A valid drawing open path remains valid even if unusable as a nestable outer boundary.
No permanent source change or implicit zero/stock change through layout manipulation.

## Precise whole-import sizing — GEOM2-A1

[GEOM2-A1 specification](GEOM2_A1_PLAN.md) is an early PLANNED/unimplemented subtask
within transformations, using existing uniform Scale rather than independent Width/
Height storage. Effective local pre-rotation whole-import bounds and original GEOM1
pivot have different roles. Selected status bounds and project envelope are distinct;
hidden view layers do not change source membership. Preserve accepted GEOM1 authority,
lineage/history, preview/CAM identity and safe unsupported contexts. No selection/instance
editing, measurement system, repair change or format migration included.
