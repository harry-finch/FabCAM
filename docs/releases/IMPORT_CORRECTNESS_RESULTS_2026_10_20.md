# Import correctness — DXF OCS, BLOCK transforms, SVG/DXF units — 2026-10-20

Tracking **ivac-feder**. Bounded import checkpoint on the working tree based on
`298feb7`; software evidence only. No commit, push, build or publication.
Units policy proposal: [ADR-0026](../shopbot-modernization/adr/ADR-0026-explicit-import-units.md)
(PROPOSED). **HARD STOP for human review.**

## 1. Findings

Independent fixtures and analytic expectations:
[`tests/fixtures/import-correctness/generate.py`](../../tests/fixtures/import-correctness/generate.py)
implements the DXF Arbitrary Axis Algorithm and INSERT semantics in Python,
independently of the Rust importer; [`check.py`](../../tests/fixtures/import-correctness/check.py)
compares `ivac import` output segment by segment (lines, arc centres/sweeps and
direction, circles, sampled ellipse points). Before the fix: **13/38 pass**
([`before.json`](import-correctness-2026-10-20-evidence/before.json)).

| Finding | Verdict | Root cause |
| --- | --- | --- |
| OCS normal `(0,0,-1)` ignored | **Confirmed** | ARC/CIRCLE/LWPOLYLINE/2D POLYLINE/TEXT/INSERT read OCS coordinates as WCS: mirrored X, reversed arcs/bulges. `(0,0,1)` was correct. |
| ELLIPSE with `-Z` normal | **Confirmed** | Minor axis hard-coded as `Z × major`; DXF uses `normal × major`, so partial ellipses swept the wrong side. |
| Tilted / zero normals | **Confirmed** (silent) | Imported as if in XY; now refused (see contract). |
| BLOCK base point ignored | **Confirmed** | INSERT used `location + R·S·p`; DXF is `location + R·S·(p − base)`. Rotated, scaled, nested and repeated inserts were all offset. |
| INSERT extrusion ignored | **Confirmed** | Insertion point and rotation live in the INSERT's OCS. |
| Mirrored INSERT (negative scale) | **Refuted** | Already correct since the 2026-10-17 input lot. |
| MINSERT arrays | **Confirmed** (silent) | `column/row_count` ignored: one copy imported instead of the array. |
| SVG “1 px = 1 mm” | **Confirmed, deliberate** | usvg runs with `dpi = 25.4`: physical units exact; `px`, unitless, `%` and viewBox-only sizes are user units imported 1:1 as mm. CSS says 1 px = 1/96 in. |
| DXF unitless | **Confirmed, deliberate** | `$INSUNITS` absent/0 → 1 unit = 1 mm, silently. |
| Import warnings visible to users | **New finding** | The frontend never displays `ImportOutput.warnings` (existing or new). |

## 2. Coordinate and units contract (now enforced)

- **OCS**: for a unit normal with `|N_xy| ≤ 1e-9` (`OCS_PARALLEL_TOL`, absorbs only
  float noise), `+Z` is the identity and `-Z` maps `(x, y) → (−x, y)` (Arbitrary
  Axis Algorithm), also reversing arc direction and bulge sign. Any other normal,
  zero or non-finite, is **refused** with the entity type and layer; OCS
  elevation is ignored as for every 2D import.
- OCS entities: ARC, CIRCLE, LWPOLYLINE, 2D POLYLINE, TEXT (position; a mirrored
  baseline keeps its true direction, the editable text itself is not mirrored),
  INSERT. WCS entities unchanged: LINE, POINT, SPLINE, MLINE, MTEXT, 3D
  POLYLINE/mesh. ELLIPSE: WCS centre/major axis, minor axis = `normal × major`.
- **INSERT**: `parent ∘ OCS_insert ∘ T(location) ∘ R(rotation) ∘ S(sx, sy) ∘ T(−base)`,
  nested recursively; an entity's own OCS is applied first in block space.
- **MINSERT** (`column_count` or `row_count` > 1) is refused, not guessed.
- **Units**: unchanged values; DXF without units and SVG without physical
  `width`/`height` now return an explicit warning stating the 1 unit = 1 mm
  assumption. An explicit import scale silences it.
- Exact geometry kept: similarity transforms keep circles/arcs (bulge, centre);
  non-uniform ones keep the existing 0.01 mm sampled budget; endpoints exact.

## 3. Corrections and tests

[`dxf_in.rs`](../../crates/ivac-core/src/input/dxf_in.rs): `ocs_transform`,
per-entity OCS composition, base-point translation, INSERT extrusion, MINSERT and
tilted-normal refusal (`refusal` → `Error::bad_input` with a hint), ELLIPSE normal
sign, mirrored TEXT baseline, DXF unit warning.
[`svg_in.rs`](../../crates/ivac-core/src/input/svg_in.rs): root `width`/`height`
inspection and warning (geometry untouched).

Tests: [`tests/import_correctness.rs`](../../crates/ivac-core/tests/import_correctness.rs)
runs all 38 analytic cases (OCS ±Z, scaled normal, refused normals, LINE stays
WCS, ellipses, base point, rotation+scale, mirror X, mirror Y+rotation, nested,
repeated, INSERT OCS, entity OCS inside a block, combined OCS, MINSERT, DXF and
SVG units, transforms/nested groups, Inkscape/Illustrator-style headers) and
asserts no unexpected warnings; explicit scale silences unit warnings; unit test
for TEXT under `-Z`. After the fix: **38/38**
([`after.json`](import-correctness-2026-10-20-evidence/after.json)).

## 4. Impact on existing sources

Saved projects store imported segments: nothing changes on open. A file is
re-read only on explicit import, Tauri source reload or reimport; a changed
source changes the generation context key and the per-op cache key, so dependent
output is invalidated. GEOM1 drawings with accepted repairs still refuse source
replacement. No migration; **pipeline version unchanged (63)** — the pipeline,
posts and caches are untouched.

[`impact.json`](import-correctness-2026-10-20-evidence/impact.json): 53 real files
(V1 validation kit regressions, ShopBot M1 fixtures, PERF1, HATCH evidence, logo
SVGs) give **byte-identical geometry** before/after; only files with ambiguous
units gain the new warning. No representative SolidWorks/Fusion/AutoCAD export
was available: those remain unverified beyond the synthetic DXF semantics.

## 5. Regressions

| Gate | Result |
| --- | --- |
| `cargo fmt --check`; Clippy strict `--all-features`; pedantic gate | pass |
| `cargo xtask schema-check` | pass (no wire change) |
| `cargo test --workspace` | **1604 passed, 0 failed** |
| CI `--all-features --exclude ivac-tauri` | **1599 passed, 0 failed**, 1 ignored (harness-only TEST1) |

Included: import correctness 2, importer unit tests, drawing audit 13, GEOM1 23,
geometry sources 7, ShopBot M2 27, CPS 5, V-Carve 10, SAFE-Z 14+15, provenance 12,
comment safety 1, FANUC goldens 12. `golden_corpus` skips without the optional
`refs/` corpus. Frontend unchanged and not re-run.

## 6. Limitations and remaining V1 risks

- Units: the 1 unit = 1 mm assumption stays the default and **its warning is not
  shown in the UI** (nor any importer warning) until ADR-0026 is decided.
- Tilted-plane entities and MINSERT arrays are refused rather than interpreted.
- HATCH recovery (raw scanner) applies neither OCS nor INSERT context
  (existing follow-up ivac-aoany). SPLINE/LINE with Z variation are projected.
- MTEXT keeps its legacy rotation approximation; mirrored text is not mirrored.
- No real third-party CAD export was tested.
