> UX4.1 continuation: human UX review requested template/sidebar refinements.
> Initial gate ivac-51gg is superseded by ivac-si5w. Current UI and qualification:
> [UX4.1](UX4_1_QUALIFICATION.md). Original template semantics/evidence limits remain.

# UX4 — Reusable operation templates

2026-10-14 · engineering **ivac-epg2** · human gate **ivac-51gg**.
User requested UX4 continuation, accepting UX3 software handoff under ivac-k4z3.
Local uncommitted implementation; hard stop before UX5. Independent FS, native
and physical qualification gates retain their evidence limits.

## Bounded V1

Templates is in the Operations header. Select an operation, name and save its
model; choose a library entry, destination geometry and cutting tool, then add.
Pocket also supports an explicitly rebound distinct finish tool. Surface
uses an explicit copy of current stock, never the captured rectangle coordinates.

V1 supports front-side Profile, Pocket and planar surfacing with Endmill tools
without Whirl. Unsupported operations/tool types, backside, patterns, authored
approach/manual or mixed tab placements are refused. Automatic tab settings and
portable contour/pocket parameters use an explicit whitelist. Distinct-tool
recipes require explicit step depth; single-tool inherited step is resolved and
frozen at capture. No source drawing IDs/layers, physical tool IDs, source surface
bounds, knowledge/proposal/evidence blobs or machine setup enter the model.

Depths, entry/strategy settings and resolved RPM/feed/plunge values are retained.
Destination defaults never recalculate these values. New operations contain manual
rough/finish snapshots and current geometry/tool IDs. A compatible tool family
check is not physical cutting suitability: review material, size, reach and machine.
Normal generation performs its existing checks. Library edits never change a job.

## Architecture and compatibility

[ADR-0015](adr/ADR-0015-reusable-operation-templates.md).

| Seam | Disposition |
| --- | --- |
| operation-templates.ts | New pure whitelist capture/validation/explicit binding |
| workspace.ts | Add local library, validate on load, serialize writes, propagate errors |
| OperationTemplates.svelte / OperationsList | Existing Modal and inspector styles; one library entry point |
| commands/history, build-project, project persistence | Reuse ordinary operation addition and guarded v3/v6 formats |
| FS engine/data, CAM geometry, Rust, posts, API schema | Unchanged by UX4 |

Library entries version 1 live in additive workspace schema 1; old missing arrays
load empty. Invalid/duplicate entries are skipped. Browser storage uses the current
origin/profile, native uses the existing workspace file. Explicit save errors are
visible; ordered writes are per store, not global multi-window transactions.
Library is not embedded in .ivac-project jobs. No library import/export/sync,
older-app downgrade preservation or new native runtime/bundle qualification.

## Software evidence

- **1172 frontend tests pass, 1 optional skip; 127 files.** Eleven UX4 cases cover
  three kinds, source/evidence omission, actual overrides, source default step,
  distinct tool roles, stale/missing geometry, invalid tools/mode/stock, malformed
  models, legacy/load/save/reload, deep-copy independence, atomic Undo/Redo,
  build-project wire binding, ordered writes and explicit/debounced storage failure handling.
- Svelte/TypeScript check: zero errors/warnings. ESLint/Prettier pass.
  Production WASM/frontend build passes; existing chunk/import warnings remain.
- No UX4 core/post changes. UX3's 1074 core and 8 surfacing integration passes
  are prior milestone evidence, not a new UX4 Rust rerun.
- Actual browser: capture surface, empty explicit choices block addition, bind
  stock/tool, add operation ID2, Undo returns one operation, Redo restores two.
  Downloaded v6 project preserves manual 10000/1000/500 and explicit -0.5 step;
  source operation remains unchanged. Generate/export produces a normal .sbp
  containing both operations. Feed edit invalidates output and disables export.
- Reload/reopen retains library independently of project selection. Rename,
  remove and in-dialog restoration succeed; the existing job retains its old name.
  Fixed string/numeric select mismatch discovered visually: selected tool remains
  visibly selected. Save actions serialize and show transport failures.
- Pocket model rebound from Ø12 T1 to Ø8 T7 with different defaults (9000/500/100)
  retains 10000/1000/500 and depth -2/step -1. Actual downloaded v3 project stores
  current object ID1 and T7/manual slots. Normal ShopBot generation succeeds.
  A source Profile initially enabled caused expected stock/work-envelope warnings;
  it is disabled in the supplied fixture to isolate pocket/template verification.

Actual downloaded projects: [surface](UX4-software-surface-saved.ivac-project.json),
[pocket](UX4-software-pocket-saved.ivac-project.json). Software-only inputs, not
cutting advice; duplicate operations deliberately demonstrate library independence.

Captures: [stock/tool binding](ux/UX4-template-binding.png),
[pocket tool rebinding](ux/UX4-pocket-rebinding.png),
[generated pocket](ux/UX4-pocket-preview.png).

## Human review / next action

Open the [local review session](http://127.0.0.1:18767/?api=http%3A%2F%2F127.0.0.1%3A18772).
Library is local to this browser origin; a desktop installation has its own store.

1. Open [contour source fixture](UX4-software-contours.ivac-project.json). Select
   Pocket, save a named model, verify empty destination choices, bind drawing and
   T7. Verify retained values and Undo/Redo. Generate and review warnings/preview.
2. Save/reopen. Rename/remove/restore the model; verify the job stays independent.
   Profile capture is also supported; review its stock/work-area requirements
   before enabling it. Two-tool templates need distinct explicit role choices.
3. Open [surface source fixture](UX3-software-surfacing.ivac-project.json). Capture
   a model, change destination stock, explicitly bind it and a flat tool. Check
   new rectangle/depth/rates and normal stale/export protection after an edit.
4. Review refusal of incompatible/source-bound operations and incomplete binding;
   review keyboard/focus and compact dialog readability.

**HARD STOP: UX4 awaits ivac-51gg before UX5.** No commit/push or physical/native
qualification is implied by this software handoff.
