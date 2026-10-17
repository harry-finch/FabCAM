# UX7 — request-scoped generation status and elapsed time

2026-10-16. Local implementation after explicit REL1-A acceptance. Human UX7 review pending;
UX6-C has not started. No commit, push, release, credentials or notarization.

## Behavior and timing contract

The existing native, WASM-worker and HTTP execution adapters remain in place. No CAM kernel,
postprocessor, persisted project format or machine-output capability changed.

A runtime request ID plus document epoch owns every callback and final cleanup. Concurrent
starts are refused. Before accepting output, the captured generation context must still match;
cancelled or obsolete responses cannot overwrite the current result. Document changes and
component teardown invalidate ownership and abort supported requests. Starting a request also
invalidates the previous export identity, including when the new request fails.

Elapsed time uses monotonic performance.now(), from valid request acceptance before request
preparation through accepted result transformation/storage, or owned failure/cancellation.
The display refreshes every 250 ms, rounded to 0.1 s; its refresh cadence is not measurement
precision or an ETA. Terminal duration freezes until another valid run or document change.
No completion delay is inserted. Edits mark the completed result obsolete.

Preview simulation remains asynchronous and separately guarded by the existing simulation
pending/diagnostic export gates. Generation time is neither preview completion nor estimated
machine cutting time. Critical pipeline diagnostics receive a blocked completion label.

Progress is indeterminate: actual operation names may appear, but operation count is not
converted into a runtime percentage. op_completed no longer increments an already completed
index. Thus one operation cannot be presented as 200% or 2 of 1. No invented kernel stages.
Cancellation stays pending until acknowledgement on streaming paths. Buffered, two-sided and
GEOM1 edited-drawing paths disclose unavailable cancellation rather than claiming interruption.

## Automated evidence

- Full frontend suite: **146 files, 1324 tests PASS, 1 skipped** (ux7-tests.log), plus
  **10 Node WASM packaging checks PASS** run by npm test.
- Svelte/TypeScript: **0 errors, 0 warnings** (ux7-check.log).
- Targeted ESLint: PASS on the six changed/new lifecycle and component files.
- Nine injected-clock/ownership cases: advancing and frozen success; failed, cancelled,
  obsolete and blocked terminal states; repeated/concurrent requests; late callbacks;
  input changes; document invalidation/teardown; fast completion without artificial delay.
- Two generated-state regressions: one-operation completion bounds and old export-identity
  invalidation. This is a logic-only rune-storage harness; not a rendered Svelte assertion.
- Existing lifecycle/context/export/frontend safety suites remain in the full suite.

## Native macOS UX review

The app was launched directly from target/release/bundle/macos/FabCAM.app, without
replacing /Applications/ivaCAM.app or /Applications/FabCAM.app. The saved native-text
fixture was reopened. This review does not qualify installation on a clean host.

- In-progress accessibility observation: actual operation name, Cancel, and
  “Generating toolpaths… 0.3 s”; no visual percentage or X-of-Y operation estimate.
  [Final running capture](ux7-final-running.png): long operation names retain their beginning,
  use ellipsis and expose the full name in the tooltip.
- Successful V-Carve: 18,874 moves / 18,982 output lines; terminal duration 0.9 s.
  Subsequent observations showed the same frozen duration. [Screenshot](ux7-completed.png).
- Repeat generation: fresh duration 0.7 s, same move count. [Screenshot](ux7-rerun.png).
- Editing step/pass marks status obsolete and disables Download .sbp immediately.
- A positive maximum carving depth is rejected by the existing V-Carve guard:
  “Generation failed after 0.1 s”; old Download .sbp remains disabled. No false
  completed/success state. [Screenshot](ux7-failed.png).
- Invalid tests were not saved; Undo returned the project to its saved state.
- The captured failure text initially used an undefined warning token; corrected
  to the existing --warn token after review. The final rebuild also disables
  cancellation on the non-interruptible GEOM1 path.
- Native cancellation could not be completed before this small fixture finished;
  it is **not claimed as a new native PASS**. Controller cancel/late-response cases
  and existing adapter tests pass. Full platform and long-running UX acceptance
  remain separate from these bounded observations.

## Qualification limits

Software-only. Native macOS observations are recorded below; Windows 11 and Ubuntu runtime
checks remain pending host access. Full backend cancellation/transport fault injection is not
claimed by the pure controller tests. No physical machining qualification.

Prior CI candidates (source 5ac4023576d72d450912357887273195faab0a72), installed baseline and
REL1-A DMG are retained. This UX7 build is a dirty local revision based on
4ebd43394829f1fb8372e5f9152143b9dcf3c868. Development ad-hoc signature does not qualify
Developer ID, notarization, Gatekeeper distribution or clean-host installation.

## Final local build evidence

Final macOS arm64 app build: **PASS**, app-only bundle with development ad-hoc signing
(ux7-build.log). Strict deep codesign verification: **PASS** (ux7-codesign.log).
No installer was republished; no installed baseline was overwritten. App path:
`target/release/bundle/macos/FabCAM.app`.

Final native launch/reopen/generation: **PASS**, 18,874 moves, terminal **1.0 s**;
[final completed capture](ux7-final-completed.png). The running capture above is from
the same final build and confirms the ellipsis correction. The earlier failure screenshot
precedes the warning-token color fix; its rejection behavior is unchanged.
Build warnings remain about existing bundle chunk sizing and static/dynamic Tauri imports.
Source/binary hashes are recorded in ux7-local-provenance.json. Final comment-only edits
clarify existing safety rationale and do not affect the built behavior.

## Manual review to perform

On a representative workshop project: start generation, check elapsed time while the UI
remains usable, cancel a sufficiently long streaming run, regenerate, then edit an operation
and confirm stale status/export blocking. Review toolbar wrapping at the intended desktop
window width. Keep these human UX observations distinct from Windows/Ubuntu installation
acceptance and macOS trusted distribution.

## Human gate

**HARD STOP after UX7 review. UX6-C requires the next explicit approval.**
