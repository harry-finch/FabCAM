# Desktop candidates — preparation checkpoint

**SEC1 update (2026-10-20):** both manual and tag-triggered builds now call the
shared mandatory CI qualification at the same source SHA. Both produce only the
three V1 candidate formats (macOS arm64 DMG, Windows x64 NSIS, Ubuntu amd64 DEB).
The old universal/tag-release path below is historical, not active. Manifests bind
source SHA and qualification run ID; current version is beta.3/pipeline63. Manual
draft promotion is explicitly disabled and has no publish step until protected
human approval is proven operational. See [strict qualification](../BUILDING.md#sec1-strict-software-qualification).
The dated preparation and old candidate evidence below retain their original scope.


> 2026-10-19 — **Consolidation UX A–D terminée, candidats BLOQUÉS** (beta.3/pipeline62). [Bilan et preuves](UX_V1_CONSOLIDATION_2026_10_19.md). Core1204, frontend1387/1 SKIP +10 Node, intégration59 et TEST1 optimisé256/256 PASS. Un correctif clavier local ; défaut sécurité lecture/export **ivac-3xqtb** et réserve chargement/format **ivac-52tpd** soumis sans correction. Revues **ivac-amibd / ivac-he4lt OPEN**. Candidats existants beta.3/pipeline61 inchangés. Aucun commit/push/publication ni nouveau candidat. **HARD STOP pour validation.** Les notices antérieures sont historiques.

2026-10-16 · FabCAM 0.9.0-beta.1 · ivac-2p53v.
User accepted the stabilization lot and authorized desktop preparation.
Windows 11 and Ubuntu workshop hosts become available again **Monday 19 October**,
according to the user. No scheduled automation or physical machining is implied.

## REL1-A update — 2026-10-16

Public productName is now FabCAM. Prior ivaCAM filenames and results below describe
preserved historical candidates. New local macOS test builds use
`--config tauri.dev-signing.conf.json` for an assembled-bundle ad-hoc signature.
The manual candidate workflow applies that overlay only on macOS and strictly
verifies the bundle before upload. No Developer ID signing or notarization is implied.
[Transition and qualification](v1-stabilization-campaign/REL1_A.md).

## Approved community distribution policy — REL1-B1

V1 macOS candidates retain valid ad-hoc signing and FabCAM public names. Apple
Developer ID/notarization/stapling are optional post-V1, not release blockers.
DMG packaging, identity, SHA-256 verification and successful fresh-download clean-host
installation/runtime remain mandatory. Publish checksums alongside release artifacts
only after explicit approval. Document “not notarized by Apple” and standard per-app
Open Anyway; never disable Gatekeeper or remove quarantine.
[Full plan and clean-host protocol](v1-stabilization-campaign/REL1_B_PLAN.md).
Windows/Ubuntu Monday tests and their policies are unchanged.

## Deliverables and limits (historical initial candidates)

| Target | Candidate format | Build environment | Installation/runtime gate |
| --- | --- | --- | --- |
| macOS arm64 | DMG containing ivaCAM.app (FabCAM display name) | Local macOS 26.5.2 arm64; manual CI uses macos-15 arm64 | Local build evidence below; clean install/upgrade remains separate |
| Windows 11 Intel/AMD x64 | NSIS `*-setup.exe` | Windows Server 2022 x64 hosted runner | Windows 11 host test pending Monday; runner is not Windows 11 qualification |
| Ubuntu 26.10 amd64 | DEB | Ubuntu 22.04 x64 hosted runner | Ubuntu 26.10 desktop test pending Monday; build baseline is not runtime qualification |

The initial macOS candidate is arm64, matching the reference Mac. No Intel Mac
qualification is claimed. The existing tag workflow retains its universal build;
this preparation does not remove that option or promise it tested.

Version remains 0.9.0-beta.1: this checkpoint prepares packaging of the current
beta rather than declaring RC/1.0. Before a subsequent source publication, reassess
whether the change merits beta.2 per [VERSIONING](VERSIONING.md). Existing bundle
identifier `org.opcenter.ivacam`, `productName=ivaCAM`, executable and data paths
remain unchanged; installation identity migration is outside this checkpoint.

## Build workflow

`.github/workflows/desktop-candidates.yml` is manual (`workflow_dispatch`) only,
with `contents: read`. It builds the three targets, runs frontend/packaging/version
checks, collects an installer plus `candidate.json` and `SHA256SUMS.txt`, and uploads
GitHub Actions test artifacts for 14 days. It does **not** tag, create a Release,
request signing or deploy. It must be committed/pushed before GitHub can execute
this new workflow; user explicitly authorized publication and manual dispatch on 2026-10-16.

Native Rust remains 1.88.0, Node 24, pnpm 10. Tauri CLI is the pinned npm binary
`@tauri-apps/cli@2.12.3`, also used locally. This avoids compiling a floating CLI
against the application's older Rust pin. Cargo builds use `--locked`. The existing
production WASM guard runs in Tauri's beforeBuildCommand. Frontend type checks run
after the build, because a fresh checkout has no generated WASM bindings yet.

Corrections to the preexisting tag workflow:

- Replace retired macos-13 with macos-15; install both actual Rust architecture
  targets rather than passing `universal-apple-darwin` to rustup.
- Collect universal DMG output from `target/universal-apple-darwin/...`.
- Pin Windows builder to windows-2022 and the CLI to the tested version.
- Fail missing installer uploads and require the entire build matrix to succeed
  before assembling a tag-triggered draft. Existing signing/publication behavior
  otherwise remains; no tag or release has been created here.

The collector refuses missing, empty, ambiguous/stale-version artifacts, mismatched
platform/target parameters, unverified WASM packages and a nonempty destination.
It records SHA-256, source commit/dirty status and the WASM fingerprint. Its declared
build target is not a substitute for independent architecture or runtime inspection.
Signature and installation are explicitly **not certified** by the manifest.

[GitHub macOS 13 retirement](https://github.blog/changelog/2025-09-19-github-actions-macos-13-runner-image-is-closing-down/),
[Tauri distribution formats](https://v2.tauri.app/distribute/),
[Windows installer/WebView2 behavior](https://v2.tauri.app/distribute/windows-installer/).

## Local reproduction

From the repository root, install Node 24/pnpm 10, the pinned Rust toolchain and
wasm-pack, then `pnpm --dir frontend install --frozen-lockfile`. With the matching
native OS prerequisites from [BUILDING](../BUILDING.md), use:

```sh
# Run in crates/ivac-tauri; choose exactly one target/format pair.
npm exec --yes --package=@tauri-apps/cli@2.12.3 -- tauri build --target aarch64-apple-darwin --bundles app,dmg -- --locked
npm exec --yes --package=@tauri-apps/cli@2.12.3 -- tauri build --target x86_64-pc-windows-msvc --bundles nsis -- --locked
npm exec --yes --package=@tauri-apps/cli@2.12.3 -- tauri build --target x86_64-unknown-linux-gnu --bundles deb -- --locked
```

Disable the local sccache override if unavailable: `RUSTC_WRAPPER=''` on macOS/Linux,
`$env:RUSTC_WRAPPER=''` in PowerShell. On Linux without lld, use `RUSTFLAGS='-D warnings'`
as the workflows do. Do not attempt to use the macOS executable on Windows/Linux.

From the repository root, collect the chosen target:

```sh
node scripts/collect-desktop-candidate.mjs macos aarch64-apple-darwin
node scripts/collect-desktop-candidate.mjs windows x86_64-pc-windows-msvc
node scripts/collect-desktop-candidate.mjs ubuntu x86_64-unknown-linux-gnu
```

Local macOS build in this session omits `--target` (native arm64), so its collector
uses third argument `target/release/bundle`. Output is gitignored under
`dist-test/desktop-candidates/<platform>/`. Preserve an old candidate in a separate
directory before collecting another; never mix installers from different builds.

## Monday acceptance protocol — software only

Record each result per platform using [the result template](desktop-preparation/PLATFORM_RESULT_TEMPLATE.md).
Retain the original installer and checksums with the results.

1. **Identity/checksum:** record OS edition/build, architecture, previous installed
   app version, artifact hash and candidate source commit/dirty flag. Verify SHA-256
   with `shasum -a 256` (macOS), `Get-FileHash -Algorithm SHA256` (PowerShell), or
   `sha256sum` (Ubuntu). Inspect digital-signature status; do not represent an unsigned
   test build as signed/notarized. Record any OS refusal rather than disabling protections.
2. **Clean install:** use a clean test account or VM snapshot. macOS: open DMG and
   copy app to Applications in that test environment. Windows: run NSIS and record
   WebView2 presence/bootstrap requirements; first install can require a network.
   Ubuntu: `sudo apt install ./<installer>.deb`; record installed dependencies and
   Wayland/X11 session. Confirm application name, version, architecture and launch.
3. **Upgrade/persistence:** on a separate snapshot, start from the prior recorded
   version with a saved project, tool inventory and machine profile. Back them up,
   upgrade without deleting app data, reopen and compare values. Do not claim upgrade
   qualification from a clean install or change package identity during this test.
4. **Native workflow:** import SVG and DXF references, set machine/tool, generate,
   inspect 2D/3D and export OpenSBP to a local file. Confirm native CPS is present and
   no localhost server is needed. Repeat offline after prerequisites are installed.
5. **Stabilization references:** reopen `v1-stabilization/tool-edited-saved.ivac-project.json`:
   text is visible and tool T2 D10 retained. Test operation-tool editing versus a
   deliberately different same-ID inventory copy, Cancel, Apply, Undo/Redo and
   save/reopen. Preserve feed/plunge/RPM overrides. GEOM1's own acceptance remains
   independent; use its fixtures without silently approving that milestone.
6. **CAM/output:** exercise deep nested Pocket, contour/island, V-Carve text and
   existing explicit refusal references. Compare bounds, depths and retained islands
   against software reference evidence. A screenshot alone does not qualify geometry.
   Verify generated file and preview correspond; keep program files as evidence.
7. **Stale/cancel/refusal:** after Generate change cutter/geometry; old output must
   not remain valid for export. Cancel generation and retry. Unsupported ShopBot
   strategies/RPM transitions must refuse explicitly, never export partial success.
8. **Performance/usability:** record cold startup and cold/warm generation using the
   same fixture and timings, native file dialogs, fonts, resizing, 3D contrast/navigation,
   close/relaunch and project reopening. Record errors, not just a green summary.
9. **Uninstall/reinstall:** test only in the disposable environment; document whether
   user data is preserved. No workshop machine execution is part of this protocol.

## Handoff

Build/inspection results are in [preparation results](desktop-preparation/RESULTS.md).
This checkpoint prepares candidates; platform installation/upgrade acceptance remains
open under ivac-2p53v. GEOM1 and existing FS/UX/physical reviews remain independent.
**HARD STOP before release or the next milestone.** User authorized commit/push and manual workflow dispatch on 2026-10-16; no tag or
Release is authorized. Installation acceptance remains separate.

## REL1-B1 packaging guards (local implementation)

Both macOS workflows now verify the actual DMG via
`scripts/verify-macos-community.py` before upload: read-only mount, strict ad-hoc
signature, public/stable identity and exact architecture. Collector SHA256SUMS includes
candidate.json; candidate workflow adds macOS verification evidence to the checksummed
set. The release workflow rechecks macOS hashes after transfer and includes verification
evidence in its final checksums. No workflow dispatched for this change.
[Checkpoint results](v1-stabilization-campaign/REL1_B1_RESULTS.md) and
[clean-host record](v1-stabilization-campaign/REL1_B1_CLEAN_INSTALL.md).
