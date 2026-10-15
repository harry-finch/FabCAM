# UX6-A — Machine Profiles UX Redesign

2026-10-15 · **ACCEPTED — ivac-osa4z closed; user authorized UX6-B**.
Phase 0 approved by the user's explicit authorization of A; engineering ivac-jhfns.
[Implementation, regression results, actual UI captures and limits](UX6_A_RESULTS.md).
The original acceptance specification follows. HARD STOP before separately approved B.

## Scope and provisional information architecture

Compact left profile browser/right selected detail: inspect, create, duplicate and
supported import/export; identify current project assignment separately. Existing
picker immediately applies a profile today; future browsing/“Use for this project”
semantics need explicit approved mapping, not an assumed UI-only active state.

Provisional sections follow actual Phase 0 field matrix:

- General: name, process, envelope, relevant known constraints, authoritative status.
- Program output: post/dialect, output units/features, tool change and legitimate
  program spindle/end behavior contextual to the selected post.
- Tools: machine loadout, explicit shared-library add/remove, tool-details access.
- Advanced: estimation assumptions, specialized/legacy/process settings as classified.

Essential safe Z/limits or consequential uncommon parameters must remain discoverable
and clearly explained, not automatically buried in Advanced. Distinguish physical
constraints, validation, estimation, preview and emitted behavior; fields may overlap.
Preserve incompatible authored options and output refusal, not silently normalize them.
ShopBot/GRBL/LinuxCNC/process options cannot imply unimplemented capability.

Lightweight first-profile assistant: identity/process → supported output → workspace/
known limits → review → save. Permit draft where existing model safely supports it;
unknown limits stay unknown. Draft/configured/software/physical labels require evidence,
not an invented lifecycle. No live homing/probing/controller controls.

## Edit/application compatibility and visual requirements

Audit Apply/Revert/current-draft versus profile assignment, auto-mirror and tool-copy
effects before relabeling. Profile browsing cannot silently change project assumptions.
Assignment uses existing history/compatibility/stale/export checks, preserves manual
cutting values and exposes tool-binding conflicts. Missing local profile must not
prevent reopening its embedded project machine/tools snapshot. Preserve file formats.

Mandatory Professional Workshop UI + CAM UX: controlled widths, short aligned fields/
units, quiet list/sections, contextual help and keyboard/focus. No wide monolithic form,
dashboard, excessive badges or hidden incompatible state. Shared patterns with B.

## Future acceptance criteria and evidence

1. Profiles findable, inspectable and current project assignment unmistakable.
2. Browsing/editing distinct from assignment with approved state transitions.
3. Sections reflect source-backed field ownership/runtime effects.
4. Output options contextual to post/process and unsupported values explained.
5. Machine limits versus estimate assumptions clearly distinguished.
6. Compact readable aligned numeric values/units and long-name handling.
7. Unsupported capabilities/unknown evidence stay explicit.
8. Machine-tool association workflows remain functional and accurately labeled.
9. Apply/Revert/import/export/destructive scope understandable and preserved.
10. Existing profiles/projects/formats and missing-profile snapshots compatible.
11. CAM/output, compatibility, history, stale/export and saved cutting values preserved.
12. Relevant automated regressions and actual desktop visual/accessibility review pass.

Screenshot matrix: profile list/summary, General/Program output/Tools/Advanced, long
name, unvalidated/software-only profile, incompatible saved options, draft/application
and narrow desktop. Tests cover switch/edit/revert/cancel/import/delete, geometry/tool
effects and save/reopen/Undo within supported storage scopes. Human approval and
**HARD STOP before UX6-B**; passing tests alone are not visual acceptance.

Phase 0 resolved browse/edit/assign ownership and creation scope. A implements that
contract without schema changes; human software/visual acceptance remains open.
