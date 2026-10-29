# ALPHA-3 — Explicit Main Cutting and optional Finishing

## Implementation update after approval

The user approved ADR-0032 and the v10/v2 policy. Implementation is IN PROGRESS,
not blocked on another approval. The initial typed Main/Finishing model and numeric
validation are added in `cutting_stages.rs`; they are not yet wired to generation
or persistence. The remaining preflight text and receipt describe the unchanged
starting baseline, not current implementation qualification. Full pipeline, reader,
UI and motion qualification remain outstanding. No commits.


Date: 2026-10-28. **Contract audit / versioning checkpoint; implementation NOT DONE.**
Tracking: `ivac-4sylp.3`. Recommendation: **NO-GO for new machining behavior until
the proposed persistence decision is approved**, then proceed with the bounded
implementation. This is not final ALPHA-3 software or human acceptance.

## Source and isolation

Branch: `feature/alpha-3-main-finishing`, dedicated `FabCAM-alpha-3` worktree.
Starting/current HEAD: `96181814c6f97ac19390dade7a0d3110b2f4d681`, descendant of
accepted ALPHA-2 software `58ddde72295a01cbe43ce0270c98e1f79f2f1039`.
Version remains `0.9.0-beta.3`. Only audit tests and documentation are added;
production Rust, frontend, schemas, lockfiles, posts and existing fixtures are unchanged.
No implementation commit, merge, push, installer, application launch or hardware use.

Primary V1.1, ALPHA-1/2 and TOOL-META stay separate. The preflight receipt records
their heads and tracked status, the inherited ALPHA-2 fingerprint, new test hashes,
reader hashes and local evidence hashes. Private drawings are neither loaded nor
copied by these tests. `socle.dxf` remains diagnosed, **not repaired or CAM-ready**;
ALPHA-2's twenty refused probes and three real crossings remain unchanged evidence.

## Actual current source behavior

| Boundary | Verified behavior / implication |
|---|---|
| Creation | `op_defaults.ts` + `seedCutting` create Rough/Finish slots for Profile and ordinary Pocket. This is not an explicit finishing toggle. |
| Roles/validation | `cutting_parameters.rs` and frontend `cutting-parameters.ts` require Profile Rough + Finish; incomplete/invalid assigned rates refuse. Removing unused Profile Rough still refuses. Backend `nocontour` instead requires Rough only; frontend ordinary Pocket roles do not reflect that backend exception. |
| Setup/header | `setup_resolver.rs` resolves explicit roles; Profile program header selects Finish. Creation defaults do not replace missing authored generation rates. |
| Profile compensation | `offset_builder.rs` marks every Profile offset Finish, including Inside/Outside/On. `gcode/emit.rs` selects Finish RPM/feed/plunge for all its axial passes. Rough is required but does not drive these contours. |
| Axial schedule | `gcode/z_schedule.rs` uses step/depth list; `finish_step` inserts a thin final Z interval, still on the same contour/rates. It is not a separate wall/floor finishing stage. |
| Retained stock | `stock_to_leave_mm` increases the effective compensation radius for Inside/Outside and Pocket roughing. Profile does not remove it; On-contour ignores it. **Pocket with a distinct finish cutter uses that cutter's radius without adding retained stock**, unlike its single-tool branch; the new contract must remove this ambiguity. |
| Entries/holding | Existing setup/finalization feeds tabs, authored approach, leads, ramp/helix and SAFE-Z into the shared contour emitter. Adding a second cutter cannot assume those placements remain physically equivalent. |
| Pocket geometry | `pocket_fill.rs` generates the compensated boundary plus strategy fill. Without allowance/distinct finish cutter, the boundary itself is tagged Finish; fill uses Rough. With allowance it emits rough geometry then a dedicated final boundary. |
| Pocket strategies | Cascade uses nested rings; Spiral stitches rings into a continuous open fill; Zigzag uses raster strokes; Trochoidal has an independent engagement-loop fill. Common wall tags do not make these equivalent material-removal strategies. |
| Islands | The pipeline inflates island boundaries for cutter clearance, with additional cascade handling. Final walls and island walls need independent allowance/finish reach verification; no new island qualification is claimed here. |
| `nocontour` | Skips the wall ring, requires only Rough in Rust, ignores XY finish allowance with a warning. This is not the desired complete Main-only wall/floor contract. |
| Distinct tools | `op_drivers/dual_tool.rs` partitions existing rough/finish offsets; a real safe toolchange envelope separates them. A second tool requires actual reachable finish offsets. |
| Simulation/provenance | Decoded movement carries `op_id`, `tool_ref` and output line; tool provenance is generation-local. There is no independent Main/Finishing stage reference. Same-tool stages cannot be identified merely by tool. |
| Report | `report-cutting.ts` deliberately filters Profile to Finish, reflecting current emission rather than the unused Rough slot. Screen/export share this rate formatting. It must change with actual stage emission. |
| Persistence/transports | Portable frontend v9 and native `cutting_version:9` are different envelopes. Native/HTTP/WASM/worker clients check capability `cutting-parameters-v9`; each needs an explicit new contract check. |
| Freshness | `generationContextKey` includes the built machining project plus drawing/context. Every new stage value must reach that wire model so stage edits invalidate generation and export. |
| Presets | Current v1 collections validate explicit role values and exact context signatures. Changing the operation/stage contract must not retain old applicability or attestation by accident. |

The source review used current ALPHA-1/2 reports, original alpha feedback in the
separate ALPHA-1 worktree, ADR-0030, `CUTTING_CONTRACT.md`, `TESTED_PRESETS.md` and
FS-RESET-3 results. The requested **FS-COMPAT-OPS audit was not found** in the
available FabCAM files. No conclusions are attributed to that missing document.
`fab-design-system` is not available in the supplied skills; existing components
and the actual CAM UX skill remain the UI reference. This branch currently ships
EN/DE locales, not FR: the affected ALPHA-3 FR/EN controls need real locale wiring
and explicit fallback disclosure, not a claim that the whole application is French.

## Source-bound baselines

`crates/ivac-core/tests/alpha3_preflight.rs` authors synthetic requests from the
existing CLI-v9 rectangle fixture. LinuxCNC is used for deterministic rate/geometry
inspection; bundled ShopBot is separately exercised for its refusal/section behavior.
Synthetic values are diagnostic inputs, **not machining recommendations**.

| Probe | Measured result on the starting software |
|---|---|
| Profile Inside/Outside/On | Authored Rough 11000 rpm / 811 feed / 111 plunge; Finish 13000 / 413 / 113. Output uses Finish; removing Rough refuses. |
| Desired Main-driven Profile | **Expected FAIL**: F811 never drives the contour. |
| Desired Profile without unused Finish | **Expected FAIL**: deleting Finish refuses. This describes the missing product capability, not a valid new format fixture. |
| Outside retained stock 0 → 1 mm | Decoded cutting X bounds change 9..36.5 → 8..37.5 mm; no nominal finishing return. |
| Inside retained stock 0 → 1 mm | X bounds 11..34.5 → 12..33.5 mm. On stays 10..35.5 mm. |
| Pocket retained stock 1 mm, allowance 0.4 | Same-tool final X bounds 12..33.5; distinct Ø1 mm finishing tool reaches 10.5..35 (cutter edge at the original wall), removing the supposedly retained stock. `dual_tool_finish_radius` explains the difference. |
| `finish_step = -0.1` | Z passes -0.25/-0.5 become -0.25/-0.4/-0.5; same XY envelope and Finish rates. |
| Cascade, allowance 0 / 0.4 | 119 / 132 decoded moves, both Rough and Finish rates. |
| Zigzag, allowance 0 / 0.4 | 82 / 95 decoded moves, both rates. |
| Spiral, allowance 0 / 0.4 | 98 / 111 decoded moves, both rates. |
| Three ordinary strategies with `nocontour` | Only Rough; allowance 0 and 0.4 produce identical executable output, with ignored-allowance diagnostic for 0.4. |
| Trochoidal, allowance 0.4 | 42,498 decoded moves, Rough plus implicit Finish wall. This is characterization, not authorization of ALPHA-3 trochoidal finishing. |
| ShopBot same cutter / differing RPM | Refuses unsupported intra-section `spindleSpeed`. |
| Existing distinct cutter sections | Generates with two resolved tool bindings and differing RPM under the existing manual-section contract; no new post capability. |

Move counts characterize different output shapes; they are **not coverage proofs**.
Synthetic request/response pairs and raw logs stay in ignored
`dist-test/alpha3-preflight/`. Their hashes are in the sanitized receipt.
The two failing tests remain active, not ignored or weakened. The current test
suite is therefore intentionally RED pending implementation. No golden is changed.

## Reader-boundary decision requiring approval

Recommended policy: [Proposed ADR-0032](../shopbot-modernization/adr/ADR-0032-explicit-main-and-optional-finishing.md).

| Probe against the current old reader | Frontend portable loader | Native Rust JSON | Real WASM JS generation |
|---|---|---|---|
| Add operation stage flag, retain old slots | Accepts | Accepts, drops flag | Accepts, old program |
| Add nested field beside `cutting.rates` | Refuses | Refuses | **Ignores it, old program** |
| Unknown mandatory `main` role | Refuses | Refuses | Refuses |
| Replace cutting shape, omit old required `rates` | Refuses | Refuses | Refuses |
| Outer project v10 | Refuses before restore mutation | Different envelope | Different envelope |
| Native `cutting_version:10` | Different envelope | Generation refuses | Generation and streaming refuse; zero stream events |

The existing real-WASM preset JSON reader also accepts an empty v1 collection and
refuses the proposed v2 envelope. Actual new staged entries remain to be implemented
and qualified after approval.

The actual `project-v9.ts`, frontend cutting validator and Rust cutting validator
are byte-identical at integrated `1f353ef`, primary `34f4169`, ALPHA-1 `4d0bf27`
and this ALPHA-2 descendant. The runtime probes use the current descendant,
including a freshly built WASM package; this is not four installed-app trials.
Both native-file and browser-file loaders call that frontend validator before
replacing state. Native deserialization is tested directly; **installed Tauri not run**.

Keeping v9 with a mandatory replacement shape is demonstrably possible at the
tested boundaries. We do **not** claim v10 is mathematically the only safe design.
We recommend v10 because it names the changed machining contract explicitly,
protects at the whole-document boundary, and avoids two incompatible cutting
contracts behind the existing v9 capability. A discriminator simply added next
to old slots is insufficient, particularly in WASM.

Approval requested for:

1. Portable project **v10**, native cutting **v10**, distinct transport capability;
   drawing v1, ALPHA-2 v2 snapshot/diagnostics and CAM drawing envelope v1 unchanged.
2. Explicit refusal of v9 project loading in the new editor, original bytes kept;
   inspect with the preserved ALPHA-2 app and configure a new v10 project. No
   automatic values/stage conversion, overwrite, or silent original-path save.
3. Preset **collection v2** for new stage data. Read/preserve v1 history, retain
   exact matching for unaffected families, refuse applying old Profile/Pocket
   roles to new stages. New staged entries go into a new collection, not an
   automatic rewrite. No ALPHA-4 reuse or new physical attestation.

New-reader behavior is a **proposal, not implemented/tested**. Mandatory follow-up
tests: actual v10 save/reopen, malformed/missing discriminator, disabled operations,
mixed shapes and role/cutter mismatch, old-file no-mutation/no-write, actual old
reader rejection, all native/WASM/HTTP/CLI/worker and drawing/two-sided/streaming
entry points, v1/v2 collection round trips and refusal of false applicability.
ALPHA-3 §7 says to implement an increment only after approval; §24 requires a stop
for an unapproved breaking version strategy. No persistence change was made.

## Proposed bounded execution model

**KEEP:** imported/drawing identities, ALPHA-2 repair proofs, stock/work-origin
behavior, robust offsets, contour entries/tabs, safety validators, posts and export
publication guards. **EXTEND:** stage planning, actual rates/tools/schedules,
movement attribution, simulation/report consumers, freshness, schema and UI.
**REPLACE:** Profile/Pocket implicit Rough/Finish semantics. **DEFER:** unsupported
special finishing combinations, ALPHA-4, TOOL-META and FabSpeed integration.

- One required Main stage, with explicit selected cutter, integer RPM/feed/plunge,
  applicable depth schedule and radial engagement. No fallback to old Finish.
- Optional Finishing is absent/OFF on creation. Enabling exposes an explicit cutter,
  independent rates/schedule and applicable allowances. Incomplete values refuse;
  disabling removes all finish-only influence on geometry/output/provenance.
- Independent retained stock is the target of this operation when explicitly
  enabled. Lateral finishing allowance is additional material reserved by Main
  and removed by Finishing; it cannot silently include retained-for-later stock.
- Main-only Profile Inside/Outside/On walks the requested compensated target
  through its depth schedule. Main drives every actual cutting/plunge segment.
- Profile finishing Inside/Outside reserves lateral allowance, then walks the
  final compensated contour using the actual finish cutter. On-contour lateral
  allowance and Profile floor allowance refuse as undefined. An explicitly
  requested zero-allowance repeat is labeled as such, not certified removal.
- Pocket Main-only includes its wall-defining path as part of Main plus the
  selected fill strategy, and reaches the requested floor. Do not equate
  `nocontour` with finishing disabled; unsupported old combinations refuse.
- Pocket finishing separates lateral and floor allowance. Main floor is final Z
  plus the positive floor allowance. Floor finishing requires an actual clearing
  path over the reachable final region, **not just a wall ring**. Wall and floor
  paths must preserve islands and remove the claimed reserved material.
- Finish schedules respect axial limits. A smaller finishing cutter cannot be
  assumed to clear inaccessible corner/rest regions with one wall contour. Prove
  coverage with bounded geometry checks or refuse; never declare completion from
  the presence of a Finish tag.
- Initial ordinary candidates: Cascade/Zigzag/Spiral with qualified entry, tools
  and containment. Trochoidal finishing, whirling overlays and unsupported
  combinations refuse unless separately proven within this milestone. Halfpipe
  remains its existing independent family/meaning and receives no finishing model.
- Both stages use the existing holding/lead and safe-move rules. Distinct cutter
  tabs require the existing physical-width/placement constraints to hold for
  both paths. Changed/unprovable holding intent refuses, never remaps silently.
- Same-tool stages do not create a toolchange. ShopBot differing RPM stays refused
  with field-level explanation; existing qualified distinct-tool envelopes remain.

The initial plan can use a small internal stage plan with separately built offsets
and resolved setups, dispatched through common emitters. It must prevent the
current path-level `is_finish` tag from choosing a different authored stage's rates.
This does not require changing ShopBot output capability or ALPHA-2 geometry rules.

## Simulation, report, preset and freshness plan

Attach result-local stage attribution to emitted motions alongside existing tool
provenance, keeping actual operation identity. Bind Main/Finishing to authored
values and output ranges; decode and verify those associations independently.
Screen/export reports show only active/emitted stage tools and values, intended
allowances and any unproven/incomplete result. Simulation carves actual paths;
stage selection or configuration alone is not a completion certificate.

Canonical generation inputs include every stage field, assigned cutter and known
limit. One atomic operation edit drives Undo/Redo and freshness. Saving does not
refresh an old program. Existing topology/placement, CPS, full-program simulation,
numeric/resource/cancellation and transactional-export guards remain mandatory.

Preset signatures distinguish the contract and finishing state. Historical
Profile Finish presets cannot become Main or newly tested Finishing. Exact
unaffected matching remains; no cross-operation reuse is introduced. Future
FabSpeed copying must review these stage contexts independently; no synchronization
or recommendation calculation is part of ALPHA-3.

## Current checks and evidence limits

| Check | Actual result |
|---|---|
| Rust baseline, all features | 10 characterization tests pass; 2 deliberately failing product baselines; 0 ignored. |
| Existing cutting-v9 Rust regressions | 13 pass; no product behavior changed. |
| Frontend focused set | 52 pass across readers, real WASM, restore/lifecycle, report, capability and freshness. |
| New tests formatting / lint | rustfmt, Prettier and focused ESLint pass. |
| Strict Clippy | New Rust test target with core all-features passes `-D warnings`. |
| Real WASM | Fresh release WASM built from this worktree; freshness fingerprint verified. This is a test dependency, not an installer/publication. |
| Final workspace/release gate | NOT RUN for a new implementation; none exists. The intentional red tests preclude a final PASS claim. |
| Browser UI / new FR/EN controls | NOT RUN / NOT IMPLEMENTED; no UI was changed. |
| Installed native / physical | NOT RUN; normal application-data profile untouched. |

Commands and hashes are recorded in `ALPHA_3_PREFLIGHT_RECEIPT.json`. Offline pnpm
install lacked one cached tarball; matching-lockfile dependencies were copied from
the ALPHA-2 checkout into this isolated worktree. A partial copy encountered existing
symlinks and was completed with rsync; the succeeding tests use local copied modules.
No dependency or lockfile was changed. Logs and generated caches remain untracked/ignored.

## Next authorized checkpoint after version approval

Implement the versioned model, then Profile, ordinary Pocket, stage consumers,
UI and integrated qualification. Keep baseline evidence before replacing tests
with assertions on the real new contract. Compare unchanged output byte-for-byte;
for changed Profile/Pocket programs inventory old/new motions, swept geometry,
origin/SAFE-Z, remaining stock, tools and stage rates. Never refresh goldens blindly.

Final acceptance still requires every ALPHA-3 criterion, a final source-bound
release gate, real browser/WASM walkthrough and a reviewed commit inventory.
No implementation commits are proposed at this preflight. **HARD STOP at the
version decision; ALPHA-4 and all integration/publication remain unstarted.**
