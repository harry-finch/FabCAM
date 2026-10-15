# PERF1-B1 — bounded ShopBot post reuse

2026-10-15 · software qualification complete; human review pending · **HARD STOP before further work**.

## Implemented scope

User approved B after PERF1_A_AUDIT. This delivers B1 only: bounded exact-input
memoization of successful bundled ShopBot post output in PipelineCache. Four LRU
entries, 16 MiB accounted retained storage; no disk persistence. Geometry/IR
construction and validation still execute. All warnings, extension/metadata,
ShopBot decoding, line mapping, freshness and remaining-material analysis remain.
Custom posts are not memoized. See [ADR-0018](adr/ADR-0018-bounded-shopbot-post-reuse.md).

The audit's evidence defects are corrected: actual request hashes checked before
running, missing/stale response artifacts rejected, mixed outcome series rejected,
refusals checked across repetitions, missing comparisons explicitly `not_verified`,
response-path arguments instead of fixed /tmp reliance, and unequal path lengths
never assigned a misleading truncated coordinate delta. Native/WASM new evidence
carries exact response hashes. Old A evidence is preserved, not rewritten.

Opt-in browser diagnostics now distinguish timer scheduled/fired, configure begin/end,
replay begin/end, LOD ready, and the first actual render after LOD readiness. The
existing one-second debounce and simulation algorithms are unchanged.

## Paired native results

Same Apple M4 environment as A, release cps+profiling on both sides. Original A
profiled binary retained; candidate built from B sources. Three fresh-process pairs
per case, serial and alternating control/candidate order, seven calls/process:
first process-cold, second warmup, five warm measurements. No compilation alongside
measurements. Thus warm n=15 per side, cold n=3. OS page caches not flushed.

| Case | Control warm median | B1 warm median | Warm reduction | Control cold median | B1 cold median |
| --- | ---: | ---: | ---: | ---: | ---: |
| Test medial CPS | 832.38 ms | 15.36 ms | 98.16% | 833.61 ms | 839.35 ms |
| Many-segment profile CPS | 1276.59 ms | 8.23 ms | 99.36% | 1310.64 ms | 1279.09 ms |
| Pocket islands CPS | 427.09 ms | 353.36 ms | 17.26% | 435.93 ms | 419.88 ms |

Test cold median changes +0.69%, within observed small-sample variation; no claim
of faster novel work. The native hit target <=400ms / >=50% improvement is exceeded.
Pocket improvement is only eliminated CPS work, not a B2 optimization.

Process peak RSS medians: Test 84.4→76.8 MiB, many-segment 136.5→126.1 MiB,
Pocket 38.5→32.5 MiB. These are seven-call process peaks, not allocation counts or
proof of long-session memory behavior. Cache retention is bounded independently
by implementation/tests. Transient IR serialization and active VM memory remain.

Raw records: [paired summary](perf1-b/PAIRED_SUMMARY.json), `perf1-b/paired/`,
`perf1-b/native-profile/`. `scripts/perf1-b-summary.py` reproduces paired statistics
and verifies paired response hashes. `scripts/perf1-paired.py` reproduces ordering.

## Output and guard evidence

All 18 A requests re-run seven times on B. Four refused case/post combinations
keep the same errors; all 14 returning responses match A SHA-256 exactly (including
the two empty-source diagnostics, which are not accepted machining). Repetitions
are stable. [Comparison evidence](perf1-b/output-comparisons.json).

New private-cache tests compare full responses against uncached execution for
unchanged input, changed depth/feed/diameter/name/clearance and A→B→A. Other tests
cover exact source/name/properties/IR keys, custom-post bypass, changed-script and
spindle-property refusals, warm cancellation, count/byte eviction, oversized bypass,
clear and concurrent access. No global cache resets used by these tests.

First generation or genuinely changed IR still executes the post. Cancellation
latency while the VM is already executing is unchanged; there is no claim that a
cache fixes cancellation misses. No physical or native desktop qualification.

## Validation gates

- Core release cps+profiling unit suite: 1081 passed, serial (existing global
  cache test isolation constraint); includes Halfpipe and V-Carve regressions.
- Independent V-Carve qualification: 10 passed; ShopBot post guards: 7 passed.
- Frontend: 1263 passed, 1 optional skip in 139 files. Check: zero errors/warnings.
- Python evidence regressions: 3 passed. Rustfmt, Clippy `-D warnings`, frontend
  lint/format and release frontend/server builds pass. Existing build chunk-size
  and mixed import warnings remain.

No change to geometry, tolerances, post script, OpenSBP policy or project schema.
The frontend simulation timing probes are diagnostic only.

## Browser qualification

Actual in-app browser, same release frontend port 18795, retained A HTTP server
18794 versus new non-profiled B server 18796. Same saved Test project, alternating
serial calls, seven generations per side; warm medians use runs 3–7 (n=5).
First two runs are excluded, no compilation during measurement. This is a small
single-session comparison, not a universal latency guarantee.

| End-to-end marker | Control | B1 | Reduction |
| --- | ---: | ---: | ---: |
| Response received | 822.8 ms | 52.9 ms | 93.6% |
| First rendered 3D frame | 899.2 ms | 141.4 ms | 84.3% |
| First frame after relief calculation | 2259.4 ms | 1515.5 ms | 32.9% |

The relief is not ready at the first wireframe frame. New probes isolate the
unchanged debounce (~1001 ms), simulation configure (292→309 ms median) and actual
replay (63→63 ms). Thus post reuse improves first feedback strongly, but cannot
remove the separate simulation wait. Repeated-run growth from A was not reproduced
in this paired session; its cause/memory behavior is not established. No simulation
optimization justified solely by these data.

[Raw control](perf1-b/browser-control.json), [raw candidate](perf1-b/browser-candidate.json),
[summary](perf1-b/BROWSER_SUMMARY.json). Reproduce with
`python3 scripts/perf1-browser-summary.py` after capturing seven completed runs.
The browser download has the exact native program SHA-256
`8ad60bebff66c831019a0ae8d08bd94fd089dfdfee01b01bc20bfbaacf99b12d`,
with 18874 preview moves. See [export comparison](perf1-b/browser-output-comparison.json).

UI invalidation: depth -1→-0.8 disables stale export; regeneration yields 17788
moves and restores export. Undo restores -1, marks stale again, and regeneration
returns 18874 moves. The saved fixture is unchanged. Additional timings are in
`perf1-b/browser-invalidation.json`, excluded from the seven-run benchmark.

## Handoff

Engineering ivac-rnsg and timing attribution ivac-lf1e complete. Human result
review **ivac-sf7y OPEN — HARD STOP**. B2 Pocket analysis, VM interruption
(ivac-sxmw), further optimization and publication require separate authorization.
No commit/push, machine motion, native desktop or physical qualification performed.
[Final browser preview](perf1-b/PERF1-B-Test-http.png).
