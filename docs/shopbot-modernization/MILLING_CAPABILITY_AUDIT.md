# Milling capability audit and track redirection

Date: 2026-10-13. Source: `96e41ec4e8a3b593579079cbb6c051e070cd03ea`.
Scope: implementation/test inspection and planning, not feature implementation.
Human next-milestone approval: **ivac-s7zo**. FS4: **ivac-mwry**, deferred.

## Current direction

FS3.8 supplies the software architecture for later calibration. Freeze further FS
expansion: no extra manufacturer imports, preset tuning, math changes, FS UX redesign
or physical tests during this phase unless another explicitly approved feature
requires a narrowly scoped integration. External source evidence remains
ReferenceData; no Sorbonne Tested/Validated claim without actual machining.

FS4 — Physical Feeds & Speeds Validation
- Status: **DEFERRED**.
- Reason: requires physical lab / ShopBot access, tools and representative materials.
- Blocking further software development: **NO**.
- Resume: existing FS3.8 capture, private storage, resolver, history and deliberate
  validation workflow remain usable; no architecture redesign is required.

This is neither failure nor blocker. Milling CAM audit is authorized now; the next
implementation milestone still requires approval. Existing historical FS human
review records are retained, not silently closed or made dependencies of milling.
WJ/ProtoMAX and VINYL remain deferred.

## Classification and evidence limits

READY means the stated software subset has implementation plus relevant analytical
or semantic coverage; it does not certify every option or physical machine result.
PARTIAL means a useful implementation has scope/support/qualification gaps.
MISSING means no dedicated capability was found in the searched CAM/state/UI paths.
NEEDS UX WORK is an available backend capability or behavior poorly exposed to
users. BUGGY requires demonstrated wrong behavior, not merely an untested feature.
NEEDS PHYSICAL VALIDATION is a separate machine-evidence limit; physical follow-ups
are deferred and do not block software work. No newly reproduced numerical CAM bug
is claimed in this static audit. Known cache-test race is recorded separately.

## Current milling feature audit

| Feature | Classification | Implementation and tests inspected | Scope / gap and priority |
| --- | --- | --- | --- |
| Profile | READY | `cam/offsets`, `gcode/emit.rs`; M1 square/rectangle/circle inside/outside/on-line, open-path and disconnected-path invariants; M2 square/circular SBP goldens | Standard 2.5D compensation and depth schedule are established. M4 physical evidence is a specific MDF exterior square, not every option. Keep; P0 regression baseline. |
| Pocket | PARTIAL | `cam/offsets/pocket_fill.rs`, `op_properties/PocketSection.svelte`; M1 concavity/hole/island/stepover/finish-allowance corpus, M2 island SBP | Cascade subset is qualified; zigzag/trochoidal/dual-tool interactions have distinct semantics and warnings and are not covered by the same ShopBot physical proof. Keep/extend qualification; P0 entries, P2 advanced strategies. |
| Drill | READY | `pipeline/op_drivers/drill.rs`, `snapshots_drill_tabs_whirl.rs`; M1 centers/depth/pecks, M2 peck SBP | Simple/peck common subset exists; spot/chamfer, multiple tools and other cycles need their own scoped qualification. Lab spindle mode forbids tool changes. Keep; P1 productivity, later physical evidence. |
| V-carve | PARTIAL | `cam/vcarve.rs`, `cam/vcarve_emit.rs`, `pipeline/op_drivers/vcarve.rs`, VCarveSection; medial-axis distance, holes, narrow-chord and ratchet/ramp tests | Actual variable-Z medial-axis engine, not merely a UI. Closed regions and V-bit geometry required; outside-text frame is an existing deferred issue (ivac-si66). Dedicated Vectric/ShopBot comparison and physical evidence absent. Keep; P2. |
| Ramp / entry | PARTIAL | `gcode/entry.rs`, `gcode/emit.rs`, `pipeline/setup_resolver.rs`; generic helix/retract and ramp/tab snapshots | Lines split at ramp boundary; arcs do not split mid-arc. Unfittable helix falls back to ramp. Generic varying-Z circular records conflict with current ShopBot planar-arc preflight. P0, proposed UX1. |
| Climb / conventional | READY | `cam/offsets/winding.rs`, `cam/offsets/mod.rs` winding tests, OpPropertiesPanel | Closed inner/outer/island paths account for spindle handedness; open/on-line cuts deliberately skip winding enforcement. Separate finish direction exists; trochoidal behavior must be read with its warning. Keep; P0 regression matrix, not blanket physical approval. |
| Tabs | PARTIAL | `cam/tabs.rs`, `cam/offsets/tabs.rs`, `pipeline/tabs.rs`, `gcode/tabs.rs`; auto/manual/mixed, short-count/dedupe, rectangular/ramped snapshots and frontend tests | Substantial existing feature. End-to-end ShopBot combinations with curved contours, entry fallback, ramped tabs, short/open contours need explicit invariants; trochoidal tabs have a warning/restriction. Keep/extend; P0 qualification, P1 placement UX. |
| Lead-in / lead-out | PARTIAL | `gcode/leads.rs`, ProfileSection; free-side geometry, arc fit and straight fallback tests | Fit checks do not constitute general stock/island/fixture avoidance; open/concave/tight contours and tab+entry combinations need emitted-output proof. Keep; P0 qualification with entries, P1 intent preview. |
| Stock definition | NEEDS UX WORK | StockPanel, `sim/footprint.ts`, `state/stock-outline.ts`, `api/build-project.ts`; driver/manual-centering, outline and stock-gizmo tests | Manual offsets are relative to geometry bbox center, not coordinates of the stock corner. Zero offsets can correctly draw displaced stock. Dimensions/placement/material are distinct. Existing corner readouts help, but origin naming/defaults still deserve a focused setup milestone. P0. |
| Work origin / WCS | PARTIAL | StockPanel, `buildWorkOffset`, `pipeline/warnings.rs`, CPS context; wire tests for XYZ and G55/G56 | Generic offsets persist, but ShopBot MVP supports one operator-established work zero (workOffset 1). UI availability of G55–G59 is not ShopBot support. Need contextual capabilities and shifted-XYZ preview/SBP agreement. Keep/extend; P0. |
| Machining warnings | NEEDS UX WORK | `pipeline/warnings.rs`, GenerateBar, `warning-display.ts`, sim diagnostics; warning rendering and cache-hit tests | Aggregated pipeline/sim chip, go-to and configurable export gates exist. Critical gates are not a proof of physical safety; stale/not-run/coarsened sim must be distinguishable. Ramp-arc wording needs reconciliation with actual emitter behavior. P0. |
| Dogbones / T-bones | MISSING | Search across crates/frontend for dogbone/tbone/t_bone and operation/geometry forms | No dedicated generator/interactive corner relief found. Imported CAD relief is a workaround. Add as geometry preparation, not ShopBot post behavior, after base entry/coordinate proof. CAD1, P1. |
| Spoilboard surfacing | PARTIAL | `pipeline/op_drivers/surface_mill.rs`, ReliefMillSection, `cam/surface_mill.rs`; varying-Z ballnose relief test | Existing relief raster requires a source grid and ballnose; face_mill_overlay is a spiral overlay, not a bounded planar spoilboard operation. Pocket on a stock rectangle is a workaround. Dedicated flat-cutter/overtravel/stock-region workflow missing. P1. |
| Operation templates / reusable presets | MISSING | `state/project-operation-ops.ts` duplicateOperation and command history; tool presets and FS presets inspected | Duplicate exists; named reusable CAM operation templates with safe geometry/tool/material rebinding do not. Tool defaults and cutting-knowledge records are not operation templates. P1, after stable setup/entry semantics. |
| Toolpath preview | READY | `gcode/preview.rs`, CPS recorder IR preview, 2D/3D renderers | Displays rapid/cut/arc paths with operation attribution; IR preview may differ in representation from linearized SBP. Must compare emitted geometry semantically; screen appearance alone insufficient. Keep; P0 regression, P1 better entry/tab intent. |
| Simulation | PARTIAL | `sim/heightmap`, `dexel`, `sweep`, rapid/holder checks; analytic swept-arc, stock removal and collision tests; frontend driver tests | Grid/representation-dependent verification, adjustable/coarsened resolution; missing tool-holder geometry weakens checks; back-side export currently pipeline-only. Not digital-machine certification. P0 truthful verdicts; P2 refinement. |
| Fixtures / keep-outs | NEEDS UX WORK | `project/fixture.rs`, `sim/fixture_check.rs`, `state/project-entity-ops.ts`, fixture hit tests; StockPanel explicitly removed add UI | Box/cylinder/polygon obstacle data, CRUD commands and collisions exist; ordinary Stock workflow no longer exposes creation. Collision detection is not automatic rerouting/keep-out avoidance. P0 restore a compact accessible declaration/review workflow; do not rewrite collision backend. |

Paths in this table are relative to `crates/ivac-core/src` for Rust modules and
`frontend/src/lib` for UI/state modules. M1/M2 test files live under
`crates/ivac-core/tests`. Existing capability is upstream-origin code unless the
fork documentation explicitly records a ShopBot/FS addition. Do not replace mature
kernels merely because their UI needs work.

## Gaps found and recommended priority order

1. **P0: qualify entries for the actual ShopBot post.** Trace: generic entry emits
   circular XYZ → CpsRecorder::record_arc retains target Z → ShopBot preflight
   rejects `r.end.z != p.z` before onCircular linearization. Planar XY arcs are
   linearized at the existing post tolerance, but that does not make helixes
   supported. This is a traced support gap; add a minimal real-pipeline fixture
   before choosing a fix. No failure was newly executed in this audit.
2. **P0: stock/origin/capability clarity and fixture declaration.** One physical
   operator-established zero, stock corner vs bbox-centered offsets, safe-Z frame,
   unsupported WCS choices and missing ordinary fixture-entry UI have stronger
   dependencies than productivity features. Existing simulations need explicit
   obstacle geometry and reliable coordinate setup.
3. **P0/P1: tab/lead/warning end-to-end qualification.** Existing unit/snapshot
   evidence is useful but does not prove each combined emitted ShopBot path.
   Show requested vs resolved entry and avoid a false clean/not-run/stale verdict.
4. **P1: dogbones/T-bones (CAD1)** for sheet-stock assembly, then bounded flat
   **spoilboard surfacing** with explicit margins/overtravel and work area checks.
5. **P1: reusable operation templates**, preserving explicit current cutting
   values and requiring safe geometry/tool rebinding rather than copying stale
   source IDs or private-source snapshots.
6. **P2: broader V-carve, pocket strategies, dual-tool finishing and simulation
   accuracy/performance**; **P3** optional advanced convenience improvements.

Known BUGGY infrastructure: ivac-htc8 is a global pocket-cache test race; run affected
Rust regression suites serially. ivac-5g51 can accept partial WASM build outputs;
require a complete verified package before desktop/browser delivery. Neither issue
alone demonstrates wrong profile or pocket geometry. Do not expand FS as a detour.

## Dependencies between features

- Source geometry + explicit stock/WCS frames underpin entries, fixtures, preview
  and emitted-output comparisons. A drawn preview is not a substitute for them.
- Supported generic movements → post capability agreement → meaningful ShopBot
  entry/tab/lead corpus → trustworthy warnings/export state.
- Reliable source corner topology + cutter radius → dogbones/T-bones. Reuse geometry
  representations, keep topology epsilon vs path/output/physical tolerances distinct.
- Stock footprint + work envelope + flat-cutter capability + safe-Z conventions →
  bounded spoilboard surfacing; relief finishing does not supply that contract.
- Stable operation parameter/context ownership + undo/persistence + deliberate tool
  and geometry rebinding → operation templates. FS3.8 is sufficient infrastructure;
  physical FS4 is not a dependency of any software milestone above.

## Suggested next milestone

**UX1 — ShopBot-compatible milling entries and explicit fallback qualification.**
Small scope: Profile and cascade Pocket Direct/Ramp/Helix intent, actual generated
entry and ShopBot supported output; contextual controls and fallback messages.
Use the existing geometry → CAM → generic path/IR → post boundary. Treat coordinate
setup only as existing test preconditions; do not bundle a full Stock redesign,
fixture editor, dogbone generator, surfacing and templates into UX1.

First reproduce the helix support mismatch in a minimal real core/CPS fixture.
Then decide the smallest extension: valid capability-driven lowering to generic XYZ
segments at an explicit error bound, or clearly constrain unsupported requests until
support is qualified. Never insert ShopBot commands into generic geometry or turn
an unsupported helix silently into a vertical plunge. Improving common ramped
entries is the minimum useful output; full helical support must be scoped explicitly.
No new shared model or ADR is assumed necessary; assess serialization/schema,
simulator, generic posts and CPS consumers before any such change.

## Suggested acceptance criteria for UX1

- **UX1-01**: minimal square/circular/concave-pocket test drawings expose current
  Direct/Ramp/Helix behavior through actual CAM/CPS. Include open paths, holes/
  islands and too-small entry space; retain the existing shifted-origin regression; define expected results
  before altering implementation.
- **UX1-02**: supported entry reaches final depth, respects configured pass depth,
  contains cutter swept volume in permitted region, has no below-clearance XY
  rapid and no hidden fallback. Hole/island safety must be proved, not inferred
  from a helix planner that only sees one boundary's vertices.
- **UX1-03**: emitted `.sbp` movements and generic preview agree within documented
  path/linearization tolerance, including Z on ramp/helix. Keep numerical/output
  tolerance separate from unperformed physical finish/dimension validation.
- **UX1-04**: impossible/unsupported entry yields a clear op-scoped explanation,
  explicit safe resolved strategy or rejection. G55–G59 remain unsupported for
  ShopBot MVP; no silent controller-offset approximation or unknown machine limit.
- **UX1-05**: prove selected climb/conventional direction on closed inside/outside/
  island contours and retained tab material across relevant entry/lead combinations;
  no wholesale direction/tab rewrite. Include deterministic repeated generation.
- **UX1-06**: compact Adobe-style inspector shows requested/resolved entry and
  contextual advanced controls; no new card/database/parallel warning framework.
- **UX1-07**: save/reopen, manual cutting values, undo/redo, cache-hit warnings and
  existing M1/M2/M4 output invariants survive. Run meaningful geometry/semantic tests,
  serial cache-sensitive suites, frontend checks and complete WASM build verification.
- **UX1-08**: software handoff ends at human approval. New physical capability
  claims remain NEEDS PHYSICAL VALIDATION until lab access returns; FS4 stays deferred.

## Validation performed for this audit

No application code changed; no Rust rebuild or new machine test performed.
Inspected implementation and actual test assertions, including M1 analytical
compensation/coverage/pecks and M2 SBP semantics; historical results remain in
M1_QUALIFICATION, M4_VALIDATION and FS3_8_QUALIFICATION, not a fresh broad pass claim.
M4 is limited operator-reported MDF square evidence and VCarve comparisons.

Fresh frontend checks, 2026-10-13: **124 pass in 9 files**.

```sh
PATH=/opt/homebrew/opt/node@24/bin:$PATH npm run test -- src/lib/api/build-project.test.ts src/lib/state/stock-outline.test.ts src/lib/canvas/stock-gizmo.test.ts src/lib/cam/tabs.test.ts src/lib/canvas/fixture-hit.test.ts src/lib/components/warning-display.test.ts src/lib/sim/warnings.test.ts
PATH=/opt/homebrew/opt/node@24/bin:$PATH npm run test -- src/lib/sim/driver.test.ts src/lib/services/gcode-export.test.ts
```

Commands run from frontend; first suite 89 pass, second 35 pass. These establish
frontend data/selection/warning/export helper contracts, not all Rust CAM semantics.
Documentation links and whitespace checked. No screenshot or physical-evidence
promotion used as proof. No commit/push authorization for this new checkpoint.

## Documentation updated and hard stop

ROADMAP, STATUS and DEVLOG now record nonblocking deferred FS4, frozen FS expansion
and milling priority. This audit, validation index and document index provide a
fresh-session handoff. Historical qualifications stay intact.

**STOP — await human approval (ivac-s7zo) before starting UX1 or any other next
milling implementation milestone.**

## UX1 scope clarification — continued 2026-10-13

UX1 qualifies milling entry strategies against ShopBot post capabilities and
establishes preview-to-machine-output consistency. Preserve existing engineering
work; this milestone covers CAM and post compatibility as well as inspector UX.
Requested strategy differs from machine representation only through explicit,
deterministic resolution reflected in final preview. See ADR-0007 and UX1
qualification for authoritative revised acceptance/evidence.

“Test fixture” above means sample geometry. Square/concave/pocket-island drawings
remain necessary CAM geometry tests. Real fixtures (clamps/obstacles/keep-outs),
creation/editing, stock reference redesign, numeric WCS transformation repair and
collision workflow acceptance are CAM2/UX2. UX1 retains already-written coordinate
safeguards/regressions without expanding them. Full entry/tabs/leads combination
qualification is also deferred; current behavior must not regress and known unsafe
combinations must reject.

## Future milling plan update — 2026-10-13 (documentation only)

UX1 software qualification remains as recorded; this update does not alter its
[evidence](ux/UX1_QUALIFICATION.md) or reopen work. Remaining Material Analysis /
Rest Material Visualization is a new **planned, not implemented** diagnostic CAM4
feature. Existing stock-removal simulation and intended Pocket fills do not claim
this generated-path residual-region capability. CAM2/UX2 remains next, followed by
CAM3 broader tabs/leads qualification, then CAM4 analysis/overlay and coordinated
CAD1 dogbones/T-bones. Retain earlier UX3/UX4 surfacing/template names for later work;
V-carve qualification and advanced Rest Machining remain future capabilities.

[Remaining material plan](REMAINING_MATERIAL_PLAN.md) records geometry/path-source
questions, depth/stock/reference dependencies, intentional-vs-accidental residuals,
Phase A/B/C and provisional acceptance. Analysis must use actual generated/resolved
machining geometry where appropriate; final machine residuals should preserve UX1
execution-path parity. Generic/target distinctions and exact representation require
future geometry/architecture audit, not an implementation decision here.
