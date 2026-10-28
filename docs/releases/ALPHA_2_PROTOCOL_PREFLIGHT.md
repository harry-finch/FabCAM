# ALPHA-2 — source-bound counterexamples and protocol decision

2026-10-28. **STOP at the explicitly requested protocol gate; NOT implemented or qualified.**
Tracking: `ivac-4sylp.1`. Bounded architecture approved; this report requests only
its transport/persistence refinement, not renewed approval of local repair policy.

## Source and preservation

Isolated worktree `FabCAM-alpha-2`, branch `fix/alpha-2-geometry`, based on
`4d0bf277c4023acc980f39accf8c8a40b77dbd13`. Production source unchanged.
Only new diagnostic example/tests and this report added. No implementation commit.
Original alpha branch, its untracked plan/evidence, primary V1.1 and TOOL-META
worktree preserved. Primary/TOOL-META tracked trees inspected clean.
The private DXF hash still matches the previously recorded receipt. Input was read
only. Source-derived coordinates are stored only in a new local temporary JSON,
not a distributable fixture or tracked report. No native application launch.

## Executed counterexamples, before predicate changes

`cargo test --offline -p ivac-core --test alpha2_baseline`:

- Two deliberate failing regressions: wrong first-contour attribution; safe local
  gap repair blocked by an unchanged distant crossing. These remain red, not
  ignored or weakened. ALPHA-2 cannot be declared complete.
- Two passing regressions: independent crossings/overlaps/touches still refuse
  generation; exact closure after explicit snap/connector at 0.001x/1x/1000x,
  rotation/translation, stable contour ID and original endpoints unchanged.

The source-bound `alpha2_probe` uses the actual importer, drawing evaluator and
existing full topology validator. It isolates pairs with their original adjacency
semantics, preserving primitive coordinates and transforms. Temporary clone IDs
are diagnostic only, never project identities or a production repair algorithm.

The private input reproduces ten approximate closures that are not exact. It also
contains **three proper segment crossings**, independently confirmed with exact
rational determinants over the imported binary floating-point coordinates. These
are not tolerance-only false positives. Two involve the first/last edges around
an approximate closing junction; another is a nonadjacent self-crossing. Fixing
only endpoint gaps cannot be assumed to fix the drawing. No automatic crossing
repair is proposed. Detailed indices and rational-oracle evidence remain local in
`dist-test/alpha2/`, without private coordinates in this report.

## Why a protocol decision is needed

1. `Finding.contour` is a required `Identity`. A global issue cannot be represented
   truthfully as `null` or as a multi-edge diagnostic within that type. Existing
   frontend code dereferences it. Changing its type in place breaks readers.
2. `DrawingRequest` uses `deny_unknown_fields`; Preview accepts only geometry,
   repairs and bindings. It carries neither operation/placement state nor a
   source-revision receipt. Additional required safety context is not understood
   by the old reader. No optional/default-empty placement proof is acceptable.
3. One `CONTRACT = 1` currently serves the drawing model, transport, evaluation
   result and generation. The frontend stores the whole response in the project.
   `validateProjectGeometry` requires snapshot, response and drawing revisions 1.
   A blanket increment would accidentally make existing geometry unreadable.
4. `resolve_edited_objects` separately refuses enabled edited operations with
   tabs/approach placements. Preserve that CAM guard: permitting safe editing and
   preserving placement data will not silently qualify machining with placements.
5. Frontend stale protection already includes operations and drawing state, but
   its captured key must also account for all source-resolution context (including
   work offset/text/auxiliary geometry used in Start). An echoed token alone is
   not a backend proof of unaffected placement ownership.

Three new reader-boundary tests establish required-contour rejection of null,
unknown placement-context rejection, and explicit rejection of revision 2.

## Recommended revision — awaiting approval

Separate the constants and responsibilities rather than renumbering geometry:

| Contract | Existing | Proposed |
| --- | --- | --- |
| Stored geometric primitives/identity/lineage | drawing v1 | Keep v1, unchanged semantics |
| Existing evaluation/preview request path | protocol v1 | Retain existing strict global behavior for v1 |
| New diagnostic/scoped preview path | absent | Explicit protocol v2; no fallback to v1 |
| New saved scoped-edit snapshot | snapshot/response v1 | snapshot/response v2 containing drawing v1 |
| Edited CAM generation | generation v1 | Keep v1 and all full-drawing/placement guards |
| Cutting project format | v9 | Keep v9, with mandatory geometry-revision validation |

New v2 diagnostics identify a typed reason, actual contour/edge identities when
known, and completeness/budget state. Global issues have no invented identity.
V2 preview requires the relevant operation and placement context with source
revision, returns the verified before/after dependencies and revision, and never
returns a CAM eligibility claim. Rust recomputes safety from authoritative data;
frontend applies only while the captured complete project context still matches.
Persisted receipts are not trusted as future CAM proof.

Old v1 snapshots stay readable with their original semantics; an explicit new
repair creates v2. No bulk migration, destructive conversion or implicit downgrade.
Old frontend readers already reject snapshot/response v2; old backends reject
request v2. Verify those exact rejection paths in real frontend/native/WASM tests.
If inspection proves any old supported reader bypasses the nested revision guard,
stop again before saving and propose a project-version increment. Do not assume
that retaining the outer v9 is sufficient without that evidence.

Alternative: retain v1 unchanged and limit this lot to read-only diagnostics in a
separate API. That does not deliver the approved scoped repair/placement workflow.
Do not smuggle v2 acceptance behind the old success response or encode machine-
readable diagnostics in strings.

## Validation limits and remaining work

This is pre-implementation evidence, not integrated software qualification.
No predicate, continuity threshold, schema, transport, frontend, CAM or output
changes have been made. Browser/WASM integrated acceptance is NOT RUN for ALPHA-2.
Full scoped proof, containment/adjacency witnesses, cancellation/budgets, placement
conflict regressions, stale apply, Undo/Redo and persistence remain to implement.

Initial existing-test invocation without `--all-features` hit `CpsUnavailable` in
one drawing test; retained as an invocation failure, not a production defect.
Corrected command:
`cargo test --offline -p ivac-core --all-features --test alpha2_protocol --test geom1_contract --test drawing_audit`.
Result: **39 PASS / 0 FAIL** (3 reader-boundary, 23 GEOM1, 13 drawing-audit).
The separate new baseline target remains **2 PASS / 2 expected FAIL**.
Local logs and source-file SHA-256 hashes are in `dist-test/alpha2/receipt.json`.

Proposed eventual commit inventory (none authorized or created):
1. Protocol/structured diagnostics and compatibility regressions.
2. Exact continuity and scoped backend repair proof, with semantic regressions.
3. Proven-safe placement preservation, revision-bound frontend apply and UI review.
4. Source-bound integrated qualification and documentation.

HARD STOP: approve the protocol-v2 / snapshot-v2 / drawing-v1 separation before
implementing incompatible transport or persistence changes. No ALPHA-3/4,
TOOL-META, postprocessor change, installer, integration, push or physical machining.
