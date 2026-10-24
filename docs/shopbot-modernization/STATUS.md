# FabCAM — current status

Current focus: **FabCAM V1.1 — macOS Personal Alpha**. SEC-1 software is
qualified at `135095392c2a5711acc79f5286aae7599214585a`: targeted Tokio correction,
reviewed consolidation, 20/20 canonical gates and rebuilt local arm64 app.
[SEC-1 report and publication receipt](../releases/SEC_1_REPOSITORY_CONSOLIDATION.md)
record the normal `FabCAM-v1.1` push separately from the tested build source. Public release, main merge,
version promotion, FabSpeed integration and hardware operation remain excluded.
[Release plan](../releases/RELEASE_ROADMAP.md) · [FabApps vision](../FABAPPS_VISION.md).

| Work | Current disposition |
|---|---|
| FS-RESET-3 | COMPLETED: explicit v9 actual-role parameters, neutral CAM safety and tested collections; legacy recommendation engine removed |
| QUAL-1A.4 | COMPLETED software integration: `8e2c9ce`, report `f2b0427`; [numerical/motion limits](../releases/QUAL_1A_4_PRODUCTION_INTEGRATION.md) retained |
| QUAL-1B | COMPLETED software cancellation: `fd8fed6`, report `aeae820`; [fixed campaigns](../releases/QUAL_1B_COOPERATIVE_CANCELLATION.md) meet 200 ms p95, without a universal GUI latency claim |
| QUAL-1C | BUILT / SOFTWARE TESTED: `700de73`, report `9520d98`, [19/20 gate](../releases/QUAL_1C_MACOS_PERSONAL_ALPHA.md); app not launched, native human acceptance pending |
| SEC-1 | QUALIFIED: `1350953`, 20/20 canonical gates, current audit PASS, 77 native/WASM cases exact, TEST1 256 PASS; rebuilt local arm64 app. Publication outcome/exact remote SHA in the report's post-push receipt |
| Primary local edits | PRESERVED / PENDING: pocket diagnostics, fixed-stock/drawing placement and header UI; separate uncommitted source and reports, not qualified by SEC-1 |
| Optional V1.2 | PLANNED one-way FabSpeed → FabCAM; [ADR-0031](adr/ADR-0031-optional-one-way-fabspeed-catalogue.md) Proposed, no implementation |

Tokio 1.52.2's RUSTSEC-2026-0386 is addressed by the minimal lockfile update;
1.54.0 is excluded because of the separate Interval race. The security commit
preserves every other resolved package. Focused tests: native Tauri 15, HTTP 21,
file export 11, CPS 55, 23 unchanged output references; full audit and strict
affected pedantic Clippy pass. Final full qualification: Rust 1,592 PASS / 0 FAIL /
1 intentional ignored worker; frontend 1,498 PASS / 1 optional locale skip.
Native/WASM replay and the 23 fixed CLI references are exact.

The earlier watcher failures remain recorded as sandbox failures; the unchanged
five watcher tests passed on the Mac in QUAL-1C and SEC-1 focused qualification.
Actual native dialogs, file round-trips and cancellation recovery remain untested
in the installed app. Static ACL checks or mocks cannot close them. Two-sided
publication is not pair-atomic; CLI streaming does not acquire full-program
simulation qualification. Automatic dowel injection remains excluded.

MAT-1, tested collections and generic machine/tool libraries remain retained.
Primary local drawing placement has bounded tests and pending human UX review;
its reported header translation failure and combined-source qualification remain
open. Initial dirty source/evidence and original STATUS/roadmap copies are
fingerprinted and preserved in the local SEC-1 workspace receipt.

Frozen `main`/V1 remain `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca` with their
original acceptance limits. Dated reports are unchanged; the earlier
[roadmap consolidation](../releases/ROADMAP_CONSOLIDATION.md) records planning
history rather than the current A4/B/C status.

Next product milestone: **Personal Alpha — Actual macOS Launch & Native Workflow
Acceptance**, using the separate macOS test account and the newly rebuilt
[SEC-1 bundle/procedure](../releases/SEC_1_REPOSITORY_CONSOLIDATION.md#remaining-acceptance-and-hard-stop).
The historical QUAL-1C bundle remains evidence, not the corrected app. No launch in the usual
application-data profile. Supervised QUAL-2 follows only by separate authorization.
**HARD STOP after the qualified SEC-1 development push and report.**
