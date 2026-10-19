# V1 Environment Maintenance & Release Gate — 2026-10-19

Software release-gate maintenance checkpoint only. Not a publication decision.

## 1. Initial state

- Branch `main`, HEAD `298feb7` (full `298feb7bace5866130b835c2f45bdda1cca5edb1`); ~970 untracked evidence entries (preserved, untouched); no tracked modifications.
- macOS (Darwin 25.5.0, arm64), Node v26.10.0, npm 11.19.1, rustc/cargo 1.88.0 (pinned), wasm-pack 0.15.0.
- Missing at start: `pnpm`, `corepack` (Node ≥ 25 no longer bundles Corepack), `cargo-deny`.

## 2. README parity

Original discrepancies (`scripts/check-readme-parity.sh`):
1. Heading skeleton: `README.md` has 12 headings (H1 + 11 H2); `README_de.md` had 7 (Overview, Platforms, Workflow, Status, Origin+License merged, Contributing were condensed).
2. The German language bar linked `README.md` instead of the literal `./README.md` the checker requires (hidden behind failure 1).

Corrections (`README_de.md` only): restructured to the same 12 headings (Überblick, Hauptfunktionen, Screenshots, Unterstützte Plattformen, Einstieg, Arbeitsablauf, Status und Einschränkungen, Planung, Herkunft, Lizenz, Mitwirken); translated the previously missing content (platform table, per-operation feature list, full limitations list, roadmap, project-origin and licensing details, contributing); language bar now `./README.md`. Links, build instructions, attribution and license text are preserved. The checker is unchanged and reports `ok (… share 12 headings)`.

## 3. pnpm

- Declared version: no `packageManager` field; lockfile `lockfileVersion 9.0`; CI pins `pnpm/action-setup` `version: 10`; `docs/BUILDING.md` requires ≥ 10.
- Installed `pnpm@10.34.7` into a user-local prefix (no global install, no repo change, lockfile untouched):

```sh
npm install --prefix ~/.local/pnpm-10 pnpm@10.34.7
export PATH="$HOME/.local/pnpm-10/node_modules/.bin:$PATH"
```

- Because Node 26 lacks Corepack, adding a `packageManager` field (to use `npm i -g corepack`) was considered but not done: it would alter `package.json` and is a decision for the maintainer.
- Caveat: CI uses Node 24 (`.nvmrc` = 24); this machine ran Node 26.10 and everything passed anyway.

## 4. cargo-deny

- Installed `cargo-deny 0.20.2` via `cargo install --locked cargo-deny` (builds with the pinned 1.88.0; ~1 min).
- Result of `cargo deny check bans licenses sources advisories`: **bans ok, licenses ok, sources ok, advisories FAILED**.
- Four errors, all class "unmaintained", **no vulnerabilities**:

| Advisory | Crate | Pulled in by |
| --- | --- | --- |
| RUSTSEC-2024-0449 | `unsafe-libyaml` 0.2.11 | `serde_yaml` ← `xtask` (dev tooling) |
| RUSTSEC-2026-0370 | `zvariant` 5.12.0 | `zbus` 5.16 ← `tauri-plugin-opener` 2.5.4 ← `ivac-tauri` |
| RUSTSEC-2026-0371 | `zbus_names` 4.3.2 | same |
| RUSTSEC-2026-0372 | `zvariant_derive` 5.12.0 | same |

- Also: ~12 existing `deny.toml` ignores (gtk3-rs RUSTSEC-2024-04xx) are no longer matched (warnings only; stale entries).
- Not remediated: adding ignores is a security-policy change; fixing requires upgrading Tauri plugin deps (zbus ≥ 6) or replacing `serde_yaml` in xtask. **Decision required.**

## 5. Full `scripts/pre-release.sh`

Command: `PATH="$HOME/.local/pnpm-10/node_modules/.bin:$PATH" scripts/pre-release.sh` at `298feb7` + the README_de change. Exit status **1** (only cargo-deny failed). Log kept in session scratchpad, not committed.

| Gate | Result |
| --- | --- |
| cargo fmt --check | pass |
| clippy -D warnings + pedantic | pass |
| cargo test --workspace --all-features | pass (1609 passed, 0 failed, 1 ignored by summed `test result` lines; earlier checkpoint cited 1,601) |
| xtask schema-check | pass |
| readme parity | pass |
| pnpm install --frozen-lockfile | pass |
| codegen drift / i18n keys drift | pass |
| pnpm lint / check | pass |
| pnpm test | pass (1438 passed, 1 skipped) |
| pnpm build | pass |
| wasm-pack build + cps size guard | pass |
| cargo-deny | **FAIL** (4 unmaintained advisories, §4) |
| ui smoke (headless) | **skipped** — no chromium/chrome on PATH |

No genuine production-code or test failures. Failure classification: cargo-deny = dependency-policy finding (not environment, not script defect).

## 6. Repository changes

- `README_de.md` modified (uncommitted).
- This report added (uncommitted).
- No dependency, lockfile, script, source, schema or deny.toml changes. Codegen guards left the tree clean. STATUS/DEVLOG not updated: nothing in them needed correction; update once the cargo-deny decision is made.
- Outside the repo: `~/.local/pnpm-10`, `~/.cargo/bin/cargo-deny`.

## 7. Remaining items / decisions

1. cargo-deny advisories: add documented ignores to `deny.toml`, upgrade deps, or accept red. (Awaiting approval.)
2. UI smoke gate not run (needs Chromium/Chrome on PATH).
3. Optional: declare pnpm via `packageManager` in `frontend/package.json`; prune stale gtk3 ignores in `deny.toml`.
4. Version inconsistency noticed: Cargo workspace is `0.9.0-beta.3`, both READMEs say `0.9.0-beta.2` (not changed).
5. Untracked qualification evidence (~970 entries) remains uncommitted.

## 8. Reproduce

```sh
npm install --prefix ~/.local/pnpm-10 pnpm@10.34.7
export PATH="$HOME/.local/pnpm-10/node_modules/.bin:$PATH"
cargo install --locked cargo-deny      # run outside the repo or with the pinned toolchain
scripts/pre-release.sh
```
wasm-pack must be on PATH for the wasm gates; install Chromium for the UI smoke gate.

## 9. Recommendation

The software release gate is **not yet fully green**: every gate that ran passed except cargo-deny (four unmaintained-crate advisories, no vulnerabilities), and UI smoke was skipped. It becomes green once the advisory policy decision is made and UI smoke is run. A green script would still not mean V1 is publishable: native installers, platform acceptance, human reviews and physical ShopBot qualification remain separate gates.
