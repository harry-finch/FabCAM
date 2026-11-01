# PERF-1 — measured performance optimization

## Source and decision

Worktree `FabCAM-perf-1`, branch `feature/perf-1-measured-optimization`, starts at
accepted DOC-CONSOLIDATION-1 `62b8a6bcf8a19bc98e57af0a254adf0ae78508eb`, a descendant
of accepted SAFETY-FIX-1 and SEC-HARDEN-1. Canonical V1.1 remains
`16eb34fd07354627b3c3c2f48a227b30ba1cc3c2`. No canonical integration is implied.
The original Argon audit was unavailable; hypotheses were checked against source
and measurements rather than attributed to an unseen report.

**Accepted by the maintainer as a bounded software-qualified milestone.**
The maintainer authorized the three proposed local commits after inventory verification.
Merge, push, installer, publication, FabSpeed integration and physical machining
remain outside this checkpoint.

## Implemented changes

Only three existing production files change:

- `stage_polygon/pocket.rs`: immutable precomputed edge AABBs reject irrelevant
  distance calculations in signed clearance and path safety. Boundary order,
  winding and original narrow predicates remain. Broad-phase padding uses the
  existing `FLAT`; it does not change acceptance tolerances. A separated-box case
  still falls back to the original intersection predicate, preserving its existing
  floating-point behavior even for ill-conditioned near-parallel chords.
- `stage_polygon.rs`: extract that identical intersection predicate; test-only
  budget instrumentation compiles out of production.
- `program-qualification.ts`: compute content identity once per synchronous export
  status / warning-summary call. No cross-call cache, optimistic revision counter,
  async reuse or document-state rewrite. Export still rechecks after the Save dialog.

All original logical edge charges and cancellation checkpoints remain in place,
including pruned edges. `MAX_WORK=5,000,000`, 5,000-chord/path limits, precision,
clearance, cutter/stage attribution, output mapping and refusal rules are unchanged.
No golden was updated. No new accepted geometry or machining strategy is claimed.

## Measurement method and results

Synthetic V10 fixtures in `tests/fixtures/perf1/`: rectangle (4 source segments,
281 moves), convex (5/465), concave L (6/402), curved (4/284), island (8/2,654),
distinct cutter (5/570), near-budget curved pocket (4/890), and bounded path-budget
refusal (6/no program). Input, complete response, program and motion hashes are in
[measurement evidence](evidence/perf1/measurements.json). Shapes derive from accepted
ALPHA-3.2 software cases; the near-budget case changes only synthetic size/depth.
No private drawing is included.

Native release CLI and production default web WASM use one warmup and seven
sequential fresh runs per case. Native wall time includes process startup; WASM
compile/init are excluded, JS conversion/generation included. Direct decoded-proof
measurements use seven repetitions on identical decoded outputs; original and new
math have the same test-only instrumentation. Tests clear the pipeline cache.
Frontend uses five warmups, seven rounds of 20 edits, identical hashed inputs and
real production-bundled diagnostics/history/snapshot functions. Timings cover the
synchronous diagnostics phase, not an entire rendered gesture or WASM worker roundtrip.

Median milliseconds, measured on this macOS arm64 host:

| Area / synthetic case | Before | After | Ratio before/after | Outcome |
| --- | ---: | ---: | ---: | --- |
| Decoded finishing proof / curved | 42.41 | 18.24 | 2.33× | Exact output |
| Decoded finishing proof / near budget | 288.73 | 121.21 | 2.38× | Exact output |
| Native full CLI / curved | 117.07 | 79.03 | 1.48× | Exact response |
| Native full CLI / near budget | 538.14 | 296.57 | 1.81× | Exact response |
| Production WASM / curved | 216.77 | 160.52 | 1.35× | Exact response |
| Production WASM / near budget | 962.62 | 535.78 | 1.80× | Exact response |
| Browser diagnostics / 10,000 lines | 5.93 | 1.85 | 3.20× | Stale edits rejected |
| Browser diagnostics / 512² height grid | 15.76 | 5.26 | 2.99× | Stale edits rejected |
| Node diagnostics / 10,000 lines | 12.01 | 4.18 | 2.88× | Same shared harness |
| Node diagnostics / 512² height grid | 38.30 | 14.45 | 2.65× | Same shared harness |

Raw min/max/median, seven samples, memory observations and all eight cases are in
the evidence. For example browser drawing ranges are 5.83–6.14 → 1.84–1.91 ms,
relief 15.74–16.31 → 5.24–5.33 ms. Small cases do not show universal improvement:
native rectangle 6.11 → 6.52 ms; WASM convex 36.99 → 43.58 ms and concave
29.78 → 34.91 ms. These smaller absolute regressions are disclosed, not averaged
away. The retained benefit is concentrated in expensive curved proofs and repeated
large-project diagnostics; no universal speedup or statistical significance is claimed.

Per export diagnostic, full context serialization falls 4 → 1; per warning summary
2 → 1. The 10,000-line context is 957,679 UTF-8 bytes; relief 1,540,801 bytes.
Thus the combined six original serializations become two, with no changed identity.
`projectHasUnsavedWork` retains its original content snapshot algorithm. Browser
saved/edited/restored checks pass; separate medians stay about 0.83 ms for the large
drawing and 2.77 → 2.63 ms for relief. History cycle medians are below browser timer
resolution in these cases; this is not evidence that complete Undo/Redo is free.

Native near-budget peak RSS is 17,973,248 → 17,793,024 bytes; WASM near-budget linear
memory remains 6,160,384 bytes, curved 5,636,096 bytes. Rectangle WASM uses one extra
64 KiB page (1,835,008 → 1,900,544); precomputed bounds cost 32 bytes per edge plus
vector allocation. Browser relief observed heap across a run is 62.5–68.7 MB before,
41.1–44.2 MB after. GC, retained state and sampling affect these observations;
no leak-freedom or monotonic-memory claim follows. Node heap samples are also retained.

## Proof accounting, correctness and review

All seven per-case logical-work traces match exactly (sum, charge calls, peak).
The near-budget peak is **4,241,712 / 5,000,000**. Its aggregate 42,305,873 charges
span multiple independent original Budget instances, not one enlarged budget.
The known case still refuses the original 5,000-path proof limit; its work trace
also matches. Eight native and eight WASM before/after complete response/refusal
comparisons pass. There is no newly accepted budget-refused case.

Three new deterministic property regressions cover 20,000 segment pairs at six
reaches, boundaries/large-coordinate signed-clearance bit equality, and separated
near-parallel intersections. Existing decoded-proof rejection tests remain intact.
Focused Astra Medium read-only review found no blocker in pruning, original
predicate fallback, logical accounting, cancellation or synchronous freshness.
Its nonblocking suggestion to exercise the combined fallback and near-parallel
case was incorporated in tests. No production semantic change followed that review.

Freshness regressions cover direct edits, every Main/Finishing rate and allowance,
Pocket scope, tool geometry/metadata, stock/origin/machine/source mutations, real
preset reuse, material selection and Undo/Redo. Existing import-reload, fixed-stock,
snapshot/reopen, document replacement, post-await export and async-generation
invalidation suites remain authoritative; they run in the canonical gate. The
new count regression proves no context survives a later edit or Undo. Separate
unsaved checks assert edited=true and restored=false with real snapshot functions.

## Historical CNC and safety replay

Both [release](evidence/perf1/corpus-release.json) and
[debug](evidence/perf1/corpus-debug.json) comparisons pass **109/109**: 96 accepted,
13 refused. Complete responses include decoded movements, stage/tool attribution,
origin/SAFE-Z, regions, line mappings and warnings. All accepted program hashes
also match SEC-HARDEN-1; debug responses match its original complete response files.
[Eleven targeted SAFETY refusals](evidence/perf1/safety-refusals.json) retain exact
errors and emit neither programs nor two-sided files. The original Helix, overcut
and affected two-sided restrictions remain; cancellation, export, HTTP/import/CSP
regressions pass in the canonical suite.

An exploratory cross-profile check initially stopped on last-bit region coordinates
for `alpha32-circle-walls_only` when comparing release with historical debug.
The unchanged release baseline has the same discrepancy. No predicate was changed
to accommodate it: final comparisons use matching profiles (original/candidate
release and accepted/candidate debug), both complete and exact. Raw failed and
successful checks remain local; no golden or historical evidence was rewritten.

## Chaining decision

No production chaining change. Existing endpoint spatial hashing already avoids
the suspected repeated full search/front insertion. Seven repeats cover ordered,
random, reversed, micro-gap, near-endpoint, competing and open/multiple contour
cases at approximately 100/1,000/5,000/10,000 segments. Median ranges across variants:
0.014–0.018 / 0.135–0.166 / 0.681–0.874 / 1.596–2.115 ms. Source coordinates,
ordered membership and open/closed results are repeat-identical. This cost does
not justify a third algorithm change. No automatic welding/snapping is introduced.

## Qualification and preservation

Canonical `scripts/pre-release.sh`: **20/20 PASS**, executed once on stabilized
runtime/test source. **1,689 Rust tests pass / 3 ignored; 1,572 frontend tests pass
/ 2 skipped**. The added manual benchmarks were executed separately; the inherited
ignored worker/skipped case retain their existing suite policy.
[Gate and review receipt](evidence/perf1/qualification.json).
Focused Rust pocket tests: 4 pass (three new properties plus existing decoded
mutation test); frontend targeted suites: 45 pass, one explicit manual benchmark
skip; strict core Clippy, frontend lint and type checks pass. Two ignored Rust
manual benchmarks were explicitly run serially; the frontend manual benchmark was
explicitly run for original and candidate. Ordinary CI does not assert machine timing.
Invalid sandbox-RSS and overlapping exploratory timing attempts remain local and
are excluded from reported measurements.

Final software inventory: **1,293 non-Markdown files**, fingerprint
`b42027a117a0529a974f8c5746d0794803687f7a6a474cd1c305a6fbf4643d27`.
[Exact inventory](evidence/perf1/software-files.json) extends the accepted 1,271
non-Markdown entries with 22 explicit PERF-1 test/fixture/benchmark/script paths.
Only the three production paths above change among existing non-Markdown blobs;
generated contracts, dependencies, security policy and historical fixtures stay intact.
Final inventory verification passes after qualification. Only inventory-script
enumeration was adjusted during the gate to remain usable after staging/commit;
its syntax/inventory checks were rerun. Runtime, tests and gate inputs remained
frozen. Final native executable and default WASM/JS match the measured artifacts.

All six other worktrees retain their original HEAD and porcelain-state hash,
including dirty canonical V1.1 and the safety-audit worktree. Their ignored evidence,
archives and binaries are not edited; benchmark artifacts are isolated copies in
this worktree. Raw data/logs/binaries and the temporary browser harness builds stay
ignored under `dist-test/perf-1/`. The benchmark tab/server were closed without
touching user servers. Installed-native, remote HTTP and physical acceptance remain
unqualified; existing safety/security exclusions remain binding.

## Accepted local commit groups

1. `perf(core): prune finishing distance checks conservatively` — two core files,
   original logical accounting, colocated property regressions and their referenced
   test-only Rust benchmark module (required by `mod perf1`).
2. `perf(frontend): share synchronous generation freshness checks` — diagnostics
   implementation, count/mutation regressions, shared benchmark harness.
3. `test(perf): qualify measured gains and exact CNC preservation` — synthetic
   requests, reproducible drivers, sanitized receipts and
   this report/current-state updates. Depends on both implementation groups. Exact
   paths are in [the proposed inventory](evidence/perf1/proposed-commits.json).

Deferred: persistent identity caching, unsaved-state redesign, tile/spatial indexes
beyond edge bounds, transport rewrites and chaining changes. Any further work needs
its own evidence and scope. [Local consolidation receipt](evidence/perf1/consolidation.json)
records commit/source preservation. **HARD STOP after the three authorized local commits.**
