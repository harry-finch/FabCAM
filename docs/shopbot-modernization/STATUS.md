# FabCAM — current status

Documentation checkpoint: **2026-10-20**, work `ivac-lgudk`; human review **ivac-yg3sm OPEN**.
[Documentation index](../README.md) · [Release plan](../releases/RELEASE_ROADMAP.md) ·
[V1 readiness and gates](../releases/V1_READINESS.md) · [Historical reports](../releases/HISTORY.md).

## Inspected snapshot and active work

- Source baseline: **`94cdd6bbdb5887ffce48adb86b8ea8a088402ac9`**, beta.3,
  **pipeline63** (ADR-0026 import units/warnings commit).
- This documentation branch starts at that revision. Its documentation-only commits
  do not incorporate the primary checkout's unfinished NUMSAFE work.
- **NUMSAFE `ivac-knphk`: paused by the user, unfinished/uncommitted changes present**
  in the primary checkout. No completion or qualification is inferred. Reinspect
  its final source, evidence and human decision before updating this snapshot.
- Current authorized action: documentation consolidation/review only. No new CAM
  milestone, package, push, merge, release or physical trial is authorized here.

## Implemented versus qualified

| Area | Evidence at the inspected snapshot | Acceptance boundary |
|---|---|---|
| Import correctness and explicit units/warnings | [Correctness report](../releases/IMPORT_CORRECTNESS_RESULTS_2026_10_20.md): 38 analytic cases, 53 real files; [units UX report](../releases/IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md): 1452 frontend tests and browser evidence | Implemented in the baseline; human acceptance `ivac-feder` pending |
| Export qualification / full-project replacement | [ADR-0024 results](../releases/UX_SAFETY_LOAD_RESULTS_2026_10_19.md): independent full-program verifier and load isolation; defects `ivac-3xqtb` / `ivac-52tpd` closed in tracker | Human checkpoint `ivac-3i00w` remains open; earlier red evidence preserved |
| Per-move tool provenance | [ADR-0025 implementation](../releases/MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md) and [follow-up](../releases/ADR0025_FOLLOWUP_RESULTS_2026_10_19.md) | Architecture approved, implementation/software evidence recorded; `ivac-b0js3` / `ivac-b0js3.1` still in progress |
| UX A–D | [Consolidation](../releases/UX_V1_CONSOLIDATION_2026_10_19.md), superseded for its two defects by ADR-0024 | A/B/C accepted; D `ivac-he4lt` and consolidation `ivac-amibd` reviews remain open |
| Software release script | [Gate closure](../../V1_RELEASE_GATE_CLOSURE_2026_10_19.md) records 16 gates passing on its stated earlier source | Historical result, not a rerun on this branch or NUMSAFE |
| Desktop candidates | [Beta.3 delivery](../releases/beta3-delivery-2026-10-18/README.md), source `db5bde5`, pipeline61 | Existing builds and bounded macOS smoke; do not contain pipeline63/NUMSAFE |
| Platform and ShopBot release qualification | [Platform/workshop protocol](../releases/beta3-delivery-2026-10-18/PROTOCOLE-V1.md) | Current-source clean installation/runtime and release-level physical qualification not established |

The figures above are cited historical results, not tests run by this documentation
checkpoint. Implementation, software evidence, human acceptance, platform acceptance
and physical qualification are separate states. Bounded operator-reported M4 evidence
remains in [M4 validation](M4_VALIDATION.md); it is not blanket release qualification.

## Release blockers and next action

1. Finish/review NUMSAFE on its own source revision; resynchronize this snapshot.
2. Resolve the [open human gates and conditional reserves](../releases/V1_READINESS.md),
   preserving unsupported/two-sided and machine/post restrictions.
3. Establish combined-source regression evidence before matching candidates;
   installation/runtime, workshop acceptance and publication remain separate gates.
4. Review this documentation checkpoint. Evidence retention/relocation proposals
   require explicit approval; **HARD STOP after documentation**.

The last broad source/candidate acceptance record is [2026-10-18](../releases/ACCEPTANCE_CANDIDATES_2026_10_18.md).
It does not accept later checkpoints. No gate is closed by this documentation edit.
[Original status chronology](history/STATUS_2026_10_20.md) retains all earlier claims,
versions, limits and approvals. M0–M4 names and baseline
`e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1` are preserved.
