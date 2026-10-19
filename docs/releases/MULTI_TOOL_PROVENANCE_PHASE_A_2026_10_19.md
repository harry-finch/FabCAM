# ivac-b0js3 — Multi-tool provenance, Phase A

Date: 2026-10-19. **ARCHITECTURE PROPOSED — implementation NOT authorized.**
Reviewed current dirty workspace at HEAD `66f229ea21cccf73a76fd45d0e9f9fe3721a41a1`, beta.3/pipeline62, not HEAD alone. Argon is review input, not an accepted specification. This checkpoint changes documentation/evidence only. Existing review gates and unsupported workflows remain open.

## Recommendation

**EXTEND** generated movement provenance with a nullable, typed, result-local tool reference and a compact immutable tool-binding table. Carry planned identity through the existing post interface/IR; qualify emitted intent in owned output adapters. **KEEP** actual emitted ShopBot movements as preview/simulation authority, the Rust simulator, independent ADR-0024 verification, safety guards and project formats. **DEFER** broad performance work, arbitrary CPS output qualification and advanced two-sided acceptance.

An ADR is necessary: this changes a shared response contract and the meaning of a qualified movement. [ADR-0025](../shopbot-modernization/adr/ADR-0025-movement-tool-provenance.md) records the proposal. No fallback from missing provenance to `op_id`, first tool, geometry, or tool ordering is acceptable for qualification.

## Argon findings: independently checked

References are repository-relative paths and current source line anchors; symbol names remain authoritative as lines evolve.

| Finding | Verdict | Evidence and consequence |
|---|---|---|
| `ToolpathSegment` loses tool identity | **Confirmed** | `crates/ivac-core/src/gcode/preview.rs:67`: only op/line identity. Native interpreter tracks OP, not tool selection/activation; CPS `ir_preview` at `gcode/cps_recorder.rs:722` discards section tool; ShopBot decoder at `pipeline/shopbot_resolution.rs:10` tracks only OP. |
| Rough/finish and spot share an operation | **Confirmed** | `pipeline/op_drivers/dual_tool.rs:29`, `drill.rs:35,150,233,370`; CPS `mid_section_toolchange` at `gcode/cps_recorder.rs:348` splits sections retaining op ID. A drill can use **three** tools (spot, drill, chamfer), not at most two. Declaring a finish tool does not guarantee an emitted finish: offsets may have no finish ring. |
| Playback carves finishing/spot with primary tool | **Confirmed** | `frontend/src/lib/components/Scene3D.svelte:753` resolves by op only; fallback is `project.data.tools[0]`, not necessarily first enabled tool. Wrong cutter/holder can hide or create collisions. Exact example magnitudes/holder contact were not geometrically reproduced here. Full verifier intentionally refuses declared distinct finish/spot tools (`state/program-qualification.ts:59`): known fail-closed limitation, not verified-safe output. |
| Dowels invent a new tool absent from the project | **Refuted** | `pipeline/two_sided_emit.rs:300` now requires exactly one declared matching Drill with explicit rates and valid tip, then uses its ID. Existing test passes. |
| Synthetic dowel op absent from verifier map | **Confirmed**, conditional | Synthetic op ID still allocated at `two_sided_emit.rs:324`; frontend map is built only from enabled user ops. Missing op fails rather than silently qualifies. Earlier geometry/post/back-fixture guards may reject first; “all jobs systematically fail here” is too strong. This is **not** a fail-closed bypass. |
| Cache only keys by segment index | **Refuted** | `sim/sweep.rs:1612,3499`: fingerprint includes segment, tool, holder, fixtures and grid context. Changed-tool regression passes. Prefix-material changes still require invalidation; this is not a proof that all Scene3D lifecycle edits are correct. |
| Auto grid uses first enabled tool | **Confirmed** | `program-qualification.ts:69`, `Scene3D.svelte:787`, `sim/config.ts:71`: Ø25.4 gives requested 1.693333 mm versus Ø1.5 → 0.1 mm. Reordering UI ops changes selection. Backend `group_ops_by_tool` alone does **not** change this frontend selection: that extra allegation is refuted. Collision misses in the proposed example remain unmeasured. |
| Qualification-key race after editing | **Refuted as stated** | `qualificationKey` checks actual generation context; `ProgramQualification.svelte` cancels on null; `qualification-run.ts` rejects old responses. Existing edit/stale/export tests pass. |
| Program-only operations fail tool lookup | **Partially confirmed** | Leading enabled `toolId=0` without a corresponding tool rejects immediately. An emitted segment stamped with another unmapped program-only ID also fails. A no-motion Pause/Marker later in a program need not fail. Homing/Probe already rejected for CPS (`pipeline.rs:1487`); arbitrary includes are not generally qualified. This is a lookup/coverage problem, not the alleged race. |
| CPS tool number is exactly project identity everywhere | **Partially confirmed** | Owned bridge `pipeline/cps_ctx.rs:288` assigns `number=t.id`, recorder copies it. Generic `ivac-cps::ir::ToolSpec` has no project identity guarantee; arbitrary IR/posts are separate. |
| ShopBot decoder supports M2/M3/CG and lab C9/&Tool | **Refuted** | Decoder accepts JZ/J2/M3 plus the bounded control subset; unsupported commands error. `crates/ivac-cps/posts/shopbot.cps:75,100,198` requires manual M0 strategy; lab mode only preloaded flat tool 1, TR/SO, **no C9 or tool changes**. Arcs are linearized. |
| TOOL comment + diameter closes semantic equivalence | **Refuted** | A comment cannot establish swap execution; diameter does not identify cone, effective wear, flute/shank/holder. Even correctly labelled planned IR does not prove final movement or transition equivalence. |
| Add serde default plus op fallback is safely compatible | **Refuted** | It can turn missing/contradictory identity into a plausible wrong cutter. Deserialization compatibility is not qualification compatibility. “40 Rust +15 TS lines” is unsupported. |

## Current end-to-end provenance and exact loss boundaries

1. **Project**: `project/tool.rs:49` defines `ToolEntry.id:u32`. There is no separate magazine number field in this model. `Op.tool_id`, finish and spot references select a definition. `build_tool_index` (`pipeline.rs:1548`) collects by ID into a HashMap; duplicate IDs can overwrite. First-match frontend lookup is different. No positive/nonzero guarantee should be inferred from `u32` or normal UI allocation.
2. **Planning**: `setup_resolver.rs:301` uses tool ID as setup number. Dual/drill drivers select actual tool roles. `emit_toolchange_envelope` (`pipeline.rs:3017`) knows new identity; native strategy chooses output. For scaffold-owning CPS it calls `mid_section_toolchange` instead. Machine-change station moves and retractions occur at boundaries: the **outgoing** tool occupies the retract before the change.
3. **Native**: final text is parsed (`pipeline.rs:945`). LinuxCNC default emits `Tn M6`; custom templates may replace it (`gcode/linuxcnc.rs:426`). GRBL tool emission can be a no-op (`grbl.rs:212`). `Ignore` still retracts/stops but emits no swap signal (`pipeline.rs:3233`). Interpreter drops identity even when output contains it. `Tn` selection alone must not mean activation.
4. **CPS**: owned ToolSpec number currently equals project ID, but ToolSpec omits much simulator geometry (holder, shank, full form profile, etc.). Section splitting retains tool but preview extraction discards it. `preview_with_line_sync` (`cps_recorder.rs:802`) zips line numbers on equal segment count; this is **not** movement/tool semantic equivalence. PassThrough and RapidMachine are omitted from generic IR preview.
5. **Bundled ShopBot**: pipeline selects `resolved_preview` for the bundled ID (`pipeline.rs:927`), decodes actual rounded moves/ordered rapid axes. It currently ignores tool-load comments. Manual `onSection` retracts, emits load/zero instruction plus PAUSE, then spindle instruction; lab branch permits only preloaded tool 1. No output marker proves physical execution.
6. **Consumers**: full verification and interactive rendering independently reconstruct by op. `verify-program.ts:40` uses previous tool only for op0 and otherwise errors; Scene3D uses first-tool fallback. Generated TS/OpenAPI, WASM Simulator deserialization, fixture checks and all ToolpathSegment constructors must be audited when extending the type. Save/reopen stores source (`state/project-file-ops.ts:144`), not authoritative generated qualification.

## Authoritative identity model

- **A — project definition**: `ToolEntry.id` selects a definition within a project. It is not a globally unique physical asset tag.
- **B — planned tool**: concrete generation-time definition bound to a movement/phase. Capture immutable geometry, including wear/tip/holder, from the planner's actual selection.
- **C — emitted intent**: output adapter confirms which declared tool the program requests or assumes preloaded, and where that intent changes. Maintain an explicit project-ID ↔ program-number binding, even though the owned bridge currently uses equal numbers. Never infer equality for third-party programs or physical magazine contents.
- **D — mounted hardware**: unknown to FabCAM. Manual load, M6, PAUSE, a controller macro or operator acknowledgement is a procedural assumption, not measured state.

B is authoritative for planned simulation; C checked against B is required to label a supported final program verified. On disagreement, reject qualification and emit a specific diagnostic; do not repair it by trusting B or comments. Preserve an inspection-only planned preview with an unverified label if useful.

## Proposed movement/API contract

Recommended conceptual shape (names finalized in implementation review):

```
ToolpathSegment.tool_ref: Option<GeneratedToolRef>
GeneratedToolRef: nonzero u32, local to one generated result/side
PipelineResponse.tool_provenance:
  revision
  authority: planned | resolved | unavailable
  bindings: [{ ref, project_tool_id, program_tool_number, tool: ToolEntry }]
  transitions: [{ output_line, before_ref?, after_ref, evidence_kind }]
  diagnostics
```

Rules:

- References resolve uniquely into immutable full tool definitions; table sorted deterministically, refs allocated in first actual use order. No collisions across front/back tables because each result owns its namespace. Same op can use multiple refs; synthetic ops need no frontend operation lookup.
- `None`/missing field means **unknown**, never “use previous” or “tool absent.” Zero is invalid as a generated reference; project ID zero, if encountered, remains an explicit project ID, not a sentinel. Reject duplicate project IDs/bindings rather than silently renumbering saved data. An explicitly absent cutter would need a separately supported process contract; not introduced for V1 milling.
- Every simulated milling movement, including rapids/retracts, needs a known body envelope. No-motion comments/pauses have no ToolpathSegment and require no fictitious tool. A program-only movement may retain a known active tool only through the backend's explicit modal state. Unsupported probe/homing/raw changes invalidate coverage rather than borrowing the next tool.
- Missing provenance can deserialize for legacy wire inspection, but is unavailable for new-contract qualification. No `opTools` fallback. Explicit contradictory/missing ref errors cannot be replaced by another lookup.
- Initial unknown position remains unknown as in the current ShopBot decoder; record initialization coverage limitations. A known initial/preloaded tool is a declared assumption. Do not retroactively assign incoming tool to pre-swap retractions or initial unbound motion.
- Carry planned section/span identity before postprocessing (unique execution ordinal, **not** reused op_id). Validate it against output; a tool_ref becomes resolved only after that check. Transitions are sparse, not a second motion graph.
- Generated response/schema/TS/WASM evolve together. Bump pipeline/cache and qualification revisions, include provenance version/bindings/output identity/effective simulation config in keys. Old API/WASM combinations fail visibly; old generated caches regenerate. No persisted-project migration, crate/bundle rename or tool-library renumbering needed.

Alternative: `tool_id:Option<u32>` plus a generation-time definition map and separate evidence can work for current project-only tools. It saves one indirection but couples generated identities to project IDs and complicates future non-project definitions. Bare `u32=0` with fallbacks is rejected. A full new machine-event pipeline is unnecessary. Proposed result-local binding is the modest extra scope recommended for explicit identity/geometry/evidence separation.

## Native/CPS/OpenSBP propagation and resolved verification

Add a small typed tool-intent boundary to the existing post abstraction, reused by boundary, rough/finish and spot/drill/chamfer paths. Place activation **after** the outgoing-tool clearance/change boundary. Cache replay must preserve the same events and order as cold generation. CPS sections carry project binding/unique ordinal explicitly; do not treat section.tool.number as a universal project foreign key.

For owned native posts, retain text-decoded movement authority and add bounded recognition of their actual tool-change semantics. Standard LinuxCNC T/M6 and owned manual-pause envelopes can be qualified independently; GRBL/custom templates/HPGL do not acquire capabilities from a marker. No-op toolchange, missing pause, unsupported offsets or custom macro semantics yield planned-only/unavailable coverage. A single-tool `Ignore` run may declare an initial preloaded tool under a documented contract; **multiple planned tools with Ignore cannot be qualified as changed**. Do not automatically split files or add pauses.

For bundled ShopBot, extend only its owned adapter/post metadata, keeping the existing command allowlist. Prefer a versioned structured section/binding marker over parsing human prose. Validate marker identity/order/uniqueness and expected tool binding against the generation record, then validate the existing load instruction plus PAUSE/clearance sequence (or the already-restricted preloaded lab tool1 branch). A marker alone, removed PAUSE, contradictory number, moved boundary or unrecognized command must fail. Manual declarations remain explicit operator obligations. Do not add C9, CG, M2, ATC or new V-bit/Engrave permissions.

Decoder movements, line numbers and feed/dwell remain those of actual output. Verify transition location and motion ownership against expected sections, allowing only documented post-added clearance and arc/cycle expansion with existing rounding/chord tolerances. Do not zip by equal counts, infer sections from geometric similarity, or compare diameter alone. The binding carries full simulator geometry; post-facing effective diameter/type/angle are cross-checked within output precision. This verifies software intent, not the dimensions of the mounted cutter.

Third-party CPS remains **planned IR preview**, unless separately qualified by an output adapter. Missing/modified metadata is not silently trusted. User raw comments/includes cannot forge trusted provenance: reserve/escape the owned marker namespace and reject duplicate or unexpected markers. Owned emitter provenance plus expected transition validation is required, not authentication by comment spelling. Existing unsupported include/machine-coordinate/probe restrictions remain.

## Simulation geometry, transitions and resolution

Both Scene3D and full verifier consume the same pure ref→immutable definition resolver. Full verification still owns independent completion and diagnostics; playback Home/End never changes export authority. Replace whole-field/checkpoint state on any relevant generation, binding, tool geometry or resolution change; the warning cache fingerprint alone cannot repair a changed earlier material-removal prefix.

Reuse Rust `ToolProfile::from_tool` (`sim/heightmap.rs:568`) and `HolderProfile::from_tool` (`sim/holder.rs:94`): effective wear diameter, V tip/angle, flute length, shank and holder all matter. Never build holder geometry from a rounded emitted diameter. Preserve unknown-body diagnostics and SAFE-Z publication guards. The retract before a tool swap is checked with the outgoing holder, subsequent movement with incoming holder; same-number/different-definition bindings must not silently merge.

Recommend auto resolution derived from the smallest **effective cutting diameter among referenced cutting tools**, shared by both consumers and independent of UI op order. This is a heuristic improvement, not continuous collision accuracy: pointed V-bits, fine form features and large stock caps invalidate any diameter-only assurance. Keep manual resolution and existing cell budgets/coarsening warnings; record requested/actual cell size, actual allocated cells and cap. Check integer grid rounding against cap during implementation qualification. Do not change budget/accuracy policy merely to pass a provenance test.

## V1 scope matrix (proposed; no new qualification awarded)

| Workflow | V1 treatment |
|---|---|
| Single-sided flat-tool rough/finish within existing ShopBot contour/pocket subset | Required provenance correction and decoded-output tests before lifting the current refusal. |
| Multiple ordinary supported milling/drill operations, manual tool loads | Required: inter-op and transition envelopes as well as intra-op identity. |
| Spot then drill with tools already allowed by selected post | Required generic/native semantics; ShopBot only its existing supported pairing. |
| V-bit spot or V-bit post-drill chamfer | Generic planned/native test coverage; current ShopBot drill+V-bit remains refused. No blanket V-bit/Engrave unlock. |
| Pause/Marker without motion | Must not invent a tool or reject a following valid program merely because first op has toolId0. Preserve active binding across no-motion events. |
| Homing/Probe/raw includes/custom macros | Retain capability refusals and explicit incomplete coverage; not solved by adding tool identity. |
| Manual M0 / manual M6 / ATC | Test each owned supported output separately. M6/ATC is not thereby added to ShopBot. Physical execution remains unqualified. |
| Ignore with different tools; arbitrary third-party CPS | Planned-only, unavailable final-output qualification. No automatic repair. |
| Existing simple front/back without fixtures/dowels | Preserve supported bounded behavior and separate side results; no new acceptance claim. |
| Synthetic dowels, back fixtures, advanced two-sided registration | Explicit future qualification. Representation should preserve existing declared drill identity, but correct lookup alone does not qualify stock flip, fixture coordinates, through-drill/registration or headers/line offsets. |
| Relief ball nose, T-slot/form undercuts | Simulation/performance stress only where supported; do not expand ShopBot allowlist for benchmarks. |

## Performance assessment and bounded measurement proposal

No large-job latency/RSS measurements were obtained in this Phase A. None of Argon's numeric targets is accepted.

| Claim | Classification |
|---|---|
| 256 chunks repeat tool deserialize, profile build, diagnostics, CSR refresh | **Plausible optimization**, repeated calls confirmed at `ivac-wasm/src/sim.rs:372,406`; no demonstrated performance failure. Diagnostic accumulation must remain complete. 51,200 same-op segments produce 200 calls, but op boundaries may increase count. |
| CSR sorts/allocates every advance | **Plausible optimization**, `sim/dexel.rs:541` confirmed. Empty top-down sidecar is cheap; form-tool cost needs measurement. Logging call exists; 200 browser console lines depend on logging configuration. |
| JSON snapshot then postMessage clone costs 150–300 ms /60 MiB | Copy paths confirmed (`ProgramQualification.svelte`, `qualification-run.ts`); numeric claim **unsupported**. Deep Svelte proxy count is not established. |
| Interactive + worker doubles memory | Coexisting allocations confirmed; exact RSS doubling **unsupported**. Meshes, up to12 playback checkpoints, WASM heaps, arrays and back surfaces exceed the simple 4 MiB dense-grid estimate. |
| Worker initialization / compilation caching makes overhead acceptable | **Plausible optimization/measurement topic**; no measured acceptance. New worker/module initialization exists, browser cache behavior is not a guarantee. |
| Cache corruption on tool edit | Alleged **correctness issue refuted** for cache key; broader playback prefix invalidation remains a regression requirement. |

Bound benchmark campaign: representative 300×200 2.5D plaque (~4.5k moves), full-sheet 2440×1220 flat-tool nesting (~35k, **Direct or supported Ramp**, no unsupported helix), 150×150 relief (~180k) and small form-tool undercut (~2k). Last two are generic simulator stress cases, not ShopBot qualification. Use fixed saved inputs and actual emitted segment counts; add repeated-256-boundary and late-collision cases.

Run browser/server and packaged macOS first, Windows11/Ubuntu when available; cap250k and configured native cap; 2D versus active3D; cold run plus five warm runs; two-sided separate. Measure wall time from generation completion through snapshot, worker startup/import, allocation, sweep and publication; main-thread long tasks, cancellation latency, process peak RSS/JS/WASM heaps, allocation after10 regenerate/cancel cycles, actual cells, warning counts, deterministic output, Home/End stability. Set thresholds only after baseline, retain existing product resource contracts. Isolate one optimization at a time; chunk-size/CSR/snapshot/persistent-worker work needs its own evidence and approval, not bundled into provenance.

## Minimal implementation phases after approval

1. Establish red semantic regressions and typed contract, immutable bindings, duplicate/unknown handling, codegen and revision invalidation. No policy unlock.
2. Propagate planned identities through existing native/CPS hooks and cached replay. Qualify owned emitted transitions, bundled ShopBot decoded movement correspondence and malformed-output refusals. Keep unsupported variants refused.
3. Share resolver across interactive/full simulation, use bindings for grid selection, handle program-only events. Remove specific blanket multi-tool refusal **only** when full coverage and post evidence are present. Keep independent lifecycle/export guards.
4. Integrated software tests, visual review and bounded performance baseline. Update evidence, then HARD STOP. Advanced two-sided and performance refactors remain separate.

## Required semantic regressions

- Ø8 rough → Ø2 finish: compare carved wall and a fixture/holder contact only the short finishing tool causes; verify outgoing-tool retract and incoming-tool approach. Empty finish ring must not invent a swap.
- Spot→drill→chamfer: three distinct bindings under one op; exact cone/cylinder swept cross-sections, depth/feeds and change locations. ShopBot rejects disallowed V-bit drill pair; do not rewrite fixture to weaken guard.
- Wear, shank/flute/holder differences with equal nominal diameter; missing body geometry retains incomplete diagnostics. Tool reuse A→B→A and same-op transitions.
- Missing/zero/duplicate refs, duplicate project IDs, missing tool definitions, contradictory marker/number, deleted pause, omitted change, injected fake marker and tool selected without activation all refuse qualification.
- Initial preload, every supported manual strategy, multi-tool Ignore refusal, pre-change retract/unknown startup; pauses/markers at start/middle, unmapped synthetic op with a declared drill, unsupported homing/probe/include retained.
- Native text versus CPS planned IR versus owned ShopBot **decoded output**: independent decoder checks coordinates, feeds, ordering and tool changes; rounded output and expanded cycles/arcs, cold/warm cache and streaming parity. Same motion count is not an oracle.
- Legacy absent provenance deserializes but remains unverified; mixed old WASM/backend/frontend refuses; save/reopen keeps source IDs/settings/unknown fields; regenerate restores new runtime evidence.
- Auto min-effective diameter and operation reorder; manual override, cap rounding/coarsening, V-bit feature limits, resource exhaustion and worker error/cancel.
- Tool/holder edits stale immediately; delayed old reply rejected; same-result Home/End/arbitrary scrub leaves complete diagnostics and export unchanged; native Save-dialog race; front/back independent identity.
- Preserve TEST1, SAFE-Z, ShopBot M2, CPS, V-Carve and existing frontend lifecycle/export suites; no relaxed tolerances or unsupported post pairs.

## Phase A evidence and remaining decisions

[Evidence directory](multi-tool-provenance-2026-10-19-evidence/README.md): **37 existing frontend tests PASS**, including real WASM; **5 existing Rust tests PASS**; read-only counterexample probe reproduces leading tool-less lookup and unknown synthetic-op lookup, and computes first-tool resolution dependence. Probe mock is not geometry qualification. No permanent fixtures or production source changed; no UI acceptance, full-suite rerun, native package, physical test or benchmark claim.

Human approval requested for ADR-0025: result-local nullable tool reference plus immutable binding/evidence table; explicit planned versus resolved authority; no legacy/op fallback; multi-tool Ignore remains unqualified; preserve narrow V1 post scope and defer advanced two-sided/performance. Alternative optional project-ID field is viable only with the same binding/evidence and safety rules; **bare field stamping is insufficient**.

**HARD STOP.** ivac-b0js3 remains open; implementation, commit, push, packaging and publication are not authorized by this report. Earlier ivac-3i00w/UX review gates are not closed by this architecture work.
