# Preview navigation polish — 2026-10-15

User-requested focused follow-up ivac-tgxsn, after UX6-B acceptance. No new milestone.

- 2D `canvas/render/chrome.ts::drawGrid` previously returned without drawing when
  1mm was smaller than6px. It now chooses a decimal minor interval with6–60px
  spacing and tenfold major interval. Lines remain anchored to world zero using
  screen-space modulo; viewport-bounded loops avoid accumulated world-step error.
  The ordinary10px/mm view retains1mm minor/10mm major spacing.
- 3D `Scene3D.svelte` explicitly sets OrbitControls.mouseButtons.MIDDLE to
  THREE.MOUSE.PAN. Local installed OrbitControls source confirms this uses pan;
  left orbit, existing right pan/context tap, wheel zoom and touch mapping remain.
- `ShortcutHelp.svelte` documents middle-drag and retained right-drag for3D pan.

Grid is visual only: no snapping interval, drawing/project coordinates, source
selection, machining tolerances, CAM generation or export/persistence format changes.

## Qualification

42 targeted tests pass in3 files: renderer/grid, viewport transforms and pointer-down
intent. Grid regressions cover0.000001–1000000px/mm, bounded density, signed pan
origin alignment, ordinary1/10mm spacing and invalid scales. Svelte check0 errors/
warnings; lint and production build pass (existing bundle warnings). [Logs](preview-navigation/CHECKS.txt).

Actual local browser/native HTTP demo simple.dxf: repeated wheel zoom out and in,
fit restoration,3D open and revised help reviewed. [Zoom out](preview-navigation/grid-zoom-out.png),
[zoom in](preview-navigation/grid-zoom-in.png), [3D scene](preview-navigation/view3d.png).
No generated program, authored user file or machine/tool library mutated.
The central-button drag itself was not automated: available CUA drag API exposes
no mouse-button parameter. Mapping is verified from actual Scene3D initialization
and installed Three controls; no claim of physical mouse or packaged-native testing.

User review of the two requested improvements follows. GEOM1 remains proposed,
not launched. Independent gates and prior publication limits retained.
