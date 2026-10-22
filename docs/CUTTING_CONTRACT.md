# V1.1 cutting, preset and safety contract

Authoritative implementation: `ivac-core::project`, `cutting_parameters`, `presets`,
`pipeline::setup_resolver`; generated wire schema: `schema/openapi.yaml` and
`frontend/src/lib/api/generated.ts`. [ADR-0030](shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md)
is Accepted. No recommendation crate, source-pack runtime, Trial lifecycle or FabSpeed
client is part of the current application.

## Persistence and generation

A frontend ProjectFile has version 9. A PipelineRequest is a different envelope with
`project.cutting_version: 9`. Every enabled milling operation contains
`params.cutting.rates`: an exact role/tool-id set with positive integer `rpm`,
`feed_mm_min` and `plunge_mm_min` for execution. Nullable values permit incomplete
editable projects; generation refuses incomplete roles. Duplicate/misassigned slots,
unsupported versions, invalid numeric/depth schedules and known limit violations refuse.
Tool defaults seed creation only; generation never reads them as milling-rate fallback.

Rust JSON generation structures reject unknown fields. Frontend project-file loading
explicitly rejects retired source/recipe keys, including null placeholders; unrelated
extension fields retain existing file preservation behavior. WASM uses a pre-conversion
Reflect guard for retired `fs_context`, `applied_cutting` and `fs_full_width` keys because
serde-wasm-bindgen does not enforce Rust unknown-field behavior generally. This guard
covers simple, two-sided, streaming and drawing-machining entry points before events;
it is not a claim that every unknown JS property is rejected everywhere.

Generic library physical IDs are preserved. If a generic old library entry lacks a
physicalId but has fsLocalIdentity.id, that same ID is retained; reference bindings
are otherwise inert metadata. This is not a v1–8 project migration or source resolver.
MaterialIdentity is the neutral exact material id/revision pair. MAT-1 taxonomy,
aliases and presentation stay independent of calculation.

## Actual milling roles

| Family | Required assignment / effective emission |
|---|---|
| Profile | Rough and Finish on the authored tool; existing contour emitter uses Finish |
| Pocket | Rough and Finish; finish_tool may differ; nocontour requires Rough only |
| Drill | Drill plus enabled Spot and Chamfer roles on their actual selected tools |
| Chamfer | Rough, and Finish when the operation has a finishing pass |
| Halfpipe | Rough on its supported cutter; radius/reach restrictions retained |
| PlanarSurface, V-Carve, Engrave | Rough; their existing explicit strategy/entry constraints remain |
| Relief, Waterline | Rough; explicit model/depth and compatible cutter restrictions remain |
| Thread, TSlot, Dovetail | Rough; specialized geometry/process restrictions remain |
| Front/back | Same authored operation roles on each side; automatic dowel injection excluded |
| Program controls / non-milling processes | No invented milling rate slots; their separate process validators remain |

The fixed 23-case corpus and `pipeline/cutting_v9_tests.rs` compare actual spindle,
feed, plunge, tools, provenance and program transitions, with negative structural/
limit tests in `cutting_parameters.rs`. Current output fixtures are immutable references,
not workshop recommendations.

## Tested collections

`fabcam-preset-collection` version 1 is strictly validated, limited to 2 MiB and 256
presets. It stores explicit attestation, context, operation values and role/tool
signatures in named collections. Review/copy requires exact compatible machine,
material, operation and tool context. It never changes tools/machine/material or
creates a live link. Loaded snapshots survive library edits/deletion. Software limits
validation and workshop attestation remain separate. Imported claims are not evidence
of testing on the current machine. Native/browser storage retain optimistic conflict
checks and explicit collision handling; no old store is automatically reset.

## Safety owners and regression entry points

| Responsibility | Retained owner and regression entry points |
|---|---|
| Explicit roles/positive values, known ratings, no mutation | `cutting_parameters.rs`; `pipeline/cutting_v9_tests.rs`; `tests/explicit_contract.rs`; `cutting-v9-wasm.test.ts` |
| Geometry/reach/specialized mandatory limits | Neutral validator, setup resolver and op drivers; specialized core tests; `ux3_qualification.rs` |
| Numeric/resource limits | `tests/numsafe.rs`, `numsafe2.rs`, NUMSAFE3 core/CLI regressions; `scripts/numsafe3-cli-check.py` |
| SAFE-Z, origin, fixtures, cutting direction | Existing SAFE-Z/simulation tests; `shopbot_m2_post.rs`, CAM3/UX qualification matrices |
| Tool identity, setup transitions and stock simulation | `tests/tool_provenance.rs`, full-program simulation/export frontend tests; fixed output corpus |
| Preset context, immutable reviewed copy, bounded parser | `presets.rs`, `preset_collections.rs`, `cutting-workflow.test.ts`, `preset-library-v9.test.ts` |
| Save/reopen, Undo/Redo and stale export | `project-v9.test.ts`, document lifecycle, `cutting-workflow.test.ts`, `test1-lifecycle.test.ts` |
| Native authorization and publication | Tauri canonical path + existing filesystem scope; `ivac-file-export` 11 tests; SEC1 probes/static ACL regressions |
| CPS trust, streams, cancellation/recovery | Bundled-only CPS, CLI/HTTP/core stream tests, PERF1 regressions and TEST1 watchdog including C2 |
| Removed APIs cannot reappear / fresh WASM | `removed-legacy-contract.test.ts`, `explicit_contract.rs`, `wasm-package-state.test.mjs` |

See the [final exclusion/replacement ledger](releases/fs-reset-3-evidence/final-test-ownership.json)
for obsolete behavior retirement and [results](releases/FS_RESET_3_RESULTS.md) for actual
pass/fail counts. A retained test location does not by itself claim that every external
platform or latency target passed.

## Publication and qualification boundaries

Tauri authorizes the canonical program destination through its existing fs scope
before staging. Existing configured/dialog scopes remain unchanged; this is not a new
dialog-only policy. `fs:allow-stat` and bounded collection file-size inspection remain.
`ivac-file-export` preserves old bytes on preflight/publish failure and atomically
publishes a single staged file where the platform supports the existing implementation.
Two-sided export buffers/stages both programs with fresh destinations, then publishes
front and back separately. Failure of the second publication reports the surviving
front and missing back; the pair is not atomic and existing files are not overwritten.

Canonical `generate --request` uses the common buffered validator/pipeline. Its stdout
write/shell redirection is not transactional. CLI streaming retains resource/preflight
refusals and staged file publication, but is not equivalent to application full-program
simulation qualification; stdout cannot be rolled back. Bundled-only CPS and ShopBot
restrictions are unchanged. Static ACL and mocked/unit filesystem tests do not establish
installed-native acceptance. Browser/three-platform/physical acceptance remain separate.
