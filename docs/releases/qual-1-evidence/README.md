# QUAL-1 evidence index

2026-10-23. Qualified software source:
`558efc4f7e7cfb5d008c88cf7469a958cc8dc053`, branch `FabCAM-v1.1`.
See the [qualification report](../QUAL_1_SOFTWARE_QUALIFICATION.md) for conclusions.
**17/20 mandatory gates; NO-GO for unconditional final qualification/release.**

## Committed receipts

| Evidence | Purpose |
|---|---|
| `environment.json`, `SOURCE_FINGERPRINTS.json` | Toolchain, lockfile/script/request/binary/module identity; baseline and final source binding |
| `initial-matrix.json`, `final-matrix.json`, `final-gates.json` | Initial gaps, final dispositions and all 20 mandatory gates |
| `final-pre-release.log`, `final-pre-release-exit.json` | Unmodified canonical run; all criteria retained |
| `preliminary-pre-release.log` and exit receipt | Earlier run, including missing frontendDist prerequisite; not the final result |
| `final-workspace.log`, `workspace-summary.json` | All workspace targets and doctests, including failures; TEST1 worker separately executed |
| `additional-exits.json`, `test1-summary.json`, `test1-frontend.json`, `final-test1.log` | Final TEST1/C2 watchdog and complete subprocess results |
| `final-real-wasm.log`, `cli-corpus-summary.json`, `FINAL_WASM_COMPARISON.json`, `final-strict-parity.log` | Real WASM contracts, unchanged fixed outputs and explicit retained parity failure |
| `MATH_SUMMARY.json`, `STAGE_SUMMARY.json`, `GEOMETRY_SUMMARY.json` | First arithmetic divergence, stage localization and sampled geometry diagnostics |
| `diagnostic-math-check.txt` | Six analytical checks of the point/segment distance helper; no continuous swept-volume claim |
| `cancellation-before.json`, `cancellation-after.json`, `cancellation-observations.json` | Both fixed campaigns and all 140 raw observations; original 200 ms target remains failed |
| `cancellation-stage-observations.json` | Diagnostic callback timing, excluded from performance qualification |
| `cancel-regression-before.log`, `cancel-regression-after.log`, `cps-fix-tests.log`, `fix-clippy.log`, `FIX_OUTPUT_COMPARISON.json` | Focused correction: failing reproduction, positive/negative regressions and output equality |
| `pnpm-clean-*.log`, `cargo-deny-writable.log`, `deny-writable.toml`, `loopback-permission.log` | Environment investigation, cached install and unresolved online/listener constraints |
| `UI_REVIEW.md`, `ui/` | Disposable-origin visual walkthrough and original screenshots |
| `MANUAL_ACCEPTANCE.md`, `host-gates.sh` | Exact pending host and actual file-transfer procedures; NOT TESTED here |
| `pre-report-preservation.json`, `PRESERVATION.json` | Original local edits preserved; only the new QUAL-1 status prefixes staged |
| `RAW_SHA256SUMS.json` | SHA-256 and size of each retained raw local artifact, including large traces/binaries and TEST1 input/output pairs |

Raw evidence remains at `dist-test/qual1/` in the original checkout. The inventory
also fingerprints referenced pre-existing baseline binaries and fixture manifests.
Do not delete that directory or mistake compact committed summaries for all raw data.
The raw inventory is a point-in-time snapshot; later repository status/commit receipts
are kept outside that raw directory to avoid self-referential hashes.
Captured logs and unified patches retain their original whitespace, including blank
context lines; the authored-document whitespace check excludes those verbatim records.

## Replay and diagnostic boundaries

Use a **clean disposable checkout** and the pinned toolchain. The user checkout has
unrelated uncommitted edits and must not be reset or cleaned. The host procedure builds
frontendDist before Tauri compilation and then runs the unchanged pre-release script.
Fresh online advisory data and an unrestricted event/listener/browser environment are
still prerequisites; cached pnpm success is not an online registry qualification.

The additional canonical commands are recorded in `additional-exits.json` and in
`final-additional.py`: full workspace/all-features/no-fail-fast, TEST1 watchdog with
`--frontend`, three real-WASM test files and the unchanged strict parity checker.
Run `scripts/test1/build.py` to obtain the actual release worker path before invoking
`scripts/test1/run.py`; never copy an old platform-specific binary name blindly.
Use a new evidence output directory on replay, preserving existing observations.

The captured `final-additional.py`, `final-cli-compare.py`, `qualify-fix.py`,
`instrument.py`, `compare_trace.py`, `compare_geometry.py`, `trace-wasm.mjs`,
`math-probe.mjs` and `math_probe.rs` are one-off reproducibility records with recorded
absolute paths. Review/adapt their **paths only** to a fresh clone/evidence directory
before use. `diagnostic.patch`, `qual1_trace.rs` and `cancel-diagnostic*.patch` describe
temporary instrumentation: apply only in that disposable clone, never the user's
working checkout. Verify instrumented output equality and remove instrumentation
before production performance or final gate measurements. Diagnostic build/setup
attempts and their logs are retained, including failed setup attempts; only the
identified successful instrumented runs feed the reported trace measurements.

`compare_geometry.py` aligns ordered vertices and samples local polyline distances;
it does not certify continuous Hausdorff distance, swept volume or physical safety.
The standard-library math probe isolates native/WASM arithmetic, not a proposed
production implementation. No tolerance or golden reference was changed.

`final-additional.py` collects results after failed commands; its exit code cannot
replace the individual exit receipts. Strict parity intentionally remains failed.
The before/after cancellation campaigns use fixed sample counts and the existing
nearest-rank p95; do not retry until green or raise the threshold.

Installed native/macOS/Windows/Linux file workflows and physical ShopBot acceptance
remain separate. Static ACL, mocked dialogs and browser screenshots do not close them.
No installer, publication, hardware action or next milestone is authorized here.
