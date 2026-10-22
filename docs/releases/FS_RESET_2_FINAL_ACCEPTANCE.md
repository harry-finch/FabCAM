# FS-RESET-2 final acceptance and FS-RESET-3 preflight

Date: **2026-10-22 (Europe/Paris)**. Source: **`62440f87e06d36eacfc707c4d850680c2899879e`**,
branch **`FabCAM-v1.1`**. Engineering: `ivac-u8wzv`; human gate **`ivac-y25vq` remains OPEN**.
References: [architecture](FS_RESET_ARCHITECTURE.md), [implementation](FS_RESET_2_IMPLEMENTATION.md),
[Accepted ADR-0030](../shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md).

## Decision

**NO-GO for FS-RESET-3 at this checkpoint.** The v9 CAM architecture and targeted machine-output
checks remain sound within their recorded scope. The watcher failures are now reproduced on
the pre-FS-RESET-2 baseline under equivalent conditions. However, two concrete acceptance gaps
require separately authorized, bounded production corrections before legacy removal:

1. **`ivac-tnu7d`: the active machining report reads legacy tool defaults, not v9 operation
   values, and omits an independently assigned spotting tool from its tool list.**
2. **`ivac-c93e1`: native collection import invokes `fs::stat`, which the current Tauri
   capabilities do not grant.** File-dialog path selection does not grant a missing command.

No production code, tests, timeouts, permissions or safety guards were changed. No legacy
infrastructure was removed. The next human decision is authorization for those two repairs,
not approval to begin FS-RESET-3. Browser file transfer remains blocked by its authorization
mechanism and has a manual acceptance procedure below; that tooling refusal alone is not a
product defect. Installed-platform and physical acceptance remain separate.

## A. Baseline reconciliation and acceptance matrix

All four implementation commits are ancestors of the inspected branch HEAD, in the approved order:

| Slice | Commit |
|---|---|
| Core/contracts/storage | `ac9e38961be7b047a35a0f8612c670f437b22321` |
| Operation workflow/v9 | `b6a41d6251c490a26620b9f0e376debff8de2b7a` |
| Preset library/MAT-1 | `f681737760e7a63903ec3927a36962cfee4985c5` |
| Actual-role integration | `356e1730a37172a03b0a40054927075ea28525b5` |
| Documentation handoff | `62440f87e06d36eacfc707c4d850680c2899879e` |

Initial tracked Git status contained only pre-existing edits in `RELEASE_ROADMAP.md` and
`STATUS.md`; no uncommitted production changes were treated as qualified. This checkpoint
adds documentation, exact inventories and diagnostic evidence only. Those two original edits
remain unstaged. No other agent is active in this task tree. The other worktrees remain on
`7e1c24b937e9bc01aa320537c835ffc30fc9389c` and `85a93c4a467ff5965655d5737354d9500c2e94fc`;
`main` remains `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`.
A clean, detached baseline checkout was created only for comparison and removed afterward.
No pre-existing worktree or user application-data directory was changed or reset.

| Capability | Implemented | Software-qualified at this checkpoint | Investigation / human acceptance / later gates |
|---|---|---|---|
| Explicit rates per actual role; no generation default seeding | Yes | 7 contract + 12 integrated v9 tests included in the 25-test cutting filter; actual WASM checks | Human operation UX acceptance open; no machining acceptance |
| Neutral machine/tool/geometry checks, unknown-limit disclosure | Yes | Positive/negative core tests; known limits refuse without clamping | Keep specialized restrictions and full final-source gate |
| V9 save/open, incomplete editing, reviewed copy and Undo/Redo | Yes | Frontend v9/workflow/WASM subset passes | Real saved-file browser round-trip not established here |
| Collection schema, native atomic storage and stale writes | Yes | 6 core preset tests; 3 native storage tests | Native dialogs are not qualified by these tests |
| Browser collection create/export/import | Implemented | Empty collection created on separate test origin; automated workflow tests pass | Download denied; populated real round-trip/collision cycle remains manual |
| Native collection file import | Implemented but blocked by capability mismatch | Static ACL expansion reproduces missing `stat`; mocks do not test this | `ivac-c93e1`, bounded repair approval required |
| Reported setup rates/tool list | Active but incompletely migrated | Exact helper diagnostic shows v9/default discrepancy | `ivac-tnu7d`, blocking truthful operator handoff |
| Watcher source reload | Existing implementation unchanged | Same 3 passes / 2 failures before and after FS-RESET-2 | Backend/event-delivery limitation in this environment; host verification pending |
| Ordinary supported two-sided / explicit Drill | Yes, bounded | Front/back byte/provenance and distinct/shared Drill role tests pass | Automatic injection excluded; controller/platform/physical gates unchanged |
| Native program publication / CLI streaming | Retained | 11 file-export tests pass; v9 stream preflight/byte tests pass | Per-file atomicity only; partial pair possible; streaming is not full-program simulation |

Architectural acceptance is recorded; final product/UX acceptance is **not** inferred. The
previous implementation report is a historical engineering receipt, not a complete native
acceptance certificate. Its blanket “dialog-granted scope” shorthand needs the qualification
in section C: static `$HOME/**` permissions already exist, and command permission and path
scope are different checks. Its generic “explicit values” statement does not cover the
unmigrated ReportDialog identified here.

## B. Native watcher investigation

Source and exact environment: [comparison](fs-reset-final-evidence/watcher-comparison.json),
[dependency versions](fs-reset-final-evidence/watcher-dependency-versions.json).
macOS 26.5.2 / aarch64; rustc/cargo 1.88.0; notify 8.2.0, notify-types 2.1.0,
fsevent-sys 4.1.0 and tempfile 3.27.0 on both application revisions.
Baseline: `184c2088beda0d7bb65a6ea97b7e19f6d28c658d` (before slice 1).
The watcher source bytes are identical. The relevant lockfile versions are identical;
the application manifest gained storage dependencies, not a watcher implementation change.

| Conditions, same host/sandbox | Baseline | Current |
|---|---|---|
| `cargo test --offline -p ivac-tauri --lib watcher::tests`, normal test concurrency, default temp root | 3 PASS / same 2 FAIL | 3 PASS / same 2 FAIL |
| Same command, `-- --test-threads=1` | 3 PASS / same 2 FAIL | 3 PASS / same 2 FAIL |
| Same compiled test binaries, serial, `TMPDIR` set to separate private directories under this workspace | 3 PASS / same 2 FAIL | 3 PASS / same 2 FAIL |

Logs: `fs-reset-final-evidence/watcher-{baseline,current}-{parallel,serial,workspace-tmp}.log`.
Cargo target artifacts were shared to avoid rebuilding unrelated dependencies, but the
baseline and current test binaries were separately compiled from their actual source paths.
Temporary files were created only by the tests and a private probe, never real CAD sources.
The same unchanged 100 ms post-subscription wait, 2 s single-event deadline and 600 ms debounce
observation window were used. Serial execution rules out concurrency between these five tests
as a sufficient explanation. Moving to the workspace rules out the default temporary-directory
location as a sufficient explanation; it is not a test of a different filesystem implementation.

Implementation inspection: `set_paths` watches parent directories nonrecursively, canonicalizes
paths, accepts Create and relevant Modify events, matches declared targets and throttles emits
for 200 ms. A replacement initializes a new watcher before replacing the old one. The notify
FSEvents backend schedules/starts a run loop and signals readiness through a channel; the
upstream start return value is not checked there. This is a diagnostic observation, not proof
that the start call is the failing mechanism.

An independent **diagnostic-only** raw notify probe records zero RecommendedWatcher callbacks
before any application path filter or debounce, while PollWatcher sees three changes on each
of the two private roots. See [probe source](fs-reset-final-evidence/watcher-probe.rs),
[manifest](fs-reset-final-evidence/watcher-probe.Cargo.toml), lock and
[output](fs-reset-final-evidence/watcher-raw-probe-locked-deps.log). It uses a minimal standalone
dependency graph with common versions matched to the repository; it is not a production
fallback or a claim of complete application feature-graph equivalence. Polling is an observation
control only; production behavior is unchanged.

**Disposition:** demonstrated pre-existing failure in this host/sandbox environment, not
introduced by FS-RESET-2. Evidence points upstream of the app's filtering/debounce, toward
native event delivery. It does **not** distinguish sandbox denial from host/FSEvents behavior
or prove installed applications fail. No synchronization-only test fix is justified by these
results; no timeout increase or watcher patch is proposed. Keep an outside-sandbox/native-host
reproduction as a platform acceptance task. This known baseline exception alone would not
prevent staged cleanup, provided it remains explicit and the tests stay enabled.

## C. Collection file acceptance and discovered blockers

### Browser result and manual procedure

The browser used `http://fsreset-source.localhost:5173/?api=wasm`, a separate origin from the
user's ordinary local project store. It began empty. A collection named
**FS-RESET SOFTWARE ONLY — no machining evidence** was created. At the first Export attempt,
the browser tool rejected authority to download files from that origin and reported declined
approval. No downloaded file was read, no synthetic tested preset was added, and no target
origin/import/collision cycle was performed. No alternative browser, API, privileged command
or indirect transfer was attempted. “Collection export requested” in the UI is not proof that
a file was downloaded. [Screenshot](fs-reset-final-evidence/browser-empty-synthetic.jpg).
The empty test collection is retained on that disposable origin; no private workshop pack was
accessed, overwritten or deleted. No synthetic attestation was represented as physical evidence.

Manual procedure, pending human execution in a permitted browser/profile:

1. Use a disposable browser profile/origin and a dedicated temporary folder. Confirm its
   collections list is empty; keep existing workshop stores untouched.
2. Create a collection named `SOFTWARE ONLY — NOT MACHINING EVIDENCE`. Create an example
   operation with synthetic machine/tool physical IDs and an exact material revision, then
   explicitly save one software-fixture preset. Name and notes must both say
   `SYNTHETIC ATTESTATION FOR SOFTWARE TEST ONLY — NOT PHYSICALLY TESTED`. The maintainer's
   authorization for this fixture permits testing the attestation control, not a real claim.
   Record the generated collection/preset IDs, roles, RPM/feed/plunge, schedule and context.
3. Export through the UI to the private folder. Open the actual downloaded JSON. Check
   `format=fabcam-preset-collection`, `version=1`, IDs/name, parameter values, context signature,
   notes and `tested=true` as the explicitly synthetic source claim. Record a file hash.
4. Import that exact file into a second clean disposable origin/profile. Inspect restored
   values and identity. The UI must disclose imported author attestations without claiming
   current-machine testing. Export again and compare parsed documents; whitespace is immaterial.
5. Re-import into the same target: cancel first and verify no change. Exercise independent
   copy: new collection ID, preserved preset values, original unchanged. For replacement,
   review the collision explicitly; create a concurrent edit from a second tab before the
   reviewed write and require stale-token refusal with both original content and newer edit
   preserved. Refresh and deliberately replace only the disposable fixture if testing success.
6. Re-export the original source collection and compare its parsed content/hash with step 3.
   Record browser, source SHA, files, outcomes and screenshot. Remove only explicitly disposable
   test data through a permitted procedure afterward; do not use a local-data reset.

Automated collision/attestation/selected-file tests pass, but they do not replace these actual
browser-storage/file checks. Pending manual acceptance is not, by itself, a product defect.

### Native boundary: proven configuration gap

`frontend/src/lib/services/preset-collection-files.ts` calls `stat(path)` after the open dialog,
then checks the 2 MiB bound before `readTextFile`. Expanding **both** default and desktop
capabilities against the generated ACL manifest yields no `fs::stat` command grant:
[reproducer](fs-reset-final-evidence/inspect-native-acl.py),
[result](fs-reset-final-evidence/native-import-acl.json).
`fs:default` grants app-directory read commands, but not `stat`; the additional permissions
include text read/write and binary write, and the desktop capability adds window state only.
The dialog can extend the selected path scope, not enable a disabled command. Native import
therefore has a source-demonstrated permission mismatch before reading a selected collection.
No live installed dialog was run; exact displayed host error text remains to be captured.

**`ivac-c93e1` requires authorization before a repair.** Review a minimal command permission
with appropriate path scope, or a bounded scope-checked import operation; preserve the size
check before unbounded reads and propagate errors. Do not remove `stat`/size checks merely to
make the mock green, grant blanket new filesystem permissions, or bypass the file dialog.
Add an ACL-aware regression and then qualify a selected-file native round-trip, cancellation,
oversize/malformed refusal, stale writes and preservation on failure.

Existing static `$HOME/**` (and named home subdirectories) text-read/write permissions remain
unchanged. The report must not describe the existing policy as exclusively one dialog-selected
path. `publish_program_file` still checks `app.fs_scope().is_allowed(path)` before staging;
new current collections still accept safe IDs under app-data `presets/collections`, reject
symlink/traversal/oversize/stale writes, and use same-directory atomic replacement. These three
native storage tests and eleven `ivac-file-export` tests pass on this source. Native JSON
backup export is not a transactional machine-program export or multi-document transaction.

### Active setup-report values

[Exact helper reproduction](fs-reset-final-evidence/report-rates-reproduction.json),
[reproducer](fs-reset-final-evidence/reproduce-report-rates.mjs): Profile Finish has authored
`13000 / 410 / 110`, while library defaults are `18000 / 2500 / 500`.
`operationRates` from the legacy `feeds-speeds.ts` returns the latter. Active
`ReportDialog.svelte:80` calls it for Profile/Pocket and directly reads defaults for other
operations. Both the on-screen report and its export model use `opRates`. The tool list at
line 104 includes primary/finish IDs but not an independent `spotFirst.spotToolId`.

**`ivac-tnu7d` requires authorization before a repair.** This is an inaccurate operator setup
report, not evidence that the v9 CAM emitter uses those wrong values. The actual-role output
regressions pass. A fix should use neutral actual-role selectors, show incomplete values as
incomplete, include Spot/Chamfer/secondary tools, and never synthesize defaults. Qualify
Profile, dual-tool Pocket and shared/distinct Drill roles, edits/reopen, missing values and
report tool attribution. Keep report/simulation qualification distinctions clear.

## D. V1.1 two-sided capability matrix

The maintainer explicitly confirms the exclusion. No assistant is implemented here.

| V1.1 capability | Disposition / evidence |
|---|---|
| Ordinary supported native two-sided operations without auto holes | Retained; new v9 test compares exact front/back bytes and resolved provenance with unchanged inputs |
| Explicit Drill registration preparation | Ordinary visible Drill operations with explicit Drill/Spot/Chamfer tools and values; same/distinct-tool output tests pass |
| Missing/invalid drilling rates, invalid peck/chamfer/reach | Refused by normal v9 validation; no default substitution |
| Automatic dowel injection in v9 two-sided generation | **Unsupported**; explicit error occurs before injection; existing settings show notice and explicit disable action in Stock panel |
| Two-sided CPS / translated XY registration / back-face surfacing | Prior refusals retained; no broader ShopBot/controller or machining capability claim |
| Per-side export | Qualification and authorization retained; per-file atomic publication, not pair atomicity |
| Registration creation assistant | Future only, `ivac-7cotu`: generate ordinary editable Drill operations with Undo/Redo and normal validation; no hidden operations |

`two_sided_emit.rs:153` gates v9 before `inject_dowel_op`. The Stock panel defaults to a flip
axis only and explains explicit Drill preparation. The new regression exercises ordinary
front/back output, automatic-hole refusal and invalid back-side rates. HPGL/non-two-sided
entry paths retain their existing single-program semantics; this exclusion does not claim
new two-sided non-milling support. No incorrect successful automatic injection was found.

## E. FS-RESET-3 dependency preflight (no deletion authorized)

### Reproducible inventory

The source-bound inventory contains **4,386 reference rows in 254 operational files**,
plus **302 protected historical files**, from 3,194 tracked files at the source SHA above.
[Receipt](fs-reset-final-evidence/inventory-receipt.json),
[per-line references](fs-reset-final-evidence/legacy-references.csv),
[per-file classification and hashes](fs-reset-final-evidence/legacy-files.csv),
[protected history](fs-reset-final-evidence/historical-references.csv), and
[reproduction script](fs-reset-final-evidence/inventory.py) are the exact inventory.
The script combines lexical references, import edges, whole legacy-module closure and
explicit indirect gate/fixture candidates. It does not prove arbitrary dynamic reachability.
Untracked user data is deliberately excluded from removal candidates. Re-run after each
future boundary change. Classification applies to the cited responsibility; mixed files
must not be deleted wholesale. SAFE TO REMOVE means after consumers have been detached.

| Boundary / consumers | Classification | Required disposition |
|---|---|---|
| `ivac-feeds-speeds` calculation/ranking/adaptation, source datasets | SAFE TO REMOVE after detachment | Remove obsolete product behavior, never transplant ranking behind the new preset UI |
| Crate `model`, `dataset`, `validation`, `source_pack`; core `Project.material` identity | REPLACE FIRST | Move useful generic identity/material contracts to neutral ownership; separate schema/data parsing from reusable safety assertions |
| `ivac-core::feeds_speeds`, `cutting_parameters` legacy branch | REPLACE FIRST | v9 is authoritative; internal version-absent fixtures still use saved-rate/effective-machine/applied-project validation. Replace callers before deleting fallback |
| `setup_resolver` | REPLACE FIRST / KEEP | Remove old applied-context warnings/manual-origin metadata only after preserving general tool, geometry, SAFE-Z, fixtures, origin and specialized restrictions |
| Internal CLI geometry-to-Setup generation | HUMAN DECISION | Retain with an explicit initialized operation contract or consciously exclude capability; do not silently drop CLI behavior when removing fallback |
| HTTP, Tauri, WASM, worker recommendation/source-pack endpoints | REPLACE FIRST then SAFE TO REMOVE | Detach registries/commands/exports, generated OpenAPI/TS and consumers together; retain current v9 generation and preset transport |
| Native source-pack storage and browser old stores | SAFE TO REMOVE after detachment | Remove access code, not existing user-owned files or IndexedDB data |
| Active frontend project settings, operation/tool commands, workspace/file-load normalization | REPLACE FIRST | Remove source-settings and legacy slot normalization while retaining creation-only defaults, role edits, Undo/Redo, stale-result guards and generic inventory |
| Tool/machine physical identity with `fsLocalIdentity` fallback | REPLACE FIRST / KEEP | Preserve neutral physical/provenance identity before removing old reference metadata |
| Report/palette selectors and `operationRates` | REPLACE FIRST | Resolve `ivac-tnu7d`; never substitute defaults for actual v9 role values |
| Old trial/recommendation/preset-selection components and stores | SAFE TO REMOVE after active imports detach | Explicit current preset library, review/copy behavior and attestation remain |
| Legacy persistence geometry guards | REPLACE FIRST | Current `project-geometry-validation.ts` already owns generic checks; migrate residual test coverage before deleting old helper |
| Sienci assets/registry/public JSON and source mapping | SAFE TO REMOVE after test/consumer detachment | Preserve license/history evidence; MAT-1 generic family/alias/custom material presentation remains |
| Translation keys including `fs.material`/finishing labels | REPLACE FIRST / KEEP | Prefix alone is not deletion authority: active generic UI still consumes some keys |
| `wasm-package-state.mjs` dependency fingerprints | REPLACE FIRST | Remove obsolete FS/data inputs but retain fail-closed freshness checks and updated source closure |
| CI, pre-release, SEC1, NUMSAFE3, perf, TEST1 adapters | REPLACE FIRST / KEEP | Convert old request fixtures to v9 before removing fallback; retain budgets, negative assertions, complete output comparisons and C2 |
| Old project-version tests, examples, benchmarks, `explicit-project.fixture.ts` | TEST FIXTURE | Test-only explicit conversion is not runtime v1–8 migration; keep geometric/performance baselines with new actual-role inputs |
| Reports, ADR history, V1 evidence, frozen branches/worktrees and private packs | KEEP | No mass deletion, historical rewriting or local-data reset |

Import-edge supplements: [frontend](fs-reset-final-evidence/frontend-import-edges.txt),
[Rust](fs-reset-final-evidence/rust-legacy-edges.txt),
[i18n](fs-reset-final-evidence/i18n-consumer-edges.txt).
A concrete indirect gate dependency is `numsafe3-cli-check`: its old TEST1 request lacks
v9 cutting values. An early format rejection must not be mistaken for the expected budget
refusal. Repair the fixture before considering that gate qualified after removal.

### Safety ownership (retained, not legacy deletion targets)

| Invariant | Current v9 owner / evidence | Removal constraint |
|---|---|---|
| Required rates and actual role assignment | Core `cutting_parameters`, v9 frontend contract; cutting and real-WASM regressions | Missing editable values may persist; generation must refuse them, never seed defaults |
| Rough/finish/spot/chamfer attribution | Actual-role selectors, setup resolution, emitted provenance; dual Pocket/shared-distinct Drill tests | Retain independent tool IDs and rates; repair report consumer separately |
| Known machine RPM/feed limits | Neutral cutting validation and machine constraints | Keep positive/negative bounds, no clamping; unknown limits remain disclosed |
| Known tool RPM/geometry limits | Neutral cutting validation plus generic setup/tool validation | Do not remove specialized mandatory restrictions or fabricate ratings |
| Depth, engagement, flute/reach restrictions | Cutting and operation/setup validation | Preserve integer/depth/overlap/peck/reach coverage formerly bundled with experimental recipes |
| SAFE-Z, fixtures, work origin | Generic setup resolver and pipeline guards | Remove only legacy metadata paths, never these checks |
| Numeric/resource budgets | Geometry/toolpath/pipeline limits and CLI bounded streaming contracts | Keep adversarial input and output limits; correct v9 fixture reachability |
| Simulation/full-program qualification | Simulation/pipeline qualification and export eligibility | Streaming is not full-program simulation qualification |
| Generation/export freshness | Frontend operation revision, stale generation and export checks | Keep edit/undo/preset-copy invalidation behavior |
| Native file-export authorization | Tauri `publish_program_file` checks `app.fs_scope().is_allowed` before `ivac-file-export` staging | Preserve authorization and atomic per-file publishing; collection import ACL is a separate open defect |
| Two-sided partial publication | `ivac-file-export`, per-side result/error contracts | A complete first side may remain if second publication fails; disclose partial result, do not claim pair atomicity |
| CLI streaming refusals/limits | Public v9 entry validation and CLI/pipeline streaming restrictions | Preserve unsupported-case refusals, explicit limitations and bounded memory/output behavior |

The targeted tests establish retained behavior for their exercised cases, not exhaustive
physical safety or platform acceptance. Public v9 generation is separated from recommendation
selection; internal old-format paths remain real dependencies until replaced.

### Test migration and intentional exclusions

No tests are deleted or weakened at this checkpoint. Before deleting each family, retain
its non-obsolete safety coverage in the indicated neutral/current suite.

| Legacy family | Intentionally obsolete behavior | Required retained/replacement coverage |
|---|---|---|
| Calculation/ranking/adaptation/sensitivity | Product recommendations, score/order and adaptive math | Explicit role bounds, malformed/non-finite values, known limits, no fallback/clamping |
| Source packs, revision policy, lab recipe lifecycle | Old formats, ranking eligibility and recipe revisions | New collection CAS, validation, atomic failure preservation, collision/copy semantics, attestation separation, size/path boundaries |
| Experimental trial validation | Old trial promotion/eligibility | Positive/integer cutting fields, depth/flute/geometry, known machine/tool limits, specialized restrictions |
| Ranked selectors/comparison | Candidate ranking and adaptation | Reviewed copy, context mismatch disclosure, current tool/machine/material preservation, pinned review state |
| v1–8 persistence and old applied slots | Backward compatibility (not required) | v9 round-trip/rejection, incomplete editable operations, geometry binding/replacement, Undo/Redo, custom assets |
| Core pipeline and post fixtures | Legacy parameter representation | Explicit actual-role fixtures; exact valid output/provenance comparisons, both sides, Drill roles, TEST1 including C2 |
| MAT-1 source mapping | Sienci-specific lookup | Generic family/alias/custom-material identity and presentation |
| Native source storage/security | Obsolete source-pack commands | Current collection storage plus program export authorization, stale writes, failure preservation and partial publication |
| WASM/schema/worker exports | Recommendation APIs | Current v9 serialization, malformed input, role contracts, generated-model agreement and package freshness |
| Old translation/component tests | Dead recommendation/trial UX | Active material/tool labels, explicit editor, preset review/library interactions |
| CLI/NUMSAFE3/release scripts | Legacy request construction | Requests must reach the intended safety check; preserve precise refusal/output assertions and complete release gates |

### Exact proposed removal sequence — only after separate human authorization

1. Resolve the two acceptance blockers in separately authorized bounded work; requalify
   affected reports, native permission contract and collection transfer. Obtain the human
   decision for direct CLI geometry-to-Setup behavior. Reconcile the inventory to that SHA.
2. Extract remaining neutral identity and generic validation ownership; convert internal
   CLI/test/benchmark inputs to explicit roles. Keep old code until all retained callers
   and negative invariants are qualified. Commit this boundary separately.
3. Detach active frontend normalization, reporting, source/reference settings and obsolete
   UI imports; preserve generic inventory/MAT-1, role edits and preset library. Qualify
   creation, reopen, Undo/Redo, stale generation and actual WASM contracts; focused commit.
4. Remove obsolete HTTP/Tauri/WASM/worker endpoints, storage command registrations and
   associated capability declarations; regenerate schema/TS bindings and verify native
   scope/error contracts. Do not delete application-data files. Focused transport commit.
5. Remove core legacy fallback/adapters and the recommendation crate once remaining imports
   are zero; remove obsolete workspace dependencies. Re-run core/CPS, all operation roles,
   CLI/HTTP/native/WASM and exact output comparisons before committing.
6. Remove dead assets, source registries, components, translations and obsolete test families
   using the coverage mapping above. Update build fingerprints, gate inputs and CI closure;
   preserve V1/history and all retained guard tests. Re-run inventory and full release gate.
7. On the final source revision, run complete TEST1 **including C2**, output corpus and
   required frontend/Rust/WASM/schema/native gates. Investigate every unexplained output
   difference. Publish the source-bound FS-RESET-3 report and stop for human acceptance.

These are reviewable future boundaries, not permission to execute them now. No FabSpeed API
integration belongs to this sequence. Installed-platform and physical qualification remain
separate release requirements; an explicit pending manual test cannot hide a known defect.

## F. Source-bound qualification

All following tests exercised production source `62440f87e06d36eacfc707c4d850680c2899879e`;
this checkpoint changes documentation/evidence only. Commands used offline Cargo with
`RUSTC_WRAPPER=''`. No complete release gate was rerun without production changes.

| Check | Result | Evidence |
|---|---|---|
| `cargo test --offline -p ivac-core --features cps cutting_` | 25 PASS, including v9 roles, two-sided output and refusal, golden output and streaming cases | [log](fs-reset-final-evidence/core-cutting.log) |
| `cargo test --offline -p ivac-core --features cps presets::` | 6 PASS | [log](fs-reset-final-evidence/core-presets.log) |
| `cargo test --offline -p ivac-tauri --lib preset_collections::` | 3 PASS; native store, not installed dialog | [log](fs-reset-final-evidence/native-collections.log) |
| `cargo test --offline -p ivac-file-export` | 11 PASS, including second-side failure preservation | [log](fs-reset-final-evidence/file-export.log) |
| Vitest: `preset-collection-files`, `preset-library-v9`, `cutting-v9-wasm`, `cutting-workflow`, `project-v9`, `cutting-contract` | 23 PASS in 6 files; 3 actual-WASM tests; plugin mocks are not native ACL proof | [log](fs-reset-final-evidence/frontend-targeted.log) |
| Current/baseline watcher, parallel/serial/private workspace temp | Each 3 PASS / same 2 FAIL | Section B evidence |
| Raw locked-dependency watcher probe | FSEvents 0 callbacks, polling 3 on each tested filesystem location | Section B; diagnostic only |
| Actual legacy report helper diagnostic | Reproduces incorrect rates; not a passing feature test | [JSON](fs-reset-final-evidence/report-rates-reproduction.json) |
| Resolved native ACL diagnostic | Import calls stat; configured capabilities do not grant command | [JSON](fs-reset-final-evidence/native-import-acl.json) |
| Browser actual file round-trip | NOT COMPLETED: download authority denied | Section C/manual protocol |

Test totals are filtered suite results, not counts of newly added tests. Existing broad FS2
results retain their original source/limitations; this report does not upgrade those claims.
Reproduction scripts are diagnostic artifacts, not changes to production or test timeouts.

## G. Readiness decision and exact next action

| FS-RESET-3 readiness criterion | Disposition |
|---|---|
| 1. v9 CAM safety ownership established | YES for inspected boundaries and exercised contracts; report consumer needs repair |
| 2. Remaining dependencies inventoried | YES, source-bound line/file inventory plus indirect gates; refresh after changes |
| 3. Removal order explicit/testable | YES, sequence above; CLI capability decision still required |
| 4. Watcher failures justified | Pre-existing under equivalent tests; native event delivery cause not conclusively attributed; installed-host verification pending |
| 5. Import/export acceptance classified | Manual transfer pending; demonstrated native permission configuration defect is a blocker |
| 6. Automatic-dowel exclusion recorded | YES; future explicit-operation assistant only |
| 7. No unexplained machine-output/critical regression | Targeted output comparisons pass; misleading setup report is a blocking operator-information defect |
| 8. Frozen V1 unaffected | YES; no V1 branch/worktree/source changes |

**Recommendation: NO-GO temporarily.** The missing browser manual transfer alone would not
block internal cleanup. The two demonstrated defects must not be concealed by mocked tests
or by a large aggregate pass count. `ivac-y25vq` remains OPEN.

**Exact next action:** maintainer reviews this checkpoint and explicitly authorizes bounded
repairs for `ivac-tnu7d` (v9 report rates/tools) and `ivac-c93e1` (native import authorization),
with affected regression tests and source-bound receipts. Those repairs are not implemented
here because this mission requires a hard stop before production changes. Then execute the
manual transfer protocol where authorized, record remaining installed-host watcher limits,
and obtain the separate FS-RESET-3 authorization. None of this authorizes mass removal now.

Final V1.1 release still requires the complete final-source gate, TEST1/C2 and corpus,
installed-platform/dialog acceptance, human UX acceptance and applicable physical/controller
qualification. Software-only preset attestations do not satisfy workshop testing.

Preservation receipt: [tracked-file/worktree/source checks](fs-reset-final-evidence/preservation-receipt.json).
All previously tracked non-documentation file hashes match the starting snapshot; the four
implementation commits are ancestors of the tested HEAD. Other worktree entries are identical.
Evidence hashes: [SHA-256 manifest](fs-reset-final-evidence/SHA256SUMS.json). Existing unrelated
untracked evidence remains outside this commit. The web UI (127.0.0.1:5173) and engine
(127.0.0.1:8766) were still listening at handoff and were left running.


## Authorized blocking repairs — 2026-10-22

The maintainer explicitly approved `ivac-tnu7d` and `ivac-c93e1`, **not FS-RESET-3**.
The original acceptance report above is preserved as a historical baseline.
Repairs start from `76cf19e7cc799e8002e62cdab5dc144785515fe7`, including Sol's committed
UX refinement. Inspected active worktrees and Sol's idle chat before editing. Work is
isolated in `FabCAM-fs-reset-2-repairs`, branch `codex/fs-reset-2-repairs`.
The original checkout, its pending STATUS/RELEASE_ROADMAP changes, V1 worktree and
untracked prior evidence are untouched. No shared UX components or translation files
were modified. Integration into `FabCAM-v1.1` is still a separate human decision;
no merge/rebase/push was performed.

### A — Authoritative machining report

`report-cutting.ts` projects explicit v9 operation inputs, matching both role and assigned
tool ID. Profile reports Finish, not its stored unused Rough slot. Pocket exposes
Rough/Finish (Halfpipe only Rough); Drill exposes Drill, Spot and enabled Chamfer;
Chamfer's optional finish role is retained. Missing, invalid, ambiguous or mismatched
slots render dashes, without reading legacy tool-default values or inventing defaults.
Program-only operations have no tools/rates. Non-milling operations without v9 milling
roles display no milling rates; their primary tool remains listed.

The screen and Markdown use the same formatter and the same actual tool list, including
independently assigned spotting tools. The existing statement that these are saved
operation inputs, not measured machine speeds, remains visible. Generation freshness,
validation, geometry, rates, serialization and machine output were not modified.
Five new semantic tests cover Profile, dual-tool Pocket, Halfpipe, Drill/Spot/Chamfer,
missing/wrong/duplicate rates, edited values, actual v9 snapshot/restore, Markdown and
screen wiring. A disposable real ReportDialog fixture showed the expected three tool
IDs and distinct role values. The actual Save Markdown download was read and compared
against the displayed values. [Downloaded report](fs-reset-repairs-evidence/exported-report.md),
[screen text](fs-reset-repairs-evidence/report-screen.txt),
[capture](fs-reset-repairs-evidence/report-roles.png).
The fixture is synthetic SOFTWARE QA; no tool/material suitability or physical result
is implied. The temporary review page was removed from application source.

### B — Native preset import command permission

The production delta is exactly the string `fs:allow-stat` in the existing default
capability. No glob, path scope, dialog workflow or file-read code changes. The existing
2 MiB stat-before-read bound is unchanged. `scripts/check-preset-import-acl.mjs` resolves
actual dependency ACL manifests generated by `cargo check -p ivac-tauri`: the old
permission set does not authorize `fs::stat`, the corrected set does, and allow-stat
adds only that command with no configured scope. All previous path scopes compare
identically to the baseline. This does not shrink the pre-existing HOME read scope.

Five file-transfer tests cover selected bytes, cancellation, oversize-before-read,
malformed data, command/scope/read refusal, and no write on import failure. Three
preset-library tests cover collision choices and preservation. Three real native
storage tests cover atomic current-document writes, stale/conflicting writes,
malformed/oversized/busy data, traversal and symlink refusal.

**Native acceptance limitation:** command manifests/build and native storage are tested;
no installed native file chooser was exercised with this new capability. Browser UI is
not native ACL evidence. On the next approved native test build, select a disposable
valid collection, cancel once, select >2 MiB and malformed files, then exercise a duplicate
ID with cancel/new-ID/explicit replacement and verify existing collections are unchanged
on failures. Do not claim installed-platform acceptance until those results are recorded.
The prior watcher failures are neither modified nor waived by this repair.

### Qualification and handoff

Commands, final results and exact repair hashes are recorded below. Software qualification
does not close human acceptance `ivac-y25vq` or Sol's UX gate. Native selected-dialog
acceptance and physical/controller gates remain separate. **HARD STOP; no legacy removal,
V1 change, release, publication, installer distribution or hardware operation.**


| Final check | Result |
|---|---|
| `npm test` (Node 24) | 1589 PASS / 1 existing locale skip, 187 files; 10 Node WASM guard tests PASS |
| `npm run check` | Svelte/TypeScript 0 errors / 0 warnings |
| `npm run lint` | ESLint/Prettier PASS |
| `npm run build` (offline cached Rust/WASM tools) | Fresh isolated-worktree WASM build and production build PASS; existing chunk-size advisories |
| `cargo check --offline -p ivac-tauri` | PASS; dependency ACL manifests generated |
| `node scripts/check-preset-import-acl.mjs` | PASS, old ACL counterexample and new command authorization |
| `cargo test --offline -p ivac-tauri --lib preset_collections` | 3 PASS |
| `cargo test --offline -p ivac-core --features cps --lib cutting_v9_tests` | 12 PASS, including standard/specialized/two-sided/ShopBot output equivalence |
| Actual report UI → Save Markdown | PASS, downloaded bytes contain all observed distinct role values |
| `git diff --check` | PASS |

Exact implementation commits:
- A: `81382100b56b1540833532bd451546bd3c493288`.
- B / final production source: `496c46f0e60472b36c529768d0d3974987615fd0`.

Checks ran on these same source bytes before committing; only this appended handoff and
evidence follow in a documentation commit. No complete workspace/native watcher rerun
is claimed. Logs and SHA-256 inventory: [evidence](fs-reset-repairs-evidence/SHA256SUMS.json).
The report's screenshot fixture explicitly has no generated program and synthetic tools;
its purpose is truthful saved role attribution, not tool/process eligibility qualification.
At the narrow initial browser viewport the existing wide report table wraps heavily;
the desktop view was also inspected. No report-layout redesign was included.
