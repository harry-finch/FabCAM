# ADR-0013: Persistent CAD corner relief before CAM evaluation

Status: Accepted architecture by user on 2026-10-14; software qualified; human CAD1 review pending
Date: 2026-10-14

## Context and decision

CAD1 is the geometry preparation phase C of CAM4. The user approved the
[architecture checkpoint](../CAD1_AUDIT.md) and preceding bounded A/B review.
Maintain the geometry → CAM → generic execution → post boundary.

Use a pure Rust corner evaluator with explicit millimetre geometry, object/corner
references, removal side, mode and actual effective cutting diameter. It returns
exact lines/arcs and refusals. It does not know tool inventory, operations, machine
brands, feeds/speeds or post syntax. Normal CAM compensation and output remain
unchanged. HTTP, WASM/worker and Tauri expose the same evaluator.

A project has one editable relief batch in V1. Original imported geometry and its
layout transforms remain authoritative and unchanged. The persisted modifier stores
its recipe, cutter ID/effective diameter, evaluator version, exact source identity
and the accepted evaluated ImportResponse. This is a bounded modifier, not a CAD
feature tree. The batch may select multiple objects/corners across the combined
imports. It uses one cutter, mode and removal side; editing replaces that batch.
Different tools/modes within one project batch are outside V1.

Evaluate after layout transforms and before WCS. Every ordinary geometry consumer
uses the same accepted result through transformedImport. Source/transform/cutter
changes require explicit preview and reapplication; they never silently rescale
reliefs. Preview temporarily draws the original source plus proposed geometry,
without mutating the project or the spatial index used by ordinary selection.

One history command applies, edits or removes the batch. Authored tab and approach
bindings refuse perimeter changes rather than drifting to new positions. Persist
with guarded `ivac-project-cad` version 5, so older readers reject the file instead
of machining the source while ignoring the modifier. Legacy versions remain readable;
removing the modifier returns to the format required by the remaining project data.

## Numerical and scope decisions

For a vertex V with incident unit rays u and v, a dogbone cutter center is
`V + r normalize(u+v)`; a T-bone center is `V+r u` or `V+r v`. Intersections on
incident edges use `t=max(0,2 dot(center−V,ray))`; splice the removal-side circular
arc, split at at most π. All constructed radii equal the effective cutter radius;
there is no hidden extra clearance. Effective frontend diameter follows the existing
`diameter − wearOffsetMm` convention. No measured runout/wear is inferred.

Right/obtuse removal-side line-line corners are supported; acute corners are refused:
the exact-radius lobe can have an isolated feasible cutter center that normal
continuous compensation cannot visit. A plunge operation/access-slot policy would
be separate work. Selected arc-line/arc-arc corners, malformed/open/intersecting
contours, short-edge/batch/topology conflicts and budgets beyond 512 source segments
are explicit refusals. Unselected arcs and nested contours retain geometry/identity.

EPS 1e−7 mm is for source connectivity/degeneracy, not physical fit. Directional
thresholds are dimensionless; Cavalier topology predicates retain their own library
tolerances. Emitted vertex coverage is checked within 0.00015 mm, reflecting 4-decimal
XYZ rounding rather than promising physical fit. CAD1 does not widen A/B's curved
residual diagnostic scope; that display remains Not analyzed.

## Consequences and validation

No ShopBot-specific correction enters geometry. Source segments, transformed evaluated
geometry, normal tool compensation and decoded emitted motion are separately tested.
Existing normalized tab positions are not reinterpreted silently. Project restore
must preserve every saved import and transform after its import initializer; otherwise
an accepted modifier would immediately lose its source identity.

One-cutting-tool metric single-sided Profile/Cascade Pocket is the initial integration
boundary. Patterned operations, mismatched profile removal side, dual-tool/allowance/
synthetic-frame pockets and authored tab/approach bindings are refused in the frontend
adapter. Native backend users receive explicit geometry and can use normal CAM APIs;
the CAD endpoint itself never promises downstream operation qualification.

See the [CAD1 qualification record](../CAD1_QUALIFICATION.md) for actual results and remaining human/native gates.
