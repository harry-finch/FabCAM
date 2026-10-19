# STEP1 — STEP Import & 2D Geometry Extraction

2026-10-19 · **PLANNED — NOT IMPLEMENTED · V2+ HIGH PRIORITY**.
[Unified V2 priority view](V2_WORKSHOP_PRIORITIES.md). No STEP parser/kernel/viewer,
schema or CAM support introduced here; preserve V1 and native OMX V1.5 priorities.

## Product boundary and stages

Import STEP → inspect supported 3D model → select planar face → preview/confirm 2D
contours → normal 2D selection → explicitly create existing CAM operations. Avoid
returning to external CAD solely for a DXF export. Face/contour selection is not machining
intent: circle does not create Drill; rectangle does not create Pocket.

| Stage | Future scope and independent delivery |
| --- | --- |
| STEP1-A — STEP Import & Planar Face Contour Extraction | First high-priority deliverable: well-formed mechanical solids, planar face selection/highlight, outer/inner wires, supported straight/circular edges, explicit 2D frame and dimensions, accepted contours in existing canvas; explicit unsupported/ambiguous cases |
| STEP1-B — Advanced Projected Geometry Extraction | Separate follow-up: selected-direction orthographic projection, edges across faces, visibility/occlusion, depth-level filtering/labels, overlapping-edge handling and contour choice; optional 3D reference display |
| STEP2 — Automatic Machining Feature Recognition | Separate OPTIONAL future milestone: holes/blind holes/pockets/counterbores/slots/qualified prismatic features → proposed operation → human review/Apply. Neither dependency nor initial STEP1 behavior |

STEP1-B may expose original Z as context, never infer operation depths silently.
Neither STEP1-A nor B promises automatic 3D CAM or a full CAD modeling interface.

## Existing foundation and feasibility gate

Current import/Segment/drawing/GEOM1, source identity, transforms, 2D canvas and 3D
scene/relief-STL display are reuse candidates, **not STEP B-Rep import/face topology**.
Audit actual import/renderer boundaries before selecting libraries or coupling to CAM.

Future feasibility audit compares Open CASCADE, separate FreeCAD/Open CASCADE prototype
and sufficiently capable Rust STEP/B-Rep libraries. No dependency selected or license
claim made now. Measure accurate read/access to solids/faces/wires/edges, planar
detection, circular/curved edges, topology/precision, multiple solids/assembly transforms,
format subsets and malformed/resource-limited input handling. Confirm legal redistribution,
license obligations, native size/maintenance and macOS/Windows/Linux packaging.
Native CAD kernels do not automatically work unchanged in WASM: audit isolated adapter,
native/optional backend strategy and browser capability/refusal, not a hidden cloud need.

Gate before production dependency: authorized small feasibility PoC with analytical STEP
fixtures, accurate extracted loops/frame and three-platform packaging evidence/risks;
human architecture decision. Prefer isolated CAD source adapter, not STEP conditions
throughout pure Rust CAM. No PoC/kernel installation or artifact created by this update.

## Geometry, coordinates and precision

Explicit selected face, normal/machining orientation and local orthonormal XY frame,
STEP placement/units→drawing transform, winding/outer-inner classification, closure,
line/arc/circle representation and independent error budget. Selected face need not
align with global STEP XY. Do not silently flatten nonplanar/unsupported curves.
Distinguish source face, stock and work zero; selection establishes no physical datum.
Separate import/kernel accuracy from later CAM approximation and physical tolerance.

## Source identity and project lifecycle

Respect [ADR-0021](../shopbot-modernization/adr/ADR-0021-edited-drawing-contract.md)
and GEOM1: retained immutable STEP revision, accepted extraction/parameters, reliable
entity provenance, stable imported/contour identity and operation bindings. Reuse
one authoritative accepted drawing for all consumers. Reimported B-Rep face/edge
indices are not stable identity; changed source with ambiguous mapping requires
explicit review/reselection, never nearest/coincident-geometry rebinding.
Qualify Undo/Redo/save/reopen and stale-output invalidation. Schema/storage strategy
awaits audit; no automatic version migration or preserved-ID promise unsupported by evidence.

## UX and cross-milestone dependencies

Professional Workshop UI + CAM UX: compact 3D inspect/planar-face highlight → extracted
loop preview → confirm → ordinary 2D workflow. Clearly distinguish source solid,
selected face, accepted contours and CAM selection. No full CAD workspace/dashboard.
GEOM1 supplies cleanup/lineage/topology; GEOM2 related frame/interaction math must be
audited/reused without making every editing tool prerequisite. ARCH-MACH keeps geometry
source independent of machine/process; extracted 2D may serve any qualified compatible
process. STEP2 recognition is never required to use contours. Optional later MCP uses
authoritative services, not independent extraction logic.

## Qualification and acceptance

Independent known dimensions/units; rectangles/inner loops/circular holes/line-arc;
rotated/translated/non-global-XY faces; multiple solids/assembly policy; closure/winding;
invalid/degenerate topology and unsupported curves/surfaces; exact frame/round-trip
references; deterministic extraction; accepted-preview/CAM geometry consistency.
Lifecycle: source revision/provenance, reimport ambiguity, history/save/reopen/binding
preservation and no stale export. Regress existing SVG/DXF/GEOM1/ShopBot output.
Actual desktop face-picking/2D transition screenshots/accessibility and packaging review.
Screenshots or successful parsing alone prove neither dimension accuracy nor manufacturing
compatibility. A/B each end at human software review with explicit supported subset;
physical claims require separate evidence. STEP2 needs its own detection/approval scope.

Open: kernel/license/native packaging, supported STEP/solid/assembly subset, WASM strategy,
face frame/orientation convention, curve budget, persistent extraction/provenance and
source changes. No detailed production API/schema frozen before feasibility evidence.

**HARD STOP — awaiting human approval of the updated STEP roadmap.**
