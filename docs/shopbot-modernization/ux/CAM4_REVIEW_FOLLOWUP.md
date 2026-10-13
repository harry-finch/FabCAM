# CAM4 review follow-up — playback and 3D visibility

Date: 2026-10-13. Engineering ivac-ubcr; CAM4 A/B human gate ivac-tvox remains
pending. User reported repeated BigInt errors while scrubbing, inefficient pocket
retracts, warning-chip alignment, missing 3D path visibility and FS coverage gaps.

## Corrections

Rust WASM `nearest_checkpoint` returns i64; the actual generated binding declares
bigint. The driver previously annotated number and passed a bigint replay base to
Math.min. Normalize that return at the simulator boundary, preserving -1 sentinel
and uint32 index range; invalid results fall back to full replay. Regression tests
exercise real bigint values through backward replay planning. No Rust checkpoint,
heightfield, geometry or toolpath math change.

Warning chips now use inline flex with aligned icon/text and a consistent gap;
SVG glyph wrappers no longer rely on baseline layout. A compact Toolpaths checkbox
controls generated path lines and arrows in 3D, respecting wireframe/both mode.
Stock, cutter and simulation remain active. View state is transient, never saved
in the machining project. Visibility has its own scene effect; hiding paths does
not rebuild path buffers or reconfigure simulation. Initial rebuild seeds the
current visibility without depending on future checkbox changes.

## Continuous pocketing audit

Cascade intentionally emits distinct contour blocks with retract/reposition.
Existing Spiral stitches roughing rings where boundary/island guards admit bridges;
otherwise it falls back to separate rings. A wall/finish contour can remain
separate, so do not promise one uninterrupted move for every pocket. Added compact
strategy guidance; no silent strategy change or new bridge mathematics.

Four existing Spiral unit tests pass, including continuous-chain and island/
nonconvex fallback tests. On the actual software 40mm square with Ø3 cutter and
two depth passes, browser CPS output changed from13 offsets/168 moves to2 offsets/
135 moves when explicitly selecting Spiral. Independent listing counts: J2=13→2,
JZ=16→5 (including program positioning/retract commands). No physical performance
claim. CAM4 A/B residual analysis still explicitly excludes Spiral.

## Feeds & Speeds answer

The currently enabled reference pack is narrow ReferenceData: Sienci LongMill
context, exact Ø3.175/6.35 single-flute cutters, the source's softwood/soft plywood/
MDF material identity and full-slot conditions/DOC. Metric Ø3/6, two flutes,
generic wood.softwood identity, Pocket engagement/depth and unknown FS machine/tool
identity are not interchangeable with those references. Material label alone does
not make a record compatible. Matching/math/data stay unchanged; no invented
cutting data, manufacturer import or Sorbonne Tested/Validated promotion. FS
expansion remains frozen. Common metric pocket coverage is a future data gap.

## Evidence

Frontend1112 passes /1 optional skip in120 files; check zero errors/warnings,
lint/fmt and complete frontend build pass. No Rust code changed; four targeted
existing Spiral tests pass. Actual browser Ø3 Cascade and Spiral repeated backward/
forward scrubs with checkpoint-scale paths had no console/UI errors. Checked
Toolpaths off/on on the final build with stock/simulation intact.
[Hidden-path proof](CAM4-review-hidden-paths.png) is visual evidence. No new desktop
app bundle, physical test, project schema or post change. Refresh the user's page
to load the updated asset bundle before repeating the test.
