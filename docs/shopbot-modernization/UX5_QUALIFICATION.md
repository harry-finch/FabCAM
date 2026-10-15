# UX5 — Command Palette & Quick Actions software handoff

2026-10-14 · Engineering ivac-0vbt; human software gate **ivac-txve**.
The user explicitly accepted the UX5 audit/ADR design (ivac-2zh9) and authorized
implementation. This handoff stops before later milling work or publication.

## Discoverability continuation — human review pending

2026-10-14 · refinement ivac-snsy, within the existing UX5 gate **ivac-txve**.
The human review requested removal of the Welcome palette hint. The three primary
entry actions, A1 branding, introductory copy, recents, example and Help remain.
No replacement banner, search bar, card, floating button or toolbar control.

Audit decision: retain the existing File/Project menu, which already owns global
New/Open/Import/Close actions. Its sole palette launcher now reads **Command
Palette…**, with **⌘K on macOS / Ctrl+K on Windows/Linux**. Help documents this
same opening shortcut plus ↑/↓, Enter and Esc; it adds no duplicate launcher.
The Welcome Help and shortcuts path remains available for discovery with no
new permanent Welcome content. The shared `commandPalette.show()` and existing
`resolveShortcut` handler remain authoritative. New helper only formats the
platform shortcut; registry/search/ranking/availability/handlers/history/safety
and project formats are unchanged.

Verification: **1232 frontend passes, 1 optional skip, 135 files**; check0 errors/
warnings, lint and production build pass; diff whitespace check passes.12 targeted
presentation/platform tests complement existing search, dispatch, keyboard,
availability, history and safety regressions. Source-contract tests intentionally
cover the Welcome/menu/Help presentation; actual interaction is verified below.

Actual macOS in-app browser,1280×720:

- Welcome retains New/Open/Import and no palette content element; Cmd+K enters
  the search, Esc restores New project.
- Empty active project: Cmd+K opens; Generate remains disabled for missing
  compatible geometry; Esc restores the2D tab.
- Loaded five-operation fixture, one geometry selected, and a selected cutting
  operation: Cmd+K opens in all contexts; search and Up/Down remain functional.
- File opens with keyboard Enter; the existing menu callback opens the palette.
  Search autofocus, Tab remains within the shared modal, Esc returns to File.
- Operation Name text entry rejects the global shortcut; an open Templates
  dialog remains the only dialog after Cmd+K. Existing modal/input guards remain.
- Help displays ⌘K and concise search/navigation/execution/dismissal instructions.
- Two actual Save downloads before/after palette open, query, arrows and Esc are
  **byte-identical**, including selected geometry, tools, five operations and
  manual slots. SHA256:35c42ff483a497d39d8db781e84fdfca14bd9d850497c4f2434b7e8f768ceeca.

Current screenshots:
[clean Welcome](ux/UX5-polish-welcome.png),
[palette from Welcome](ux/UX5-polish-welcome-palette.png),
[File menu and macOS shortcut](ux/UX5-polish-menu.png),
[loaded workspace palette](ux/UX5-polish-workspace.png),
[Help documentation](ux/UX5-polish-help.png).

No observed regression or remaining implementation issue in this bounded polish.
Windows/Linux Ctrl+K mapping and display are unit-tested; their native runtimes
are not newly qualified. Earlier native/IME/picker/physical limits remain. No
publication or later milestone started. **STOP at human review ivac-txve.**
The initial implementation evidence below remains historical where it mentions
its former Welcome launcher or1220-test count.

## Implemented scope

Cmd/Ctrl+K and the File menu open a compact local palette. It searches
commands, tools and existing v1 operation templates. English/German UI and explicit
English/French/German aliases are local metadata; no network search, AI, FS call,
geometry calculation or persistence occurs while searching.

31 command descriptors plus existing recent projects cover file/history actions,
Profile/Pocket/Drill creation, selected-operation duplication/deletion, sidebar and
workspace navigation, 2D/3D, Generate/Cancel/Warnings, front/back export, template
browse and selected-operation capture. Owners retain their existing transactions,
discard handling, errors, capability checks and file operations. CAD reveals the
existing Layers corner-relief controls; it does not fabricate a CAM operation.
Advanced operation wrappers, stocking/removing tools and editing tool drafts are
left in their ordinary interfaces.

Search uses prepared, whitelisted name/comment/kind/id/dimension metadata. Actual
name matches precede generic aliases; word/prefix/substring matches precede a
bounded alphabetic one-edit/transposition fallback. Numeric filters never fuzz:
`6mm upcut`, `3.175 single flute`, decimal commas, `1/8in`, and known `V-bit 60`
metadata are tested. Bare integers stay textual. Known diameter tolerance is
1e-6 mm; missing physical metadata cannot satisfy a filter. Exact disabled actions
remain discoverable with their reason. Stable ids break ties; the UI shows at most
100 rows and asks for a more precise query when there are more.

Tool results preserve inventory/project origin even for colliding ids. Enter
opens read-only inspection. Explicit Assign requires a real, stocked, finite-
diameter, kind/mode-compatible tool and a selected cutting operation. A matching
inventory copy must match the stocked definition, not merely its id. Saved cutting
roles must cover the destination tool; missing roles offer parameter review and
never relabel/recompute slots. Legacy operations explicitly retain existing tool-
default behavior. Assignment uses `project.updateOperation` and its normal history.

Template Enter preselects the stable id in UX4.1's existing binder. Geometry and
cutting tools remain explicit; the binder owns validation, frozen manual values,
creation, undo and persistence. No template/model/schema/CAM/post changes.

The dispatch bridge checks owner generation, document epoch/history/selection,
current availability and captured result identity. Disposed/replaced owners and
changed targets refuse; actions are never queued for a future document. Repeated
execution is blocked; Cancel remains usable while an asynchronous Generate owns
the dispatch. GenerateBar owns actual generation/cancellation and export. A shared
pure policy preserves its aggregate front pipeline+simulation critical checks,
back pipeline critical checks and per-side work-area gate. Raw file export still
owns fixture/freshness checks and the existing recheck after a native save picker.

## Verification

Frontend: **1220 passed, 1 optional skip, 133 files**; Svelte/TypeScript **0 errors,
0 warnings**; ESLint/Prettier and production build pass. No new Rust/CAM/post code;
existing build warnings for shared static/dynamic imports and large chunks remain.
`git diff --check` passes. Component budgets remain unchanged (App below2200,
GenerateBar below1100; palette logic stays in dedicated modules).

42 additional tests cover ranking, aliases, typo/numeric/metadata cases, deterministic
origin identities, no search mutation, owner disposal/replacement/context/availability,
repeat/cancel behavior, saved tool roles, incompatible/program-only/Whirl cases,
front/back export policy and keyboard/IME decisions. Existing owner, generation-
context, history, document-lifecycle and template regressions also pass.

Actual Codex in-app browser at 1280×720, plus 1100×660:

| Scenario | Observed result |
| --- | --- |
| No document vs New empty document | Export asks to open/create; empty active Generate asks for compatible geometry |
| Unsaved Close/Cancel | Existing discard dialog opens; Cmd+K does not nest another palette; Cancel retains six operations and returns focus; Undo restores five |
| Welcome/menu/Cmd+K | Search autofocus; Escape restores Welcome/File trigger; Tab wraps inside the shared modal |
| French `poche`, exact template name, `T1`, `6.35 single flute`, no results | Commands/templates search together; actual name precedes a generic kind alias; origin collisions visible; physical filter and no-result states clear |
| Inspect/Assign | Inspection leaves tools/operation untouched; saved T7 operation refuses T1 missing roles; inventory T1 differs from stocked T1 and cannot be assigned; explicit legacy Profile T1→T7 works, Undo restores T1 |
| Generate/Export | Existing four-operation fixture generates140 moves/248 ShopBot lines; palette export downloads a real metric `.sbp` through the owner |
| Stale output | Tool change disables palette export; subsequent project Save does not make a changed output fresh |
| Critical output | Oversized software fixture generates one critical warning; palette disables export and Enter leaves it refused |
| Template handoff | Matching UX4.1 pocket preselected; explicit drawing/T7 choices create independent op5; actual Save/reopen retains five operations and original manual slots |
| History actions | Direct Pocket count5→6, duplicate→7, delete→6, Undo→7, Redo→6 through ordinary owners |
| Reduced desktop | Palette remains inside viewport; scroll results, keyboard footer and close/search controls remain visible |

[Actual saved five-operation project](UX5-software-created-saved.ivac-project.json):
v3, new op5 binds object1/T7, depth-2/step-1, rough and finish10000rpm,
1000mm/min feed and500mm/min plunge. It has the same ordinary manual slots as UX4.1;
other operations remain independent. **Software-only fixture, not machining approval.**
[Deliberately oversized critical fixture](UX5-software-critical.ivac-project.json)
uses D100 and is intentionally unsuitable for machining.

Actual captures:
[palette](ux/UX5-palette.png), [origin collision](ux/UX5-origin-collision.png),
[saved-role refusal](ux/UX5-tool-saved-values-refused.png),
[stale export](ux/UX5-export-stale.png), [critical export](ux/UX5-export-critical.png),
[reduced desktop](ux/UX5-small-desktop.png).
The earlier SVG remains a design concept, not runtime evidence.

## Search performance

Reproducible: Node24 `node frontend/scripts/palette-benchmark.mjs`.
[Synthetic corpus/results/environment](ux/UX5-search-benchmark.json):50commands,
5000origin-qualified tool records (collisions/missing metadata),200templates;
30 warm runs per each of9 queries. AppleM4/arm64/macOS, Node24.21.0:
**cold index7.838ms; largest query p95 1.303ms; largest observed sample1.526ms**.
Accepted budgets cold≤150ms and warm p95≤50ms pass. These measure pure indexing,
matching/ranking, **not browser paint, native dialogs or CAM generation**.

## Evidence limits and review

Actual runtime evidence is the local browser/HTTP-CPS path. Native macOS app,
Windows/Linux shortcut conflicts and native IME input were not newly executed;
composition/repeat guards are unit-tested. Back-program/work-area policy and
late native-picker freshness retain existing software checks/tests, without a
new native two-sided/picker runtime claim. Fast generation did not establish a
new long-running cancellation performance claim; dispatch cancellation/repeat
semantics are tested. No physical machining, machine-control, safety certification,
new native bundle, commit, push or release is implied.

Review the palette with the supplied fixture: search `poche`, `T1`, a template
name; navigate with arrows, Inspect a tool, verify an explicit Assign/refusal,
then Generate/Warnings/guarded Export. Confirm the compact presentation and
keyboard focus. **Next action is human gate ivac-txve.** Later roadmap work is
milling/V-carve qualification; its scope remains to be confirmed after UX5 review.
