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
