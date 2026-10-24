# FabCAM — current status

**Current focus: FabCAM V1.1 — macOS Personal Alpha**, for the maintainer's own
Mac and controlled ShopBot-focused evaluation. Public release, general university
fablab deployment and Windows/Linux distribution are deferred. Fast development
is distinct from software, personal, physical, workshop and public acceptance.
[Release plan](../releases/RELEASE_ROADMAP.md) · [FabApps vision](../FABAPPS_VISION.md) ·
[Update/evidence report](../releases/MACOS_ALPHA_ROADMAP_UPDATE.md).

| Work | Status and immediate boundary |
|---|---|
| QUAL-1A.4 | IN PROGRESS, `ivac-awe23`, chat “Astra élevé”, isolated `codex/qual-1a4-production`. Provisional uncommitted handoff names `8e2c9ce`, 17/20 gates; no verified closure/primary integration |
| QUAL-1B | PLANNED: correct cooperative preflight cancellation and safe publication; retain 200 ms p95 target, explicit scoped latency disposition only with independent correctness evidence |
| QUAL-1C macOS | PLANNED: actual maintainer-Mac startup, import/libraries, project/preset persistence, explicit parameters, generation/simulation, reports, stale/invalid-export refusal and native permissions/dialogs |
| QUAL-2 / Personal Alpha | PLANNED supervised representative pilot and human scope acceptance; NOT PHYSICALLY QUALIFIED. Software-only evaluation can precede accepted machining cases |
| V1.2 INT-0–3 | PLANNED optional FabSpeed → FabCAM integration; not an alpha blocker. [ADR-0031](adr/ADR-0031-optional-one-way-fabspeed-catalogue.md) remains Proposed; no sync implemented |

FS-RESET/v9/explicit roles, neutral CAM validation and tested local collections
are delivered; old FS/PRESET engine/recipe work is completed/superseded history.
Frozen V1/RC1 stays at `ef807cf`, without new release or physical acceptance.
Historical [QUAL-1](../releases/QUAL_1_SOFTWARE_QUALIFICATION.md) and the provisional
A4 report retain watcher failures, blocked current advisory audit and Chromium/CDP
smoke; none becomes PASS due to the reduced release scope. Historical Test-medial
p95 **200.353 ms > 200 ms** remains open. Actual installed native persistence and
file workflows, SAFE-Z/NUMSAFE/provenance/security/output regressions remain gates.
Classify product defects, baseline failures, environment limits, unexecuted tasks
and explicitly accepted restrictions separately; no safety exception is granted here.

Observed development host: **arm64, macOS 26.5.2**. Confirm intended installation
target and actual installed configuration at QUAL-1C. No inference for other Macs.
General signing/notarization, public packaging and other-platform installers are
deferred unless required by the chosen local installation method. Workshop Beta
and Public Release need later distinct decisions; no immediate public RC objective.

Next: finish the owner-reviewed exact-source QUAL-1A.4 handoff, then separately
authorize QUAL-1B. Plan bounded macOS acceptance and representative pilot fixtures;
software preparation may be accepted while physical use remains excluded.
No hardware, build or installer action is authorized by this documentation update.

Planning lives on isolated `codex/roadmap-fabspeed`, extending documentation commit
`f9316eb`; it is not integrated into primary `FabCAM-v1.1` (inspected `6148b10`).
Primary user edits and A4 source/evidence are untouched. Future application needs
semantic reconciliation with A4's final receipt, not replacement of owner files.
[Prior consolidation](../releases/ROADMAP_CONSOLIDATION.md) and historical snapshots
retain previous decisions; no human gate is closed here. **HARD STOP after docs.**
