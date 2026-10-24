# FabCAM documentation

Current focus: **FabCAM V1.1 — macOS Personal Alpha**, on the maintainer's own
Mac, with supervised ShopBot cases accepted progressively. General fablab/public
distribution, Windows/Linux installers and public signing/notarization are deferred;
canonical safety gates and actual failures remain. [FabApps vision](FABAPPS_VISION.md)
records independent product maturity and future shared design/website initiatives.

Current V1.1 implementation uses explicit operation values and local tested
collections; the integrated recommendation engine is removed.
[QUAL-1A.4](releases/QUAL_1A_4_PRODUCTION_INTEGRATION.md) and
[QUAL-1B](releases/QUAL_1B_COOPERATIVE_CANCELLATION.md) are implemented and qualified
within their recorded software limits. [QUAL-1C](releases/QUAL_1C_MACOS_PERSONAL_ALPHA.md)
built the macOS app; native user acceptance remains pending. SEC-1 corrects Tokio
and consolidates this source. [SEC-1](releases/SEC_1_REPOSITORY_CONSOLIDATION.md)
records 20/20 canonical gates, the rebuilt local app and publication receipt;
[STATUS](shopbot-modernization/STATUS.md) owns current source/limits.
[Release roadmap](releases/RELEASE_ROADMAP.md) separates frozen V1, V1.1
Personal Alpha qualification, planned optional V1.2 FabSpeed integration and deferred CAM work.
[Accepted ADR-0030](shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md)
records separation; [proposed ADR-0031](shopbot-modernization/adr/ADR-0031-optional-one-way-fabspeed-catalogue.md)
records approved one-way product principles and OPEN technical contracts.
No synchronization, release or physical acceptance is implied.

Start with [current STATUS](shopbot-modernization/STATUS.md), then select a route.
Frozen V1 evidence remains in [V1 readiness](releases/V1_READINESS.md). Historical
FS/PRESET reports describe their original sources; they are not current user guides.

## Read by task

| Need | Entry points |
|---|---|
| Understand/install/build FabCAM | [Project README](../README.md), [build guide](BUILDING.md), [Windows](BUILDING_WINDOWS.md), [quickstart](QUICKSTART.md) |
| Contribute or work as an AI agent | [Contributing](CONTRIBUTING.md), [AGENTS](../AGENTS.md), [CLAUDE](../CLAUDE.md); repository skills under `.agents/skills/` |
| Understand architecture and compatibility | [General architecture](ARCHITECTURE.md), [fork-specific contracts](shopbot-modernization/ARCHITECTURE.md), [upstream analysis](shopbot-modernization/UPSTREAM_ANALYSIS.md), [ADRs](shopbot-modernization/adr/README.md) |
| Add CAM, machine or post functionality | [Extension recipes](CONTRIBUTING.md#extension-recipes), [machine guide](development/ADDING_A_MACHINE.md), [machine template](development/MACHINE_INTEGRATION_TEMPLATE.md), [CPS](CPS_POSTS.md) |
| Author workshop cutting data | [Tested preset collections](TESTED_PRESETS.md), [current cutting/API contract](CUTTING_CONTRACT.md) |
| Validate a change | [Testing strategy](shopbot-modernization/TESTING_STRATEGY.md), [CAM validation register](shopbot-modernization/CAM_VALIDATION.md), [TEST1 contracts](releases/TEST1_CONTRACTS.md), [SAFE-Z contracts](releases/SAFE_Z_CONTRACTS.md) |
| Assess V1 / prepare candidates | [Release scope/priorities](releases/RELEASE_ROADMAP.md), [V1 readiness](releases/V1_READINESS.md), [candidate process](releases/DESKTOP_CANDIDATES.md), [version policy](releases/VERSIONING.md), [signing](SIGNING.md) |
| Find a result or qualification artifact | [Historical/report catalogue](releases/HISTORY.md), [local-only evidence register](releases/LOCAL_EVIDENCE_2026_10_19.md), [beta.3 delivery](releases/beta3-delivery-2026-10-18/README.md) |

## Canonical document responsibilities

| Document | Owns | Does not imply |
|---|---|---|
| `README.md` / `README_de.md` | Product entry, build pointers, capabilities, limits and attribution | Current qualification of every advertised inherited operation |
| `shopbot-modernization/STATUS.md` | Current inspected source, active work, blockers, next authorized action | A history of all milestones |
| `releases/RELEASE_ROADMAP.md` | One authoritative release plan, dependencies and deferred scope | Permission to implement future work |
| `releases/FABCAM_V1_SCOPE.md` | V1 scope and acceptance criteria | Evidence that each criterion is satisfied |
| `releases/V1_READINESS.md` | Criterion/gate disposition, evidence and next decision | Human/platform/physical acceptance from tests alone |
| `shopbot-modernization/ROADMAP.md` | Track navigation and historical milestone references | A competing release plan |
| `shopbot-modernization/DEVLOG.md` | Chronological engineering decisions and result summaries | A second current-status source |
| Architecture / ADRs | Stable boundaries, decisions, compatibility and their acceptance history | New approval by an index edit |
| Dated reports / evidence | Exact source, commands, observations, failures and limits | Qualification of newer revisions |

## Navigation and ownership

Contributors update the relevant contract and a dated result together; the milestone
owner records its source/evidence; the human maintainer owns approvals and release
acceptance. Link to authoritative reports rather than copying their full contents.
Use [history](releases/HISTORY.md) when a document has been superseded.

NUMSAFE/NUMSAFE2 are integrated; use the [reconciliation checkpoint](releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md)
for its historical integrated source, bounded Feeds & Speeds acceptance and gates.
Use current STATUS for the post-FS-RESET source; the earlier engine is superseded. No evidence retention policy is
changed by this index; [retention options](releases/DOCUMENTATION_CHECKPOINT_2026_10_20.md)
are proposals requiring approval. Preserve GPL/upstream attribution and separate
manufacturer/local data rights in the [licensing matrix](shopbot-modernization/feeds-speeds/DATA_SOURCES.md).
