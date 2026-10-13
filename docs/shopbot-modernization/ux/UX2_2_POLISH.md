# UX2.2 — Final Application Shell Polish

2026-10-13 · continuation ivac-8u1p · **awaiting human approval ivac-ein1**.
No new milestone, commit, push or native app package. Existing Atelier design retained.

## Refinements

**Machine:** `MachineSettings.name` is the portable authoritative name;
`MachineProfile` has no short-name or validation-status field. The presentation
helper extracts explicit literal qualifiers (SOFTWARE ONLY, unvalidated, etc.)
from that name. For existing FS/UX/CAM fixture prefixes it shows the remaining
machine name beside a small amber Lucide icon and the untruncated qualifier.
Full identity remains in the button's tooltip and accessible name. Ordinary names
retain their content and ellipsis where necessary. No inferred validation state,
positive certification, metadata migration or navigation change.

**Import:** Layers retains the dominant Open file button and SVG/DXF guidance.
Removed the redundant empty-canvas invitation. File → Import and Layers → Add
remain available for subsequent drawings. Welcome remains distinct from an empty
project. No import service or additive/replacement semantics changed.

**Generation:** welcome has no generation toolbar; empty projects and geometry
without operations have no irrelevant idle warning chip. Geometry still prompts
operation creation. Operations without output retain Generate-first messaging.
Existing generated/diagnostic states remain visible, including when no operations
remain. Shared summary now marks changed generated output stale even before sim
has run; it uses the existing generation context supplied by callers. No parallel
validation status. Critical counters, error notifications, stale export and critical
export checks are unchanged.

## Tests and observations

- `npm --prefix frontend run check`: 0 errors, 0 warnings.
- `npm --prefix frontend run lint`: passed.
- `npm --prefix frontend test`: 124 files, **1145 passed, 1 optional skip**.
- `npm --prefix frontend run build`: passed; existing chunk/dynamic-import advisories.
- `git diff --check`: passed.
- New machine identity tests: short/long names, fallback, software-only fixture,
  explicit unvalidated/unsuitable names, serialized name round trip. Full tooltip
  and accessible identity verified in actual browser UI.
- Summary tests: welcome/empty/no-operations, operations/idle, clean, stale before
  simulation, warning and critical counts retained. Existing lifecycle and
  generation-context regressions pass, including save/cancel/failure behavior.
- Built browser: New → primary Layers import `rectangle_100x50.svg`; File → Import
  `circle_d50.svg` adds a second drawing (both retained; 260 segments). No idle
  warnings with zero operations.
- Existing `UX2-software-collision.ivac-project.json` opened through picker: name
  resolves to `ShopBot post` + `SOFTWARE ONLY`, full `UX1 …` identity accessible.
  Reviewed header at 1280×720 and 1024×768; limitation stays visible.
- Existing local native server at port 18772 generated the CPS fixture: seven
  warnings, one critical, collision alert. Download attempt rejected by existing
  critical check. Thickness edit 12 → 13 made output stale and download disabled.
  Save kept it stale. Reopening the downloaded project retained 13 mm and full
  machine name, reset generated output and correctly requested generation.

## Review captures

1. [Welcome](UX2_2-polish-welcome.jpg)
2. [Empty project](UX2_2-polish-empty.jpg)
3. [Imported geometry, no operation](UX2_2-polish-geometry.jpg)
4. [Operation before generation](UX2_2-polish-operations.jpg)
5. [Generated critical warning fixture](UX2_2-polish-warning.jpg)
6. [Long software-only machine identity, 1024 px](UX2_2-polish-machine.jpg)
7. [Saved project with stale output](UX2_2-polish-stale.jpg)

## Limits and handoff

Literal name qualifiers are not structured machine validation; absence of a qualifier
never means validated. Unknown free-form qualifications remain in the full name;
this change does not claim to understand arbitrary prose. Native file dialogues,
German layout and OS scaling still need desktop review as recorded in the main
qualification. Browser + native-server tests are not a newly packaged Tauri test or
physical machine qualification. No CAM/post/FS logic changed.

STOP: awaiting human approval. Do not start CAM3 or any other milestone.
