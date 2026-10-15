# ADR-0020 — Connect cooperative CPS cancellation to the existing host probe

2026-10-15. User accepted PERF1-B2 and authorized the proposed cancellation work.

## Audit and decision

KEEP Boa0.21.1, the synchronous pipeline and transport cancellation contracts.
EXTEND the existing `__ivac.abortCheck` host function: it currently always returns
false, although the unmodified generic driver already consults it every1024
records. Connect it to the caller's monotonic atomic cancellation flag.

`ivac-cps` remains independent of ivac-core. A new additive cancellable API accepts
an owned Arc<AtomicBool>; existing run_post/run_post_with_limits behavior and
budgets remain. The default Engine host still reports false. A GC-traced wrapper
holds only an ignored Rust atomic handle (no GC references), using Boa's safe
capture constructor. The already-locked boa_gc0.21.1 becomes a direct dependency
for its Trace/Finalize derives; no new VM implementation or runtime thread.

Check cancellation before initialization, after bundle evaluation, after IR
conversion, after execute and before returning a successful output. The atomic
flag is authoritative: exception text containing the internal cancellation marker
cannot forge a Cancelled outcome. If a post catches the driver's exception, the
post-call atomic check still refuses the result. Cancellation exposes no partial
PostOutput. Pipeline mapping already produces Cancelled and skips successful-post
cache insertion; successful uncancelled caches and output remain unchanged.

## Boundary and alternatives

This is cooperative, not instruction-level preemption. Parsing, IR conversion,
individual callbacks (including onOpen/onClose), native built-ins and loops that
never return to the driver may delay cancellation. Existing finite loop/recursion
budgets remain. A measured ≤200ms p95 target concerns the representative bundled
ShopBot fixtures only, never arbitrary user scripts or a hard real-time bound.

Boa also offers budgeted async Script evaluation. Using it throughout bundle
execution and exported callbacks would require a separate execution/cleanup and
error-attribution design; dropping a suspended future is not assumed equivalent
to safe synchronous termination. A subprocess boundary has packaging/transport
costs. Both are DEFERRED pending explicit scope approval, not implemented here.

Browser WASM already uses worker termination for hard cancel and its default build
has CPS disabled. This change concerns cooperative native HTTP/Tauri calls that
pass a token; it does not establish on-device native desktop qualification or
change WASM worker/fallback behavior, machine output or geometry.

## Qualification

Compare fixed-delay native and actual HTTP cancellation with retained B2; record
requested/returned times, all terminal outcomes and absence of result/partial
output. Repeat uncancelled full-response/hash checks, retry after cancellation,
cache non-insertion and post guard/budget tests. No physical-machine execution.

## Outcome

See [qualification results](../PERF1_CPS_CANCELLATION_RESULTS.md). Cooperative
latency improves materially, but native/HTTP Test p95 remains216/218ms: the200ms
target is NOT qualified. Do not infer a hard bound from a1024-record checkpoint.
Phase attribution and stronger interruption remain deferred (ivac-q1nz1), subject
to human review ivac-nh6q. No phase-level cause is asserted without measurements.
