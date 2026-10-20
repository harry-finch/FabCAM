> Current2026-10-14: UX5 design accepted by the user (ivac-2zh9), implementation
> authorized and software handoff available in [qualification](../UX5_QUALIFICATION.md).
> Human software gate **ivac-txve** is now the next action. Earlier proposed/
> unimplemented/approval-pending wording below records the original checkpoint.

> 2026-10-14: user accepted UX4.1 and requested UX5 continuation.
> The first action-audit/design checkpoint is complete: [UX5_AUDIT](UX5_AUDIT.md)
> and proposed ADR-0016. Palette remains unimplemented; ivac-2zh9 now reviews this
> concrete design before implementation. Earlier continuation notes are historical.

> Current next action: UX4.1 refinement awaits human review ivac-si5w.
> UX5 remains unimplemented; UX4.1 does not authorize its development.

> 2026-10-14 current continuation: UX3 accepted under ivac-k4z3; UX4 now has a
> bounded software handoff and awaits ivac-51gg. UX5 remains unimplemented and
> requires separate continuation authorization. Earlier sequence notes are historical.

> 2026-10-14 continuation: CAD1 is accepted for the user-requested UX3 launch.
> UX3 is implemented locally and now awaits ivac-k4z3; UX4/UX5 remain unstarted.
> The original planning snapshot below is retained as history.

# UX5 — Command Palette & Quick Actions

2026-10-14 · **PLANNED — NOT IMPLEMENTED**. Documentation record ivac-70y7;
human planning gate **ivac-2zh9**. No UI, command registry or search implementation
is authorized by this plan. No commit/push or subsequent milestone is authorized.

## Placement and product goal

Functional sequence: **CAD1 — Dogbones / T-bones → UX3 — Planar spoilboard
surfacing → UX4 — Reusable operation templates → UX5 — Command Palette & Quick
Actions**. Existing labels are retained. CAD1/CAM4 C is currently an architecture
checkpoint ([audit](../CAD1_AUDIT.md)); UX3/UX4 remain unstarted. UX2.2 shell
implementation and independent review gates are not rewritten or reopened here.
See [roadmap](../ROADMAP.md) and [status](../STATUS.md). Each milestone ends at
human approval; recording UX5 neither advances CAD1 nor starts UX3/UX4/UX5.

Provide a VS Code-style, keyboard-driven local search entry for commands, tools
and eventually templates. Improve discoverability and repeated-workflow speed
without permanent toolbar/menu growth. Ordinary menus, toolbar and contextual
inspectors remain usable; beginners need not learn the palette. Keep it lightweight,
responsive and consistent with the existing professional desktop application.

## Existing action architecture — preliminary source inventory

Before implementation, audit all action entry points, availability, shortcut and
focus ownership; classify KEEP / EXTEND / REPLACE / DEFER. Source presence is not
new behavior qualification. Paths below are repository-relative.

| Seam | Existing responsibility | Proposed disposition |
| --- | --- | --- |
| `frontend/src/lib/state/app-menu.ts`, `frontend/src/App.svelte` | Pure `resolveShortcut` decides actions; App dispatches effects and owns contextual UI state | KEEP typing guards and effects; EXTEND common action access after audit |
| `frontend/src/lib/components/ProjectMenu.svelte`, `WelcomeScreen.svelte` | Shared file actions and shell callbacks, including New/Close | KEEP pathways; add a discoverable palette entry only after design approval |
| `frontend/src/lib/services/file_ops.ts`, `workspace-session.svelte.ts` | Open/import/save/export, recents and unsaved-work guards | KEEP authoritative handlers; no duplicate palette I/O |
| `frontend/src/lib/state/project.svelte.ts`, `project-operation-ops.ts`, `project-machine-ops.ts` | Operation creation/duplicate/update and tool assignment; project mutation methods | KEEP undoable pathways and existing compatibility constraints |
| `frontend/src/lib/state/commands.ts`, `history.ts` | Undoable mutation commands and history | KEEP; distinct from discoverable application-command metadata |
| `frontend/src/lib/components/GenerateBar.svelte`, `frontend/src/lib/state/generate-bus.svelte.ts` | Existing generation owner and request seam | KEEP pipeline owner; audit request completion/error feedback before registry dispatch |
| `frontend/src/lib/state/generation-context.ts`, `generated.svelte.ts` | Actual input identity and generated/pipeline state | KEEP freshness authority; no palette-only ready flag |
| `frontend/src/lib/state/workspace.svelte.ts`, `tool_inventory.ts`, `tool_table.ts`, `tool_picker.ts`, `op_tool_constraint.ts` | Shop inventory, stocked/project tools, table filtering and tool compatibility | KEEP identity/ownership; EXTEND transient search over existing metadata |
| `frontend/src/lib/components/ToolLibraryDialog.svelte`, `Modal.svelte` | Existing tool-library workflow and dialog infrastructure | KEEP drafts/commit semantics; evaluate reuse for focus and compact presentation |

The audited application uses a shortcut resolver, shell dispatch, component-local
effects and mutation history; this plan does not claim a unified CommandRegistry
already exists. The earlier [UX2.2 audit](UX2_2_AUDIT.md) is historical;
its no-New/Close observations are superseded by [implementation evidence](UX2_2_QUALIFICATION.md).

## Proposed CommandRegistry — concept, not a frozen API

Use a thin search/dispatch layer over authoritative application actions. Do not
build an independent hardcoded callback collection in the palette component.
Conceptual descriptor: **id, title, category, keywords, shortcut, context requirements,
availability/disabled reason, execute**. Titles and aliases must remain usable
without knowing IDs. Illustrative identities include `project.new`, `project.open`,
`project.save`, `geometry.import`, `operation.create.profile`,
`operation.create.pocket`, `operation.create.drill`, `operation.create.dogbone`,
`operation.duplicate`, `tool.search`, `tool.assign`, `stock.edit`,
`toolpath.generate` and `toolpath.export`; these are not an implemented/frozen API.

Registry metadata → local search/context ranking → selected command → existing
handler → existing validation/history/persistence. Eventually menus, shortcuts
and palette should share the same action binding and availability contract.
Choose incremental adoption after auditing component-coupled effects; no shell
rewrite or second command execution state machine. An availability check informs
the result, while execution rechecks live authority rather than trusting a cached
search result. Keep async cancellation, busy/error feedback and document/selection
changes within existing action lifecycle ownership. Registry metadata and search
state are runtime concerns, not new project serialization requirements.

## Candidate categories and commands

| Category | Candidate actions |
| --- | --- |
| Project | New, Open, Save, Import SVG/DXF; existing Recent projects access |
| CAM operations | Create Profile/Pocket/Drill; duplicate/delete selected operation; dogbone/T-bone only after CAD1 implementation/qualification |
| Tools | Find/inspect tool, open library, explicitly assign to selected operation, review cutting recommendation |
| Workspace and setup | Edit stock, set declared work origin, switch 2D/3D, show simulation |
| Toolpaths and export | Generate, review warnings, export machine code |
| UX4 templates, future | Search templates, create from template, apply a compatible template to selected operation through UX4 |

These are candidate responsibilities, not a requirement to expose all at once.
Audit the implemented set and existing action names first. Planned functionality
must never appear executable. Work-origin commands edit CAM intent, not physical
zero; cutting review uses existing explicit preview/Apply, not automatic rate changes.

## Context and availability

Read existing project/document/selection/tool/machine/generated state. Cover no
project, loaded-empty project, selected suitable/unsuitable geometry, selected
operation, no/current/stale generated paths, blocking errors and machine/post
incompatibility. Reuse existing suitability checks; do not calculate CAM geometry
in the registry. Pocket needs suitable geometry, assignment needs an appropriate
operation and compatible existing tool, and export needs current output and all
existing policy checks. No-project context prioritizes New/Open/Recent; selected
geometry prioritizes supported creation; an operation prioritizes tool change,
cutting review and duplicate. Relief is eligible only once available.

Prefer visible unavailable implemented commands with a concise, accessible disabled
reason rather than silently hiding everything. Do not enable a command because it
ranks highly. Re-evaluate availability at execution if selection, document, tools,
post or generation state changed while searching. Keep no-project distinct from
new-empty-project, using existing shell document state, not geometry absence alone.

## Tool search and deterministic ranking

Example queries: `6mm upcut`, `3.175 single flute`, `V-bit 60`.
Search the existing authoritative library, not a duplicate database. Current shop
inventory is `workspace.tool_inventory`; the machine/project stocked set is
`project.data.tools`. Preserve same-ID relationships and legacy/project-local tools.
Audit which set each result represents and show stocked/compatibility context.
Searching an unstocked tool must not silently stock it or assign it; reuse an
explicit existing stocking/assignment workflow, or inspect it in the library.

Index reliable existing diameter, type/family, flute count, name and documented
additional attributes. Current `tool_table.ts` filters name/comment/kind by substring
and separately supports diameter/flute sorting; richer numeric/unit/fuzzy matching
is future work, not already implemented. Treat upcut/angle terms only as searchable
where supported by actual metadata/name; never infer a physical attribute or invent
a tool definition from a query. Qualify unit handling, missing metadata and aliases.

Opening/searching/navigating has no project mutation. Without a suitable selected
operation, offer library inspection. With one, explicitly distinguish **Inspect**
from **Assign**, using current validation and undoable assignment. Tool search never
automatically changes the selected operation or cutting values.

Use case-insensitive partial words, common aliases and lightweight fuzzy matching.
Propose deterministic relevance: text quality, explicit context relevance, stable
identity tie-break; finalize exact rules after audit. Same query/context/library/locale
must yield the same ordering. No opaque personalization, behavioral telemetry,
remote backend/AI or large search dependency. A disposable read-only local index may
be derived from authoritative data, but never becomes another editable tool store.
Measure realistic library sizes and select an explicit latency budget/test corpus
at design review; avoid claiming responsiveness from an unmeasured implementation.

## Keyboard and desktop UX

Proposed opening keys: macOS **Cmd+K**, Windows/Linux **Ctrl+K**. `app-menu.ts`
does not currently resolve K, but that alone does not qualify availability: audit
native/webview/browser bindings, other component handlers, editable controls and
existing platform shortcuts before adoption. Select an alternative if needed and
document it; keep an existing-menu or compact discoverable entry available.

Type to search; Up/Down to navigate; Enter to execute the explicitly selected enabled
action; Escape to close. Autofocus search and retain predictable keyboard selection.
Disabled results cannot execute. Stop palette keystrokes from reaching background
selection/shortcut handlers; qualify IME composition and typing guards. On dismissal
restore previous workspace focus; on navigation/another dialog transfer focus to
the meaningful destination, with fallback if the prior element disappeared.

Use canonical [$professional-workshop-ui](../../../.agents/skills/professional-workshop-ui/SKILL.md)
plus [$cam-ux-ui](../../../.agents/skills/cam-ux-ui/SKILL.md).
Review a compact upper-center floating dialog, autofocus field, short scrollable
results, small Lucide icons, concise category/context text, useful shortcut hints,
visible keyboard selection and one-line disabled reasons. Reuse shared tokens and
accessible dialog/focus patterns; no oversized cards, decorative effects or
full-screen alternative navigation. Shared skill rules remain canonical.

## Safety, persistence and technical boundaries

The palette must not bypass project readiness, geometry suitability, machine/post
compatibility, blocking warnings, stale paths, destructive confirmation or
save/overwrite safeguards. Execute via existing handlers; generated output visible
in a preview is not proof of freshness. Edit → Save with old paths remains stale.
Do not weaken warning policy or call CAM/post/FS engines directly from search.

Project modifications use existing Undo/Redo and persistence; navigation/query state
stays transient. Current dialogs' draft/Apply/Cancel semantics remain authoritative.
No CAM computation, operation semantics, tool-management behavior/database, machine
control, postprocessor behavior, project format or new execution state machine.
Any later serialization need requires justification and separate compatibility
approval; this plan adds none. FS4 remains deferred/nonblocking; FS work is unchanged.

## UX4 dependency

UX4 remains its own milestone. Once implemented and approved, expose its search,
compatibility and create/apply handlers: select geometry → palette → find template
→ explicit compatible selection → existing UX4 create/configure workflow.
Template geometry/tool rebinding, saved values, validation and history remain UX4
authority. UX5 does not implement templates. Until then no executable template
commands; the basic command/tool palette does not need a speculative template API.

## Future acceptance and qualification

All criteria are planned; no PASS or software qualification is claimed:

1. Reliable keyboard/menu opening with platform/browser shortcut conflicts resolved.
2. Responsive local search with partial/alias/fuzzy and numeric-tool query coverage;
   measured realistic library performance and deterministic result ordering.
3. Up/Down/Enter/Escape, focus return/transfer, typing and IME behavior qualified.
4. Context-aware command availability for no-project/empty/geometry/operation/output.
5. Unavailable implemented commands give useful accessible feedback and never execute.
6. Menu/shortcut/palette actions share authoritative handlers with no divergent logic.
7. Real tool metadata searches the existing inventory/stocked sets accurately.
8. Tool assignment requires an explicit compatible action; search/inspect do not mutate.
9. Project edits preserve Undo/Redo and save/reopen; searching creates no history.
10. Stale/busy/blocked/incompatible export and destructive/save guards cannot be bypassed,
    including context changes after a result was selected.
11. Approved UX4 templates integrate through existing validation/workflow; if unavailable,
    record integration deferred and expose no executable placeholder.
12. Compact Professional Workshop UI/CAM UX conformity, Lucide/shared tokens.
13. Existing menus, toolbar, inspectors and shortcuts continue to work.
14. Frontend and relevant action/history/tool/generation/file regression tests pass.
15. Accessibility and desktop-size visual review completed with representative captures
    at normal/reduced desktop widths, long/localized labels and disabled/no-result states.

Targeted future tests cover registry dispatch parity, ranking ties/aliases/numeric
queries, nonmutation, actual compatibility, stale/busy guards and changing context.
Review screenshots and native/browser keyboard/focus evidence at the human gate.
No physical-machine qualification is implied.

## Open architecture questions and first checkpoint

Resolve component-local effect exposure without duplicating handlers, progressive
menu/shortcut adoption, one availability contract with execution revalidation,
inventory versus stocked/project-local search, localization/aliases/units,
stocking/assignment destination, dialog handoff, generation async feedback,
shortcut conflicts, UX4's eventual contract and measured performance limits.

First separately authorized checkpoint: exhaustive action/shortcut inventory and
state matrix, proposed minimal registry adapter contract, search/ranking examples,
compact interaction/focus design and test plan. Freeze the detailed schema only
after that audit and human architecture/design approval. Current CAD1 decision
remains the next action; no UX5 development begins here.

**STOP — waiting for human approval before implementing UX5.**
