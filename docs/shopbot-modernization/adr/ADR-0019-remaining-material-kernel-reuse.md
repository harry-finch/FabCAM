# ADR-0019 — Bounded reuse of the remaining-material planar kernel

2026-10-15. PERF1-B2 authorized after user acceptance of B1.

## Decision

EXTEND the existing PipelineCache with a separate four-entry exact-key LRU,
8 MiB accounted retained allocation cap. KEEP the remaining-material algorithm,
all numerical constants, region construction, stock and operation eligibility,
execution-path qualification, area calculation and diagnostics unchanged.

Reuse only `remaining_regions(targets, moves, radius, depth)`, after every existing
eligibility check. This is a pure planar subtraction; output is cloned as native
coordinates, never rounded through serialization. Full ordered targets (including
holes and operation IDs), full final movement records, radius, depth and pipeline
version form exact serialized key bytes. Nonfinite coordinates bypass reuse.
Numerical constants and runtime are process-immutable; nothing is persisted.

Stock, units, strategy, source representation, tabs, allowance, tool kind and
execution provenance are deliberately NOT bypassed: they are checked each time
before lookup. A changed stock can reuse a qualified kernel only if its actual
kernel inputs remain identical. Unsupported results are not cached.

All backends using PipelineCache share this extension; no CPS, ShopBot or UI
semantics are introduced into the kernel. Public models/schema/project storage
and frontend freshness rules are unchanged. Remaining analysis stays synchronous.

## Limits and validation

Count/key/result-capacity eviction, exact input equality, clear lifecycle and
poisoned-lock bypass; no lock held over Clipper execution. Concurrent identical
misses may duplicate work. In-flight work may repopulate after clear, but requires
an exact key on subsequent lookup. A complete pure kernel may remain cached if
its enclosing request is later cancelled; partial responses are never authorized
by a cache hit. Existing synchronous cancellation latency is not solved here.

The 8 MiB cap counts retained keys, region/vector capacities and entry structs;
allocator metadata, fixed container allocation, transient serialization and
active geometric calculation are excluded. More than four distinct qualified
pockets can evict each other; no guaranteed speedup for all projects or edits.
Oversized inputs/results bypass reuse. First calculations still pay the kernel
plus a small identity/copy cost. Existing CPS cache has its separate 16 MiB cap.

Qualification must compare complete pipeline responses (including analysis),
change targets/holes, movements, depth, cutter and stock eligibility, and measure
paired cold/warm stage and pipeline CPU/RSS. Target: at least 50% repeated-stage
reduction on the A Pocket islands fixture, with exact unchanged results.
