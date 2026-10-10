# Status — 2026-10-10

M3 software profile and documentation complete; review/approval pending.
M2 was approved, committed and pushed. M4 has not started.

## Git state

- Checkout: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
- Fork/origin: `https://github.com/harry-finch/ivacam.git`.
- Upstream: `https://github.com/aalarchiv/ivacam.git`.
- Branch: `feature/shopbot-mvp`.
- HEAD and origin handoff: `047a75872ca7ea7d28077555c9859f92fa9a7411` (approved M2).
- M3 snapshot, tests, issue record and documentation uncommitted.
  Publication of M3 needs explicit authorization; no PR, merge or upstream sync.

## Result and validation

Native [Sorbonne profile](../../machines/sorbonne-shopbot-prs.ivac-machine.json)
loads through existing machine import, saves/reloads and applies as a workspace
profile. User reports gantry 120 × 60 cm, 2011, SB3.8; nominal XY 1200 × 600 mm.
All remaining physical facts are **TO VERIFY ON MACHINE**, in the
[machine record](shopbot/SORBONNE_MACHINE.md). Zero clearance deliberately blocks
SBP export; zero Z travel is an unknown sentinel. No numeric physical speed/RPM
recommendation, new UI/schema/API or production CAM/post changes.

M3: frontend load/save/profile/CPS/wire integration **1 pass**; Rust CAM-to-post
clearance refusal **1 pass**; Svelte/TypeScript **zero errors/warnings**. Existing
M2 qualification remains the regression/build baseline. Native dialogs/filesystem
mocked; no live picker or physical machine test. See [M3 qualification](M3_QUALIFICATION.md).

M2 baseline: bundled CPS id `shopbot`, metric XYZ and `.sbp` extension; **manual spindle/RPM
and tool changes**, mandatory operator pauses. XY arcs linearized at 0.01 mm;
simple/dwell/peck drilling expanded. Unsupported policies/records fail explicitly.
CPS-enabled transports expose the post; standalone browser WASM cannot run CPS.
See [M2 qualification](M2_QUALIFICATION.md) and [ADR-0001](adr/ADR-0001-shopbot-cps.md).

Rust all-features workspace excluding Tauri: **1,158 pass** with serial test
execution; core without CPS **1,060 pass**. Nine M2 tests include parameter loops,
five required full-text SBP goldens and independent output-coordinate checks.
Existing M1 corpus passes. Clippy/formatting, schema/version, headless Rust,
release WASM and frontend builds pass. Frontend: **1,044 pass, one optional
locale skip**, Svelte zero errors/warnings. CLI `ivac posts` lists shopbot/sbp.

No controller parser/Preview, physical cut, VCarve, Windows/Linux run or Tauri
bundle tested. Pocket golden has 31,733 lines; buffering/performance unverified.
Existing cache-test parallel race (`ivac-htc8`) and partial WASM freshness guard
(`ivac-5g51`) remain open; current workaround/results recorded in TESTING_STRATEGY.
M2 Beads task `ivac-732u` closed after software qualification.
M3 Beads task `ivac-c5lt` closed after profile qualification. Rust formatting,
targeted Clippy, frontend test ESLint/Prettier and Git whitespace checks pass.

## Next action

Review M3 and authorize its commit/push; authorize M4 separately. Before physical
work, complete the machine record with verified travel, jog/feed/plunge limits,
spindle/RPM and zero/tool-change/clearance procedures. M4 remains the hardware gate.
ProtoMAX remains deferred,
assessment B; no CAM/process boundary change or waterjet semantics introduced.

STOP — waiting for human approval before starting the next milestone.
