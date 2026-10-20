# FabCAM release roadmap — approved direction

Approved direction: **2026-10-15**, `ivac-zgmds`; V1 requires macOS, Windows
and Linux. Approval of scope is not release acceptance.

**Role:** authoritative release scope, priorities and dependencies. This plan does
not authorize implementation, publication or a change of qualification status.
For the inspected revision and next action use [STATUS](../shopbot-modernization/STATUS.md);
for outstanding acceptance criteria use [V1 readiness](V1_READINESS.md).
[Earlier checkpoint preamble](history/RELEASE_CHECKPOINTS_2026_10_20.md) and
[historical reports](HISTORY.md) preserve the dated engineering record.

## Executive summary

Make FabCAM 1.0 a practical ShopBot milling application: a small, explicit set of
import-to-OpenSBP workflows, installed desktop use, reliable saved projects and
representative supervised workshop validation. FabCAM 1.5 adds native ProtoMAX OMX preparation replacing LAYOUT, with MAKE
execution, after proving its actual output contract. FabCAM 2.0+ retains advanced
manufacturing work without making it a prerequisite for either release.

Preserve useful existing features. Reduce promises and expansion, not safety checks.
FabCAM prepares machining intent and output; the operator/controller establishes
physical state and executes. Neither release requires FabSBP.

## Current project maturity

This is the planning-time assessment retained from the approved roadmap. Use
[STATUS](../shopbot-modernization/STATUS.md) for the current inspected source and
[V1 readiness](V1_READINESS.md) for live-checkpoint dispositions.

The [current engineering status](../shopbot-modernization/STATUS.md) and dated
qualification records remain authoritative evidence. The old roadmap accumulated
historical “pending” and “not implemented” statements later superseded by continuation
records; they are retained as history, not used as current release status.

| Maturity | Verified repository evidence / consequence |
| --- | --- |
| Implemented, bounded software qualification and continuation accepted | M0–M4 foundation; UX1, CAM2/UX2/UX2.1, CAM3, CAM4 A/B, CAD1, UX3, UX4/4.1, UX5, bounded V-Carve, PERF1 A/B1/B2, UX6 Phase 0/A/B. Acceptance does not broaden each report's matrix. |
| Implemented with open review | GEOM1 A–F engineering complete; ivac-e2jnb open. FS3.6–3.9.1 independent review gates and UX2.2/native/hierarchy gates remain. |
| Partial qualification | Curved rectangular tabs can retain excess stock; medial-axis V-Carve has incomplete area coverage; cancellation target missed; latest integrated desktop installation not qualified. |
| Planned, not implemented as qualified capabilities | GEOM2-A/A1/B, GEOM3, NEST1/2, two-sided registration, paired inlays, waterjet. Inherited code is not release qualification. |
| Physically validated, narrow historical scope | [M4 r2](../shopbot-modernization/M4_VALIDATION.md): operator-confirmed Preview, air cut and MDF square cut, qualitative dimensional/depth conformity and VCarve semantic comparison. No instrument readings or general machine envelope proof. |
| Physically untested additions | Post-M4 CAM/geometry/FS additions have software evidence only. FS4 remains deferred for lab access. Neither current V1 nor ProtoMAX has release-level workshop qualification. |
| Current correction checkpoint | Bounded XOR/Pocket, project-tool, text-reopen and WASM integrity fixes delivered; see stabilization results. Three-platform candidate and workshop qualification remain pending. |

GEOM1's latest recorded checks are 1198 core, 23 focused references overlapping that
suite, 44 CPS and 1312 frontend passes with one existing skip, plus build/schema/lint
checks. These are historical [GEOM1 results](../shopbot-modernization/GEOM1_RESULTS.md),
not tests rerun for this planning task and not a release certificate.

## FabCAM 1.0 — ShopBot Milling

[Scope, operation and acceptance matrices](FABCAM_V1_SCOPE.md) define the proposed
minimum. Required: reliable SVG/DXF, bounded GEOM1 cleanup, single-sided stock/XY
origin with existing Z restrictions, machine/tool copies, manual cutting values and
limited explainable FS assistance, Profile/Pocket/Drill, resolved preview, fail-closed
OpenSBP, persistence, a tested desktop package, docs and workshop evidence.
Keep already delivered surfacing, tabs/leads, dogbones and V-Carve within their
explicit matrices; broader variants are not blanket release blockers.

## V1 acceptance matrix

Use the [V1 matrix](FABCAM_V1_SCOPE.md#v1-acceptance-matrix) as the review sheet.
“Software qualified,” “human accepted” and “physically validated” are independent.
Every advertised supported operation needs release-level evidence; optional features
without it remain explicitly limited/software-only, subject to human scope approval.

## V1 required work

1. Complete/review current independent audit-correction checkpoints and triage unresolved
   supported-scope safety/correctness risks; preserve existing human gates.
2. Establish integrated regression/output evidence on the combined final source tree,
   freezing exact supported operation/platform matrix and revision.
3. Deliver the strictly bounded V1-required [TEST1 property suite](TEST1_PLAN.md), after
   human contract/infrastructure/budget approval; minimize and retain confirmed failures.
4. Consolidate only duplicated geometry primitives with confirmed defects or demonstrated
   remaining V1 ShopBot risk (GEOM-CORE conditional); rerun affected integrated/TEST1/output
   checks after corrections. No broad mechanical refactoring required.
5. Rebuild/test packages from that qualified source, then complete macOS/Windows/Linux
   installation/upgrade/runtime gates and planned physical ShopBot qualification.
6. Freeze RC, complete docs/limits, rerun affected checks and obtain explicit V1 acceptance.
   Publication remains separately authorized; broader consolidation is not a V1 blocker.

This is stabilization, not a new broad architecture program. Existing ADRs remain:
[post](../shopbot-modernization/adr/ADR-0001-shopbot-cps.md),
[FS ownership](../shopbot-modernization/adr/ADR-0003-feeds-speeds-data-ownership.md),
[private sources](../shopbot-modernization/adr/ADR-0005-source-packs-and-cutting-knowledge.md),
[resolved preview](../shopbot-modernization/adr/ADR-0007-shopbot-resolved-preview.md),
[coordinates](../shopbot-modernization/adr/ADR-0008-work-origin-frames.md),
[job intent](../shopbot-modernization/adr/ADR-0009-job-intent-machine-setup.md),
[geometry identity](../shopbot-modernization/adr/ADR-0021-edited-drawing-contract.md).
No parallel geometry engine, hidden rate recalculation, post-specific repair or
serialization change is proposed here.

## V1 conditional features

GEOM2-A1 exact whole-file Width/Height is useful but existing uniform Scale and
external SVG/DXF preparation provide a path. Cost is bounded but nontrivial: edited
bounds, frozen pivot, identity, history and freshness tests are required. Include only
if a demonstrated sizing workflow justifies it. No full GEOM2-A prerequisite.

Existing surfacing, relief, V-Carve and reusable templates are retained. Expanding
these, tightening cancellation below 200 ms or broadening UI polish is conditional
on a measured task-completion/safety problem. Cosmetic completion is not a gate.

## V1 deferred features / classification register

Categories describe release necessity, not implementation effort. Cost labels below
are qualitative remaining work, not estimates or authorization. “Retain” never means
remove existing code. This register covers the existing roadmap families and their
named refinements; detailed historical acceptance criteria remain linked below.

| Existing item | Category | Disposition and rationale | Remaining cost / dependency |
| --- | --- | --- | --- |
| M0–M4 foundation | V1 REQUIRED | Retain; core geometry/post/machine evidence, narrowly scoped physical baseline | Integration + new workshop evidence; medium |
| FS0/FS1/FS2/FS3/FS3.5 | V1 REQUIRED | Retain engine/data/manual-value persistence; usable conservative assistance | Acceptance/persistence review; low–medium |
| FS3.6, FS3.7, FS3.8, FS3.9, FS3.9.1 | V1 REQUIRED | Retain bounded existing UX, provenance, approximate matching, private packs and presets; close relevant reviews, no dataset expansion | Existing human gates; medium verification |
| FS3.9 sensitivity follow-up | EXPERIMENTAL / RESEARCH | Preserve audit; continuity/math changes not required for ordinary workflow | Separate evidence/approval |
| FS4 minimal selected trials | V1 REQUIRED | Required before recommending real ShopBot use of selected settings; software work remains unblocked | Lab/operator; medium, physical |
| FS4 exhaustive coverage | POST-V1.5 | No comprehensive material/tool database requirement | High, ongoing evidence |
| FS5 diagnostic assistant | POST-V1.5 | No necessary role in first complete workflow | New evidence/UX; high |
| UX1 / CAM2 / UX2 / UX2.1 | V1 REQUIRED | Retain entry, single-sided coordinates, fixtures and safeguards | Integrated candidate matrix |
| UX2.2 shell/welcome, Drawings/Layers, preview navigation | V1 REQUIRED | Retain usable new/open/import and clear selection; native review remains | Native/keyboard review; low–medium |
| Hierarchy/cosmetic follow-ups | V1 CONDITIONAL | Keep delivered polish; further changes only for comprehension/accessibility defects | Human review; low |
| CAM3 tabs/leads/direction | V1 REQUIRED | Bounded ordinary Profile combination; no unrestricted topology promise | Workshop matrix; medium |
| CAM4 A/B residual diagnostics | V1 CONDITIONAL | Retain depth-scoped diagnostic; not a proof of complete stock removal | Existing scope sufficient; no expansion |
| CAD1 / CAM4 C dogbones/T-bones | V1 CONDITIONAL | Include limited existing capability; not mandatory to cut ordinary parts | Physical sample if advertised workshop-qualified; medium |
| UX3 surfacing | V1 CONDITIONAL | Keep bounded raster, useful workshop task; not required to surface every stock geometry | Physical coverage/overtravel trial if included; medium |
| UX4 / UX4.1 templates | V1 CONDITIONAL | Retain existing local templates; extensive recipe automation unnecessary | Existing behavior regression; low |
| UX5 palette | V1 REQUIRED | Retain accepted workflow and keyboard access; no expansion | Integrated UI review; low |
| UX6 Phase 0/A/B | V1 REQUIRED | Accepted machine/profile/tool ownership UX; test persistence and identity | Native verification + defect triage |
| V-Carve A1–A4 / final diagnostics | V1 CONDITIONAL | Include bounded groove/spine modes as limited; no arbitrary clearing promise | Text reopen triage + physical sample if advertised; medium |
| Advanced V-Carve / paired inlays | EXPERIMENTAL / RESEARCH | Current coverage partial; paired fit unqualified, defer to 2.0+ | High geometry + physical evidence |
| PERF1-A/B1/B2 | V1 REQUIRED | Retain measured exact-input reuse; representative performance acceptance | RC measurement; medium |
| CPS cancellation improvement | V1 CONDITIONAL | Retain safe result suppression; 200 ms target missed is not automatically unsafe | ivac-nh6q review; stronger boundary may be costly |
| GEOM1 | V1 REQUIRED | Bounded repairs already implemented; common import failures justify inclusion | Human acceptance + integrated checks, no rewrite |
| GEOM2-A1 precise dimensions | V1 CONDITIONAL | Existing scale is usable; include only for demonstrated exact-size need | Low–medium with ADR-0021 compatibility proof |
| GEOM2-A full transforms | POST-V1.5 | Existing whole-file transforms suffice initially | Shared editing audit; medium–high |
| GEOM2-B basic drawing | POST-V1.5 | External CAD/import sufficient | Qualified transforms/source creation; high |
| GEOM3 advanced utilities | POST-V1.5 | Select individually later, not a full CAD program | Per-tool topology proof; high |
| NEST1 manual sheet layout | POST-V1.5 | Existing manual file positioning can suffice for first waterjet workflow | GEOM2-A/instances; medium–high |
| NEST2 true-shape / multi-sheet | POST-V1.5 | Optimization not a first-release requirement | NEST1 + independent constraints/license audit; high |
| Common-line cutting | EXPERIMENTAL / RESEARCH | New process/topology/order risks; not included in initial nesting | High; WJ evidence first |
| Two-sided machining/registration | POST-V1.5 | Keep inherited code explicitly unqualified for ShopBot; single-sided V1 | Separate frames/registration/physical tests; high |
| REST1 — True 2D Rest Machining | V2+ HIGH PRIORITY | Refines existing rest-machining plan; diagnostic is not a safe cleanup operation | Actual residual/depth/engagement/accessibility and separate CAM proof; high |
| OMX1 / WJ1–WJ4 | V1.5 REQUIRED | Native OMX → MAKE → ProtoMAX, no LAYOUT preparation dependency; contract/compatibility unknown | Early OMX1 research/architecture gate then bounded process/lead/bridge/sequence/physical proof |
| VINYL / other processes / broader posts | POST-V1.5 | Retain inherited functions without extending release support claims | Separate process matrices |
| FabSBP job metadata/shared profiles/setup automation | POST-V1.5 | Apps independently useful; controller responsibility stays separate | Explicit cross-project contracts |
| Future measurements / background tracing / advanced automation | POST-V1.5 | Existing selected bounds and external CAD suffice | Individually scoped; no new milestone invented |
| Open correctness/build/persistence issues | TECHNICAL DEBT / DEFECT | Severity by supported-workflow impact, never hidden by feature labels | Triage table in V1 scope |

## FabCAM 1.5 — ProtoMAX Waterjet

[Bounded scope and acceptance matrix](FABCAM_V1_5_SCOPE.md). Reuse process-independent
GEOM1 and positioning; add explicit waterjet intent and required native OMX output
for MAKE, replacing LAYOUT in normal preparation. Technical compatibility remains
unverified; infeasibility is a product/release blocker, not a DXF/LAYOUT fallback. No automatic
nesting, fake milling tools, direct machine control or mandatory FS dependency.

## ProtoMAX early feasibility gate

**OMX1 — Native Format Research & Output Contract** refines the existing bounded
ProtoMAX feasibility documentation/research
checkpoint in the existing WJ track. It can run alongside V1 stabilization, after
separate authorization, without competing with critical ShopBot fixes. Obtain exact
machine/software versions, lawful format references and an actual accepted sample;
assign parameter ownership and prove the transfer/import workflow. Unknown output is
currently a V1.5 blocker; direct OMX is mandatory, not an optional candidate.
No format fields or G-code compatibility may be invented. See [OMX1](OMX1_PLAN.md).
This checkpoint plans that investigation; it does not establish a verified contract.

## V1.5 acceptance matrix

See [the separate waterjet matrix](FABCAM_V1_5_SCOPE.md#v15-acceptance-matrix).
All process/output/hardware gates remain unqualified. ShopBot non-regression is a
mandatory V1.5 gate. Target-software preview is distinct from independent decoding.

## FabCAM 2.0+ roadmap

[Unified workshop priority view](V2_WORKSHOP_PRIORITIES.md) coordinates high-priority
STEP1-A, SMART1, FIT1-A, SMART2 optimization and existing REST1, followed by FIT1-B
and separately scoped projection/recognition. Existing HELIX1 HIGH, VCARVE2, GEOM2/3,
NEST1/2, inlays/two-sided, PERF2 and optional MCP1 retain targets; capacity/order
conflicts require explicit human review. No bundled commitment or V1/V1.5 expansion.


## Dependency graph

```mermaid
flowchart TD
  A[Approved scope: macOS, Windows and Linux] --> B[V1 stabilization and correctness triage]
  G[GEOM1 human acceptance] --> B
  B --> C[Integrated software and output matrix]
  C --> T[Bounded TEST1 and only risk-backed GEOM-CORE]
  T --> D[Rebuilt macOS, Windows and Linux install and upgrade]
  T --> E[ShopBot Workshop Validation plus bounded FS4]
  D --> E
  E --> F[RC regression, docs and human go/no-go]
  F --> R[Separately authorized V1.0 publication]
  F --> H[AUDIT3: time-boxed post-V1-acceptance retrospective]
  H --> Y
  A -. authorized V1.5 planning .-> K[ARCH-MACH boundaries and integration workflow]
  K --> W[OMX1 native format research and output contract]
  W --> X[Human native OMX contract and WJ architecture approval]
  R --> Y[WJ1-3 native OMX model and qualification]
  X --> Y
  G --> Y
  Y --> Z[MAKE acceptance and WJ4 physical qualification]
  Z --> Q[V1.5 RC, ShopBot non-regression and human approval]
```

Solid arrows show prerequisites; AUDIT3 arrows only sequence the post-acceptance
retrospective before main V1.5 implementation and do not block V1 release. Research need not wait for V1
publication, but implementation effort stays behind V1 stabilization. Workshop
failures return to a bounded fix and affected requalification before RC approval.
No arbitrary calendar dates; operator access and output evidence are real dependencies.

## Release readiness / packaging plan

**macOS, Windows and Linux are all V1 REQUIRED**, per human approval on 2026-10-15.
Each requires a usable desktop package and actual install/upgrade/runtime qualification
of the supported ShopBot workflow. Failure on any declared target blocks V1. Existing
macOS evidence is historical; Windows/Linux build infrastructure is not candidate
qualification. Exact OS versions, CPU architectures, Linux distributions and package
formats will be pinned in the next scope checkpoint; this is not support for every
variant. macOS Apple Silicon remains the initial macOS test candidate. No app identity
or version changes here. See the [packaging plan](FABCAM_V1_SCOPE.md#packaging-and-documentation).

## Release-blocker policy and definition of done

**Release blocker:** incorrect/unsafe output, silent geometry/binding or work-origin
changes, corrupted/lost project/tool data, export safeguard bypass, unsupported output
presented as valid, essential workflow impossible, or failed install on a declared
supported platform. An unresolved credible correctness risk blocks the affected
support claim until reproduced and resolved or explicitly excluded with an effective
refusal. A release cannot hide blockers in a generic debt bucket.

**Important but potentially deferrable:** advanced-case explicit refusal, moderate
UX inconvenience, uncommon tool types and performance outside representative loads.
**Future enhancement:** nesting, advanced CAD/simulation, automation and new processes.
Severity follows observed impact and scope, not existing Beads priority alone.

For each release-critical milestone: implementation, targeted independent tests,
relevant regressions, no known unsafe output within scope, persistence checks,
relevant UI review, limits documented, accurate STATUS/ROADMAP and human approval.
Physical milestones additionally require actual operator evidence tied to candidate,
machine/tool/setup and agreed measurements. Software success cannot close them.

With the roadmap approved, additions require an explicit necessity/safety/effort/regression/
V1.5-delay assessment and human scope-change approval. No automatic feature promotion
because it is interesting or nearly finished. The same rule bounds V1.5.

## Proposed roadmap file changes

- `docs/shopbot-modernization/ROADMAP.md`: concise release overview above preserved historical milestone plan.
- `docs/shopbot-modernization/STATUS.md`: current planning checkpoint, unchanged engineering/human gates.
- `docs/shopbot-modernization/DEVLOG.md`: append-only planning record.
- `docs/shopbot-modernization/README.md`: entry links to release review documents.
- `docs/shopbot-modernization/FUTURE_PROCESSES.md`: current WJ release placement above preserved historical assessment.
- `docs/releases/RELEASE_ROADMAP.md`: this proposal, classification, dependencies and approval decisions.
- `docs/releases/FABCAM_V1_SCOPE.md`: milling acceptance, triage, workshop, packaging and documentation.
- `docs/releases/FABCAM_V1_5_SCOPE.md`: waterjet feasibility, parameter ownership and acceptance.

## Existing human gates

Read-only local tracker snapshot 2026-10-15; no gate closed by this planning task.

| Gate | Current disposition |
| --- | --- |
| ivac-e2jnb | GEOM1 software/UX acceptance OPEN; first existing review to complete |
| ivac-tm7u / ivac-7yii | FS3.7 source/licensing and FS3.8 local evidence workflow OPEN |
| ivac-t0fk / ivac-anp0 / ivac-vgu4 | FS3.9 policy, FS3.9.1 and FS3.6 visual reviews OPEN |
| ivac-ein1 / ivac-1drzo | UX2.2/native and hierarchy review OPEN; no cosmetic redesign implied |
| ivac-nh6q | Cancellation improvement/remaining target review OPEN; ivac-q1nz1 investigation deferred |
| FS4, V1 Workshop Validation | Physical work deferred/unperformed for current scope; new release gate proposed, not completed |
| V1.5 ProtoMAX physical / release approval | Proposed future gates; no qualification |

UX5/V-Carve/PERF1 A/B1/B2/UX6 approvals already recorded remain accepted; older
report headers do not reopen them. Earlier source publication authorization does not
authorize a new commit, branch, version bump, release or machine trial here.

## Suggested next milestone

**V1 Scope Freeze & Release-Blocker Triage** has delivered its [handoff](V1_SCOPE_FREEZE_TRIAGE.md).
The next proposed milestone is the bounded stabilization lot: geometry correctness,
tool/text lifecycle and release asset integrity, followed by platform qualification.
GEOM1 human acceptance remains open; implementation needs lot approval.
GEOM2-A1 is not the automatic next engineering step.

## Approval recorded and remaining execution decisions

The user approved the roadmap on 2026-10-15 with one amendment: **Windows and Linux
are required for V1 alongside macOS**. This supersedes the initial macOS-first policy.
The release classifications, bounded capabilities and stabilization direction are
accepted planning; existing software/UX and physical gates are not thereby closed.

The next checkpoint must pin supported OS versions/architectures, Linux distributions
and package formats, operation claims and candidate acceptance evidence. Actual
workshop materials/tools/tolerances/rates remain operator decisions. GEOM2-A1 remains
conditional; no automatic launch of it or WJ research follows this approval.

**HARD STOP — roadmap approved; no next milestone, build campaign, physical trial or
release started by this documentation update.**

## REL1-B revised release gates — approved 2026-10-16

| Milestone | V1 priority | Acceptance / state |
| --- | --- | --- |
| REL1-B1 Community macOS distribution | REQUIRED | Valid ad-hoc FabCAM DMG, verified identity and SHA-256, clean browser-download/install/runtime with documented per-app Open Anyway where needed; local packaging preparation complete; clean-host qualification pending. Packaging, integrity and failed/missing clean-install checks block release. |
| REL1-B2 Developer ID / notarization / stapling | OPTIONAL POST-V1 | Missing membership, Apple identity or ticket does not block V1. No credentials/services configured now. |

GitHub Release distribution and checksum publication require separate explicit approval.
Never disable Gatekeeper or remove quarantine. Windows/Ubuntu test plans remain unchanged.

## AUDIT3 — post-V1 Git history and defect-provenance retrospective

**PLANNED — NOT STARTED · POST-V1 APPROVED RELEASE checkpoint, not a V1 blocker.**
After explicit human acceptance of validated FabCAM 1.0, use its frozen accepted
commit/tag and retained evidence for [AUDIT3](AUDIT3_PLAN.md), before main V1.5
implementation. Existing release/audit identifiers and gates are preserved; current
0.9.0-beta.2 is not the approved V1 historical endpoint. V1.5 ProtoMAX stays the
next product-development priority; authorized early feasibility is not interrupted.

Time box proposed at three engineering days, confirmed at launch. Evidence gaps
remain visible at expiry; extension requires human scheduling approval. Consolidate
Argon/Astra confirmed defects, deduplicate root causes and classify provenance with
historical/test support and confidence. Separate root cause from exposure, counts
from unresolved suggestions, and audit distribution from overall project defect rate.
Technical report, executive summary, charts/tables, engineering retrospective and
unpublished factual public draft require human review and accurate upstream/GPL credit.

Dependencies: accepted V1 reference/approval/evidence, accessible upstream/fork history
and finding/fix/test records. Newly identified critical regressions are separately
triaged/tracked only when authorized, without silently reopening V1 scope. No audit,
Git history investigation, issue/tag/release creation or publication in this roadmap
update. **HARD STOP — awaiting human approval of this addition.**

## Post-audit engineering consolidation — 2026-10-18

**PLANNED — documentation only.** [Policy/backlog](POST_AUDIT_CONSOLIDATION.md).
TEST1 bounded deterministic property subset is V1 REQUIRED; expanded fuzzing is POST-V1.
GEOM-CORE is V1 CONDITIONAL only for confirmed defects or demonstrated remaining
supported-ShopBot risk, with systematic unification deferred. CAM-DIAG and TOOL-TYPES
are POST-V1 (targeted critical bugs separately triaged); PERF2 is V2+/measured performance,
TIME2 POST-V1 and optionally V1.5 if practical without delaying initial workshop gates.

V1.5 ProtoMAX is not contingent on completing every Rust refactor. Post-V1 work enters
that campaign only for demonstrated necessity or separately approved low-risk scope.
AUDIT3 retains its accepted-V1 trigger/time box before main V1.5 implementation. Current
audit reviews, beta/source/package identities and platform/physical gates stay intact.
No measured 4–8× parallel speedup or <5% timing accuracy claim; no implementation,
new dependency, golden change, hardware work, GitHub issue or publication here.

## ShopBot safety / native ProtoMAX / advanced CAM refinement — 2026-10-18

All additions **PLANNED**, not implemented/qualified; current audit reviews, TEST1,
conditional GEOM-CORE, installer/platform/physical gates and AUDIT3 retained.

| Milestone | Release disposition |
| --- | --- |
| SAFE-Z | V1 REQUIRED safety assessment: common-frame stock/work/start/rapid/output and fixture checks; no new Z-zero workflow or unproven top-Z comparison |
| UX8 — ShopBot Setup Sheet | V1 HIGH VALUE / strongly recommended bounded printable intent view; unknown physical state explicit, no large report subsystem |
| TAB-SAFE | V1 CONDITIONAL; confirmed unsafe supported tab-entry case becomes blocker. No automatic rectangular→ramped default change |
| OP-ORDER | V1 CONDITIONAL / V1.5 REQUIRED assessment; actual through-cut/holding dependencies and warnings, not silent reordering |
| OMX1, WJ1–WJ4 | V1.5 CRITICAL/REQUIRED: lawful real OMX contract → explicit process → leads/kerf/pierce → XY bridges/order → native MAKE and physical qualification |
| REST1 / HELIX1 / VCARVE2 | V2+ advanced CAM; REST1/HELIX1 high priority, flat-bottom V-Carve distinct from paired inlays |

[ShopBot scope and acceptance](SHOPBOT_WORKSHOP_ADDITIONS.md),
[native OMX research](OMX1_PLAN.md), [WJ scope/matrix](FABCAM_V1_5_SCOPE.md),
[advanced CAM contracts](ADVANCED_CAM_PLAN.md). Real OMX samples will be user-provided;
none investigated now. No silent downgrade to LAYOUT, no prototype writer, output/
geometry/serialization/artifact change, machine trial, GitHub issue or publication.

## ARCH-MACH — Machine & Process Extensibility Contract

**PLANNED — V1.5 architectural foundation and first practical validation**;
[scope/extension inventory](ARCH_MACH_PLAN.md), [developer guide](../development/ADDING_A_MACHINE.md)
and [reusable specification](../development/MACHINE_INTEGRATION_TEMPLATE.md).
Planning precedes native OMX writer architecture; verified OMX1 evidence then drives
minimal WJ process/adapter implementation and qualification. No generic plugin system
or universal IR rewrite, no V1 blocker absent real supported-ShopBot risk. ProtoMAX
validates the approach only after actual V1.5 software/MAKE/physical evidence.

Same-process machines should normally add qualified capabilities/profile/adapter/
validator/fixtures/docs, not core geometry changes. New processes may need explicit
operations/semantics/UI. WAZER and Graphtec/Silhouette are post-V1.5 conceptual case
studies, not verified protocol support or permission to implement. Proposed local
machine-integration skill links this guide/template; no skill installed now.
Current TEST1/AUDIT3/ShopBot/native OMX product decisions and gates are preserved.

## MCP1 — AI Integration & MCP Server

**PLANNED — POST-V1.5 / FabCAM 2.0+**, optional local integration; no AI/cloud/client
requirement for FabCAM, no V1/ProtoMAX/current-audit/TEST1 delay. [Plan and API inventory](MCP1_PLAN.md).
A architecture/API/state/security audit → B useful read-only inspection/analysis/recommendation
MVP → separately approved C transactional proposal/preview/validate/human-Apply → D guarded
generation/output analysis/export permissions. Candidate local fabcam-mcp/stdio is unfrozen;
remote listener needs separate review. No server/dependency/network/schema/command now.

Reuse authoritative services, source IDs/units/lineage and FS saved-value/privacy contracts;
separate offline documents from desktop-owned unsaved state/history. Capabilities follow
ARCH-MACH qualification, not hardcoded machine claims; ARCH-MACH never depends on MCP.
Explicit project/filesystem scopes and read/write/generate/export authority, revision
conflicts, bounded resources and untrusted-content defenses are future acceptance.
No FabCAM MCP machine control; FabSBP hardware authority is separately designed.
[Developer planning entry](../development/MCP_INTEGRATION.md). Each phase requires human
review; roadmap addition does not authorize implementation or publication.

## STEP and Smart Workshop refinements — 2026-10-19

All **PLANNED — NOT IMPLEMENTED**, subject to human roadmap/scope approval.

| Milestone | Placement / boundary |
| --- | --- |
| STEP1-A — STEP Import & Planar Face Contour Extraction | V2+ HIGH, among first major capabilities; isolated kernel/packaging feasibility gate, explicit accepted local contours then existing CAM, no automatic operations |
| STEP1-B — Advanced Projected Geometry Extraction | Follow A; direction/visibility/depth/overlap filtering; edge Z is context, not automatically operation depth |
| STEP2 — Automatic Machining Feature Recognition | Optional later detected feature→reviewed proposal; never prerequisite for STEP1 |
| SMART1 — Smart Layer-to-Operation Mapping | V2 HIGH: configurable deterministic layer/preset/template proposals, explicit rules/context/tool conflicts and human Apply |
| SMART2 — Smart Tabs & Collision-Aware Leads | Advanced optimization V2; V1 only demonstrated supported-scope safety/defect work through existing TAB-SAFE/audit gates; no default/algorithm replacement from suggestion alone |
| FIT1-A — Fit Calibration Assistant | V2 HIGH preferred first fit deliverable: explicit signed test patterns and measured local provenance, no universal fit value |
| FIT1-B — Targeted Assembly Compensation | Later V2: selected mating geometry, preserved source/preview/approval; fit versus finishing allowance distinct |
| REST1-A/B/C | Refines existing V2 HIGH REST1 into candidate regions, safe paths and guided UX; no direct diagnostic-polygons→Pocket assumption |
| MULTI1 — Unified Multi-Process Workflow | Product objective linked to ARCH-MACH/native OMX foundation V1.5, progressive V2 UX; no rival architecture milestone or silent operation reinterpretation |

[STEP feasibility/identity/qualification](STEP_PLAN.md), [Smart contracts](SMART_WORKSHOP_PLAN.md),
[one proposed priority view](V2_WORKSHOP_PRIORITIES.md). No production dependency/kernel,
viewer, operations, UI/schema/output changes or physical/calibration claims. Current
V1 candidates remain blocked by recorded issues/review; V1.5 native OMX without LAYOUT
and its acceptance requirements remain intact. No machine control/FabSBP dependency.
