# FabCAM — current status

Documentation consolidation inspected `FabCAM-v1.1` at
`6148b1059ab6020f751069d5c3a7d64b8047fb6b`. Work is isolated on
`codex/roadmap-fabspeed`; primary user edits and QUAL-1A.4 are preserved.
[Release roadmap](../releases/RELEASE_ROADMAP.md) ·
[Source/ownership and evidence inventory](../releases/ROADMAP_CONSOLIDATION.md).

| Line / milestone | Status | Evidence / immediate boundary |
|---|---|---|
| V1 / RC1 | HISTORICAL frozen baseline `ef807cf`; no release or physical acceptance inferred | [V1 readiness](../releases/V1_READINESS.md); frozen refs unchanged |
| V1.1 FS-RESET-1/2/3, v9, explicit-input CLI | COMPLETED engineering; final acceptance BLOCKED by remaining gates | [ADR-0030](adr/ADR-0030-fabcam-fabspeed-separation.md), [removal results](../releases/FS_RESET_3_RESULTS.md). Explicit actual-role parameters, tested collections, independent CAM validation; no recommendation engine |
| QUAL-1 | COMPLETED investigation / bounded polling correction | [Source `558efc4` receipt](../releases/QUAL_1_SOFTWARE_QUALIFICATION.md): 17/20 mandatory gates, not unconditional qualification; historical V-Carve/cancellation gaps retained |
| QUAL-1A.4 | IN PROGRESS, `ivac-awe23`, chat “Astra élevé”, isolated `codex/qual-1a4-production` | Observed commits `c74f5e9`, `61c00ba`; no verified final report/integration at inspection. Prototype results do not accept production |
| QUAL-1B / QUAL-1C / QUAL-2 / RC V1.1 | PLANNED; physical ShopBot NOT QUALIFIED | Cancellation preflight, full software/host/file/platform gates, supervised acceptance, then candidate/human release decisions |
| V1.2 INT-0–3 | PLANNED; no synchronization implemented | [Proposed ADR-0031](adr/ADR-0031-optional-one-way-fabspeed-catalogue.md); approved one-way ownership, transport/storage OPEN; not V1.1 blockers |

Known qualification gaps: unrestricted-host native watcher tests, current advisory
audit, unmodified Chromium smoke, actual browser project/collection transfers,
installed Tauri dialogs/storage and macOS/Windows/Linux runtimes. Historical
ShopBot Test-medial cancellation p95 **200.353 ms** exceeds the unchanged 200 ms
target. QUAL-1A.4 must bind final arithmetic, JSON parsing, strict P1 and motion
results to actual integrated source; no stale prototype acceptance is assumed.
No new software tests/builds ran for this documentation-only task.

FabCAM owns explicit operation values, local independent resources and tested
collections, simulation/generation/validation/export. FabSpeed owns calculation,
experiments and its catalogue/validation. Optional future sync is FabSpeed → FabCAM
only; no CAM payload upload, automatic application or live runtime dependency.
FS/PRESET historical engineering is not incomplete because it was intentionally
superseded; retained MAT-1 presentation is not a new task.

Next: let QUAL-1A.4's owner finish its exact-source receipt and human handoff.
Separately authorize QUAL-1B, followed by QUAL-1C/QUAL-2 and reviewed RC V1.1.
INT-0 documentation may be studied separately in parallel; synchronization code
starts only after contract review on an appropriately qualified future branch.
The isolated documentation commit requires semantic reconciliation with any later
QUAL-1A.4 status changes; it is not automatically integrated into primary/main.

[Historical status and preserved user notes](history/STATUS_BEFORE_FABSPEED_CONSOLIDATION.md)
retain old checkpoints. No human gate is closed by this task.
**HARD STOP after documentation; no software, push, merge, release or hardware.**
