# macOS native candidate smoke test — 2026-10-16

User authorized installation and runtime checks before Windows/Ubuntu availability Monday 19 October.
Candidate from CI run 38863177268, commit `5ac4023576d72d450912357887273195faab0a72`, version `0.9.0-beta.1`.
Host: macOS 26.5.2 (25F84), arm64. Installed to `/Applications/ivaCAM.app` from the verified DMG mounted read-only.
No prior bundle existed at that path. Existing user data WAS present: this is not a clean-account installation or a controlled upgrade qualification.

## Verified

- Backed up Application Support, WebKit storage and preferences under `dist-test/macos-runtime-38863177268/backup/` before launch. Backups are private local evidence, not tracked.
- Copied bundle without replacement; native executable independently identified as Mach-O arm64; plist version 0.9.0-beta.1.
- Launch succeeded, UI identifies FabCAM v0.9.0-beta.1 (5ac4023), `tauri://localhost`. No browser/server used for the test. Network was not disabled, so strict offline qualification remains open.
- Existing recent-project entries survived. Native Open and Save dialogs worked.
- Opened `tests/fixtures/shopbot-m4/offline-square.ivac-project.json`, generated 137 moves / 165 program lines, observed 3D stock/toolpath and expected manual-spindle warning.
- Native CPS OpenSBP export `dist-test/macos-runtime-38863177268/native-square.sbp` is byte-identical to `tests/fixtures/shopbot-m4/offline-square.sbp`: 4409 bytes, SHA256 `60b02f035137a4a850c06d61a1796cbcac7e0c10e6d910763d441d2e1055d8ff`.
- Opened stabilization `tool-edited-saved.ivac-project.json`: text Test and T2 diameter 10.00 mm retained (historical name PROJECT COPY D12 is unchanged). Native V-Carve generated 18874 moves / 18982 lines, export available, 3 warnings. This is a runtime smoke check, not new V-Carve geometric qualification.
- Saved as `dist-test/macos-runtime-38863177268/native-text.ivac-project.json`; parsed JSON equals the reference in every root field, including tools, text, operations and overrides.
- Quit via native menu, returned to welcome screen, reopened saved test copy through Reopen. Text outlines visible; generated output correctly absent until another generation.
- Source projects were not overwritten. App remains installed and open on the saved test copy.

## Distribution issue

`codesign -dv` reports only linker ad-hoc signature, Info.plist not bound, no sealed resources or TeamIdentifier.
Unsandboxed `spctl --assess --type execute --verbose=2 /Applications/ivaCAM.app` exits 1:
`code has no resources but signature indicates they must be present`.
The bundle has no quarantine attribute in this CLI-download path. Successful launch here does NOT validate Gatekeeper acceptance after a browser download. No protection was disabled, quarantine removed, or signature altered.
Distribution signing/notarization must be addressed separately and retested on a clean environment.

## Remaining qualification

Clean account/VM, quarantined distribution launch, controlled upgrade, full inventory persistence, raw SVG/DXF import matrix, deep pocket/refusal/stale/cancel matrix, strict offline repeat, timed performance, uninstall/reinstall remain open.
No physical machining, tag, release or new production code. Windows 11 and Ubuntu remain for Monday.
HARD STOP before another milestone; this report records a bounded successful native smoke test with a distribution-signature issue, not full platform acceptance.
