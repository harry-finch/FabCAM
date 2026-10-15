# NEST1 — Sheet Layout & Manual Nesting

2026-10-14 · **PLANNED — NOT IMPLEMENTED · MEDIUM PRIORITY**.
Depends on reliable GEOM1 source preconditions and qualified GEOM2-A transforms,
not every GEOM2-B primitive. [Shared architecture/dependencies](NESTING_PLAN.md).

## Product scope and workflow

Define sheet → select coherent part contours/details → quantity → source-referenced
instances → move/rotate/numerically place → check margins/spacing/overlap → review
→ normal CAM. Manual layout only; automatic optimization is NEST2.

Part is not one contour or automatically one layer: preserve outer boundary, internal
holes, pockets, engraving and disconnected features together. Source part owns the
definition; each instance owns identity/transform. Moving or duplicating an instance
does not change source geometry. Audit current project consumers before freezing schema.

Reuse stock size/units/orientation, explicit usable boundary/margins/keep-outs and
fixtures, resolving sheet/current-stock linkage rather than duplicating stock. No
implicit work-zero/stock repositioning. Candidate controls: select/drag, numeric place,
rotate, duplicate, delete, quantity, qualified snapping and useful alignment. Share
GEOM2-A mathematics with separate source-edit versus instance-placement history.

Optional user-authored allowed angles/fixed orientation, grain and mirror permission
must be honored where supported; unsupported restrictions must not be silently ignored.
Do not assume unrestricted rotation. Display exact boundary overlap, near-spacing and
edge/keep-out/fixture violations separately from unqualified toolpath safety.

CAM binding for source pockets/profile across instances needs its own design approval.
All paths use accepted transformed geometry through normal CAM, compensation/WCS/order
and machine checks; no finished-code translation. Placement edits invalidate generated
results. Preserve operation references, parameters and final output/preview parity.

## UX, exclusions and future acceptance

Compact contextual sheet-layout mode, visible sheet/margins, selectable instances,
placement inspector with quantity/XY/angle and actionable invalid-placement feedback.
Follow shared UI/CAM UX; no second CAD editor. Reuse existing UX5 actions later.
Exclude optimizer, common-line cutting, full inventory/multi-sheet optimization,
project migration now, machine output changes and physical validation.

| Qualification matrix | Planned proof |
| --- | --- |
| Parts/instances | Creation/quantity/duplication/delete, grouped outer/holes/details, source unchanged by placement, stable references |
| Geometry | Translation/rotation, allowed rotations/mirroring/grain, exact/near overlap and spacing, sheet margins, fixtures/keep-outs and tolerance edges |
| Lifecycle | Preview/cancel/atomic history, save/reopen/old-project compatibility, source change and placement invalidation |
| CAM | Correct operation-to-instance quantity/mapping, unchanged rates/tools, regeneration, compensation/work origin and final output/preview parity |
| UX/limits | Desktop keyboard/numeric/selection feedback and human review; geometric validity never advertised as machining-safe without process/path proof |

Acceptance requires dedicated architecture audit, independently checked placement and
constraint geometry, explicit supported restriction set, normal CAM/regression evidence,
representative screenshots and human gate before NEST2. Geometry validity is not physical
hold-down/clearance certification. Open: part membership, sheet relation, instance binding,
source revision policy, spacing geometry/process interface and first restriction subset.

**STOP — waiting for human approval before implementation.**
