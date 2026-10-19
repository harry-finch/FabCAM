# V1 Release Gate Finalization — 2026-10-19

Maintenance checkpoint following `V1_ENVIRONMENT_RELEASE_GATE_2026_10_19.md`. Source: HEAD `298feb7` plus the uncommitted changes listed below. No CAM, post, G-code/OpenSBP, provenance, schema or export-safety code touched.

## 1. Advisories — confirmed causes and recommended disposition

| Advisory | Crate | Path | Compatible fix? |
| --- | --- | --- | --- |
| RUSTSEC-2026-0370 / -0371 / -0372 | `zvariant`, `zbus_names`, `zvariant_derive` (5.x) | `zbus 5.16` ← `tauri-plugin-opener 2.5.4` ← `ivac-tauri` | **No.** The advisories say these crates were merged into `zbus` ≥ 6. `cargo update --dry-run` only reaches `zvariant 5.15.1` (same standalone crate). `tauri-plugin-opener` 2.5.5 and 2.7.2 (2.7.2 also needs Rust 1.90, repo pins 1.88) both still depend on `zbus ^5.9`. Resolving requires a zbus 6 migration upstream in Tauri's plugin — a major migration, not forced. |
| RUSTSEC-2024-0449 | `unsafe-libyaml` 0.2.11 | `serde_yaml 0.9.34+deprecated` ← `xtask` (build tooling only, not shipped) | **No low-risk fix.** Replacing `serde_yaml` (used in `xtask/src/main.rs` schema generation) risks changing generated `schema/openapi.yaml` formatting; the maintained fork `serde_yaml_ng` also uses `unsafe-libyaml`. |

All four are "unmaintained" notices, not vulnerabilities. **Recommended disposition (NOT applied, needs approval):** add four narrowly scoped, commented ignores to `deny.toml`'s `[advisories].ignore`, each stating "unmaintained, no vulnerability; transitive via <path>; revisit when Tauri's opener plugin moves to zbus 6 / when xtask drops serde_yaml":

```toml
"RUSTSEC-2024-0449",  # unsafe-libyaml unmaintained (xtask-only via serde_yaml; build tool, not shipped)
"RUSTSEC-2026-0370",  # zvariant unmaintained (transitive: tauri-plugin-opener -> zbus 5)
"RUSTSEC-2026-0371",  # zbus_names unmaintained (same path)
"RUSTSEC-2026-0372",  # zvariant_derive unmaintained (same path)
```
(Optionally also prune ~12 stale gtk3-rs ignores that no longer match — warnings only.) Result today: `cargo deny check bans licenses sources advisories` → bans/licenses/sources ok, **advisories FAILED**.

## 2. Tooling and documentation changes

- `README.md` / `README_de.md`: removed the hardcoded `0.9.0-beta.2`; the line now points to `docs/releases/VERSIONING.md` (which lists beta.3 as current, matching `Cargo.toml`). No new duplicated version number; heading parity, links and translation intact (`readme-parity: ok`, 12 headings).
- `docs/BUILDING.md`: added a short "Reproducible tool setup" note (Node 24 per `.nvmrc`, pnpm 10.34.7 local install since Node ≥ 25 has no Corepack, how Chrome is detected by the smoke gate).
- **pnpm declaration:** `packageManager` NOT added to `package.json`. `pnpm/action-setup` errors when both `packageManager` and `version: 10` are present, and `version: 10` appears in four workflows (`ci.yml` ×2, `release-desktop.yml`, `desktop-candidates.yml`). Switching is feasible (add the field, drop those `version:` lines) but changes CI/release workflows I cannot test here — decision for you.
- Outside the repo (user level): Node 24.21.0 from existing Homebrew `node@24`; pnpm 10.34.7 in `~/.local/pnpm-10`; `~/.local/chrome-shim/google-chrome`, a 2-line `exec` wrapper around `/Applications/Google Chrome.app` (a plain symlink makes Chrome abort, because it must be launched from inside its bundle). `scripts/ui-smoke.sh` and its acceptance criteria are unchanged.

## 3. UI smoke result — FAIL (stale script, not a confirmed app regression)

With Chrome available the gate runs for the first time. Output: boot OK; tabs tools/machine/settings/help/about responsive and visible; no page exceptions. Two checks fail:

1. `tab project: visible=NO` — the script expects `main.split` visible on the Project tab. Since the Atelier shell (`App.svelte:1228–1236`), `main.split` is hidden until a document is active and a Welcome screen is shown instead, by design.
2. `machine>tooling: rendered=NO` — the script clicks a `.sub-tab` button labelled "Tooling" / "Settings"; no `.sub-tab` element exists in `frontend/src` any more (the Machine sub-navigation was redesigned). The Settings form check passed.

The script (`scripts/ui-smoke-tabs.mjs`) was last updated before these UX changes. Conclusion: script defect — the smoke test needs updating to the Atelier shell (open/create a document first; use the current Machine sub-navigation selectors). I did not edit it: that changes the gate's acceptance criteria and should be reviewed. No app freeze or exception was observed.

## 4. Complete `scripts/pre-release.sh` result

Command: `PATH="/opt/homebrew/opt/node@24/bin:$HOME/.local/pnpm-10/node_modules/.bin:$HOME/.local/chrome-shim:$PATH" scripts/pre-release.sh` — Node v24.21.0, pnpm 10.34.7, rustc/cargo 1.88.0, wasm-pack 0.15.0, cargo-deny 0.20.2, Google Chrome (headless). Exit **1**.

Pass: fmt; clippy -D warnings + pedantic; cargo test --workspace (1609 passed, 0 failed, 1 ignored); xtask schema-check; readme parity; pnpm install --frozen-lockfile; codegen drift; i18n keys drift; lint; svelte-check; vitest (1438 passed, 1 skipped); build; wasm-pack + cps size guard.
Fail: **cargo-deny** (decision pending, §1); **ui smoke** (stale script, §3).
Skipped: none.

## 5. Remaining decisions / blockers

1. Approve (or reject) the four documented `deny.toml` ignores in §1.
2. Approve updating `scripts/ui-smoke-tabs.mjs` / `ui-smoke-table.mjs` to the current UI (I recommend it; it is the only way the gate can pass without weakening what it checks — it still must verify tab panels mount, stay responsive and the tool table works).
3. Decide on `packageManager` + removing `version: 10` from the four workflows.
4. Not covered by this gate: native installers, platform acceptance, human reviews, physical ShopBot qualification.

## 6. Files modified (all uncommitted) and reproduction

Modified: `README.md`, `README_de.md`, `docs/BUILDING.md`; added: this report and the previous one. Untracked evidence untouched.

```sh
npm install --prefix ~/.local/pnpm-10 pnpm@10.34.7
mkdir -p ~/.local/chrome-shim
printf '#!/bin/sh\nexec "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" "$@"\n' > ~/.local/chrome-shim/google-chrome
chmod +x ~/.local/chrome-shim/google-chrome
export PATH="/opt/homebrew/opt/node@24/bin:$HOME/.local/pnpm-10/node_modules/.bin:$HOME/.local/chrome-shim:$PATH"
cargo install --locked cargo-deny   # once
scripts/pre-release.sh
```

**Recommendation:** the software gate is not green: two failures, both with a clear cause and neither a CAM or production-code defect. HARD STOP — awaiting review.
