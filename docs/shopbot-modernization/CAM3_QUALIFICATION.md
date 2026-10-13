# CAM3 — Tabs / Lead-in / Lead-out software qualification

Date: 2026-10-13. Engineering scope: bounded software qualification and reproduced
holding-material defects. Human gate: **ivac-wh2m**, pending. CAM4 not started.
User approved progression from UX2.1 after its follow-up fixes/documentation.
No physical trial or general-purpose stock-removal certification is claimed.

## Behavior and corrections

All dimensions are mm. Tests use a Ø6 cutter, 100mm square or Ø50 circle,
final Z=-5 and 2mm maximum pass increment; reference Z is unchanged.

| Finding reproduced before correction | Result |
| --- | --- |
| Tab footprint default used tool diameter instead of configured width | Configured width drives emission; explicit per-tab width still takes precedence |
| Taller manual height override was ignored on intermediate passes | Compute each tab's absolute top from final floor plus its height; resolve the required lift per pass |
| Curved placement projected onto a chord | Project onto the swept arc, or its nearest endpoint outside the sweep |
| Curved ramp restarted at each chord and cut back into the tab | Continuous height envelope across chord seams; bounded 0.005mm geometric chord error and at most 0.25mm nominal chord length |
| Inside-profile lead-out crossed a neighboring holding tab at final depth | Lead-out uses the same tab-aware emission as the contour |
| Manual tab at entry was plunged through before tab traversal | Clamp direct entry to the protected tab top when the entry lies inside its footprint |

No saved schema, Tool/CuttingParameters model or FS mathematics change. Corrected
jobs intentionally produce different XYZ output after regeneration; saved geometry,
manual rates, tab declarations and origin remain authoritative. Generic fixes stay
outside ShopBot syntax; bundled post and its unsupported-entry guards are unchanged.

## Independent evidence

`crates/ivac-core/tests/shopbot_m2_post.rs` adds four CAM3 semantic tests:

- CAM3-T01: configured width20 spans the expected square-edge footprint; a height3
  override preserves Z=-2 even on intermediate passes, while outside the tab the
  contour reaches final depth. Protection checks are nonvacuous samples of emitted
  SBP movements, independent of the production tab emitter.
- CAM3-T02: **48 combinations** of square/circle × inside/outside × climb/conventional
  × Rectangle/Ramp tab × Off/Straight/Arc lead. Verify direction by signed XY area,
  depth bounds, tab-plane presence, square tab-center preservation, clear XY rapids,
  exact emitted-move/returned-preview endpoints and serialized project round-trip.
- CAM3-T03: manual curved tab center survives for Rectangle and Ramp; sampled chord
  radius stays within 0.0102mm of expected compensated radius28. The allowance
  includes output rounding; it is not a physical machining tolerance.
- CAM3-T04: manual rectangular tab at contour entry survives direct descent.

Existing core tests cover auto/manual/mixed resolution, short-count clamping,
per-tab overrides, line ramps/triangles, leads and open contours. M1 independently
covers compensation, pocket islands/concavity and finish allowance. UX1 entry
corpus/refusal tests retain curved/concave/open/island coverage; CAM3 does not
broaden every one of those into a tabs-plus-leads correctness claim.
Two snapshot digests change intentionally with corrected configured tab widths;
line ramp/triangle tests now explicitly set their stated 3mm width. Unrelated
Drill/face-mill digests stay unchanged. Semantic material tests, not hashes alone,
justify the correction.

## Execution and evidence limits

| Combination / concern | Qualification disposition |
| --- | --- |
| Direct entry + tabs + leads on simple square/circle | Bounded software matrix above; not physical qualification |
| Curved rectangular tabs | PARTIAL: conservative whole-intersecting-arc lift may leave substantially more material than requested width; no precise remaining-material claim |
| Curved ramped tabs | Continuous sampled XYZ profile and compensated-radius/center checks; no analytical general stock sweep or guaranteed physical holding strength |
| Ramp entry with tabs or leads | Explicit ShopBot refusal retained; generic direct substitution is not silently enabled |
| Helix, short-path entry fallback, unsupported WCS/Z/two-sided | Existing explicit refusals retained |
| Manual ramp tab at a corner/entry, dense overlapping placements, open/concave tabs+leads | No blanket end-to-end qualification; existing lower-level tests do not establish every combination |
| Leads versus neighboring stock/islands/fixtures | Leads are not automatic avoidance; existing resolved collision warnings/export gate remain relevant |
| Intentional residual/allowance | Tab top is measured above final output floor, including through-depth; operator must account for stock thickness/Z surface. No new material analysis or safety guarantee |

CAM4 must consume actual generated/resolved moves and distinguish intentional
holding material/allowance from inaccessible material, including conservative arc
lifts. Further arc-width precision or the remaining topology matrix requires its
own scoped qualification (ivac-57kp), not a silent enablement during CAM4.

## Reproducible checks

Run from repository root, with scratch build artifacts outside root target/debug:

```sh
export CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_INCREMENTAL=0 RUSTC_WRAPPER=''
cargo test -p ivac-core --features cps --offline --lib -- --test-threads=1
cargo test -p ivac-core --features cps --offline --test shopbot_m1_qualification -- --test-threads=1
cargo test -p ivac-core --features cps --offline --test shopbot_m2_post -- --test-threads=1
cargo test -p ivac-core --features cps --offline --test snapshots_drill_tabs_whirl -- --test-threads=1
cargo clippy -p ivac-core --features cps --offline --lib --test shopbot_m2_post -- -D warnings
npm --prefix frontend run build
```

Results: core1056 pass; M1 11 pass; CAM3 targeted4 pass (48 matrix combinations);
full final ShopBot26 pass; updated snapshots4 pass; Clippy pass. Complete optimized
WASM/frontend build passes (existing Tauri mixed static/dynamic import warning).
Native CPS server rebuilt and restarted on the same review port18772. Serial testing retains
the known global pocket-cache test isolation policy. No frontend UI logic changed.

## Human review fixture

Open [CAM3 software tabs](CAM3-software-tabs.ivac-project.json) in the browser
using the CPS native API URL recorded in [STATUS](STATUS.md). This is SOFTWARE
ONLY, with illustrative settings, not the qualified Sorbonne machine profile.
Outside square profile: Ø6, depth=-5, step=-2, width20, base height1, one manual
tab at t=0.125 with height3, Direct entry and straight leads. XY work zero20/30
remains deliberate; preview displays drawing coordinates.

Generate, inspect the tab in 3D/toolpath and the emitted listing, Save/reopen and
regenerate. Check that its top stays Z=-2 (not an intermediate-pass lift above an
already-cut tab) and the surrounding edge reaches Z=-5. Switch Rectangle/Ramp
and check sloped transitions; no physical test required for this software handoff.
A Ramp entry with tabs must still refuse rather than silently substitute Direct.
Do not run this software fixture on the machine as a lab-qualified program.

Actual browser proof: fixture import/generation produced147 resolved moves and179
SBP lines; Save/reopen preserved width20, manual height override3 and XY20/30.
Regeneration again produced147 moves/179 lines; independent sampling of the actual
browser-exported SBP at the shifted tab point (40,-23) confirms minimum Z=-2.
[Browser proof](ux/CAM3-tabs.png) shows the 3D preview and configured width20.
The fixture has two expected warnings: manual ShopBot spindle policy and
out_of_work_area because the illustrative XY20/30 origin gives negative job
coordinates. They are retained visibly; this is not an executable lab setup.
No UI styling or frontend logic change; prior1107-pass frontend evidence is
historical, not rerun/claimed as a new CAM3 result.

## Continuation approval

User approved CAM3 and requested the next milestone on2026-10-13; ivac-wh2m closed.
Bounded CAM4 A/B started without broadening CAM3's physical/topology evidence.
