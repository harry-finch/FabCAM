# V1 Release Gate Closure — 2026-10-19

Follows `V1_RELEASE_GATE_FINALIZATION_2026_10_19.md`. Source: branch `main`, HEAD `298feb7`, plus the uncommitted changes below. No CAM, post, G-code/OpenSBP, provenance, schema or export-safety code changed.

## 1. Changes made

1. `deny.toml`: four approved advisory exceptions with dependency path, reason and review trigger (§2).
2. UI smoke rewritten for the Atelier shell (§3): `scripts/ui-smoke-tabs.mjs`, `scripts/ui-smoke-table.mjs`, new shared `scripts/ui-smoke-lib.mjs`.
3. `scripts/ui-smoke.sh`: cleanup race fix (see §4). Assertions and exit semantics unchanged.
4. README parity / version pointers and `docs/BUILDING.md` tool-setup note (from earlier checkpoints, still uncommitted).
5. `STATUS.md` and `DEVLOG.md`: one verified-facts note each; replaces the stale "README parity red; cargo-deny unavailable" remark.
6. Deferred as instructed: `packageManager` in `package.json` and any CI workflow change.

## 2. cargo-deny

`cargo deny check bans licenses sources advisories` → **advisories ok, bans ok, licenses ok, sources ok** (cargo-deny 0.20.2). Added to `[advisories].ignore`:

| ID | Crate | Path | Review trigger |
| --- | --- | --- | --- |
| RUSTSEC-2024-0449 | unsafe-libyaml | xtask → serde_yaml 0.9 (build tool, not shipped) | xtask drops serde_yaml / adopts a safe YAML emitter, or the advisory becomes a vulnerability |
| RUSTSEC-2026-0370 | zvariant | ivac-tauri → tauri-plugin-opener → zbus 5 | opener/Tauri moves to zbus ≥ 6, or a vulnerability is reported; remove 0370–0372 together |
| RUSTSEC-2026-0371 | zbus_names | same | same |
| RUSTSEC-2026-0372 | zvariant_derive | same | same |

All are "unmaintained" notices, not vulnerabilities. No dependency was upgraded; other ignores untouched.

## 3. Updated UI smoke coverage

Selectors use landmark names (`nav[aria-label="Main areas"]`, `"Machine sections"`), button text and `aria-pressed` / `data-tool-id`, not layout classes. Every evaluate has a freeze timeout; any uncaught JS exception now **fails** the run (previously only printed).

- Tabs: boot with six areas; Welcome shown with no project and workspace hidden; "New project" creates a project, Welcome disappears, workspace visible; Tool library / Machine / Settings / Help / About each responsive with exactly one visible panel and workspace hidden; Machine › General (settings form) and Machine › Tools (tooling rendered, section pressed); return to Project shows the workspace.
- Table: 120-tool seeded inventory renders 50 items, "Page 1 of 3", "120 tools"; search "plasma" → 12 items, no pager, "12 of 120 tools"; clear restores 120; sort by diameter (first item Ø0.5; page 2 begins at Ø3 mm, which proves the sort — unsorted would be Ø1.5); next page; selecting a tool shows its detail heading and `aria-pressed=true`.
- Negative control: changing one expected value made the table script print `TABLE SMOKE: FAIL` and withhold "all passed"; the script was restored.
- Coverage not previously present: Welcome/project creation, project return, exception failure. Nothing removed except selectors for UI that no longer exists (old `.sub-tab`, sortable column headers, `.table .row`).

## 4. Full release-script run

Command: `PATH="/opt/homebrew/opt/node@24/bin:$HOME/.local/pnpm-10/node_modules/.bin:$HOME/.local/chrome-shim:$PATH" scripts/pre-release.sh`
Revision `298feb7` + working-tree changes. Tools: Node v24.21.0, pnpm 10.34.7, rustc/cargo 1.88.0, wasm-pack 0.15.0, cargo-deny 0.20.2, Google Chrome 155.0.8059.82 (headless, via `exec` wrapper named `google-chrome`).

- Run 1: exit 1 — all gates passed except "ui smoke" although it printed "ui-smoke: all passed". Cause: a script defect — `cleanup()` ran `rm -rf` on Chrome's profile while Chrome was still writing ("Directory not empty"), failing the EXIT trap. Fixed by waiting for Chrome to exit and making profile removal non-fatal; test results still determine the exit status.
- Run 2 (final): **exit 0, "all gates green"**. 16/16 pass, none skipped: fmt; clippy -D warnings + pedantic; cargo test --workspace (1609 passed, 0 failed, 1 ignored); xtask schema-check; readme parity; pnpm install --frozen-lockfile; codegen drift; i18n keys drift; lint; svelte-check; vitest (1438 passed, 1 skipped); build; wasm-pack + cps size guard; cargo-deny; ui smoke.

## 5. Remaining release blockers

The software gate is green. This does **not** make FabCAM V1 release-ready or authorize publication. Still open and separate: native desktop installer qualification (macOS signing/upgrade; Windows/Linux packages untested); platform acceptance; human reviews (e.g. ivac-3i00w, ivac-b0js3 and other open gates in STATUS); physical ShopBot qualification; explicit authorization to publish. Environment notes: gate was run on Node 24 with a user-local pnpm and a Chrome wrapper outside the repo; CI continues to use its own setup; `packageManager` pin and `version: 10` workflow cleanup deferred.

## 6. Files modified (uncommitted)

Modified: `README.md`, `README_de.md`, `deny.toml`, `docs/BUILDING.md`, `docs/shopbot-modernization/STATUS.md`, `docs/shopbot-modernization/DEVLOG.md`, `scripts/ui-smoke.sh`, `scripts/ui-smoke-tabs.mjs`, `scripts/ui-smoke-table.mjs`.
New: `scripts/ui-smoke-lib.mjs`, `V1_ENVIRONMENT_RELEASE_GATE_2026_10_19.md`, `V1_RELEASE_GATE_FINALIZATION_2026_10_19.md`, this report. Untracked qualification evidence untouched.

## 7. Proposed commit (not made)

One maintenance commit on `main`:

```
chore(release-gate): make pre-release.sh green end-to-end

- README/README_de: restore heading parity, drop hardcoded version
- deny.toml: document four approved unmaintained-advisory exceptions
- ui-smoke: rewrite for Atelier shell, fail on JS exceptions, fix cleanup race
- docs: BUILDING tool setup; STATUS/DEVLOG verified-facts note; gate reports

No CAM, post, output or schema change. Software gate only; installer,
human and physical ShopBot qualification remain open.
```
Stage explicitly: the nine modified files, `scripts/ui-smoke-lib.mjs` and the three `V1_*_2026_10_19.md` reports; do not `git add -A` (untracked evidence directories). Commit/push await your authorization.

## 8. Reproduce

```sh
npm install --prefix ~/.local/pnpm-10 pnpm@10.34.7
mkdir -p ~/.local/chrome-shim && printf '#!/bin/sh\nexec "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" "$@"\n' > ~/.local/chrome-shim/google-chrome && chmod +x ~/.local/chrome-shim/google-chrome
cargo install --locked cargo-deny
export PATH="/opt/homebrew/opt/node@24/bin:$HOME/.local/pnpm-10/node_modules/.bin:$HOME/.local/chrome-shim:$PATH"
scripts/pre-release.sh
```
