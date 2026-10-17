# REL1-A — public FabCAM package identity

> Policy update 2026-10-16: [REL1-B1/B2](REL1_B_PLAN.md) supersedes Apple identity/notarization as a V1 requirement. Community clean-install/integrity gates remain required. Historical evidence below is unchanged.

2026-10-16. Phase 0 policy approved by user. Separate review gate before UX7 and UX6-C.
No commit, push, release, credential access or notarization authorized/performed.

## Change

Tauri productName becomes FabCAM. This supplies .app/DMG, NSIS/MSI and Linux package/desktop names.
Existing org.opcenter.ivacam identifier, ivac-desktop executable, Rust crates, project extensions/kind,
data paths and GPL/upstream attribution are preserved. Tauri 2.11.0 path/desktop.rs derives application
config/data/local-data directories from the unchanged identifier, not productName.
CI artifact labels now FabCAM, including the legacy release workflow's matching download paths.
No workflow dispatched. Existing 5ac4023 installers remain untouched in dist-test/desktop-ci/38863177268.
Version stays 0.9.0-beta.1 for this unpublished local change; identify by source + dirty status and hashes.
Review beta increment before any subsequent publication.

## Simple transition from prior test installations

These are test installations, not supported automatic production upgrades. No migration framework,
package-manager replacement scripts, automatic uninstall or user-data deletion has been added.
Back up projects and application data before manual transition. Do not run both names concurrently.

- macOS: keep the archived old DMG; quit old ivaCAM, copy FabCAM.app into Applications. The new filename
  can coexist with ivaCAM.app; remove only the obsolete application bundle after verifying the new app,
  never its Library application data/preferences. Current installed baseline is deliberately preserved.
- Windows: before installing FabCAM, inspect the old test install and uninstaller. Back up application
  data and projects; uninstall the old application only with data-retention options verified. Never select
  removal of user data. If retention cannot be established, stop for review. Install FabCAM fresh and verify
  access to unchanged identifier-based data. Product-name install directory/registry/shortcut changes are
  accepted for these beta tests; automatic NSIS upgrade is not claimed. MSI's default upgrade GUID changes:
  no cross-name MSI upgrade claim or pinning/migration infrastructure added for unused test MSI installs.
- Linux: inspect whether iva-cam is installed. Preserve config/data and projects. Remove only the old
  test package (not purge/autoremove), inspect the package-manager transaction before approval, then install
  fab-cam. No automatic package conflict/replacement script is introduced. Stop if file ownership conflicts
  or any proposed action touches user data. Ubuntu host execution remains pending Monday.

These manual transition steps require host verification; no Windows/Linux upgrade acceptance is inferred.

## Ad-hoc development signing (bounded REL1-B remediation)

`crates/ivac-tauri/tauri.dev-signing.conf.json` explicitly selects signingIdentity "-".
Use `tauri build --config tauri.dev-signing.conf.json --bundles app,dmg -- --locked` from the Tauri crate.
This overlay is used only by the macOS test-candidate workflow. Production config has no ad-hoc override;
no credential store is accessed by this task. Tauri signs the final assembled bundle before packaging.
CI strictly verifies `FabCAM.app` before upload; do not mutate the signed bundle afterward.

Ad-hoc structural validity is not Developer ID trust, notarization or Gatekeeper distribution acceptance.
Those, and clean-host qualification, remain UNQUALIFIED. No Apple submissions are made.

## Evidence

Local build log: rel1a-build.log. Collector tests: 4 pass. Both workflows: actionlint pass.
Actual local app/signature/runtime verification is recorded below after build completion.
Windows/Linux build, public installed labels and manual transition: NOT TESTED for this change.
No CAM/post code changed; existing output goldens and earlier tests remain historical baseline evidence.

## Actual local result

- Build app+DMG PASS; `FabCAM_0.9.0-beta.1_aarch64.dmg`, preserved in `dist-test/rel1a/` with provenance.json (dirty source 4ebd433).
- DMG `hdiutil verify` PASS (rel1a-dmg-verify.log).
- Strict assembled-bundle codesign PASS (rel1a-signature.log): identifier org.opcenter.ivacam,
  ad-hoc + runtime flags, Info.plist bound, sealed resources present. No Developer ID identity.
- Plist CFBundleName/DisplayName both FabCAM; internal executable ivac-desktop; version unchanged.
- Installed /Applications/FabCAM.app without overwriting /Applications/ivaCAM.app. Installed copy strict verification PASS.
- Quit old build and launched new path: native UI shows FabCAM 0.9.0-beta.1 and local dirty build provenance.
  Existing recent-project list retained; pre-rename native-text.ivac-project.json reopened, text layer and
  operation present. No source project overwritten or data deleted. App left open for review.
- Developer ID, notarization, Gatekeeper distribution and clean-host installation: UNQUALIFIED;
  no credential lookup, Apple submission or security bypass performed in this implementation.
- Windows/Linux changes are configuration only: new packages and transitions NOT TESTED locally.

Build log explicitly reports ad-hoc signing of executable then assembled app, before DMG packaging;
notarization skipped. Existing Vite size/dynamic-import warnings remain. No CAM/output changes.

**HARD STOP — awaiting human review of REL1-A and development-signature results before UX7.**
