# ADR-0018 — Bounded process-local ShopBot post result reuse

2026-10-15. Accepted implementation direction by user authorization of PERF1-B
following PERF1_A_AUDIT; performance/qualification acceptance remains pending.

## Context

The representative Test CPS post takes 98.4% of profiled pipeline time. Existing
PipelineCache already owns per-operation results and an interpretation memo.
Whole-program CPS recording remains uncached and geometry validations run anew.

## Decision

EXTEND PipelineCache with a separate four-entry LRU, maximum 16 MiB of accounted
entry storage (keys, owned string/vector capacities and entry structs; allocator
metadata and fixed container allocation are not included). No disk persistence.
Cache only successful `PostOutput` for selection `Bundled { id: shopbot }`.
Inline/path selections and other posts always execute, even with identical source.

Exact serde bytes of source, filename, entire Program, properties, pipeline/IR
versions and default execution budgets are the key. Runtime/Boa/prelude/build
identity is immutable for the cache lifetime, implicitly scoped by process;
restarting/upgrading starts empty. No hash collision can grant a hit.
The existing bundled script and deterministic host have no external file/network
or time-dependent input on this path. A future nondeterministic bundled change
requires revisiting eligibility; arbitrary CPS scripts are not assumed pure.

Lookup is after source resolution, IR generation and cancellation check.
Insertion follows successful VM completion and another cancellation check.
Hit output retains every diagnostic and metadata field; downstream warnings,
ShopBot decoding, provenance, remaining-material analysis and freshness still run.
No failure, partial VM output or observed cancelled post is inserted. A later
pipeline cancellation may leave an already-successful post cached, but returns
no valid partial pipeline response. Existing post cancellation latency on misses
is unchanged. Check cancellation again after lookup/computation.

No lock held over VM execution. Concurrent misses may compute twice; exact keys
keep this correct. Poisoned locks and oversized entries bypass storage. Clear
uses the existing PipelineCache clear lifecycle. A concurrent in-flight successful
post may repopulate after clear, as existing memo semantics allow; it still requires
an exact input match and cannot become stale output.

## Consequences

Unchanged generation and recently revisited exact IR can hit; new work still pays
VM execution plus key/clone cost. Serialization creates transient allocations;
the retention bound is not a bound on total request/VM/response memory. No schema,
project, machine, post code, geometry tolerance or generic-operation-cache change.
No promised speedup for text/geometry/tool/depth changes that alter the IR.

Qualification: exact full responses and refusals, source/name/property/IR changes,
A→B→A, LRU and byte eviction, clearing/concurrency, cancellation, custom-post bypass,
cold/warm paired controls, first-frame and relief-ready timings. B2 Pocket reuse,
VM rewrite, changed debounce and bounded VM interruption remain separate.
