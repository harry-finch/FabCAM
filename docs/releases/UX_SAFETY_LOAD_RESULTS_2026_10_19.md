# UX consolidation — export safety and project replacement

2026-10-19 · beta.3 / pipeline62 · local uncommitted checkpoint

**P0 ivac-3xqtb and P1 ivac-52tpd corrected and software-tested. HARD STOP for
human review ivac-3i00w before candidate preparation.** The user approved
ADR-0024's background verifier after [Stage A](UX_SAFETY_LOAD_STAGE_A_2026_10_19.md).
UX D (ivac-he4lt) and consolidation (ivac-amibd) remain separate gates.

## Root causes confirmed / before evidence

Stage A's retained executable reproduction produced three expected failures:
actual export called the mocked writer despite a critical simulator warning;
Home-style prefix replacement removed the export block and report critical count;
restore/snapshot inherited cpsPost from the previous document. Two controls passed.
The actual Home/End geometry reproduction is preserved in the earlier consolidation.
No unsafe program was exported in that reproduction. No incorrect LinuxCNC bytes
were established from the metadata inheritance finding.

## P0 correction and invariant

`ProgramQualification.svelte` runs independently of Scene3D, using a dedicated
request-owned worker. The same Rust/WASM Simulator executes the entire generated
preview path, front then back where applicable; it returns diagnostics, elapsed
time and simulator version. Each grid is freed in finally, and its worker is
terminated at completion, replacement, failure or cancellation. No rendering mesh
or playback checkpoints are allocated in the verifier. Numerical grid and tool
serialization helpers are shared with playback; the first enabled operation now
selects the automatic cell resolution in the scene too.

A transient GeneratedState qualification record is keyed by document epoch,
generation version, actual generation input context, qualification revision,
resolution settings, cell budget and transport cap. The loaded simulator version
is recorded with the result. No qualification is persisted or reused across
sessions/builds. The effective configuration is deterministically derived from
that identity; neither playhead nor cosmetic display settings belongs to it.

Pending/current/stale/failed/unavailable are explicit. Current means the entire
supported program was simulated and can include critical warnings; it does not
mean safe for a machine. Unknown tools, incomplete simulator ingestion, worker
failure and unsupported configurations cannot yield a clean qualification.
Late replies are discarded. Reset/new generation clears qualification; document
rollback restores the old state but its changed epoch requires fresh verification.

`simDiagnostics` remains the playback-prefix channel for scene markers and stock
rendering. Scrubbing changes that channel only. Full-program warnings feed the
warning summaries, Project Report and Markdown. Front/back diagnostics are both
included; back simulation warnings are labelled and do not offer an incorrect
jump into the front playback. An incomplete verification is not labelled clean.

## Actual export enforcement / policy

`programExportStatus` and `exportSafetyProblem` are the shared decision used by
the toolbar/palette and **file_ops.exportGeneratedGcode**, including the call
immediately before native write after the asynchronous Save dialog. Existing
context/result identity, empty-output, geometry/pipeline/capability/post safeguards
remain. All current program-download callers route through this service:
GenerateBar, command palette, App export actions and GcodePanel save callback.
STL and project/report saves are distinct artifacts, not executable-program exports.

| Policy | Complete critical simulation | Pending/stale/failed/unavailable |
|---|---|---|
| blockOnCriticalSimWarnings enabled | Refused | Refused |
| Explicitly disabled | This preference does not refuse | Not represented as verified; this preference does not refuse |

Mandatory pipeline fixture refusal remains unconditional. The separate work-area
preference is also enforced at the actual service, independently of sim policy.
No preference was changed silently. No ShopBot strategy/tool capability was widened.
No backend/CLI publication contract is claimed replaced by this frontend verifier.

## P1 persistence contract and correction

Full project restore now starts with fresh canonical machine/stock/tool defaults,
then restores the loaded document. Missing optional machine keys cannot inherit
from the previous project. Explicit inactive cpsPost, unknown compatible machine
keys, authored tools and operations survive. Legacy supportsToolchange still
migrates to toolchangeStrategy. An absent legacy dialect resolves to LinuxCNC
rather than an unrelated last-used workspace post; explicitly authored dialects
win. No schema or project version change.

The same merge audit exposed stock optional-key inheritance and empty tool arrays
falling through to the preceding job. Full restore now respects an explicit empty
tool list and uses fresh stock defaults. Missing tool data uses the existing
canonical starter tool. New drawing/reset workflows still retain shop defaults
by their original contract. Embedded project snapshots remain authoritative;
workspace profiles are not auto-applied. Load resets history; subsequent machine
edits remain undoable. Existing source bindings, F&S and authored extensions remain.

The real browser CPS → LinuxCNC → save comparison has no inherited cpsPost; tools
and operations are equal. The only saved difference is the existing omission of
an all-zero G54 workOffset; equality holds after that documented normalization.

## Regression results

All final evidence is under [the evidence directory](ux-safety-load-2026-10-19-evidence/final/).
HEAD: `66f229ea21cccf73a76fd45d0e9f9fe3721a41a1`, dirty tree including preserved UX A–D.
Node 24.21.0; Vitest 2.1.9; local macOS arm64. Build metadata and source hashes
are recorded in the evidence manifest. Prior failing intermediate logs are retained;
only the named final logs below establish the completed checkpoint.

| Check | Final result / evidence |
|---|---|
| Frontend | **158 files, 1429 PASS, 1 existing SKIP** + **10 Node script tests PASS**; `tests-accepted.log` |
| Type check | **0 errors / 0 warnings**; `check-accepted.log` |
| Lint + formatting | PASS; `lint-accepted.log`; no size-budget relaxation |
| Frontend/WASM freshness + build | PASS; `build-accepted.log`; existing bundle-size/static-dynamic import notices remain |
| SAFE-Z extended | **15 PASS** |
| ShopBot M2 post | **27 PASS** |
| V-Carve qualification | **10 PASS** |
| CPS posts | **5 reported PASS**; optional external FANUC oracle absent, so that case is a documented no-op rather than executed oracle evidence |
| TEST1 direct | **4 oracle tests PASS**, worker test ignored in direct harness |
| TEST1 bounded workers | **256/256 PASS**, 25.98 s, no failures/timeouts; `test1.log` and `test1/summary.json` |
| Diff whitespace | PASS |

Rust targeted command, from repository root (log `rust-regressions.log`):

```sh
RUSTC_WRAPPER='' cargo test --offline --release -p ivac-core --features cps,profiling --test safe_z_extended --test test1_properties --test shopbot_m2_post --test cps_posts --test vcarve_qualification
PATH=/opt/homebrew/opt/node@24/bin:$PATH python3 scripts/test1/run.py --binary target/release/deps/test1_properties-a28bea547eac3aa1 --output docs/releases/ux-safety-load-2026-10-19-evidence/final/test1 --frontend
```

Frontend commands from `frontend`, with Node24 on PATH: `npm run check`,
`npm run lint`, `npm test`, and
`RUSTC_WRAPPER='' CARGO_NET_OFFLINE=true npm run build` (Cargo/Homebrew also on PATH).
The last build passed without rebuilding distribution packages. Rust core-wide,
Clippy, native Tauri installation, Windows/Ubuntu runtime and physical machining
were **not rerun**; targeted Rust regressions cover the relevant unchanged boundaries.

Permanent tests: worker cancellation/replacement/errors/incomplete replies;
complete chunked traversal, both sides, diagnostics retained across chunks and
finally/free; real WASM clean/colliding paths; direct native and browser export,
Save-dialog race, explicit preferences, mandatory fixtures, work area, missing
back qualification; actual context edits (stock/tool/machine/fixtures/origin),
settings versus playback/cosmetics; restore/save/reopen, legacy/empty data,
unknown/inactive metadata, profile snapshot and post-load Undo/Redo. Existing
GenerationRuns tests cover cancelled/obsolete generation responses. Stage A
reproduction source remains archived; permanent regressions use the new authority.

## Browser / visual evidence

Actual built application at `127.0.0.1:18803`, HTTP backend at `127.0.0.1:18804`.
Local browser review, not desktop-installation qualification. Both report beta.3.

- Diagnostic 431-move fixture: pending verification disables Download; completion
  reports one critical rapid-through-material plus one work-area warning.
- Home, End, ArrowLeft and repeated Home preserve the complete-program critical
  warning and disabled export. Final-build proof: `04-final-home-blocked.png/.txt`.
- Report at Home retains **1 critical / 1 non-critical**, operation error status
  and the rapid warning. Actual saved Markdown: `report-at-home.md`.
- CPS project → LinuxCNC project → save: `after-cps-roundtrip.ivac-project.json`
  and `roundtrip-result.json` prove no cpsPost inheritance.
- A derived **software-only** fixture with stock top/origin/work area consistent
  with its program completes verification without warnings; switching to 2D does
  not invalidate it; actual local download succeeds (`software-clean.ngc`). This
  is test output, not a job approved for machining.
- No worker exception observed. One earlier report-layout ResizeObserver notice
  labelled benign by the application appeared; no failed action followed it.

The real WASM regression logs linear-memory usage (about 1.5 MiB initial, 1.6 MiB
after these tiny fixtures) and elapsed time in `tests-accepted.log`. This is
Node/WASM committed linear memory, **not whole-browser RSS or a production bound**.
Browser controls remained responsive during the small fixture workflow and the
pending state was visible. Large-job memory/latency and packaged-runtime reserves
are not closed by this evidence. Only one verification worker is active; playback
may still allocate its own independent grid/checkpoints.

## Files / compatibility / limits

Production changes: new `sim/config`, `qualification`, `qualification-run`,
`qualification.worker`, `verify-program`; new `state/program-qualification` and
`project-defaults`; new ProgramQualification component mounted in App. Extended
generated lifecycle/rollback, shared export policy and actual file service,
GenerateBar/PhoneWarnings/ReportDialog, Scene3D first-enabled-tool choice and
shared driver configuration. English/German status strings updated. Focused tests
are separate for export, project replacement, qualification lifecycle, real WASM,
context and warning status. Source manifest lists exact paths and hashes.

Qualification is explicitly unavailable for non-milling processes, missing/invalid
stock/tools, enabled operations declaring different rough/finish or spot tools
within one op, and two-sided programs with fixtures. The current segment contract
cannot disambiguate those tool changes or qualify back fixture placement. These
are visible, fail-closed coverage limits under the active policy, tracked in
**ivac-b0js3**, not guessed successes or instructions to disable safety. They can
restrict workflows that previously exported without complete verification; review
this compatibility impact at the human gate.

No geometry engine, generated program bytes, post capability, project format,
application identity, source binding, F&S provenance or physical machine setting
was intentionally changed. Old pipeline61 desktop candidates remain unchanged
and are not qualified by this browser/source checkpoint.

## Release disposition / human gate

The two reproduced defects are resolved within the stated software scope.
**ivac-3i00w OPEN** is the review gate for this checkpoint and its coverage limits;
UX D and consolidation gates remain distinct. No commit, push, merge, tag,
publication, installer rebuild or physical machining occurred. No candidate
preparation starts automatically. After human acceptance, reconcile remaining
release gates and request separate authorization for source publication/candidates.
