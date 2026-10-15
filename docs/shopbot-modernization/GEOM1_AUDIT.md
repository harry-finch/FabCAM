# GEOM1 — Architecture audit and implementation proposal

2026-10-15 · Historical pre-implementation audit. Architecture subsequently approved; see [implementation and qualification](GEOM1_RESULTS.md).
Engineering: ivac-cbgkk. User explicitly selected GEOM1 after accepting UX6-B.
This document supersedes the plans' pending scheduling language, not their
architecture review requirement. The audit itself changed no production code; subsequent work is recorded in the results.

## Preconditions

V-Carve software review ivac-1b75 is closed (2026-10-15); its sampled-coverage,
native and physical limits remain. UX6-B ivac-y2aln is accepted. Independent
PERF cancellation latency, hierarchy and physical gates remain separate: GEOM1
neither closes them nor depends on achieving a machine qualification.
Existing uncommitted preview-navigation/tool-library polish is preserved.

## Source-backed findings

Paths are repository-relative. These are implementation observations, not a
claim that the proposed repairs already work.

| Boundary | Existing behavior and consequence | Direction |
| --- | --- | --- |
| `crates/ivac-core/src/geometry.rs`, `input.rs` | Flat LINE/ARC/CIRCLE/POINT with layer/color; no persistent segment identity. Import object IDs are chain positions. `object_index` maps back by endpoints, without comparing layer/type/bulge: insufficient provenance for cleanup. | KEEP primitives; EXTEND provenance for edited drawings. |
| `input/svg_in.rs` | usvg transforms and Y inversion; flattened curves, common `svg` layer; lines below 1e-6 mm discarded. Unitless coordinates conventionally map 1:1 to mm; physical units use importer conversion. Recursion cap means the curve approximation is not a new guaranteed cleanup error bound. | KEEP importer. Analyze retained segments only; do not promise original Bézier/group recovery. |
| `input/dxf_in.rs` | Unit conversion with 1.0 fallback; `push_line` retains lengths strictly above 1e-4 mm; point entities survive as POINT. Curves/text/blocks have distinct conversion paths. | KEEP importer; report points separately, never auto-delete drill locations. |
| `state/file-transform.ts::combineImports` | Later import IDs are offset by preceding imports' maximum object IDs. Removing the last object in an earlier import shifts later references. Equal layer names are merged. | EXTEND edited-source identity/resolution; no cross-import/layer repair by default. |
| `api/build-project.ts`, `pipeline.rs`, `pipeline/selection.rs` | Generation sends segments then chains the entire pool again; object source IDs index that new array. The UI's `objects[]` is not an authoritative CAM identity contract. Text is appended before chaining. | EXTEND the shared geometry-to-CAM binding boundary for edited drawings; a frontend-only ID fix is insufficient. |
| `cam/chaining.rs` | Adaptive fuzzy distance = max(0.001 mm, bbox diagonal × 0.0001). Endpoint matching/closure does not constitute explicit repair permission. | KEEP legacy CAM behavior; cleanup must measure actual endpoints independently. New edited-source resolver must prove agreement or refuse. |
| `project.svelte.ts`, `corner-relief.ts` | Effective import is either combined imports or a source-key-matching relief snapshot. Relief already refuses changed source/tool and authored tab/approach conflicts. | EXTEND one effective drawing pipeline; no second canvas-only result. |
| `file-transform.ts` | Transform pivot derives from input bbox; bounds currently use endpoints, insufficient for arc extrema. A changed bbox can move otherwise untouched geometry under rotation/scale. | Freeze original transform pivot for edited imports and qualify exact arc bounds. |
| `commands.ts`, `history.ts`, `generation-context.ts` | Snapshot commands and transactions available; freshness derives from actual generation inputs, independently of Save dirty flag. | KEEP; one repair batch including bindings, and errors in generation key. |
| `project-file-ops.ts`, `feeds-speeds-persistence.ts` | Guarded kinds/versions 1–6, conditional save format. Unknown versions rejected. | EXTEND with guarded edited-drawing format; all existing files continue to load. |
| `import-ops.ts::reimportFromPath`, `desktop.ts` | Auto-reload can replace a watched source; function directly assigns imports despite its older undo-transaction comment. Missing numeric IDs are warned, but reused IDs can mean different shapes. | EXTEND conflict handling for edited sources; no silent rebase or auto-reload over repairs. |

Concrete identity counterexample: A contains local objects 1 and 2, B contains
local object 1. B is displayed as object 3. Removing A's object 2 makes B object 2.
Retaining the old numeric selection 3 cannot express B reliably. A coincident
A/B pair can additionally be regrouped by whole-pool CAM chaining. Bounds/ID
presence alone do not prove that an operation kept its intended source.

## Recommended architecture

See proposed [ADR-0021](adr/ADR-0021-edited-drawing-contract.md). Retain immutable
imported geometry plus one current editable snapshot per import, with stable
entity/contour identities and provenance. This is not a parametric feature tree.
Pure Rust analysis/proposal/validation belongs in generic geometry preparation,
not in Profile/Pocket, the frontend canvas or a ShopBot postprocessor. Expose the
same implementation through native/server/WASM; schema and generated types must
be updated together with capability guards for an older connected backend.

Order: retained import → accepted drawing edit → existing file transform using
original pivot → combined effective drawing → explicitly reapplied corner relief
→ CAM source resolution → normal toolpaths/simulation/output. A pending proposal
is transient. Only accepted geometry and reference state are persisted.

Introduce stable IDs only where needed for this contract; preserve legacy loading
and unchanged generation for projects without edits. At the generation boundary,
resolve stable source membership against the actual chained result. Reject any
mixed membership, split/merge, missing entity or unsupported synthesized source
that cannot be mapped unambiguously. Do not infer identity from equal endpoints.
Global rechaining, text, patterns/frames and source-combine ordering must be in
this qualification; no silent fallback to positional IDs. A native Project/API
extension is required to enforce the contract independently of UI checks.

Before first editing a legacy drawing, resolve its existing operation bindings
against the exact current pipeline result. If provenance cannot be established,
require explicit source reselection. Do not reinterpret old numeric IDs as newly
stable IDs. All/Layers operations keep their selection semantics, but their
changed material scope must be shown. Joining two contours is allowed for an
operation only when the full resulting membership matches its selected intent;
joining a selected to an unselected contour blocks Apply until resolved. Removed
referenced geometry similarly requires a reviewed rebind, not an empty list
(which can mean All). Preserve Difference order and disabled-operation references.

Apply revalidates the full source/import/transform/reference/modifier snapshot,
selected proposals and tolerances, then commits geometry and all approved bindings
in one history action. Undo restores both. Reimport, transform, text or operation
edits invalidate pending analysis. Overlapping selected proposals require a fresh
combined validation; individually valid proposals are not automatically a valid batch.

Initial conservative conflicts: refuse Apply while corner relief is present, or
when affected contours have authored tabs/approach points. Explain removal and
reapplication; do not silently discard them. Future automatic relocation is outside
this scope. Watcher changes on edited imports show a conflict, preserving accepted
edits; explicit replacement/reset is undoable and revalidates all references.

## Bounded repair scope proposed

| Finding | Initial action |
| --- | --- |
| Open / nearly closed line chains | Highest priority: explicit endpoint snap, connector, or closure. Same import/layer only; unique pair, no competing candidates. Snap moves one explicitly identified endpoint to the other; connector preserves both. Closure names its chosen mechanism. Check the entire resulting topology. |
| Exact duplicate LINE or complete line-only path | Explicit removal selection, including reversed order; preserve chosen survivor and provenance. Cross-layer/import coincidences are diagnostic only. Near duplicates are initially diagnostic, not tolerance-based automatic deletion. |
| POINT / zero-length / short valid segment | Separate diagnostics. POINT never silently deleted; deleting non-point degeneracies must pass topology/reference checks. Short length alone is not a repair criterion. |
| Simplification | Line-only chains, explicit error budget and protected corners/endpoints. No generic short-edge deletion. Reject changes to winding, intersections, containment or minimum-feature separation. |
| Crossing, touch, overlap, near miss | Diagnostic with affected geometry; no automatic complex topology healing. Unsupported analytic-curve cases explicitly Not analyzed, never zero issues. |
| Arc/circle endpoint changes; live generated text | Diagnosis within qualified capabilities only. Defer arc snapping/refitting, curve simplification and editing live text glyphs; do not flatten silently. |

## Numerical contract to implement and test

All UI tolerances are drawing-space millimetres after transforms. Proposed join
suggestion 0.100 mm, editable and never permission to repair. Inclusive distance
comparison `distance <= join_mm`; reject nonfinite/negative input and report actual
distance with sufficient precision. Conversion to local coordinates accounts for
uniform scale; reject singular transforms. Closure and simplification have separate
fields; simplification is opt-in, proposed initial deviation 0.010 mm. These are
software proposal defaults, not physical machining accuracy promises.

Exact-duplicate mode uses exact retained primitive geometry (including reversal),
not the join threshold. Numeric predicate error and topology uncertainty must be
separate from user tolerances. Implement robust predicates or explicit uncertainty
refusal; do not choose an arbitrary global epsilon at implementation time. Test
near decisions with representable just-below/at/just-above inputs and large offsets.

Simplification acceptance requires a continuous, bidirectional distance bound
between retained source polyline and replacement, not merely vertex sampling or
a one-sided RDP claim. Protect acute vertices/endpoints and check replacement
segments against untouched geometry, including islands. Error is measured against
the imported polyline; source-curve flattening error is separate and may be unknown.
Use an independently implemented analytic/reference checker in qualification.
If proof or bounded runtime fails, refuse that proposal with a reason.

Spatial candidate indexing and deterministic ordering are required. Bound work,
expose cancellation and report incomplete analysis explicitly. No arbitrary data
size or timing PASS is claimed in this audit; choose measured budgets using small
analytic fixtures and realistic dense imported drawings before acceptance.

## UX contract

Apply `professional-workshop-ui` and `cam-ux-ui`: contextual compact Drawing
inspector, entry through Drawings/Geometry and existing palette owner dispatch.
Selection remains possible while choosing the analysis scope. Explicit scope,
Analyze, diagnostic rows, distance/tolerance in mm, before/after overlay and
independent correction selection. Rows identify import/layer/entity and affected
operations. Original/removed/added/moved geometry uses labels/line styles as well
as restrained colors, distinct from toolpaths. Enter focuses a finding; keyboard
selection and visible focus remain usable at narrow desktop widths.

Apply stays disabled with a specific reason for stale/ambiguous/unsupported batches;
Cancel leaves the drawing unchanged. After Apply, say how many corrections were
made and that machining results need regeneration. Do not show a global clean/pass
state when curves or part of the analysis were not assessed. No new UI was built
or visually qualified during this audit.

## Incremental implementation and acceptance

1. Reference counterexamples first: two-import ID shift; coincident different-layer
   endpoints; fuzzy closed but physically gapped contour; transformed bbox pivot;
   live text/global rechain; reimport during preview. Preserve legacy baseline.
2. Implement the accepted identity/persistence/effective-geometry foundation, one
   analysis/preview/apply lifecycle, initially explicit straight-chain joins.
3. Add qualified duplicate/degeneracy actions and bounded simplification; keep
   unsupported cases honest. Validate combined batches independently.
4. Qualify closed/open/nested/acute/complex drawings, equality thresholds, invalid
   inputs, arcs, all source modes, attachments, save/reopen, one-batch Undo/Redo,
   source reload, stale generation/export and analysis cancellation. Verify actual
   generated input, decoded output and preview agree for representative operations;
   no new post capabilities. Run relevant CAD1/V-Carve/Halfpipe regressions.
5. Desktop/keyboard/narrow review and human software acceptance. HARD STOP before
   GEOM2-A or another milestone; no physical machining qualification.

## Evidence at this audit checkpoint

Frontend existing baseline command (from `frontend/`, Node 24 PATH):
`npm test -- src/lib/state/file-transform.test.ts src/lib/state/corner-relief.test.ts src/lib/state/generation-context.test.ts src/lib/state/commands.test.ts src/lib/state/project-types.test.ts`
Result: **5 files, 86 tests pass**. This proves existing contracts only, not cleanup.
Core command: `RUSTC_WRAPPER='' cargo test --offline --release -p ivac-core --features cps --test geometry_source_ux`.
Result: **7 tests pass**. Initial runs with sccache failed with Operation not
permitted, including outside the sandbox; disabling that compiler cache succeeded.
These tests preserve existing source semantics, including known limits; they do not
prove the new identity contract. Human decision gate: **ivac-rcmta**.
No new implementation, migration, full suite, build or browser claim.

## Decision for human review

Approve the source-preserving edited snapshot + stable binding foundation, guarded
new project format and conservative repair subset above? The alternative of only
repairing an unreferenced single import avoids the format/API work but does not
meet stable multi-drawing/operation lifecycle expectations. A full CAD model and
parametric history would be substantially broader than GEOM1; neither is recommended.
