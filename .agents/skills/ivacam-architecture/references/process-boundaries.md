# Manufacturing process boundaries

The project should share infrastructure without pretending all manufacturing processes are identical.

## Good candidates for shared infrastructure

Imported geometry, layers/entities, transforms, contour topology, project persistence, selection, undo/redo, visualization primitives, machine envelopes, operation ordering infrastructure, and post-processing framework.

## Milling concepts

Tool diameter, spindle, Z cutting depth, stepdown, plunge, ramp/helix, pocket clearing, and tool changes.

## Waterjet concepts

Traverse, pierce, kerf, cut quality, lead-in/lead-out, slit, and etch.

Do not encode waterjet as milling with dummy spindle/Z/feed values.

Likewise, do not create a new universal IR merely because future waterjet exists. First inspect whether ivaCAM's existing laser/plasma/process abstractions already provide the required boundary.
