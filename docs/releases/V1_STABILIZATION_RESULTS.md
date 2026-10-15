# V1 bounded stabilization — software correction checkpoint

2026-10-15 · User authorization: “ok je valide ce lot de stabilisation”.
Engineering tracking: ivac-chhz0. This supersedes the implementation stop in
[V1 triage](V1_SCOPE_FREEZE_TRIAGE.md), not the independent acceptance gates.

## Changes and counterexamples

| Defect | Evidence before correction | Result |
| --- | --- | --- |
| XOR, ivac-j2po | [Red test](v1-stabilization/geometry-red.log): third equal contour incorrectly disappeared after the empty intermediate | Continue the symmetric-difference fold. 2/3/4 identical operands and A xor A xor disjoint B follow parity. Updated English/German explanation. |
| Deep Pocket Auto, ivac-dli7 | [Red emitted-path test](v1-stabilization/pocket-red.log): cutter entered retained island at (37.0894,22.0029) | Use selected-ancestor parity: even depths are pockets, immediate odd-depth descendants are islands. Preserve original analytic contours and existing offsets. |
| Operation tool identity, ivac-hs9i | Project T2 D12 opened inventory T2 D6 ([before](v1-stabilization/tool-before.png)) | Gear opens existing project-backed draft modal, explicitly labelled “Edit this project tool”; inventory remains separate. No new identity model. |
| Text-only reopen, ivac-vt6e | Text existed but was tiny at full-machine fit ([before](v1-stabilization/text-before.png)); false missing-drawing diagnostic | Fit cached text bounds, freeze the frame during origin movement, reset on document identity/newly available text/Fit. Retain raster-only bed behavior. Recognize text in the missing-drawing check; orphan-layer checks remain. |
| WASM integrity, ivac-5g51 | [Triage reproduction](v1-triage/partial-wasm-repro.json): wasm-only package accepted | Require wasm, JS, declarations and package manifest, nonempty assets, wasm header and expected manifest references. Successful locked build records SHA-256 input/asset fingerprints. Production fails closed without a valid package or build tool. |

Geometry changes are generic. No ShopBot-specific workaround, post capability
extension, new milling strategy or geometry serialization change was introduced.
Halfpipe takes its existing separate path. `spindleSpeed` changes remain explicitly
refused; the existing ShopBot guards and refusal tests are retained.

## Geometry qualification

`crates/ivac-core/tests/v1_stabilization.rs` adds an independent oracle over actual
emitted moves. Its matrix covers Cascade, Zigzag and Spiral; three/four nested
squares; full and partial explicit selections, including a missing ancestor.
Every expected even-depth region must receive cutting moves. Samples at at most
0.2 mm spacing check selected parity and cutter-to-boundary distance for D2.
The 0.002 mm test allowance covers serialized coordinate rounding; it is not a
new production tolerance or a physical accuracy claim. The simple red fixture
also remains as its own regression.

This is a bounded rectilinear oracle, not a proof of arbitrary curved topology or
complete area clearing for every strategy. Existing concavity, islands, source
selection, offset and volume tests remain the broader regression layer.

The deep-Pocket CPS test independently decodes OpenSBP M3/J2/JZ and compares every
resolved move endpoint, kind and line association with the returned preview
(1e-9 representation comparison; no machine-accuracy implication). Postprocessor
protection is unchanged. [Native/WASM parity](v1-stabilization/wasm-native-parity.json)
also compares the complete LinuxCNC program byte-for-byte: 9,804 preview moves and
a verified cut in the depth-two region using the freshly built WASM.

## Actual UX review and persistence

Reviewed in the in-app browser on macOS with Vite 18803 / native API 18804.
The existing API process was sufficient for text preview; corrected geometry is
qualified by fresh Rust tests, CLI and rebuilt WASM, not by claiming that old
server process was rebuilt.

- Project T2 D12 opens its own draft. Apply D10 updates the operation; inventory
  T2 stays D6. Undo restores D12; Redo restores D10.
- Cancel then Discard leaves D10. The dirty-draft confirmation is preserved.
- Save through the real browser UI, reopen the downloaded project: D10 remains.
  Saved text, operations and manually authored feed/plunge/RPM values are unchanged.
- Text is clearly framed on desktop (1280×900 and 1440×960) and narrow (900×800)
  views. Valid text source shows “not generated” rather than “no drawing”.
- Stable-frame unit coverage checks asynchronous glyph arrival, movement without
  reframing, explicit Fit and document replacement. Manual pointer dragging was
  not separately qualified in this checkpoint.

[Structured checks](v1-stabilization/ui-checks.json),
[desktop reopen](v1-stabilization/text-reopen-desktop.png),
[narrow reopen](v1-stabilization/text-reopen-narrow.png),
[project editor](v1-stabilization/tool-after.png).
Existing history/generation-context regressions verify that tool changes invalidate
output even after Save and that Undo/Redo preserve manual cutting parameters.

## Build guard contract

The fingerprint includes root Cargo manifests/lockfile, toolchain files, `.cargo`,
WASM/core/feeds-speeds sources and manifests/build scripts, embedded core assets,
feeds-speeds data, and the guard implementation. It detects changed contents even
with old timestamps, added/deleted inputs, and changed/missing output assets.
Build inputs changing during compilation prevent recording a successful stamp.
`wasm-pack` uses `--locked`; after success the guard validates the package again.
This is a local freshness/completeness contract, not a signed supply-chain attestation
or a guarantee of bit-reproducibility across Rust installations/environment flags.

Only `npm run dev` passes `--dev`. That mode may use a complete stale package when
wasm-pack is unavailable, with an explicit warning. Incomplete packages still fail.
`npm run build` has no such fallback. Ten Node guard tests are now part of `npm test`,
including missing assets, mixed bindings, old timestamps, concurrent source edits,
embedded assets and release/dev/no-toolchain distinctions.

## Executed checks

Final result: **1,202 core tests in series, 44 CPS, 1,313 frontend (1 existing skip),
and 10 WASM guard tests pass**. Counts are recorded in [checks.json](v1-stabilization/checks.json).
Logs are retained beside this report, including the failed parallel run rather than
silently replacing it. Full frontend tests, Svelte/TypeScript check, ESLint/Prettier,
core Clippy with warnings denied, schema drift check, CPS guards, rebuilt frontend
and WASM freshness verification are included.

The parallel core suite reproduced known ivac-htc8: the global pocket cache count
was 3 instead of 2. Serialized tests are the qualification run. This remains an
open test-isolation issue; it was not addressed by modifying geometry expectations.
Build warnings about large chunks and ineffective dynamic import remain informational.

Reproduction (repository root; Node 24 and Rust tooling on PATH, sccache disabled):

```sh
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps -- --test-threads=1
RUSTC_WRAPPER='' cargo test --offline -p ivac-cps
RUSTC_WRAPPER='' cargo clippy --offline -p ivac-core --features cps --tests -- -D warnings
RUSTC_WRAPPER='' cargo run --offline -p xtask -- schema-check
npm --prefix frontend test
npm --prefix frontend run check
npm --prefix frontend run lint
RUSTC_WRAPPER='' CARGO_NET_OFFLINE=true npm --prefix frontend run build
node frontend/scripts/ensure-wasm-fresh.mjs
RUSTC_WRAPPER='' cargo run --offline -p ivac-cli -- stream-gcode docs/releases/v1-stabilization/deep-pocket.request.json --output docs/releases/v1-stabilization/deep-pocket-native.ngc
node docs/releases/v1-stabilization/wasm-native-parity.mjs
```

## Remaining release gates and hard stop

The bounded code corrections do not make V1 release-ready. Current macOS DMG,
Windows 11 x64 NSIS and Ubuntu 26.10 amd64 DEB clean install/upgrade/runtime tests
remain required (follow-up ivac-2p53v). No Windows/Ubuntu runtime was available in this session; no VM,
installer installation or physical machining was performed. Three-platform candidate
qualification and current cold/warm desktop performance measurements remain pending.

GEOM1 human review (ivac-e2jnb) is explicitly still open. Other FS/UX/cancellation,
physical and release acceptance gates remain independent. Preserve the narrowly
validated M4 evidence; do not extend it to these changes.

**HARD STOP before the next checkpoint.** Next review: these bounded fixes, then
prepare/qualify desktop candidates and secure the Windows 11 test environment.
No commit, push, merge or release in this lot. Existing unrelated dirty work preserved.
