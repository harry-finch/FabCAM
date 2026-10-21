# FabCAM roadmap navigation

**Role:** navigate the named engineering tracks and historical milestone decisions.
The single authoritative release plan is [RELEASE_ROADMAP](../releases/RELEASE_ROADMAP.md).
Use [STATUS](STATUS.md) for current work and [V1 readiness](../releases/V1_READINESS.md)
for acceptance criteria, blockers and human gates. This page is not a second diary.

| Track | Authoritative entry | Boundary |
|---|---|---|
| V1 ShopBot milling | [V1 scope](../releases/FABCAM_V1_SCOPE.md) | Bounded operations, output, platform and workshop acceptance |
| FS | [Feeds & Speeds](feeds-speeds/README.md) | Independent milling engine; manual values authoritative; FS4 physical work deferred |
| UX / CAD | [Release priorities](../releases/RELEASE_ROADMAP.md) and [historical checkpoints](../releases/HISTORY.md) | Preserve accepted scope and separate software/human gates |
| V1.1 presets/materials | [Canonical V1.1 milestones](../releases/RELEASE_ROADMAP.md#fabcam-11--presets-and-materials-ux) | PRESET-1/2/3 accepted and integrated locally; MAT-1 unstarted; no push or release |
| WJ / ProtoMAX | [V1.5 scope](../releases/FABCAM_V1_5_SCOPE.md) | Deferred process-specific qualification; no implementation permission from a plan |
| VINYL / advanced workshop | [Future processes](FUTURE_PROCESSES.md) and [release plan](../releases/RELEASE_ROADMAP.md) | Lower-priority/deferred scope and explicit future approvals |

[PRESET integration checkpoint](../releases/PRESET_INTEGRATION_2026_10_21.md):
accepted library, strict Draft copy and unified selector; API `5ca5632`, native
copy guard `bed3a92`, frontend `b8deaef`. MAT-1 remains unstarted.

## Remaining work and dependencies

[Historical V1-FREEZE integration and proposed freeze contract](../releases/V1_FREEZE_READINESS_2026_10_20.md)
retains all human/platform/physical gates; no new functionality is launched.

| Work | Status and dependency | Acceptance boundary |
|---|---|---|
| V1 review and candidate qualification | RC1 `ef807cf` built; retained gates in [V1 readiness](../releases/V1_READINESS.md); combined-source gate before separately authorized matching builds | Human reviews, clean-host platform records and supervised workshop evidence, then explicit V1 acceptance |
| Milling correctness reserves | `ivac-2zll` RPM transition; `ivac-53ky9` resource budget; `ivac-42ylx` cancellation; ADR-0025 two-sided coverage | Triage supported V1 risk before expanding scope; no silent post/geometry change |
| Security follow-up | [SEC1-01–09 matrix and future custom CPS confinement](../releases/SEC1_FOLLOWUP_ROADMAP.md); separate post-V1 security track | V1 bundled-only policy and pending corrections verified before acceptance; no automatic V1.5 dependency |
| Independent UX/data | [Illustration, stock coloring, terminology and existing source-pack guide](../releases/SEC1_FOLLOWUP_ROADMAP.md#independent-ux-and-data-follow-ups) | Planned UX is not a SEC1 blocker; existing guide review reused |
| FS follow-up | FS0–FS3.9 delivered, [acceptance matrix](../releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md#feeds--speeds-milestone-matrix); FS4 deferred/nonblocking; sensitivity smoothing remains proposed | No restart of FS0, automatic physical-evidence promotion or FS4 launch |
| AUDIT3 | After explicit accepted V1, before main V1.5 implementation | Approved time box, retained accepted source/evidence; not a new V1 blocker |
| ProtoMAX V1.5 | Approved [V1.5 plan](../releases/FABCAM_V1_5_SCOPE.md); ARCH-MACH planning and lawful OMX1 evidence before native writer/process work | Native MAKE plus actual platform/physical qualification; no LAYOUT substitution |
| V2+ | [Release plan](../releases/RELEASE_ROADMAP.md): advanced CAM, STEP, smart workflows, optional MCP | Separately approved bounded milestones; no dependency imposed on current V1 |

M0–M4 remain the completed ShopBot foundation, baseline
`e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`; do not renumber them or infer
physical qualification beyond their evidence. Priority remains FS foundation →
priority milling UX/CAM → WJ → VINYL. Planned work is not implemented work.

The [complete previous roadmap](history/ROADMAP_2026_10_20.md) preserves milestone
scope, acceptance criteria, exclusions and chronological authorization records.
Read it for historical decisions, not to select a current milestone automatically.

<a id="future-fabcam--fabsbp-workflow"></a>

Future [FabCAM/FabSBP workflow](FABCAM_FABSBP_WORKFLOW.md) remains an accepted
planning direction with implementation deferred; the historical anchor is retained.
