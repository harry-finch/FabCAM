# Post-audit engineering consolidation — bounded release policy

> Consolidation checkpoint — 2026-10-18: [combined results, confirmed blockers and approval boundary](POST_AUDIT_CHECKPOINT_2026_10_18.md); [TEST1 contracts](TEST1_CONTRACTS.md), [SAFE-Z contracts](SAFE_Z_CONTRACTS.md). Assessment complete, implementation awaiting approval.

2026-10-18 · **PLANNED — no implementation authorized**.
Product decisions: V1 stable/installable/qualified ShopBot; V1.5 near-term verified
ProtoMAX; V2+ broader advanced manufacturing and architectural/performance work.
Recommendations are not completed capabilities or prerequisites for every release.

## Release-critical path

Current audit-correction checkpoints → unresolved safety/correctness review → integrated
regressions on combined final source → bounded [TEST1](TEST1_PLAN.md) → only demonstrated
release-critical geometry consolidation → rebuild/test desktop packages → selected
platform installation/runtime gates → planned physical ShopBot qualification → RC/docs/
explicit V1 acceptance. Repeat affected integrated/property/output checks after any fix
or consolidation; tests on a previous tree do not qualify changed source/packages.
Required platform/physical gates and current human audit reviews are not closed here.

After V1: time-boxed [AUDIT3](AUDIT3_PLAN.md), evidence-backed recommendation backlog,
separate critical-regression triage, authorized ProtoMAX feasibility. V1.5 then prioritizes
verified output contract, explicit waterjet model, software/output tests and physical
qualification. Consolidation enters V1.5 only if specifically necessary or demonstrated
low-risk and separately approved, never as a blanket prerequisite or delivery expansion.

## Recommendation disposition and implementation gates

| Milestone | Release target | Bounded contract / advancement trigger |
| --- | --- | --- |
| TEST1 — Property-Based Geometry & CAM Qualification | **V1 REQUIRED**, strictly bounded | Highest-priority new work after current audit review/integrated baseline; contracts A/B/C/D, deterministic budgeted CI, reproducible local extension, saved minimized failures. Expanded fuzzing POST-V1 |
| GEOM-CORE — Canonical Geometry Primitives | **V1 CONDITIONAL / POST-V1 CONSOLIDATION** | Pre-V1 only duplicated primitive already causing confirmed defect or demonstrable remaining supported-ShopBot risk; otherwise document duplication and defer systematic migration |
| CAM-DIAG — Explicit Diagnostic Contexts | **POST-V1** | Replace implicit diagnostic channels only for demonstrated remaining correctness defect or approved parallelism requirement; operation-scoped typed results/context, compatible caller migration |
| TOOL-TYPES — Explicit Tool Diameter Semantics | **POST-V1**, unless release-critical defect | Stronger explicit accessors/newtypes/effective-cutter contracts after compatibility audit; targeted defect fix before V1, broad type-level migration later |
| PERF2 — Selective Parallel Computation | **V2+ / PERFORMANCE-DRIVEN** | Measured representative hotspot and independent regions; deterministic/cancellation/diagnostic/memory/backend proof before choosing Rayon or another implementation |
| TIME2 — ShopBot Runtime Calibration | **POST-V1**, potentially V1.5 if practical | Real workshop measured dataset, estimator/parameter consumer audit and independent validation; never delays initial ShopBot physical qualification |

All six entries are **PLANNED**, not IN PROGRESS or SOFTWARE/PHYSICALLY QUALIFIED.
AUDIT3 remains separately PLANNED post-explicit-V1 acceptance and time-boxed before
main ProtoMAX campaign; no Git archaeology here. Current source beta.2/pipeline56
and pipeline54 package evidence retain their exact scopes, not release acceptance.

## GEOM-CORE — canonical contracts, not mechanical replacement

Candidate shared primitives: signed contour area, exact arc-aware bounds, full-circle
recognition, endpoint continuity/closure and relevant containment/intersection. Reuse
existing primitives; audit whether a light SegmentChain abstraction is warranted.
Separate exact topology closure, user-distance proximity and fuzzy CAM connectivity.
Polygon inclusion, region Boolean containment and cutter-envelope clearance have
different contracts; do not merge simply because functions resemble each other.

Each pre-V1 candidate needs defect/risk evidence, named consumers/semantics/units,
minimal reversible scope and independent before/after tests. Preserve numerical
tolerances, source identities, toolpaths, deterministic output and serialization.
No broad call-site replacement or new abstraction absent demonstrated release need.
Post-V1 systematic consolidation requires a source-backed migration/qualification plan.

## CAM-DIAG — explicit operation-owned diagnostics

Existing thread_local channels include trochoidal and offset diagnostic paths, with
scoped cleanup/nesting/cancellation improvements. Their existence alone is not a defect
or justification to rewrite now. Future CamDiagnostics/equivalent typed result/context
must preserve warning identity/severity/ownership, nested/repeated jobs, cancellation/
early-error cleanup and old caller compatibility. Prove no cross-job leakage or hidden
thread affinity, with concurrent/isolation tests before parallel callers. Do not remove
existing lifecycle guards during migration. No context schema selected here.

## TOOL-TYPES — distinct physical meanings

Nominal diameter, effective cutting geometry, tool body/shank, holder envelope and
process kerf are not interchangeable; two diameter wrappers alone do not describe
all collision geometry. Audit accessors/newtypes/typed cutter representations against
CAM, FS, preview, collision, post, wire and saved-project consumers. No private
ToolEntry field conversion or breaking API before migration/compatibility review.
Preserve manual operation values and process boundaries; no milling diameter masquerading
as waterjet kerf. Targeted proven bugs may be release-critical, broader migration is not.

## PERF2 — conditional parallel speedup, not a promise

Candidate surface_mill/drop-cutter, waterline and simulation sweep workloads require
real profiling before parallelization. Claimed **4–8× is unverified**, not a target or
result. Measure comparable sequential baseline, independent task partition, stable
ordering/reduction, cancellation/diagnostic isolation, peak memory and synchronization
overhead; compare before/after representative workflow, not just kernel throughput.
Audit native/desktop and each actual WASM feature/worker/thread configuration; Rayon
does not guarantee an automatic correct sequential fallback in every WASM build.
Do not introduce Rayon now or make parallelism a V1/V1.5 dependency. Retain existing
PERF1 evidence/cache/fail-closed contracts; accept only measured qualified benefit.

## TIME2 — workshop evidence before calibration

Current `sim/timing.rs` describes a trapezoidal model; stored jerk does not imply
implemented S-curve behavior. Audit exact consumers of rapid/acceleration/toolchange/
pause/profile fields before interpretation. Capture representative actual ShopBot
jobs: exact source/program/version/machine/controller/tool/profile/datum, simulated
versus measured runtime, cutting/rapid sections, corners/acceleration, changes/pauses
and observation method. Separate controller/operator waiting from motion where possible.

Propose calibration only from that dataset with reproducible fitting and held-out
validation, documented applicability/uncertainty. No <5% or other numerical accuracy
promise without measurements. Do not alter feed, geometry or controller settings to
match estimates. Human-authorized physical measurements remain distinct from software
qualification. Schedule after initial ShopBot physical acceptance; V1.5 inclusion is
optional if practical and not delaying ProtoMAX, not a new release gate.

## Scope, acceptance and remaining approvals

No production/test implementation, diagnostic/tool/geometry refactor, dependency,
golden/serialization/output change, hardware work or GitHub issue creation now.
Each eventual milestone begins with bounded evidence/contract review and ends at
human software review; physical and release acceptance remain independent states.
Changes affecting qualified output require independent geometry/decoded OpenSBP/
preview/reference checks, relevant combined regressions and compatible persistence.

Human decisions still needed: TEST1 exact first property set/framework/CI caps;
specific pre-V1 GEOM-CORE risk-backed candidates (or explicit no consolidation needed);
future diagnostic/tool API migration; PERF2 measurement/backend scope; TIME2 access/
dataset/protocol and optional V1.5 timing. Approving this roadmap does not start coding.
Proposed next new checkpoint: TEST1 contract/infrastructure proposal after current
audit reviews and integrated baseline; do not bypass those unfinished gates.

**HARD STOP — awaiting human approval of the updated roadmap before implementation.**
