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


## M1 — restored tooling and executed gates (2026-10-10)

The tables above are the historical M0 baseline. M1 used the pinned Rust
1.88.0 toolchain installed through the [official rustup installer](https://rust-lang.org/tools/install/),
with rustfmt/clippy and wasm32-unknown-unknown. Homebrew provided wasm-pack
0.15.0, sccache 0.18.0, Node 24.21.0 and Beads 1.3.1 (Dolt 2.4.2).
pnpm 10.34.6 is installed under `~/.local/share/ivacam-tools/node_modules/.bin`.
No repository manifest or lockfile change was required.

Homebrew updated simdjson during installation, which broke the pre-existing
Node 26.8.2 binary. It was repaired by upgrading that Homebrew Node to 26.10.0;
Node 24 was used for final validation. The first Rust run's CPS differential
failure was an environment failure from the broken Node binary. The repeated
unchanged-source baseline with Node 24 passed 1,136 tests before CAM fixes.

For reproducible commands on this host:

```sh
export PATH="/opt/homebrew/opt/node@24/bin:$HOME/.cargo/bin:$HOME/.local/share/ivacam-tools/node_modules/.bin:$PATH"
export RUSTC_WRAPPER=""
cargo build --workspace --exclude ivac-tauri --locked
cargo test --workspace --exclude ivac-tauri --all-features --locked
cargo test -p ivac-core --locked
cargo test -p ivac-core --test shopbot_m1_qualification --locked
cargo fmt --all -- --check
cargo clippy --workspace --exclude ivac-tauri --all-features --all-targets --locked -- -D warnings
cargo xtask schema-check
cargo xtask version-check
wasm-pack build crates/ivac-wasm --target web --release
cd frontend
pnpm install --frozen-lockfile
pnpm check
pnpm test
pnpm build
```

| Gate | Final observation |
| --- | --- |
| Headless Rust build | Pass, workspace excluding ivac-tauri |
| All-features workspace tests | 1,149 passed, 0 failed/ignored, including unit/integration/doc tests |
| Core without CPS | 1,060 passed, 0 failed/ignored, including doc tests |
| New M1 qualification suite | 11 passed across 16 static required fixtures; no optional skip |
| Clippy and formatting | Pass; all workspace non-Tauri targets/features, warnings denied |
| Schema/version guards | OpenAPI unchanged; Tauri version matches workspace 0.5.0 |
| Release WASM build | Pass; wasm-bindgen 0.2.120 built/cached by wasm-pack, wasm-opt cached |
| Frozen frontend install | Pass under pnpm 10; ignored esbuild lifecycle script warning, platform binary worked |
| Frontend check/test/build | Svelte 0 errors/warnings; 106 files, 1,044 pass and 1 optional locale skip; production Vite build passes |

Counts include upstream optional tests that return success when refs are absent;
those external DXF/Autodesk oracles remain unqualified. Test logs were saved
under `/private/tmp/ivacam-m1-*.log` for this session; durable evidence is this
record and the regression tests. No Tauri bundle, Windows/Linux execution,
cargo-deny audit, controller preview, VCarve or hardware check performed.
The final frontend build reran the WASM freshness guard after core source changes.


## M2 — OpenSBP qualification (2026-10-10)

Use the M1 PATH/RUSTC_WRAPPER environment above. Executed:

```sh
cargo test --workspace --exclude ivac-tauri --all-features --locked -- --test-threads=1
cargo test -p ivac-core --locked -- --test-threads=1
cargo test -p ivac-core --features cps --test shopbot_m2_post --locked
cargo clippy --workspace --exclude ivac-tauri --all-features --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask schema-check
cargo xtask version-check
cargo build --workspace --exclude ivac-tauri --locked
cargo run -p ivac-cli --locked -- posts
wasm-pack build crates/ivac-wasm --target web --release
# frontend: pnpm check; pnpm test; pnpm build
```

Observed: all-features workspace 1,158 pass, zero failed/ignored; feature-off
core 1,060 pass. Nine M2 tests added (six pipeline, three owned-post guards),
with parameter loops and five required goldens; all 11 M1 corpus tests pass.
Clippy, format, schema/version and headless build pass. CLI lists shopbot/sbp.
Frontend 106 files, 1,044 pass and one optional locale skip; Svelte clean.
Release WASM and production frontend build pass after packaging recovery.

A parallel workspace run failed the pre-existing global-cache count test
`pocket_cascade_cached_islands_change_key`: expected 2, observed 3. Its local
mutex does not exclude other pipeline callers. Serial suite passes; open issue
ivac-htc8. No cache source or synchronization change is hidden in M2.

Sandbox blocked wasm-opt during automatic freshness rebuilding, leaving JS/WASM
but no package.json. A retry skipped compilation because the current guard
considered partial output fresh; Vite could not resolve ivac-wasm. A complete
wasm-pack build with permission restored package metadata, then pnpm build
passed. Guard hardening is tracked as ivac-5g51; no fake package or config change.

Logs: `/private/tmp/ivacam-m2-*.log`, temporary session evidence. Goldens and
M2_QUALIFICATION are durable. Missing external refs remain unqualified; no
Tauri desktop bundle, Windows/Linux run, controller/VCarve/hardware validation.
