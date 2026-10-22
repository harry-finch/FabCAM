# FS-RESET-3 bounded engineering receipts

Branch `FabCAM-v1.1`; engineering tracker `ivac-wjymz`. Full removal was authorized by
the maintainer on 2026-10-22. These are incremental receipts, not final acceptance.
Starting HEAD: `951b12f4477f64fd5e9c229b693748f23e794a0e`.
CLI Option A `030e2149f5bcb7f356711face54caafcf6155843` is an ancestor.

## A — source inventory and fixed comparisons

The starting scan covers 3,348 tracked files, 258 operational files and 4,385 exact
reference lines. `start-consumers.csv` uses REMOVE/MIGRATE/KEEP/TEST-ONLY dispositions;
`start-historical-references.csv` protects 337 historical files. The scanner also
includes whole-module candidates and indirect gate scripts; lexical matches are not
wholesale deletion authority. Mixed files require symbol-level review.

All existing untracked evidence, two pre-existing tracked documentation edits, other
branches/worktrees and application-data namespaces are preserved. Complete initial
status, patch, worktrees/branch refs and tracked-content hashes are in the local
`dist-test/fs-reset3/start-*` files. No production data was read or erased.

Commit `a5907ac4ae284feb44c9984d1c3cc17ade524c88` freezes 23 explicit v9 cases generated
at starting HEAD. `tests/fixtures/fs-reset3-output` covers native LinuxCNC/GRBL,
ShopBot Profile/V-Carve, all supported milling role families and both sides. The new
integration test compares exact program bytes, every toolpath field, provenance,
line bindings and statistics. Both expected and actual JSON pass through the same
serializer/parser representation; this avoids a one-ULP serde Value round-trip
artifact. There is no numerical tolerance or executable-program normalization.

## B — neutral generation dependency

All pipeline requests now require explicit v9 values, including internal consumers.
`cutting_parameters` no longer delegates absent-version validation, saved rates or
machine limits to `feeds_speeds`. Setup resolution no longer reads recommendation
metadata; PlanarSurface no longer requires a recommendation origin. Machine limits
and neutral `ToolEntry.max_rpm` remain authoritative. General setup representation,
SAFE-Z, work origin, numeric budgets, provenance and post security remain in place.

There is no live milling-rate, contour-step or drill-peck default at generation.
Creation-time seeding is unchanged. Existing non-milling process setup stays separate.
Unknown limits remain disclosed; specialized mandatory restrictions remain mandatory.
A pocket without a finish wall ring does not resolve its unused secondary tool.
Automatic dowel injection stays refused for v9; explicit Drill roles remain supported.

The test-only `tests/support/explicit.rs` authors old geometric fixture constructors
into v9 before the pipeline call. It does not participate in any production adapter,
never changes already-authored v9 or invalid-version requests and never modifies
fixed output references. Test/probe serialization records the authored request.

`test-replacements.json` records each retired/replaced family. Invalid inputs that
previously warned or clamped now assert explicit refusal. The independent simulation
volume/stock-floor thresholds and NUMSAFE pass budgets are retained. Analytic volume
fixtures author the same formerly effective geometry (4 mm chamfer width, identical
V-bit cone, explicit height-grid range). The already-approved v9 per-op-cache bypass
is asserted directly; exact tee cap/overflow/recovery, CPS cache invalidation and
cancellation remain independently covered.

Three obsolete applied-recommendation generation tests are retired; immutable output
comparisons plus v9 saved-role/provenance regressions own their useful responsibilities.
The old engine, transports and disconnected frontend remain temporarily for phases
C–E. No claim of dependency closure or final release qualification is made here.

### B qualification

- Core with all features, all normal test binaries: **1,467 PASS, 0 FAIL, 1 ignored**
  (`core-b-all-features.log`). The ignored TEST1 worker must run through its watchdog
  in final qualification, including C2; it is not waived or counted as qualified here.
- Subsequent header-only/default cleanup: **1,255 unit PASS**, immutable 23-case corpus
  **1 integration PASS**, NUMSAFE2 **9 PASS** (`core-b-header2.log`). The additional
  regression replacing live tool defaults with zero/NaN while retaining explicit
  operation rates and depth list passes (`no-live-defaults-b.log`).
- `cargo test --offline -p ivac-cli`: **13 PASS**. Actual SEC1 and NUMSAFE3 CLI probes
  pass, retaining external CPS refusal, pass-budget refusal and publication assertions.
- Core strict pedantic Clippy with the release gate's documented allows and rustfmt
  pass. Small inherited test-style lint findings were fixed; no lint was disabled.
- No workspace-wide, frontend, WASM, installed-native, physical or final-release
  acceptance is claimed by phase B. Later source qualification remains mandatory.

Every fixed pre-removal program, toolpath and provenance comparison passes unchanged.
Header-only milling setup carries zero (absent) rate metadata when there is no cutting
operation; it no longer reads a library's unrelated rates or spindle minimum. No
machining operation is seeded or clamped by that path. The retained actual-cut corpus
is byte-identical. Legacy fixtures requiring invalid clamping were rewritten as
refusals; positive volume fixtures explicitly author the same effective geometry.

## Phase C — frontend dependency detachment

Parent source: `c5b13c9348aa0fd5ddbb08cdb27e78358e7e5ba4`.
The actual authored-v9 UI remains the product path: CuttingParameters,
TestedPresetsLibrary, named collections, reviewed copy and attestation are retained.
Removed the disconnected recommendation, source-pack, adaptation and Trial UI/state
closure plus old readers and their retired behavior tests. `phase-c-test-retirement.json`
records per-file removed tests, embedded safety concerns and replacement locations.
322 unused translations were removed using the unchanged i18n dead-key scanner;
`keys.ts` was regenerated, not hand edited. MAT-1 keyboard focus was moved to its own
component helper with equivalent boundary/no-selection tests.

Physical identities now have a neutral creation/load helper. Existing physical IDs
are never overwritten; old **library** `fsLocalIdentity.id` is retained as the same
physical identity when absent, while reference bindings remain inert unknown metadata.
This does not load or migrate v1–8 projects or require old source stores. Generic
machine/tool settings and unknown fields remain preserved. No application-data file
or browser store was cleared. V9 generation uses only the neutral physical ID.

Active source enablement, fsContext state and legacy rate helpers were removed.
Operation assignment routes through existing v9 role rebinding: authored numbers
survive actual tool changes, new roles stay visibly incomplete, and Undo/Redo restores
tools and values. Geometry, lifecycle and stale-export tests now use the actual v9
contract, including malformed geometry and late async mutation refusal.

Qualification on the phase-C source fingerprint: 170 frontend test files, **1489
passed, 1 skipped** (existing optional German translation completeness), 0 failed;
Svelte/TypeScript 0 errors/0 warnings; ESLint/Prettier PASS; production build PASS.
The real freshly compiled WASM adapter/serialization tests ran in that full suite.
Initial wasm-pack invocation lacked cached tools on PATH and failed with EPERM;
rerun with the existing matching wasm-bindgen/wasm-opt cache passed offline and
recorded the verified package fingerprint. No guard was disabled. Vite retains its
large-chunk warning; no claim of installed-native or physical acceptance is made.

Transport declarations/core adapters still exist for the next ordered phase D;
they are not consumed by the retained product UI. Full final-source gate, TEST1/C2
and desktop/narrow UX walkthrough remain phases F–I.

## Phase D — retired transports and source storage

Parent source: `a15d99755c4e08039a6c0e6ba000e843f02b31de`.
Removed the old core recommendation/capture adapters and their Trial implementation,
Tauri source-pack commands/storage module/registrations, `/feeds-speeds` HTTP route,
WASM recommendation/capture/pack exports, worker messages and all client wrappers.
Removed the advertised `feeds-speeds` capability on HTTP/Tauri/WASM. Current presets,
generation, simulation, bundled CPS and native export remain registered.
No filesystem capability was changed: `fs:allow-stat`, import byte bounds and program
publication authorization remain intact. No obsolete user source files were deleted.

`project::MaterialIdentity` now owns the exact id/revision pair without the old crate.
The Rust project and operation-parameter request structures reject unknown fields;
removed fields cannot silently disappear at the JSON boundary. Frontend authoring
models now describe only v9; obsolete context/recipe keys are explicitly refused on
load, including null placeholders. Unknown unrelated project-file extensions remain
preserved by the existing file lifecycle, separate from the strict generation request.
OpenAPI schemas were regenerated by xtask and TypeScript by openapi-typescript.

Real WASM testing found that serde-wasm-bindgen 0.6.5 visits only declared struct
fields, so Rust `deny_unknown_fields` alone does **not** reject retired keys there.
A bounded transport check now rejects `fs_context`, `applied_cutting` and
`fs_full_width` before JS conversion on simple/two-sided/streaming/edited-drawing
entry points. It does not calculate rates or replace neutral validation. Actual
WASM tests exercise object/null/false/undefined forms and ensure refusal before
stream events. This is an explicit retired-field guard; it is not a claim that the
JS transport rejects every unknown future field in every struct.

Qualification: core integrations all passed (216), with **1229 unit tests passed**
after repairing two old wire fixtures. Those fixtures contained misplaced fields
that the previous reader ignored (`helix`, disabled tabs/leads and a root `tabs`);
their unchanged analytical offset assertions now exercise the current contract.
The fixed 23-program corpus is unchanged and passes. CLI/server/native-file-export
suites: **43 passed**; scoped Tauri non-watcher suite: **8 passed, 5 watcher tests
filtered** for this phase only. Full unfiltered watcher qualification remains
mandatory in H. Native WASM crate: **24 passed**, including authored-v9 TEST1 replay
and collision diagnostics; old bridge fixture requests are authored once using the
existing test-only helper. No production migration/default path was introduced.
Strict/pedantic Clippy on all targets/features of core, CLI, server, Tauri and WASM
passes. Small pre-existing CLI lint issues and the native WASM simulation tool
fixture's missing neutral fields were corrected without changing assertions.

Frontend: **1492 passed, 1 existing optional locale skip**, 171 files; typechecking,
lint, production build and fresh actual WASM PASS. Real SEC1 and NUMSAFE3 CLI probes
PASS. Source/schema checks PASS. Initial failing logs are retained locally; final
receipts identify repaired fixtures and the WASM boundary defect, not waived tests.
One build failed with ENOSPC; only this checkout's regenerable Rust incremental
cache was removed and retries used `CARGO_INCREMENTAL=0`. Another build correctly
refused to stamp WASM while source inputs changed; it was rerun to a verified stamp.

The old recommendation crate remains in Cargo and the old shipped datasets remain
on disk until ordered phase E. No retained production Rust/TS consumer imports its
API or types. Generic physical IDs/max RPM, neutral MAT-1 vocabulary and role safety
have independent owners. Final-source workspace gate, TEST1/C2 and UI acceptance
are still pending; no installed-native/browser-transfer or hardware claim follows
from these adapter and filesystem unit tests.


## Phase E — recommendation crate and shipped assets removed

Parent: `a9d25bde4ad2c0102ed95e17874a00c6d7906ea3`. Removed the 22 tracked
crate files and 11 shipped Sienci/license/registry assets, workspace/dependency and
lock entries. No user pack, project, application directory or IndexedDB data was
removed. Cargo metadata has no recommendation package or edge. Neutral material
identity, physical identity, limits, taxonomy and safety owners remain unchanged.
The removed families and their retained safety coverage are enumerated in
`phase-e-test-retirement.json`; obsolete numerical/ranking policies have no successor.

Auditing the original request validator uncovered the missing rejection of an
explicit zero machine minimum RPM. Neutral validation now rejects it alongside the
three other zero known ratings, without mutating rates; absent limits remain valid
with disclosure. The new regression proves all four refusal paths and unknowns.
This is a transferred guard, not a new rating or calculator.

Final core unit suite: **1230 PASS**; strict contract **3 PASS**; fixed 23-case corpus
**1 PASS**, no output changes. The preceding complete core integration run also
passed. Strict pedantic core Clippy PASS. WASM rebuilt and verified on final E source.
Freshness checks: **12 PASS**, including source deletion/old-mtime changes in neutral
validators/presets/project and independence from removed assets. Full workspace,
watchers, TEST1/C2 and visual acceptance remain mandatory in later phases.

Frontend final E: **1492 PASS, 1 existing optional locale skip**; production build PASS.


## Phase F/G — indirect fixture contracts and extended output comparisons

Parent `f4896e4`. Eighteen reviewed explicit v9 PERF1 request copies now live in
`tests/fixtures/perf-v9`; historical requests/evidence remain unchanged. Five
cache/cancellation/remaining-stock regressions read those inputs directly, without
per-run fixture seeding. Feed invalidation changes the operation role value. The
frontend fixture author is explicitly test-only and writes new candidates under
dist-test. Native/WASM/cancellation scripts use current requests and explicit output
locations. No thresholds or required gates were relaxed. Missing-layer cases reach
`op_source_missing_layer` / `op_source_empty`, with no cutting moves (the native
LinuxCNC header retract is retained), rather than failing early on a version check.
B8 reaches the same independent V-Carve overcut refusal.

Five affected Rust tests and strict pedantic checks PASS; frontend script lint PASS;
fresh WASM rebuilt and verified. Eighteen native requests were compared with a fresh
build of an archived **951b12f** source tree using identical v9 input bytes: program,
toolpath, provenance, line index and stats are identical for every successful case;
two B8 cases retain identical overcut errors. Historical PERF1 Test-outline refusals
pre-date the approved FS2 baseline: both 951b12f and current code generate identical
programs. The earlier records are preserved, not rewritten as current expectations.

The strict native/WASM PERF1 comparison **FAILS** on two pre-existing V-Carve medial
cases: square (5376 native / 5368 WASM moves) and Test text (18342 / 18351). These are
executable-program differences, not merely comments or the 1e-10 preview allowance.
Investigation compared current WASM against the preserved d6b08fe package, whose
core/WASM source is identical to 951b12f (`git diff` empty): all nine before/after
WASM cases have identical successful outputs or identical errors. All 18 native
before/after cases likewise match. Therefore FS-RESET-3 introduces no difference,
but cross-runtime V-Carve output equivalence remains unqualified. The strict harness
continues to fail; no assertion/tolerance was changed to hide this. Determining the
underlying cross-target numerical cause is separate technical debt.

`/usr/bin/time -l` fails on sandbox `sysctl kern.clockrate`; full resource profiling
is unavailable. Separate direct functional repeats are not claimed as RSS or speedup
measurements. Native cancellation keeps the original 200 ms p95 threshold. HTTP/SSE
and installed-platform evidence remain separate. Final-source full gate, TEST1/C2,
canonical 23-case comparison and visual acceptance follow this boundary.

Native cancellation: all 70 observations cancelled without a valid result. The first
run passed the p95 target; the repeat with no other compiler/benchmark running
**FAILS** unchanged 200 ms p95: candidate many-segments 229.928 ms, Test/50 ms
202.479 ms; control Test/50 ms also 200.643 ms. Test/1 ms is 0.555 ms candidate.
This is not a clean performance/cancellation-latency qualification. The functional
cancel/cache regressions pass, but host variability versus a latency regression
requires a controlled follow-up; no timeout, polling interval or assertion was changed.


## Bounded UX correction during final walkthrough

Source parent `ded2348`. Three machine-limit tooltips still described legacy automatic
clamping. Their English and German strings now describe v9 milling refusal with saved
values unchanged and absent ratings unknown. No layout, validation or numerical code
changed. ESLint/Prettier PASS; frontend **1492 PASS / 1 existing locale skip**, including
12 source-freshness checks. Final-source pre-release will be rerun after this commit.
