# Input audit — sixth lot — 2026-10-17

Tracker `ivac-9rv3y`. Local corrections above the fourth and fifth audit lots;
source version remains 0.9.0-beta.2. No commit, push, installer rebuild or
publication. Software qualification only; HARD STOP for human review.

## Disposition of the six findings

1. **HATCH metadata desynchronization reproduced.** After the recovery pass,
   rebuild the per-segment object index and object metadata together with bounds
   and layers. Recovered segments now resolve BYLAYER colors from the layer
   table, apply color-layer naming, then apply the same resolved-layer whitelist
   convention as other DXF entities. A recovered square has four memberships
   and one object; excluding its layer removes it. This restores the metadata
   contract used by selection and drawing/relief requests, without UI changes.

2. **NURBS degree underflow reproduced.** Guard `degree >= cps.len()` before
   knot sizing/de Boor evaluation, including `usize::MAX`. Retain the existing
   control-polygon fallback convention; DXF import explicitly warns that an
   invalid-degree spline was replaced by its control polygon. This is not a
   claim of accurate reconstruction of malformed geometry. Valid NURBS retain
   their existing fixed sampling policy, outside this correction's error budget.

3. **Mirrored DXF curves reproduced.** Transform original ARC endpoints with
   the composed affine matrix and reverse bulges when its determinant is
   negative. Both polyline variants use the same edge implementation. Similarity
   transforms preserve circular arcs and circles; nonuniform transforms use
   bounded line approximation instead of inventing a circle at a mean radius.
   ELLIPSE major and minor vectors both receive the full linear transform,
   including nested rotation/nonuniform scaling. Original polyline endpoints
   remain exact after transformation, and full-circle endpoints close exactly.
   Editable TEXT/MTEXT's legacy size/rotation approximation is not changed.

4. **SVG angle/length confusion reproduced.** A 300 mm straight cubic formerly
   became 512 segments; it now produces one. Adaptive de Casteljau uses a
   **0.01 mm** control-hull distance bound to the **finite** endpoint chord in
   transformed millimetres. This also preserves collinear backtracking and
   loops that an infinite-line flatness test would erase. Depth 20 is a resource
   limit: an unresolved leaf rejects the import rather than relaxing tolerance.
   Short nonzero chords are retained to avoid introducing open boundaries.

5. **SVG font implicit commands and closure reproduced.** Support repeated
   tuples, M-to-L transition, relative m/l/a, Z/z closure and cursor reset.
   Malformed/nonfinite numeric tokens and invalid arc flags now fail rather
   than being silently discarded. The loader still supports only M/L/A/Z;
   this is not a full SVG path/FontForge font implementation. Bundled font tests
   remain part of the core suite.

6. **HATCH pair desynchronization reproduced.** Consume code/value lines as
   pairs even when the code is malformed. Circular angles use bounded modular
   reduction, with finite-value checks. Full circles split into two finite
   semicircles instead of a coincident-endpoint, near-infinite bulge. Both
   clockwise and counterclockwise extreme-angle cases are exercised.

## Numerical and compatibility boundaries

For a transformed parametric ellipse `p(t)=c+u cos(t)+v sin(t)`,
`|p''| <= |u|+|v|`. Chord interpolation error is at most
`(|u|+|v|) * delta_t² / 8`. Select the segment count from this bound at **0.01 mm**
after unit conversion and the complete affine transform. A nonfinite result
or a count exceeding 65,536 rejects the DXF import. Circular arcs under a
similarity transform retain their exact bulge representation instead.

The SVG 0.01 mm budget applies to the Béziers delivered by usvg after its
parsing/normalization; it does not certify source conversion or f32 rounding
for arbitrarily large coordinates. DXF/SVG import error is distinct from the
existing 0.002 mm V-Carve preparation budget. No downstream clearance tolerance
or ShopBot protection was relaxed. Font arc sampling and valid NURBS sampling
are unchanged and are not covered by the new curve-import bound.

Follow-up `ivac-aoany` tracks block-aware HATCH recovery: the raw scanner has
no INSERT transform context and may recover definition coordinates. This
requires a separate fixture-backed correction; it is not claimed fixed here.
HATCH recovery remains a limited raw-text extractor: ellipse/spline boundary
edges, binary HATCH and block-aware HATCH expansion are not newly qualified.
Neither `intern.rs` nor text rendering architecture required a change. Existing
saved geometry is not automatically reimported or migrated. Newly imported
geometry and SVG-font rendering can change as described above.

Pipeline **51 → 52** invalidates prior generated caches. The intentional hash
fixture becomes `fec63c9c88fad786`. No machining golden is updated by this lot.

## Validation

The seven initial regressions all failed before the fixes. The expanded
regression group contains 15 tests: metadata/filtering, degree bounds/fallback
warning, mirror endpoints/bulges, nested affine circle/ellipse sampling,
impossible budget rejection, finite-chord Bézier checks, font path semantics,
malformed tokens, paired parsing and bounded HATCH arcs.

Final core suite: **1,138 PASS**, serialized, 82.62 s, including all 15 new
regressions. Integration: **84 reported PASS** (CPS 4 + Drawing audit 13 +
GEOM1 23 + geometry sources 7 + ShopBot 27 + V-Carve 10). The optional FANUC
oracle caveat below applies. Formatting and whitespace checks PASS.

Commands from repository root with `RUSTC_WRAPPER=''`:

```sh
cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1
cargo test --offline -p ivac-core --features cps --test drawing_audit --test geom1_contract --test geometry_source_ux --test vcarve_qualification --test shopbot_m2_post --test cps_posts
cargo fmt -p ivac-core -- --check
git diff --check
```

[Evidence](input-audit-2026-10-17-evidence/). Core suite is serialized because
of the pre-existing global-cache test isolation issue `ivac-rt80e`. The optional
upstream FANUC oracle remains absent; an early-returning test is not oracle
qualification. No visual UX, frontend/WASM production build, native installer,
clean-host or physical machining validation is claimed. macOS clean-host and
Windows/Ubuntu acceptance gates remain open. Human review precedes any next
checkpoint or distribution work.
