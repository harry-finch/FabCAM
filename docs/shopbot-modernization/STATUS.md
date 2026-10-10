# Status — 2026-10-10

M1 software qualification complete, following approval to continue after M0
and install the required tools. Three CAM defects corrected; deterministic
SVG/DXF corpus passes. M2 has not started.

## Git state

- Checkout: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
- Fork: https://github.com/harry-finch/ivacam.
- Origin: `https://github.com/harry-finch/ivacam.git`.
- Upstream: `https://github.com/aalarchiv/ivacam.git`.
- Branch: `feature/shopbot-mvp`.
- Last committed/pushed handoff: `7752ef281350faca18f737b3cb13986e6abd0474`.
- Audited upstream: `9168104b4fc4e02e916740a54bd1e56b5019f662`.
- M1 source, tests, fixtures, issue records and documentation are **uncommitted**.
  Explicit authorization is required for commit/push under the master prompt.
  No PR or merge. Results below describe the working tree, not the M0 commit.

## Validation

Rust 1.88.0, wasm32 target, rustfmt/clippy, wasm-pack, sccache, Node 24, pnpm 10
and Beads installed. All-features Rust workspace excluding Tauri: **1,149 passed,
zero failed/ignored**; core without CPS: **1,060 passed**. The new suite has
**11 passing tests across 16 fixtures**. Headless build, Clippy, formatting,
schema/version guards, release WASM and frontend production builds pass.
Frontend: **1,044 passed, 1 skipped**; Svelte zero errors/warnings.
See [M1 qualification](M1_QUALIFICATION.md) and [testing](TESTING_STRATEGY.md).

No Tauri desktop bundle, Windows/Linux run, VCarve comparison, ShopBot Preview
or physical machining performed. Optional external oracles are absent. No SBP
post exists yet. Evidence qualifies the specified simple milling corpus.

## Next action

Review M1 and authorize commit/push. M2 requires milestone approval: CPS/native
ADR, authoritative OpenSBP research, minimal post implementation and tests.
Future ProtoMAX remains deferred, assessment **B — moderate architectural
extension**; M1 changes no process or serialization boundaries.
Beads `ivac-h52u` and `ivac-n92a` closed with software evidence.

STOP — waiting for human approval before starting the next milestone.
