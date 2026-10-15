# Visual hierarchy and contrast refinement

2026-10-15 · focused continuation of UX2.2, not a new major milestone.
Engineering ivac-28e92 complete; **human visual review ivac-1drzo pending**.
Authoritative language: Professional Workshop UI, with CAM UX safety rules.
Current PERF gates, including CPS cancellation ivac-nh6q, remain unchanged.

## Initial assessment and selected changes

Actual baseline captured before editing, using the running frontend with the
existing `UX2-software-setup.ivac-project.json`. Its square, fixtures, long tool
name, software-only profile and Profile inspector exercise the compact shell.
The global navigation was already slightly lighter than the command bar
(`#2a2a2a` versus `#222222`), but its context labels used the same treatment as
destinations. Inspector section labels used the same muted gray as help text.
At visible-grid zoom, closely spaced minor lines competed with the drawing.
The existing structure, active tab and operation selection were already useful.

Only four source files change, and only CSS declarations:

| Area | Selected refinement | Reason and boundary |
| --- | --- | --- |
| Global navigation | New derived `--bg-navigation`, 80% elevated surface / 20% border | A small graphite luminance difference from the command bar; reuse its existing separator and active tab |
| Machine / operation tool | Color the existing leading border of inactive context buttons; labels 0.74rem | Shared quiet context cue; names retain their original size, ellipsis, full titles and accessible names |
| Inspector sections | New derived `--text-section`, 65% muted text / 35% primary text | Geometry, Cut, Cutting parameters and advanced disclosure headings become a little easier to locate; no extra boxes, spacing or weight |
| Dark minor grid | Existing `--grid-minor` changes `#262626` → `#222222` | Reduce dense background detail; retain major grid, axes, work zero, stock, selection and snapping |

The project toolbar remains `--bg-panel`, as do sidebar/inspector surfaces; the
canvas remains `--bg-app`. Four strongly contrasting bands would add noise.
No added shadows, badges, cards, icons, accents, controls or navigation markup.
No generic text/help contrast reduction. Active navigation styling stays intact.
The context buttons retain their actual navigation actions; no live connection
or machine-readiness treatment is introduced.

## Tokens, dimensions and grid decision

Tokens live in `frontend/src/app.css`; both new structural roles derive from
existing theme colors and follow explicit/automatic dark and light themes.
Only the dark minor-grid token changes; the light grid remains as before.
Consumers are AppNavigation and the existing shared operation styles, including
the scoped Cutting parameters headings in FeedsSpeedsSection. No broad global
selector or component rewrite is needed.

Measured dark solid-color contrast: navigation/toolbar 1.108→1.168; section
heading/panel 6.08→8.06; minor grid/canvas 1.173→1.116. These quantify the tonal
direction, not a human scanning-time study or an accessibility ratio for grid lines.
Primary control text, warning/error colors and font sizes are unchanged.
Header height remains **30.289px at 1366×768** and **45.789px at 1024×768**.
The existing Tool library wrap in the smaller window is retained, not expanded.
No horizontal document overflow was observed, including the long-name stress case.

Grid audit: `canvas/render/chrome.ts` draws 1mm minor / 10mm major lines only
when the minor spacing reaches 6px. Below that threshold both grid layers are
already hidden. This behavior is unchanged; overview captures therefore need
not contain a grid. At the same visible-grid zoom the lighter visual load helps
distinguish major references and work zero. Normal/overview/closer views were
compared using Fit and the same native wheel gestures. No spacing, snap,
coordinates, transform, visibility threshold or painting algorithm changed.
The canvas caches resolved colors: after CSS hot reload the existing theme
controls were switched Light → original Auto to refresh that cache before the
grid evidence was retained. The final build starts with the correct tokens.

## Actual visual comparisons

All images are unedited screenshots of the running frontend. Main comparisons
use 1366×768; smaller-window comparisons use 1024×768. The same saved project,
operation and generated result are used within each pair. The generated result
has 136 moves, 160 SBP lines and six settled warnings. Initial, ungenerated and
generated views are separate comparisons; their layouts are not mixed as a pair.

| Required view | Before | After |
| --- | --- | --- |
| A — drawing and operations | [Workspace](visual-hierarchy/before-workspace.png) | [Workspace](visual-hierarchy/after-workspace.png) |
| B — selected Profile, Geometry/Cut/Cutting parameters | [Inspector](visual-hierarchy/before-inspector.png) | [Inspector](visual-hierarchy/after-inspector.png) |
| C — machine / selected-operation tool context | [Header in inspector](visual-hierarchy/before-inspector.png) | [Header in inspector](visual-hierarchy/after-inspector.png) |
| D — actual warnings, software-only profile | [Warnings](visual-hierarchy/before-warnings.png) | [Warnings](visual-hierarchy/after-warnings.png) |
| E — 2D normal fit after generation | [Normal](visual-hierarchy/before-grid-normal.png) | [Normal](visual-hierarchy/after-grid-normal.png) |
| E — closer visible grid | [Closer](visual-hierarchy/before-grid-close.png) | [Closer](visual-hierarchy/after-grid-close.png) |
| E — zoomed-out overview | [Overview](visual-hierarchy/before-grid-overview.png) | [Overview](visual-hierarchy/after-grid-overview.png) |
| F — Welcome | [Welcome](visual-hierarchy/before-welcome.png) | [Welcome](visual-hierarchy/after-welcome.png) |
| G — smaller desktop | [1024px](visual-hierarchy/before-small.png) | [1024px](visual-hierarchy/after-small.png) |

Supplementary checks are after-only stress/acceptance views, not before/after
comparisons: [long context](visual-hierarchy/after-long-context.png),
[long context at 1024px](visual-hierarchy/after-long-context-small.png),
[light theme](visual-hierarchy/after-light.png),
[machine area](visual-hierarchy/after-machine.png),
[sticky inspector](visual-hierarchy/after-sticky-inspector.png).
`long-context.ivac-project.json` is derived from the existing software fixture:
only machine/tool names differ. It is not a qualified machine profile.

## Accessibility, state and regression checks

Actual UI checks:

- Machine destination and selected-operation tool shortcut retain their actions.
  Keyboard Tab from Machine reaches Tool library with the existing solid blue
  focus outline; Enter opens the actual library, including tools/filter controls.
- Long names truncate inside context values, while full identity remains in
  title/accessible name. At 1024px the stress machine's value is 84px for 385px
  of text; its SOFTWARE ONLY label remains fully visible. No connection-status
  or safety qualification meaning is added.
- Profile selection opens the existing inspector. Scrolling it retains the
  Operations header at y=200.453px. Section font size, padding, density, sticky
  rules and selected-operation styling remain unchanged.
- Open loads reviewed fixtures; Generate returns actual results; Save emits a
  project download and leaves Saved project visible. No save-format change.
- A temporary operation rename shows Unsaved changes, stale warnings and disabled
  Download. Undo restores export, Redo disables it, final Undo restores the saved
  input. [Stale state](visual-hierarchy/after-stale.png).
- The oversized-pocket fixture retains **2 warnings / 1 critical**, with explicit
  tool-too-large text. With the existing critical-block setting checked, clicking
  Download refuses export and shows the existing alert. The settled button itself
  remains enabled in this pre-existing design; refusal occurs in the action guard.
  Do not confuse that with the disabled stale-output button.
  [Critical diagnostics](visual-hierarchy/after-critical.png),
  [export refused](visual-hierarchy/after-blocked-export.png).
- The incompatible V-Carve fixture shows the existing stock-top/depth qualification
  error and no downloadable output. [Failed generation](visual-hierarchy/after-error.png).
- Clicking the actual square edge in 2D selects one 100×100mm object, center
  (60,60), with its explicit status text. Escape and operation selection still work.
  Snap mathematics and drawing controllers are unchanged and covered by the full
  frontend suite; no new physical snap/measurement claim is made.
- Welcome retains branding/actions and proportions. Light theme remains readable;
  original Auto theme is restored. Temporary viewport overrides are reset at handoff.

[Boundary audit](visual-hierarchy/BOUNDARY_AUDIT.json): 775 of 779 captured source
files are unchanged; the four exceptions are CSS-only. Both Svelte non-style
sections match their pre-polish source exactly. Core, state, rendering algorithms,
post, warning/export policy and persistence source are unchanged relative to the
working tree at the start, including its pre-existing uncommitted PERF work.
This is not a claim that the entire working tree equals published main.

Checks from frontend, Node24:

```sh
npm run check
npm test
npm run lint
npm run build
```

Results: **1263 passed / 1 existing skipped, 139 test files**; Svelte/TypeScript
zero errors/warnings; ESLint/Prettier pass; production build pass. Existing build
advisories concern large chunks and mixed static/dynamic Tauri imports; no new
dependency or bundle restructuring. No extra tests that merely mirror CSS were
added. No CAM/physical requalification is asserted for this presentation change.

## Rejected changes, documentation and handoff

Rejected as unnecessary: changing all structural surfaces, adding another header
separator or stronger active underline, moving navigation items, contextual badges,
brighter controls/help, heavier headings, subsection backgrounds, larger spacing,
grid density/zoom logic changes, and a broader shell redesign. The selected changes
keep the workspace primary while giving scan landmarks a little more distinction.

Updated this qualification, UX2.2's qualification index, design-token direction,
DEVLOG and current STATUS. Functional ROADMAP and all existing approval gates are
untouched. No ADR is needed: existing components/tokens are extended; architecture,
models and process boundaries are retained.

Remaining observations: small desktop Tool library wrapping is pre-existing;
native tooltip-popup rendering and subjective comfort across prolonged real use
remain human review items. No user study, every localization or physical-machine
validation is claimed. **STOP for visual review ivac-1drzo. No automatic next work,
commit, push, merge or release.**
