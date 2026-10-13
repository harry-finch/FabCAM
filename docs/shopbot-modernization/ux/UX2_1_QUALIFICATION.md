# UX2.1 — Direct Manipulation & Collision Feedback

Date: 2026-10-13. Engineering ivac-yfg8; human gate **ivac-svnf** pending.
The user explicitly requested this small follow-up to completed CAM2/UX2.
CAM2 gate ivac-97fb is accepted; its original qualification record is unchanged.
No CAM3, Feeds & Speeds change, collision-math change or persistence migration.

## Error-message fix

Actual failure paths: GenerateBar's catch and file_ops.reportError converted
non-Error transport objects with String(input), losing their message and producing
[object Object]. api/error-notification.ts now whitelists structured backend errors,
readable Error/string/message fields, and a neutral recovery fallback. ProjectState
normalizes notification input defensively; WASM worker serializes only normalized
structured errors. Shared parser retains localization code and scalar params.
Legacy file-operation catches use the same message conversion. Regression tests
cover direct/JSON backend errors, malformed objects and nested message objects.
An actual refused nonzero-Z generation also displayed its readable backend message
with no object-coercion text. No arbitrary object coercion or blind diagnostic JSON
becomes a normal toast.

## Collision visual hierarchy

A persistent compact red status row names colliding fixtures and states whether
the existing critical-warning setting blocks export. If that setting is disabled,
the row says so rather than claiming blocking. Info/clean/stale/warning/critical
chips use small Lucide SVGs and existing semantic tokens. Fresh critical fixtures
have red outlines; a selected colliding fixture retains a distinct dashed/thicker
outline. Selected fixtures paint last. No giant card or automatically opened
floating warning panel. Detailed diagnostics remain under the warning chip.
Collision candidates retain UX2 conservative height/unknown-holder limitations;
this presentation is not physical clearance certification.

## Show collision

Select the first identified collision fixture, reveal Stock inspector, switch to
2D and frame its declared bounds with surrounding context. Framing uses the live
container after revealing 2D, avoiding hidden zero-sized canvas measurements.
Pipeline candidates provide fixture/op identity, not an exact contact point:
no invented marker or allegedly exact path-segment highlight is added. Associated
warning text remains in the compact row; detailed warnings remain accessible.
Repeated Show works for the same fixture without moving model coordinates.

## 2D selection and dragging

Reuse box/disc/local-polygon hit tests and the existing selection reducer.
An already selected fixture wins an overlap; otherwise first matching fixture in
project order wins deterministically. Clicking selects; dragging an already
selected fixture moves only its drawing-frame XY origin. Hover uses pointer/grab,
active drag uses grabbing. The 3 CSS-pixel threshold is interaction tolerance,
not machining tolerance or snapping. Numeric editing remains the precision and
keyboard-accessible interface. Direct dragging is mouse/pen in 2D; existing touch
and other geometry/tool-marker interactions are preserved, not expanded here.

## Synchronization, history, persistence and invalidation

FixtureGesture writes the existing command target during drag, not a shadow
render position. The captured pointer transform prevents feedback/layout reflow
from changing the drag's coordinate delta. Release creates one updateFixtureCommand
with no cross-drag coalescing; no pointer-move history entries. Undo restores the
initial origin; Redo restores final origin. Click-only creates no history entry;
Escape/pointer cancel/lost capture roll back the live change. Numeric updates use
existing commands and both 2D/3D read the same fixture records.

Existing project snapshot/storage fields persist dragged positions. Z, dimensions,
local polygon vertices, work zero, stock, tools, operation geometry and rates are
unchanged by dragging. Existing generationContextKey includes fixture coordinates:
live motion makes previous export/collision validation stale, even after Save.
Fresh collision highlight/message is removed rather than asserted as current.
Regenerate explicitly refreshes the verdict; export rules are unchanged.

## Tests and actual browser evidence

| ID | Evidence | Result |
| --- | --- | --- |
| UX21-01 | Direct and encoded structured errors, readable strings/Error, malformed/nested objects and object-coercion sentinel | PASS |
| UX21-02 | Actual core collision setup: 7 warnings, 1 critical; compact row names Clamp 1, says Export is blocked, Download refuses | PASS |
| UX21-03 | First Show from 3D reveals 2D, selects Clamp 1, frames it; red selected outline and numeric inspector visible | PASS |
| UX21-04 | Real mouse box drag changes origin 7/60 → -25.974910394265258/60; Undo restores 7/60, Redo restores dragged XY | PASS |
| UX21-05 | Actual Save/reopen retains all three edited fixtures; saved workOffset, stock, imports, operations and tools match pre-drag records exactly; Z/kind unchanged | PASS |
| UX21-06 | Real cylinder mouse drag changes Y 15 → 32.82531194295896; polygon click selects in canvas, drag changes XY to -4.304812834224629/21.060606060606045; Undo/Redo preserves polygon-local vertices | PASS |
| UX21-07 | Regenerate detects the newly moved polygon and names Fixture 3; numeric correction to X=-20 invalidates old verdict then clears collision on explicit regeneration | PASS |
| UX21-08 | Pure gesture uses actual command target/History: 100 moves create one release command; box/cylinder/polygon, cancellation/no-op and separate successive drags | PASS |
| UX21-09 | Hit tests include all shapes and overlap-selected precedence; red+dashed painter test; pure frame math centers declared fixture bounds | PASS |
| UX21-10 | Existing generation-context/critical-warning tests and byte-identical actual recovered SBP versus UX2 preserve behavior | PASS |

Frontend suite: **119 files, 1,106 passing / 1 optional skipped**. Svelte/TypeScript
check: zero errors/warnings; ESLint/Prettier and complete frontend/WASM freshness
build pass. Core/server builds with CPS; no Rust or post-processing behavior changed
in UX2.1. Native UX2 evidence remains in its original record; no new full native
suite or physical/native-bundle qualification is claimed.

Commands: npm --prefix frontend run test / check / lint / build; offline
cargo build -p ivac-server --features cps using /private/tmp/fabcam-ux21-target,
CARGO_INCREMENTAL=0 and debug symbols disabled. Root target/debug remains absent;
checkpoint scratch build is cleaned after verification.

[Collision feedback and focused inspector](UX2_1-collision.png),
[recovered 3D setup](UX2_1-recovered.png),
[readable backend error](UX2_1-error.png),
[saved software-only drag fixture](../UX2_1-software-drag.ivac-project.json).
Examples use the existing public ReferenceData software context, not ShopBot
calibration or cutting advice. Unknown holder/envelope warnings remain visible.

## Shared UI and deferred work

Professional Workshop UI + CAM UX/UI: current neutral inspector, existing spacing
and semantic tokens, small official @lucide/svelte icons, visible text/focus and
optional numeric access. Other requested skills guard architecture/geometry/tests
and documentation; no new icon family or fixture storage abstraction.

Snapping is deferred (ivac-t68s): grid, stock corners/edges, geometry bounds,
work zero and temporary modifier to disable snap. No stock resizing/drag, imported
SVG/DXF placement, CAD constraints or freely draggable work-zero marker was added.
A future explicit Set work zero → click workflow needs separate approval.
CAM3 tabs/leads remains the next major CAM milestone, waiting for human approval.
FS4 remains deferred/nonblocking; FS mathematics, UX/data/evidence remain frozen.
No commit/push or new desktop app bundle is authorized or claimed.

## 2026-10-13 — continuation approval

User explicitly validated and requested the next checkpoint after inspector fixes
and roadmap documentation. Human gate ivac-svnf closed; bounded CAM3 software
qualification authorized. This does not add physical evidence or authorize CAM4.
