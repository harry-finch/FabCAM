# ADR-0026: Explicit import units for unitless DXF and non-physical SVG

Status: **AUTHORIZED for bounded implementation (2026-10-20); implemented locally — human UX/architecture acceptance pending**
Date: 2026-10-20
Scope: import correctness checkpoint ivac-feder; evidence in
[import correctness results](../../releases/IMPORT_CORRECTNESS_RESULTS_2026_10_20.md).

## Context

Verified on the real parser path (usvg 0.45 with `dpi = 25.4`):

- SVG `width`/`height` in `mm`, `cm`, `in`, `pt`, `pc` import at their true size.
- SVG in `px`, unitless, `%` or with only a `viewBox` imports **1 user unit =
  1 mm**. CSS defines 1 px = 1/96 in (0.2646 mm); Illustrator-style files use
  72 px/in. An A4 page exported as `595.28px × 841.89px` imports 595 mm wide
  (2.83×); an Inkscape px document imports 3.78× too large.
- DXF with `$INSUNITS` absent or 0 imports 1 drawing unit = 1 mm.

The current convention is deliberate (`svg_in.rs`, DXF parity) and saved
projects store imported segments, so it cannot be changed silently. Since
2026-10-20 the core emits an explicit `ImportOutput.warnings` entry for these
cases, but **the frontend does not display any importer warning today** — not
this one, nor existing ones (unsupported entity, missing block, invalid
spline). The user therefore still cannot see the assumption.

## Proposed decision

1. **Surface importer warnings** in the import flow (toast or import summary),
   for every warning the core already returns. No geometry change.
2. **Explicit unit choice for ambiguous sources**: when the core reports an
   ambiguous size, show the measured bounding box under the current assumption
   and offer *1 unit = 1 mm* (current), *96 px/in (CSS/Inkscape)*, *72 px/in
   (Illustrator/PDF)*, *inch* or *custom*. The choice sets the existing import
   scale (`ImportOptions.scale`) and is recorded with the import entry, so
   reimport/reload reproduces it.
3. **Default unchanged** (1 unit = 1 mm) until a human decides otherwise;
   physical-unit SVGs and DXFs with `$INSUNITS` stay silent.

## Alternatives considered

- Switch SVG px to the CSS 96 dpi default: standard-conforming, but changes the
  size of every existing px/unitless import on reimport and contradicts current
  documented DXF/SVG parity; still wrong for 72 px/in exporters.
- Refuse ambiguous files: safe but blocks common workflows (laser/vinyl SVGs).
- Warning only (current state): honest in the API, invisible in the UI.

## Consequences

Frontend and i18n work (EN/DE), an `ImportEntry` field for the chosen scale and
project-format compatibility (absent ⇒ current behaviour). Saved projects keep
their stored segments; only new imports or explicit reimports use a choice.
Generated output is invalidated through the existing generation context key.
No pipeline or post change; no ShopBot-specific logic.

## Validation

Fixtures `svg_px`, `svg_unitless`, `svg_viewbox_only`, `svg_illustrator_pt_as_px`,
`dxf_units_absent`, `dxf_units_unitless` (tests/fixtures/import-correctness) with
each choice; save/reopen/reimport round trip; visible warning UI review.
**HARD STOP: human decision required before implementation.**

## Implementation record (2026-10-20, ivac-feder)

Results: [import units and warnings](../../releases/IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md).

**Proposed vs implemented.** Both proposals are implemented. Differences: the
summary opens *before* the source is committed (so a cancelled import changes
nothing and DXF text entities are created at the chosen size); it opens only for
assumed units or importer warnings, not for clean physical/declared imports. The
proposed separate `ImportEntry` field was not needed: the single authoritative
record is the stored importer result — `unit_scale` plus the new
`unit_status`/`unit_source` — so there is no second scale representation.

**Persisted user decision.** Confirming the summary re-imports the same bytes or
path with an explicit `ImportOptions.scale` (also for 1 unit = 1 mm), producing
`unit_status = explicit`. Save/reopen keeps it; Tauri source reload passes the
same scale; other statuses reload without one. The importer refuses an explicit
scale on an SVG with a physical size (no double conversion) and any scale that is
not finite within 10⁻⁶–10⁶ mm per unit.

**Remaining assumptions.** Default stays 1 unit = 1 mm; nothing infers 72 dpi.
Sources saved earlier read as `unknown` and keep their geometry and scale.

**Still unsupported.** Changing the units of an already committed source (re-open
or re-add the file instead); a browser session cannot re-read a saved source.
MINSERT, tilted entities and HATCH-in-block remain refused or limited.

