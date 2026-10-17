# FabCAM version policy

## Current: 0.9.0-beta.2 — 2026-10-17

Source publication of CAM clearance/chaining/cache fixes, Feeds & Speeds audit
corrections, generation timing, UI consistency and packaging preparation advances
the beta counter. Previously built beta.1 candidates and their checksums remain
historical artifacts; they do not contain these later source fixes. No installer
rebuild, workflow dispatch, tag or binary release is implied by this publication.
Open human, clean-host and physical acceptance gates remain unchanged.

## Previous: 0.9.0-beta.1 — 2026-10-15

The user requested an assessment and version update after the V1 scope freeze and
bounded stabilization. The inherited 0.5.0 no longer represents the implemented
ShopBot milling workflow, CAM/FS/UX work, GEOM1 and stabilization fixes.

**0.9.0-beta.1** identified the first stabilization snapshot: the bounded V1
software scope is substantially implemented and undergoing integrated stabilization.
The number is a maturity decision, not a percentage complete or a qualification
claim. No intermediate 0.6–0.8 releases are implied.

The beta qualifier matters: GEOM1 human review, current macOS/Windows 11/Ubuntu
installation and upgrade qualification, integrated acceptance, user documentation
and release-level workshop evidence remain pending. See the
[V1 scope](FABCAM_V1_SCOPE.md), [stabilization results](V1_STABILIZATION_RESULTS.md)
and [current engineering status](../shopbot-modernization/STATUS.md).

## Progression toward 1.0

| Version | Meaning and gate |
| --- | --- |
| `0.9.0-beta.N` | Reviewed stabilization snapshots; increment the beta counter when publishing the next meaningful stabilization checkpoint. |
| `1.0.0-rc.N` | Required scope and release-blocking correctness issues resolved; required platform, integrated and workshop evidence reviewed; explicit human authorization to prepare a release candidate. |
| `1.0.0` | Final integrated candidate accepted by the user with required installation, documentation and physical evidence. |
| `1.0.x` | Compatible corrective releases after 1.0, with relevant regression and release checks. |

The approved roadmap retains 1.5 for bounded ProtoMAX waterjet and 2.0+ for
advanced manufacturing. These horizons do not authorize implementation or imply
that the present beta includes those capabilities.

## Version sources and publication

`Cargo.toml` workspace version and `crates/ivac-tauri/tauri.conf.json` must agree.
Cargo packages inherit the workspace version; Cargo.lock records their resolved
versions. The private frontend remains unversioned: Vite reads the Tauri version
for the window/browser title and About, with a separate Git build stamp.
Rebuild the UI/native application to update previously compiled artifacts.

Each source-publication checkpoint must record whether a version bump is needed;
a meaningful beta stabilization delivery increments its beta counter. Updating a
version does not itself create a Git tag, binary release or installer migration.
Historical qualification reports, screenshots and raw logs retain the version
actually tested. Runtime storage identities, file-format versions, package names
and GPL attribution are independent of the public application version.
