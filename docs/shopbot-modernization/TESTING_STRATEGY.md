# Build and test baseline

Executed 2026-10-10 on macOS in the new checkout, before any CAM source changes.
This records observed results, not upstream claims. Full build instructions are
in [BUILDING](../BUILDING.md); the actual CI matrix is authoritative for gates.

## Environment

| Component | Required / observed |
| --- | --- |
| Rust | `rust-toolchain.toml` pins 1.88.0, rustfmt/clippy, wasm32 target; rustup/cargo not installed on PATH or usual cargo paths |
| Node | `.nvmrc` / CI use 24; observed v26.8.2 (different baseline) |
| pnpm | CI uses 10; observed 11.25.0 |
| Git / gh | 2.50.1 Apple Git-155 / 2.102.0 |
| wasm-pack | Not available; generated `ivac-wasm/pkg` absent |
| bd | Not available; issue tracking could not be initialized/queried |

## Commands and results

| Command (cwd) | Observed result |
| --- | --- |
| `cargo build --workspace` (root) | Could not start: command not found; zero crates compiled |
| `cargo test --workspace --tests` (root) | Could not start: command not found; Rust passed/failed/ignored counts unknown |
| `pnpm install --frozen-lockfile` (frontend) | 225 packages installed; exit 1 under pnpm 11 because esbuild build script was not approved (`ERR_PNPM_IGNORED_BUILDS`) |
| `pnpm test`, `pnpm check`, `pnpm build`, `pnpm run codegen` (frontend) | Wrapper attempted reinstall; aborted noninteractive modules removal (`ERR_PNPM_ABORTED_REMOVE_MODULES_DIR_NO_TTY`); actual runners then invoked directly below |
| `node node_modules/vitest/vitest.mjs run` (frontend) | 106 files passed; 1,044 tests passed, 0 failed, 1 skipped, total 1,045 |
| `node node_modules/svelte-check/bin/svelte-check --tsconfig ./tsconfig.json` (frontend) | 0 errors, 0 warnings |
| `node node_modules/typescript/bin/tsc -p tsconfig.node.json` (frontend) | Passed, no diagnostics |
| `node node_modules/eslint/bin/eslint.js .` (frontend) | Passed, no diagnostics |
| `node node_modules/prettier/bin/prettier.cjs --check src` (frontend) | Passed for frontend source files |
| `node node_modules/openapi-typescript/bin/cli.js ../schema/openapi.yaml -o src/lib/api/generated.ts` then `git diff --exit-code -- src/lib/api/generated.ts` (frontend) | Generated client unchanged; passed |
| `node scripts/ensure-wasm-fresh.mjs` (frontend) | Exit 1: missing WASM pkg and wasm-pack; frontend build cannot reach Vite |
| `node crates/ivac-cps/prelude/tests/run.mjs` (root) | 39 passed, 0 failed/skipped; fixture freshness checks passed; not a Boa runtime test |
| `scripts/check-readme-parity.sh` (root) | Passed, 7 matching heading skeletons |

Direct runner invocations use the installed dependencies without changing the
upstream manifests or dependency policy. The pnpm-generated untracked
`frontend/pnpm-workspace.yaml` was removed; no package/lockfile changes retained.
The esbuild platform binary sufficed for the existing Vitest run. No fake WASM
package was created. Rust/WASM tools were not installed as part of this audit.

The skipped frontend case is the optional strict locale-completeness check in
`frontend/src/lib/i18n/coverage.test.ts` (`skipIf(!strict)`). No flakiness was
established from this single run. Environment/tooling failures above are baseline
limitations, not regressions caused by M0. No full release/desktop bundle, Rust
schema guard, cargo-deny or Windows/Linux execution was completed here.

## Reproducible follow-up gates

Provide Rust 1.88.0, target wasm32, wasm-pack and the frontend's Node 24 / pnpm 10.
`.cargo/config.toml` requires sccache; CI uses `RUSTC_WRAPPER=""` as an explicit
override. Linux also has an lld target flag; GUI builds need platform WebView
dependencies and frontend dist. The broad `--workspace` command includes Tauri;
the fast CI logic matrix explicitly excludes it.

```sh
RUSTC_WRAPPER="" cargo test --workspace --exclude ivac-tauri --all-features
RUSTC_WRAPPER="" cargo build --workspace --exclude ivac-tauri
RUSTC_WRAPPER="" cargo test -p ivac-core
RUSTC_WRAPPER="" cargo test -p ivac-core --features cps
RUSTC_WRAPPER="" cargo xtask schema-check
RUSTC_WRAPPER="" cargo xtask version-check
wasm-pack build crates/ivac-wasm --target web --release
cd frontend
pnpm install --frozen-lockfile
pnpm run check
pnpm test
pnpm run build
```

## Existing test inventory and missing oracles

- Core unit tests co-locate with import, CAM, pipeline, post and simulation code.
- `ivac-core/tests/volume_validation/{profile,pocket,drill}.rs` provides existing
  analytical/material volume coverage; `snapshots_drill_tabs_whirl.rs`,
  `arc_sweep_parity.rs`, `tessellation_tolerance.rs`, `waterline_acceptance.rs`
  provide other regression/invariant tests. They were inspected, not executed.
- `ivac-core/tests/golden_corpus.rs` silently returns success after printing a
  skip notice when `refs/viaconstructor/tests/data` is missing: a green test
  count would not mean any DXFs ran. This checkout has no refs corpus.
- `ivac-core/tests/cps_posts.rs` and `ivac-cps/tests/golden_fanuc.rs` have optional
  Autodesk oracle files under refs; absent here. `ivac-cps/tests/differential.rs`
  also requires Node for the V8 side. Do not count missing oracles as coverage.
- Frontend tests cover state/history/machines/tools/UI helpers and rendering
  math; passing them does not validate the Rust CAM paths.

M1 must establish dimensional and topological assertions rather than relying
on visual plausibility or snapshot identity. Reuse existing tests first and
extend with the approved corpus. Controller and hardware gates remain human-led.
