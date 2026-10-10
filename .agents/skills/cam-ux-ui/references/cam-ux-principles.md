# CAM UX principles

## Main workspace

A strong default information architecture is geometry/layers, operation tree, central canvas, contextual operation properties, warnings/status, and simulate/export actions. Do not force this layout if the existing UI has a better established pattern.

## Operation creation

A user should be able to answer at a glance:

- Which geometry will be machined?
- With which operation?
- With which tool/process?
- On which side/region?
- To what depth?
- In what order?
- Are there warnings?

## Geometry selection

Make selected, assigned, unassigned, and invalid geometry distinguishable. When practical, show the tool centerline separately from source geometry.

## Parameters

Group parameters by intent: tool/process, geometry/compensation, depth, entry/exit, tabs/holding, feeds/speeds, advanced.

## Defaults

A default should be safe enough for context, explainable, and visibly derived where relevant. For machine/material/tool presets, show where a value came from.

## Origins and stock

Origin setup is high risk. Show stock top vs machine bed Z reference, XY job origin, machine home, work envelope, and stock dimensions graphically.

## Warnings

Prefer actionable messages. Example: `Profile 3 uses a closed-contour operation on an open path. Close the path or choose an open-profile mode.`
