# ADR-0023 — executable publication guard

2026-10-18 · FabCAM 0.9.0-beta.2 · source pipeline61 · local, uncommitted.
Baseline commit: `2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b`.
Human review: **ivac-8371y OPEN**. No commit, push, installer build or publication.

## Implemented contract

The existing asynchronous pipeline now refuses executable success when the actual
preview path intersects the declared fixture envelope. The failure is a structured
error with optional `diagnostic_preview` containing only path, warnings and optional
setup side. No `gcode` field or executable text is returned in that diagnostic.
HTTP returns a client error; Tauri and WASM preserve the structured failure, including
edited-drawing generation. Progress streaming retains cancellation and never sends
`Done` for a refused collision. A back-side failure labels the back setup and
returns neither side's executable program.

Both write-through APIs refuse **all fixture-bearing requests**, including distant
fixtures, before writing any bytes. HTTP `/generate/gcode` rejects before a successful
status/body. Clients must use buffered `/generate` or asynchronous progress streaming.
CPS fixture checks require the bundled ShopBot decoded execution path; arbitrary CPS
recorded intent is not accepted as evidence of its actual output.

The collision check distinguishes known dimensions from unknown coverage:

- Cutter-tip footprint is checked against the actual fixture Z interval.
- An explicitly positive flute length extends the known cutter volume even if the
  holder is unknown. No drill-length default is used for publication validation.
- Declared shank length/diameter and valid holder dimensions extend the body bound.
  Missing, invalid or zero-height coverage does not become a fabricated dimension.
- Complete status requires flute length, explicit nonnegative free shank length,
  positive shank diameter and a positive finite holder shape. Otherwise
  `fixture_clearance_unknown` remains visible. Unknown alone does not ban generation.
- The previous conservative maximum-radius capsule/Z-interval method is retained.
  It can reject empty space within the bound; this is not exact swept-solid avoidance.
  Possible primary, finish and spot-drill tools are included when provenance is ambiguous.

The frontend converts rejected paths back into the drawing frame, clears older front
and back output, preserves unsaved edits, and presents the existing fixture diagnostics.
The local preview adapter has empty program text; export refuses that state regardless
of warning preferences. Export buttons and palette actions also block. Late/obsolete
responses remain rejected by request ownership/context. A fresh successful generation
replaces the diagnostic. Narrow-screen status now compares actual generation inputs,
not the saved-file dirty flag, so unsaved diagnostics are not falsely marked stale.
English/German fixture messages explain the mandatory refusal; the old bypass copy
was removed. Saved projects, tool fields, history and Undo/Redo contracts are unchanged.

## Validation

Final combined results: **1204 core, 15 focused SAFE-Z, 14 HATCH/clearance,
27 ShopBot, 8 CPS guards, 18 HTTP, 24 native WASM, 3 focused native Tauri,
1367 frontend + 10 packaging checks PASS** (1 existing frontend skip).
[Machine-readable evidence and source hashes](adr0023-2026-10-18-evidence/checks.json).
The evidence manifest was verified against file bytes.

- Core full suite, SAFE-Z focused cases, HATCH/stock clearance, ShopBot regressions.
- HTTP buffered error and pre-header direct refusal; native WASM and Tauri structured
  diagnostic preservation. Native bridge tests do not qualify browser WASM or a packaged
  desktop runtime.
- Frontend full suite, typecheck and lint; schema/code generation; Rust Clippy and format.
- TEST1 bounded **256/256 PASS**, 27.48 seconds, unchanged complexity/error budgets.
- Live local HTTP UI: example profile crossing a box fixture; generation refused,
  preview retained, fixture and incomplete-body diagnostics readable. Narrow viewport
  reviewed. Screenshot in the evidence directory. Desktop button/palette policy tested
  in code; no claim of a new packaged application qualification.

Tests cover zero-byte raw/preview streams, known cutter and full-holder collisions,
partial body metadata, common-frame translation, edit/restore with cached inputs,
progress refusal/cancellation, prior-output replacement and preference-independent
export. Existing analytic sloped-body and output uncertainty tests are retained.

Intermediate findings: parallel core tests raced shared geometry-cache counters;
serial execution is the established validation mode. Pipeline61 changed the expected
cache-key version hash. The old ShopBot fixture test depended on the unknown-height
footprint heuristic; it now declares a real flute length to assert a known-volume
collision. Its other pocket/frame assertions are preserved. Frontend dead-copy and
component-size guards required removing obsolete bypass text and extracting shared
error/dialect helpers; no thresholds were relaxed.

## Limits and handoff

Software evidence only. Unknown initial machine pose, undeclared fixtures/dimensions,
controller behavior, non-milling processes and physical clearance remain unqualified.
Existing installers are still pipeline54. No automatic route avoidance, Z raising,
stock-removal safety certification or new ShopBot commands were introduced.

This implements ivac-h51gq; it does not automatically close full SAFE-Z/TEST1 D2,
resource-domain follow-up ivac-53ky9, platform installation or physical gates.

Human UX check: create a fixture across a contour and Generate; expect a blocking
message with visible path and no export. Move the fixture away and regenerate; the
collision disappears (missing-body warning may remain). Undo the fixture move and
regenerate; refusal must return. No machine execution is needed for this review.

**HARD STOP — await human review before another checkpoint.**
