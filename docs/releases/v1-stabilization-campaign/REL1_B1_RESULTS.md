# REL1-B1 — community macOS packaging checkpoint

2026-10-16 · ivac-2p53v.4 · Implementation authorized; no commit, push or publication.

## Implementation

- Existing ad-hoc overlay retained for local/candidate builds and selected explicitly
  by the generic macOS release build. Apple secret environment references removed
  from that workflow; no Apple credential configuration or access.
- New read-only scripts/verify-macos-community.py checks DMG integrity, mounts read-only,
  checks exactly FabCAM.app and Applications shortcut, verifies strict bundle signing,
  stable identity/version and exact architectures. Detaches in finally; rejects existing
  evidence paths. No signing mutation, Gatekeeper bypass or quarantine manipulation.
- Both workflows invoke this verifier before upload. macOS release evidence/checksums
  are uploaded and rechecked after artifact transfer before draft release assembly.
  No workflow executed in GitHub. Existing publication triggers are unchanged.
- Collector SHA256SUMS now covers both installer and candidate.json. macOS manifest
  labels community distribution and no Apple notarization without claiming qualification.
- Future draft release notes disclose ad-hoc/non-notarized status and Open Anyway.
  Release checksums already cover the assembled artifacts; verification evidence is added.
- Windows/Linux build and signing policies unchanged. No CAM, data path, bundle ID,
  project schema, application source behavior or upstream attribution changed.

## Validation

Four Node collector tests PASS, including metadata byte checksums and preservation of
previous destinations. Four Python tests PASS: identity changes and missing/unexpected
architectures rejected, valid arm64/universal identities accepted. Both workflows pass
actionlint. These tests do not substitute for actual DMG or clean-host checks.

First local app+DMG build: compiled and ad-hoc signed, then sandbox hdiutil create failed
with “Device not configured”. Original log retained; local retry requested with the
necessary sandbox permission. Final artifact results recorded below after completion.

## Clean-host gate

User reports another Mac is available. User identifies the host as MacBook M3 with macOS 14.6 (arm64), matching this candidate architecture. Exact OS build and clean state still require host verification.
No artifact has been transmitted. [Protocol](REL1_B1_CLEAN_INSTALL.md) is ready for
fresh browser download through an explicitly approved delivery channel. This checkpoint
has no publication/distribution authorization. Existing-profile native smoke tests remain
historical checkpoint E evidence, not a clean install. Clean-host acceptance is NOT TESTED
and remains a V1 release blocker. Missing Developer ID/notarization is NONBLOCKING.

HARD STOP at this checkpoint; user review before any next implementation or distribution.

## Final local artifact result

- Local retry build app+DMG PASS (rel1b1-build-retry.log). No notarization attempted.
- DMG hdiutil integrity PASS. Read-only mounting required the sandbox permission;
  after approval, strict signature and identity verification PASS and image detached.
  Initial sandbox attach failure preserved separately.
- FabCAM 0.9.0-beta.1, org.opcenter.ivacam, internal ivac-desktop, arm64 exactly;
  ad-hoc sealed bundle verified in the actual DMG. Applications shortcut present.
- Evidence: [verification JSON](rel1b1-dmg-verification.json),
  [successful mount/verify log](rel1b1-dmg-verification-retry.log).
- Preserved candidate in `dist-test/rel1b1-community/macos-arm64/` with DMG,
  candidate.json (dirty source/toolchain/WASM/source hashes), verification JSON,
  README, clean-install protocol and SHA256SUMS.txt. Independent `shasum -a 256 -c`
  from this directory: all five files PASS. An initial invocation from repo root
  found no manifest; corrected working-directory invocation passed.
- DMG SHA-256: `4a3d536f2a816cc04fef6660e3800026e15e7b8b5a562f1d20b9e38193d2d997`.
- No installed application replaced, source project modified or data deleted.
  Prior CI and REL1-A candidates remain preserved. No new native runtime or installed-copy
  acceptance claimed; checkpoint E remains separate historical evidence.
- No GitHub workflow dispatched, credentials configured/accessed, artifact transmitted,
  commit, push or release. Universal workflow changes are linted, not CI-built here.

Local engineering/preparation checkpoint complete. REL1-B1 release qualification remains
OPEN pending the clean-download/install/runtime record, installed-copy signature/identity
and human acceptance. Another Mac is available; MacBook M3/macOS 14.6 is the declared test host; clean state and approved delivery
channel remain to be established. Windows/Ubuntu Monday plan unchanged.

The checksummed candidate protocol is the original blank test template; the repository
protocol now records the user-supplied M3/macOS 14.6 host. Artifact bytes/checksums were
not changed after verification. No macOS 14.6 runtime success is inferred.
