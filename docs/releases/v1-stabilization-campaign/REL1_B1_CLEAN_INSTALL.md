# REL1-B1 — clean-download macOS qualification record

Status: **NOT EXECUTED — release blocker**. This is a test record template, not evidence.
Use a clean Mac/VM without prior FabCAM data or per-app launch approval. Do not delete
existing user data to simulate cleanliness. Use a delivery channel separately approved
by the user; this checkpoint does not authorize GitHub upload/publication.

## Record before testing

- Tester/date:
- macOS version/build and architecture: user reports MacBook M3, macOS 14.6, arm64; exact OS build and clean state to confirm on host.
- Clean-environment provenance (new VM/account limitations):
- Candidate URL and browser/version:
- Source commit/dirty provenance/version:
- DMG filename, SHA-256 and matching candidate.json/evidence hashes:
- Quarantine state (read only); initial launch warning screenshot:

## Required observations

| Check | Expected | Result / evidence |
| --- | --- | --- |
| Fresh browser download | Exact final DMG/checksums; no pre-existing per-app approval | NOT TESTED |
| SHA-256 | All downloaded files match SHA256SUMS.txt | NOT TESTED |
| DMG and bundle | hdiutil verify; strict codesign; FabCAM / org.opcenter.ivacam; expected version/architecture | NOT TESTED |
| Install to Applications | FabCAM.app copied without overwriting unrelated data | NOT TESTED |
| Installed bundle | Strict signature and identity verified on installed copy | NOT TESTED |
| First launch | Standard Open Anyway when offered; record exact warning | NOT TESTED |
| Relaunch | Successful subsequent ordinary launch | NOT TESTED |
| Saved project | Open, save a new test copy, quit/reopen; contents retained | NOT TESTED |
| Native generation | Reference square and V-Carve; warnings and export guards retained | NOT TESTED |
| Preview/export | 3D visible, expected counts; square OpenSBP byte comparison with offline-square.sbp | NOT TESTED |
| Persistence | Test settings retained after restart; existing-data transition tested separately | NOT TESTED |

FabCAM is **not notarized by Apple**. If blocked solely for unidentified/unnotarized
software, the tester may follow System Settings → Privacy & Security → Open Anyway
for FabCAM, authenticate and confirm Open. See [Apple guidance](https://support.apple.com/en-us/102445).
Do not disable Gatekeeper or remove quarantine. Stop on damage/malware alerts,
checksum/signature mismatch, missing supported approval route or launch failure.
Record managed-device restrictions; never work around organizational policy.

Packaging/integrity failures and absent/failed clean-host results block V1.
Missing Developer ID/notarization/ticket alone does not. No hardware machining is
part of this protocol. Attach results to REL1_B1_RESULTS.md and obtain human review.
