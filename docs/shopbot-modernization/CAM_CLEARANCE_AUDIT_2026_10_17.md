# CAM clearance / lifecycle audit — 2026-10-17

Tracker `ivac-tnkdu`. Local software implementation; human review pending.
No commit, push, release, installer rebuild or physical qualification.

## Decisions and changes

| Audit point | Disposition |
| --- | --- |
| Ray starts outside occupied grid | The existing backstop returns **all** segments, so a long approach is a performance fallback, not lost geometry. No false-negative counterexample was established from the supplied scenario. Retained the conservative fallback. Exact cell-corner crossings now include both side cells (supercover). Added external/far/diagonal/near-border oracle comparisons. |
| Growing V-Carve knots | Remove interior nodes only when XY, target and cut are all identical under the same affine f64 interpolation, with zero added tolerance. Keep genuine tiny breakpoints. Limit intermediate sweep knots as well as emitted points. Do not repeatedly spend a 1e-11 approximation budget across many sweeps. |
| Invalid selected indices | Use checked lookup and return an explicit index error; no panic. |
| Nearly flat arc cancellation | If sagitta <= 0.002 mm, the chord satisfies the existing curve budget. Otherwise use the stable equivalent `4*asin(sqrt(error/(2*r)))` instead of subtracting a tiny number from one before acos. Reject invalid radii. Preserve edge/subdivision budgets. |
| Coordinate threshold | Existing threshold is 1e6 **mm**, i.e. 1,000 m, not 1 m. Retained and made explicit in the error. No new large-coordinate qualification claimed. |
| Clearance all-pairs cost | Introduce a bounded segment-box grid broad phase; keep the identical piecewise quadratic narrow-phase proof over candidates. Use the maximum nonnegative effective cutter radius over the interpolated segment. Include a numerical expansion of 1e-7 mm + 64 ulps of coordinate/radius scale. |
| Trochoidal diagnostic leakage | Operation-scoped RAII around offset construction clears stale top-level entries and cleans on every exit, including cancellation/error/unwind. Multiple regions accumulate; nested scopes restore their parent's events. Keep legacy public take/emitter APIs compatible. No claim that the legacy kernel is now pure. |
| Repeated tab flattening | Cache tessellated points, segment lengths and total length locally per referenced object. Preserve placement order, closed/open semantics and per-tab overrides. |
| Drop-cutter pruning | Sort footprint by ascending offset; prune using the actual field maximum including off-grid stock top. Use strict comparison and original tie order to preserve signed-zero bit patterns. Positive field values remain supported; do not assume all callers are bounded by z=0. |

## Safety and performance boundaries

The broad phase stores at most 500,000 segment/cell references. Oversized storage
falls back to a bounding-box scan. Queries over more than 50,000 cells use the same
safe fallback. Each operation retains a 20-million candidate-pair analytical proof
budget and a 50-million broad-phase work budget (cells/references visited). These
are explicit failures, not successful incomplete operations. Worst-case complexity
is still potentially O(P*E); neither constant time nor a 100–1,000x speedup nor the
absence of future budget errors is promised. No timing benchmark performed.

The retained analytical proof checks whole interpolated segments, including the
stationary minimum of distance-squared minus cutter-radius-squared. Broad-phase
filtering does not replace it with point samples. Differential tests compare
accelerated and all-edge proofs for intersecting/clear paths and islands. A
5,000-edge / 5,000-point distant-boundary fixture (25 million naive pairs) now
passes without spending that many analytical checks. Grid fallback coverage is
also tested against direct box overlap.

The trochoidal compatibility sink remains thread-local for existing low-level
callers; the pipeline owns its lifecycle through a non-Send scope guard. A future
API returning geometry and diagnostics together can replace it separately without
silently dropping earlier region warnings in this fix.

Pipeline discriminator **48 → 49** invalidates previous generated-result caches.
The stable-hash test is updated for that intentional version change. Saved cutting
values and project formats remain unchanged; no postprocessor-specific geometric
exception was introduced. The existing DMG does not contain these changes.

## Validation coverage

Checked indices; huge-radius near-straight arc; hundreds of V-Carve passes and
retained depth/XY breakpoints; broad-phase/exhaustive proof parity; grid storage
fallback; far/exact-corner ray probes; operation/nested/multi-region diagnostic
cleanup; repeated curved-contour tabs; flat/sloped/positive/negative-zero surface
fixtures compared bit-for-bit to the original unsorted exhaustive drop-cutter.

The global geometry-cache test-isolation issue `ivac-rt80e` remains open, so the
complete core suite runs serially. Final counts and logs follow below.

**HARD STOP:** human review before another checkpoint. Existing macOS clean-host,
Windows/Ubuntu and physical qualification gates remain independent and open.

## Final software qualification

- `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1`:
  **1,108 passed**, including all new regressions and existing Halfpipe/FS checks.
- `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test geom1_contract --test geometry_source_ux --test vcarve_qualification --test shopbot_m2_post --test cps_posts`:
  **71 passed** (23 + 7 + 10 + 27 + 4), including A1–A4 V-Carve qualification and
  decoded output/preview correspondence.
- `cargo fmt -p ivac-core` / `git diff --check`: PASS.
- [Evidence logs](cam-clearance-2026-10-17-evidence/). Stable-hash change is expected
  from version 49; the updated snapshot passes the final suite.
- No parallel-suite PASS, production frontend/WASM build, new installer, measured
  speedup, rendered UI review or physical machining qualification claimed.
