# CAD1 — Corner relief architecture checkpoint

2026-10-14 · ivac-g9gn · audited baseline `f6b88c9`.
User explicitly selected CAD1 / CAM4 C, requesting audit and important decisions
before implementation. This document records the original proposal. The user accepted its architecture and
bounded A/B human prerequisite on 2026-10-14; see ADR-0013 and CAD1 qualification.
No UX3, UX4, FS expansion, physical qualification or publication is authorized here.

## Prerequisites

CAM3 continuation is accepted in STATUS. CAM4 A/B has bounded software evidence
in [CAM4 qualification](CAM4_QUALIFICATION.md). On this baseline, rerun results:

- `cargo test -p ivac-core --features cps --offline --lib pipeline::remaining -- --test-threads=1`: 6 passed.
- `cargo test -p ivac-core --features cps --offline --test shopbot_m2_post cam4_pocket_residual_tracks_emitted_path_and_work_origin -- --test-threads=1`: 1 passed (actual emitted XYZ, XY origin, allowance refusal).
- `npm --prefix frontend run test -- src/lib/state/remaining-material.test.ts src/lib/state/generation-context.test.ts src/lib/state/history.test.ts src/lib/state/file-transform.test.ts`: 34 passed.

Rust environment: `CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime`,
`CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`,
`CARGO_INCREMENTAL=0`, `RUSTC_WRAPPER=''`. Node from `/opt/homebrew/opt/node@24/bin`.
These are targeted prerequisite checks, not a fresh full-suite qualification.

**Audit-time prerequisite:** human A/B gate `ivac-tvox` was open; the user
subsequently accepted this checkpoint and CAD1 architecture, closing that gate.
CAD1 authorization is recorded separately; it does not fabricate a visual review.
UX2.2 human/native gate `ivac-ein1` also remains recorded independently.
FS4 is deferred and does not block software work.

`pipeline/remaining.rs` explicitly excludes curved targets and analytic arc paths.
Relief geometry will introduce arcs. The existing residual overlay must therefore
continue to say Not analyzed for those cases until separately qualified. Recommend
independent CAD1 swept-cutter tests without expanding A/B diagnostics in this milestone.

## Source audit and reuse

Paths below are repository-relative.

| Existing seam | Disposition and consequence |
| --- | --- |
| `crates/ivac-core/src/geometry.rs`: Segment LINE/ARC, bulge, BBox | KEEP exact lines/arcs; no toolpaths or controller commands in the geometric evaluator |
| `crates/ivac-core/src/cam/chaining.rs`: closed chains, containment | EXTEND validation around reuse; its adaptive endpoint tolerance is not a CAD relief tolerance |
| `crates/ivac-core/src/cam/offsets/`, Cavalier/Clipper dependencies | KEEP CAM compensation; audit arc-preserving geometry operations before choosing local splice vs boolean union |
| `frontend/src/lib/state/project-types.ts`: ImportEntry source/transform; ProjectFile v1–4 | EXTEND with explicit persisted relief metadata and evaluated geometry; preserve old-file defaults |
| `frontend/src/lib/state/file-transform.ts`: combineImports | EXTEND the common effective-geometry seam; namespace IDs consistently; no second canvas-only relief implementation |
| `frontend/src/lib/state/project.svelte.ts`: transformedImport | KEEP common input for canvas, 3D, snapping, footprint and build-project |
| `frontend/src/lib/state/commands.ts`: setImportsCommand; `history.ts` | EXTEND with one atomic Apply command; history supports transactions but is session-only |
| `frontend/src/lib/state/import-ops.ts`: transforms update tabs/approach points | EXTEND safeguards; perimeter edits invalidate normalized tab locations even with unchanged object ID |
| `frontend/src/lib/api/build-project.ts`; core `Project.segments` | KEEP evaluated segments entering normal CAM; no relief synthesis during post-processing |
| `pipeline/remaining.rs`; `tests/shopbot_m2_post.rs` | KEEP A/B scope; reuse emitted-path comparisons for independent CAD1 proof |

No dedicated dogbone/T-bone implementation was found in core/frontend source.
The existing Relief surface subsystem is 3D surface milling, not corner relief.
Frontend transformed bounding boxes currently scan endpoints: arc extrema need
qualification/correction at the CAD1 integration seam, including stock sizing.
Multi-import object IDs depend on preceding imports' maximum ID: adding objects
can rebind later operations. Preserve contour IDs/cardinality in the proposed V1,
or explicitly remap every affected reference; never silently renumber.

## Recommended architecture — decision requested

Add a bounded **non-destructive corner-relief modifier**, not a general CAD tree.
Keep immutable imported source, selected source corners, mode/side, tool identity
and diameter snapshot, evaluator version, input fingerprint and accepted evaluated
geometry. Support removal/editing after reopening, not just session Undo.
Determine the minimal project-file version transition during the schema change;
older readers must not silently discard modifiers and machine the original contour.

Use one pure Rust evaluator in core, exposed through existing transport conventions
(WASM, HTTP and desktop). Inputs are explicit geometry, removal side, corner
references and physical cutter diameter in mm. Output is candidate segments,
corner acceptance/refusal reasons and source/result mapping. Tool-library lookup,
operations, feeds/speeds and ShopBot syntax stay outside this evaluator.

Evaluate in the final drawing frame after import scale/mirror/rotation, before WCS.
An accepted snapshot is shared by display and CAM. Do not scale a completed relief
along with the drawing and thereby change its physical cutter diameter. Tool,
source or transform changes invalidate the recipe and require preview/reapply;
stale modifier state blocks dependent generation/export. Fingerprints prevent late
asynchronous previews from being applied to changed inputs.

Applying must atomically update geometry and affected references. Retain object
identity where topology permits. For authored tabs/approach points, refuse Apply
with an actionable explanation unless exact remapping is proven; do not preserve
the old normalized perimeter coordinate blindly or delete settings silently.
Account for operations selecting all geometry or a layer, not only explicit IDs.

Alternative: replace imported segments using existing Undo alone. Simpler, but loses
editable source/recipe after reopening. Not recommended for the requested workflow.
This persistence/evaluation boundary warrants an ADR once the user accepts it.

## Geometry contract and first implementation scope

Recommend closed, simple contours with explicit material/removal side and
line-line selected corners, including nonorthogonal and concave multi-vertex
contours, holes/islands and reversed winding. Existing arcs away from the selected
corner must be preserved and checked for intersections. Arc-line/arc-arc selected
corners initially receive an explicit unsupported reason, not guessed construction.

Use finite positive actual configured Endmill diameter, radius D/2. Software
cannot infer measured wear or runout. No hidden enlargement or physical fit claim.
Dogbone relief is oriented on the appropriate corner bisector; T-bone selects one
incident edge explicitly, with both alternatives previewable. Construction must
define cutter-center feasibility, connected access from the removal region and
reach to the original vertex for arbitrary admitted angles; a displayed circular
lobe alone is not proof of machinability. Derive and test the construction before
choosing exact-arc splicing/union. Do not transplant a 90-degree formula to all angles.

Require a simple closed result, consistent winding/containment, no unintended
island/neighbor intersection, no erased thin web, and no merged disconnected
contours. Reject degenerate/nearly-collinear corners, short-edge interference,
overlapping reliefs and unreachable cutter centers when invariants cannot hold.
Batch validation considers reliefs together. Report incompatible corners before
Apply; apply only the explicitly confirmed compatible set as one undo step.

Radius-equals-cutter offset degeneracy is a specific proof obligation: verify the
normal Profile/Pocket planner actually visits the necessary cutter-center region.
If it does not, stop and propose a generic CAM correction or an explicit geometric
clearance policy; never add hidden oversizing or a ShopBot-only machining move.
Keep topology, flattening, chaining, export rounding and physical tolerances separate.

## UX proposal and review criteria

Use `professional-workshop-ui` from
`/Users/mugen/Documents/01_Projets/KiPartManager/professional-workshop-ui/SKILL.md`
and the repository `cam-ux-ui` skill. Reuse compact inspector controls, shared
semantic tokens, Lucide, focus treatment and existing canvas interaction patterns.

Flow: select contours → Corner relief → choose cutter and Dogbone/T-bone → select
corner markers → review overlay → Apply. Offer explicit Select compatible corners
within the selected contours, never automatic whole-project application. Show D,
selected/compatible counts, T-bone side and concise rejection reasons. Keep original
contour visible beside the proposed contour and show the cutter disk/center on hover.
Provide keyboard-accessible corner selection, non-color-only states, Escape/Cancel,
and one Apply action. Preview is transient and does not dirty the project.

Usability review: single/batch selection and quick correction. Comprehension review:
material side, cutter diameter, extra removed material, and distinction between
geometry preview and generated machining. Information architecture: geometry command
separate from machining operations. Visual review: dark compact inspector, narrow
window, long names, errors and keyboard focus; genuine runtime captures after code.

## Implementation sequence and acceptance

1. Resolve this checkpoint and record ADR: storage/evaluation contract, bounded
   corner scope, independent CAD1 proof versus unchanged A/B diagnostic exclusions.
2. Analytical core fixtures and pure evaluator; geometry-only transport contract.
3. Persisted modifier, shared effective geometry, stable references, history/staleness.
4. Explicit corner selection and transient preview with UX review.
5. Normal CAM generation and decoded ShopBot execution qualification; full relevant
   regressions, schema/type checks, lint/build and native/browser review evidence.

| ID | Required proof (planned, not yet run) |
| --- | --- |
| CAD1-G01 | 90° dogbone and both T-bone choices, several diameters: independently calculated centers, radius, endpoints and reachable original vertex |
| CAD1-G02 | Acute/obtuse angles, translation/rotation/mirror/reversed winding and unit normalization; scale does not silently scale cutter size |
| CAD1-G03 | Concave complex contours, holes/islands, distant arcs, adjacent selected corners and batch conflicts preserve topology and unrelated geometry |
| CAD1-G04 | Invalid diameters, duplicate/tiny edges, self-intersections, nearly-collinear corners, narrow webs, overlapping lobes and unreachable configurations refuse explicitly |
| CAD1-S01 | Apply/Undo/Redo, edit/remove after save/reopen, old project load, source reload, diameter change, multi-import references and stale async preview |
| CAD1-S02 | Tab/approach/source binding, exact arc bounds, stock frame and invalidation even after Save; failed Apply leaves all project state unchanged |
| CAD1-P01 | Geometry → Profile/Pocket → final preview → decoded ShopBot XYZ agrees within separately declared output error; independent cutter-sweep proof reaches corner without unrelated gouges |
| CAD1-P02 | XY work origin, direction, depth, supported leads/tabs and exact-radius offset degeneracy; unsupported contexts refuse without machine-specific geometric behavior |
| CAD1-U01 | Actual single/multiple/invalid corner interactions, Cancel, keyboard, narrow inspector, tool-change warning and readable original/proposed overlay |

HARD STOP at CAD1 human geometry/UX review after qualification. UX3/UX4 remain
unstarted. The original architecture checkpoint was accepted; implementation proceeds under
ivac-ddyn and ADR-0013, ending at the CAD1 human gate.
