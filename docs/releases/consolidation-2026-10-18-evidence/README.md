# Consolidation evidence — 2026-10-18

Software tests and diagnostic observations only. Read [checkpoint](../POST_AUDIT_CHECKPOINT_2026_10_18.md).
`checks.json` records environment, outcomes and file hashes. `source-sha256.json`
fingerprints882 current source/test/frontend assets and manifests; this is a dirty
working tree on the recorded HEAD, not a new commit or a packaged candidate. Previous
audit modifications were retained. Entry/final status inventories show this checkpoint's
new example/docs. Runtime source pipeline58 is unchanged by consolidation.

Commands below were executed from the repository, except npm commands from frontend.
Their stdout/stderr is retained in matching `.log` files. No tests invoked an installer
or connected to a machine. No benchmark/property-stage timing budget is claimed qualified.

```sh
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test cps_posts --test shopbot_m2_post --test snapshots_drill_tabs_whirl --test vcarve_qualification --test cad1_qualification --test v1_stabilization --test geom1_contract --test drawing_audit --test geometry_source_ux --test volume_validation --test waterline_acceptance
RUSTC_WRAPPER='' cargo test --offline -p ivac-feeds-speeds -p ivac-cps -p ivac-server -p ivac-wasm
RUSTC_WRAPPER='' cargo test --offline -p ivac-tauri --lib -- --test-threads=1
PATH=/opt/homebrew/opt/node@24/bin:$PATH node crates/ivac-cps/prelude/tests/run.mjs
RUSTC_WRAPPER='' cargo clippy --offline -p ivac-core --features cps --lib --example consolidation_probe -- -D warnings
RUSTC_WRAPPER='' cargo clippy --offline -p ivac-server -p ivac-cps -p ivac-tauri -p ivac-wasm --all-targets -- -D warnings
cargo fmt --all --check
git diff --check
```

Tauri ran outside sandbox for native filesystem events. Core serial scheduling avoids
known global-cache test interference; it does not fix/qualify parallel test isolation.
The optional FANUC oracle is absent:6CPS and1core integration test early-return despite
being reported PASS by Rust. They are excluded from executed counts. The independent
Node/V8 harness ran separately and passed39tests. Frontend1skip is retained explicitly.

From `frontend/`, with Node24 on PATH:

```sh
npm test
npm run check
npm run lint
```

## Replaying observations

```sh
RUSTC_WRAPPER='' cargo run --offline -p ivac-core --features cps --example consolidation_probe
```

The example includes `block-hatch.dxf` and emits JSON containing input requests, full
pipeline output and import outcome. `observations.json` retains that exact output;
`observation-summary.json` extracts bounds, warning kinds, JZ/J2/J3 lines and known
preview rapids. HATCH expected bounds are analytically(100,200)..(110,210); actual
bounds are(0,0)..(10,10). The SAFE-Z path includes known rapid17→5 at XY20,17 inside
the declared100x100 stock spanning Z0..18. These are **confirmed counterexamples**,
not tests that assert current incorrect behavior as the desired contract. Future fixes
must convert them to explicit desired-behavior regressions. The initial XY is unknown;
no fabricated decoder start pose is used. Full independent decoded-output property
implementation is proposed separately in TEST1.

No fresh browser render, WASM browser execution, installation, Windows/Linux run or
physical machining was performed. No package hashes are changed or publication made.
