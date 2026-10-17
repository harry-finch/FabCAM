# REL1-B — approved community macOS distribution strategy

2026-10-16 · Planning decision approved by the user. Implementation awaits the next
explicit approval. No build, credential access, commit, push, publication or distribution
is authorized by this document. Supersedes the Phase 0 requirement for trusted Apple
identity/notarization as a V1 release gate; historical test results remain unchanged.

## REL1-B1 — Community macOS Distribution — V1 REQUIRED

FabCAM 1.0 may be distributed as an open-source community application through GitHub
Releases after explicit publication approval. **FabCAM is not notarized by Apple.**
A valid ad-hoc signature seals bundle content; it is not an Apple Developer ID identity.
An Apple Developer membership is not required by the approved V1 policy.

Required implementation and acceptance evidence:

1. Retain the valid assembled-bundle ad-hoc signing process for development and
   community builds. Use the existing tauri.dev-signing.conf.json overlay for community
   packaging; do not silently fall back to an unsigned or invalidly signed bundle.
2. Produce a correctly packaged FabCAM DMG. Verify disk image integrity, mounted bundle
   strict codesign verification, public name/version/architecture and stable bundle ID
   org.opcenter.ivacam. Verify the installed copy too. Record source revision, dirty state,
   toolchain, artifact names and hashes. Never modify a bundle after signing.
3. Generate SHA256SUMS.txt from the final distributable bytes and candidate metadata.
   Reverify independently after download. Publish checksums alongside artifacts only
   when a release is explicitly authorized. A checksum is integrity evidence, not
   an Apple identity or notarization claim.
4. On a clean macOS environment with no prior FabCAM approval/data, download the exact
   candidate afresh through a browser. Record URL/channel, OS/architecture, checksum,
   initial warning and quarantine state without removing it. A local unquarantined
   launch or existing-profile test does not satisfy this gate. Use a separately approved
   candidate delivery channel; do not publish merely to perform the test.
5. Install and launch using the standard per-application Open Anyway route below if
   offered. Verify relaunch, project open/save/reopen, native generation/preview/export
   and settings persistence. Record results and screenshots. Initial unidentified or
   unnotarized-app blocking is expected, not itself a defect; failure to install/run by
   the documented supported route remains blocking. Managed-host policy restrictions
   must be recorded, never bypassed; qualify the declared supported environment.
6. Preserve application data, internal crate/executable names, project formats, bundle
   identifier, upstream attribution and existing compatibility/safety guards. Keep the
   REL1-A documented beta transition and independent existing-data retention checks.

Release blockers: packaging defects, invalid ad-hoc signature, identity mismatch,
checksum mismatch, missing clean-install evidence or unsuccessful clean-install/runtime
checks. Missing Developer ID, notarization and stapled ticket are NONBLOCKING for V1.
A generic spctl rejection alone is not the community acceptance criterion; record its
reason and require the complete clean-host user flow. Damaged bundle/malware reports
are not to be normalized as expected unidentified-developer warnings.

## User installation guidance — draft for the release

Download the DMG and SHA256SUMS.txt from the approved FabCAM GitHub Release. Verify
its checksum before installation (for a directory containing all listed files,
`shasum -a 256 -c SHA256SUMS.txt`). Copy FabCAM.app to Applications, then launch it.
FabCAM is ad-hoc signed and **not notarized by Apple**.

If macOS blocks it because the developer cannot be verified or Apple has not checked
it, and you trust this verified download: open System Settings → Privacy & Security,
find the notice for FabCAM and choose **Open Anyway**, authenticate if requested,
then confirm **Open**. This approves this application; it does not disable Gatekeeper.
The precise wording depends on macOS. If the option is absent, installation fails,
or macOS reports damage/malware, stop and report the exact message and artifact hash.
Do not disable Gatekeeper, remove quarantine attributes, or prescribe terminal security
bypasses. The tester performs any OS approval deliberately; this plan does not grant
blanket permission to automate security dialogs.

Source: [Apple — Safely open apps on your Mac](https://support.apple.com/en-us/102445),
checked 2026-10-16. The clean-host procedure is planned, not yet executed.

## REL1-B2 — Developer ID signing and notarization — OPTIONAL POST-V1

Future opt-in improvement: Apple Developer membership, Developer ID Application signing,
notarization, stapling and first-launch acceptance without the community exception flow.
These are not FabCAM 1.0 blockers. Do not configure Apple credentials, certificates,
keychains or notarization services now. Revisit only with explicit future authorization.

## Current evidence and next implementation step

REL1-A and checkpoint E prove local ad-hoc structure and existing-profile launch; they
are not clean-download/clean-install acceptance. CI baseline artifacts remain preserved.
The generic release workflow currently contains optional Apple secret references and
macOS publishing behavior: implementation must audit the community path, ensure valid
ad-hoc packaging without Apple credentials, and require integrity evidence before any
future authorized upload/release. No workflow was changed or dispatched in this plan.

Next proposed step: implement REL1-B1 packaging/checksum guards and prepare the clean-host
protocol/candidate, then perform host checks when an appropriate environment and delivery
are authorized. Windows 11 x64 and Ubuntu amd64 tests remain scheduled for host availability
Monday 19 October; their signing/support policies and physical CAM gates are unchanged.

**HARD STOP — report this plan and await approval before implementation.**

Implementation authorization: user approved REL1-B1 on 2026-10-16, without commit, push or publication; original planning stop above is historical. Clean-host acceptance remains independent.
