# GEOM1 — approved refinements and implementation contract

2026-10-15. User approval attachment aeab0819-c7c9-4bef-aa5e-775d8793cb35.
ADR-0021 core direction accepted; phases A–F authorized. No publication authorized.
This contract refines the audit; it does not reopen the architectural choice.

## Identity and binding

Identity is `(import ID, entity ID)`; entity IDs are monotonically allocated within
an import and never recycled, including deleted entities. Original source segment
ordinal is provenance within an immutable source revision, not an editable identity.
Contour membership uses explicit segment identity, not endpoint reconstruction.
An unchanged or uniquely modified contour retains its ID and provenance. Merge
creates a new ID with all parent contour IDs. Split creates new IDs with their
parent; deletion retains a tombstone. Lineage transitions are validated against
before/after membership and cannot be invented by the frontend.

| Transition | Binding rule |
| --- | --- |
| Unchanged / unique modified | Preserve explicit contour binding and operand order. |
| A+B→C | Initially always require explicit reselection for any explicit binding to A/B, even if both were selected. This conservative rule avoids proving boolean operand equivalence. |
| A→B+C | Require reselection; no automatic choice of one or both children. |
| A deleted | Mark binding unresolved; never replace with empty list meaning All. |
| Other import | Preserve import/entity identity exactly; numeric array positions are not authority. |

All/Layers remain dynamic scope selectors and require preview of their changed
scope. Unresolved explicit bindings remain persisted, visible and fail generation
and current-output eligibility. Disabled operations retain their unresolved state;
reenabling cannot silently manufacture a valid association. Explicit reselection
creates a new binding through the same owner command. Manual rates/depths/tools
are not modified. Legacy positional bindings must be proven using actual chaining
membership/provenance before conversion; unsupported ambiguous legacy selection
requires reselection. Endpoint equality alone is forbidden as identity evidence.

## Effective drawing and transport

Rust owns preparation, lineage, repair decisions and evaluated contour membership.
The frontend sends input/proposals and displays Rust's evaluated result; it contains
no duplicate snap/join/simplification mathematics. Source coordinates and immutable
pivot → accepted local edit → file similarity transform → applicable relief →
authoritative drawing. Selection, bounds, source resolution and generation use that
same evaluated result with explicit work-origin adapters. A stale asynchronous result
cannot replace a newer drawing. CAM re-evaluates/validates saved geometry instead of
trusting a display cache. Legacy drawing generation remains on its current path.

Stable contour membership must survive the generic source-resolution boundary;
whole-pool fuzzy chaining cannot reinterpret edited IDs. Shared numeric adapters
can map stable contour identities to ephemeral CAM indices only inside Rust, with
one mapping consumed by preview and CAM. Native/server/WASM must expose the same
contract revision, initially `geom1-v1`. No fallback to unedited segments on missing
capability or transport failure. Generation requests for edits must also contain a
required recognized discriminant so an old Rust deserializer rejects them, not just
an optional unknown field that old serde would silently ignore.

Relief and affected tabs/approach are initial explicit Apply conflicts. No automatic
relocation or rebasing of those features. The new format can preserve their data
without implying that every interaction is qualified. Synthetic stock/text/pattern
sources need explicit mapping or refusal; no guessed binding.

## Coordinates and tolerances

Input geometry is local mm. User distances are post-file-transform drawing mm,
before WCS. For a uniform scale `s != 0`, local distance limit is `limit_mm/abs(s)`;
translation/rotation/mirroring preserve distances. Evaluate threshold distance as
`hypot(local_delta) * abs(s) <= limit_mm`, avoiding subtraction of two large world
coordinates where possible. Exact equality is included. Zoom never enters requests.
The immutable original bbox center supplies pivot even after extrema are removed.
Nonuniform scale is absent from the existing FileTransform; any future unsupported
matrix is rejected, not approximated. Nonfinite/singular transforms or numeric
uncertainty are refused. Large/small finite factors require reference coverage and
output-resolution checks before qualification, not arbitrary epsilon widening.

Join, closure and simplification tolerances remain separate. Exact duplicate tests
compare full retained primitive semantics with reversal, including layer/provenance.
Simplification requires a continuous bidirectional deviation bound and topology
checks; imported-curve approximation remains a separate unknown/error contribution.
No global CAM fuzzy/flattening/output tolerance changes. Physical accuracy unproven.

## Persistence and compatibility

Registry currently uses versions 1–6. Reserve `ivac-project-geometry`, version 7,
with edit contract revision 1. Validate both kind and version and every required
original/edit/provenance/lineage/binding/transform field. Unsupported revisions,
inconsistent memberships and unsupported feature combinations fail explicitly.
Versions 1–6 retain previously supported reading and original unedited save-kind
selection; no mass migration. v7 preserves all compatible operations, physical
tools, profiles, cutting knowledge, surfacing, relief and historical compatible
fields. Do not silently drop unknown data: preserve inert extension data where
safe; reject unknown geometry-affecting features/revisions. Legacy code currently
constructs snapshots field-by-field, so this requires deliberate round-trip tests.

Old readers reject the new kind/version. Old backends reject the edited-source
request discriminant and capability negotiation. Test actual old-contract decoding,
not only JSON parse success. v7 cannot downgrade automatically. External source
changes preserve accepted snapshot and expose a conflict; no automatic replay on
replacement. Explicit source replacement/reset is a reviewed undoable action.
History remains session-local: reopen restores accepted state, then subsequent
Apply/Undo/Redo must work; this does not promise restoring a prior session's stack.

## Implementation and test matrix

| Phase | Implementation | Independent evidence required |
| --- | --- | --- |
| A | This contract, ADR and compatibility/lineage rules | Source registry/API review; no ambiguity resolved by geometry guess |
| B | Rust identities, lineage, evaluator and source adapter | Unique modification, merge, split, deletion, unrelated import; coordinate/membership equality |
| C | Analysis/proposals/Apply lifecycle, bounded repairs | Exact/below/above gaps; snap vs connector; duplicates/points/open/acute/complex cases; whole-batch validation and cancellation |
| D | v7 validation, legacy load/save, backend feature guard | Representative v1–v6 plus v7 round trips, old reader/backend refusal, fields preserved, external replacement conflicts |
| E | Compact Drawing workflow and common output guards | Actual screenshots, keyboard/narrow UI, preview=commit=CAM, Undo/Redo, stale export, decoded output/resolved preview parity |
| F | Results and docs | Actual command/results and remaining limits; human acceptance HARD STOP |

Numeric fixtures include rotated 90° and arbitrary-angle, translated, mirrored,
0.001×/1000× scales and independent transforms, with threshold-neighbor cases and
same geometry at different zoom levels. Region/operation fixtures include nesting,
Difference operand order, live text/stock context and multiple identical imports.
Regression includes CAD1, V-Carve, Halfpipe and ShopBot refusal/output guarantees.
No GEOM2, primitives, nesting, new CAM/post capability or machine qualification.

No operator decision remains on merge/split/delete policy: conservative unresolved
binding is explicitly permitted by the approval. Any newly discovered case where
this contract cannot prevent wrong-source machining must stop implementation of
that behavior and be escalated with evidence.

## Implemented conservative subset (qualification refinement)

The accepted line subset and evidence are in GEOM1_RESULTS. Optional simplification
includes the documented 64-ULP-scale arithmetic margin, protects previously edited
edges, and has a separate 200-segment candidate budget. Exact repeated closed loops
inside a single legacy chain create a new survivor identity and require explicit
binding review. Curve-overlap topology is unanalysed/refused rather than guessed.
The legacy fuzzy interpretation is used only as a generation refusal check; it never
reassigns authoritative IDs. A synthetic stock scope is explicitly refused until
its stable adapter is qualified. Changes to accepted source transforms require
reset/reanalysis in this GEOM1 implementation; automatic rebasing is not implied by
the architecture. Dedicated generation is buffered; cancellation discards its result
without claiming the independent PERF cancellation-latency gate.
