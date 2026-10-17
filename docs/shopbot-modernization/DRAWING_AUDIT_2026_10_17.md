# Drawing audit — fifth lot — 2026-10-17

Tracker `ivac-g74he`. Applies on top of the locally validated offsets lot
(pipeline 50); source package remains 0.9.0-beta.2. No commit, push, installer
rebuild or release. HARD STOP for human review. Software qualification only.

## Audit disposition

1. **Arc rejection confirmed and corrected.** Overlapping bounding boxes no
   longer imply rejection of every curve. Existing cavalier_contours analytical
   line/arc and arc/arc intersection routines resolve candidate pairs. Only
   endpoint contacts of adjacent edges of the same contour are allowed;
   overlaps, non-adjacent contacts and crossings still refuse generation/repair.
   A two-semicircle circle and a mixed line/arc contour pass. An intact circle
   also generates through the authoritative Drawing API.

2. **Exact closure is intentional, not silently replaced by fuzzy closure.**
   GEOM1 generation already explicitly rejects a disagreement between the CAM
   fuzzy chainer and authoritative edited membership/closure. Keeping the CAM
   `closed` flag would silently bless unrepaired endpoints and contradict that
   contract. `Evaluated.closed` now additionally requires exact internal
   continuity. Tiny gaps print scientific notation instead of `0.000000000 mm`,
   with explicit repair guidance. `connected` stays exact. Original coordinates
   remain unchanged; curve gap repair/simplification is still unsupported.

3. **Global simplification restrictions confirmed and corrected; panic not
   reproduced.** Previously the identity comparison and drawing validation
   preceded the reported unwraps, protecting those accesses for valid inputs.
   The actual defects were global curve/point exclusion and whole-batch identity
   equality. Containment is now checked immediately around each simplification
   step, independently of preceding/following authorized repairs. A batch can
   close a different contour and delete a zero line before simplifying another
   contour. One simplification per contour per batch and protection of
   previously modified edges remain in force.

4. **Boundary witness ambiguity corrected.** Exact orientation plus finite-edge
   membership explicitly rejects a witness on an edge. Merely lying on an edge's
   infinite supporting line outside the segment does not trigger rejection.
   The half-open parity convention at equal Y already handled normal vertex
   alignment; it was not itself a general ray-casting failure.

5. **Transformation order already consistent.** Rust Drawing, CAM helpers and
   frontend file transforms all use scale → mirror X → mirror Y → rotate →
   translate around the frozen pivot. Added cross-checks for both mirrors,
   negative/positive scale, 30° rotation, translation, arc centers and bulges.
   No order or frontend matrix was changed.

6. **Snap collapse now refused before mutation.** A snap whose target is the
   edited edge's start reports an explicit collapse error. Previously final
   topology validation also refused the zero line; this was not silent successful
   acceptance. Modified-edge protection in Simplify remains intentional.

7. **Layer binding optimized.** One segment/membership pass selects contour IDs
   using borrowed layer names, followed by an ordered contour pass. Removed
   repeated full membership scans and `layer.to_string()` allocations. BTreeSet
   costs are O(S(log L + log C) + C log C), rather than O(C×S). No benchmark or
   speedup factor claimed. Adjacent-edge lookup in the topology sweep also no
   longer allocates and scans a contour-membership vector per candidate pair.

## Containment and numerical boundaries

Only the line contour changed by the simplification is used as a polygonal
container. All endpoints of all other unchanged segments serve as witnesses,
including isolated drill points and curved contours. Final complete topology
validation still rejects any intersection with surviving curves or lines.

For an open line chain, a **virtual closing chord used only in the proof**
protects the sliver swept by simplification. Its fixed endpoints are not moved;
it is not added to the drawing, not emitted, and does not change `closed`.
This also protects tiny-gap contours that previously escaped the closed-only
containment pass. Witnesses directly on the proof boundary cause conservative
refusal; some harmless open-chain simplifications can therefore remain refused.

Line orientation retains the existing exact dyadic fallback. Curve predicates
use the existing library's floating arithmetic and a 1e-9 mm positional
incidence tolerance, separate from user join/deviation tolerances. No curve
flattening is introduced. Degenerate/near-flat curves that the library would
classify as straight, inconsistent stored centers, and nonfinite computed arc
geometry refuse analysis. This is not an exact-arithmetic proof for arbitrary
floating-point curve inputs, nor general curve editing support.

The 2,000-segment topology and 200-segment simplification-suggestion budgets
remain. Serialization, original provenance, lineage, binding review rules,
manual settings and machine-output safeguards remain unchanged. Pipeline
version **50 → 51** invalidates old generated caches; the expected hash fixture
becomes `af92c62b07133d83`. Existing machining goldens are unchanged by this lot.

## Evidence

Before corrections: 3 of the first 5 regression tests failed as expected
(adjacent curves, unrelated curves blocking simplification, zero-looking gap).
The other two confirmed point/sliver refusal and ordered layer selection.

New integration suite: 13 tests PASS. Frontend file-transform suite: 11 PASS.
Full core: **1,123 PASS** in serial (84.35 s). Integration: **84 reported PASS**
(CPS 4 + drawing audit 13 + GEOM1 23 + geometry sources 7 + ShopBot 27 +
V-Carve 10). Formatting and whitespace checks PASS. Optional FANUC caveat below.

Commands from repository root (`RUSTC_WRAPPER=''`, offline):

```sh
cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1
cargo test --offline -p ivac-core --features cps --test drawing_audit --test geom1_contract --test geometry_source_ux --test vcarve_qualification --test shopbot_m2_post --test cps_posts
cargo fmt -p ivac-core -- --check
git diff --check
```

Frontend: `vitest run src/lib/state/file-transform.test.ts` (Node 24).
[Logs](drawing-audit-2026-10-17-evidence/).
Core tests run serially because of the previously recorded global-cache test
isolation issue `ivac-rt80e`. The optional upstream FANUC oracle file remains
absent; its test returns early and does not establish oracle qualification.

No frontend/WASM production build, visual UX acceptance, native package,
clean-host test or physical machining is claimed. Existing macOS/Windows/Ubuntu
release gates remain open. Await human review before another checkpoint.
