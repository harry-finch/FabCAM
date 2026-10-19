# Smart Workshop features — bounded assistance, not silent automation

2026-10-19 · **PLANNED — NOT IMPLEMENTED** for the capabilities below.
[Unified V2 priorities](V2_WORKSHOP_PRIORITIES.md) protect ShopBot V1/current blockers,
native OMX V1.5 and existing approvals. Shared workflow:
**Inspect → Propose → Explain → Preview → Human approval → Apply → Validate**.

## SMART1 — Smart Layer-to-Operation Mapping — V2 HIGH PRIORITY

Reuse OpSource::Layers, current named layers/source lineage, UX4 operation templates,
machine/tools, stock/material and command/history. Existing templates support bounded
Profile/Pocket/planar-surface types; they are not already an automatic mapping service.
SVG import may flatten authoring layers/groups; do not promise named rules where source
metadata was lost. No inference from names alone proves machining suitability.

Import → inspect names/conventions → optional workshop rule preset → propose operations/
tools → consolidated preview/explanations → missing/ambiguous/incompatible review →
explicit atomic Apply. Examples are **rule proposals**: POCKET_-3mm final Z intent,
DRILL_5mm nominal-hole intent (not tool diameter/depth automatically), CUT_THROUGH
derived only from supported stock/Z convention. Missing context refuses/asks, not guessed.

Configurable deterministic rules, reusable workshop presets (including future authorized
portable exchange), explicit precedence among rule/template/user/current tool context,
conflicts/unmatched layers, units/malformed names and unsupported operation types.
Keep physical tools, saved cutting values and FS suggestions separate; no automatic
recommendation Apply. Preserve layer/source IDs and existing operation bindings/history/
save compatibility; changed layers must not silently recreate/duplicate operations.
Qualification: overlapping/conflicting rules, multiple imports/shared names, missing
stock/material/tools, source changes, deterministic/idempotent proposals, review/cancel,
Undo/save/reopen/freshness and normal CAM/output safeguards. Human architecture/UX gate.

## SMART2 — Smart Tabs & Collision-Aware Leads

V1 only demonstrated supported-scope defects/safety qualification through existing
TAB-SAFE/current audit gates; **advanced placement optimization V2**, not a new V1
feature requirement. Reuse qualified tab/corner/lead/offset/collision work, including
versioned holding semantics; no replacement/default change from convenience alone.

Tab proposals consider corner distance/turns/small radii, useful straight spans,
count/width, cutter footprint, access and declared holding constraints. Preserve manual
placement/intent. No valid arrangement must be reported, never silently omit tabs.
Ease of removal/visual location does not establish physical holding strength.

Leads check current and neighboring contours, islands/protected material/nearby parts,
declared fixtures and process-appropriate swept envelope. Alternative entry/qualified
shape or shortening only within explicit contract; otherwise explain refusal. No
arbitrary adjustment merely to make generation succeed. Milling Z tabs and future
waterjet bridges remain separate; use WJ2/WJ3 where applicable, not milling emulation.

Independent curves/concavity/nesting/narrow-gap/close-part cases, deterministic candidates,
count/holding/clearance preservation, exact output-decoder/preview correspondence and
unsupported/refusal/history/manual override tests. Physical holding evidence separate.

## FIT1 — Assembly Fit & Material Calibration — V2 HIGH PRIORITY

### FIT1-A — Fit Calibration Assistant, preferred first deliverable

Measured thickness, fit convention, tool/context and explicit clearance/interference
range/variant count → proposed small test pattern → review → ordinary qualified CAM
→ separately authorized physical comparison. Nominal15 mm versus measured14.7 mm
is context, not an automatic instruction to resize every slot. Example signed −0.2…+0.2
variants are illustrative, **not universal fit allowances or machine settings**.
Define which dimension/sign changes which mating side, radial/diametral versus axial
meaning and measurement provenance. Optional chosen local calibration must record
material/tool/machine/context/date/evidence; untested is never Validated material data.
Do not mutate FS source evidence or saved rates; fit calibration is distinct from FS.

### FIT1-B — Targeted Assembly Compensation, later

Explicit selected slots/tabs/mating features, original and proposed dimensions and
preserved source → preview → approval → Apply. Candidate recognition does not imply
every rectangular recess is a joint. No unrelated dimension changes. Preserve GEOM1
lineage/original pivot/history/bindings/freshness. Independently test concavity/arcs/
corners/dogbones/cutter compensation and split/merge conflict/refusal. Fit allowance
is not generic machining finish allowance: document order/ownership, never apply twice.

FIT1 acceptance: explicit dimensional/sign contracts, independently correct generated
test geometry/proposals, provenance without physical claims, compatible operation/manual
values and source lifecycle, clear contextual UX and human software review. Actual fit
requires recorded authorized material/tool trials; no current calibration/migration.

## REST1 — True Area Rest-Machining — V2 HIGH PRIORITY

Refines the existing True2D Rest Machining milestone in [advanced CAM plan](ADVANCED_CAM_PLAN.md),
not another rest engine. Larger-tool roughing → qualified residual/depth → smaller-tool
candidate regions/parameters → review → explicit rest operation. Existing CAM4/Clipper2,
resolved output, dual-tool and entry foundations are partial reuse, not safe targets.

- REST1-A: residual diagnostics → explicit candidate regions, with actual stock at
  relevant depth, provenance, unsupported contexts and independent coverage oracle.
- REST1-B: smaller-tool process-aware paths with effective geometry, accessibility,
  engagement/intermediate passes, islands/protected material, safe entry/exit and prior
  operation sequence. Never directly pocket arbitrary diagnostic polygons as proof.
- REST1-C: guided inspect/select tool/explain rates/preview/Apply workflow with lineage,
  history/freshness and decoded-output parity. No arbitrary geometry/automatic optimization.

Numerical and physical domains remain bounded and separately qualified. A/B/C each
require explicit audit/acceptance; no production path or schema now.

## MULTI1 — Unified Multi-Process Workflow, linked product objective

**V1.5 ARCH-MACH/ProtoMAX foundation, progressive V2+ UX**, not a second general
multi-process architecture milestone. Reuse one cleaned source and familiar workflow
across qualified milling/waterjet/future processes. Switching machine never silently
turns milling operations into waterjet or changes cutting semantics. Transfer requires
explicit qualified mapping; otherwise preserve intent and propose reviewed process-specific
replacement. Geometry reuse is not automatic valid-program creation. No FabSBP dependency.

## Shared UX, services and gates

Mandatory Professional Workshop UI + CAM UX: compact contextual review/explanation,
clear measured/inferred/unknown values and limitations, no dashboard or irreversible
automatic changes. Existing authoritative application services/commands own mutation;
future optional MCP consumes them, not a second AI logic path. No proprietary-competitor
superiority claim without evidence. V1.5 remains native OMX/MAKE without LAYOUT;
SMART/FIT/REST features are not prerequisites for that release.

Each feature begins with source/contract audit, independent input domains/oracles and
bounded performance, then regression/lifecycle/refusal/output and actual UX evidence.
Define schema/preset/calibration/operation mapping only after approval; no new deps,
automatic operations, geometry/output/serialization/UI changes or physical trials here.

**HARD STOP — awaiting human approval of the updated Smart Workshop roadmap.**
