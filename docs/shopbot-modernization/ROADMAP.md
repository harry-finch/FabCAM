# FabCAM roadmap navigation

**SAFETY-FIX-1 — accepted bounded software-qualified containment; locally consolidated (2026-11-01).**
Canonical V1.1 remains `16eb34fd07354627b3c3c2f48a227b30ba1cc3c2`.
Backend refusals contain unproven helical entry, ineffective two-sided holding /
untransformed Back inputs, and legacy Profile overcut. No schema migration.
[Source-bound qualification and limitations](../releases/SAFETY_FIX_1_MOTION_GUARDS.md).
Software `e09fdca` matches all 1,325 accepted source files.
Three local commits authorized; no functional requalification of Helix, unrestricted
two-sided machining or legacy overcut. **HARD STOP; no canonical integration,
installer, release or physical acceptance.**

The single active plan is [RELEASE_ROADMAP](../releases/RELEASE_ROADMAP.md).
Use [STATUS](STATUS.md) for current source/ownership and [V1 readiness](../releases/V1_READINESS.md)
for frozen V1 gates; dated reports retain source-bound facts and acceptance limits.

| Track | Authoritative entry | Boundary |
|---|---|---|
| Frozen V1 / RC1 | [V1 scope](../releases/FABCAM_V1_SCOPE.md), [readiness](../releases/V1_READINESS.md) | Existing frozen baseline; no inferred physical/publication acceptance |
| V1.1 macOS Personal Alpha | [Qualification path](../releases/RELEASE_ROADMAP.md#v11--explicit-cutting-parameters-and-qualification) | QUAL-1A.4/1B complete; QUAL-1C built; SEC-1 → actual native Personal Alpha walkthrough → separately authorized QUAL-2; public/multiplatform release deferred |
| Optional V1.2 | [INT-0–3](../releases/RELEASE_ROADMAP.md#v12--optional-one-way-fabspeed-integration), [proposed ADR-0031](adr/ADR-0031-optional-one-way-fabspeed-catalogue.md) | One-way FabSpeed → FabCAM; no sync implemented, no V1.1 blocker |
| FabApps portfolio | [Vision](../FABAPPS_VISION.md) | Independent lifecycles; future design principles and website, no shared-package mandate or implementation |
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

<details>
<summary>Historical milestone checkpoints — superseded sequencing</summary>

**INT-ALPHA-1 — accepted bounded software candidate; locally consolidated (2026-11-01).**
Accepted ALPHA-4 `78be459` already inherits ALPHA-1–3.2. TOOL-META descriptive
specifications are ported onto V10 without machining changes or catalogue sync.
109 exact CNC cases, 8 exact WASM cases and 4 exact native/HTTP cases pass.
20/20 mandatory gates PASS; 1662 Rust/Tauri and 1565 frontend tests.
[Source-bound integration report](../releases/INT_ALPHA_1_INTEGRATION.md). Three authorized local commits; no primary integration.
**HARD STOP after local consolidation. Native/platform/physical acceptance remains separate.**

**ALPHA-4 accepted as bounded software-qualified implementation; locally consolidated.**
The 1315-file qualified inventory is unchanged. Selective cross-context values,
advisory provenance, atomic history, V10 persistence and strict matching are accepted.
Stepover remains unsupported; source physical attestation never transfers.
[Report and commit receipt](../releases/ALPHA_4_REVIEWED_PRESET_REUSE.md). Earlier pending statements below are historical.
No merge, push, installer, TOOL-META/FabSpeed integration or physical acceptance.
**HARD STOP after the three authorized local commits.**

**ALPHA-4 — reviewed preset reuse software-qualified; human acceptance pending (2026-10-30).**
Isolated `feature/alpha-4-preset-reuse`, accepted ALPHA-3.2 base `38fc911`.
Separate selective numerical copy into V10 Main / already-enabled Finishing;
strict tested-preset matching stays unchanged. Advisory origin carries no target
physical attestation. 20/20 mandatory gates PASS; 1659 Rust / 1559 frontend tests.
[Source-bound report](../releases/ALPHA_4_REVIEWED_PRESET_REUSE.md).
Earlier ALPHA-4 “planned/not authorized” statements below describe preceding
checkpoints. No commit, integration, installer, release or physical acceptance.
**HARD STOP after this checkpoint for human review.**

**ALPHA-3.2 accepted and locally consolidated.** Software `610d35d6031cb94e4b93473ce1cff661b2b85421`
exactly matches all 1,308 qualified files. Earlier pending-review statements are
historical. Bounded software acceptance only; native/workshop/physical exclusions
remain. No integration or next milestone. **HARD STOP.**

**ALPHA-3.2 — bounded implementation qualified; human acceptance pending (2026-10-30).**
ADR-0033 is approved; historical executable preservation was waived. The isolated
worktree adds explicit Walls / Floor / Both scope, checked axial coverage, reachable
floor proof, curves/simple islands and revision-safe UI. Project/cutting v10 stays;
new `pocket_finishing_v2` and capability are strict. Conservative geometry, cutter,
entry and resource refusals remain. [Report and limitations](../releases/ALPHA_3_2_POCKET_FINISHING.md).
All changes are uncommitted on baseline `81e8d47d3ca6860303d534393bad37056ea0cfb2`.
20/20 mandatory gates PASS; 1,659 Rust / 1,549 frontend tests, 79 real transport
cases and three actual flipped pairs. **HARD STOP for human acceptance.**
No commit, push, merge, installer, release, ALPHA-4 or physical operation.

**ALPHA-3.1 accepted and locally consolidated — 2026-10-30.** Human acceptance
covers the bounded polygon implementation and documented limitations.
Core `59d8e233fa54facb9873b7203068ed45ad291011`; software/tests `56743de13a1f66005d7e24519fc298e66bf9c224`;
all 1,302 qualified software files match the final fingerprint exactly.
[Acceptance receipt](../releases/evidence/alpha31/consolidation.json).
Earlier pending-review/uncommitted statements are historical. ALPHA-3.2 proceeds
under accepted ADR-0033; its current human gate is documented above.
No merge, push, installer, release or physical operation.

Latest isolated software checkpoint:
[ALPHA-3.1 bounded polygon finishing](../releases/ALPHA_3_1_GENERALIZED_FINISHING.md).
20/20 gates pass; convex/concave Profile and simple Pocket finishing are qualified
within explicit proof/strategy limits. Human review and commits are pending;
no integration, native installation, physical acceptance or next-milestone authority.


</details>
