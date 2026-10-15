# CPS cooperative cancellation — results and remaining limit

2026-10-15. User accepted PERF1-B2 and authorized this follow-up (ivac-sxmw).
**Cooperative improvement implemented; the proposed p95 ≤200 ms target is NOT
qualified. HARD STOP for human review (ivac-nh6q).** No physical qualification,
publication, generic VM rewrite, or next milestone started.

## What changed

The existing generic CPS driver's probe, already called every 1024 records,
now reads the native cancellation token instead of always returning false.
The additive `run_post_cancellable` API and phase-boundary checks return
`PostError::Cancelled` without exposing a partial `PostOutput`. The pipeline
retains its cancellation/error mapping and refuses successful-post cache insertion
when canceled. A fake cancellation marker in a JavaScript error remains a runtime
error unless the actual token is canceled. See [ADR-0020](adr/ADR-0020-cooperative-cps-cancellation.md).

Geometry, ShopBot post source, generic JavaScript driver, runtime budgets,
output formatting, and UI source are unchanged by this follow-up. The existing
locked boa_gc dependency is now direct for the safe host-capture tracing wrapper.

## Measured cancellation latency

Elapsed time from cancellation request to pipeline return (native), or terminal
SSE event (HTTP). Release macOS ARM64; B2 retained binary is hash-verified against
B2 evidence. Fixed-delay requests are end-to-end measurements, not proof of the
exact VM instruction active when cancellation arrives. p95 uses nearest rank.

| Case | n per version | Median before → after | p95 before → after |
| --- | ---: | ---: | ---: |
| Native Test, cancel at 50 ms | 20 | 750.240 → 212.933 ms | 760.444 → 216.292 ms |
| Native dense contour, cancel at 50 ms | 10 | 1180.615 → 193.342 ms | 1191.827 → 201.581 ms |
| Native Test, cancel at 1 ms | 5 | 0.673 → 0.662 ms | 0.783 → 0.722 ms |
| HTTP/SSE Test, cancel 50 ms after token | 10 | 743.341 → 206.894 ms | 861.627 → 217.530 ms |

All 70 final native observations returned Cancelled and no successful result.
All 20 HTTP cancellation runs acknowledged the token and terminated with
`cancelled`, without `result` or `error`. Afterward both servers' successful retry
responses equal the full native response, including NC text, preview and diagnostics.

**Performance assertions intentionally fail** in both measurement scripts because
the 200 ms target is missed. This is not a green performance gate. The initial
native series overlapped the output matrix; its summary is retained under
`initial-overlapping-run` and is not used above. The final native series was run
without concurrent compilation/benchmark. Matrix timing values are not used for
speed or overhead claims; matrix output equality remains valid.

Raw evidence: [native](perf1-cancel/NATIVE_SUMMARY.json),
[HTTP](perf1-cancel/HTTP_SUMMARY.json), [environment](perf1-cancel/ENVIRONMENT.json),
[18 output comparisons](perf1-cancel/output-comparisons.json).

## Correctness and UX qualification

- 44 CPS tests pass: host reads live atomic, pre-canceled scripts never execute,
  uncanceled API output parity, forged marker refusal, existing golden outputs,
  differential tests, budgets and ShopBot guards.
- 1086 core unit tests pass, including canceled execution followed by a cache miss
  and exact retry output parity. Timing is measured separately from unit assertions.
- Integration runner reports 41 pass: 4 CPS pipeline, 27 ShopBot, 10 V-Carve.
  The optional FANUC oracle is absent and returns early: 40 checks actually run,
  no new FANUC qualification claimed. The first
  integration run exposed an obsolete two-sided success expectation, contradicting
  the already documented CAM2 guard (ivac-jtkj). The test now checks structured
  WorkOrigin refusal; production guard unchanged. Initial and final logs retained.
- Clippy `-D warnings`, release probe/server builds, cargo check and format check pass.
- All 18 original fixture outcomes match B2 exactly: successful full-response
  hashes or identical refusal text. Seven calls per fixture remain consistent.
- Actual HTTP UI: open saved Test demo, rename operation to force cache miss,
  Regenerate → Cancel. Previous preview remains stale; Download .sbp disabled.
  Regenerate again succeeds: 18,874 moves, 18,982 NC lines, Download enabled.
  Screenshots: [canceled](perf1-cancel/UI-cancelled.png),
  [retry](perf1-cancel/UI-retry.png). No UI redesign or saved user-project overwrite.

The browser review proves state transitions, not a visually timed latency claim.
Existing V-Carve remaining-material and software-only warnings are retained.
The default WASM worker implementation and native desktop app were not newly
qualified here; this qualification uses the native core and actual HTTP transport.

## Reproduction

From repository root, with no competing benchmark/build:

```sh
RUSTC_WRAPPER='' cargo test --offline --release -p ivac-cps
RUSTC_WRAPPER='' cargo test --offline --release -p ivac-core --features cps,profiling --lib -- --test-threads=1
RUSTC_WRAPPER='' cargo clippy --offline --release -p ivac-cps -p ivac-core --features ivac-core/cps,ivac-core/profiling --lib -- -D warnings
RUSTC_WRAPPER='' cargo test --offline --release -p ivac-core --features cps,profiling --test cps_posts --test vcarve_qualification --test shopbot_m2_post
python3 scripts/perf1-cancel.py --control /private/tmp/fabcam-perf1-cancel/control --candidate target/release/examples/perf1_probe
python3 scripts/perf1-cancel-http.py
```

HTTP harness requires freshly started B2 server18797 and candidate18798; run it before
uncanceled requests populate their caches. It intentionally fails if cancellation
cannot be acknowledged, a result escapes, retry differs, or p95 exceeds200ms.
Raw records include request/binary hashes. The retained binary is a local temporary
artifact; source hashes identify the dirty working-tree candidate.

## Remaining decision

Cancellation is cooperative. Parsing, JSON-to-JS conversion, long individual
callbacks (including ShopBot whole-program preflight), and context cleanup may
still delay return. These phases have NOT been timed individually here; none is
claimed to be the demonstrated cause of the remaining ~213 ms.

Follow-up **ivac-q1nz1**: attribute these phases first, then propose a generic safe
interruption/execution boundary if justified. Do not weaken post guards, change
geometry or promise a universal hard deadline. This requires a separate approval;
review gate **ivac-nh6q** remains open. The original latency objective remains
partially unmet despite the measured improvement.
