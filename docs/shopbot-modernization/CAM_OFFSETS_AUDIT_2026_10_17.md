# CAM offsets audit — fourth lot — 2026-10-17

Tracker: `ivac-likhm`. Source baseline: `73dca6e` (0.9.0-beta.2).
Local changes only; no commit, push, package rebuild or publication.
Software qualification only. HARD STOP for human review.

## Findings and changes

| Audit item | Finding and implementation |
| --- | --- |
| Waterline clearance | Confirmed. Raw circular slices with R=10 mm and a Ø6 mm cutter generated centers beyond R−r=7 mm. `clear_level` now geometrically erodes boundary **and holes** before rasterization. Separate surviving components and holes are preserved; raster receives zero additional axis trimming. |
| Pocket raster inset | Confirmed directional rather than geometric **additional** inset. The first cutter compensation was already present, so the audit's general gouging claim does not follow from that second inset alone. Wall-pass raster now erodes the compensated center domain by one radius in every direction. No-contour raster keeps its existing single compensation. |
| Cut direction | Confirmed two-arc circles had zero chord-only area, and CCW inflated island walls were classified as inner walls. Area now shares the exact bulge/lens formula with object offsets. Pocket outer walls are normalized CCW and island walls CW before the existing one-time cut-direction pass. M3/M4 × climb/conventional exercised, including CW source geometry. No new serialized context field. |
| Bridges | Confirmed 7 interior samples missed the audit's 3 mm notch on a 40 mm bridge. Bridges now partition at **all** polygon-edge intersections, including vertex hits and collinear edge events, and classify each intervening interval. Same rule protects island interiors, including traversal exactly through island vertices. Unsafe bridges split chains / trigger existing cascade fallback. |
| Round joins | Both former 0.25 mm settings reduced to 0.001 mm sagitta, with Clipper precision 4 (0.0001 mm grid). Geometric erosion adds 0.0011 mm only when holes/reflex corners can create rounded chords. Convex outer polygons retain the requested exact erosion distance; the rectangular GEOM1 reference stays unchanged. |
| Waterline topology | Nesting precomputes bounds/absolute areas, rejects impossible containment pairs, computes each surviving relation once, and selects the smallest containing loop as parent. Checked slicing rejects non-degree-2 junctions and unclosed/degenerate chains. The production driver propagates a structured `PipelineError::Waterline`; no partial pipeline response is returned. |
| Approach point | Project onto line/arc, split at the projection, then rotate the closed path to that entry. Warning uses actual contour distance. Arc geometry, layer/color, source index, finish flag and XY tab anchors remain intact. Open strokes retain their entry order. |

The low-level `pocket_zigzag` diameter argument remains legacy scan-axis trim,
explicitly documented as **not** cutter compensation. Production paths use
`pocket_zigzag_inset` and zero trimming. The same geometry is consumed by
preview and post emission; no ShopBot-specific geometry correction was added.

## Numerical contract and limits

- Coordinates and clearances are in mm. Round-join sagitta 0.001 mm is separate
  from Clipper's 0.0001 mm grid and the bridge incidence tolerance 1e-9 mm.
- Rounded erosions have a conservative 0.0011 mm extra inset per offset step;
  this may leave slightly more stock, and can collapse a marginal thin region.
  Cascades retain their existing 4096-ring cap and diagnostic.
- The original island inflation padding remains. This work does not redefine
  the historical high-overlap island over-inflation policy.
- V-Carve's existing continuous clearance proof/error budget remains active;
  none of its acceptance thresholds were relaxed for these offset changes.
- The stitch grid remains 0.001 mm. Identical mesh edges retain canonical
  interpolation; nearby points across a cell boundary are not distance-welded.
  Ambiguous degree counts now fail explicitly. This is **not** a general mesh
  repair or self-intersection certification. Degree-2 contours still require
  the existing simple, non-crossing/laminar input assumption.
- The legacy Vec-returning waterline inspection APIs remain source-compatible
  and return empty on an invalid slice. Manufacturing uses the new checked
  Result-returning APIs and propagates errors.
- The nesting prefilter avoids point-in-polygon tests on disjoint bounds;
  worst-case nested families remain quadratic in loop pairs. No claimed 50×
  speedup or general performance benchmark.

## Validation evidence

Five new fixtures were run before correction: all five failed for the expected
reasons (Waterline circle, two-arc area, island orientation, thin notch, and
mid-side approach). The targeted final run passes 24 tests, including nine
existing audit tests and fifteen new tests for this lot.

Final core suite: **1,123 passed**, serial, 84.19 s. Formatting and whitespace checks pass.
Integration suites: **71 reported passed** (CPS 4, GEOM1 23, sources 7, ShopBot 27, V-Carve 10).
The optional upstream FANUC file is absent; that test returns early and does
not establish FANUC oracle qualification. No other test failure remains.

Commands (repository root, offline, `RUSTC_WRAPPER=''`):

```sh
cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1
cargo test --offline -p ivac-core --features cps --test geom1_contract --test geometry_source_ux --test vcarve_qualification --test shopbot_m2_post --test cps_posts
cargo fmt -p ivac-core -- --check
git diff --check
```

Pipeline version **49 → 50** invalidates prior toolpath/geometry caches.
The hash regression was updated deliberately to `cb98077d2bdad79f`.
The GRBL golden digest changed from `111f30f31bf89c56` to `8a3359e11295e3c3`.
A controlled replay with only the old area formula restored reproduces the old
digest. The textual diff consists solely of two G2/G3 direction corrections on
the two-arc profile, with identical coordinates, feeds and depths. The restored
old formula was removed immediately after this comparison. The GEOM1 ShopBot
reference artifact was **not** regenerated or altered.

The ShopBot pocket test golden was updated only after its independent SVG/DXF
program decoder passed depth, cutter-clearance and floor-coverage checks. It
now has 31,901 lines versus 31,733 (+0.53%), reflecting finer round joins,
conservative offsets and corrected island-wall direction. This is a test
fixture change, not a distributed machining program. Golden acceptance now
runs **after** semantic checks.

The V-Carve perimeter-island case previously expected an overcut refusal caused
by the coarse offset. It now passes the unchanged production clearance proof.
Its reference test checks raw square/island clearance at 101 XYZ positions per
segment and target-depth attainment on both loops; A4 additionally checks its
OpenSBP decoded output, preview parity and save/reopen. The sampled independent
oracle complements, rather than replaces, the existing continuous production
proof. No broad Engrave/V-bit export authorization was introduced.

Evidence: [logs and reviewed output diff](cam-offsets-audit-2026-10-17-evidence/).
The previously known parallel global-cache test isolation issue `ivac-rt80e`
is outside this change; full core qualification uses one test thread.

## Handoff

Human review is required before another checkpoint. Existing macOS DMG and
prior release evidence do not contain these changes. No new frontend/WASM or
native installer build, visual UX acceptance, clean-host installation or
physical machining is claimed. macOS clean-install and Windows/Ubuntu gates
remain unchanged. No automatic next milestone.
