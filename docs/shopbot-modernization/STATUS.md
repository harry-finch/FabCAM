# Status — 2026-10-10

M2 software implementation and qualification complete; review/approval pending.
M1 was approved, committed and pushed. M3 has not started.

## Git state

- Checkout: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
- Fork/origin: `https://github.com/harry-finch/ivacam.git`.
- Upstream: `https://github.com/aalarchiv/ivacam.git`.
- Branch: `feature/shopbot-mvp`.
- HEAD and origin handoff: `7737a2a5c8e3766b03869b8e6618e3e32c7e1f0f` (approved M1).
- M2 source, tests, output fixtures, issue records and documentation uncommitted.
  Publication of M2 needs explicit authorization; no PR, merge or upstream sync.

## Result and validation

Bundled CPS id `shopbot`, metric XYZ and `.sbp` extension; **manual spindle/RPM
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

## Next action

Review M2 and authorize its commit/push. M3 needs explicit milestone approval:
gather actual ShopBot facts and produce the lab machine profile; mark unknowns
TO VERIFY ON MACHINE. M4 remains the hardware gate. ProtoMAX remains deferred,
assessment B; no CAM/process boundary change or waterjet semantics introduced.

STOP — waiting for human approval before starting the next milestone.
