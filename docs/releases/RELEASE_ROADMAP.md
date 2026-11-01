# FabCAM release milestones and acceptance gates

## Current state and authority

The accepted security feature line contains the integrated ALPHA-1–4/TOOL-META
software and subsequent SAFETY-FIX-1/SEC-HARDEN-1 containment. It is not merged
into canonical V1.1. [STATUS](../shopbot-modernization/STATUS.md) owns exact source
identity and qualification provenance; [technical roadmap](../shopbot-modernization/ROADMAP.md)
owns workstreams; [HISTORY](HISTORY.md) owns source-bound chronology. Replace this
current-state section when scope changes; do not prepend milestone banners.

The immediate product target remains a limited **V1.1 macOS Personal Alpha**.
Software acceptance is bounded and does not qualify installed-native file workflows,
authenticated remote HTTP access or physical machining. Documentation reconciliation
is accepted for local documentation commits only. No merge, installer, publication or machine operation
is authorized by this roadmap.

## Release lines

| Line | State / purpose | Gate before advancement |
| --- | --- | --- |
| Frozen V1 | Separate historical source/candidate line, with its original contracts | Follow [V1_READINESS](V1_READINESS.md) and the original source-bound records; do not project V10/security acceptance backwards |
| V1.1 software | ALPHA-1–4, compatible TOOL-META and motion/security containment accepted on feature lineage | Explicit canonical integration decision; verify ancestry, complete dependencies and qualified source identity |
| V1.1 personal native evaluation | Existing historical builds do not represent the latest security source automatically | Separately authorized matching build, isolated profile, install/start/open/save/reopen/watch/export and data preservation checks |
| Supervised ShopBot evaluation | No current release-level physical qualification | Controller review, actual spindle/tool/stock/WCS/holding verification and supervised air/material trials with measured results |
| Workshop Beta / public V1.1 | Deferred beyond personal alpha | Defined supported matrix, unresolved safety/security dispositions, platform/install/upgrade checks, release-engineering integrity and explicit publication approval |
| Optional V1.2 | FabSpeed exchange direction only | [ADR-0031](../shopbot-modernization/adr/ADR-0031-optional-one-way-fabspeed-catalogue.md) remains Proposed; INT-0 review before transport/storage implementation |
| Broader CAM / WJ / VINYL | Deferred tracks | Separate scope, architecture, budget/proof and machine acceptance decisions |

## Independent acceptance gates

1. **Source identity:** select the actual committed feature lineage, not an old
   installer, a branch label or uncommitted work from another checkout.
2. **Software:** retain exact reports, failed/refused cases and compatibility limits.
   The canonical 20-group gate is required for qualifying executable changes;
   documentation-only edits use focused checks and executable blob equivalence.
3. **Human scope acceptance:** accepted bounded milestones do not authorize
   functional expansion or remove their documented exclusions.
4. **Installed native/platform:** mock IPC and browser checks do not establish
   real native dialogs, persistence, file grants, signing, upgrades or OS acceptance.
   macOS, Windows/Linux, mobile and web each keep their own verified scope.
5. **Physical:** software output does not prove actual machine setup or cutting
   safety. Historical M4 observations remain limited to their original experiment.
6. **Release/publication:** independently authorize integration, candidate creation,
   signing/distribution and publication; preserve licensing and upstream attribution.

## Open exclusions and blockers

Preserve SAFETY-FIX-1 refusal of unproven helical entry, legacy Profile overcut and
affected two-sided holding/Back configurations. Preserve scoped-finishing geometry,
entry/cutter/coverage budgets and same-tool ShopBot RPM-transition limitations.
Complete geometry repair, universal finishing, rest machining and stepover reuse
are not implemented claims. Native installed acceptance, authenticated remote HTTP
and physical machining remain unqualified. Parser/HTTP bounds are containment,
not authentication or universal memory/latency/preemption guarantees.

Detailed original findings, including security dispositions and historical frozen
V1 human gates, stay in [V1_READINESS](V1_READINESS.md),
[SEC1 follow-up](SEC1_FOLLOWUP_ROADMAP.md),
[SAFETY-FIX-1](SAFETY_FIX_1_MOTION_GUARDS.md) and
[SEC-HARDEN-1](SEC_HARDEN_1_APPLICATION_SECURITY.md). Older OPEN/uncommitted/planned
claims describe their snapshots; they do not revoke later explicitly accepted work.

## Deferred policy and evidence

Public/multiplatform distribution, broader hardware support, authenticated remote
service mode, optional FabSpeed integration, performance campaigns and speculative
CAM/process extensions need separate authorization. No current feature requires a
live calculator. Evidence deletion, relocation or retention changes require their
own approval; ignored/untracked data are not disposable merely because absent from
an isolated worktree. Preserve existing archives and qualification binaries.
