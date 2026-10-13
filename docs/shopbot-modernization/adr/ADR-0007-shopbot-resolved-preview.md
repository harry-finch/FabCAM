# ADR-0007: Bundled ShopBot resolved preview from emitted XYZ

Status: Implemented in UX1; human review pending (ivac-cb64)
Date: 2026-10-13

## Context

UX1 qualifies milling entry strategies against ShopBot post capabilities and
establishes preview-to-machine-output consistency. It is CAM capability/post
qualification as well as UX. Generic CPS IR preview retained analytic arcs and
combined rapid axes while the bundled ShopBot post emitted 0.01 mm linearized XYZ
and ordered Z/XY jogs. Simulation could therefore use a representation different
from the final machine program. Existing CPS framework already performs capability
resolution (`preflight` + kernel `linearize`); a new universal capability model is
unnecessary. Generic Helix emission also has geometric fallbacks and a subsequent
contour re-plunge; neither can be hidden by machine adaptation.

## Decision

KEEP generic Direct/Ramp/Helix intent and geometry algorithms machine independent.
Formalize the bundled ShopBot CPS preflight/linearization as the resolution boundary.
EXTEND final preview with a small adapter decoding only the owned post's supported
metric absolute `JZ`, `J2`, `M3`, `MS` movement subset (and reviewed dwell/scaffold).
Use emitted rounded coordinates/feeds and source lines for preview, simulation,
envelope warnings, playback and timing. Unknown command/invalid numeric output
fails the pipeline rather than reverting to a misleading IR preview. It is not a
public SBP interpreter or a parser for arbitrary user programs.

Requested strategy remains in the operation/project. Derived per-op
`entry_representation` diagnostic params carry requested/resolved/target, tolerance
and preview source through the existing warning channel. Expected adaptation is
informational, compact by default; details are collapsed. Generic geometry fallback
also emits an explicit intent/resolution comment; ShopBot refuses it. Genuine
Helix remains explicitly unsupported until full region containment and coherent
entry-to-contour transitions are qualified; no new Helix-to-Ramp machine fallback.

Initial operator XYZ is unknown: omit the initial establishment of clearance and
first XY placement until a position is known. Do not invent an initial trajectory.
All subsequent movement endpoints and lines, including all cuts, match emitted SBP.
Manual intervention at tool-change/zero/spindle pauses is not simulated. Geometry
before Generate remains generic; only the final generated toolpath is machine resolved.

## Alternatives considered

- Continue IR-only preview: insufficient for straight chord simulation and split jogs.
- Instrument every CPS primitive into a new common execution trace: larger runtime/
  schema/transport change; defer until multiple qualified posts need this guarantee.
- Generic CAM ShopBot branches: rejected; machine semantics belong at post/adapter.
- Enable all Helix solely because CPS can linearize it: rejected; geometric intent,
  islands and contour re-plunge need independent qualification first.

## Consequences

No new persisted state, operation/tool type, request/response field or schema
migration. Existing warning params suffice. Other bundled/custom CPS posts retain
existing IR previews; copied/inline scripts named ShopBot are not automatically
qualified by filename. SBP command inventory and Direct output goldens unchanged.
The adapter must be updated deliberately if the owned post's command subset expands.
No general stock/WCS/fixture work, FS data/math/UX expansion or hardware claims.

## Validation

Independent test decoder compares Direct/Ramp Profile and Pocket final preview
segment-for-segment with SBP: exact rounded endpoints, motion class, op/source
line, split-rapid clearance and deterministic output. Circle inside/outside ×
climb/conventional verifies analytic geometry at 0.0102 mm, separately from physical
tolerance; linearized resolved segments have no analytic ArcXY descriptor.
Generic Profile/Pocket Helix tests prove local descent/depth/direction and preserved
requested intent, distinguish impossible radius fallback, and assert ShopBot refusal.
Owned parser tests cover output precision/feed/dwell/line mapping and fail-closed
unknown command/non-finite/unestablished cutting motion. M1/M2/tab/lead/history/
persistence and generic post regressions remain required. See UX1 qualification.
