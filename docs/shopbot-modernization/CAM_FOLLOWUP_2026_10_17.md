# CAM follow-up — chaining, Halfpipe witnesses and geometry cache

2026-10-17 · tracker `ivac-uns33` · local software checkpoint; human review pending.
User-supplied second CAM audit. No commit, push, publication or new native installer.

## Disposition

| Finding | Result |
| --- | --- |
| input.rs still reconstructs source membership | Already fixed by the preceding checkpoint using `segments_to_objects_indexed`; confirmed unchanged. |
| Arc probe depends on optional center | Confirmed and fixed. The authoritative endpoints/bulge determine the midpoint; an absent or stale center no longer changes it. Degenerate zero-bulge/zero-radius input retains the chord fallback. |
| Points absorbed by chains | Point entities are excluded from the endpoint grid and emitted as separate open singleton objects, preserving original source ordinals even for coincident points. |
| Reverse chaining repeatedly inserts at index zero | Accumulate upstream segments and source ordinals, reverse once, append the forward chain. Seed scanning is monotonic instead of restarting a linear search. This removes the specific prepend/seed-scan quadratic work, not a blanket complexity guarantee for spatial-grid searches. |
| Halfpipe shared-facet witness | Confirmed and fixed. Coincident contact feet cannot count as two distinct boundary witnesses. Depth no longer jumps to the profile radius in the demonstrated case. Existing true-corner logic and 150-degree threshold remain. |
| Signed zero cache misses | Full geometry keys canonicalize +0/-0 identically. |
| 64-bit cache-key collisions | Full canonical geometry equality replaces digest-only keys for both geometry caches. Ring order/lengths, hole count, every coordinate, algorithm, pipeline version and pocket delta are retained. A forced-collision test verifies distinct entries remain distinct. Merely appending lengths to a hash would not guarantee correctness. |
| Repeated inscribed-circle skeleton calculation | Route through `medial_axis_cached(region, None)`. Keep tie-breaking and island clearance checks unchanged. Cold/warm results are compared against uncached medial-axis output. |
| Frame bbox ignores curved extrema | Use arc-aware per-object bounds, skipping empty objects and retaining the existing all-empty fallback. Circle framing now includes radius plus padding in both axes. |

## Boundaries and compatibility

- Pipeline version **47 → 48** invalidates old op/geometry results. The stable-key
  regression snapshot is intentionally updated for this discriminator change.
- Source ordinals and saved explicit cutting values remain authoritative. No
  project-format, bundle identity or ShopBot postprocessor changes.
- The Halfpipe correction is explicitly requested in this audit, superseding the
  earlier instruction to leave Halfpipe unchanged during the V-Carve-only work.
  This is a bounded witness-degeneracy fix, not a new corner-detection model.
- Witness separation uses `1e-9 * max(d1,d2,1)` mm: numerical coincidence tolerance,
  not physical clearance. No arbitrary 5-degree exclusion is introduced. The wider
  nearest-two-facets heuristic is retained and is not newly physically qualified.
- Cache capacity remains 64 entries per algorithm. Exact keys add storage and
  comparison proportional to input size; this is not a zero-cost collision fix.
  Cancellation insertion policy is preserved. No benchmark speedup is claimed.
- The pre-existing parallel global-cache test-isolation issue `ivac-rt80e` remains
  open; complete library qualification uses one test thread.

## Regression coverage

Arc probes with missing/stale centers and both bulge signs; points coincident with
contour vertices and each other in both input orders; a 5,000-edge reverse chain
with exact ordinal/order checks; shared-facet Halfpipe depth versus its analytical
circular profile; circle frame padding; signed-zero equality and deliberate hash
collisions with different same-size geometries; cold/warm medial-axis parity.

Final run results and commands are appended below. Existing release candidate and
clean-host tests are separate. **HARD STOP** for human review; no next checkpoint
or publication is authorized by this handoff.

## Final qualification

- `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --lib -- --test-threads=1`:
  **1,098 passed**, including Halfpipe and all new regressions.
- `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test geom1_contract --test geometry_source_ux --test vcarve_qualification --test shopbot_m2_post --test cps_posts`:
  **71 passed**, including output/preview correspondence and geometry-source behavior.
- `cargo fmt -p ivac-core` and `git diff --check`: PASS.
- The old stable-hash snapshot failed as expected after incrementing the pipeline
  discriminator; the new version-48 snapshot passes in the complete suite.
- [Logs](cam-followup-2026-10-17-evidence/). No parallel-suite PASS, frontend/WASM
  production build, performance benchmark or physical qualification claimed here.
