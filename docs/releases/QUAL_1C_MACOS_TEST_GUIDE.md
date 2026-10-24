# QUAL-1C — Personal Alpha test guide

This is a local, non-machining evaluation. Do not send any program to a machine.
See [the qualification report](QUAL_1C_MACOS_PERSONAL_ALPHA.md) for the current
recommendation, artifact identity and outstanding software gates.

## Separate macOS account

The maintainer chose a separate macOS test account. Do not launch this candidate
in the usual account: the unchanged identifier `org.opcenter.ivacam` would reuse
its workspace, recent files, collections, window state and WebKit storage.
Use a standard test account created through macOS System Settings. Do not change
the bundle identifier, reset existing application data or change filesystem ACLs.
Copy only the verified `.app` and synthetic QA files into that account through an
explicitly chosen transfer location; no private presets or production projects.
Do not remove quarantine attributes or disable Gatekeeper.

## Build and identify

From the isolated `FabCAM-qual-1c` checkout, use the pinned Rust 1.88 toolchain,
Node 24, pnpm 10, wasm-pack and matching wasm-bindgen, wasm-opt, Tauri CLI 2.

```sh
git rev-parse HEAD
git status --short
pnpm --dir frontend install --frozen-lockfile
cd crates/ivac-tauri
# Use cargo tauri, or the installed Tauri CLI executable:
cargo tauri build --config tauri.dev-signing.conf.json --bundles app -- --locked
```

The build hook regenerates/freshness-checks WASM and the production frontend.
The output is `target/release/bundle/macos/FabCAM.app`. `RUSTC_WRAPPER=''` may
be used where sccache is unavailable. No public installer is needed.
Verify `codesign --verify --deep --strict --verbose=2 /path/to/FabCAM.app` and
compare the artifact manifest in the report. An ad-hoc signature is not Apple
notarization. After the report's outstanding security decision, launch by opening
this exact app in the test account. In About, record the build revision and version.

## Short first-use protocol

Synthetic inputs are prepared locally in `dist-test/qual1c/qa-inputs/`: a 40 × 30 mm
SVG rectangle, malformed JSON and a file exactly 2 MiB + 1 byte. They are not
production presets or machine programs.

Create a `Documents/FabCAM-QUAL1C-QA` folder in the test account. Use only synthetic
geometry and labels such as **SOFTWARE QA ONLY — never machined**.

1. Launch, switch Project/Machine/Tools/Presets, resize, open/close dialogs and
   restart. Record startup errors and whether the test workspace persists.
2. Import a simple SVG rectangle with explicit mm dimensions. Inspect dimensions,
   set stock/origin, machine, tool and material. Add Profile. Enter both main and
   finishing rates explicitly (Profile contour uses Finish). Save a v9 project,
   close/reopen it, compare geometry, all identities, stock/origin and cutting values.
   A v1–8 file must be refused without modifying it.
3. Clear one rate: editing/saving remains possible but generation must refuse.
   Try a known limit violation and then restore valid synthetic values. Check
   independent Pocket rough/finish and Drill/Spot/Chamfer roles on disposable copies.
4. Create a clearly named QA collection and preset. Any attestation exercised here
   is a synthetic UI test, **not physical evidence**. Review/load, edit values and
   check Loaded/Modified status and Undo/Redo. Delete the QA source preset: copied
   operation values must survive. Context mismatch must refuse automatic loading.
5. Export the QA collection through the native dialog; import that actual JSON.
   Check cancel, ID collision/replacement, malformed JSON and >2 MiB refusal.
   Check originals remain intact. Record any `stat`/scope error verbatim; do not
   broaden permissions. Test a deliberately unwritable disposable destination.
6. Generate Profile, inspect toolpath, complete full-program simulation and inspect
   report tool/rates. Export to a fresh QA destination. Change a rate: old output
   must become stale and cannot be exported as qualified. Recompute, requalify,
   export again; compare saved text with the qualified program.
7. Cancel a sufficiently long generation. Cancelled output must never become current
   or exportable; a following ordinary generation must succeed. No GUI latency
   target is inferred from the earlier native benchmark.
8. Modify an imported **QA** source file externally; check the source-change warning.
   Restart and reopen saved project/collection. Do not erase the test data until its
   evidence is reviewed.

Native dialogs, native engine startup and save/reopen need actual observations;
unit tests or a browser screenshot cannot complete this checklist.

## Errors and evidence

Stop the affected workflow on an unexplained error, wrong geometry/rates or stale
export acceptance. Keep the original file; save the exact error, steps, screenshot,
About revision, macOS version and a minimal non-private reproduction. Native logs
are under the test account's `~/Library/Logs/org.opcenter.ivacam/`; if absent,
launch its `Contents/MacOS/ivac-desktop` from Terminal and capture stdout/stderr
in the QA folder. Do not run that command in the usual account.

Record each step as PASS / FAIL / NOT RUN with filename and observation. Return
that record for human review. No public distribution or QUAL-2 follows automatically.
