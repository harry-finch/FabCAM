# ADR-0010: Depth-scoped remaining-material diagnostics

Status: Implemented for bounded CAM4 A/B; human review pending
Date: 2026-10-13

## Context

CAM3 continuation is approved. Remaining-material planning requires generated-path
analysis rather than a drawing-corner heuristic, with explicit scope and no false
job-wide safety claim. Existing Pocket regions and returned toolpaths are reusable;
heightmap/dexel simulation does not supply a qualified planar residual polygon.

## Decision

Extend the buffered pipeline response with optional per-operation
`remaining_material` diagnostics. Preserve project serialization and the toolpath.
Reuse intended Pocket RegionPreview polygons, existing Clipper2 offsets/booleans
and hole-preserving polytree conversion. At the operation's final floor, subtract
the union of flat-cutter capsules/disks from its intended removal region. Clip
linear XYZ movements to the portion reaching that depth; ignore Rapids and
shallower passes. Target holes are excluded, not diagnosed as failed machining.

V1 A/B qualifies metric Cascade Pocket, one Endmill, closed straight-sided targets,
linear final returned moves and a Z0 stock with the analysis plane inside its
thickness. Unsupported/intentional contexts return explicit reasons, not a zero
residual verdict. Individual operations are analyzed independently; no job/tool
accumulation. Numerical precision/area threshold and bounded work are recorded in
[qualification](../CAM4_QUALIFICATION.md).

The adapter distinguishes actual output interpretation from arbitrary CPS
planning IR. Native interpreted linear output and bundled ShopBot resolved output
can qualify; arbitrary CPS IR cannot. No ShopBot semantics enter sweep geometry.
Existing [ADR-0008](ADR-0008-work-origin-frames.md) work/drawing transform also maps
residual polygons while preserving program text, areas and Z.

The compact 2D overlay uses existing region drawing caches and warning tokens.
Generation input identity suppresses stale diagnostics after any relevant edit,
including after Save; no dirty-flag shortcut. View selection/visibility is transient,
not persisted manufacturing state. No cleanup operation, automatic corner relief
or guessed residual cause is introduced.

## Alternatives considered

Input-corner detection cannot prove actual residuals. Reusing the visual heightmap
as exact planar geometry would conflate grid resolution and physical accuracy.
A global job residual without depth/tool accumulation semantics is ambiguous.
Silently treating tabs/allowance or unknown CPS output as removed material would
hide intent or invent a reassuring verdict. A new geometry engine/IR is unnecessary.

## Consequences

Additive response/API schema and regenerated TS types; missing fields from older
backends mean unavailable analysis. Saved project version, Tool/CuttingParameters,
FS math and export/post code remain unchanged. Streaming/legacy CLI output is not
extended to claim this diagnostic; full buffered HTTP/WASM/Tauri responses carry it.
No machine-control/FabSBP work, physical validation or new app bundle.
Curved targets/analytic arcs, tabs/allowances, multiple tools and broader strategies
remain explicit limitations. CAD1/Phase C requires separate geometry scope/approval;
remaining-material analysis does not silently alter geometry.

## Validation

Analytical corner area, depth clipping, plunge disk, island holes, small-area
threshold and concave target clipping; end-to-end emitted ShopBot path/origin and
intentional-allowance refusal; frontend frame/freshness tests plus browser overlay,
stale-after-Save and unsupported-state review. Keep CAM1–CAM3 regressions and schema
compatibility checks. Physical machining evidence remains separate.
