> Accepted by the user on 2026-10-14 (ivac-si5w). UX5 audit/design continuation
> authorized; the recorded desktop/native/physical evidence limits remain.

# UX4.1 — Operation Templates & Sidebar UX Refinement

2026-10-14 · engineering **ivac-01ux** · human review **ivac-si5w**.
Human review requested a focused UX correction after the initial UX4 handoff.
The old review ivac-51gg is superseded by this combined review, not accepted as
physical/native qualification. Local uncommitted work; **STOP before UX5**.

## Existing UX issues identified

Initial dialog mixed capture, library management and reuse; Model/Template terms
competed. Operations used five header children in a four-column grid, placing
its count away from its title and sometimes wrapping Add. Header-contained modal
clicks also bubbled into Operations' section toggle. Scroll ownership was split
between active hosts and component wrappers, letting section context disappear.

## Template dialog changes

Reuse is the main workflow: local substring search, compact selectable rows,
operation type and source cutter diameter, destination controls, then Create
operation. Native buttons expose pressed selection and work with Enter/Space.
A matching current canvas selection is proposed as geometry; tools stay explicit.
Empty library/no results/no selected operation have specific guidance. English
UI consistently says Template; serialized types and fields retain their names.

## Save/Rename/Delete workflow

Save as template… is contextual to the selected operation; Save current… is a
secondary library action. A dedicated view names the source, prefills the name
and explains retained settings/excluded bindings. New UUID capture preserves
existing duplicate-name/update policies. Per-template details menus expose rename
and delete. Delete opens a confirmation naming the template and explaining that
created operations stay unchanged; Cancel/Escape returns to the library. Confirmed
removal retains the existing in-dialog Undo. No library migration or import/sync.
The modal is a header sibling, so its actions cannot collapse the sidebar.
Creation reveals Operations and selects the new operation without replacing one.

## Template preview and compatibility handling

Expandable preview reads actual stored settings: applicable strategy/offset,
depths/list/step, pocket overlap or surface stepover, entry, directions/leads,
finish step/allowance and stored cutting roles. Missing values are not invented.
One concise review statement covers destination tool/material/machine. Known
rough/finish diameter differences get a compact amber explanation; same diameter
is not presented as certified suitability. No automatic rate/DOC recalculation.

Create is disabled until the same instantiateTemplate validator accepts current
bindings; its refusal explains the next missing/invalid input. Apply revalidates
and uses addOperationCommand, preserving history, stale state and generation
warnings/export guards. Workspace, template version 1, job v3/v6, machine/tool
compatibility and core/FS/ShopBot semantics are unchanged. In a small desktop,
the dialog footer remains visible when its preview scrolls.

## Sidebar scrolling architecture

The existing accordion has one active top-level pane (or none). Collapsed sibling
headers remain naturally reachable. Its active Stock/Layers/Text/Operations host
owns the vertical scrollport; intermediate component wrappers no longer add a
second scrolling/hidden boundary within the main sidebar. Other layouts retain
their existing wrappers. Operations maintains one selected inline inspector.

## Sticky-header implementation

Shared sidebar-section-head uses CSS sticky, top 0, existing opaque group-header
surface and z-anchor token, bounded by its own section. Stock directly owns its
padded scrollport; its existing padding token offsets the sticky top so no field
bleeds above it. No duplicate header layer, permanent unrelated stack or sticky
operation rows. Switching the accordion moves the context naturally; collapsing
all sections preserves normal headers. Active regions support native keyboard
scrolling with visible focus, without changing a numeric input's value.

## Operation-counter placement and semantics

Shared inline identity groups title/count, with actions on the right. Operations
reads project.data.operations.length directly: disabled entries and all sides
count; templates, generated paths and selected/enabled counts do not.
Text uses textLayers.length. Layers retains its prior domain context: usable
layer count for one drawing, import count for multiple drawings; segment/layer
stats remain in the count tooltip. Stock retains dimensions rather than a count.
No separate counter state. Add, Templates and Group ops by tool remain available.

## Shared components reused

Mandatory [Professional Workshop UI](../../.agents/skills/professional-workshop-ui/SKILL.md),
CAM UX/UI, architecture, CAM validation and project documentation skills applied.
Existing Modal/focus handling, inspector spacing/actions, app.css group-head,
design/stacking tokens and Lucide icons reused. New TemplateParameterPreview is
small and presentation-only; operation-template-ui.ts owns pure search, display
rows and validation feedback. No UI framework or UX5 registry introduced.

## Tests and regressions

**1178 frontend tests pass, 1 optional skip; 128 test files pass.** Six new UI-helper
cases cover local search/order/empty results, authoritative refusals including
stock/finish roles, differing diameter with unchanged manual values, stored-only
preview and irrelevant-field exclusion. All eleven UX4 persistence/manual/history
cases, existing sidebar accordion, lifecycle, Modal, i18n and size guards pass.
Svelte/TypeScript: zero errors/warnings. ESLint/Prettier and production frontend /
WASM build pass. Existing chunk/import build warnings remain. No Rust/CAM changes
or new physical/native evidence; no numerical expectations relaxed.

```sh
npm --prefix frontend run check
npm --prefix frontend run lint
npm --prefix frontend test
npm --prefix frontend run build
```

Actual browser verification (default 1280×720; additional 1100×660):

| Cases | Observed proof |
| --- | --- |
| Browse/search/select/empty/no source | Existing UX4 entries work unchanged; POCKET 12 filters locally; Enter selects; empty-origin library explains capture and disables Save current/Create |
| Save/rename/delete | Context save captures selected T7 operation; rename persists; delete Cancel preserves it, confirmed delete leaves job unchanged; Undo restores; reload retains renamed/captured entries |
| Binding and review | Missing tool/geometry disable Create; current valid canvas selection is proposed; Ø12→Ø8 difference shown; validator refuses unsupported/invalid contexts in tests |
| Values/history/job persistence | Created T7/object1 operation retains 10000/1000/500 and -2/-1; actual v3 download/reopen retains four ops including disabled Profile, old job name unchanged |
| Counter mutations | Create 3→4, Undo 4→3, Redo 3→4; duplicate 4→5, delete 5→4, Undo 4→5, Redo 5→4; group toggle keeps4; different project32 and reopened job4 |
| Generation/export | Normal .sbp includes the new operation, 10000 RPM and 16.666667 mm/s feed; feed edit marks stale and disables export; no override bypass |
| Scroll/sections | Expanded depth and finish controls; end of long inspector; 32 rows with16 disabled; switching selected31/32; Layers24 with File transform; Text12 with editor; Stock; collapse/expand and no stacking |
| Layout/focus | Sidebar resized280→390→280; small desktop keeps header/count/actions within265px content width (scrollWidth=clientWidth); native Home/End/PageDown; contextual menu Enter/Escape and Modal Tab/Shift+Tab trap |

Measured from visible DOM: Operations header matches scrollport top after
scroll919.5/1293.5 and dense scroll1573/2126.5; Layers header matches at scroll414;
Text at scroll133. Stock's padding correction removes its top gap. Small-preview
Create ends at y593 within660px. Browser viewport override and sidebar width were
restored. These are UI measurements, not controller/machining qualification.

[Actual created/saved project](UX4_1-software-created-saved.ivac-project.json),
[dense UI-only fixture](UX4_1-software-sidebar.ivac-project.json) (32 operations,
24 overlapping layer rectangles, 12 text entries; not a machining setup).

## Before/after screenshots

| Required view | Screenshot |
| --- | --- |
| Before: combined UX4 dialog | [Before](ux/UX4_1-before-templates.png) |
| Before: separated count / initial sidebar | [UX4 baseline](ux/UX4-pocket-preview.png) |
| A — existing local library | [Library](ux/UX4_1-A-library.png) |
| B — empty library / no operation selected | [Empty](ux/UX4_1-B-empty-library.png) |
| C — selected template and target choices | [Targets](ux/UX4_1-C-targets.png) |
| D — saved parameter preview | [Preview](ux/UX4_1-D-preview.png) |
| E — contextual rename completed | [Renamed](ux/UX4_1-E-renamed.png) |
| F — known cutter diameter difference | [Review](ux/UX4_1-F-diameter-review.png) |
| G — total count next to Operations | [Count](ux/UX4_1-G-operation-count.png) |
| H — long inspector after scrolling | [Scrolled](ux/UX4_1-H-scrolled-operation.png) |
| I — transition to Layers | [Layers](ux/UX4_1-I-layer-transition.png) |
| J — smaller desktop | [1100×660](ux/UX4_1-J-small-desktop.png) |

Additional: [save view](ux/UX4_1-save-template.png),
[delete confirmation](ux/UX4_1-delete-confirmation.png),
[many operations](ux/UX4_1-many-operations.png), [Text](ux/UX4_1-text-sticky.png),
[Stock](ux/UX4_1-stock-sticky.png), [small scrolling preview](ux/UX4_1-small-preview.png).
All are actual application captures. Early A–E document the tested interactions;
F/J/small-preview show the final diameter icon/footer and small-desktop correction.

## Documentation updated

STATUS, ROADMAP, DEVLOG, architecture/UX notes, UX4 qualification continuation and
navigation now identify UX4.1 and its current human review. ADR-0015 semantics
remain valid; no new model/serialization decision required.

## Remaining issues / human review

No known blocker in the scoped desktop browser workflow. The established model
opens one operation inspector at a time: multiple selected/expanded operation
states were not added. Many-operation switching and several expanded subsections
of the selected inspector were tested instead. Full native bundles and mobile
layouts are not newly qualified; physical suitability remains a separate review.
Template types remain the bounded UX4 V1 set. UX5 is unimplemented.

Review [the local session](http://127.0.0.1:18767/?api=http%3A%2F%2F127.0.0.1%3A18772),
source fixture, targets/preview, contextual save/rename/delete, and scrolling dense
fixture. **STOP — awaiting human review ivac-si5w.** No commit/push/merge/release.
