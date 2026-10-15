> Current2026-10-14: UX5 design accepted by the user (ivac-2zh9), implementation
> authorized and software handoff available in [qualification](../UX5_QUALIFICATION.md).
> Human software gate **ivac-txve** is now the next action. Earlier proposed/
> unimplemented/approval-pending wording below records the original checkpoint.

# UX5 — Action audit and architecture/design checkpoint

2026-10-14 · engineering **ivac-some** · design review **ivac-2zh9**.
User explicitly approved UX4.1 (ivac-si5w) and requested the next milestone.
This completes UX5_PLAN's first checkpoint. **Palette not implemented**; proposed
API/search/UI below require the plan's architecture/design review before coding.
No publication, native keyboard or machining qualification is implied.

## Findings that determine the design

1. **No unified app-command registry exists.** Pure app-menu shortcut decisions
   feed App-local effects. File menu and toolbar bind some services directly;
   operation/CAD/template UI owns local effects. Mutation history is a different
   layer and must not become a second searchable application registry.
2. **Export has two guard layers.** GenerateBar.downloadGcode checks all critical
   pipeline/sim warnings and optional work-area blocking, including back-program
   policy. file_ops.exportGeneratedGcode checks current input identity, idle run,
   same program identity and fixture collision, then rechecks after native Save.
   Calling only the file service would omit some owner policy. Palette export
   must invoke the same protected GenerateBar action as its Download button.
3. **Tool IDs are not enough to merge origins.** Inventory is the editable shop
   store; project tools are stocked/current copies plus legacy local definitions.
   stockTool remaps an ID when a different existing definition holds it. Search
   keys must encode origin. Same-ID/different-definition entries remain distinct.
4. **A saved cutting role is tied to its tool ID.** operationRates returns null
   when appliedCutting exists but lacks a slot for the destination tool. A new
   assignment must not silently fabricate/relabel values or call FS. Direct
   assignment is eligible only when existing role requirements are satisfied;
   otherwise offer the existing operation-parameter review workflow.
5. **Library navigation is not necessarily storage-pure.** ToolLibraryDialog can
   seed an empty inventory from the project on first opening, and owns a draft.
   Search itself must not mount that editor or trigger seeding. Inspecting a
   project-only result cannot blindly focus an unrelated same-ID inventory row.
6. **UX4.1 is ready for navigation integration.** show() and saveOperation(id)
   already open authoritative template workflows; optional initial-template ID
   is a UI-only extension, not template application/rebinding logic in UX5.

## Current action inventory and dispositions

Paths are relative to frontend/src. KEEP means use the existing owner;
EXTEND means expose a thin adapter over it. DEFER means no executable placeholder.

| Action family / existing entry | Authority and context | UX5 disposition |
| --- | --- | --- |
| New / Close — ProjectMenu, App | services/file_ops.resetDocument + App.switchDocument; discard/save confirmation, load serialization, shell reset | EXTEND shared shell binding; preserve confirmations |
| Open / import — toolbar, Welcome, File menu, drop | file_ops.openAny/addDrawing, hidden browser inputs/native picker, document-load commit | KEEP; explicit Import creates/appends via existing flow |
| Save — toolbar, Ctrl/Cmd+S | file_ops.saveProject; actual project snapshot/path | KEEP; active document and no load/run |
| Recent project — menu / Welcome | workspace-session.openRecentProject; discard guard, desktop paths/browser picker | KEEP guarded owner; no direct loadFromPath bypass |
| Clear recents, samples, report, quit | ProjectMenu/workspace-session/ReportDialog | DEFER direct palette mutations; navigation/report may be exposed by existing shell callbacks |
| Undo/Redo — toolbar / shortcuts | project.canUndo/canRedo/undo/redo, history labels | KEEP; suppress while typing; busy guard in dispatcher |
| Add Profile/Pocket/Drill — picker/context menu | project.addOperation, buildOpEntry, effective modes, existing source/tool selection | EXTEND shared action; selection suitability is eligibility, not a geometry computation |
| Other picker kinds | OpKindPicker.ALL_PICKER_KINDS/OP_REQUIRES, capability filtering; standalone Helix omitted | KEEP normal picker; DEFER direct palette entries outside bounded V1 |
| Pocket Outside | OperationsList local wrapper transaction; frame/difference intent | DEFER direct command until local wrapper is shared/qualified; existing picker remains |
| Duplicate/Delete/Reorder — Operations | project.duplicateOperation/removeOperation/reorderOperation, command history | KEEP duplicate/delete selected; no new irreversible-delete policy; DEFER reorder shortcut duplication |
| Group operations / face filters | project.setGroupOpsByTool; list-local filter | KEEP existing controls; DEFER palette shortcuts in V1 |
| Stock / work-origin / fixtures | App accordion reveal; StockPanel updates via existing commands | EXTEND navigation only; no physical zeroing or implicit numeric change |
| Layers / transforms / text / corner relief | LayerList/TextList/CornerReliefPanel and canvas selection | EXTEND pane/CAD workflow reveal when eligible; KEEP draft/preview/Apply/Cancel; no synthetic dogbone operation |
| 2D/3D / fit / playback / warnings | App activePane, canvas/scene owner, GenerateBar warning panel | EXTEND explicit view/warnings callbacks; preserve local fit/playback keys; no new simulation engine |
| Generate / Cancel | GenerateBar.run/cancelRun and generation lifecycle; generateBus.seq is request-only | EXTEND owner binding; no success claim from seq bump; owner retains epoch/abort/progress/error |
| Export machine code / STL | GenerateBar protected Download + file_ops; STL driver separately | EXTEND protected machine-code action; DEFER STL direct palette in V1 |
| Machine / settings / help / About | App mainTab and lazy components; embedded drafts retained | EXTEND navigation through shell callbacks; do not reset drafts |
| Tool search / inspect | workspace.tool_inventory and project.data.tools; ToolLibraryDialog source='inventory' or 'project' | EXTEND read-only search; inventory editor uses its own draft/Apply/Revert; project-only details read-only until proper source-qualified editor handoff |
| Stock/remove machine tool | MachineTooling, stockTool, project.addTool/removeTool | DEFER palette stocking/removal; no automatic stock from search |
| Assign cutting tool | OpPropertiesPanel/PlanarSurfaceSection updateOperation, tool family/mode and saved roles | EXTEND explicit qualified assignment for supported selected ops only; refuse missing slots and route to parameter review |
| Cutting recommendation / preset | FeedsSpeedsSection/Recommendation dialog; preview and explicit Apply | KEEP existing workflow; navigation only if exposed; never call engine from search |
| Operation templates | OperationTemplates show/saveOperation, UX4 instantiation + command history | EXTEND result-to-existing-dialog navigation; no replacement of selected operation, no new template editor |

Palette V1 command set: New/Open/Import/Save/Close/Recent, Undo/Redo,
Profile/Pocket/Drill creation, duplicate/delete selected operation, Stock/Layers/
Text/CAD/settings/machine/tool-library/help navigation, 2D/3D, Generate/Cancel,
Warnings, protected export and approved template opening/capture. Tool search
adds Inspect and a distinct eligible Assign action. Other existing actions stay
available in their ordinary UI; no future feature is advertised as implemented.

## Context matrix — proposed eligibility

Availability is live and distinct from ranking. Absent/disabled actions explain
why. Execution checks the captured target and re-evaluates authority.

| Context | Eligible / useful | Refused / next action |
| --- | --- | --- |
| No document | New, Open, recent, library/settings/help | Save, op mutation, Generate, export: open/create project |
| Empty document | Save/Import/setup/navigation; template dialog | Direct contour creation without suitable source; Generate until supported geometry/text/surface plus enabled work |
| Suitable selected contour | Create Profile/Pocket and template configuration | Tool suitability is still checked; no clearance/physical safety claim |
| Hole/square selection | Drill uses existing tool-picker heuristic | Search itself never selects a physical drill or changes operation |
| Selected operation | Duplicate/Delete, source/template/cutting review; explicit qualified Assign | Missing op or program-only tool assignment; selected target changed while palette open |
| Inventory-only tool | Inspect; open ordinary stocking workflow | Assign before current machine stocks that definition |
| Stocked compatible tool | Inspect; explicit Assign if roles/context permit | Wrong family/effective mode, missing tool/diameter, surfacing Whirl, missing stored cutting role |
| Idle project, stale or no output | Generate, warnings/setup | Export until current generation |
| Running/loading | Relevant progress/Cancel owner, nonmutating navigation as supported | Document/operation/tool mutations and concurrent export/run |
| Current output + policy enabled critical/work-area findings | Show warnings, fix/setup | Owner's exact blocked export policy, front/back independently |
| Native Save dialog open, inputs changed | Existing service rechecks after picker | Old captured output must not be written |
| Different document/selection/tool definition after search | Refresh results and explain changed context | Do not apply old result to new target automatically |
| Another modal / IME composition / editing | Existing dialog retains ownership; composition commits text normally | No nested palette or accidental Enter execution/global typing shortcut |

Do not use project.data.dirty alone as export freshness: saving a modified job
can clear file dirty while generated paths remain stale. Do not serialize a whole
CAM project on each search keystroke; derive compact availability from owners and
leave authoritative generationContextKey rechecks in dispatch/export.

## Minimal adapter contract — proposal, not an implemented API

```text
Localized descriptor {stable id, category, title key, aliases, shortcut hint}
  + transient target {origin, id, document epoch, selected-op id, relevant identity}
  → pure query/ranking over whitelisted metadata
  → live owner availability {enabled, reason}
  → explicit selected action
  → recheck expected target + current availability
  → existing shell/component/service action
  → existing history/persistence or dialog/run lifecycle
```

Suggested owning modules: pure command descriptors/context predicates/search;
small shell binding factory supplied with existing App callbacks; component-owned
Generate/export/warning and template actions register/unregister their callbacks
with teardown. The palette component owns only query, selected result, keyboard
and presentation. Avoid an App god-component callback pile, parallel mutation
history, global success/busy flags or persistence changes. Bind existing menus/
shortcuts to the same actions incrementally; preserve all original typing guards.
A disposed/unavailable owner gives an explicit disabled result, not a queued
command silently replayed on another document. GenerateBar.run remains the single
owner; action completion means dispatched, not machine output certified.

Possible identities: command:project.open, command:operation.create.pocket,
tool:inventory:7, tool:project:7, template:<UUID>. Tool result Enter defaults to
Inspect. Assign is a separately labelled button, never selection-on-highlight.
No assignment to an invented tool and no implicit stocking. Templates open the
UX4.1 binder preselected by stable ID, preserving its geometry/tool decisions.

## Search/ranking specification for design review

- Read-only derived metadata, not a second editable store. Commands: localized
  titles/explicit aliases; tools: name/comment/kind/nominal diameter/flutes/known
  tip angle; templates: name/kind/source diameter. No geometry, cutting evidence,
  knowledge blobs or flattened private data in the index.
- Case-insensitive word/prefix/substring search; normalize diacritics; explicit
  English/French/German command/kind aliases. Bounded single-edit/transposition
  fuzzy fallback for alphabetic terms of at least4 characters. Never fuzzy-match
  numeric diameter, flute count or angle.
- Explicit diameter units: mm and inch/fraction convert to existing internal mm.
  Decimal 3.175 means mm; decimal comma is accepted. Bare integers remain textual
  unless attached to a unit or a known flute/angle term. Display the interpreted
  physical filter. Exact numeric comparison uses a declared small floating-point
  tolerance; it does not select the closest cutter automatically.
- `6mm upcut`: actual nominal diameter6; upcut must occur in real name/comment,
  not inferred from family. `3.175 single flute`: diameter3.175 and flutes1.
  `1/8in`: diameter3.175. `V-bit 60`: known conical type/angle60° required for
  physical matching; absent metadata cannot be certified from a label alone.
- Order tuple: text match quality, explicit context relevance, stable category/
  origin key, stable identity. Availability never makes a poor match appropriate;
  disabled exact matches remain visible/selectable for explanation. Same query,
  locale, context and entries give identical ordering. No recency personalization,
  telemetry, remote service or large dependency.
- Proposed corpus:50 commands/aliases,5000 tool records (inventory/current/local,
  same-ID collisions, absent fields),200 templates;1/8in/mm/comma/flutes/angle,
  prefix/typo/empty/no-result/CJK/long localized titles and tie cases.
  Proposed budget: warm query p95≤50ms on a recorded desktop, cold index≤150ms;
  measure30 runs/query, report environment and max. **Not measured/qualified yet.**

## Keyboard and focus design

Cmd+K / Ctrl+K is provisional: no K shortcut or explicit Rust accelerator was
found in the audited tree; browser/WebView/Windows/Linux conflicts remain runtime
qualification work. Add a discoverable existing-menu action so keyboard conflict
never removes access. Do not steal shortcuts from editable controls or another
modal. Palette search owns typing/Up/Down/Enter/Escape and stops background
handlers; Enter during IME/composition or repeated keydown never dispatches.

Reuse Modal and shared design tokens/Lucide. Input focus stays in search using
listbox active-descendant semantics; disabled rows expose reason and cannot run.
Escape returns to still-existing trigger/workspace focus. Navigation closes the
palette before the destination dialog opens; explicitly transfer focus there
instead of Modal's old-trigger restoration winning a race. Save/discard prompts
and library drafts keep their own lifecycle. Long results scroll, concise footer
labels the selected action; no permanently expanded management controls.
[Concept illustration](UX5-palette-concept.svg) is a design mockup, not a screenshot
or implemented UI. Only two matching example results are shown.

## Validation and implementation sequence after approval

1. Pure descriptors/search/context and injected dispatch: deterministic numeric/
   alias/typo/tie corpus, no mutations from indexing/search/focus, stale-target
   refusal, missing/disposed owner and repeated activation protection.
2. Share existing shell/menu/shortcut action bindings; add palette Modal/input/
   results; validate no document/empty/selected op and geometry contexts.
3. Register protected Generate/export/warnings, then template dialog navigation.
   Prove menu/button/shortcut/palette parity with identical owner callbacks;
   test critical pipeline+sim, work area, front/back, edited→saved→stale, busy
   run/load, document changed during native Save and abort/error lifecycle.
4. Tool search with origin-aware identity/read-only Inspect; explicit compatible
   Assign through current operation update, Undo/Redo, preserved saved roles,
   refused missing role and no engine/stocking side effects. Source-qualified
   editor handoffs must preserve drafts before they are exposed.
5. Actual browser/native keyboard/focus/IME and normal/reduced desktop review,
   disabled/no-result/long labels/large corpus, search latency measurement, full
   frontend and relevant owner regressions, save/reopen. Human UX5 software gate.

Existing seam regressions rerun for this checkpoint: **143 passed in12 files**
(app-menu, tool table/inventory/picker/constraint, feeds-speeds, generation context,
warnings summary, templates/UI helper, document lifecycle and Modal). Full1178
frontend passes/1skip and check/lint/build belong to the accepted UX4.1 baseline;
no new palette functionality/performance/keyboard PASS is claimed. No source code
was changed by this audit; no CAM numerical expectations altered.

## Review decision / next milestone

Approve this bounded adapter/availability/search/focus design at **ivac-2zh9**
before implementation, as required by UX5_PLAN's first checkpoint. The next step
is **UX5 implementation**: command/tool/template search, keyboard navigation and
existing guarded action dispatch, followed by its own human software review.
After UX5, the roadmap lists later milling/V-carve qualification; its scope and
priority remain to be confirmed, not an automatically authorized UX6.
