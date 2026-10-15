# NEST2 — Automatic True-Shape Nesting

2026-10-14 · **PLANNED — NOT IMPLEMENTED · FUTURE / ADVANCED**.
Build on qualified NEST1 sheets/parts/instances/constraints and shared transform model.
No optimizer/library chosen; [shared plan](NESTING_PLAN.md) preserves current priority.

## Objective and algorithm research checkpoint

Useful bounded-time placement of irregular outlines, not bounding-box packing alone:
quantities, sheet size, spacing, orientations/grain, keep-outs and material utilization.
No global-optimality promise. Audit polygon packing/no-fit polygons, constructive
heuristics/local search and suitable existing libraries for FabCAM license compatibility,
Rust integration, robustness, determinism, holes/nonconvex shapes and performance.
Do not implement a large new geometry library or add a dependency in this task.

Document objectives/tradeoffs (sheet count, true material area, unused area, time,
grain/fixture constraints) rather than one opaque score. Repeatability needs defined
seed/input/policy/budget; qualify deterministic or bounded-time behavior and limits.
Any result must pass independent placement/collision checks, not just optimizer checks.

## Proposal, metrics and advanced boundaries

Configure → run → preview utilization/warnings → explicitly accept/reject → manually
adjust. Preserve current manual arrangement until accepted, allow comparison and
coherent Undo/Redo. Cancel/timeout/impossible result leaves prior layout intact and
explains limitations. Layout acceptance is not a machining-safety verdict.

Define sheet area, summed true part area, occupied layout area, unused area, utilization
and sheet count with explicit hole/usable-area conventions. Bbox occupation is a
different metric; approximations and internal-hole treatment must be labeled.
Qualify placement into holes separately; do not assume it is always allowed or machinable.

Multi-sheet is a phased advanced extension after single-sheet true-shape qualification:
identical/different sheets and remnants when supported; no full material inventory
requirement. Each supported sheet/setup/output mapping needs separate evidence.
Common-line cutting is a separate future extension, excluded from NEST1/NEST2 initial
acceptance: shared kerf edges alter cut topology/order/leads/separation/hold-down/output.
It is not a packing option presumed safe for milling or waterjet.

## Future qualification and acceptance

Independent checks for irregular nonconvex polygons, multiple types/quantities, holes,
narrow gaps, allowed rotations/grain/mirror rules, margins, keep-outs, impossible layouts,
seeded repeatability, bounded performance/cancellation, exact versus approximate metrics,
manual adjustment after acceptance, history/save-reopen and normal CAM/toolpath parity.
Multi-sheet tests only when that phase is supported, including identity and output mapping.

Acceptance: audit-approved algorithm/license and measured budget; no constraint-violating
proposal; deterministic policy/limitations stated; independent geometric oracle; explicit
non-destructive acceptance and manual control; lifecycle/CAM regressions, desktop
accessibility/captures and human gate. No physical machining proof from arrangement.

Open: algorithm/dependency, collision representation/tolerance, objectives/tie-break/seed,
budgets, hole policy, metrics, multi-sheet scope, process-clearance inputs and library
maintenance. Resolve in future audit, not frozen now.

**STOP — waiting for human approval before implementation.**
