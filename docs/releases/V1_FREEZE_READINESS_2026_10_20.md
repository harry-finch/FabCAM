# V1-FREEZE — baseline readiness and integration decision

2026-10-20 · `ivac-zdgjf` · **Option A integrated; conditional Go for human freeze review.**
The maintainer explicitly approved Option A after the first Phase A report.
Documentation is now reconciled with eb747eb and the newer SEC1 evidence.
The scope below remains subject to final human freeze approval `ivac-4k4qx`.
The exact proposed SHA and aggregate final result are in the
[final local receipt](../../dist-test/v1-freeze-2026-10-20/final-receipt.json).
It identifies this report-bearing commit after Git assigns its hash; acceptance
requires its successful complete run, not an earlier revision's green result.

## 1. Git and worktree state

Primary `ivacam` is on `main`, **eb747eb1e0002b85b7ecd41b7de94906c61a3937**.
Tracked working tree/index were clean when inspected. There are **8,228 untracked
files**, predominantly retained qualification evidence, including the two SEC1
audit reports. No untracked production source was identified. Nothing was deleted,
stashed, staged from another checkpoint or relocated.

Remote GitHub main was read through `gh api` and matches local `origin/main`:
**274f54399b1f1aa368007574a48a05bba2707fdb**. Local main is seven commits ahead;
no fetch, push or remote workflow was initiated. Remote lookup first failed inside
the sandbox, then succeeded with authorized read access.

| Worktree / branch | Revision | Disposition |
|---|---|---|
| `ivacam` / `main` | `eb747eb` | Integrated engineering + SEC1 handoff; primary checkout preserved |
| `FabCAM-docs-consolidation` / `codex/sec1-roadmap` | `7e1c24b` | One unmerged documentation reconciliation commit based on `6043bfe`; clean tracked tree |
| `codex/docs-consolidation` (no active worktree) | `744774d` | Already ancestor of main; guide/consolidation must not be integrated twice |
| `feature/shopbot-mvp` (historical) | `c9d3f3b` | Historical branch, not candidate source |
| `FabCAM-v1-freeze` / `codex/v1-freeze-readiness` | final SHA in receipt | Isolated integrated source; Option A merge + bounded diagnostic/regressions + current freeze documentation |

Tracked-clean does not mean evidence-free. Local evidence in the primary is not
silently transferred into an isolated checkout or treated as published evidence.

## 2. Integrated engineering baseline

Main contains NUMSAFE3 `a480181` / `6043bfe`, trusted-only CPS `5e789e5`, truthful
publication/cleanup results `b348921`, strict exact-revision qualification
`d7b36381a531d55c86ea9d39f490417e8b1eb525`, and final SEC1 handoff `eb747eb`.
[SEC1 results](SEC1_REMEDIATION_RESULTS_2026_10_20.md) record engineering completion;
`ivac-dqlol` is CLOSED, separate human review `ivac-1mqti` is OPEN.
No production implementation remains dirty in the inspected tree. Tracker
IN_PROGRESS entries for import/provenance/candidates do not by themselves prove
uncommitted source changes or human acceptance.

## 3. Documentation integration and options

A read-only three-tree comparison (`git merge-tree 6043bfe main codex/sec1-roadmap`)
shows conflict markers in **V1_READINESS, STATUS and DEVLOG**; CPS_POSTS changed
on both sides. It did not modify branches or create an actual merge.
The earlier reconciliation says SEC1-01/04/08 are still pending at 6043bfe;
main now has their implementation/evidence. Copying that old state would regress
current status. Its nine-finding follow-up, deferred CPS confinement and independent
UX entries remain useful. Its audit report link still depends on local evidence.

**Option A approved and integrated** as merge commit 57aab6a: the documentation
commit 7e1c24b is in the ancestry, without duplicating 744774d already on main.
Semantic resolution retains newer SEC1 facts/CPS policy and all pending reviews,
plus the nine-finding follow-up, deferred custom CPS confinement and UX entries.
No accepted ADR/historical report or primary evidence was changed. SEC1 audit
report integration remains an explicitly documented local-evidence dependency.

## 4. Compact human acceptance matrix

I = implemented; S = bounded software evidence; H = human checkpoint accepted;
P = current-candidate installed-platform qualification; M = release-level physical
qualification; R = release approved. **I → S → H → P → M → R are independent states.**
An earlier architecture approval is not H for a later implementation checkpoint.

| Gate | I / S and evidence | H / remaining limitation | Candidate vs release dependency |
|---|---|---|---|
| SEC1 `ivac-1mqti`; audit `ivac-1ahyt` | Yes / [SEC1 receipt](SEC1_REMEDIATION_RESULTS_2026_10_20.md) | OPEN; bounded policy/engineering approval exists, final review pending; remote enforcement untested | Review current correction/limits before endorsing baseline; remote promotion protection is a later publication gate, not permission to enable promotion |
| NUMSAFE3 `ivac-zulb3` | Yes / [results](NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md) | OPEN; budgets/per-file new-pair contract approved, implementation review pending; no hostile-parent/crash/pair-atomic guarantee | Baseline review; installed failure/cancel behavior still to qualify |
| ADR-0024 `ivac-3i00w` | Yes / [verifier/export results](UX_SAFETY_LOAD_RESULTS_2026_10_19.md) | OPEN; independent full-program verifier implemented; software tests do not accept physical output | Resolve baseline review; exercise candidate export/freshness/refusal workflows |
| ADR-0025 `ivac-b0js3` / `.1` | Yes / [follow-up](ADR0025_FOLLOWUP_RESULTS_2026_10_19.md) | IN_PROGRESS; design approved, checkpoint acceptance pending; back-face fixtures/dowels unqualified | Review supported single-sided provenance; no two-sided support inferred |
| ADR-0026 / import `ivac-feder` | Yes / [correctness](IMPORT_CORRECTNESS_RESULTS_2026_10_20.md), [units UX](IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md) | IN_PROGRESS; human UX/architecture checkpoint pending; legacy/unit/refusal limits retained | Review baseline scope and exercise installed imports/round-trip |
| Atelier `ivac-he4lt`, `ivac-amibd` | Yes / [consolidation](UX_V1_CONSOLIDATION_2026_10_19.md) | OPEN; A/B/C accepted, D/consolidation still require review | Usability/acceptance checks may use candidates once explicitly authorized; no implied closure |
| Docs `ivac-yg3sm`, guide `ivac-7w1e1` | Delivered / [guide](../feeds-speeds/CUSTOM_DATA_SOURCES.md) and loader evidence | OPEN; guide integrated; retention proposal not accepted | Docs scope review; not a missing FS engine or permission to remove evidence |
| FS0–FS3.9 | Delivered / [actual acceptance matrix](PROJECT_STATE_RECONCILIATION_2026_10_20.md#feeds--speeds-milestone-matrix) | Bounded recorded acceptance; FS4 physical work deferred | Do not restart software milestones; restrict physical recommendation claims |
| Performance `ivac-53ky9`, `ivac-42ylx` | Existing bounded guards/evidence | OPEN conditional reserves; no unlimited workload/universal 200ms guarantee | Measure representative packaged workloads; actual supported freezes/OOM/incorrect success block affected release claims |
| Platform / workshop / release | Historical limited evidence only | P/M/R **not established** for current source | These are objectives of the future candidate campaign, not prior software test results |

No human gate was closed. See [V1 readiness](V1_READINESS.md) for the canonical
register; this matrix is a decision snapshot, not an acceptance action.

## 5. ivac-2zll — exact bounded V1 treatment

[Real CAM→ShopBot matrix](V1_POCKET_RPM_QUALIFICATION_2026_10_20.md): single-tool
rough/finish at different effective RPM is refused inside a section. A distinct
finish tool with actual wall offsets creates a section boundary and supports
17000→10000 in manual mode. Same tool ID does not split; no wall ring means no
finish tool/RPM is executed. Lab preloaded-tool-1 mode refuses tool 2 regardless
of RPM; ATC/M6/Ignore machine strategies remain refused. Positive and negative
controls run twice for exact determinism; four focused regressions PASS.

The bounded diagnostic adds operation/context/action while preserving refusal,
settings and emitted valid commands. `ivac-2zll` remains OPEN pending human V1
scope acceptance, not a missing blanket multi-tool capability. No new spindle
control/output contract was implemented or authorized.

## 6. Proposed capability contract — not freeze approval

| Capability | Proposed V1 disposition / explicit boundary |
|---|---|
| SVG/DXF and units | Supported within qualified import matrix; explicit unit choice, warnings and preserved source intent; no universal CAD compatibility |
| Stock / fixtures / work origin | Limited to accepted single-sided frames and SAFE-Z checks; physical zero/holding remains operator-established; translated/unqualified combinations retain refusal |
| Tools / machine profiles | Project copies, explicit ownership/provenance, actual supported tools; UI availability is not ShopBot capability |
| Feeds & Speeds | Retain FS0–FS3.9, source packs and manual overrides; bounded suggestions, no implied physical validation of every setting |
| Profile / Pocket / Drill | Bounded existing ShopBot tool/entry/geometry matrix; preserve unsupported-case guards |
| V-Carve / inherited operations | Only qualified subsets, explicit limitations; no blanket advanced clearing/Relief/Waterline ShopBot qualification |
| Multi-tool | Supported section/manual-tool combinations only; Pocket rough/finish matrix qualified above; same-section changes refused; lab preloaded mode excludes tool changes |
| OpenSBP | Bundled ShopBot, metric bounded milling and qualified scaffold; no new spindle/ATC/probing/controller semantics |
| Simulation / full-program verifier | Independent current full-program qualification/freshness; playback or successful bytes alone is insufficient |
| Program export | Complete qualified output; native per-file transactional contract, precise cleanup/partial-pair reporting; no pair-atomic/crash/hostile-directory guarantee |
| Legacy projects | Preserve serialized configuration; unsupported custom CPS remains data, not executable fallback |
| CPS trust | Bundled trusted execution only; imported/embedded/path custom execution refused before evaluation |
| Deferred | Real custom-CPS confinement, advanced UI/interactive tool illustrations, stock surface/core colors, 3D naming, advanced two-sided work, ProtoMAX V1.5 and V2+ CAM; retain future roadmap rather than declare complete |

These bounded classifications retain [V1 scope](FABCAM_V1_SCOPE.md). Software
qualification, human acceptance, installed-platform and physical evidence remain
independent; unsupported inherited/UI-visible operations are not promoted.

## 7. Integrated qualification and reproducibility

The [final receipt](../../dist-test/v1-freeze-2026-10-20/final-receipt.json) is
mandatory: it records exact source SHA/tree, full tracked-file fingerprints before
and after, complete gate exit, counts, corpus identity and logs. The proposed
candidate is **that exact tested commit**, not main, origin/main or an arbitrary
newer branch tip. The receipt supplies the self-referential SHA after commit.

| Check | Measured component result / final-run requirement |
|---|---|
| Workspace all-features | 1666 reported Rust PASS, 1 ignored TEST1 worker; must pass again in final aggregate |
| Strict/pedantic Clippy | PASS with repository's canonical allow-list; no weakened warnings |
| Schema/version/codegen/readme | PASS; no generated-contract drift |
| Frontend lint/types/Vitest/production build | PASS; 1460 Vitest PASS, 1 pre-existing locale assertion skipped |
| Import, NUMSAFE1/2/3, CPS trust, ShopBot M1/M2, SAFE-Z, V-Carve, provenance/verifier, native export | PASS in workspace; new Pocket matrix included, native file export 11 PASS |
| TEST1 watchdog | 256/256 PASS, 44.40s during preparation; separate final-commit replay required, budget 120s |
| Output corpus | 136/136 byte-identical to pinned NUMSAFE SHA-256 manifest; no error placeholders; final replay required |
| WASM / size / cargo-deny / headless smoke | PASS; no installed-platform or physical claim |
| Aggregate pre-release | Final exact-commit receipt must say exit 0 and 20/20; prior 19/20 is explicitly not qualification |
| Links | 289 relative links/anchors checked; one unchanged local-only audit dependency verified in primary |

Environment: macOS 26.5.2 arm64, Rust/cargo 1.88.0, Node 24.21.0, pnpm 10.34.6,
wasm-pack 0.15.0, wasm-bindgen 0.2.120, cached wasm-opt and Chrome exec wrapper.
A shared Cargo target cache is used; isolated `target/debug` points to that cache
so existing CLI checks execute the freshly built binary at their expected path.
This is build layout, not a production/test-guard change. Frontend dependencies
were installed from the frozen lockfile; no lockfile changed. The temporary corpus
runner uses current core source and the same dependency versions (only its own
local package differs). Its first direct-rustc linkage attempt failed due to
mismatched serde artifacts; the Cargo-resolved runner passes the pinned corpus.

Retained failed attempts: initial bootstrap lacked frontend/dist required by
Tauri and exposed two pedantic test-style issues, corrected and rechecked. The
second full run passed 19/20; its CLI gate could not find the shared-cache binary
at the isolated default path. After fixing the ignored build layout, both CLI
checks were rerun; final full qualification remains governed by the final receipt.
No failed run is counted as green, no source/test/evidence was hidden or removed.

The ignored TEST1 worker is exercised through the separate watchdog. Seven
optional FANUC oracle paths may return early because the non-distributed oracle
is absent; reported passes do not establish those seven oracle comparisons.
The frontend skip remains explicit. Build warnings remain warnings, not physical
qualification. No package/install/network-service/hostile-directory claim follows.

Reproduce with the pinned toolchain and documented build prerequisites:

```sh
RUSTC_WRAPPER='' scripts/pre-release.sh
python3 scripts/test1/build.py
python3 scripts/test1/run.py --binary <printed-executable> --output <new-directory> --cases 32 --budget 120 --frontend
```

Complete command/environment/log hashes, corpus harness and source fingerprints
are retained in `dist-test/v1-freeze-2026-10-20/`. Nothing was moved from earlier
evidence. The final qualification harness repeats the full gate, TEST1 and corpus
on one clean commit and rejects any tracked-source change during the run.

## 8. Remote CI and open blockers

GitHub read returned three successful historical candidate runs; latest is
**39125180897**, source `db5bde5d05b25328f29e27d210b7efeb8c0f429a`.
Current SEC1 source is not on origin/main and has no current-source remote
qualification established here. Local workflow fixtures/YAML parsing do not prove
remote execution. Push/workflow dispatch remain unauthorized and pending gates.

Immediate conditions for final freeze endorsement: successful exact-commit receipt and
relevant human checkpoint dispositions, including acceptance of the now-qualified
`ivac-2zll` limited-support matrix. Integration Option A is no longer pending. Later release gates:
remote CI, current installers/upgrade/runtime, representative performance, physical
ShopBot evidence and explicit V1 go/no-go. Deferred hardening is not automatically
a V1.5 prerequisite. Promotion remains disabled and must stay so.

## 9. Proposed functional freeze policy

Only after explicit approval of scope and exact final SHA: no new features,
broad UX redesign, speculative refactor or machine/post capability. Permit only
reproduced defects, safety, installation and qualification corrections, each with
regression evidence, focused commit and renewed acceptance of affected gates.
Any changed source creates a new candidate revision; earlier evidence must state
its applicability. **This report does not approve or activate the freeze.**

## 10. Candidate preparation plan — no installers built

Source SHA: **exact `source_sha` in the final receipt**, also stated in the
checkpoint handoff. Human approval of that full SHA is required before builds. Keep current version
`0.9.0-beta.3` / pipeline63 unless separately approved; use an explicit candidate
run/manifest identifier, not an invented release tag.

[Candidate workflow](../../.github/workflows/desktop-candidates.yml) calls shared
CI and produces artifacts, not a release. Proposed targets: macOS Apple Silicon
`aarch64-apple-darwin` app/DMG (macos-15 build runner); Windows 11 x64
`x86_64-pc-windows-msvc` NSIS (windows-2022 runner); Ubuntu Linux amd64
`x86_64-unknown-linux-gnu` DEB (ubuntu-22.04 runner). Test host OS/version and
architecture must be recorded separately; Ubuntu 26.10 is the existing candidate
tracking target, not proof from the 22.04 build. No Intel/universal or all-distro promise.

Require exact SHA/version/run/qualification identity plus SHA-256 manifests for
all transferred installers. macOS community ad-hoc signature/DMG verification;
Developer ID/notarization optional post-V1. Record Windows/Linux signing state
honestly; no signing claim inferred. Promotion workflow remains hard-disabled.
No build, release tag, draft, publication or hardware operation was performed.

## 11. Future platform and supervised workshop acceptance checklist

For each declared host: record OS/architecture, downloaded artifact/run/SHA-256,
clean install/launch, expected security prompts, basic UI, save/reopen, upgrade
and user-data preservation, imports/units, machine/tool copies, manual FS settings,
Profile/Pocket/Drill and included limited-operation fixtures, regeneration/cancel,
stale/unsafe export refusal and independently checked ShopBot output. Retain
failures/screenshots/logs; a CI package is not installed-platform acceptance.

For supervised ShopBot tests: pin machine/controller/post/mode, operator, work
zero, stock, fixtures, tools and authored limits; inspect output/controller preview,
perform authorized air cut before sacrificial-material tests; cover selected
Profile/Pocket/Drill, supported tool/RPM transitions and advertised limited modes;
record dimensions/depths, holding and stopping observations. Keep M4's historical
physical evidence bounded. No hardware authority is granted by this checklist.

## 12. Recommendation

**Conditional Go for human functional-freeze review**, conditional on successful
20/20 final exact-commit receipt and the recorded human scope/checkpoint decisions.
Option A and bounded RPM investigation are completed; no new functional milestone
is required before candidate tests. `ivac-2zll` is a qualified exclusion/support
matrix awaiting acceptance, not a blanket ban on dual-tool Pocket.

**No-Go for release or unapproved candidate construction.** Human review
`ivac-4k4qx`, independent existing reviews, remote CI, native platform/workshop
acceptance and publication remain separate. Passing tests closes none of them.

## 13. Exact next action and hard stop

Review the successful final receipt's exact tested SHA, this capability
matrix and pending human gates, and explicitly approve the functional freeze and
candidate construction. Any later code change requires a new qualified revision.
No push, tag, installer build, draft release, promotion activation or hardware.
**FINAL HARD STOP — awaiting human freeze and exact candidate-SHA approval.**
