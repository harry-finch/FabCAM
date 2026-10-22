# FabCAM documentation

Current V1.1 architecture review: [FS-RESET-1](releases/FS_RESET_ARCHITECTURE.md) —
explicit operation cutting parameters and tested presets, independent FabSpeed calculation.
[ADR-0030](shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md) is proposed;
**HARD STOP before implementation**.


Start with [current STATUS](shopbot-modernization/STATUS.md), then select a route.
This index separates product setup, engineering contracts, plans and evidence.
Source/qualification snapshot: [V1 readiness](releases/V1_READINESS.md).
Active branch: [v1.1 UX changes and verification](releases/FABCAM_V1_1_UX_2026_10_21.md),
[PRESET-1 read-only library checkpoint](releases/PRESET_1_READONLY_2026_10_21.md)
and [PRESET-2 strict-copy checkpoint](releases/PRESET_2_DUPLICATE_ADAPT_2026_10_21.md).

PRESET-1/2/3 are accepted and integrated locally on `FabCAM-v1.1`: library, strict
Draft copying and the [unified selector](releases/FABCAM_PRESET3_AUDIT_AND_IMPLEMENTATION.md).
[Integration commits, checks and remaining boundaries](releases/PRESET_INTEGRATION_2026_10_21.md).
No push or release; MAT-1 remains unstarted.

## Read by task

| Need | Entry points |
|---|---|
| Understand/install/build FabCAM | [Project README](../README.md), [build guide](BUILDING.md), [Windows](BUILDING_WINDOWS.md), [quickstart](QUICKSTART.md) |
| Contribute or work as an AI agent | [Contributing](CONTRIBUTING.md), [AGENTS](../AGENTS.md), [CLAUDE](../CLAUDE.md); repository skills under `.agents/skills/` |
| Understand architecture and compatibility | [General architecture](ARCHITECTURE.md), [fork-specific contracts](shopbot-modernization/ARCHITECTURE.md), [upstream analysis](shopbot-modernization/UPSTREAM_ANALYSIS.md), [ADRs](shopbot-modernization/adr/README.md) |
| Add CAM, machine or post functionality | [Extension recipes](CONTRIBUTING.md#extension-recipes), [machine guide](development/ADDING_A_MACHINE.md), [machine template](development/MACHINE_INTEGRATION_TEMPLATE.md), [CPS](CPS_POSTS.md) |
| Author workshop cutting data | [Custom Feeds & Speeds sources](feeds-speeds/CUSTOM_DATA_SOURCES.md), [synthetic examples](feeds-speeds/examples/README.md) |
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
for current source, Feeds & Speeds acceptance and remaining gates. No evidence retention policy is
changed by this index; [retention options](releases/DOCUMENTATION_CHECKPOINT_2026_10_20.md)
are proposals requiring approval. Preserve GPL/upstream attribution and separate
manufacturer/local data rights in the [licensing matrix](shopbot-modernization/feeds-speeds/DATA_SOURCES.md).
