# Presets library — adjustable compact layout

**Current disposition:** accepted and integrated locally on 2026-10-21.
[Commit/qualification receipt](PRESET_INTEGRATION_2026_10_21.md). The dated observations below describe
the original pre-integration snapshot, including then-open gates and historical failures.

2026-10-21, `FabCAM-v1.1`, HEAD `1935878` plus existing uncommitted work.
User-requested UI follow-up `ivac-v0g58`. No commit/push for this intervention.

Removed Plunge and Evidence/status **table columns only**. The preset detail retains
both; evidence filtering also remains available. The table now has nine columns.
Initial column widths fit the available viewport (verified 922px content / 922px
scroll width in a 1280px browser). Manual widths remain adjustable with horizontal
scrolling available when needed. No cutting values or qualification states changed.

Column boundaries and the detail-pane divider support pointer dragging, arrow keys
(10px; Shift 40px), Home/End bounds and double-click reset. The right pane ranges
from 260px to a viewport-bounded maximum of 640px; default 340px. Width choices are
view state, not saved project data. The existing narrow-screen stacked layout is
retained. The shared Splitter gains optional controlled keyboard/ARIA bounds;
existing callers without a controlled value retain their mouse behavior.
The focusable ARIA separator uses a documented Svelte checker suppression for its
intentional keyboard interaction; it remains a real separator with position/bounds.

Isolated browser replay on port18813 verified nine headers/cells, preserved detail
information, column drag +60px, pane drag +60px, keyboard resizing, min/max bounds,
reset, and details hide/reopen. [Screenshot](presets-layout-2026-10-21-evidence/presets-resizable.png).
[Typecheck](presets-layout-2026-10-21-evidence/typecheck.log): zero errors/warnings;
[preset projection tests](presets-layout-2026-10-21-evidence/preset-library-tests.log):
6 PASS; scoped ESLint/Prettier pass. No new tests for this reversible presentation edit.

Concurrent PRESET-3 implementation appeared during this work, including updates to
PresetsLibrary, the shared catalog/schema and FeedsSpeedsSection. Those additions
were preserved; no source/application semantics were changed by this UI task.
The final [full-suite snapshot](presets-layout-2026-10-21-evidence/concurrent-full-suite.log)
is **1492 PASS, 1 existing skip, 3 FAIL** across169 files, plus WASM guard10 PASS.
Failures belong to the parallel FeedsSpeedsSection size (1133/1000) and its ongoing
i18n reference/allowlist changes. No guard budget/test was changed to hide them.
This does not claim the concurrent tree is globally qualified. No Rust/CAM/schema
modifications or production build were performed for this layout-only follow-up.
