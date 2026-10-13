# UX2.2 — Application Shell & Welcome Experience

Date: 2026-10-13. **DESIGN CHECKPOINT COMPLETE — not implemented.**
User selected UX2.2 as the next development milestone. Source audit, two shell
alternatives and concrete lifecycle scope are in [UX2_2_AUDIT.md](UX2_2_AUDIT.md)
(ivac-7737). Historical placement discussion below is preserved; current choice
advances the design checkpoint without closing earlier CAM4/FS human reviews.
Planning record ivac-3267; human planning gate **ivac-dwdd**. This document
does not authorize UX2.2 implementation, another CAM3 run or later milestones.

## Placement and existing decisions

Requested conceptual sequence: UX2.1 — Direct Manipulation & Collision Feedback
→ UX2.2 — Application Shell & Welcome Experience → CAM3 — Tabs / Leads.
This conflicts with the actual roadmap: UX2.1 continuation was approved
(ivac-svnf closed), CAM3 already has bounded software qualification and approved
continuation (ivac-wh2m closed), and CAM4 A/B is software-qualified awaiting
human gate ivac-tvox. See [roadmap](../ROADMAP.md),
[UX2.1 continuation record](UX2_1_QUALIFICATION.md) and
[CAM3 qualification](../CAM3_QUALIFICATION.md). These facts do not establish
blanket CAM or physical completion.

Keep UX2.2 beside UX2.1 in the UX planning taxonomy, without retroactively making
it a prerequisite of CAM3. Proposed execution placement: after current CAM4 A/B
and FS3.9 handoff reviews, as the next shell/design checkpoint before separately
approved later milling expansion. Human approval must resolve that placement.
The current CAM4 and FS3.9 gates remain independent; FS4 stays DEFERRED and
nonblocking. No FS3.9 redesign or sensitivity-policy implementation is included.

## Goal and boundaries

Improve first launch, project entry/switching, main toolbar discoverability,
project status and visual coherence within the existing desktop application.
The canvas/preview remains the dominant work area. Scope is the application shell,
welcome experience and directly related navigation/project-state presentation.

Excluded: full frontend/workspace rewrite, every-inspector overhaul, FS3.9 redesign,
FS engine/data/evidence changes, CAM algorithms, persistence semantics or project
format migration, machine setup engine, ShopBot post changes and FabSBP integration.
Existing file/import/export semantics stay authoritative. Any missing lifecycle
capability or behavioral change needs an explicit scoped approval before coding.

## Initial source audit and ownership

This is a read-only source inventory, not desktop UX qualification. Start eventual
work with a complete command/location/shortcut/enabled-condition audit, including
native menus, keyboard handlers, workspace controls and mobile/browser fallback.
Classify each responsibility KEEP / EXTEND / REPLACE / DEFER before layout decisions.

| Existing seam | Verified responsibility | Planned disposition |
| --- | --- | --- |
| `frontend/src/App.svelte` | Project/Machine/Tools/Settings/Help/About tabs; desktop Open, Recent, Save, Report, Undo/Redo, GenerateBar and 2D/3D controls | EXTEND shell composition after auditing usage and duplication |
| `frontend/src/lib/components/GenerateBar.svelte` | Generate, Download/export, progress, stale-context and warning presentation | KEEP execution/gating; review compact placement and preview/simulation access |
| `frontend/src/lib/services/file_ops.ts` | Open/save/import/sample/export flows; `confirmDiscardIfDirty`; export freshness checks | KEEP shared handlers and semantics across entry points |
| `frontend/src/lib/state/project.svelte.ts` | Project data, history, `hasUnsavedWork`, generated-state access | KEEP authority; audit distinction between no project and deliberately new empty project |
| `frontend/src/lib/state/generated.svelte.ts`, `generation-context.ts` | Generated output and runtime identity of actual CAM/post inputs | KEEP generation authority; no independent header readiness model |
| `frontend/src/lib/state/workspace.svelte.ts`, `workspace.ts` | Recent projects, last project and installation/view state; browser localStorage and Tauri app-data workspace file | KEEP local storage; EXTEND welcome presentation only |
| `frontend/src/lib/components/RecentMenu.svelte` | Names/path tooltips, open callback and clear-list action | KEEP capability; evaluate shared presentation and missing-file recovery |

Recent projects already exist; UX2.2 is not a new recent-project persistence system.
Audit startup/reopen behavior in `frontend/src/lib/services/workspace-session.svelte.ts`
before defining first launch. File path or geometry absence alone must not be assumed
to distinguish no-project from new-empty-project lifecycle. Resolve that question
without a shadow project store or an unapproved format/behavior change. No ADR is
required for this plan; record one if later approval changes an ownership boundary.

## Top toolbar review objectives

Candidate responsibilities are an audit checklist, not a requirement to add buttons:

- Project: New, Open, Save/Save As, Import geometry, name and unsaved indication.
- Editing: Undo/Redo and actions relevant to the active workspace context.
- Machining: Generate toolpaths, preview/simulation access and machine-code export.
- Application: Settings, Help/documentation and relevant contextual diagnostics.

Evaluate global application/project commands, contextual workspace commands and
primary job actions separately. Final grouping follows actual usage frequency and
context, with frequently used actions directly accessible and uncommon commands
in existing menus/disclosure. Avoid duplicate functionality, a large ribbon,
oversized SaaS header or putting every action in the bar. Audit existing name/status
locations before moving them. Preserve shortcut consistency and drawing space.

Canonical visual language: repository-local
[$professional-workshop-ui](../../../.agents/skills/professional-workshop-ui/SKILL.md)
plus [$cam-ux-ui](../../../../.agents/skills/cam-ux-ui/SKILL.md).
Use those rules rather than copying a new token/style specification into this plan.
The review must check compact spacing, restrained dark desktop surfaces, coherent
type, small consistent Lucide icons (existing `@lucide/svelte`), subtle semantic
accents, accessible focus/tab order, tooltips and names for icon-only controls,
primary/secondary hierarchy and understandable disabled/loading/error feedback.
Disabled explanations must be reachable without relying on hover over a disabled
button. Minimize modals and stay consistent with existing panels; no Adobe branding.

## Project status and export readiness

Show concise state and a useful next action, with details in existing diagnostics.
Candidate states: no project; project loaded; unsaved modifications; toolpaths not
generated; generated/current; generated/stale after editing; export blocked by
authoritative errors or current export policy. Loading/generating/failure states
must remain understandable without turning the header into a diagnostics dashboard.

Reuse `hasUnsavedWork` for discard protection and the appropriate existing dirty
and path state for labels. Disk-save status and generated-path validity are different.
Reuse generated result, `inputContextKey`, `generationContextKey`, pipeline state
and current warning/export policy; do not introduce new status calculations or a
UI-only source of truth. An old visible preview never proves export readiness.
Saving cannot make edited toolpaths current. Current output is not physical machine
readiness or clearance certification. Unsupported/unavailable states stay explicit.

## Welcome and distinct workspace empty states

Conceptual welcome: **FabCAM**, Create a new project, Open an existing project,
Import an SVG or DXF, then a compact Recent projects list. Optional example project
and documentation/getting started links remain secondary. Map each action to audited
handlers; confirm New and Close semantics before exposing a new entry point.

Use a clear, attractive compact desktop entry surface: no decorative large cards,
marketing, statistics, complex onboarding questionnaires, oversized navigation,
redundant buttons or online-dashboard layout. A tutorial is not mandatory.

| State | Intended presentation |
| --- | --- |
| A. No project loaded | Welcome; clear New/Open/Import and local recents |
| B. New project, no imported geometry | Normal workspace; contextual invitation to import |
| C. Geometry loaded, no operations | Keep drawing visible; concise operation/setup next step |
| D. Operations loaded, no current toolpaths | Keep workspace visible; generate/regenerate action and accurate reason |

Ordinary edits, deleted geometry, stale paths and failed generation must not send
users back to welcome. Helpful optional guidance may show Import geometry → Configure
stock/tool → Create operation → Generate → Preview → Export. It must not force a
linear workflow or remove experienced users' direct access to commands.

Recents reuse platform-local workspace storage. Show clear names and useful file
context, one-action open, actionable missing/moved-file feedback and optionally
remove an obsolete entry. Audit existing clear-list behavior versus per-entry removal
before approving an extension. Browser entries may lack reusable native file access;
explain the fallback rather than promising access the platform cannot provide.
No cloud, telemetry or remote indexing, and no implementation in this checkpoint.

## Transition and persistence qualification

Future qualification must cover startup without a project (including existing reopen
policy), opening, creating, importing, closing, returning to welcome, opening another
project/recent/example, unsaved work, cancelling dialogs and failed/unsupported imports.

Reuse the existing Save & continue / Don't save / Cancel guard for destructive
transitions. Save failure or cancellation must abort the requested transition;
discard requires deliberate selection. Include a newly imported drawing never saved
as a project, not just `dirty` projects. Never silently discard work. Cancelling a
picker keeps the prior context; failed imports explain recovery without silently
destroying that context. Preserve additive versus replacement import semantics.
If the audit exposes an existing destructive failure, propose the repair separately
before changing semantics. Keep project history, serialization, recent-list storage
and generation validity in their existing ownership boundaries.

## Future acceptance and evidence

All criteria below are **planned**, with no implementation pass claimed:

1. Coherent no-project welcome on first launch and approved return-to-welcome flow.
2. New, Open and Import discoverable through audited, shared action entry points.
3. A/B/C/D states distinguished; normal editing retains the workspace.
4. Header actions logically grouped, nonredundant and canvas space preserved.
5. Project identity and unsaved work clearly indicated, including never-saved imports.
6. Switching/closing cannot silently discard work; save/cancel/failure paths qualified.
7. Generation/export state reflects authoritative context and policy; edit → Save
   with old preview remains stale and unavailable for export until regeneration.
8. Disabled/loading/error states explain the reason and available recovery.
9. Keyboard access, focus, shortcuts and accessible action names preserved.
10. Professional Workshop UI and CAM UX conformity; Lucide and shared tokens reused.
11. Desktop layouts reviewed at 1280×800, 1440×900 and a reduced 1024×768 window,
    including long names, translated text and display scaling; no lost primary action.
12. Existing load/save/Save As/import/preview/export behavior remains functional,
    including supported native desktop and browser fallback paths.

Eventual evidence: targeted automated state/transition/unsaved-work/readiness tests,
desktop-size UX review, representative welcome and workspace screenshots (empty,
loaded, dirty, stale, blocked), and human review before any next milestone.
Extend existing file, workspace, history and generation-context tests rather than
mirroring CSS. No physical qualification or new machine/post evidence is implied.

## Dependencies, open questions and recommended checkpoint

Existing file/history/workspace/generation seams and shared UI skills are dependencies;
FS4, FabSBP and a new CAM algorithm are not. Resolve ordering against current CAM4
and FS3.9 reviews; exact no-project lifecycle and New/Close behavior; startup/reopen
policy; native/browser recent-file recovery; command relocation/overflow and preview
access; and reachable explanations for disabled controls.

Recommended first authorized checkpoint: command/location/shortcut audit, source-backed
A/B/C/D state and transition map, two compact shell/welcome design alternatives,
and a precise test/compatibility plan. Obtain human design/scope approval before
UI implementation, especially for any new lifecycle action. Subsequent bounded
implementation must end with automated evidence, desktop screenshots and human review.

**Update 2026-10-13:** user approved Atelier with the active machine in the top bar.
ivac-dwdd is closed; implementation ivac-j6ba is complete locally. The planning
criteria above are retained as the original specification; current evidence and
remaining native/human checks are in [UX2_2_QUALIFICATION.md](UX2_2_QUALIFICATION.md).
Human review gate: ivac-ein1.
