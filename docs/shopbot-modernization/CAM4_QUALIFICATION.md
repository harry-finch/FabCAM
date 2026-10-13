# CAM4 A/B — Remaining material analysis and 2D preview

Date: 2026-10-13. Engineering ivac-qk8y; human gate **ivac-tvox**, pending.
CAM3 continuation approved (ivac-wh2m closed). This qualifies a bounded A/B
checkpoint, **not all CAM4 / CAD1 Phase C**, Rest Machining or physical accuracy.

## Delivered behavior

Per-operation intended removal region minus actual flat-cutter swept region,
at the displayed final depth. The response provides regions, total/largest area,
operation/depth and explicit ready/unsupported context. Reuse Pocket targets,
Clipper2 and final returned movements, never guess from drawing corners alone.
A compact Remaining material row offers Show in 2D/Hide overlay, metrics and
collapsed detail; more than one operation has an explicit analysis selector.
Amber residual fill uses shared `--warn`, preserves holes and drawing-frame
alignment. The viewport switches to 2D when the user enables the overlay.

Generation input identity hides old polygons and metrics after relevant edits,
even after Save. New generation starts with the overlay disabled. Unsupported or
absent results never become a reassuring zero-area message. Unknown causes remain
unknown: these are unmachined regions at this depth, not proof of why they remain.
No geometry, toolpath, post, cutting values or persisted project changes.
See [ADR-0010](adr/ADR-0010-depth-scoped-remaining-material.md).

## Qualified scope and exclusions

| Context | Disposition |
| --- | --- |
| Metric milling, Cascade Pocket, one flat Endmill, straight-sided closed targets, linear final output | Bounded A/B analysis at this operation's final depth |
| Islands | Excluded holes in the intended removal region; no assertion that a gouge check was performed |
| Rectilinear concavity | Hole-preserving boolean difference; analytical L-arm test |
| Other operations/strategies, multiple or shaped tools | Not analyzed, with reason |
| Tabs / finishing or universal XY allowance | Intentional material context explicitly not analyzed; not mislabeled as failed removal |
| Curved/open targets, synthetic frames, analytic arc path | Target/path representation not yet qualified; explicit reason |
| Custom depth list, nonzero stock-top Z, depth outside stock, target outside stock | Depth/reference/stock scope not qualified |
| Arbitrary CPS planning IR | Not treated as final machine output |
| Over 5,000 returned moves per operation | Analysis budget refusal; never truncate and call it complete |
| Job accumulation, multiple depths/tools, rest machining, dogbones/T-bones | Not implemented in A/B; separate future scope |

The generic analyzer contains no ShopBot commands. The pipeline adapter admits
native interpreted linear output and bundled ShopBot decoded XYZ, preserving
UX1 output/preview parity. V1 is per operation, not cumulative job simulation.

## Numerical model

All geometry is in mm, areas in mm². Clipper boolean precision5 (0.00001mm grid)
is distinct from rounded disk/capsule approximation0.005mm and emitted-depth
comparison0.0001mm. Regions smaller than0.02mm² are omitted. A displayed zero means
no reported regions above this cutoff **within the qualified scope**, not perfect
physical machining. Individual circular tool caps are polygonal approximations;
area error is bounded separately in analytical tests. These are numerical model
parameters, not physical machining tolerances.

## Acceptance evidence

| ID | Independent evidence |
| --- | --- |
| CAM4-A01 | Square target20×20, radius3, completed perimeter/interior sweep: four corner residuals; area approximates `(4−π)×9` within0.15mm² |
| CAM4-A02 | Rapid and Z=-1 movements do not erase the Z=-2 plane |
| CAM4-A03 | A16mm² island is excluded from a100mm² target, yielding84mm² when nothing is cut |
| CAM4-A04 | Sloped motion clips at the depth crossing; vertical plunge removes a radius1 disk within its declared polygonal area error |
| CAM4-A05 | Tiny finite-radius corner residuals below the cutoff are omitted; repeated result deterministic |
| CAM4-A06 | L-shaped target minus a completed lower-arm sweep leaves25mm² only inside its upper arm |
| CAM4-A07 | Actual ShopBot square Pocket output yields four expected residuals; XY20/30 changes polygon coordinates but not area; allowance returns intentional-context unsupported |
| CAM4-B01 | Frontend transforms outer/hole residual polygons to drawing frame without changing area/depth/program or input response |
| CAM4-B02 | Helper preserves explicit unsupported/unknown states, isolates selected operation and suppresses stale/missing data |
| CAM4-B03 | Browser fixture: four regions,30.99mm² at Z=-2, aligned amber corners; toggle switches to2D and can hide the overlay |
| CAM4-B04 | Browser strategy edit hides metrics/overlay; Save retains stale state/export refusal; unsupported Zigzag generation gives a specific explanation |
| CAM4-B05 | Saved project reopened/regenerated with declared XY5/5 and operation parameters; diagnostics recomputed rather than persisted |

The browser [software fixture](CAM4-software-pocket.ivac-project.json) uses a40mm
square, Ø12mm cutter, depth=-2, step=-1, stock50×50×10, XY5/5 stock-corner reference.
Its numerical corner-area expectation is `(4−π)×36 ≈30.90mm²`; the reported30.99mm²
reflects polygonal capsule approximation. Illustrative machine settings are not
Sorbonne facts. The remaining manual-spindle ShopBot warning stays visible.

[Overlay proof](ux/CAM4-remaining.png), [stale proof](ux/CAM4-stale.png) and
[unsupported-context proof](ux/CAM4-unsupported.png) are browser evidence, not
physical material measurements.

## Checks and delivery

From repository root, scratch artifacts remain outside root target/debug:

```sh
export CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_INCREMENTAL=0 RUSTC_WRAPPER=''
cargo test -p ivac-core --features cps --offline --lib -- --test-threads=1
cargo test -p ivac-core --features cps --offline --test shopbot_m1_qualification --test shopbot_m2_post --test snapshots_drill_tabs_whirl -- --test-threads=1
cargo clippy -p ivac-core --features cps --offline --lib --test shopbot_m2_post -- -D warnings
cargo run --offline -p xtask -- schema-check
npm --prefix frontend run codegen
npm --prefix frontend run check
npm --prefix frontend run lint
npm --prefix frontend run test
npm --prefix frontend run build
```

Full core1,062; M1 11; ShopBot27; snapshots4 pass. Frontend1,110 pass /1 optional
skip in120 files; check zero warnings; lint/fmt/Clippy/schema-check pass. Complete
optimized WASM/frontend build passes with the existing Tauri static/dynamic-import
bundling warning. No native desktop app bundle or physical test. HTTP CPS server
on18772 is rebuilt and retained for review; [STATUS](STATUS.md) gives the page URL.

## Human review and next scope

Open the fixture, Generate, Show in2D and inspect the four corners plus detail.
Toggle visibility, change the strategy and confirm stale suppression, then Save:
the old verdict must remain stale. Zigzag Generate should say Not analyzed with
a specific reason. Restore Cascade, Save/reopen and regenerate. No machine test
is needed for this software UI/geometry review.

Stop at ivac-tvox. CAD1/Phase C corner relief has separate scope/approval in the
roadmap. Curved target/path qualification and intentional-material classification
must be designed before expanding the diagnostic (ivac-vcp7); the current exclusions
are not silently lifted. FS4 stays deferred/nonblocking; FS expansion remains frozen.

## User review follow-up

[Playback/visibility follow-up](ux/CAM4_REVIEW_FOLLOWUP.md) corrects the WASM BigInt
checkpoint boundary, warning-chip alignment and missing 3D path toggle. Repeated
actual browser scrub/visibility checks and frontend1112-pass evidence recorded.
Existing Spiral is the explicit continuous roughing choice; A/B residual scope
remains Cascade-only. Human gate ivac-tvox remains pending.
