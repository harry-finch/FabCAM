# PERF1 — CAM Performance Profiling & Optimization

2026-10-14 · **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED**.
Documentation checkpoint ivac-yrk8. No instrumentation, benchmark execution,
algorithm/tolerance/preview/post change, dependency or GitHub issue mutation here.

## Goal, placement and phase gates

Reduce measured user-visible CAM latency and redundant computation/memory use,
preserving numerical correctness, deterministic paths, output fidelity and safety.
Rust implementation alone is not a performance guarantee.

- **PERF1-A — Profiling & Benchmarking: PLANNED, HIGH / MEDIUM.** Reproduce the
  observed workload, instrument actual boundaries, characterize CPU/wall/memory and
  backend/UI overhead, then rank bottlenecks and propose numerical targets/optimizations.
- **PERF1-B — Targeted Optimization: PLANNED, priority determined by A.** Only
  approved measured targets; one bounded hypothesis/change/comparison at a time.

Current [V-Carve qualification](VCARVE_QUALIFICATION.md) and
[final UX](VCARVE_FINAL_UX.md) are software-ready, **human acceptance ivac-1b75
pending**. Preserve their evidence and HARD STOP. Propose PERF1-A as a near-term
checkpoint after that review, before computationally demanding future expansion
where approved; do not automatically displace GEOM/NEST/two-sided/inlay priorities.
PERF1-A ends with a reviewable report and **HARD STOP before PERF1-B**. A roadmap
entry is not authorization for either phase; B also ends at human acceptance.

## Existing performance evidence and architecture inventory

Reuse/extend existing infrastructure rather than invent parallel suites. Paths are
repository-relative; this is preliminary source inventory, not a new benchmark:

| Existing seam/evidence | Scope and proposed disposition |
| --- | --- |
| `crates/ivac-core/benches/pipeline.rs`, `tests/bench-baseline.md` | KEEP historical Criterion DXF import/chaining/offset/LinuxCNC emission; not modern whole-generation/V-Carve/ShopBot/UI coverage. Missing fixtures are skipped; A must inventory and disclose skips |
| `crates/ivac-cps/benches/post_bench.rs` | KEEP historical bundled GRBL whole-post throughput harness; EXTEND equivalent ShopBot workload after audit, not reuse GRBL numbers as current ShopBot speed |
| `docs/shopbot-modernization/vcarve/PERFORMANCE.txt`, `VCARVE_QUALIFICATION.md` | KEEP scoped synthetic topology-run wall/RSS and suite/movement-count evidence. Suite duration is not single-job/text generation latency |
| `frontend/scripts/palette-benchmark.mjs`, `ux/UX5-search-benchmark.json` | KEEP separate search benchmark conventions where useful; not CAM performance evidence |
| `crates/ivac-core/src/pipeline.rs`, `pipeline_cache.rs`, `geometry_cache.rs` | Existing progress, cooperative cancellation and caches; EXTEND measured stage/condition coverage after audit |
| `cam/vcarve.rs`, `cam/vcarve_plan.rs`, `cam/vcarve_validate.rs`, `pipeline/op_drivers/vcarve.rs` | Sampled medial geometry, pass/entry scheduling, continuous clearance and driver assembly; measure before choosing changes |
| `frontend/src/lib/components/GenerateBar.svelte`, `lib/api/wasm.worker.ts` | Existing busy/progress/cancel integration and worker execution; KEEP owner/lifecycle, audit actual responsiveness and cancellation coverage |

No PERF1 milestone already exists. Historical performance comments/numbers remain
historical and are not rerun, overwritten or promoted by this documentation task.
Open **ivac-htc8** tracks global pocket-cache test-count interference; serial tests
are existing workaround, not proof of concurrent cache correctness. Profiling must
identify interference rather than hide it with favorable warm-cache measurements.

## PERF1-A — observed case and benchmark collection

User reports **approximately 5–10 seconds** for a small V-Carve text design.
This is informal perceived latency, not a controlled reproducible baseline. Obtain
the exact project when A starts; if unavailable, prepare a clearly labeled representative
fixture and keep the observed and surrogate cases distinct. No benchmark values invented.

Record fixture/hash/source text and font/version where applicable, source resolution,
operation/mode/combination/depth/entry parameters, full V-bit geometry, machine/post/CPS
properties, input segment/contour/boundary-sample counts, generated moves/program size,
selected backend, OS/hardware/toolchain/commit, dirty-tree identity, build/features and
cold/warm caches. Include failed/refused runs separately; an early refusal is not a
speedup. Record import/load versus cached-project generation as different workloads.

Suite candidates: V-Carve rectangle, short text, letters with holes/acute corners,
complex text/logo in Perimeter and Full medial; Profile simple/many-segment/curved;
Pocket rectangle/islands/complex import; moderate SVG/DXF import. Include representative
qualified successes and difficult explicit refusals. GEOM cleanup/transforms, NEST1/2
and future WJ benchmark slots remain deferred until their features exist.

## Pipeline attribution and native/WASM/CPS equivalence

Audit actual call graph before fixing instrumentation boundaries: parse/source/text
resolution; flatten/chaining/regions; medial-axis construction; nearest-boundary radius
and depth; pass/entry scheduling; continuous clearance; path assembly; capability
resolution; OpenSBP post; final decoding; preview preparation; transport/serialization;
frontend buffer/render/simulation. Some stages overlap or occur outside Generate:
explicitly document measured boundaries, inclusive/exclusive durations and async spans.
Avoid double-counting nested timers or summing overlapping work as elapsed latency.

Separate wall time, CPU time, native memory/allocation evidence where practical,
serialization/IPC/network/worker startup and frontend rendering/UI latency. Distinguish
Generate click→valid result, result→first useful preview and full simulation/interaction
readiness; total user-perceived latency is not only backend calculation time.

Compare native Rust and WASM only with identical inputs and equivalent supported work.
The currently documented standalone WASM package lacks CPS; audit build/features
before comparison rather than assume every possible WASM build has that limitation.
That package cannot be a like-for-like full ShopBot run.
Compare shared generic stages separately, then native CPS/post/decoder overhead; audit
HTTP/Tauri/WASM paths and chosen post/features explicitly. Production-equivalent builds
where feasible, startup separate from steady-state. No inherent faster/slower claim.
Debug/release, differing validation/post or cache states cannot be presented as a fair
backend comparison. Record unavailable paths, not fabricated equivalence.

## Reproducible methodology and targets

Pin machine/OS/Rust/toolchain/features/build/backend and fixture identity; disclose
background load and relevant runtime configuration. Separate warmup from repeated
measured runs, report sample count, median and variability/tails with raw data;
explain outliers, missing fixtures and profiler overhead. Establish cold/warm/cache-hit
protocols and resource reset strategy before benchmarking. Use realistic small/medium
jobs plus bounded stress cases; keep microbenchmarks separate from end-to-end measures.

CPU/allocation/RSS and browser/worker memory observations need scope and tool caveats;
native RSS is not browser memory or an allocation count. Select measurement tools
during A, without unnecessary dependencies. Set interactive/medium-job latency,
responsiveness/cancel latency, bounded-memory and cross-operation regression targets
only after reproducible baselines. No arbitrary speedup promises or new CI threshold
policy in this plan; existing historical benchmark policy stays unchanged.

## Hypotheses to measure — not selected optimizations

Measure 0.1 mm boundary sampling/point counts, Delaunay/Voronoi and traversal,
repeated nearest-segment/radius work, continuous clearance candidate comparisons,
sweep/intersection/pass completion work, allocations/data locality and output
fragmentation/post/decoder/rendering. Do not assume the medial builder is the cause.

Candidates, dependent on measured impact: preprocessed-boundary reuse, correct
intermediate caching, allocation reduction, stable traversal, independent-region
parallelism, redundant post/preview work removal, and spatial R-tree/BVH/grid/segment
bounds indexing. **Indexing only accelerates candidate selection:** it must be
conservative with no false-negative boundary exclusion; the authoritative continuous
predicate remains. Audit libraries/licenses during A; none selected here.

No sampling-distance increase, short-branch pruning or simplification solely to
improve a benchmark. Approximation changes require a defined error budget, independent
references, before/after geometry/output comparison and human approval.

## PERF1-A deliverables and acceptance

Reproducible suite and observed/surrogate text fixture; applicable native/WASM/CPS
comparison; end-to-end stage breakdown; CPU/memory and point/move counts; ranked
bottlenecks with evidence; candidate approaches and correctness risks; measured
proposed targets; explicit recommendation/scope for B. Preserve raw baselines,
commands/build identities and unsupported comparisons. No optimization bundled into A.
Human report/scope review and **HARD STOP** before any B implementation.

## PERF1-B — measured bounded implementation contract

For each approved target: retain baseline → state hypothesis/expected affected work
→ one bounded change → independent correctness/regressions → equivalent repeated
before/after measurements → report latency/memory/stability and tradeoffs. Prefer
eliminating redundant work with identical accepted geometry; avoid broad rewrites,
complex caching/threading before evidence or many unrelated changes in one review.

Preserve V-bit depth, continuous envelope clearance, pass increments/completion,
overcut/undercut refusal, existing Perimeter/sampled-spine semantics and limits.
No skipped difficult regions, removed validation, relaxed tolerance, hidden incomplete
passes, silent accepted-geometry/output change or weaker fail-closed handling.
Changes to approximation/geometry/output require explicit error-budget comparison and
human approval, not speed alone. Keep unrelated Halfpipe/Profile/Pocket/Drill/FS behavior.

Cache identity/invalidation must cover relevant source/transforms, operation/context,
tool geometry, depth, combination, planner/version/tolerance and post/machine inputs
according to the cached layer. Prove warm/cold result equivalence, edit/Undo/save and
concurrent behavior; no stale reuse or concealment of ivac-htc8. Parallel/cached ordering,
source mapping and equivalent deterministic output cannot depend on scheduling/hash
iteration. Keep existing generation-context/stale-export guards authoritative.

Preserve capability/OpenSBP restrictions, source-line mapping, WCS transforms, final
decoding and resolved-preview parity. Any output/path-affecting change runs relevant
independent ShopBot/V-Carve qualification and unrelated-operation regressions. No
post speedup by omitting checks. Software performance is not physical machine validation.

B deliverables: selected optimizations; reproducible before/after and real workflow
benefit; independent geometry/safety/determinism proof; ShopBot parity; memory/stability;
updated performance report, remaining bottlenecks and human acceptance. No new CAM,
FS/cutting values, nesting, frontend rewrite or unnecessary dependency scope.

## Responsive generation and safe cancellation

Audit existing nonblocking/progress/cancel behavior before calling it missing. Core
cooperative cancellation and worker termination are different mechanisms; determine
check frequency/transport/resource cleanup and stale-result races on each backend.
Current worker cannot process an in-worker message while synchronous WASM runs.
Any improvement is separately chosen from A findings, not implemented now.

Future acceptance: UI remains responsive, truthful busy/elapsed/stage feedback,
indeterminate progress when percentages are not meaningful; cancellation distinct from
failure/timeout, no partial valid output, current project retained, resources released,
late results discarded. Previously generated results obey existing freshness rules;
cancel must never make stale/incomplete export valid. Measure cancellation latency
rather than assume immediate interruption of monolithic kernels.

Canonical [$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md)
and [$cam-ux-ui](../../.agents/skills/cam-ux-ui/SKILL.md): compact contextual state,
no permanent profiling panel/dashboard, noisy overlays, fake percentages or needless
blocking modal. Detailed timings belong in developer diagnostics; current preview
behavior does not change in this documentation task.

## Cross-milestone benefits and open decisions

GEOM1 proximity/duplicates/topology, NEST2 collisions/search and future WJ large
geometry may reuse profiling conventions and genuinely generic acceleration helpers.
No requirement to universalize V-Carve-specific code or implement those features for
benchmarks. GEOM/NEST/WJ remain separate planned functional milestones; FS unchanged.

Open: exact observed fixture/backend/build and perceived-latency endpoints, equivalent
backend subset, timing/profiling tools, cache isolation, representative sizes, attribution
overhead, responsiveness/cancel coverage, numerical targets and highest-benefit bounded
B scope. No new ADR selecting caching/index/threading architecture at this plan stage.

**STOP — waiting for human approval before starting PERF1-A.**
