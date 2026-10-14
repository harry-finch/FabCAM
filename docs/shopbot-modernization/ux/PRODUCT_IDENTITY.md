# FabCAM product identity, attribution and documentation — 2026-10-14

Engineering task ivac-9td7; human review **ivac-7pul**, accepted on 2026-10-14.
User established FabCAM as the public product name and approved commit/push of
the branding, identity and documentation to `main`. No new functional milestone,
release, Git rename or installer migration is part of this checkpoint.

## Branding audit

Repository-wide case-insensitive search covered `ivaCAM`, `IvaCAM`, `ivacam`,
`IVACAM`, `ivac` and `FabCAM`, including hidden nonignored files. The complete
inventory found 415 matching text files before this handoff was added. Generated
build/dependency caches and Git metadata were excluded; no private pack data was
copied into the handoff or public assets.

| Class | Findings and disposition |
| --- | --- |
| A — Public product identity | Browser/window titles, About, EN/DE quit/menu/font/post tooltips, report caption, file-picker labels, startup diagnostic, active README/build/user docs. Updated to FabCAM. |
| B — Upstream history/attribution | Original ivaCAM by Soenke J. Peters, upstream URL, ivaCAM-authored CPS posts, viaConstructor/Estlcam acknowledgements, original contributor notices. Retained and explicitly contextualized. |
| C — Internal technical identifiers | `ivac-*` Rust crates/binaries, `ivac-frontend`, API/IPC names, `__IVAC_*` defines, environment variables, existing toolchain/workflow names. Preserved. Cargo's public repository metadata and Tauri package description now identify the fork. |
| D — Persisted/install contracts | `org.opcenter.ivacam`, `productName=ivaCAM`, workspace/settings keys and paths, extensions/schema/migration IDs, CPS IDs/vendor metadata, machine-code provenance headers. Preserved. |
| E — Historical records/fixtures | Prior DEVLOG/ADR/qualification entries, old snapshots and machine-code/serialized fixtures, inherited Windows evidence. Preserved; active guides explicitly distinguish inherited evidence from current fork checks. |

Active frontend remaining mentions are intentional: a project-parser comment,
generated API description, workspace compatibility path/comment and legacy
machine-output provenance. Historical report-test inputs continue to retain their
original `generated_by` data while the newly generated report caption is FabCAM.

## Public names and browser identity

The already-correct header/Welcome and exact approved A1 assets remain unchanged.
Browser title is `FabCAM v0.5.0 (git-describe)`; HTML title/application-name and
description match. About, report identity/caption, user-visible picker names and
localized English/German product references use FabCAM. Bug-report links target
`harry-finch/FabCAM`, without opening/submitting an issue automatically.
The browser loaded the same canonical favicon and logo assets; header remains
30.2890625 px in the 1280 × 800 review viewport.

## Tauri identity and compatibility boundary

Window title and bundle descriptions use FabCAM. On macOS,
`bundle.macOS.bundleName` plus `Info.branding.plist` set CFBundleName and
CFBundleDisplayName to FabCAM. The actual runtime menu/window present FabCAM.
The bundle filename remains `ivaCAM.app`, executable `ivac-desktop`, identifier
`org.opcenter.ivacam`, version 0.5.0. Existing permissions, file-association
extensions/roles, signing, security and update configuration are unchanged.

Global productName stays ivaCAM: the installed Tauri utils 2.9 documentation and
[official reference](https://v2.tauri.app/reference/config/#wixconfig) derive the
default WiX upgrade GUID from productName. A global cosmetic change can alter
installation/package paths and upgrade identity. Windows/Linux installer/launcher
metadata therefore remains a known legacy boundary, not a proven migration.
[ADR-0012](../adr/ADR-0012-fabcam-product-identity.md) records the decision.

Tauri's actual path resolver uses bundle identifier for app-data/config paths.
The additional `ivacam/workspace.json` path and `ivac-workspace` browser key are
unchanged. Previously saved settings remained accessible and the existing M4-r2
project reopened from native recents: 4 SVG segments, one operation, the original
ShopBot machine and 500 × 500 × 12 mm stock. No save, conversion, generation or
physical machine action was performed on that file.

## README and active documentation

English and German READMEs now describe FabCAM's practical workshop focus,
implemented import/CAM/setup/FS/fixture/preview capabilities, qualification limits,
platform evidence, verified commands, basic workflow, roadmap, origin and license.
Current A1 screenshots replace the obsolete upstream social illustration.
Build, Windows, Quickstart, architecture, contributing, CPS, frontend/deploy and
fork-overview introductions are consistent. Legacy artifact names and upstream
authorship remain explicit. No historical DEVLOG entry or roadmap gate is rewritten.

FS4 remains deferred; remaining-material A/B is implemented within its existing
scope, while corner relief, surfacing, Rest Machining, FabSBP and future processes
retain their planned/deferred distinctions. Inherited additional operations are
not advertised as universally machine-qualified.

## About, version and preserved notices

About displays canonical A1 symbol, FabCAM, authoritative v0.5.0, a concise
description, ivaCAM origin/author/upstream link, GPL and fork source/status links.
Original acknowledgements, build/date/copy action and full third-party license
table remain in keyboard-accessible disclosures. Links are visibly underlined;
keyboard Tab reaches the GPL link and Enter opens Build details.

The private frontend has no independent package version. Existing Vite resolution
uses Tauri version and feeds both title and About; Cargo/Tauri agree at 0.5.0.
No version bump. The legacy `v0.4.0-29-…-dirty` git-describe field is a commit stamp,
not a second application release version.

Upstream GitHub metadata and actual Cargo/LICENSE/ABOUT content were read from
`aalarchiv/ivacam`. Both Cargo workspaces declare GPL-3.0-or-later. LICENSE is
byte-identical to the upstream text. Soenke J. Peters attribution is verified
against upstream ABOUT; source copyright notices and acknowledgement prose remain.
GitHub's GPL-3.0 detector alone is not used to infer the `or-later` declaration.

Application-source GPL licensing is separate from dependency/data/resource notices.
No manufacturer pack, source-pack provenance/evidence level or private application
data was added to distribution. This checkpoint does not relicense user-created
projects/assets or claim upstream endorsement.

## Tests and builds

Machine-readable checks: [PRODUCT_IDENTITY_VALIDATION.json](PRODUCT_IDENTITY_VALIDATION.json).

| Evidence | Result |
| --- | --- |
| Frontend suite | 124 files, 1145 passed / 1 optional skipped |
| Type checks | Svelte/TypeScript: 0 errors / 0 warnings |
| Lint/format | ESLint and Prettier pass |
| Rust project tests | `cargo test --locked -p ivac-core --features cps --lib project::`: 29 pass, including round trips/locale invariance |
| macOS build | Optimized Tauri app built locally with `--bundles app -- --locked` |
| macOS metadata/runtime | FabCAM display/window/menu, stable identifier/executable/version; existing project/recents/settings read successfully |
| Browser review | Correct title/meta/version, loaded logos/favicon paths, accessible attribution/license and disclosures |
| Protected-file comparison | 361 files identical across core/FS/CPS/schema/persistence/public data/license; only baseline Cargo.toml repository URL metadata differs, verified as a one-line change |
| Version/license/branding | 0.5.0 consistent; LICENSE unchanged; A1 source geometry/palette unchanged |
| Windows/Linux | Configuration/identity boundary inspected; no native package/runtime/upgrade test |

Initial frontend failures identified an unused About localization key and the
intentional placeholder identity change in the hardcoded-string allowlist.
Both were fixed, then the suite/check rerun passed. No CAM source, numerical
fixture, serializer, migration or cutting-data change. The known broader CPS
two-sided test follow-up remains independent; no new physical qualification claim.

## Screenshots

All are genuine captures; native uses the built app, browser uses the production
frontend. Source record and file paths are retained for human review.

- [Native Welcome / FabCAM system title](IDENTITY-native-welcome.jpg)
- [Native existing M4 project reopened](IDENTITY-native-project.jpg)
- [Native About / origin and license](IDENTITY-native-about.jpg)
- [Native original acknowledgements expanded](IDENTITY-native-attributions.jpg)
- [Browser Welcome/title](IDENTITY-browser-welcome.jpg)
- [Browser About](IDENTITY-browser-about.jpg)
- [Browser attribution/license/build disclosure](IDENTITY-browser-attribution-license.jpg)

## Remaining naming and future migration

Keep technical/storage/schema/crate names for compatibility. The packaging
productName, bundle filename and Windows/Linux installer/launcher names remain
legacy. Proposed follow-up **ivac-snqn** must capture current WiX GUID/NSIS paths,
registry/shortcut/signing/update behavior and prove an installed upgrade preserves
data/recents/private packs before a packaging-name migration. No such migration
is implemented or authorized by this checkpoint.

Human product-identity/About/documentation review is accepted. Existing CAM/FS/UX
physical and milestone gates remain independent. Source publication is explicitly
authorized; no release or installer migration is authorized by this approval.
