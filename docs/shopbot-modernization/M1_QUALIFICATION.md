# M1 — milling core qualification

Completed 2026-10-10 on macOS after approval to continue from M0. Scope:
SVG/DXF imports, profile, cascade pocket and drill machining intent. No ShopBot
post or machine profile implemented. Evidence applies to the uncommitted tree
based on `7752ef281350faca18f737b3cb13986e6abd0474`; record the eventual commit
SHA after authorization.

## Corpus and method

Eight analytic shapes in each format: see the
[fixture README](../../tests/fixtures/shopbot-m1/README.md) for coordinates and
units. `crates/ivac-core/tests/shopbot_m1_qualification.rs` runs 11 tests with
format/shape/compensation/cycle loops. Static files enter production import
dispatch; missing fixtures fail. Assertions are analytical and nonvacuous.

Common milling setup: stock 140 × 140 × 10 mm, top Z0, Ø6 flat end mill,
target −5 mm, maximum stepdown 2 mm. Existing LinuxCNC output/preview exposes
profiles/pockets; GRBL explicit expansion exposes drill moves. These checks
qualify machining intent through the existing pipeline, not OpenSBP output.

| Invariant | Expected result and tolerance |
| --- | --- |
| Import | Analytic dimensions ±0.02 mm; closed/open counts and nested containment; no warnings |
| Square/rectangle profiles | Bounds ±3 mm outside/inside, unchanged on-line; Z−2/−4/−5, cut step ≤2 mm |
| Circle profiles | Radii 28/22/25 mm ±0.02; analytic arc perimeter ±0.15 mm |
| Open/disconnected | Open length 80 mm, no closing diagonal; two separate 140 mm perimeters, no cutting bridge |
| Pocket boundary | Independent point-in-polygon and edge distances; cutting segment samples at 0/25/50/75/100%; clearance ≥3−0.02 mm |
| Pocket floor/preservation | Heightmap pitch 0.5 mm, independent 1 mm sample lattice; inside and >3.5 mm from boundary: Z−5 ±0.001; outside/island and >1 mm from boundary: unchanged Z0 ±0.001 |
| Pocket variants | Rectangle, concave L, hole and island at 50% overlap; rectangle also 25/75%; depth passes; 1 mm rough allowance stays ≥4−0.02 mm from wall, final ring at 3 ±0.02 mm, phases distinguished by feed 1200/600 |
| Island chords | Radii 0.1/3/10 mm: distance ≥radius−0.00015 mm and ≤radius+0.0013 mm |
| Drill | One center (60,60), or two (30,25)/(90,25); Ø6 118° at shoulder depth −5; tip Z=−5−3/tan(59°) ±0.001 mm; new-material pecks ≤2 mm, retract to authored Z1.5, no below-stock XY or rapid into uncut material |
| Determinism | Repeated square profile/island pocket G-code and serialized toolpath exactly equal |

0.02 mm is an output comparison tolerance, not a global topology or physical
accuracy guarantee. Raster exclusion avoids asserting exact boundaries from
interpolated cells. Paths and sample counts must be nonempty. Features smaller
than the sampling pitch are not qualified by these coverage checks.

## Reproduced defects and corrections

1. **Arc bounds omitted extrema.** DXF circle D50 had valid arcs but zero
   bounding-box height from endpoint-only bounds. `BBox::from_segments` now
   includes cardinal extrema on the signed bulge sweep. Unit regressions cover
   minor arcs in both directions and a 270° major arc; import corpus covers D50.
2. **Island chords lost clearance.** Ø6 rectangle-with-hole measured about
   2.84569 mm centerline clearance at a convex island corner: about 0.15431 mm
   intrusion. Clipper round joins used 0.25 mm sagitta. Inflation now uses
   0.001 mm sagitta and adds sagitta plus one 0.0001 mm grid unit. Conservative
   stock left is bounded to 0.0013 mm in tested scales. The existing high-overlap
   test now checks all vertices for intrusion and includes bounded padding in
   first-ring selection, replacing reliance on underinflated vertices.
3. **Cascade terminal strip was missed.** Rectangle 100 × 50, Ø6, 25% overlap
   requested 4.5 mm stride. A collapsed inner ring left an 8 mm strip; one floor
   sample remained Z−3.75 instead of −5. Cascade/spiral ring stride now caps at
   cutter radius, treating requested stepover as an upper bound. The regression
   reaches the floor. Zigzag/trochoidal requested stride is unchanged.

Corrections stay inside existing geometry and offset modules. No dependency,
schema, serialized project or process-interface change. No architectural
subsystem or ADR needed. Future ProtoMAX assessment remains B and deferred.

## Validation and limits

Before source changes: 1,136 passing all-features workspace tests. Final: 1,149
passing, zero failed/ignored (11 corpus + two arc-bound tests added). Core without
CPS: 1,060 pass. Rust/WASM/frontend builds, Clippy, formatting, schema/version
checks pass. Commands and environment: [TESTING_STRATEGY](TESTING_STRATEGY.md).

No failure remains in this corpus. Remaining limits:

- Finer island joins add vertices; radius-capped cascade/spiral can add passes
  and machining time below 50% requested overlap. Performance was not measured.
  Spiral shares the corrected ring builder but has no new M1 analytic case.
  The cap does not prove coverage for arbitrary shapes/topology.
- Complex curves, degeneracies, narrow slivers, other pocket strategies,
  tabs/ramps/leads, mixed-operation sequencing and all post dialects are not
  independently qualified by M1. Boundary raster exclusions remain explicit.
- Optional viaconstructor/Autodesk refs are absent; upstream tests may report
  success without exercising these external comparisons.
- No VCarve job supplied, controller preview, physical cut, Windows/Linux run
  or Tauri bundle tested. Deflection and actual machine limits remain unmeasured.
  M4 is still required for hardware qualification.

Beads `ivac-h52u` (corpus/tooling) and `ivac-n92a` (defects) closed after passing
software checks. Existing upstream statuses preserved.

## Handoff

Review and explicitly authorize M1 commit/push. M2 requires separate approval:
CPS/native ADR, authoritative OpenSBP syntax research, minimal deterministic
post and unsupported-semantics tests.

STOP — waiting for human approval before starting the next milestone.
