# Deferred ProtoMAX architecture assessment

M0 conclusion: **B — moderate architectural extension**. This is an engineering
assessment of the inspected code, not a tested integration or an OMX format
claim. No ProtoMAX/waterjet implementation is authorized in M0–M4.

## Current process boundary

`project/machine.rs` represents `Mill`, `Laser`, `Drag`, `Plasma` in MachineMode.
MachineConfig has a primary mode plus capabilities used by the frontend op
picker. `project/op.rs` provides common Profile/Engrave contour operations and a
DragKnife variant; laser/plasma emission is selected through machine mode rather
than a dedicated complete process IR. Shared contour params include offsets and
leads, but OpParamsCommon retains milling depth, plunge and feed assumptions.

`gcode.rs` dispatches cut-tool on/pierce/off. Laser arms at zero power before
traverse, powers on at the entry and off before the next rapid. Plasma positions
at pierce height, fires, dwells, moves to cut height and cuts in one pass
(`gcode/emit.rs`). Drag and plot-mode paths collapse multi-pass Z. Tool fields
include pierce timing and plasma heights. These are useful existing boundaries.

However, there is no first-class `Pierce` record. In `gcode/cps_recorder.rs`,
laser/plasma on/off use spindle-speed state, and the CPS Program Section has
tool/spindle/work-plane fields. `ir.rs` distinguishes rapid and linear/circular
motion, but does not express jet state, waterjet quality or OMX intent. Do not
turn these conventions into a waterjet implementation with dummy spindle/Z.

## Reuse and gaps

| Required future intent | Existing reuse / missing semantics |
| --- | --- |
| Traverse | Rapid motion, ordering and geometry can be reused; jet-off state must be explicit |
| Pierce | Plasma entry provides placement/delay precedent; waterjet pierce method/event absent |
| Cut | Line/arc contour paths reusable; process on/off and quality transitions missing |
| Kerf compensation | Offset/topology algorithms reusable; user-visible meaning currently tool-radius oriented |
| Quality | No waterjet quality model found; do not substitute feed or spindle power |
| LeadIn / LeadOut | Existing contour geometry helpers reusable; process-specific constraints need qualification |
| Slit | On-line/no-offset geometry candidate; explicit waterjet intent is absent |
| Etch | Engrave/laser concepts are architectural references, not verified waterjet etch support |
| OMX output | No post found; requires authoritative format research and licensing review later |

Source geometry, transforms, entity selection, persistence, undo/redo, machine
envelopes, fixtures, operation ordering and rendering are good shared candidates.
Milling material-removal simulation assumes cutter geometry/Z sweep and cannot
be declared a qualified waterjet simulation. Process UI fields exist in
MachineDialog, tool form profiles and OpPropertiesPanel, but waterjet settings
and capability filters would need explicit additions.

## Why B

Existing non-milling branches and post seams make a total replacement unnecessary.
Explicit waterjet operations/parameters/events, schema/client compatibility,
process-aware preview and a distinct exporter still require coordinated changes.
Hence neither a trivial new post (A) nor demonstrated wholesale architectural
failure (C) is supported by M0 evidence.

Later WJ architecture qualification should determine the smallest additive boundary and record an ADR after
reading serialization, frontend, caching, simulation and post consumers. OMAX
MAKE handoff/directives must be verified later; direct machine control remains
out of scope. This assessment makes no physical-machine or file-format claims.

Post-M4 scheduling uses the WJ track rather than global M5–M10 numbering.
See ROADMAP for the deferred sequence and priorities. The milling-specific FS
engine must not become a waterjet or drag-knife dependency; share appropriate
infrastructure only. No WJ/VINYL work is authorized by the FS planning update.

## Future geometry-preparation prerequisite

[GEOM1 — Drawing Cleanup & Repair](GEOM1_PLAN.md) is HIGH-priority planned generic
infrastructure, preferably qualified before serious ProtoMAX integration, after
active V-Carve review and existing approvals. Continuous contours, explicit open/closed
semantics, duplicates/zero-length/micro-edge diagnosis, stable topology and named
unit-aware tolerances improve waterjet inputs. Geometry repair requires preview and
explicit Apply; an intentional open/slit path or drill/reference point is not inherently
invalid. Cleanup must not add waterjet-specific algorithms or decide pierce/kerf/quality.

[Shared geometry plan](GEOMETRY_PREPARATION_PLAN.md) preserves source/effective-geometry,
identity/history and CAM-region boundaries; GEOM2/3 are later editing utilities, not
mandatory full-CAD dependencies. GEOM1 is a recommended qualified foundation, not
an authorization to start WJ or fabricate OMX/physical evidence. Milling FS stays
independent and the active V-Carve HARD STOP remains unchanged.

## Future sheet-layout compatibility

[NEST1](NEST1_PLAN.md)/[NEST2](NEST2_PLAN.md) place parts generically using qualified
GEOM2-A transforms and prepared sources. Kerf, pierce and lead clearances, quality,
cutting order and no-go/fixture constraints remain explicit waterjet-process inputs,
not an OMX exporter or milling assumptions hidden in packing. Geometric overlap/
spacing success alone never claims machining safety. GEOM2-B drawing is not required
in full for manual sheet layout; [shared architecture](NESTING_PLAN.md) keeps frames,
instances and CAM binding explicit. No WJ implementation started by this plan.

Common-line cutting is separate future work, excluded from initial NEST1/2: shared
edges need kerf/topology/order/leads/part-separation/hold-down/output qualification.
Multi-sheet/remnants can be a later NEST2 phase without full inventory management.
