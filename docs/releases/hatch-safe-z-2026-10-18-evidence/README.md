# HATCH / SAFE-Z targeted evidence — 2026-10-18

[Results and scope](../HATCH_SAFE_Z_RESULTS_2026_10_18.md).
Pipeline59 on uncommitted source, HEAD2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b.
`checks.json` records verified outcomes and hashes. `source-sha256.json` records the
current source/test assets; `changed-since-consolidation.json` identifies the targeted
changes against the preceding consolidation, preserving earlier audit modifications.

- `red.log`:3 regression failures reproduced before production correction.
- `focused-final.log`:14 focused regressions after correction.
- `core-final.log`:1,202 serial core tests; parallel global-cache isolation is still open.
- `integration-final.log`:12 suites,116 reported/115 executed (optional FANUC absent).
- `adapters-final.log`:HTTP16, native WASM22, CPS46reported/40executed.
- `node.log`:actual independent V8/prelude run,39PASS.
- `frontend.log`:1,328PASS/1skip plus10package-state Node tests.
- `probe.json`:same previous counterexamples now show explicit HATCH/clearance refusal;
  valid top0/rapid5 and top18/rapid23 retain successful output.
- `clippy-*.log`, `typecheck.log`, `wasm-check.log`, `fmt.log`, `diff.log`: static checks.
- `diagnostic-integration.log` and `diagnostic-adapters.log`: intermediate broader-test
  failures, subsequently resolved: stale expected ShopBot error origin, old synthetic
  zero-clearance fixture, HTTP streaming200 before validation. They are not final passes.

No test install, browser runtime or machine execution. Native WASM bridge tests and
wasm32 compilation do not establish browser WASM runtime qualification. Tauri compile/
Clippy checked; prior watcher tests are retained historical evidence. No new dependency.

## Reproduction commands

From repository root (Node24 placed on PATH for the Node/frontend commands):

```sh
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test hatch_safe_z
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test hatch_safe_z --test cps_posts --test shopbot_m2_post --test snapshots_drill_tabs_whirl --test vcarve_qualification --test cad1_qualification --test v1_stabilization --test geom1_contract --test drawing_audit --test geometry_source_ux --test volume_validation --test waterline_acceptance
RUSTC_WRAPPER='' cargo test --offline -p ivac-server -p ivac-wasm -p ivac-cps
RUSTC_WRAPPER='' cargo clippy --offline -p ivac-core --features cps --lib --test hatch_safe_z --test cps_posts --test shopbot_m2_post -- -D warnings
RUSTC_WRAPPER='' cargo clippy --offline -p ivac-server -p ivac-wasm -p ivac-tauri --all-targets -- -D warnings
RUSTC_WRAPPER='' cargo check --offline -p ivac-wasm --target wasm32-unknown-unknown
RUSTC_WRAPPER='' cargo run --offline -p ivac-core --features cps --example consolidation_probe
node crates/ivac-cps/prelude/tests/run.mjs
cargo fmt --all --check
git diff --check
```

From `frontend/`: `npm test`, `npm run check`.

Optional FANUC counts explicitly exclude early-return oracle tests. Their legacy
post-format fixture makes no stock-safety claim and retains its historical digest;
known-stock zero clearance now has a separate refusal regression. Arbitrary CPS,
custom axis transforms, fixtures/holders and unknown initial machine position are
not certified safe by the stock-clearance preflight. Full SAFE-Z remains ivac-j09eh.
