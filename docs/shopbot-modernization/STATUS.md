# FabCAM — current status

V1-FREEZE integration **2026-10-20**, beta.3 / pipeline63;
[freeze scope, qualification and next decision](../releases/V1_FREEZE_READINESS_2026_10_20.md).
SEC1 baseline:
[implementation, frozen-source qualification and handoff](../releases/SEC1_REMEDIATION_RESULTS_2026_10_20.md).
20/20 local software gates, 1662 reported Rust PASS and 1460 frontend PASS.
Promotion is disabled; remote CI and approval protection have not been exercised.
[Documentation index](../README.md) · [Release plan](../releases/RELEASE_ROADMAP.md) ·
[V1 readiness](../releases/V1_READINESS.md) · [Reconciliation and FS matrix](../releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md).

## Implemented and accepted boundaries

| Area | Current disposition | Qualification / next decision |
|---|---|---|
| SEC1 | Bundled-only CPS, accurate publication/cleanup reporting, shared exact-SHA qualification implemented | [Results](../releases/SEC1_REMEDIATION_RESULTS_2026_10_20.md); human review `ivac-1mqti` OPEN; no remote CI, installer or physical claim |
| NUMSAFE3 | Implemented with approved specialized budgets and transactional program exports; existing two-sided names are refused | [Results](../releases/NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md); 16/16 gates, 1654 Rust tests, 256 TEST1, 136 byte-identical outputs; human review `ivac-zulb3` OPEN |
| NUMSAFE / NUMSAFE2 | Implemented, approved engineering commits `e1a0c95` / `2c6e4e1`; tracker `ivac-knphk` / `ivac-o9gwj` CLOSED | [Numeric/runtime results](../releases/NUMERICAL_RUNTIME_SAFETY_RESULTS_2026_10_20.md), [1000-pass limits](../releases/NUMSAFE2_PASS_LIMITS_RESULTS_2026_10_20.md); retain streaming, cancellation and specialized-loop limits |
| FS0–FS3.9 | Architecture, independent engine, data, CAM integration, ownership, source packs, authoring and bounded adaptation already delivered | [Actual FS matrix](../releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md#feeds--speeds-milestone-matrix): software reviews accepted within recorded scope; FS3 usability superseded by accepted FS3.5; FS4 physical qualification deferred/nonblocking |
| Import / ADR-0026 | `d30863d` / `94cdd6b` implemented; [analytic](../releases/IMPORT_CORRECTNESS_RESULTS_2026_10_20.md) and [units/warnings UX](../releases/IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md) evidence | Human checkpoint `ivac-feder` IN_PROGRESS; commit/push approval does not close it |
| Export/load ADR-0024 | Independent full-program qualification and canonical project replacement implemented | [Results](../releases/UX_SAFETY_LOAD_RESULTS_2026_10_19.md); human review `ivac-3i00w` OPEN; underlying two engineering defects closed |
| Movement provenance ADR-0025 | Approved architecture, implemented with [follow-up corrections](../releases/ADR0025_FOLLOWUP_RESULTS_2026_10_19.md) | `ivac-b0js3` / `ivac-b0js3.1` IN_PROGRESS; back-face fixtures/dowel registration unqualified |
| UX A–D | A/B/C accepted; [consolidated software evidence](../releases/UX_V1_CONSOLIDATION_2026_10_19.md); later export/load defects corrected by ADR-0024 | D `ivac-he4lt` and consolidation `ivac-amibd` human reviews OPEN |
| Documentation | Three completed documentation commits integrated; [source authoring guide](../feeds-speeds/CUSTOM_DATA_SOURCES.md) and loader-validated synthetic packs | `ivac-yg3sm` / `ivac-7w1e1` OPEN; retention remains a proposal, no evidence disposition authorized |

## Software gate and candidates

Integrated `scripts/pre-release.sh`: **exit 0, 16/16 gates PASS** in the qualified
macOS environment; NUMSAFE3 frozen-source Rust **1654 PASS/1 ignored**, frontend
**1458 PASS/1 skip**. See the [NUMSAFE3 receipt](../releases/NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md#final-qualification-receipt).
The earlier integration remains recorded in
[reconciliation](../releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md#integrated-validation).
Historical NUMSAFE2 reports record 16 green gates and 136 byte-identical valid programs;
those observations retain their tested source and do not imply installed-platform acceptance.
The historical NUMSAFE3 run used a scoped `stat` wrapper. SEC1 now fixes the
portable fail-closed size gate and qualifies it without that wrapper; see its report.

Existing [beta.3 candidates](../releases/beta3-delivery-2026-10-18/README.md) are
**`db5bde5` / pipeline61**, with three builds and bounded macOS smoke. They do not
contain later source fixes. Current-source clean-install/runtime qualification on
macOS, Windows11 and Ubuntu, release-level physical ShopBot qualification and V1
acceptance remain pending. [M4 evidence](M4_VALIDATION.md) stays bounded and historical.

## Remaining gates and next action

[SEC1 follow-up matrix](../releases/SEC1_FOLLOWUP_ROADMAP.md) separates locally
qualified SEC1-01/04/08 from deferred hardening and custom CPS confinement.
Option A documentation integration completed on `codex/v1-freeze-readiness`;
final freeze/candidate approval remains pending. The [Pocket RPM matrix](../releases/V1_POCKET_RPM_QUALIFICATION_2026_10_20.md)
retains supported dual-tool manual sections and refuses intra-section RPM changes.

SEC1 engineering (`ivac-dqlol`) is qualified locally; **HARD STOP for human review**
`ivac-1mqti`. NUMSAFE3 review `ivac-zulb3` remains independent. [Resource/export results](../releases/NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md)
record the approved budgets, new-only pair contract, commits and remaining limits.
No push, installer packaging, publication or hardware operation was authorized.

Review [V1 readiness](../releases/V1_READINESS.md), including open human gates,
ShopBot RPM-transition `ivac-2zll`, conditional resource/cancellation reserves and
unsupported/two-sided limits. Then separately authorize matching candidates and
platform/workshop qualification. No functional milestone or release starts here.
FS0 is complete; FS4 remains deferred. V1.5 native ProtoMAX follows accepted V1
and its AUDIT3/OMX1/ARCH-MACH prerequisites; V2+ stays planned.

**HARD STOP after this integration/documentation checkpoint.** No packaging, release,
hardware action or evidence deletion/relocation. The [2026-10-18 acceptance](../releases/ACCEPTANCE_CANDIDATES_2026_10_18.md)
and [historical status](history/STATUS_2026_10_20.md) preserve earlier decisions.
M0–M4 baseline remains `e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`.
