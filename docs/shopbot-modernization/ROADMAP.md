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
| WJ / ProtoMAX | [V1.5 scope](../releases/FABCAM_V1_5_SCOPE.md) | Deferred process-specific qualification; no implementation permission from a plan |
| VINYL / advanced workshop | [Future processes](FUTURE_PROCESSES.md) and [release plan](../releases/RELEASE_ROADMAP.md) | Lower-priority/deferred scope and explicit future approvals |

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
