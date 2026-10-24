# QUAL-1A.4 — Deterministic V-Carve production integration

Date: 2026-10-24. Engineering handoff; final human acceptance remains OPEN.

**CONDITIONAL GO for the explicitly authorized local integration. NO-GO for
unconditional software qualification, release, installed-platform acceptance or
machining.** The numerical implementation reproduces the reviewed QUAL-1A.3
results, and P1 remains strict refusal with read-only context review. Mandatory
pre-release gates remain **17/20**, with the three existing blockers visible.
No new executable-motion difference was found. QUAL-1B has not begun.

## Source and local commit boundaries

| Boundary | Exact SHA / disposition |
|---|---|
| Historical qualified production | `558efc4f7e7cfb5d008c88cf7469a958cc8dc053` |
| Inspected `FabCAM-v1.1` starting HEAD | `6148b1059ab6020f751069d5c3a7d64b8047fb6b` |
| Numerical implementation | `c74f5e9c1f220a177bb5ffe956c9a50c90ba175d` |
| P1 implementation; first complete qualification | `61c00ba59a3f2b9373f5760205a698f4c7413799` |
| Diagnostic correction; final qualification source | `8e2c9ce427007f9c5a3787b34dd28647b8cdcf95` |
| Integration worktree / branch | `FabCAM-qual-1a4` / `codex/qual-1a4-production` |

The third commit changes only `scripts/qual1a4/public_numeric.mjs`: it isolates
an inspector placeholder from separate invalid-CAM probes, adds a valid control,
and asserts the exact authoritative refusal. All Rust, frontend, dependencies,
schemas and application contracts are identical to `61c00ba`. Both earlier
negative traces and corrected results are retained. The complete mandatory gate
and all-target suites were rerun after this diagnostic correction. The subsequent
handoff commit contains only this report, its evidence and documentation index;
its identity is obtainable with `git log -1 --format=%H -- docs/releases/QUAL_1A_4_PRODUCTION_INTEGRATION.md`.
Report/index drafting overlapped the end of the repeated checks; only documentation
was dirty. `FINAL_SOURCE.json` records that state and proves identical Rust, frontend,
Cargo and schema tree objects. No clean release-artifact provenance is claimed.

The isolated worktree began from the actual committed primary HEAD, not a dirty
experimental checkout. Eight candidate production files were reviewed against
QUAL-1A.3; the combined experimental patch SHA-256 remained
`3a227f63062acc37323bbfb0d0aa0d41eb56ce5a4a6655bd163f6a018ff5cbed`.
The registry dependency replaces the experimental local path. No algorithm,
postprocessor, safety threshold, schema or matching tolerance was redesigned.

## Evidence and preservation

Compact receipts are in [qual-1a4-evidence](qual-1a4-evidence/README.md).
Raw evidence is retained at
`/Users/mugen/Documents/01_Projets/FabCAM/ivacam/dist-test/qual1a4/`;
full responses, original failed attempts, binaries, benchmark processes and
TEST1 workers are deliberately outside Git. The QUAL-1A/1A.2/1A.3 reports remain
pre-existing local-only primary files, not absorbed into this commit;
HISTORICAL_REPORTS.json records their exact paths/hashes. Their relative links
resolve in that retained primary evidence layout, not in a fresh bare clone. `RAW_SHA256SUMS.txt` binds the retained
raw files; original inputs and output corpora were never regenerated in place.

Before integration, all **8,473** primary dirty/untracked file fingerprints,
**10,281** historical raw files and the **8 / 8 / 87** recorded changed files in
the three experimental worktrees matched their starting hashes. The counts
overlap where a raw file is also represented in the primary inventory; they are
not a count of unique files. Starting inventory and final integration receipts
remain local. No application-data directory or user-owned project was used.

These ten primary tracked modifications remain owned by the prior work:

- `crates/ivac-core/src/pipeline/offset_builder.rs`
- `crates/ivac-core/src/pipeline/warnings.rs`
- `docs/releases/RELEASE_ROADMAP.md`
- `docs/shopbot-modernization/STATUS.md`
- `frontend/src/App.svelte`
- `frontend/src/app.css`
- `frontend/src/lib/components/AppNavigation.svelte`
- `frontend/src/lib/components/LayerList.svelte`
- `frontend/src/lib/state/import-ops.ts`
- `frontend/src/lib/state/project-types.ts`

The source integration completed as a verified local fast-forward from `6148b10`
to `8e2c9ce`, with no overlapping changed paths or untracked collisions.
INTEGRATION.json records the actual before/after, ancestry and repeated
preservation checks. The report-only handoff is a subsequent local fast-forward;
its exact receipt remains in the raw evidence. **The qualified committed
source is distinct from the primary checkout plus these uncommitted changes.**
Those combined local edits have not acquired qualification from this report.
The clean documentation index links this checkpoint; dirty STATUS and
RELEASE_ROADMAP are deliberately not overwritten or absorbed. Historical reports,
V1 evidence, unrelated worktrees and remotes are preserved.

## Production numerical contract

`libm = { version = "=0.2.16", default-features = false,
features = ["force-soft-floats"] }` now resolves from the official Cargo registry.
The archive and lockfile checksum are both
`b6d2cec3eae94f9f509c767b45932f1ada8350c4bdb85af2fcab4a3c14807981`.
No vendored modification or path dependency remains. Runtime dependencies do not
expand beyond this one package; its optional development `no-panic` dependency
is not a runtime dependency. MIT licensing and upstream per-file notices remain
in the unmodified registry source; LICENSE.txt SHA-256 is
`3823dda7cf046602f4b4e77ec8e227863dc4736037cc85bb33d9f19febe16bb7`.
This focused provenance check does not replace the blocked current advisory audit.

The reviewed helper supplies only V-Carve tangent and segment-length arithmetic
in `cam/vcarve.rs`, `cam/vcarve_plan.rs` and `pipeline/op_drivers/vcarve.rs`.
The simplifier, medial graph traversal, entry/retract policy, postprocessors,
SAFE-Z, provenance, validators, simulation and export code are unchanged.

Workspace `serde_json` is pinned to `=1.0.149` with `float_roundtrip`.
Recorded inverse feature graphs cover workspace/core, CLI, server, Tauri and
WASM, for both serde_json and libm. The runtime graphs contain float_roundtrip;
portable libm has force-soft-floats and no default architecture feature. Tauri's
separate host/build dependency instances are not falsely described as runtime
parsers. Cargo.lock and generated model/schema clients have no unexplained drift.
See [deterministic numerics](../DETERMINISTIC_NUMERICS.md).

JSON-text ingress now rounds decimals consistently. JavaScript object ingress
already contains binary64 values; it is not a second text parser. Worker
`toCloneable` retains its existing JSON.stringify behavior (`-0` becomes `+0`);
this report does not claim signed-zero preservation through that normalization.

## Numeric and executable qualification

- **328** independent rational-oracle cases: 304 finite numbers, 9 overflow
  inputs and 15 malformed/schema cases. Every finite bit matches through native
  preset command and real WASM JSON-text entry. Overflow refuses. Invalid entry
  angles may be inspected with validation errors; that is not generation success.
- All **1,572** numeric occurrences in the original dense request match native/JS,
  including the **126** previously discrepant decimal coordinates. No custom
  parser, decimal normalization or precision tolerance was added.
- Real HTTP request-body extraction reaches the neutral validator and buffered
  generator; valid responses match the pipeline and overflow is a client error.
  Native Tauri Value-to-command decoding, generation and stored collection
  byte/token preservation are tested. These invoke real handlers in-process,
  not an installed WebView or a bound HTTP listener.
- **77** immutable requests replayed three times per native and actual WASM
  runtime: original **40 successes / 21 refusals**, plus **12 successes / 4
  refusals** in the 16 nearby cases. Each runtime's complete response/refusal
  equals its reviewed QUAL-1A.3 result. All **52 successful programs**, line
  indices and stats match native/WASM; **51/52 complete toolpaths** match.
- The existing Profile internal reconstruction difference remains
  `2.842170943040401e-14 mm`; the emitted program is exact. Universal full-path
  bit equality is not claimed.
- The separately built standard release CLI replays those 77 cases plus the
  three CPS cases (**80 expected results**), independently of the joint-package
  probe build. Canonical `generate --request`, bounded templates and streaming
  contracts also run in the workspace/CLI boundary suites.
- **23 frozen CLI cases** remain exact: 22 buffered results compare program,
  toolpath, provenance, line index and stats; the two-sided case compares both
  published program texts. This corpus was checked with both release CLI builds.
- CPS Square and Text/Test match the reviewed native candidate completely;
  B8 retains overcut refusal. WASM remains built without CPS. The actual worker
  handler plus WASM in a Node worker reproduces direct Square/dense responses;
  its host adapter is not browser-worker acceptance by itself.
- **13** prior independent numeric/geometry/order oracle tests and **7**
  QUAL-1A.3 motion-oracle regressions pass. The original 998-case portable math
  investigation and its artifacts are preserved; that separate prototype matrix
  is historical evidence, not reported as 998 fresh production tests here.

`CHANGED_FILES.json` lists the exact 37 application/diagnostic files introduced
by the first two commits and their hashes. The third commit changes only the
numeric diagnostic listed above. Fresh final binaries, WASM hashes and both CLI
build configurations are recorded separately. Diagnostic parser stacks map only
the checkout pathname when comparing raw malformed JSON; raw stderr is retained.
No successful output or substantive refusal is normalized.

## Dense-circle motion acceptance ledger

The original 256-edge request hash is
`7fe2d768b9ff91dbd5a0171e5d14ea5e5d75e46dd2d09098aa053c9b0bb628bb`.
Integrated complete responses equal the independently reviewed candidate, binding
this production integration to the existing certificate rather than inventing a
new tolerance or silently replacing a golden reference.

| Observation | Original native → integrated native/WASM |
|---|---|
| Movements | 13,996 → 14,015 |
| Plunges / rapids / cut runs | 255 / 512 / 255, unchanged |
| First seed | 15 → 53; reviewed floating-point selection consequence |
| Matched-entry regions | 189 dispositioned; 186 subdivision/traversal/non-cut, 3 joined-branch exchanges |
| Final ideal swept-cone union enclosure | ≤ 0.000414864 mm, fixture-specific |
| Chronological additional vertical-material thickness | ≤ 0.200108178 mm, fixture-specific |
| Boundary clearance margin / SAFE-Z / depth cap | ≥ 1.499757452 mm / 15 mm / −1 mm |

The full continuous cone certificate was **not recomputed**: it is reused from
[QUAL-1A.3](QUAL_1A_3_MOTION_PRESET_ACCEPTANCE.md) after exact full-response and
input-hash comparison, with independent oracle regressions rerun. Intermediate
material histories do differ (sampled differences near 1 mm; first virgin branch
volume approximately 0.10644 → 3.54659 mm³). The maintainer explicitly accepted
this software integration scope. No equivalence of instantaneous engagement,
chip thickness, forces, spindle load, holder/fixture clearance or physical
machining is inferred. No new unexplained executable motion change was observed.

## P1 behavior and persistence

The authoritative exact context comparison is unchanged. A mismatch displays
**“Preset context needs review”** with neutral review guidance and preservation
notice. Only the exact compatibility error (and its standard Error prefix) is
translated; genuine safety and validation errors remain distinct. No numeric
cause is inferred and no automatic signature migration is offered.

The existing library details and operation load review now expose the complete
stored compatibility context read-only, including machine configuration, physical
IDs, material revision, strategy, plunge, tool geometry and role assignments.
Values come from the saved signature, never a reconstruction from the current
operation. Empty arrays/objects, nulls and extension fields remain visible.
Malformed/unsupported data, unsafe numeric values, >64 KiB context, depth >16 or
>2,048 display rows fail to a bounded explanation. Display parsing grants no
permission to load and never writes the signature. Normal Svelte text escaping
is retained; there is no HTML rendering of imported values.

Original rates remain readable after refusal; “Replace parameters” is absent
without a successful review. Manual authoring remains untested. A new tested
preset still requires the existing explicit attestation, separate from software
validation; imported claims are not evidence of testing on the current machine.

**48 real old/new native/WASM outcomes** cover numeric mismatch, ordinary
compatible data, actual diameter changes, machine setup, material revision,
physical cutter identity, strategy, duplicate/rename, author claims and unchanged
source bytes. Native collection tests retain optimistic concurrency, safe IDs,
size limits, busy/stale refusal, symlink/path refusal and explicit-save
canonicalization. Frontend tests cover cancellation, failed writes, stale targets,
attestation and independence after source deletion. No private store was used.

Project **v9 remains unchanged**. Round-trip tests include incomplete editable
values, all Rough/Finish/Drill/Spot/Chamfer rates, large identity strings, material
revision, work offset and supported extension decimals. Retired versions/fields,
malformed inputs, role/tool conflicts and known-limit violations still refuse.
Tool defaults seed creation once; generation has no implicit replacement rates.

## Final software qualification

Host: macOS 26.5.2 arm64, Rust 1.88.0, Node 24.21.0, pnpm 10.34.7.
Builds used `RUSTC_WRAPPER=''` because the sandbox blocks the sccache daemon,
`CARGO_INCREMENTAL=0`, the existing shared target cache and an isolated copied
pnpm store. Lockfiles remained frozen; these settings do not skip assertions.

| Suite | Result and limits |
|---|---|
| Workspace, all features/targets, no fail-fast | 1,587 reported PASS, 2 watcher FAIL, 1 intentional ignored TEST1 worker |
| Doc tests | PASS; zero documented examples discovered |
| Strict pedantic Clippy / fmt | PASS, unchanged allow-list |
| Frontend | 172 files, 1,498 PASS, 1 existing optional strict-locale skip |
| WASM package/freshness | 12 PASS; fresh actual WASM integration and production build PASS |
| Svelte / TypeScript | 0 errors, 0 warnings |
| ESLint / Prettier / schema / generated TS / i18n drift | PASS |
| Native file export | 11 PASS including publication/cleanup failure and second-side failure |
| Tauri preset import ACL | PASS against generated manifests, including old-ACL counterexample; static only |
| HTTP | 21 PASS including real decoding and generation |
| Tauri | 13 PASS, the same 2 watcher FAIL |
| TEST1 + C2 | **256/256 PASS**, 224 native cases + 32 C2, 1.104 s |
| Independent oracle regressions | 13 + 7 PASS |

TEST1 retains the unchanged 120 s stage / 10 s worker watchdogs; it is not a full
SAFE-Z proof. The native all-target suite also runs benchmarks in test mode.
Its reported passes include pre-existing optional scaffolding early-returns:
6 FANUC oracle cases, 1 FANUC pipeline case, 2 external DXF import cases, the
external DXF corpus smoke (zero DXFs), and the optional local-project step shim
(two absent files). Five external DXF benchmark inputs are absent. These do not
constitute exercised coverage. The Node/V8 differential harness did execute.
The committed corpus and supported built-in post comparisons are independent of
those optional external references. No missing private/reference data was invented.

### Mandatory pre-release gates

| Gate | Result |
|---|---|
| cargo fmt --check | PASS |
| strict pedantic Clippy | PASS |
| cargo test --workspace | **FAIL** — 2 watcher tests |
| xtask schema-check | PASS |
| xtask version-check | PASS |
| README language parity | PASS |
| pnpm install --frozen-lockfile | PASS (cached install; update-check DNS warning retained) |
| generated TypeScript drift | PASS |
| i18n key drift | PASS |
| frontend lint | PASS |
| frontend type/Svelte check | PASS |
| frontend tests | PASS with existing optional locale skip |
| frontend production build | PASS; existing >600 kB chunk warning retained |
| wasm-pack web release build | PASS |
| WASM CPS size guard | PASS: +1,592 KiB gzip, unchanged 1,700 KiB budget |
| cargo-deny bans/licenses/sources/advisories | **BLOCKED/FAIL** |
| mandatory headless UI smoke | **BLOCKED/FAIL** |
| CLI trust and transactional export | PASS |
| SEC1 release boundaries | PASS |
| candidate identity regressions | PASS; synthetic artifacts only, no installers built |

The exact unmodified `scripts/pre-release.sh` ran every gate; **17/20 is not
20/20**. Watcher errors remain `event within 2s: Timeout` and `debounced 5 rapid
writes; got 0 events for path`. They predate this change; a sandbox explanation
is plausible, but host execution must distinguish environment from product defect.
Neither test nor its timing assertion was weakened.

cargo-deny first failed on the read-only advisory lock under `.cargo`. A separate
configuration changed only `advisories.db-path` to a task-owned writable cache;
its complete audit then failed fetching RustSec because GitHub DNS was unavailable.
An initial incorrectly positioned CLI option was retained separately. No advisory
ignore, offline substitution or reduced audit is counted as success.

The unchanged Chrome smoke launched the real installed browser, which aborted;
CDP connection then failed `EPERM 127.0.0.1:9233`. Interactive IAB observations
below do not replace this mandatory smoke gate. No watcher, smoke or security gate
was disabled. See the individual logs and final qualification JSON.

## Functional UI walkthrough

The maintainer launched the isolated Vite server at `http://127.0.0.1:5174/`;
review used the real WASM backend (`?api=wasm`) and a new local origin. All data
created here is disposable software QA, with a collection named “Disposable
QUAL-1A.4 synthetic claims” and a preset named “SOFTWARE QA ONLY — never machined”.
Selecting the attestation checkbox tested the software flow only; it is explicitly
not a workshop test claim. No real collection/project was opened or modified.

| Requested interaction | Observation |
|---|---|
| New operation / explicit values | Pocket authored; rough 950 and finish 400 mm/min, plunge 100 |
| Multiple roles | Role-specific values remain independent and appear in report |
| v9 save/reopen through browser files | **BLOCKED**: browser download authorization declined; no workaround or alternate transfer attempted |
| New tested collection | Created through existing explicit synthetic-attestation flow |
| Compatible preset review | Original values and replacement preview visible before application |
| Old numeric-context preset in browser | **NOT EXERCISED** after file-transfer denial; covered separately by 48 real command outcomes |
| Strict refusal / P1 guidance | Changing Pocket strategy to zigzag refuses, preserves recorded cascade and values |
| Recorded context | Expanded full read-only machine/material/operation/tools/roles disclosure |
| Original values | 950/400 remain visible after current finish becomes 450; no auto-application |
| No attestation renewal | Manual operation still says Manual parameters; Save tested preset disabled until fresh checkbox |
| Manual authored values | Explicitly separated from an author's old testing claim |
| Undo/Redo / staleness | Undo restores compatible context; Redo restores refusal. Editing generated feed marks toolpath stale |
| Actual V-Carve | Text I + stock outline: 2,102 movements after origin review; real 3D result shown |
| Report | Current program, full-program simulation complete; authored 18,000 rpm, 700/100 mm/min; 0 critical / 3 non-critical warnings |

The stale export menu remains visible; no file action was attempted after the
transfer refusal. Actual export rejection is qualified by automated freshness
and file-service tests, not claimed from a browser download. The manual Text/Test
attempt at different UI settings correctly stopped on the overcut guard; it was
not made to succeed by changing tolerances. A simple I was then authored for the
visual success path. Unknown limits, capped target and medial-axis scope remain
visible, and none imply physical clearance.

The Vite title retained its server-start stamp `c74f5e9-dirty`; source was hot
updated, then the final production build was verified independently. This stamp
is not represented as the final source receipt. The final rebuild reloaded the
unsaved disposable project; the synthetic collection persisted and was reopened
unchanged, with its recorded context left available for inspection. Screenshots and DOM observations
are retained; host review of real file dialogs, old numeric import and installed
Tauri flows remains necessary. Browser automation required native committed
input events; transient DOM fill/select display changes were not accepted as
saved values without report verification.

## Performance, memory and footprint

Protocol: five rotated process rounds per production / experimental / integrated
backend; seven generations each (cold, warm-up, five warm). Runs were serial after
compilation finished. Native pipeline time, WASM initialization/parse/generation,
process CPU/RSS and exact output hashes are retained. Native builds are release,
uninstrumented. Node overhead is included in process RSS, not attributed wholly
to WASM. These are observations, not a statistical performance guarantee.

Warm median generation time, milliseconds:

| Case/runtime | Production | Experimental | Integrated |
|---|---:|---:|---:|
| Square/native | 5.320 | 5.317 | 5.275 |
| Square/WASM | 9.231 | 9.583 | 9.411 |
| Text Test/native | 22.089 | 21.782 | 22.025 |
| Text Test/WASM | 36.218 | 36.623 | 36.986 |
| Text Test/CPS native | 6.720 | 6.811 | 6.927 |
| Dense/native | 18.513 | 18.469 | 18.864 |
| Dense/WASM | 29.414 | 28.814 | 29.045 |

A separately rebuilt parser probe uses the same pinned serde_json/feature as the
final runtime. Five rotated triplets ×22 parses exclude the first two per process.
For 42,058 bytes, median parse time is 0.1484 / 0.1468 / 0.1783 ms; for the
repeated-segment large parse-only input it is 17.3329 / 17.5027 / 17.7029 ms.
The small final probe is about 20% above the baseline (+0.030 ms), while the large
input is about 2.1% higher. No retry was used to replace those observations with
more favorable values. The large input is not a valid-CAM or memory-budget waiver.

| Artifact | Production | Experimental | Integrated |
|---|---:|---:|---:|
| WASM bytes | 5,309,252 | 5,368,393 | 5,368,390 |
| WASM gzip bytes, same Python compression | 1,895,286 | 1,914,631 | 1,914,769 |
| Native perf probe bytes | 11,643,776 | 11,693,680 | 11,693,648 |

Integrated WASM adds 59,138 raw bytes / 19,483 gzip bytes versus production;
experimental/integrated gzip differ by 138 bytes. Build path/provenance differences
mean equal outputs do not require identical artifact hashes. The standard CLI is
12,067,920 bytes. Native peak RSS across cases was approximately 13.8–83.6 MB;
WASM/Node 269.9–459.8 MB, comparable to the retained configurations. Full per-case
resources are in PERFORMANCE_SUMMARY.json. Existing warm-cache fresh native and
WASM builds completed; their timings are recorded, not called clean-build cost.
The new runtime dependency is the one pinned libm package. No cancellation target
was relaxed or cancellation implementation modified. The previously recorded
ShopBot Test cancellation p95 **200.353 ms > 200 ms** remains an open QUAL-1B issue;
it was not requalified away by generation benchmarks.

## Retained safety boundaries and architectural effect

FabCAM still owns explicit per-operation/per-role cutting data; FabSpeed remains
an unimplemented optional future service. Legacy recommendation code is not
restored. MAT-1, generic libraries, v9, tested collections and manual values retain
their contracts. No application-data migration runs.

Neutral CAM validation, NUMSAFE/2/3 budgets, provenance, SAFE-Z/work-origin,
specialized mandatory restrictions, continuous V-bit envelopes, known-limit
refusals, simulation qualification and export freshness remain exercised. Unknown
limits are disclosed instead of invented. Automatic two-sided dowel injection
remains excluded. Bundled-only CPS trust and ShopBot restrictions remain intact.

`ivac-file-export` stages before publication and retains complete single-file
publication semantics. Both sides are staged before two-sided publication, but
the pair is **not atomic**: front can be published while back fails. Tests retain
truthful partial-result/cleanup reporting; no blanket rollback guarantee is added.
Tauri capability inspection retains `fs:allow-stat` and bounded file access;
static ACL or mocked frontend file tests are **not installed-native acceptance**.
CLI streaming preserves preflight and cancellation/resource rules but does **not**
acquire the buffered application's complete simulation certificate. Stdout remains
nontransactional; CLI file publication keeps its separate transactional contract.

## Host acceptance and exact next decisions

Use a clean checkout of `8e2c9ce427007f9c5a3787b34dd28647b8cdcf95`, not the primary
checkout's unrelated dirty CAM/frontend edits. With pinned Rust, Node 24, pnpm 10,
wasm-pack, matching wasm-bindgen/wasm-opt, cargo-deny, an actual Chrome wrapper,
writable Cargo cache, working DNS and normal local-loopback permissions:

```sh
pnpm --dir frontend install --frozen-lockfile
node frontend/scripts/ensure-wasm-fresh.mjs
cargo test --locked --workspace --all-features --all-targets --no-fail-fast
cargo test --locked --workspace --all-features --doc --no-fail-fast
scripts/pre-release.sh
```

Retain exact source, complete logs and all 20 dispositions. If watcher failures
persist on a normal host, diagnose them as a product/platform issue before any
qualification claim; do not relax the two-second/debounce assertions. cargo-deny
must fetch/check the current advisory DB and all configured categories. Headless
smoke must pass with the unchanged script and its actual browser interactions.
Run TEST1 with the release `test1_properties` executable, its normal ignored-worker
harness, `--frontend`, default 32 cases/property and unchanged watchdogs. Replay
scripts under `scripts/qual1a4/` require the explicitly retained local evidence
layout; do not substitute absent historical artifacts or update goldens.

A human must complete permitted browser save/open and synthetic old/current
collection import/export on an isolated origin, then installed-native Tauri
file/dialog/concurrency/stat authorization checks. macOS/Windows/Linux installed
acceptance and physical ShopBot qualification are separate and unperformed.
No installer or hardware operation is authorized by this procedure.

The maintainer must separately accept (1) the exact integrated numerical/P1
implementation and dense ideal-tool limits, (2) the remaining gate/manual gaps
and required host execution, and (3) any subsequent work authorization. The
proposed next milestone is QUAL-1B cancellation analysis, **unstarted**. Human
review issue `ivac-y25vq` remains OPEN. No push, main merge, rebase, release,
version promotion, installer, FabSpeed integration or hardware operation occurred.

**FINAL HARD STOP: no next milestone without separate human approval.**
