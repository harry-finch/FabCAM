# V1-FREEZE — baseline readiness and integration decision

2026-10-20 · `ivac-zdgjf` · **NO-GO for final freeze/candidate builds at this checkpoint.**
**Phase A decision stop**, as required by the V1-FREEZE request: the engineering
source is identifiable, but the requested combined documentation baseline has
conflicts and outstanding acceptance decisions. Later phases below are an inventory
and proposed plan, not completed integration, qualification or freeze approval.

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
| `FabCAM-v1-freeze` / `codex/v1-freeze-readiness` | based on `eb747eb` | Isolated report-only worktree created for this decision record |

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

**Option A — recommended:** approve reconciliation of `7e1c24b` onto the engineering
baseline `eb747eb`, retaining the newer SEC1 facts, all pending human gates, the
future security/UX entries and the existing FS guide. Prepare a reviewed scoped
integration commit; retain the original branches/evidence. Then qualify that exact
combined tree and present its resulting SHA for freeze approval.

**Option B:** explicitly defer the unmerged nonblocking roadmap update, use main
as the engineering candidate base, and record the documentation scope exception.
The actual final source still needs qualification and the remaining review/RPM
decisions; this option does not approve the freeze either.

No option is selected implicitly. No integration, ADR status change or canonical
status rewrite was performed during this Phase A stop.

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

## 5. ivac-2zll — preliminary source conclusion, investigation pending

The existing [ShopBot post](../../crates/ivac-cps/posts/shopbot.cps) preflights all
IR sections and rejects unhandled records through `unsupported record <kind>`.
[CpsRecorder](../../crates/ivac-core/src/gcode/cps_recorder.rs) emits `SpindleSpeed`
when RPM/direction changes; the post's record switch has no corresponding case.
The [dual-tool driver](../../crates/ivac-core/src/pipeline/op_drivers/dual_tool.rs)
separates actual finish offsets and establishes a finish-tool envelope; declaration
of a finish tool alone is not proof that this path runs.

Manual mode establishes tool/Z/RPM at section boundaries. Lab spindle-1 mode
requires preloaded tool 1, a flat end mill and integer RPM <=18000, and explicitly
refuses tool changes. Neither fact establishes the exact Pocket combination matrix.
The ticket's 17000→10000 case is historical evidence, not a new reproduction here.
**Do not conclude that all two-tool Pocket operations are unsupported.**

Proposed V1 treatment: preserve deterministic preflight refusal for unsupported
intra-section RPM records; no new OpenSBP spindle behavior. After the integration
decision, reproduce same/different tool and RPM cases, with/without actual finish
offsets, manual versus lab mode and supported tool-change profiles. Check complete
refusal/no partial successful program and positive supported controls. Assess the
current generic diagnostic; propose an actionable bounded diagnostic and tests if
needed. No production change, regression claim or ticket closure occurs now.
Changing emitted commands or machine-control assumptions requires the requested
human HARD STOP before implementation. `ivac-2zll` remains OPEN.

## 6. Proposed capability contract — not freeze approval

| Capability | Proposed V1 disposition / explicit boundary |
|---|---|
| SVG/DXF and units | Supported within qualified import matrix; explicit unit choice, warnings and preserved source intent; no universal CAD compatibility |
| Stock / fixtures / work origin | Limited to accepted single-sided frames and SAFE-Z checks; physical zero/holding remains operator-established; translated/unqualified combinations retain refusal |
| Tools / machine profiles | Project copies, explicit ownership/provenance, actual supported tools; UI availability is not ShopBot capability |
| Feeds & Speeds | Retain FS0–FS3.9, source packs and manual overrides; bounded suggestions, no implied physical validation of every setting |
| Profile / Pocket / Drill | Bounded existing ShopBot tool/entry/geometry matrix; preserve unsupported-case guards |
| V-Carve / inherited operations | Only qualified subsets, explicit limitations; no blanket advanced clearing/Relief/Waterline ShopBot qualification |
| Multi-tool | Supported section/manual-tool combinations only; Pocket rough/finish RPM matrix pending investigation above; lab preloaded mode excludes tool changes |
| OpenSBP | Bundled ShopBot, metric bounded milling and qualified scaffold; no new spindle/ATC/probing/controller semantics |
| Simulation / full-program verifier | Independent current full-program qualification/freshness; playback or successful bytes alone is insufficient |
| Program export | Complete qualified output; native per-file transactional contract, precise cleanup/partial-pair reporting; no pair-atomic/crash/hostile-directory guarantee |
| Legacy projects | Preserve serialized configuration; unsupported custom CPS remains data, not executable fallback |
| CPS trust | Bundled trusted execution only; imported/embedded/path custom execution refused before evaluation |
| Deferred | Real custom-CPS confinement, advanced UI/interactive tool illustrations, stock surface/core colors, 3D naming, advanced two-sided work, ProtoMAX V1.5 and V2+ CAM; retain future roadmap rather than declare complete |

These are proposed classifications to reconcile against [V1 scope](FABCAM_V1_SCOPE.md)
and the actual operation tests after baseline selection, not a newly qualified matrix.

## 7. Qualification evidence and unexecuted work

**No Phase F suite was run for V1-FREEZE**, because Phase A's conflicting-baseline
stop precedes selecting the combined final tree. Historical SEC1 receipt reports
20/20 local gates, 1662 Rust PASS, 1460 frontend PASS, TEST1 256/256 and 136 exact
outputs. Preserve its documented ignored/early-return FANUC-oracle cases and
frontend skip. Those counts are not this checkpoint's results or acceptance of a
new freeze SHA. Rust is pinned 1.88, Node 24, pnpm 10; actual execution versions
must be captured anew with the next run, not inferred from config.

After the integration decision and any approved bounded correction: run workspace
all-features, strict/pedantic Clippy, frontend lint/types/tests/production build,
schema/version/codegen, import, NUMSAFE1/2/3, SEC1 trust, ShopBot M1/M2, SAFE-Z,
V-Carve, provenance/full-program verification, export transactions, TEST1 and
`pre-release.sh`. Compare qualified output corpus/goldens. Record source SHA,
commands, tool versions, logs, skips, failures, output hashes and limitations.
Do not weaken checks to reuse the earlier green result.

## 8. Remote CI and open blockers

GitHub read returned three successful historical candidate runs; latest is
**39125180897**, source `db5bde5d05b25328f29e27d210b7efeb8c0f429a`.
Current SEC1 source is not on origin/main and has no current-source remote
qualification established here. Local workflow fixtures/YAML parsing do not prove
remote execution. Push/workflow dispatch remain unauthorized and pending gates.

Immediate blockers to final freeze endorsement: combined documentation/source
selection; relevant human checkpoint dispositions; exact `ivac-2zll` exclusion/
positive-control decision; final-tree software qualification. Later release gates:
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

Source SHA: **not yet selected**; engineering base proposed above, resulting
integration/qualified SHA must be explicitly approved. Keep current version
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

## 12. Recommendation and decisions

**NO-GO for final functional-freeze approval or candidate builds now.**
**Conditional Go for continued freeze preparation** after selecting an integration
option and recording the required human checkpoint dispositions. This is a bounded
integration/acceptance step, not permission to add features or restart FS0.

## 13. Exact next action and hard stop

Human decision requested: approve **Option A** reconciliation scope (recommended)
or choose Option B and its documentation exception. Preserve the pending reviews;
do not bulk-close them through this choice. Once resolved, continue the authorized
source investigation/integration/qualification phases and return with the exact
qualified SHA and scope for **separate final freeze/candidate-build approval**.
No final candidate SHA can honestly be proposed for approval before that work.

Report checks only: reviewed Git comparison, read-only tracker/GitHub inventory,
source inspection, Markdown relative links/anchors and scoped whitespace/diff.
No integrated software result was invented. **HARD STOP at Phase A integration
options; no merge, push, tag, installer, promotion or hardware action.**
