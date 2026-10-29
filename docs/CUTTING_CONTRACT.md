# V1.1 cutting, preset and safety contract — ALPHA-3 v10

Authoritative implementation: `ivac-core::project`, `cutting_parameters`, `presets`,
`cutting_stages`, `pipeline::op_drivers::staged`, `pipeline::stage_provenance`,
`pipeline::setup_resolver`; generated wire schema: `schema/openapi.yaml` and
`frontend/src/lib/api/generated.ts`. [ADR-0030](shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md)
is Accepted. No recommendation crate, source-pack runtime, Trial lifecycle or FabSpeed
client is part of the current application.

## Persistence and generation

On the isolated ALPHA-3 branch, a frontend ProjectFile has **version 10** and a
PipelineRequest has **`project.cutting_version: 10`**. Adapters require the explicit
`cutting-parameters-v10` capability. See [ADR-0032](shopbot-modernization/adr/ADR-0032-explicit-main-and-optional-finishing.md)
and the [source-bound qualification](releases/ALPHA_3_MAIN_CUT_OPTIONAL_FINISHING.md).
Primary V1.1/V1 are separate preserved branches; this document does not claim integration.

Profile and ordinary Pocket require `params.stages` with `contract: main_finishing_v1`,
mandatory `main` and required nullable `finishing`. Each stage owns actual tool ID,
positive integer RPM/feed/plunge and a positive constant depth per pass. Nullable
editable values are preserved; generation refuses incomplete values. The operation's
redundant tool assignments must equal the stage assignments. Legacy rate/depth fields
cannot coexist with these stages. Finishing is OFF by default, not inferred from a
wall-path tag. Other families retain `params.cutting.rates` with their existing exact
role/tool set and execution meaning inside the v10 envelope.

The new editor refuses v1–9/future portable projects before state/history mutation;
original files are not rewritten. Old readers reject new v10 documents/requests.
No automatic legacy machining conversion. Tool defaults seed creation only;
generation uses explicit authored stage/role values and refuses known limit violations.

Rust JSON generation structures reject unknown fields. Frontend project-file loading
explicitly rejects retired source/recipe keys, including null placeholders; unrelated
extension fields retain existing file preservation behavior. WASM uses a pre-conversion
Reflect guard for retired `fs_context`, `applied_cutting` and `fs_full_width` keys because
serde-wasm-bindgen does not enforce Rust unknown-field behavior generally. This guard
covers simple, two-sided, streaming and drawing-machining entry points before events;
the same boundary checks required Main/Finishing shape/keys before serde conversion.
It is not a claim that every unknown JS property is rejected everywhere.

Generic library physical IDs are preserved. If a generic old library entry lacks a
physicalId but has fsLocalIdentity.id, that same ID is retained; reference bindings
are otherwise inert metadata. This is not a v1–8 project migration or source resolver.
MaterialIdentity is the neutral exact material id/revision pair. MAT-1 taxonomy,
aliases and presentation stay independent of calculation.

## Actual milling roles

| Family | Required assignment / effective emission |
|---|---|
| Profile | Required Main; optional explicit Finishing on its own authored tool/rates/schedule |
| Pocket (ordinary) | Required Main includes wall and fill; optional explicit Finishing; legacy nocontour refuses |
| Drill | Drill plus enabled Spot and Chamfer roles on their actual selected tools |
| Chamfer | Rough, and Finish when the operation has a finishing pass |
| Halfpipe | Rough on its supported cutter; radius/reach restrictions retained |
| PlanarSurface, V-Carve, Engrave | Rough; their existing explicit strategy/entry constraints remain |
| Relief, Waterline | Rough; explicit model/depth and compatible cutter restrictions remain |
| Thread, TSlot, Dovetail | Rough; specialized geometry/process restrictions remain |
| Front/back | Same authored operation roles on each side; automatic dowel injection excluded |
| Program controls / non-milling processes | No invented milling rate slots; their separate process validators remain |

The immutable 23-case historical corpus, controlled ALPHA-3 differences and `pipeline/cutting_v9_tests.rs` compare actual spindle,
feed, plunge, tools, provenance and program transitions, with negative structural/
limit tests in `cutting_parameters.rs`. Current output fixtures are immutable references,
not workshop recommendations.

## Tested collections

`fabcam-preset-collection` **v2** stores new staged configurations; historical v1
is preserved/inspectable and cannot implicitly apply Profile/Pocket Rough/Finish to
v10 Main. Unchanged families retain exact compatible role matching. Both formats keep
the 2 MiB / 256-preset limits, original identities, context and explicit attestation.
No automatic v1 rewrite, context transfer, physical-claim renewal or live link. Loaded
snapshots survive collection changes; private notes are not copied into projects.
Native/browser storage retain explicit collision/stale-write checks and file scopes.

## Qualified finishing and output attribution

The first implementation proves one selected, exactly closed, axis-aligned four-line
rectangle with flat endmill/direct entry and no authored tabs, approach, leads,
patterns, helix/overcut or whirling. Profile Inside/Outside lateral finishing requires
equal effective cutter diameters and allowance <= radius; Profile On/floor finishing
refuses. Ordinary Pocket Cascade/Zigzag/Spiral can finish walls/floors using its own
cutter; it re-clears the full reachable target, not optimized rest-only material.
Selected islands, arbitrary contours, Trochoidal and unsupported proof/output cases refuse.

Retained-for-later lateral stock R stays distinct: Main targets radius + R + L and
Z + F; Finish targets its own radius + R and Z. OFF means no L/F influence. A positive
allowance below 0.05 mm refuses under the bounded 0.02 mm-per-stage representation
budget; zero explicitly means a repeat. This is software precision, not shop accuracy.

Complete wall/area coverage is established before emission using bounded swept-strip
union/convex capsule proof. No finite point sampling is represented as a global proof.
Decoded-output regressions independently check sweeps, retained stock and stage rates.
Response stage bindings carry actual operation/tool/parameters and resolved output
line/segment intervals where supported. Other bundled CPS Main-only paths may be
explicitly Planned; they cannot claim decoded Finishing. Same-tool ShopBot RPM changes
still refuse; no fake boundary or new post commands. Simulation/report consume actual
emission. Distinct-tool remaining-material UI remains explicitly unsupported.

## Safety owners and regression entry points

| Responsibility | Retained owner and regression entry points |
|---|---|
| Explicit roles/positive values, known ratings, no mutation | `cutting_parameters.rs`, `cutting_stages.rs`; `tests/alpha3_v10.rs`; `pipeline/cutting_v9_tests.rs`; `tests/explicit_contract.rs`; `cutting-v9-wasm.test.ts` |
| Geometry/reach/specialized mandatory limits | Neutral validator, setup resolver and op drivers; specialized core tests; `ux3_qualification.rs` |
| Numeric/resource limits | `tests/numsafe.rs`, `numsafe2.rs`, NUMSAFE3 core/CLI regressions; `scripts/numsafe3-cli-check.py` |
| SAFE-Z, origin, fixtures, cutting direction | Existing SAFE-Z/simulation tests; `shopbot_m2_post.rs`, CAM3/UX qualification matrices |
| Tool identity, setup transitions and stock simulation | `tests/tool_provenance.rs`, full-program simulation/export frontend tests; fixed output corpus |
| Preset context, immutable reviewed copy, bounded parser | `presets.rs`, `preset_collections.rs`, `cutting-workflow.test.ts`, `preset-library-v9.test.ts` |
| Save/reopen, Undo/Redo and stale export | `project-v10.test.ts`, document lifecycle, `cutting-workflow.test.ts`, `test1-lifecycle.test.ts` |
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
