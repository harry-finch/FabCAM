# Drawings and layers clarification

2026-10-15 · focused continuation, engineering ivac-yjcv2 complete.
**Human visual review ivac-ena7m pending — STOP.** No new major milestone.

## Pre-implementation architecture audit

Recorded before UI edits. `LayerList.svelte` reads `project.data.imports`:
each ImportEntry keeps the source ImportResponse and its own FileTransform.
The old header is inconsistent: multiple imports show Drawings/import count;
one import shows its filename and a usable-layer count; empty shows Layers/zero.
The new presentation must consistently identify the import-document count.

`source.segments.length` counts imported segment records, not contours or source
CAD entities. Nonempty layer metadata (`segment_count > 0`) is aggregated by exact
layer name across imports, in first-seen order, summing counts and retaining the
first source layer's color. `combineImports` applies the same layer-name union
while namespacing object IDs; this UI refinement must not change those rules.
SVG imports use the synthetic `svg` layer, flattening SVG groups; DXF retains
its parser-supplied layer names. Two SVG files can therefore contribute 8 + 4 = 12
segments to the same `svg` layer. Empty source-layer metadata remains excluded
from the usable layer list as before.

The per-row checkbox calls `toggleLayer`, which only replaces `visibleLayers`.
2D/3D imported-geometry display and 2D hit-testing consult it. Existing selected
IDs are not cleared; new picks/box selection exclude hidden imported layers.
`buildGenerationProject`/`buildProject` use the full transformed/augmented geometry
and operation source references, never `visibleLayers`. `generationContextKey`
also excludes this view state. Thus hiding a layer does not disable its CAM
operations, invalidate output or remove its existing source selection. The old
row tooltip claiming pipeline inclusion is **incorrect copy**, not behavior.
Generated toolpaths have their separate display controls.

The header checkbox loops through all usable layer names using `toggleLayer`;
it is not a per-drawing enablement control. Mixed states currently appear
unchecked; checking reveals all, unchecking hides all. A mixed indication may
describe that same state, without changing the existing action.

The small swatch has no independent action or color editor. It is a span inside
the old checkbox label, so clicking it indirectly toggles that checkbox; it does
not edit color. The new eye target makes that activation explicit, while the
swatch/name are no longer a checkbox-label hit area.
Its current local ACI table supports 1–8/256 but misses ACI 9, unlike the shared
canvas/3D resolver. Reuse `canvas/aci-color.ts` for presentation rather than
inventing a second palette. A merged-layer swatch represents the retained first
layer metadata color; individual segment colors may differ. No renderer or
stored-color change is authorized.

Visibility is runtime/workspace view state, outside history and file dirtiness.
`snapshotProject` writes empty legacy `visibleLayers`/`selectedEntities` arrays;
per-project desktop workspace state owns visibility on reopen. Restore accepts
nonempty legacy arrays; empty is no opinion, and reopening with all hidden may
reveal all. A browser-downloaded project alone therefore does not carry hidden
view state. Preserve this behavior rather than silently migrating the format.

Removing an imported file is an undoable imports command and clears generated
output; orphaned visibility names are pruned outside history. Removing a layer
deletes all matching imported segment records across every source, filters the
parallel object mapping and layer metadata, recomputes source bounds, and uses
an undoable imports transaction. Visibility pruning itself is not undone.
Neither action has a confirmation dialog in this component. Retain action
availability, confirmation behavior and CAM freshness checks; clarify destructive
scope in its tooltip. File-transform controls remain unchanged.

Classification: KEEP import/state/history/CAM/persistence/rendering architecture;
EXTEND compact presentation and existing color resolver reuse. No core/schema
or project migration, no numerical change, no architecture ADR required.

## UI changes and control decisions

- **Drawings always counts imported documents**, including one/zero/empty imports.
  Every file has its own filename/statistics row and unchanged File transform
  foldout. An Imported files heading distinguishes these records from layers.
  File removal remains available only for multiple imports, as before.
- A compact **Layers / count** separator and the visible reminder **Hidden layers
  are still used by CAM** make the second concept and its scope explicit.
- Per-layer native buttons use existing Lucide Eye/EyeOff icons, `aria-pressed`,
  show/hide labels including the layer name, accurate view/picking-only tooltips,
  24px targets and the existing keyboard-focus ring. The existing checkbox in
  the header still controls all imported layers, with corrected label/tooltip
  and a native mixed indication. Its all/none action is unchanged.
- Read-only swatches are filled 14 px rectangles with a subtle outline, accessible
  ACI metadata and an explicit read-only/drawing-palette tooltip. They do not
  enter the tab order or open a color picker. Shared `resolveAci`/`hexToCss`
  now also show ACI 9 gray correctly; renderer and stored colors are unchanged.
- Exact layer-name provenance appears in the name's tooltip. Only a layer backed
  by multiple imports says Shared layer and lists its contributing filenames.
  Color remains the first contributing layer's metadata color; this is not a
  promise that every segment on that aggregated layer has that same color.
- Counts say segments/layer(s), with singular forms. At narrow content widths,
  file statistics move below the filename and layer counts abbreviate to seg.
  Names stay primary and count text remains secondary/readable. Deleting all
  layer geometry leaves files listed with zero counts and an accurate no-nonempty-
  layers message, rather than claiming no drawing was imported.

No new cards, badges, color editor, palette, sidebar model or confirmation flow.
English/German messages and generated key types are updated. App's region label
now identifies Drawings; existing accordion activation/state remains intact.

## CAM compatibility, persistence and actual interaction checks

The existing reviewed Geometry fixture supplies `nested.svg` 8 + `detail.svg` 4
on `svg`, and `plate.dxf` 4 on `Cuts`: three documents, two usable layers,
16 imported segments. The Pocket references `svg`; a disabled Profile retains
its saved two-layer `svg`/`Cuts` binding. Neither is rewritten by the UI.

Actual running frontend checks:

- Initial mixed state (svg shown / Cuts hidden), both hidden, all shown and
  all hidden through the master control work. Space toggles the eye button;
  focus remains visible and state is conveyed by shape/name/pressed state.
  Read-only swatches resolve ACI 7 to actual RGB 255/255/255 in the dark theme
  and have no tabindex or color-edit action.
- Selecting the real outer contour reports center (30, 30), 40×40 mm. Hiding its
  layer preserves that selected-object status, as the audited state model does.
- Generate with Cuts hidden yields 1535 moves /1603 SBP lines. Hiding svg too
  leaves Download enabled; regeneration with both imported layers hidden still
  yields 1535 moves /1603 lines, one warning and the same displayed remaining
  material summary. This is a view-state check, not a new numerical qualification
  or byte-equality claim. Unit tests additionally assert identical complete CAM
  request identity across real `toggleLayer` calls.
- Delete svg removes its geometry from both SVG imports, retaining all three
  file records with 0/0/4 segment counts and Layers 1. Output becomes stale and
  Download disables. Undo restores 16 segments and valid export; visibility itself
  remains outside history. Removing detail.svg instead gives Drawings 2 and
  svg 8; Undo restores Drawings 3/svg 12. File removal clears generated output.
- Deleting all layer geometry leaves Drawings 3, each file 0 segments, and the
  explicit empty-layer message. Undo restores the original source geometry.
- Actual Save/download/reopen retains **exact imports**, including colors,
  coordinates and transforms, and **exact operation source arrays/combine values**.
  [Saved artifact](drawings-layers/roundtrip.ivac-project.json),
  [boundary audit](drawings-layers/BOUNDARY_AUDIT.json),
  [browser observations](drawings-layers/BROWSER_CHECKS.json).
  The portable visibility array stays empty by existing design. In this run,
  reopening after an all-hidden view restored svg shown / Cuts hidden from the
  existing view-state/default path. **Do not claim portable hidden-state retention.**
  Native desktop per-path workspace reopen was not requalified in this browser run.
- Single import correctly shows Drawings 1, its filename and Layers 1, without
  adding a file-removal button. A 240 px sidebar stays usable, with full filenames,
  compact counts and no horizontal overflow. Its existing minimum width is
  preserved; 286 px was used for the ordinary before/after captures.

The incoming uncommitted working tree was fingerprinted before edits. Of 779
existing captured source files, 774 remain unchanged. The five modified files
are LayerList, App's region label, the two locale catalogs and generated keys;
the new presentation helper and its regression tests are separate. All core,
state commands, history, serialization, geometry, source resolution and renderers
are unchanged. File-transform markup/handlers compare identical ignoring formatting.
This is a comparison to task start, not a claim of a clean/published Git tree.

## Before/after visual qualification

Actual unedited screenshots, same 1366×768 viewport, same saved fixture and view
state within each pair. Narrow pairs use the existing 240 px sidebar minimum;
normal pairs 286 px. The earlier visual-hierarchy polish remains the incoming baseline.

| View | Before | After |
| --- | --- | --- |
| A/C — three imports, grouped layers, swatches | [Files/layers](drawings-layers/before-files-layers.png) | [Files/layers](drawings-layers/after-files-layers.png) |
| B — both imported layers hidden | [Hidden](drawings-layers/before-hidden.png) | [Hidden](drawings-layers/after-hidden.png) |
| D — plate.dxf File transform expanded | [Transform](drawings-layers/before-transform.png) | [Transform](drawings-layers/after-transform.png) |
| E — Pocket source svg | [Operation](drawings-layers/before-operation.png) | [Operation](drawings-layers/after-operation.png) |
| F — 240 px sidebar | [Narrow](drawings-layers/before-narrow.png) | [Narrow](drawings-layers/after-narrow.png) |
| G — keyboard focus on layer visibility | [Checkbox focus](drawings-layers/before-focus.png) | [Eye focus](drawings-layers/after-focus.png) |

Additional after-only checks:
[hidden imports with generated paths](drawings-layers/after-hidden-generated.png),
[layer deletion/stale output](drawings-layers/after-delete-layer.png),
[empty layer geometry](drawings-layers/after-empty-layers.png),
[file removal](drawings-layers/after-remove-file.png),
[reopen](drawings-layers/after-reopen.png),
[single file](drawings-layers/after-single-file.png).
Color is read-only, so keyboard focus on a color editor is not applicable;
screen readers receive its image description and Tab skips the passive swatch.

## Frontend regression checks

Seven new tests cover authoritative file/layer counts and provenance, exact-name
aggregation/first color/empty metadata, shared ACI colors including 9, actual view
toggle isolation from CAM/history/selected IDs, portable save boundaries, actual
layer-deletion Undo/Redo and import-removal count restoration. Reactive text and
workspace module initialization are mocked in this node-only suite; these are
logic/state-command tests, not mounted-component tests. Browser checks above
exercise the actual mounted controls and layout.

From frontend with Node24: `npm run check`, `npm test`, `npm run lint`,
`npm run build` all pass. **1270 passed /1 existing skipped, 140 test files**;
Svelte/TypeScript 0 errors/warnings; ESLint/Prettier pass. Existing build advisories
include chunk sizes/static-dynamic Tauri imports; raw final check tails are in
[CHECKS.txt](drawings-layers/CHECKS.txt). No numerical CAM expectations or dependencies
changed, and no new physical/controller qualification is claimed.

## Handoff and remaining UX questions

Updated this audit/qualification, Geometry source UX cross-reference, STATUS and
DEVLOG. Functional ROADMAP, visual hierarchy gate ivac-1drzo and PERF cancellation
gate ivac-nh6q remain unchanged and independent; this request supplies no implicit
acceptance of them. No new major CAM milestone, commit, push, merge or release.

Human review should judge immediate comprehension and comfort from the actual
comparisons; automated tests do not establish visual acceptance. Existing questions
remain separate: whether portable projects should carry view preferences, how
all-hidden reopen recovery should behave, and whether visibility deserves undo.
This change deliberately preserves those policies. **STOP for human review
ivac-ena7m before unrelated work or publication.**

## Human acceptance — 2026-10-15

User accepted the delivered clarification; human gate ivac-ena7m closed.
Next UX6 Phase 0 audit authorized separately; independent hierarchy/PERF gates unchanged.
