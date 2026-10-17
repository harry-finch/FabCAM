# Pocket/CPS audit — lot 8 — 2026-10-17

Engineering: ivac-lbpxs. User explicitly requested verification, corrections,
tests and a report. The attached independent audit is evidence to examine, not
an instruction to accept every proposed change. Incoming uncommitted lots 4–7
and workspace cleanup are preserved; checks cover the combined working tree.

## Assessment and changes

| Finding | Source assessment and disposition |
| --- | --- |
| Critical: combined Pocket/Outside loses tabs | Reproduced before changes: manual Outside tab and secondary disjoint-Union tab both disappear. Preserve source attribution, but map holding intent through the actual combined regions rather than `region.source_idx`. Attach each qualified tab to a reachable cutter path, refuse disappearance/ambiguity. Native final-path height regression proves the retained bridge is not cut through. |
| Critical subclaim: Boolean overcut uses a wrong original wall | Not applicable to the current branch. Boolean region emission is Pocket-only; `finalize_offsets` invokes overcut only through `profile_params()`, which returns Some only for Profile. Profile does not use this Boolean branch. No new Pocket overcut support is introduced. |
| High: an open inner stroke destroys an Auto island | Not reproduced in the stated configuration. The primary selected-depth island collection already includes the closed inner circle under Source=All, before the fallback `auto_annular` predicate. Before any production fix, Cascade/Spiral/Zigzag witnesses show identical offsets with/without an inner open stroke and the preview retains one hole. Keep the existing selection/parity contract; no speculative `.all` → `.any` edit. |
| High: CPS diameter ignores wear | Confirmed. CPS diameter now matches `effective_diameter()` used for path compensation; BallNose corner radius uses half that value. Nominal ToolEntry/UI diameter remains unchanged. BullNose's separately declared fillet radius is not silently rescaled: no new wear-deformation model is invented. |
| High subclaim: finish RPM is silently lost without a tool change | The recorder already emits a mid-section SpindleSpeed record. Existing recorder tests cover it; a new full-pipeline Pocket witness confirms the bundled ShopBot post sees and explicitly refuses `spindleSpeed`. It does not silently export the wrong RPM. No CPS capability is widened. |
| Medium: circular Arc chains / endpoint-only drill bbox | Confirmed. Recognize closed, connected, single-winding, common-circle Arc/Circle chains, deriving centers from bulges and checking any cached centers. Reject lenses/reversals, multiple windings, disconnected/inconsistent geometry. Preserve the existing small-circle size threshold. Drill bbox references now include arc extrema; a closed concave reference whose bbox center is outside its polygon refuses and requests an explicit drilling point. |
| Low: fixed auto-tab phase does not clear every corner | Confirmed on a 30×20 rectangle, four 10 mm tabs. Choose one uniform phase from the complement of all corner-forbidden phase intervals; count and spacing stay unchanged. Manual placements are untouched. If no uniform phase exists, retain requested tabs and emit `tabs_corner_clearance_unavailable` (EN/DE), rather than claiming corner safety or deleting holding intent. |
| Related preview mismatch found while checking radius usage | Frame preview used the nominal radius while the path builder used the effective radius. Align preview frame synthesis with effective diameter; a wear-aware frame-bound regression covers it. |

## Holding-tab qualification boundary

Merging every contributor's tabs by a cutter-sized distance alone is unsafe:
a tab center can lie near a surviving wall after its original boundary/span was
removed. This lot supports Boolean tabs only when the **complete source contour
survives** on one combined boundary, within the Boolean engine's declared grid.
Changed/intersected holding contours require separate holding operations or a
future qualified local-span lineage adapter; they fail explicitly even if a
particular local tab might have been usable.

The retention proof checks continuous interval coverage of every source edge,
not just vertices or sampled midpoints. It shares the existing Clipper precision
(4 decimal digits, 0.0001 mm grid), with a one-million segment-pair work budget.
That is a computational/quantization contract, not physical machining accuracy.
Missing reachable offsets, ambiguous ownership and budget exhaustion refuse.
Original source indices remain layer/color attribution; they are not repurposed
as fabricated Boolean-boundary provenance.

This does not qualify unsupported Trochoidal tabs, GEOM1 modifier combinations,
all curved-tab retention/holding strength, or later operations on the same part.

## Circle, reference and placement limits

- Circle recognition requires continuity, consistent direction and a total
  sweep of one turn. Center/radius/angle allowances cover floating arithmetic
  only. There is no broad SVG/SPLINE polygon-to-circle inference or relaxed fit.
- Arbitrary closed Drill references keep their established bbox-center policy
  when it lies inside the reference polygon. They are not automatically changed
  to a centroid or to an invented material location. Explicit point references
  remain available. Stock/fixture checks are separate.
- Corner phase uses the source polyline used for placement, including its
  tessellation vertices, and the existing 1e-6 mm placement clearance. A uniform
  phase need not exist. The warning preserves holding intent but is not a claim
  that every requested automatic layout is workshop-qualified.

## Validation

Final verification and source hashes are recorded in the
[evidence manifest](pocket-cps-audit-2026-10-17-evidence/checks.json), alongside
raw logs. Twelve new regression tests are included in these totals:

| Check | Result |
| --- | --- |
| Core library, CPS enabled, offline and serial | 1,157 PASS |
| Seven integration suites | 87 reported PASS; 86 executed, optional FANUC oracle absent |
| Frontend tests | 1,324 PASS, 1 existing skip; 10 additional WASM guard tests PASS |
| Frontend check / lint | PASS; 0 check errors/warnings |
| Clippy, warnings denied / Rust formatting / whitespace | PASS |

The before-change log retains the two failing holding-tab regressions and the
passing open-stroke counterexample. The height diagnostic records why an initial
test selector found no witness: the existing erosion pad places the wall at
y=18.9989 rather than exactly 19. The final test locates the actual wall and
checks that the bridge remains at or above Z=-3 for a -4 floor and 1 mm tab.
This adjusts witness selection, not holding-height tolerances. The hash log
retains the intentional old-snapshot failure before updating version 54.

Engineering **ivac-lbpxs** is complete. Independent human review
**ivac-v103z remains OPEN**; earlier human, physical and platform gates remain
open. These results cover the combined local tree, including incoming lots 4–7.

Pipeline discriminator advances 53 → 54 for changed output/guards; its audited
key snapshot is `0xc88f44608ba47db1`. Public version remains beta.2 pending a
separate source-publication decision. No native/WASM production build, machine
execution, installer, commit, push, tag or release is included.
