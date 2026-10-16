# Desktop preparation results — 2026-10-16

Preparation subtask ivac-2p53v.1 complete; parent platform qualification ivac-2p53v remains open.
User accepted stabilization and authorized this checkpoint. Windows/Ubuntu hosts
are unavailable until Monday 19 October, per the user. No automated reminder created.

## Actual produced candidate

- Version: 0.9.0-beta.1; reference commit `a65553b5db5c2cfd8949aa95cbe539698b0fdbbf` with local packaging changes (dirty stamp).
- File: `ivaCAM_0.9.0-beta.1_aarch64.dmg`; 10,803,320 bytes.
- SHA-256: `93b3e5eebb740ab874c3dc90e473ceef2bb4641d1687290be16bce87e77b24d7`.
- Artifact: `dist-test/desktop-candidates/macos/ivaCAM_0.9.0-beta.1_aarch64.dmg` (gitignored).
- Binary is Mach-O arm64, bundle display name FabCAM, identifier org.opcenter.ivacam.
- Ad-hoc linker signature only: no Developer ID/notarization certification. Not a public release.
- Application build succeeded; DMG created and `hdiutil verify` reports VALID.
- Actual clean installation, upgrade and application runtime were **not** tested in this preparation.

## Executed checks

| Check | Result | Evidence |
| --- | --- | --- |
| Collector regression suite | 4 pass, three target formats + fail-closed cases | [log](collector-tests.log) |
| Both workflows | actionlint 1.7.12 pass, shellcheck disabled | [log](workflow-lint.log) |
| Version alignment | Cargo/Tauri both 0.9.0-beta.1 | [log](version-check.log) |
| Local frontend/WASM/native app/DMG build | pass, locked Cargo; existing Vite size/import warnings | [log](macos-build-unrestricted.log) |
| Disk image internal checksum | VALID | [log](dmg-verify.log) |
| Version/identity/architecture/signature | inspected | [record](macos-inspection.json) |
| Artifact SHA-256 and source/WASM provenance | recorded | [manifest](macos-candidate.json) |
| Whitespace diff | pass | git diff --check |

First sandbox build compiled the application but hdiutil could not create the DMG
(device not configured). It was rerun with the approved filesystem/system permission;
the second build completed. Both logs are preserved. Temporary Tauri CLI had a dangling
symlink and was reinstalled from the standard npm registry, version 2.12.3.

No CAM/post/serialization production code changed. Full geometry suites were not
rerun for packaging-only edits; their previous bounded stabilization evidence remains
historical, not relabelled as new platform qualification.

## Pending

- User explicitly approved commit/push and manual dispatch on 2026-10-16.
  The preparation evidence above predates that publication; CI results follow separately.
- Run new manual workflow and obtain Windows NSIS / Ubuntu DEB. They are **not yet built**.
- Perform clean installation/upgrade and native workflow checks on all target hosts;
  Windows 11 / Ubuntu 26.10 access expected Monday. Hosted builder OSes do not close this gate.
- Developer signing/notarization, independent GEOM1/FS/UX reviews and physical/release
  acceptance remain separate. Do not suppress OS trust warnings to certify a candidate.

[Build commands and acceptance protocol](../DESKTOP_CANDIDATES.md).
**HARD STOP before release/next milestone.** Publication and manual CI build are now authorized; runtime qualification follows on Monday.
