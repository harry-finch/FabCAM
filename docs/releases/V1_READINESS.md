# FabCAM V1 readiness

Snapshot **2026-10-20**, committed source **94cdd6bbdb5887ffce48adb86b8ea8a088402ac9**,
beta.3/pipeline63. Primary NUMSAFE work is paused and excluded from this branch.
[STATUS](../shopbot-modernization/STATUS.md) · [Release plan](RELEASE_ROADMAP.md) ·
[Accepted V1 scope/matrix](FABCAM_V1_SCOPE.md) · [Report history](HISTORY.md).

**Not a release acceptance.** Each result belongs to the source/configuration named
in its report. No current-source regression, installed-platform or physical result
is inferred from historical passing tests or from a closed engineering issue.

## Current acceptance and blocking register

Tracker states below were read on 2026-10-20; they are a snapshot, not a live mirror.
The task's human decision must be recorded separately from implementation completion.

| Gate | Current disposition | Required next evidence / decision |
|---|---|---|
| NUMSAFE `ivac-knphk` | In progress in tracker; paused by user, primary changes uncommitted | Claude's final revision, report, regressions, remaining risks and explicit acceptance; do not certify from this branch |
| Import / ADR-0026 `ivac-feder` | In progress; implemented in inspected source, [correctness](IMPORT_CORRECTNESS_RESULTS_2026_10_20.md) / [UX](IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md) evidence recorded | Human UX/architecture acceptance; retain unit/legacy/refusal boundaries |
| Export/load ADR-0024 `ivac-3i00w` | Human review OPEN; two engineering defects closed | Review [results](UX_SAFETY_LOAD_RESULTS_2026_10_19.md), full-program authority and limitations; no return to playback-dependent qualification |
| Provenance ADR-0025 `ivac-b0js3` / `ivac-b0js3.1` | In progress; [implementation](MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md) and [follow-up](ADR0025_FOLLOWUP_RESULTS_2026_10_19.md) recorded | Human checkpoint review; back-face fixture/dowel registration remains explicitly unqualified |
| UX D `ivac-he4lt` / consolidation `ivac-amibd` | Separate human reviews OPEN | Review [D](UX_V1_LOT_D_2026_10_19.md) / [consolidation](UX_V1_CONSOLIDATION_2026_10_19.md) with the later ADR-0024 corrections, not as two still-unfixed defects |
| Large pocket `ivac-53ky9` / cancellation `ivac-42ylx` | Conditional reserves OPEN | Representative packaged-runtime measurements; no unlimited-size or universal200ms guarantee; freezes/OOM/wrong output/partial success remain blocking observations |
| Combined-source software gate | Earlier [release script closure](../../V1_RELEASE_GATE_CLOSURE_2026_10_19.md) is historical | After NUMSAFE, qualify exact combined source with relevant regressions/output/schema/build gates, retain optional-oracle skips and incomplete runs |
| Candidate freshness / CI | Existing [beta.3 artifacts](beta3-delivery-2026-10-18/README.md): db5bde5/pipeline61 | Separately authorized matching builds and integrity records after source/human gates; no new candidates created here |
| Clean installation / runtime | Current-source macOS/Windows11/Ubuntu qualification not established here | Real clean-download/install/upgrade/runtime records, actual OS versions and data preservation; CI build is not desktop acceptance |
| ShopBot workshop / physical | No current release-level physical qualification inferred | [Operator protocol](beta3-delivery-2026-10-18/PROTOCOLE-V1.md), controller preview, supervised air/material tests and measurements; retain bounded M4 evidence |
| Documentation review `ivac-yg3sm` | This consolidation awaits human review | Review source snapshot, navigation and proposed retention options; no evidence disposition implicitly accepted |
| RC / V1 / publication | No release or publication authorization | Exact supported matrix, reviewed evidence/limits, explicit RC/V1 decision and separate publication approval |

## Closed earlier checkpoints, retained limits

Tracker `ivac-l6ouh`, `ivac-4z6lh`, `ivac-j09eh` and `ivac-8371y` are CLOSED in this
snapshot. Their [final consolidation](FINAL_CONSOLIDATION_2026_10_18.md),
[TEST1](TEST1_RESULTS_2026_10_18.md), [SAFE-Z](SAFE_Z_EXTENDED_RESULTS_2026_10_18.md)
and [publication guard](ADR0023_PUBLICATION_GUARD_RESULTS_2026_10_18.md) retain their
bounded domains, failed/resource cases and qualification limits. Do not reopen them
merely by copying an old OPEN banner, or extend them to later source automatically.
The [2026-10-18 acceptance record](ACCEPTANCE_CANDIDATES_2026_10_18.md) is preserved.

## Deferred and conditional scope

Developer ID/notarization, AUDIT3, FS4, PERF2, TIME2, broader GEOM-CORE, ProtoMAX/WJ,
VINYL and unsupported advanced/two-sided variants retain their existing status in
[the release plan](RELEASE_ROADMAP.md). UX8 is bounded/conditional. Earlier GEOM1/FS
human records must be reconciled with actual decisions; no blanket closure here.

After NUMSAFE: refresh revision and report links, reconcile gates, rerun appropriate
combined-source checks, then seek candidate authorization. Until then **HARD STOP
at this documentation checkpoint**. Evidence retention/relocation needs its own approval.
