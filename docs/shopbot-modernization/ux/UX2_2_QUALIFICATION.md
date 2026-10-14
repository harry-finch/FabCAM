# UX2.2 — Atelier shell qualification, 2026-10-13

2026-10-14 continuation: [A1 branding integration](A1_BRANDING_INTEGRATION.md)
is implemented locally with a new macOS test bundle, runtime/Finder captures and
unchanged shell behavior/identity. This qualifies the branding slice only;
ivac-ein1's broader native lifecycle/human review remains open.

Status: implemented locally (ivac-j6ba) following explicit Atelier approval
(ivac-dwdd closed). Human/native desktop review: **ivac-ein1**. No commit, push,
new desktop bundle, CAM algorithm, FS calculation or post-processor change in this
checkpoint. Existing unrelated dirty work and independent review gates remain.

## Delivered behavior

- No active document: quiet welcome with New, Open, Import SVG/DXF, real local
  recents, existing reopen offer, example and help. Browser explains its picker
  fallback; it does not fabricate reusable filesystem paths.
- New opens an empty workspace using the current machine/tools and stock dimensions.
  Job geometry, operations, fixtures, text/relief, origin/flip, selection/history and
  both generated sides reset. Close returns to welcome. Both use the shared
  Save & continue / Don't save / Cancel guard, including edited empty projects.
- Import at welcome starts a document; import into a document remains additive.
  Existing Open replaces through the shared guard. Failed parse or conversion keeps
  the previous document, identity and undo history. Native cancelled/failed saves
  abort a requested transition.
- Header groups File menu, Open/Save, project identity/save state, history,
  generation and viewport. Empty workspace and geometry without operations have
  separate guidance; Generate is disabled until an operation exists.
- Machine name is visible and clickable in desktop navigation, including welcome.
  Choose/change it there before or during setup; saved projects restore their machine
  context. Long names truncate visually with full accessible label and tooltip.
  Selected-operation tool appears alongside and reveals the existing inspector.
  No single global cutter is invented for projects with multiple operations.

## Ownership and safeguards

AppNavigation, WelcomeScreen and ProjectMenu keep presentation separate from shared
file services. Runtime document active/name/epoch/baseline fields do not change the
project file schema. The saved snapshot baseline makes unsaved state independent
of legacy generation clearing `data.dirty`; saving does not refresh generation's
input context. Epoch guards reject generation output/progress for a replaced job.
The file-load transaction restores prior state/history on commit failure. Existing
workspace storage, source watcher, serializer and generation/export policy remain
owners of their respective behavior.

## Evidence

Frontend suite: 123 files, 1135 passed / 1 optional skip. The final full run exposed
only a one-line component-size budget excess; after shortening an obsolete comment,
the size guard's 3 tests passed on rerun. Check: 0 errors/0 warnings. Lint and build
passed (existing bundle/dynamic-import advisory warnings). `git diff --check` passed.

Fourteen lifecycle tests exercise shared services with real project reset/snapshot/
restore/history and mocked external I/O: empty edited guard, cancel/failure/save-close,
filename/recents, edits during save, picker cancel, failed validation, rollback after
partial restore/text conversion, additive import, empty saved document and loading
interlock. Existing generation-context, unsaved and UI guard suites also pass.

Built browser UI at localhost using WASM was exercised for welcome → New → Close,
example load, unsaved Cancel and Save & continue (download observed), opening the
CAM4 saved project, correct saved identity/machine, selecting its operation/tool,
machine navigation, tool shortcut back to inspector, and File-menu Escape/focus.
Reviewed 1024×768, 1280×800 and 1440×900 layouts with long fixture names.
Primary actions remain available; sidebar retains its existing scrolling behavior.

Screenshots:
- [Welcome](UX2_2-welcome.jpg)
- [Empty project](UX2_2-empty.jpg)
- [Unsaved guard](UX2_2-unsaved.jpg)
- [Workspace, machine and operation tool — 1280×800](UX2_2-workspace.jpg)
- [Reduced workspace — 1024×768](UX2_2-workspace-1024.jpg)

## Human/native review boundary

Native picker/save cancellation and write failures are covered by mocked integration
cases, not a newly built native app in this checkpoint. Real native reopening,
Save As and recents, German text layout, OS display scaling and a complete
Generate → edit → Save → stale export → regenerate interaction remain desktop
qualification checks under ivac-ein1. Existing automated generation validity tests
pass; this shell work provides no new physical or ShopBot post evidence.

Known separate tool inventory/project same-ID editing issue ivac-hs9i remains;
the header shortcut uses the project operation inspector. CAD1, waterjet, FS4
and all other review gates remain separate. Review this result before starting
another milestone; publication still needs explicit authorization.

## Final polish continuation

See [UX2_2_POLISH.md](UX2_2_POLISH.md) for the user-requested three refinements,
current 1145-pass evidence and updated screenshots. These supersede the earlier
empty-canvas import and long machine label. Human acceptance remains ivac-ein1.
