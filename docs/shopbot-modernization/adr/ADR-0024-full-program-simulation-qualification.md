# ADR-0024 — Full-program simulation qualification independent of playback

Status: **APPROVED architecture by the user — 2026-10-19; implemented locally, checkpoint review pending**
Date: 2026-10-19
Scope: UX consolidation follow-up, ivac-3xqtb. No project schema migration.

## Context

The current simulation belongs to `HeightfieldDriver` / `Scene3D`. Its diagnostics
are a playback-prefix view and are restored/reset during rewind. GenerateBar and
ReportDialog consume that prefix as if it qualified the whole program. The actual
`exportGeneratedGcode` service checks freshness, nonempty output and mandatory
pipeline fixture collisions, but not the simulation warning policy. A button-only
fix cannot secure exports. Stage A tests reproduce both paths.

The existing CAM worker protocol explicitly excludes Simulator. The simulator
stays on the main thread to provide zero-copy heightfields for rendering. Running
an additional complete simulation there could block the UI. Moving rendering to
a worker would be a much larger unrelated change.

## Approved decision

**KEEP** the Rust simulator and existing playback/renderer. **EXTEND** the runtime
with a headless complete-program verification worker and one transient qualification
record, independent of Scene3D mounting, playhead and playback diagnostics. Do not
create a second collision algorithm or change generic geometry/postprocessors.

Use a dedicated request-owned worker, separate from generation cancellation and
rendering. Reuse Simulator, footprint/cell-size/stock/tool/fixture preparation via
small shared pure helpers. Return diagnostics and completion metadata only, not
heightfields. Run front and back sequentially where present, releasing each grid;
never infer back qualification from front success. Scope unsupported processes or
configurations explicitly as unavailable, never as zero warnings.

Start verification after a successful current generation and re-evaluate on
relevant simulation settings changes. Qualification identity includes document
epoch, generation version/result identity, input context, side, simulator version
and effective numerical configuration (including cell-size/cap). Capture immutable
inputs. Reject results from replaced/cancelled requests and check identity again
at actual write time, including after an asynchronous native Save dialog.

States: unavailable, pending, current, stale, failed. A current result means the
entire program was checked under the recorded configuration; it may contain
critical diagnostics. It is bounded software verification, never a machine-safety
certificate. Merely allocating a grid or checking a prefix cannot yield current.

One shared export decision consumes this record, pipeline warnings, current
context and existing user preferences. The actual export service enforces it;
buttons, palette, report and Markdown use the same record. Preserve unconditional
fixture/pipeline refusal and existing capability/output guards. With critical-sim
blocking enabled, missing/incomplete/stale/failed verification blocks export. With
that preference explicitly disabled, do not silently make it mandatory; report
unverified status honestly and retain all unconditional and work-area checks.
Playback markers may continue showing prefix diagnostics.

## Alternatives considered

1. Retain a completed playback diagnostic snapshot. Smallest code delta and no
   extra simulator memory. However verification remains dependent on opening 3D
   and reaching End; missing completion must block export under the active policy.
   This is a viable explicitly manual verification workflow, but adds a user step
   and does not provide automatic verification from 2D. Not recommended.
2. Run a throwaway full HeightfieldDriver on the UI thread. Reuses code but adds
   unnecessary meshes and potentially blocking synchronous work. Not recommended.
3. Add native/server verification endpoints and a WASM counterpart. Could serve
   non-UI consumers later, but expands transport/API scope for this bounded fix.
   Defer; existing native post/publication guards remain intact.

## Consequences and recorded approval

The new worker changes the existing simulation execution boundary and adds WASM
loading, toolpath transfer and an extra simulation grid while playback is active.
Measure memory/responsiveness; bound concurrent verification to one request and
release it on completion, cancellation or document replacement. Do not promise
specific latency or memory limits without evidence. Worker initialization errors,
OOM or unavailable WASM must fail closed under the relevant preference.

The user explicitly approved the dedicated background verifier on 2026-10-19 and
authorized P0, P1 and integrated qualification without commit/publication.
[Implementation and evidence](../../releases/UX_SAFETY_LOAD_RESULTS_2026_10_19.md)
record the results and explicit unsupported configurations. Human review of the
implemented checkpoint remains separate from this architecture approval.

No persisted qualification, format/version changes, machine-profile ownership
changes, physical process qualification or installer changes. New-generation
and project-load invalidation remain transient. Generic runtime code contains no
ShopBot exceptions. Non-UI publication contracts are not claimed extended by this
frontend service correction.

## Validation before acceptance

Implement failing reproductions as permanent regressions. Verify real WASM full
simulation, not only worker mocks. Cover Home/End/arbitrary/repeated scrub, valid
program, pending/failed/unavailable/stale, stock/tool/machine/fixture/resolution
changes, cancellation, obsolete results, two-sided programs, native-save-dialog
race and direct browser/native export service calls. Verify report and Markdown
from the same authority. Run focused + full frontend/check/lint/build and relevant
Rust SAFE-Z/TEST1/post suites; real rendered UI review and responsiveness evidence.
No acceptance, issue closure or candidate preparation from Stage A alone.
