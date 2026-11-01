# FabCAM documentation

## Start here

[STATUS](shopbot-modernization/STATUS.md) is the single current source/provenance,
implementation and blocker summary. It distinguishes accepted security feature source,
older canonical V1.1 and frozen V1. Active documentation describes V10, authored
cutting values and independent validation; the integrated recommendation engine is removed.
[DOC-CONSOLIDATION-1](releases/DOC_CONSOLIDATION_1.md) records this documentation-only
accepted reconciliation and its local consolidation. No merge, installer or release follows from it.

| Goal | Authoritative route |
| --- | --- |
| Start / build | [Quickstart](QUICKSTART.md), [BUILDING](BUILDING.md), [Windows](BUILDING_WINDOWS.md), [CLI V10 examples](CLI_GENERATE.md) |
| Understand current cutting / presets | [CUTTING_CONTRACT](CUTTING_CONTRACT.md), [TESTED_PRESETS](TESTED_PRESETS.md) |
| Understand layers / process boundaries | [General architecture](ARCHITECTURE.md), [fork architecture](shopbot-modernization/ARCHITECTURE.md), [ADRs](shopbot-modernization/adr/README.md) |
| Contribute / use agents | [CONTRIBUTING](CONTRIBUTING.md), [AGENTS](../AGENTS.md), [CLAUDE](../CLAUDE.md), versioned `.agents/skills/` |
| Plan technical work / acceptance | [Technical roadmap](shopbot-modernization/ROADMAP.md), [release gates](releases/RELEASE_ROADMAP.md), [frozen V1 readiness](releases/V1_READINESS.md) |
| Find qualified evidence / chronology | [HISTORY](releases/HISTORY.md), [local evidence](releases/LOCAL_EVIDENCE_2026_10_19.md), [SAFETY-FIX-1](releases/SAFETY_FIX_1_MOTION_GUARDS.md), [SEC-HARDEN-1](releases/SEC_HARDEN_1_APPLICATION_SECURITY.md) |
| Extend / validate | [Extension recipes](CONTRIBUTING.md#extension-recipes), [machine guide](development/ADDING_A_MACHINE.md), [CPS](CPS_POSTS.md), [testing strategy](shopbot-modernization/TESTING_STRATEGY.md), [CAM register](shopbot-modernization/CAM_VALIDATION.md) |

## Canonical document responsibilities

| Document | Owns | Does not imply |
| --- | --- | --- |
| README EN/DE | Product entry, existing capabilities, limits, setup and attribution | Every inherited operation/platform is qualified |
| STATUS | Current implementation, accepted source/provenance, blockers and next human action | A milestone diary |
| Technical ROADMAP | Workstreams and their implemented/deferred status | A second release plan or automatic permission |
| RELEASE_ROADMAP | Release milestones and independent acceptance gates | Permission to merge/build/publish |
| V1_READINESS | Separate historical frozen V1 register and routing to current gates | Current V10 feature status from old OPEN claims |
| Architecture / CUTTING_CONTRACT / TESTED_PRESETS | Stable boundaries, current versions, reader/refusal and authoring semantics | Physical qualification or future integration |
| ADR index / accepted ADRs | Decision navigation and original accepted reasoning | Rewriting history to describe today's architecture |
| HISTORY / DEVLOG | Chronology and links to source-bound reports | Another competing current-state summary |
| Dated reports / receipts | Exact original source, tests, failures, acceptance and limits | Qualification of newer revisions |

## Navigation and ownership

Update each current-state section in place rather than adding milestone-banner
stacks. Link to original reports instead of duplicating them. Preserve accepted ADR
text, upstream attribution, failed/refused cases, private data and local-only evidence.
A clean isolated worktree is not an inventory of every qualification artifact.

Old FS/source-pack/recipe guides are historical; use current cutting/preset contracts.
FabSpeed is independent and optional exchange remains Proposed. No retention,
relocation or deletion is authorized by documentation cleanup; [retention options](releases/DOCUMENTATION_CHECKPOINT_2026_10_20.md)
remain proposals. Software, installed-native, platform and physical acceptance are
separate. English/German are the current frontend locales.
