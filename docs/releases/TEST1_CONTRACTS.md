# TEST1 — approved bounded contracts

Current state — 2026-10-18: implemented bounded software evidence consolidated on
pipeline61; [current matrix and limits](FINAL_CONSOLIDATION_2026_10_18.md). Human review ivac-l6ouh OPEN;
TEST1/SAFE-Z overall remain open pending scoped acceptance. No commit/publication.
The contracts below retain their original numerical budgets and scope. Historical
proposal/reproduction text is context, not an assertion that implementation is absent.

## Common execution contract

Recommended infrastructure: a small deterministic test-only Rust generator with
explicit versioned seeds, fixed tables plus bounded perturbations and domain-specific
counterexample reduction. Reuse existing Rust/TypeScript test runners; no new property
framework/dependency for this bounded V1 stage. Record generator version and PRNG
algorithm (proposed SplitMix64, wrapping u64, generator v1). Root seed0x5445535431000001, property IDs1–8 in table order, case index0–31; log each derived seed. No platform/time-dependent seed. The generator
must not call production geometry code to decide the expected result. If shrinking
requires substantial custom infrastructure, stop and propose a dev-only framework
instead of building a general fuzz engine.

Eight named properties below; **32 cases each (256 total)** in the native CI job,
including edge cases, with separate fixed refusal fixtures. IDs A1/A2 here are TEST1
IDs, not the historical V-Carve counterexamples. Each case records property, seed,
case index, complete serialized input and expected contract. Coordinates in mm,
absolute values <=1,000; <=32 segments per contour, <=4 contours, <=3 operations,
<=3 passes, <=20,000 emitted moves per case. No unbounded rejection sampling:
construct valid cases; invalid cases are deliberately tagged and must refuse.

CI target: <=120 seconds for this stage on the recorded reference runner, excluding
build; individual generation deadline 10 seconds. These are **proposed budgets, not
measured results**. Budget overrun fails the stage; no truncation, retry-until-pass
or silent case discard. A watchdog surrounds noninterruptible work. Report machine,
OS, toolchain, backend, seed/count, duration, skips and timeouts. Keep the full existing
regression stage. Local extension: up to512 cases/property and600 seconds, same
replay format; larger campaigns require another scope decision.

Shrink by removing unrelated contours/operations, reducing segment count and then
halving parameter deviations while retaining domain validity AND the same independent
failure. Save original and reduced inputs/hashes; cap reduction at128 trials/60seconds.
A failed reduction does not invalidate the original failure. Independently review
counterexamples and retain confirmed ones as ordinary regression fixtures.

## Properties and independent oracles

| ID | Domain / preconditions | Required result and oracle |
| --- | --- | --- |
| A1 — source membership | 1–4 labelled disconnected line/arc chains; unique source IDs and explicit segment ordinals, plus coincident XY from distinct sources. Permute input order and reverse qualified chains; retain 1–32 segments each. Deliberately ambiguous selections are separate invalid cases. | Compare a multiset of original provenance identities, not output positions: no lost/duplicated/reassigned source membership. Object index ordering itself need not remain identical. An ambiguous binding must refuse or expose the existing explicit ambiguity state, never pick another source silently. |
| A2 — bounds and frames | Circular arcs radius1–100, signed sweeps in [0.01, 2π], cardinal crossings, translated rectangles; translations <=100mm. Nonuniform DXF transforms remain existing fixture coverage, not an arbitrary affine-arc promise. Numeric Z origin stays0. | Analytic extrema from endpoints plus included cardinal angles must lie in reported bounds. XY work-frame forward/inverse round trip and translated stock/source alignment preserve geometry and unchanged Z. Standalone analytic formulas, not production bbox/transform helpers. NaN/Inf/overflow rejection fixtures run separately through the actual representable API. |
| B1 — flat-cutter clearance | Profile inside/outside rectangles/circles and rectilinear Pocket with0–2 rectangular islands, minimum gap > cutter diameter+1mm; D1–12mm, zero allowance, Direct entry, supported cascade/raster subset named in fixture metadata. | Analytic line/arc distance against protected half-planes, circles and island rectangles over the **whole cutter sweep**, including links/entries. Profile compensation side and radius must agree. Pocket must not cross protected material; required target-depth witness in known reachable strips. This is not a general complete-pocket coverage proof. Invalid narrow/unsupported cases require current documented refusal, not an invented universal rule. |
| B2 — V-Carve envelope | Preserve Perimeter and Full medial axis as separate case groups: rectangular grooves, wedges with30–150° corners and one small rectangular island; V-bits60°/90°, no tip-flat/nonqualified tool, <=3passes. Reuse approved continuous envelope budgets from V-Carve qualification. | Independent distance to analytic source boundaries gives allowable depth along each interpolated move. Use interval bounds/subdivision with a residual bound, never control points alone. Check both overcut and target-depth witnesses/undercut. An unresolved bound or incomplete qualified path must refuse. Perimeter is a V-bit contour; medial-axis sampling does not imply full-area pocket coverage. Halfpipe unchanged, regressions retained. |
| C1 — cold/warm equivalence | Supported B1/B2 requests and fixed unsupported requests; cold versus hit, then mutate one field at a time: source/transform, tool geometry, depth/start/clearance, operation parameters, machine feed/post settings; restore original. | Compare complete deterministic output, toolpath, source map, regions, warning kind/params and refusal category. Exclude only explicitly named wall-clock/cache statistics; predicted machining time remains compared. Count a hit with scoped instrumentation, not global entry count. Isolation is a prerequisite (ivac-rt80e); run each scenario in a dedicated process or scoped cache. No new key lint framework. |
| C2 — stale-result lifecycle | Existing frontend request-scoped state machine: edit, undo/redo, cancellation, out-of-order completion, source/tool/machine change. Generate bounded sequences <=8events with at most2concurrent requests. | Independent state table: only the latest successful matching context can export; failed/cancelled/stale response cannot become current or overwrite warnings/output. Same-context cache reuse is allowed only under existing freshness contract. Exercise actual export policy, not a mock success boolean. No timing-based sleeps as correctness oracle. |
| D1 — decoded machine parity | Qualified single-sided metric ShopBot Profile/Pocket/Drill and V-Carve subgroup, one or two tools with supported manual transitions; preserve prohibited strategy/tool/dual-side refusals. | Independent OpenSBP decoder (not production preview parser) reconstructs JZ/J2/J3/M3 and MS state; compare movements, feeds, operation/source mapping with generated intent and preview. Track initial unknown axes explicitly: no fabricated initial position or full first-rapid preview claim. Check finite values, feed/plunge bounds and no partial-success output on refusal. |
| D2 — SAFE-Z and guards | SAFE-Z Z01–Z08 cases with bounded stock/clearance variations, both machine envelope known/unknown; no new origin mode. | Apply the separately approved common-frame safety contracts below generation, decoded output and export lifecycle. Entire rapid trajectory versus known stock/fixture is assessed, not only end Z. Unsupported/unknown situations cannot be labelled qualified safe. |

## Numerical comparison rules

- Identity, refusal categories, state transitions and cache output text: exact.
- A2 analytic double-precision coordinates: absolute1e-8mm over the bounded domain;
  this is a numerical comparison budget, not manufacturing tolerance.
- B1 analytic sweeps:1e-8mm numerical uncertainty; curved **decoded chords** additionally
  use the existing documented0.01mm chord deviation. Do not apply that extra budget
  to analytic source geometry or silently allow a clearance gap to become negative.
- B2 pins the [existing V-Carve budget](../shopbot-modernization/VCARVE_QUALIFICATION.md#numerical-budget-software-not-machining-accuracy):
  scheduling1e-7mm; offset grid1e-4mm; radial clearance allowance0.0002mm;
  quadratic residual1e-9mm² (radial sqrt residual <=0.0000317mm).
  The selected straight-edge generators have zero curve-flattening error; retained
  curved regression fixtures allow0.002mm. With half-angle tangent t, the radial
  decoded bound is `curve_error + 0.0002 + 0.0000317 + sqrt(2)*0.00005 + t*0.00005`mm,
  plus1e-8mm oracle arithmetic. Convert radial error to axial by division by t;
  include scheduling1e-7mm separately for pass-depth/target completion. Independent
  interval subdivision residual must be <=1e-8mm, or fail unresolved. Medial boundary
  spacing0.1mm is a sampling setting, never an accuracy/coverage tolerance. Undercut
  target witnesses use scheduling+Z rounding and oracle residual, not the much larger
  clearance allowance to hide missing target completion.
- D1 OpenSBP XYZ uses4decimals: <=0.00005mm per serialized axis +1e-8mm arithmetic;
  two independently rounded values require the sum of their respective bounds.
  MS uses6decimals inmm/s: <=0.0000005mm/s per serialized rate. Preserve0.01mm arc
  linearization as a separate geometric bound. Validate format precision in the test.
- Unresolved topology, interval limit or nonfinite results are explicit failures/refusals,
  never absorbed by increasing epsilon. No bitwise Dexel associativity claim.

Native property tests do not qualify browser WASM runtime. Replay a fixed shared
fixture subset through WASM bridge tests; packaging freshness, real browser/desktop
and clean-host tests remain separate release gates. No production code, dependency,
property implementation or expanded capability is authorized by this proposal.

**HARD STOP — review the current consolidation; original contracts are already approved.**
