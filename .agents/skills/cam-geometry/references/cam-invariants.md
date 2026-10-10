# Standard CAM invariants

Use only the invariants relevant to the operation being changed.

## Cutter compensation

For cutter diameter `D`, nominal radial compensation is `D / 2` before allowances/wear compensation.

For a 100 × 100 mm square and a Ø6 mm cutter:

- outside profile nominal cutter-center bounding box: 106 × 106 mm,
- inside profile nominal cutter-center bounding box: 94 × 94 mm.

Account explicitly for finish allowance or wear compensation.

## Depth passes

For configured maximum stepdown `S`, no ordinary cutting increment should exceed `S`, final depth should be reached within numeric tolerance, and no pass should unintentionally exceed final depth.

## Pockets

The swept cutter area must remain within the intended machinable region except deliberate entries; islands must remain uncut; unreachable regions should remain visibly unmachined or produce diagnostics rather than be violated.

## Operation topology

Internal features must not be mistaken for external boundaries. Open geometry must not be silently closed when closure changes intent. Offset collapse must be handled explicitly.

## Tabs

Tab regions must preserve material to the configured residual thickness, and tab placement should regenerate deterministically on the intended contour.

## Leads and entries

Leads may leave the nominal contour only where the operation explicitly permits it. Entry paths must respect process constraints and safety checks.
