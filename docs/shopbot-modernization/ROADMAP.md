# FabCAM roadmap navigation

The single active plan is [RELEASE_ROADMAP](../releases/RELEASE_ROADMAP.md).
Use [STATUS](STATUS.md) for current source/ownership and [V1 readiness](../releases/V1_READINESS.md)
for frozen V1 gates; dated reports retain source-bound facts and acceptance limits.

| Track | Authoritative entry | Boundary |
|---|---|---|
| Frozen V1 / RC1 | [V1 scope](../releases/FABCAM_V1_SCOPE.md), [readiness](../releases/V1_READINESS.md) | Existing frozen baseline; no inferred physical/publication acceptance |
| V1.1 stabilization | [Qualification path](../releases/RELEASE_ROADMAP.md#v11--explicit-cutting-parameters-and-qualification) | QUAL-1A.4 IN PROGRESS; QUAL-1B/1C/2 and RC planned, final-source evidence and human decisions required |
| Optional V1.2 | [INT-0–3](../releases/RELEASE_ROADMAP.md#v12--optional-one-way-fabspeed-integration), [proposed ADR-0031](adr/ADR-0031-optional-one-way-fabspeed-catalogue.md) | One-way FabSpeed → FabCAM; no sync implemented, no V1.1 blocker |
| Later CAM / WJ / VINYL | [Deferred backlog](../releases/RELEASE_ROADMAP.md#later-cam-enhancements) | Separate architecture/process/software/platform/physical approvals; no automatic launch |
| Historical FS/PRESET | [Disposition ledger](../releases/ROADMAP_CONSOLIDATION.md#milestone-disposition) | Completed engineering subsequently superseded by ADR-0030; no restart or migration requirement |

M0–M4 remain the completed ShopBot foundation, baseline
`e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`; retain bounded physical evidence.
[Historical roadmap](history/ROADMAP_2026_10_20.md) and
[pre-consolidation release roadmap](../releases/history/RELEASE_ROADMAP_BEFORE_FABSPEED_CONSOLIDATION.md)
retain earlier scope/decisions; they are not active implementation instructions.

<a id="future-fabcam--fabsbp-workflow"></a>

[FabCAM/FabSBP planning](FABCAM_FABSBP_WORKFLOW.md) remains deferred and independently
reviewed. FabSpeed one-way integration does not authorize controller communication.
