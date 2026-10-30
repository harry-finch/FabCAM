# ALPHA-3.2 — Explicit 2D Pocket Finishing

**Human acceptance and local consolidation recorded.** The bounded software scope
and three focused commits were explicitly approved. Implementation:
`6f4f8a6f0da3f0bef1ffa1b5eac93b06925c9e8f`; complete software:
`610d35d6031cb94e4b93473ce1cff661b2b85421`. All 1,308 committed software files match the qualified
fingerprint byte-for-byte. No executable source changed, so qualification was not
rerun. The documentation commit contains this notice and the
[consolidation/preservation receipt](evidence/alpha32/consolidation.json).
Earlier pending/uncommitted statements below describe the qualification-time state.
Limits remain unchanged: unsupported Spiral budget cases, non-tangent curves,
complex islands, advanced entries, equal diameters for single-surface modes,
and minimum positive allowance 0.05 mm. Installed-native, workshop and physical CNC
acceptance are not granted. No merge, push, packaging, publication, TOOL-META or
ALPHA-4. **HARD STOP after this local consolidation.**

**2026-10-30 — bounded implementation complete; 20/20 final software gates PASS.**
Human UX acceptance is pending. All implementation remains uncommitted in
`feature/alpha-3-2-pocket-finishing`, based on
`81e8d47d3ca6860303d534393bad37056ea0cfb2`.

ADR-0033 was approved, with historical executable preservation waived. The old
branch is retained where unnecessary changes could be avoided; no migration or
silent reinterpretation was needed. No commit, push, merge, installer, release,
FabSpeed, ALPHA-4, TOOL-META or physical machining work is authorized here.

## Implemented contract and motions

Project/cutting v10, preset collection v2 and template envelope v1 remain.
`pocket_finishing_v2` is a strict new nested discriminator with required scope.
`pocket-finishing-v2` is required alongside `cutting-parameters-v10` by native,
HTTP, WASM, worker and drawing-machining adapters. The backend validates data even
when a client bypasses capability negotiation. Unknown/missing/mixed scopes,
contradictory allowances and wrong operation families refuse. Existing v1 readers
refuse the new contract; no implicit fallback is used.

| Authored scope | Main target | Finishing obligation |
|---|---|---|
| Walls only | Retained stock + lateral allowance; final depth | Outer and island walls at each authored axial level; no interior clearing fill |
| Floor only | Final walls; final depth + floor allowance | Reachable floor clearing from the Main floor to final depth |
| Walls and floor | Both declared reserves | Reachable area and wall coverage at every authored finishing level |

The independently retained-for-later stock remains separate. Each stage keeps its
own authored tool, RPM, feed, plunge and depth schedule. Zero applicable allowance
is labeled as a repeat pass, not proof of reserved-stock removal or surface quality.
Finishing is still OFF on new operations. Positive allowances remain at least
**0.05 mm**. The core contains no ShopBot-specific workaround and no postprocessor
capability was broadened.

Main and Finishing are planned before either is emitted. V2 decoded validation is
mandatory even for rectangles. Byte-oriented streams buffer the verified program;
refusal/cancellation publishes zero program bytes. WASM progress callbacks may
report work completed before a later refusal; they do not publish a `done` result
or a successful program. This is bounded software validation, not machine acceptance.

## Geometry proof and error budget

This extends the existing Pocket offset planner. It does not introduce a second
CAM engine. A verification-only region represents one outer boundary and disjoint,
single-level islands. Robust topology and exact continuity are retained. Source
coordinates, identities and explicit placement data are not repaired or remapped.
For curves, existing offset input sampling uses a 0.0005 mm chord bound with an
explicit cap; independent reference/source geometry is enclosed at 0.0001 mm.
Curved reference offsets currently require tangent line/arc joins or complete
circles. Unsupported trimming refuses.

The previous **0.003 mm planned / 0.02 mm decoded** bounds are unchanged. Source
and trajectory chord errors are debited; rounded decoded arc center/radius error
is also debited before accepting coverage. Depth matching uses the existing
0.0001 mm output precision budget. Continuous interval wall coverage and a bounded
quadtree/capsule floor proof cover entire intervals/cells, not isolated probe points.
The floor exclusion uses signed-clearance Lipschitz bounds plus source enclosure.
Wall-only additionally proves the union of Main and Finishing clears the reachable
final floor; a band left between tools cannot be called complete.

Every authored axial level has its own decoded coverage set. A later full-depth
pass cannot hide an earlier missing level. Known flute length must reach the
whole authored depth and the step. Only direct vertical entries and safe rapid
moves pass the bounded access proof; a diagonal entry or a below-surface lateral
rapid refuses. Main and finish gouges, omitted floors, omitted walls and omitted
axial levels are independent refusal conditions.

Zigzag previously eroded an already cutter-compensated center domain a second
time, leaving oblique wedges. A dedicated internal emission mode removes that
second erosion only for scoped Pocket finishing. Historical Zigzag modes are
unchanged. Spiral concave examples still exceed the decoded planar proof budget;
there is no silent strategy fallback or relaxed tolerance.

## Explicit limits and remaining refusals

- One connected region, at most eight disjoint simple islands, 2–256 source
  primitives per boundary; exact nondegenerate closure. Nested islands and
  disconnected regions require separate operations or remain refused.
- Polygon corners and tangent line/arc joins; arbitrary non-tangent curved joins,
  self-intersections, touching/ambiguous containment and collapsed offsets refuse.
- Flat endmills with known usable flute length; direct entry; ordinary Cascade,
  Zigzag and bounded Spiral. No trochoidal, whirling, Halfpipe finishing, ramp,
  helix, tabs, authored approach/leads, generated frame or Boolean-source proof.
- Single-surface modes require equal effective cutter diameters. Walls-only
  lateral reserve cannot exceed cutter radius. Combined finishing can accept
  different cutters only when complete Main/Finishing geometry proof succeeds;
  this is not optimized rest machining.
- 1,000 axial levels per stage, checked before offset/motion construction and
  again against decoded output; 5,000 paths/chords per proof set; at most eight
  distinct planar level geometries per stage; 5,000,000 checked work units per
  proof invocation and cooperative cancellation. Existing resource limits remain.
  These are local product budgets, not universal memory/time guarantees.
- The six concave/irregular-concave Spiral scope cases are **correctly refused**
  at the 5,000-path limit. They are not qualified machining results.
- ShopBot keeps same-tool equal-RPM and qualified manual tool-section rules.
  Browser canonical WASM does not execute CPS; native core/CPS tests qualify the
  software output only. Installed-native UI and physical CNC are **NOT RUN**.

## UX and persistence

The surface selector shows an explicit before/after allowance preview and requires
Apply scope. Irrelevant reserves become zero only through that atomic command.
Cancel is non-mutating; Undo restores the complete prior scope and values; Redo
restores the edit. Tools, rates, stock, material and source selection are untouched.
Scope, reserves and both stage tools participate in generation/export freshness.
The report and preset summary identify scope and authored stage values; preset
matching includes the new contract and scope. No tested attestation crosses context.

Actual production-browser review uses only the committed synthetic concave drawing
on an isolated localhost origin. Desktop 1440×960 and narrow 390×844 EN/DE views
were inspected. Keyboard Tab reaches Apply scope, and the narrow preview wraps
without horizontal overflow. Both → Walls produced 6,704 moves; Both produced
17,588; Floor produced 10,951. Floor report attributes 7,292 Main and 3,657 Finishing
moves (the two remaining program moves belong outside stage intervals), with
18000 RPM and authored 811/111 versus 413/113 mm/min. All three completed the
full-program simulation; changed settings disabled export. Spiral refusal exposes
an actionable proof-budget message and no qualified download.

The real downloaded v10 project was reopened: scope, allowances, exact authored
values and source identity remained; old generated qualification was not loaded.
The fixture intentionally lacks some machine limits; those warnings remain visible.
Simulation cell coarsening is also disclosed. Neither visual stock nor a parsed
project is used as a proof of physical readiness.

[Desktop scope preview](evidence/alpha32/desktop-scope-preview.png) ·
[Narrow German preview](evidence/alpha32/narrow-de-preview.png) ·
[Actual saved project](evidence/alpha32/browser-saved-floor.ivac-project.json).

## Qualification and final inventory

**Final qualified software fingerprint (1,308 files):**
`62265b79d933d4681d70592e273e33cca9bbbafcb162909a6a3994e9b0650c42`.
Committed base remains `81e8d47d3ca6860303d534393bad37056ea0cfb2`; there is no
ALPHA-3.2 implementation commit. [Exact file inventory](evidence/alpha32/software-files.json),
[final qualification receipt](evidence/alpha32/qualification.json) and
[proposed commit inventory](evidence/alpha32/proposed-commits.json) bind the changes.

The final complete `scripts/pre-release.sh` invocation passed **20/20**, including
headless UI, CPS size, release/security boundaries and transactional export.
Two intermediate reruns honestly failed only their headless host setup: missing
Chrome alias in PATH, then macOS Chrome aborting when invoked through that alias.
The installed executable's real path in `CHROME` fixed the invocation. The entire
gate was rerun successfully; no skip or test weakening. These outcomes remain
recorded by raw-log hashes. The independently run transport harness snapshots the
native executable so Cargo feature-set rebuilds cannot mix executable bytes within
one campaign. Its receipt records the exact native and WASM hashes.


The final source fingerprint, executable hashes, exact test counts, motion differences,
representative performance and preserved-worktree receipts are recorded in
`evidence/alpha32/`. Raw logs, caches, generated executable assets and all private
input are excluded. The earlier two deliberately RED contract regressions remain
archived by hash and are now green. No unrelated validator or golden was weakened.

Executed source-bound campaigns on the final implementation:

| Evidence | Actual check |
|---|---|
| Rust workspace/all-features | 1,659 passing tests; one existing ignored test |
| Frontend | 1,549 passing tests across 179 files; one existing optional-locale skip |
| New geometry matrix | 54 polygon/strategy/scope combinations: 48 accepted, six explicit Spiral budget refusals; 12 curve/island scope combinations accepted |
| Direct safety | 14 malformed/unsupported cases; zero/minimum reserves, distinct cutters, transforms; missing floor/wall/axial-level, gouge, rapid and over-depth mutations refuse |
| Independent oracle | Analytic circular swept-path distances and floor grid, without planner/proof helpers, against LinuxCNC, GRBL and decoded ShopBot output; interval/area proofs separately cover between samples |
| Real transport campaign | 79 CLI/WASM cases: 68 accepted, 11 expected refusals; three genuine flipped recto-verso pairs (six programs) match |
| Native / HTTP handlers | Actual Tauri command functions and Axum handlers return exact core results for all scopes and reject contradictions; no installed application claim |
| Byte-oriented streaming | Exact generated bytes; refusal publishes zero bytes; pre-cancel refuses. Shared checked-work/overflow/cancellation unit regressions remain active |
| Historical regression | 23 fixtures / 24 programs, decoded moves, tool provenance, stage bindings, line indexes and stats equal accepted ALPHA-3.1 outputs |
| Browser worker / persistence | Real production build, three scopes, simulation, stale export, Cancel/Undo/Redo, actual saved-file reopen, narrow EN/DE, report, final axial-budget refusal |

[Runtime receipt](evidence/alpha32/runtime-comparison.json) ·
[Motion inventory](evidence/alpha32/motion-inventory.json) ·
[Retained corpus](evidence/alpha32/retained-corpus.json) ·
[Browser review](evidence/alpha32/browser-review.json).
Machine-program comparisons are byte-exact. Cross-runtime XYZ fields allow only
1e-12 mm native/WASM arithmetic differences; other compared fields are exact.
The three genuine two-sided cases use a simple stock X flip; advanced dowel/fixture
configurations remain unqualified. Frozen CLI/WASM readers reject all 79 new-shape
inputs; portable old-reader probes remain in the preflight receipt. Parsed current
projects, templates and presets are not reused as geometry-validity certificates.

Representative intentional motion changes (same authored input, legacy comparator
removes only the new scope and restores the old discriminator):

| Case | Old full-area contract | New explicit contract |
|---|---:|---:|
| Rectangle Cascade / Walls | 281 moves | 137 moves |
| Rectangle Cascade / Floor | 280 | 190 |
| Rectangle Cascade / Both | 281 | 281 |
| Concave Cascade / Walls | 16,330 | 6,304 |
| Concave Cascade / Floor | 16,789 | 10,093 |
| Concave Cascade / Both | 16,330 | 16,330 |

Curved/island counterparts were previously refused; acceptance now depends on the
new planned and decoded proof, not on simply removing a refusal. Numerical timing
samples are in [performance.json](evidence/alpha32/performance.json): 17.4–34.0 MB
maximum RSS, 0.02–3.10 s on the selected fresh debug CLI processes (including one
expected refusal). Concurrent qualification can affect time; no production benchmark
or universal memory claim. Complex geometry can still refuse under the budgets.

The full release gate checks 20 mandatory groups. Its generated-file drift checks
use a private temporary Git index containing the reviewed generated TypeScript
and i18n outputs; the real worktree index remains unstaged. Regeneration must
produce exactly those bytes. This permits the normal gate before any authorized
implementation commit, without bypassing the drift checks.

Proposed future commit groups, **not created**: (1) Rust contract/proof, native
transports, synthetic regression tests and generated schema; (2) frontend adapters,
controls, strict persistence, templates/presets and UI tests; (3) this sanitized
qualification evidence and milestone documentation. Dependencies and exact files
will be listed in the final inventory. Human approval is required before commits
or integration. The next milestone is not started.

## Human checkpoint — HARD STOP

**GO for human review of this bounded implementation.** Human UX acceptance,
installed-native testing and physical machining remain separate and pending.
The seven protected worktrees and main retain their exact inspected heads and
tracked state; existing private/untracked evidence is preserved. No private drawing
or derived private coordinates entered this inventory. No normal macOS application
profile was opened. [Preservation receipt](evidence/alpha32/worktrees.json).

Review the explicit scope selector, allowance preview and the qualified geometry
limits above before accepting. The synthetic review project is available in the
local production browser; it is not a machine-ready cutting recommendation.
No commit, push, merge, packaging or publication was performed. Commit approval
and any subsequent milestone are new human decisions. ALPHA-4 remains unstarted.

---

The following dated preflight is retained as historical evidence. Its pending
approval statements were superseded by the approval and implementation above.

# Historical compatibility preflight (before ADR-0033 approval)

**2026-10-30 — Phase 1 complete to the architecture gate. Implementation NOT STARTED.**
HARD STOP before the proposed incompatible nested stage contract. This is not
ALPHA-3.2 geometry, UI or CNC qualification.

## Established baseline and consolidation

The maintainer accepted bounded ALPHA-3.1 and explicitly authorized three local
commits. They were created on `feature/alpha-3-1-generalized-finishing`:

| Group | Commit |
|---|---|
| Core proof, planner integration, synthetic fixtures/tests | `59d8e233fa54facb9873b7203068ed45ad291011` |
| Runtime boundaries, harness and eligibility text | `56743de13a1f66005d7e24519fc298e66bf9c224` |
| Qualification evidence and human-acceptance documentation | `81e8d47d3ca6860303d534393bad37056ea0cfb2` |

Before staging, the full approved 44-file inventory matched. The last group also
records human acceptance in `consolidation.json` and the four existing milestone
documents; original qualification, motion, screenshot and proposed-inventory receipts
remain byte-for-byte intact. No private input, raw log, cache or executable was staged.
All **1,302 committed software files**, resolving tracked symlinks as the qualified
inventory does, match:

`1673ddb0e22994cb716b04711c4943c4635d707dc1956c6fa9901c15a6c1224c`.

No executable source changed during consolidation. The previous **20/20 gates,
1,645 Rust / 1,542 frontend tests** remain ALPHA-3.1 evidence; they were not rerun
or represented as ALPHA-3.2 qualification. The ALPHA-3.1 worktree is clean.
[Acceptance receipt](evidence/alpha31/consolidation.json).

A new isolated worktree `FabCAM-alpha-3-2`, branch
`feature/alpha-3-2-pocket-finishing`, starts at the exact documentation SHA above,
not at the earlier ALPHA-3 HEAD. Primary V1.1, ALPHA-1/2/3, TOOL-META, frozen V1 and
main retain their inspected heads and tracked state. No merge, push, installer,
release or hardware operation. [Preservation receipt](evidence/alpha32-preflight/baseline.json).

## Existing behavior and why this is a contract change

`cutting_stages.rs` owns strict `MainFinishing::V1`, mandatory Main and nullable
Finishing. Finishing stores a cutter/rates/schedule plus L and F, with no scope.
`pipeline/op_drivers/staged.rs` constructs both execution views through the same
existing Pocket fill builder: Main uses R+L and Z+F; Finish uses R and Z.
Both invoke the complete wall/floor proof. It does not select boundary-only paths
when F is zero. Changing that interpretation changes a valid saved program.

The qualified synthetic `rectangle-cascade-0.05-0` request was regenerated on the
exact baseline binary. Its program SHA256 remains
`6dc9dc4f812c373d5d7d7087722aad333865af4c846eeb26e6a66d5e6a313222`.
The finishing interval contains **112 interior cutting movements** by the recorded
rectangle witness, despite F=0. This is a characterization of full-area legacy
behavior, not a new proof of axial wall removal or physical surface quality.

ALPHA-3.1's polygon, Zigzag coverage, Spiral/budget, source-arc and island limitations
remain exactly those in its [accepted report](ALPHA_3_1_GENERALIZED_FINISHING.md).
The architecture gate is reached before modifying predicates or broadening eligibility.
No unsupported result has been reclassified as correct machining.

## Reader audit and executed negative probes

The actual reader modules were inspected, not just the generated schema. Nine
relevant modules are byte-identical between accepted ALPHA-3 and ALPHA-3.1:
Rust stage/preset/cutting validation, WASM boundary, frontend stage/project/file
readers, templates and preset-snapshot validation. Their hashes are in the baseline
receipt. Earlier v9 readers already reject outer v10 under ALPHA-3's archived
qualification; that historical test suite was not rerun here.

Executed **9 compatibility scenarios** against unchanged frontend reader functions
(TypeScript transpilation only), the exact qualified CLI executable and real WASM.
Executable/WASM SHA256 values were checked against ALPHA-3.1's runtime receipt.
[Executable probe](evidence/alpha32-preflight/reader-probe.mjs) and
[actual outcomes](evidence/alpha32-preflight/readers.json).

| Probe | Observed outcome |
|---|---|
| Existing v10 / `main_finishing_v1` | Portable accepted; CLI and WASM program bytes equal |
| Add `scope` inside existing Finishing | Refused by Rust, frontend, real WASM, presets, templates and snapshots |
| Unknown discriminator alone | Explicitly refused, including WASM before execution |
| Unknown discriminator plus scope | Refused through regular/streaming/two-sided WASM; no callbacks/output |
| Same unsupported contract on disabled operation | Refused, not silently skipped |
| Add `finishingScope` beside portable operation fields | Accepted as an unrelated extension; does not define machining semantics |
| Add `finishing_scope` beside native params | Rust JSON refuses; generic WASM conversion ignores it and emits the unchanged v1 program |
| Hypothetical outer project/cutting 11 | Old portable, native and WASM readers refuse |
| Historical L>0, F=0 Pocket | Exact qualified old program; full-area finishing remains |

Unknown nested stage data is refused; a side field outside that guarded boundary
is unsafe. The first exploratory assertion assumed the native side field would be
ignored too; actual native JSON refused it. The final probe records the real
native/WASM distinction. No production validator or test expectation was weakened.
The distinction is already documented in the existing cutting contract's general
WASM unknown-field limits; it is not authorization to use such ignored fields.

Portable validation leaves its input unchanged. `restoreProject` calls validation
before document mutation; that ordering was inspected. This audit does **not** claim
a new actual-browser save/reopen or installed-native test. HTTP uses typed JSON and
Tauri typed `PipelineRequest` before `require_current`; those entry points were
inspected, not separately invoked in this compatibility audit. Full transport and
browser acceptance belongs to the approved implementation's final qualification.

## Recommended narrow version separation — approval required

[Proposed ADR-0033](../shopbot-modernization/adr/ADR-0033-explicit-pocket-finishing-scope.md)
contains the complete proposed contract and invariants.

| Boundary | Recommendation |
|---|---|
| Portable project | Retain v10, with strict nested discriminator |
| Native cutting envelope | Retain `cutting_version: 10` |
| Historical Profile/Pocket | Preserve `main_finishing_v1` and exact executable behavior |
| New Pocket scope | Add **`pocket_finishing_v2`** with required typed Walls/Floor/Both scope |
| Capability negotiation | Add **`pocket-finishing-v2`**; require it for the new variant |
| Tested collection | Retain v2; nested contract/scope included in matching and snapshots |
| Operation template | Retain envelope v1; strict nested guard preserved |
| Drawing / diagnostic snapshots / CAM geometry | Unchanged |

No project v11, cutting v11 or collection v3 is currently necessary: the existing
strict nested readers demonstrably reject the incompatible shape. This is still a
**new machining contract**, so it requires approval before implementation. If any
actual supported reader can bypass its guard, stop and propose an outer increment.

The new scope must be a Rust-authoritative tagged value, not inferred from L/F.
Both allowance fields remain explicit; the irrelevant allowance must be exactly
zero and its input is hidden. A contradictory request refuses. Source coordinate,
rate, numeric and output budgets do not change. New parsed data is never treated
as a geometry or material-removal certificate.

Existing operations stay v1 on open/save, shown as full-area existing behavior.
Only an explicit reviewed, atomic Undo/Redo command switches to a new scope and
invalidates generation/export. Original files remain available; no automatic rewrite.
Presets/templates retain original contracts. A change of scope is a different cutting
context and cannot inherit historical physical-test claims. The current hard-coded
v1 preset signature and rebinding/template constructors must be extended deliberately;
otherwise they could discard or mislabel the new semantics. ALPHA-4 remains excluded.

## Consequences for implementation after approval

Extend the current planner rather than replacing it. Keep the historical branch
for v1 output preservation. Add distinct v2 obligations for Main, wall band and axial
height, reachable floor, tool access and actual decoded paths. Walls-only must
produce boundary paths, not a relabeled clearing fill. Floor-only and combined may
reuse existing clearing with separate obligations. Original ShopBot rules remain.

A bounded proof change must explain each formerly refused Zigzag/Spiral case;
no global tolerance/budget increase. Curves and islands require source primitive,
containment, reachable-region, safe-entry and independent decoded proofs. No claim
of these extensions is made at this preflight. Source curves, islands, optimized
rest machining and axial wall coverage are not newly qualified here.

Required follow-up evidence: frozen old-reader rejects; v1 exact output and
save/reopen preservation; v2 no-loss round trips across all transports; malformed,
mixed and stale-state refusals; distinct scope motion/oracle mutation tests; UI
Undo/Redo/narrow/keyboard review; actual canonical release gate. Installed-native,
physical and publication acceptance remain separate.

## Handoff and status

Only this preflight's documentation and read-only probe/evidence are changed in
ALPHA-3.2. No production, schema, project-version or postprocessor change; no
ALPHA-3.2 commit. The probe's hypothetical inputs are negative test data, not an
implemented v2 contract. See issue `ivac-pbq2m`.

**GO for the established ALPHA-3.1 baseline and completed compatibility audit.**
**NO-GO for ALPHA-3.2 implementation until ADR-0033's nested-contract decision is
approved.** HARD STOP. The next authorized step after that decision is Phase 2:
true wall-only finishing with lateral/axial/access proofs, followed by the remaining
bounded phases. ALPHA-4, TOOL-META, FabSpeed, installers and hardware are not started.
