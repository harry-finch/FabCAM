# ADR-0023 — Validate milling programs before executable publication

Status: **ACCEPTED — implemented locally, human review pending — approved by user on 2026-10-18**, 2026-10-18.
Scope: SAFE-Z / TEST1 D2, ordinary supported milling. Tracking: ivac-h51gq.
User authorized implementation on 2026-10-18. No commit/push/publication authorized.
Results: [pipeline61 checkpoint](../../releases/ADR0023_PUBLICATION_GUARD_RESULTS_2026_10_18.md); human gate ivac-8371y.

## Evidence and existing boundaries

The extended SAFE-Z checkpoint reproduces three policy gaps:

1. `run_pipeline` returns executable text and `fixture_path_collision` warnings
   together. UI export blocks them only when `blockOnCriticalSimWarnings` is true.
2. `stream_gcode_to_writer` publishes native output without the preview-derived
   fixture check. A fixed colliding fixture produces467 executable bytes and no
   fixture warning. The preview-aware stream adds its warning only after writing.
   HTTP `/generate/gcode` commits200 before those checks could finish. CPS raw
   streaming already refuses explicitly and writes nothing; retain that behavior.
3. `HolderProfile::from_tool` can return Some with height0 when only a shank
   diameter is specified. Testing Option presence does not prove model completeness;
   the current pipeline can omit the unknown-clearance warning in this case.

The geometric detector is intentionally conservative (Z interval plus maximum
body radius; arc chord capsules have their existing margin). A detected overlap
can be a false positive; it must not be described as an exact physical collision.
A clean result with incomplete metadata is not evidence of a clear machine setup.

## Recommended decision

- **KEEP** async workers, request ownership, cancellation, current CAM/IR/post
  boundaries, stock preflight, explicit ShopBot unsupported guards and project data.
- **EXTEND** the common milling validation boundary: a detected collision with a
  declared cutter/body envelope blocks publication of executable output, independent
  of a UI preference. Retain diagnostic geometry/preview and fixture/operation IDs
  in a failed/non-exportable result; never mark the operation successfully generated.
  Exact error-payload shape must be checked against schema and existing clients at
  implementation. Do not expose the executable program in that rejected response.
- **EXTEND** completeness diagnostics separately from collision status. Partial,
  absent or zero-height body metadata must remain explicitly unknown; no invented
  body dimensions and no claim that a shank radius establishes a full holder.
  Unknown alone is not a new global prohibition. Distinguish a known envelope
  overlap from the existing unknown-height conservative footprint warning; retain
  a real cutter collision even if holder dimensions are unknown.
- **EXTEND** UI export to honor the common collision rejection. The existing
  preference must no longer bypass this particular publication guard. Unrelated
  warning preferences keep their present meaning. Preserve stale-result/Undo/Redo
  and post-dialog freshness checks.
- **For V1, refuse direct executable streaming when declared fixtures require
  validation**, before any byte/status success. Route callers to buffered validated
  generation. Apply this to both raw and preview-aware write-through APIs. This is
  the smallest reversible compatibility change; it avoids adding a new disk spool
  subsystem or retaining an unbounded program in an allegedly streaming route.
  Progress/event streaming remains asynchronous and supported; this restriction is
  about publication of program bytes, not UX progress or cancellation.
- No numeric Z translation, automatic clearance changes, automatic collision
  avoidance, stock-removal engine, new machine motions or ShopBot hacks in geometry.

## Compatibility and alternatives

Saved projects/tools remain unchanged. A project previously exportable by ignoring
fixture warnings may now yield a non-exportable diagnostic result. Headless clients
using fixture-bearing direct streams must switch to validated generation. These
are intentional behavior changes requiring approval, not implicit bug-fix authority.

Rejected for V1 recommendation: keep a user bypass (fails the common guarantee);
validate only after sending bytes (too late); generate twice and assume identical
custom-post output (unproven); add atomic disk-spooling everywhere now (larger
cross-platform lifecycle/security surface). Spooling can be separately designed
later if validated large-file streaming becomes necessary.

Non-milling processes, arbitrary machine-frame posts/stations, unknown initial
machine pose and physical dimensions outside the declared model remain unqualified.
No generic process abstraction is added for future waterjet/drag-knife work.

## Required acceptance evidence after approval

- Same fixture cases through core/HTTP/Tauri/WASM: known collision cannot return
  executable success; diagnostic preview survives through the agreed error contract.
- Direct streams with fixtures refuse before headers/bytes; cancellation/failure
  cannot leave a usable output. No UI/transport escape hatch for the same collision.
- Missing/partial body metadata gives explicit unknown status, with real cutter
  collisions still blocked and complete declared bodies exercised analytically.
- Existing non-fixture jobs, source/save compatibility and supported ShopBot output
  retain regression parity. Two-tool transitions and common-frame translation pass.
- UX review of the non-exportable diagnostic state and another HARD STOP.
