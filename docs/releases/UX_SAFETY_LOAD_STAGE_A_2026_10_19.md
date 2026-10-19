# UX consolidation follow-up — Stage A, 2026-10-19

> Follow-up: the user approved the background verifier; [B–D results](UX_SAFETY_LOAD_RESULTS_2026_10_19.md) supersede the pending-implementation status below. Stage A red evidence is retained unchanged.

**Root causes reproduced; implementation paused for the architectural decision
in [proposed ADR-0024](../shopbot-modernization/adr/ADR-0024-full-program-simulation-qualification.md).**
P0 ivac-3xqtb and P1 ivac-52tpd remain unresolved. Stages B–D are not implemented.
No production source changes in this follow-up. Existing dirty A–D/CAM/MCP work
preserved. No commit, push, installer, publication or physical test.

## Source and evidence

HEAD `66f229ea21cccf73a76fd45d0e9f9fe3721a41a1`, dirty beta.3/pipeline62 tree.
The existing [consolidation](UX_V1_CONSOLIDATION_2026_10_19.md) contains the actual
rendered Home/End reproduction. This stage adds direct production-function tests,
not a new browser reproduction. Evidence is in
[ux-safety-load-2026-10-19-evidence](ux-safety-load-2026-10-19-evidence/).

Five focused tests: **3 fail as expected, 2 controls pass**. The actual export
service calls mocked `writeTextFile` despite a critical simulation diagnostic;
Home-style prefix publication removes both the policy block and report critical
count; actual restore/snapshot inherits cpsPost absent from the supplied file.
Controls preserve explicitly authored inactive CPS data and the mandatory
pipeline fixture refusal. No program is written to disk or downloaded: desktop
I/O is mocked. The fixture is the diagnostic saved-preparation project, not a
machining-ready example. The Home test exercises the publication/consumer chain,
not a newly run WASM rewind; the existing browser proof supplies that evidence.

Reproduction source is retained as `stage-a-repro.test.ts.txt` outside the default
suite. To rerun, copy it to `frontend/src/lib/services/ux-safety-load-repro.test.ts`,
then from `frontend` run:

```sh
PATH=/opt/homebrew/opt/node@24/bin:$PATH ./node_modules/.bin/vitest run --config vitest.config.ts src/lib/services/ux-safety-load-repro.test.ts
```

Exit failure is expected until corrected. The temporary test was removed from
production test discovery after recording the red evidence; it was not converted
into a passing/skipped regression. Promote/adapt it to permanent tests during B/C.
`stage-a-reproduced.log` is the successful reproduction run. Earlier path/config
startup failures are retained separately and are not test evidence.

Existing focused baseline **47/47 PASS** (`existing-focused-tests.log`): same command
with `src/lib/state/generate-palette-policy.test.ts`,
`src/lib/services/gcode-export.test.ts`, `src/lib/services/shopbot-machine-load.test.ts`
and `src/lib/sim/driver.test.ts`. Full frontend, check/lint/build, Rust, TEST1,
SAFE-Z and fresh UI review are **not rerun at this architecture stop**; previous
consolidation results are historical and not claimed as validation of a fix.

## P0 — source chain and execution decision

`driver.ts` restores prefix diagnostics on rewind → `Scene3D.svelte` publishes
through `setSimDiagnostics` → `generated.svelte.ts` has only one diagnostics field
→ GenerateBar and ReportDialog use it. Report Markdown inherits the same report
input. Actual `file_ops.ts::exportGeneratedGcode` does not enforce simulator or
configurable work-area/critical policy. Its freshness and mandatory fixture
checks must remain and gain the shared policy at entry and after native Save.

No authoritative completion token exists. Merely retaining the latest nonempty
warning list would neither prove a clean whole program nor handle fresh contexts.
`wasm-worker-protocol.ts` and `wasm.worker.ts` explicitly keep Simulator on the UI
thread; the renderer needs its heightfield. An independent headless background
verifier therefore changes an established execution boundary. ADR-0024 presents
that concrete choice versus a smaller manual complete-3D verification workflow.
Recommend background verification using the existing engine, with request-owned
cancellation and one shared qualification/export policy. Do not move the renderer
or extend post capabilities.

## P1 — persistence contract and bounded correction proposed

`clearProject` retains shop machine/tools for new drawing workflows intentionally.
`restoreProject` currently overlays saved machine settings on the prior document's
machine. Thus absent optional keys survive. `migrateMachineSettings` only converts
legacy supportsToolchange to toolchangeStrategy; it does not supply canonical
defaults or clear prior keys. The loaded embedded snapshot is authoritative;
workspace profile IDs are restored but profiles deliberately not auto-applied.

For full project restore, merge the migrated loaded machine onto a **fresh canonical
machine default**, never the previous document's machine. Preserve all explicitly
stored inactive and unknown machine keys; don't remove cpsPost merely because the
active dialect is LinuxCNC. Missing machine data uses fresh defaults. Extract the
existing ProjectData defaults to a factory so initialization and restore agree.
No new schema/version appears necessary. Loading establishes a clean history
baseline; later machine edits remain normally undoable. Rollback on failed loads
must restore the prior document, including its transient generation state.

Also inspect empty tool arrays and stock overlay during C: empty tools currently
fall through and stock can inherit absent fields. Do not silently broaden the fix
or change new-drawing defaults; test full-project replacement semantics explicitly.
Existing canonical imports compatibility rules stay unchanged. Generation freshness
must derive from loaded embedded inputs; assert reset/context, not just JSON.
No evidence here establishes incorrect emitted LinuxCNC bytes.

## Review and release disposition

P0 blocks any new candidate. P1 remains open. The UX direction A–D is accepted by
the user; the distinct UX D (`ivac-he4lt`) and consolidation (`ivac-amibd`) review
records are preserved, not merged/auto-closed. Existing pipeline61 candidates are
unchanged and do not qualify pipeline62 or this future fix.

Await approval of ADR-0024's execution choice, then perform B (P0), C (P1), D
(integrated tests/UI/docs) in order, ending at the requested human review. The stop
comes from the user's explicit Stage A architecture gate. No further release
permission is requested or implied.
