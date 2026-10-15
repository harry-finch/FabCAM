# V-Carve — software qualification

Presentation follow-up: [result-oriented strategy labels and contextual depth help](VCARVE_STRATEGY_UX.md). Existing numerical evidence below is unchanged; human gate ivac-1b75 remains.

Final UX/diagnostics refinement is implemented; see [final UX audit and screenshots](VCARVE_FINAL_UX.md). Unsupported saved settings remain visible/rejected and explicitly correctable; inlay action unavailable; warning severities/export guards preserved. **Final human acceptance ivac-1b75 remains pending.**

2026-10-14 · Implemented locally, **HARD STOP: human review ivac-1b75**.
No machine connection, physical cut, production recommendation, commit or push.
User approved the targeted planner after [Phase A](VCARVE_AUDIT.md).
[ADR-0017](adr/ADR-0017-vcarve-continuous-planner.md) records the boundary.

## Audit and corrected algorithm

A1 originally ignored Perimeter step/pass and offset by full tool radius even
when maximum depth reduced the engaged radius. A2 crossed below a variable
local target between points. A3 left short chains shallow without failing.
The red reference tests were run before replacing the planner; preserved
[evidence](vcarve/RED_REFERENCES.txt). The historical audit example intentionally
continues to probe the legacy emitter (now used only by Halfpipe).

`cam/vcarve_plan.rs` replaces passes/entries for V-Carve only. The existing
region combination, axis builder, V-bit model, generic XYZ emitter, CPS recorder
and resolved-output preview remain the architecture. Halfpipe uses its untouched
legacy emitter, not this scheduler.

Perimeter remains the default saved mode: a constant-final-depth contour groove,
with progressive passes. Its XY offset uses the radius at the actual capped depth.
Full medial axis retains sampled interior spines with variable depth. All retained
chains must finish; no short-chain pruning in this driver. Neither mode promises
complete-area pocket clearing. The full medial mode name is preserved for saved
projects and explicitly explained in the inspector.

## Tool geometry, depths and continuous proof

For the supported symmetric conical V-bit, radius at axial depth h is
`r = tipDiameter/2 + h*tan(includedAngle/2)`; signed Z is `-h` relative to stock top.
Sharp 60°/90° and flat-tip 60°/90° references independently verify this formula.
Cutting diameter, optional radius cap and maximum depth bound the target. An authored
cutting length is enforced conservatively against the requested limit. No holder,
runout, stock deformation or unmodeled body clearance is inferred.

Each sweep is a piecewise-linear lower envelope of target depth, previous cut
plus step, and previous cut plus distance travelled times the entry-angle tangent.
All affine intersections are inserted. Sweeps reverse continuously at chain ends;
no re-entry at full depth into an uncleared start. The first ramp starts at stock
top. Later ramps limit **additional engagement relative to the cleared groove**;
the absolute XYZ slope can be steeper than the entry angle. Step/pass is an axial
increment on this same chain, not a universal volumetric engagement limit.

Before emission, a continuous disk-clearance proof checks the complete target
chord against every outer/hole edge. Projection-region boundaries split the chord;
quadratic minima of distance squared minus radius squared test the intervals.
This catches collisions between control points, including a tiny island invisible
at the endpoints. Shallower scheduled passes on the same XY inherit this bound.
A final entire sweep must agree with its target. Overcut, undercut/budget exhaustion,
unresolved topology and invalid inputs return an operation error, no successful
partial response. Warnings for explicit target caps mean the requested capped
path was completed, not that the uncapped region was cleared.

## Numerical budget (software, not machining accuracy)

| Stage | Budget / interpretation |
| --- | --- |
| Source curves | chord sagitta at most 0.002 mm, cloned V-Carve geometry only |
| Topology / scheduling | 1e-7 mm comparison; curve topology also uses area threshold 1e-7 mm² |
| Existing offset grid | 1e-4 mm Clipper scale; clearance allowance 0.0002 mm |
| Quadratic predicate | residual 1e-9 mm²; conservative added radial allowance sqrt(1e-9) ≈ 0.0000317 mm near zero radius |
| Medial sampling | existing boundary spacing 0.1 mm; **not** a 0.1 mm coverage/axis-location accuracy guarantee |
| OpenSBP | 4-decimal coordinates: up to 0.00005 mm per axis; parity comparison 0.00011 mm against independently rounded generic output |

For a cone half-angle tangent t, a conservative radial boundary budget after output
is `0.002 + 0.0002 + 0.0000317 + sqrt(2)*0.00005 + t*0.00005` mm.
The corresponding axial equivalent is the pre-output radial budget divided by t,
plus 0.00005 mm Z rounding. At 90° included angle the radial total is under
0.00236 mm. This accounts for approximation of retained paths, not missed branches,
physical accuracy or complete swept-volume coverage. Endpoint rounding interpolates
linearly and stays within those coordinate bounds. The pass-depth increment can
gain at most 0.0001 mm from two rounded Z values, plus scheduling comparison error.

Resource limits: 10,000 flattened edges, 20,000 input points per chain, 4,096 sweeps,
1,000,000 planned points per chain/operation, 20 million clearance edge comparisons.
Exhaustion fails explicitly. These are termination bounds, not a performance promise.

## Qualification matrix

| Area | Software result | Evidence / limit |
| --- | --- | --- |
| A1 Perimeter | PASS | five 0.2 mm levels to 1 mm; engaged radius 1 mm with D6/90° |
| A2 continuous local envelope | PASS | adversarial falling target plus independent interpolated oracle |
| A3 short-chain completion | PASS | both ends reach target; pathological tiny chain explicitly undercuts/fails |
| Cone geometry | PASS | analytic 60°/90°, sharp/1 mm flat tip, radius and depth caps |
| Rectangle / circle | PASS | both modes, source circle represented by arcs; deterministic rectangle |
| Acute / L / 0.1 mm slot / near-collinear | PASS | Full medial, both windings; independent chord-point clearance checks |
| Nested square island | PARTIAL | Full medial succeeds; Perimeter explicitly refuses overcut at offset corners |
| Disconnected regions | PASS | both modes; normal safe retract/travel |
| Full medial coverage | PARTIAL | independent continuous swept-cone oracle covers 256/800 cell centres on 20×10 mm rectangle, depth cap 1 mm, 0.5 mm grid; **32% sampled coverage, not an area bound**; known uncut point (5,2), cut point (10,5) |
| Full medial exactness | PARTIAL | sampled axis retained; no proof of all tiny branches or arbitrary-region complete coverage |
| Invalid geometry | PASS refusal | open, degenerate, self-intersecting, nonfinite/zero-length, empty/unresolved regions |
| Start / schedules | UNSUPPORTED refusal | nonzero start, Through depth, explicit Depth list, Finish step, legacy refine option |
| Inset / inlays | PARTIAL | single-ring nonsplitting inset only; islands/splits refused; paired fit is a future milestone |
| A4 ShopBot and preview | PASS bounded | separate decoder checks coordinates, ordering, rapid/cut, feeds, source line and operation ID; generic path parity; nonzero XY origin; project roundtrip identical |
| Arbitrary Engrave / V-bit operations | UNSUPPORTED | guards retain rejection; V-Carve-specific linear XYZ only; arcs refused |
| Persistence/history | PASS | both saved modes, radius cap, cone geometry; tool edits stale output; Undo/Redo; manual rates unchanged |
| UI / preview | PASS bounded | real macOS browser inspector, tool geometry, 2D source view / 3D paths, stale export, explicit refusal and Undo; screenshots below |
| Physical machining | NOT TESTED | no hardware action authorized |

## ShopBot and preview boundary

Only strategy `vcarve` with V-bit/chamfer tool type 8 is added to the bundled post.
Generic Engrave stays `engrave` and rejected. V-bits on Profile/Pocket/milling and
flat tools on V-Carve stay rejected. V-Carve circular motions are rejected rather
than guessed or broadly linearized. Existing spindle, entry, unsupported-command,
work-origin and export preflight guards remain. The post emits metric XYZ through
the established mm IR contract; this suite uses metric reference projects.
No new inch-profile V-Carve qualification is claimed.

A4 compares generic cutting endpoints with independently decoded OpenSBP, then
compares every decoded segment with the production resolved preview, including
line provenance. Initial XY is unknown until set; no invented initial segment.
The captured 2D mode displays source geometry, not an independently qualified
2D toolpath overlay. Resolved execution paths are inspected in 3D.
3D heightfield rendering remains an approximate visual aid, not a cutter collision
proof or exact swept volume. No change to its engine in this milestone.

## Tests, performance and reproduction

- 10 dedicated qualification tests, [final run](vcarve/QUALIFICATION_TESTS.txt).
- 1,074 core library tests pass serially, including 15 Halfpipe-named tests.
  An initial parallel run hit an existing shared geometry-cache count race;
  the serial run avoids that global-cache interference.
- All three Halfpipe files match their pre-work SHA-256 values.
- 7 ShopBot guard tests; 51 existing CAD1/UX3/M1/M2/drill-tabs-whirl regressions pass.
- Frontend: 1,234 pass, 1 skipped; Svelte check zero diagnostics, lint pass.
- Clippy with warnings denied passes. Final targeted V-Carve library tests: 7 pass.
- Full WASM/frontend production build and CPS server build pass. Existing bundle-size
  and mixed static/dynamic import warnings remain; not introduced by this work.

Commands from repo root (normal configured Rust/Node environment):

```sh
cargo test -p ivac-core --features cps --test vcarve_qualification -- --nocapture
cargo test -p ivac-core --features cps --lib -- --test-threads=1
cargo test -p ivac-cps --test shopbot_guards
cargo test -p ivac-core --features cps --test cad1_qualification --test ux3_qualification --test shopbot_m1_qualification --test shopbot_m2_post --test snapshots_drill_tabs_whirl
npm --prefix frontend run check
npm --prefix frontend test -- --run
npm --prefix frontend run lint
npm --prefix frontend run build
```

The final dedicated suite takes about 4.5 s in this environment (parallel tests,
not a single-job latency claim). Topology matrix paths range from 208 to 8,693
preview segments; browser 40 mm square: Perimeter 130, medial 5,862. Browser remains
interactive. An isolated eight-case topology run takes 0.18 s wall time, peak RSS 17,285,120
bytes (~16.5 MiB), zero swaps on this macOS host; [measurement](vcarve/PERFORMANCE.txt).
This covers those synthetic cases, not large production artwork or browser memory. Preserve the budgets rather
than silently dropping geometry to accelerate a result.

## Human review kit and screenshots

Open [Perimeter](VCARVE-software-perimeter.ivac-project.json) or
[Full medial](VCARVE-software-medial.ivac-project.json), Generate, inspect warnings,
switch 2D/3D and read the OpenSBP preview. All cutting values are synthetic
**SOFTWARE ONLY**, not machine settings. The browser test used the Perimeter file
then changed its mode; the operation name intentionally remains the saved name.

1. [Perimeter inspector and resolved 3D](ux/VCARVE-perimeter.jpg).
2. [Full medial inspector and resolved 3D](ux/VCARVE-medial.jpg).
3. [Source geometry in 2D after medial generation](ux/VCARVE-medial-2d.jpg).
4. [V-bit dimensions](ux/VCARVE-tool.jpg).
5. [Refused staged start and blocked export](ux/VCARVE-refused.jpg).

Review distinct mode semantics, cap warnings, refusal wording and the remaining
coverage limits. This is the recommended next checkpoint, not authorization for
inlays, two-sided machining, physical trials or another roadmap milestone.
**STOP — waiting for human review and approval (ivac-1b75).**
