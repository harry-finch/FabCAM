# Engineering reports and historical navigation

[Documentation index](../README.md) · [Current STATUS](../shopbot-modernization/STATUS.md) ·
[V1 readiness](V1_READINESS.md) · [Engineering log](../shopbot-modernization/DEVLOG.md).

A dated report is an immutable observation of its named source/configuration. A later
report can supersede its current disposition without erasing its failures, limits or
human-review record. "No commit" / "OPEN" in an old report describes that checkpoint;
consult current readiness and explicit acceptance records before acting.

## Current evidence chains

| Topic | Investigation / original result | Later disposition |
|---|---|---|
| Import correctness and units | [Correctness](IMPORT_CORRECTNESS_RESULTS_2026_10_20.md) | [ADR-0026 UX results](IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md); acceptance pending |
| Export/load safety | [UX consolidation](UX_V1_CONSOLIDATION_2026_10_19.md), [red Stage A](UX_SAFETY_LOAD_STAGE_A_2026_10_19.md) | [ADR-0024 implementation](UX_SAFETY_LOAD_RESULTS_2026_10_19.md); engineering defects closed, human checkpoint separate |
| Tool provenance | [Phase A](MULTI_TOOL_PROVENANCE_PHASE_A_2026_10_19.md), [implementation](MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md) | [Follow-up](ADR0025_FOLLOWUP_RESULTS_2026_10_19.md); do not reuse the initial partial-suite totals as full workspace proof |
| Release-gate tooling | [Environment](../../V1_ENVIRONMENT_RELEASE_GATE_2026_10_19.md), [finalization](../../V1_RELEASE_GATE_FINALIZATION_2026_10_19.md) | [Closure](../../V1_RELEASE_GATE_CLOSURE_2026_10_19.md); historical software gate, not platform/physical acceptance |
| UX A–D | [A](UX_V1_LOT_A_2026_10_19.md), [B](UX_V1_LOT_B_2026_10_19.md), [C](UX_V1_LOT_C_2026_10_19.md), [D](UX_V1_LOT_D_2026_10_19.md) | [Combined review](UX_V1_CONSOLIDATION_2026_10_19.md), then ADR-0024 for its safety/load findings |
| SAFE-Z / TEST1 | [Post-audit](POST_AUDIT_CHECKPOINT_2026_10_18.md), [HATCH/clearance](HATCH_SAFE_Z_RESULTS_2026_10_18.md), [TEST1](TEST1_RESULTS_2026_10_18.md), [extended SAFE-Z](SAFE_Z_EXTENDED_RESULTS_2026_10_18.md) | [ADR-0023 result](ADR0023_PUBLICATION_GUARD_RESULTS_2026_10_18.md), [final consolidation](FINAL_CONSOLIDATION_2026_10_18.md), [acceptance](ACCEPTANCE_CANDIDATES_2026_10_18.md); bounded evidence retained |
| Desktop packages | [Candidate preparation](DESKTOP_CANDIDATES.md), [earlier platform records](desktop-preparation/RESULTS.md) | [Beta.3 delivery](beta3-delivery-2026-10-18/README.md), pipeline61; no current pipeline63 installer inference |

## Historical foundations and detailed milestones

- [M1](../shopbot-modernization/M1_QUALIFICATION.md), [M2](../shopbot-modernization/M2_QUALIFICATION.md),
  [M3](../shopbot-modernization/M3_QUALIFICATION.md), [M4](../shopbot-modernization/M4_VALIDATION.md)
  preserve the completed M0–M4 foundation and bounded software/operator evidence.
- [Fork navigation](../shopbot-modernization/README.md), [CAM validation register](../shopbot-modernization/CAM_VALIDATION.md),
  [FS reports](../shopbot-modernization/feeds-speeds/README.md) and [ADRs](../shopbot-modernization/adr/README.md)
  retain the operation/process-specific contracts and evidence.
- Independent audit reports and their exact fixes remain at their original paths;
  find their chronological links in the preserved snapshots below and the DEVLOG.
  Plans such as AUDIT3, ARCH-MACH, STEP, OMX1 and MCP1 remain plans under the
  [release roadmap](RELEASE_ROADMAP.md), not completed reports.

## Preserved navigation snapshots

These copies retain the previous long-form state documents; only their relative
links were adjusted for the additional directory level. No qualification evidence moved.

- [STATUS before consolidation](../shopbot-modernization/history/STATUS_2026_10_20.md)
- [Detailed roadmap before consolidation](../shopbot-modernization/history/ROADMAP_2026_10_20.md)
- [Release-checkpoint preamble](history/RELEASE_CHECKPOINTS_2026_10_20.md)
- [Old V1 remaining-gates snapshot](history/V1_REMAINING_SNAPSHOT_2026_10_20.md)

Local per-case evidence is catalogued in [LOCAL_EVIDENCE](LOCAL_EVIDENCE_2026_10_19.md).
A clean worktree does not contain those untracked files; absence is not authorization
to delete them from the primary checkout. Proposed retention changes are explicitly
unapproved in the [documentation checkpoint](DOCUMENTATION_CHECKPOINT_2026_10_20.md).
