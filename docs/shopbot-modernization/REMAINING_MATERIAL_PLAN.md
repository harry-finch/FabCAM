# Remaining Material Analysis / Rest Material Visualization — future plan

Status: **CAM4 A/B BOUNDED SOFTWARE HANDOFF; C / BROADER COVERAGE PLANNED**.
Updated2026-10-13 after approved CAM2/UX2, UX2.1 and CAM3 continuation.
[Qualification](CAM4_QUALIFICATION.md) and [ADR-0010](adr/ADR-0010-depth-scoped-remaining-material.md)
define current scope/exclusions. Human gate ivac-tvox is pending. CAD1/C corner
relief retains separate scope/approval; FS4 remains DEFERRED/nonblocking.

## Purpose and V1 boundary

After generation, show material the actual cutter/toolpath cannot remove, such as
residual stock in an internal corner sharper than the cutter radius. Analyze the
generated machining result, not simply flag sharp corners in the input drawing.

```text
Target removal area − actual cutter swept area = remaining material
```

Initial V1 is diagnostic/visual only for explicitly scoped relevant 2.5D operations:
remaining-region geometry, a toggleable preview overlay and concise explanations
where causes are reliable. Optional metrics: region count, total area and largest
region area (mm²). Typical explanation: “Cutter/toolpath cannot geometrically reach
this area.” Unknown causes must remain unknown, not be guessed from the drawing.
No automatic cleanup operation, silent geometry/toolpath change, generalized
stock-removal simulation or physical accuracy claim is promised in V1.

User-facing terms: **Remaining material**, **Unmachined material**, **Unreachable
area**. Use **Rest machining** only for the later operation to remove residuals
with another cutter; the analysis and the machining operation are distinct.

## Placement and dependencies

```text
CAM2 / UX2 — Stock, Origins, WCS & Fixtures
    → CAM3 — Tabs / Lead-in / Lead-out qualification
    → CAM4 — Remaining Material Analysis + Corner Relief workflow
    → later milling: UX3 surfacing, UX4 templates, V-carve qualification
```

CAM2/UX2 establishes reliable stock, coordinate/reference model, operation depth
context and collision/keep-out context. CAM3 qualifies intentional tabs and entry/
lead interactions before residual classification. These foundations are qualified;
user continuation authorized the bounded A/B checkpoint. FS4 physical Feeds & Speeds validation is not a prerequisite to software
analysis. Every future milestone keeps its own human approval gate.

CAM4 should phase internally:

| Phase | Future scope | Dependency |
| --- | --- | --- |
| A — Analysis (bounded software handoff) | Qualify target region and actual cutter-swept region; derive remaining geometry and provisional metrics | CAM2/UX2 + CAM3; geometry/architecture audit |
| B — Preview/diagnostics (software handoff) | Overlay, visibility control, concise causes and metrics, clear analysis context/limits | Qualified Phase A |
| C — Corner relief (planned, separate scope/approval) | Dogbone/T-bone generation or assistance and reviewable geometry workflow | Coordinate with A/B; separate scoped geometry qualification |

Existing proposed **CAD1 — Dogbones / T-bones** is retained as the Phase C geometry
workstream, not a duplicate implementation or renumbering of completed work.
Dogbone generation is not a prerequisite for A or B: visualization is useful alone.
A user can see why a mating part will not fit and later choose explicit corner
relief; residual detection must not silently add a dogbone or modify intended shape.

## Geometry and architecture direction / remaining scope

Conceptual data flow:

```text
Generated/resolved toolpath + cutter geometry + operation depth/context
    → cutter swept region using tool radius
    → compare with intended removal region within defined stock/reference
    → remaining material geometry
```

Design exact representation, boolean/sweep methods, depth slices/context,
operation/job accumulation and numerical tolerances during the future milestone
with `$cam-geometry`, `$ivacam-architecture` and `$cam-validation`. Preserve the
geometry → CAM → generic path → capability resolution/post boundary. Do not add
ShopBot conditions to generic removal geometry or decide a new shared model now.

Explicitly decide whether each analysis uses generic CAM path, machine-resolved
path, or both for distinct purposes. For final “what will this machine leave
behind?” analysis prefer the actual execution representation. UX1's bundled
ShopBot final preview uses emitted XYZ: residual overlay must agree with that path,
including linearized curves and output precision. A generic planning analysis, if
also provided, must be labeled separately; do not present it as machine output.
Unavailable/stale generation or unqualified target representation must not produce
a reassuring final verdict. Exact policy is future design work, not settled here.

Existing `pipeline/regions.rs` supplies intended Pocket region previews and
`sim/sweep.rs` provides cutter sweep/simulation infrastructure; these are candidates
for reuse/audit, not evidence of this feature already being complete. Existing
simulation resolution/coarsening and partial removal checks must not be mistaken
for a qualified residual-region overlay or exact planar analysis.

Distinguish **intentional material** (tabs, allowances, staged roughing/finishing,
preserved islands) from **unreachable/unmachined material**. Define the intended
removal region at the analyzed operation/depth rather than subtracting from the
entire drawing or stock. Account for multiple tools where relevant to the chosen
scope; do not interpret intentional finishing allowance as failed machining.

## UX and visual language

Pair the installed `$professional-workshop-ui` (shared desktop visual language)
with `$cam-ux-ui` (CAM workflow/interaction/safety). See
[design direction](ux/DESIGN_DIRECTION.md). They remain separate skills; do not
create a competing local visual framework.

Keep workspace dominant and controls compact. Possible future preview controls:
Toolpaths, Rapids, Remaining material. Use SVG/application icons from the selected
project icon family, not emoji; audit current adoption before choosing/installing
anything (shared skill prefers Lucide; existing consistent Tabler is acceptable).

Restrained amber/orange semantic warning overlay, optionally transparent, must be
distinct from normal stock, paths, selection and blocking errors. Reuse a suitable
shared semantic token; no isolated hardcoded color. Use text/icon as well as color.
Example compact state: “4 unreachable areas · 38 mm²”, with a small warning icon.
Details/causes/limits belong in disclosure or tooltip; no large cards, duplicated
machine/tool context or warning surfaces covering the workspace. Visibility toggle
and keyboard/accessible naming are part of future UX acceptance.

## Provisional future acceptance targets

These remain broader design/test targets, **not blanket current capability claims**.
Bounded A/B evidence and unmet curved/intentional/job-accumulation scope are explicit
in CAM4_QUALIFICATION. Further fixtures/invariants/thresholds precede expansion:

- Sharp internal corner with a too-large cutter produces residual geometry.
- A sufficiently small cutter produces no spurious residual above the documented
  numerical threshold; do not treat mathematically sharp corners as fully machinable
  by any finite-radius cutter.
- Circular/concave boundaries and pocket islands remain topologically correct.
- Intentional tabs, allowance and retained islands are classified separately.
- Overlay aligns with generated/resolved path, stock reference and operation depth.
- Overlay can be disabled; concise cause/metrics are consistent with analysis scope.
- Repeated analysis is deterministic; stale/unsupported context is explicit.
- Geometry/topology, curve flattening and output-precision tolerances are documented
  separately from physical machining tolerance.
- No residual geometry is invented outside defined stock/intended removal region.

Prefer analytical shapes and independent swept-region/output comparisons before
visual review. Screenshots alone do not establish geometry or physical accuracy.

## Later advanced capability — Rest Machining

Separate future feature, after reliable remaining-material analysis:
primary cutter → residual detected → explicit smaller cutter choice → cleanup
operation generation. It is excluded from V1 and Phase C. Record only this concept
now; target rebinding, cleanup strategies, rates and persistence need a later audit,
qualification and approval, not premature implementation/design here.
