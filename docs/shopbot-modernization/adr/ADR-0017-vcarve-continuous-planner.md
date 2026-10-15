# ADR-0017: V-Carve-only continuous pass planner

Status: Architecture approved by user; implemented locally, software review pending
Date: 2026-10-14

## Context

The evidence audit reproduces ignored Perimeter pass depth, interpolated ramp
overcut and silent short-chain undercut. Halfpipe shares the legacy emitter and
must remain unchanged. Existing saved modes and geometry/post boundaries remain.

## Decision

Add V-Carve-specific continuous piecewise-linear planning and pre-emission cutter
envelope validation. Preserve the existing region/medial-axis architecture and
generic XYZ emission. Complete every retained target chain or fail the operation.
Qualify Perimeter as a contour groove and Full medial as sampled spines, not an
arbitrary complete-area pocket. Authorize only V-Carve/type-8 linear XYZ in ShopBot,
after independent decoded-output/preview parity. Preserve all other post guards.

## Alternatives considered

- Fix the shared legacy emitter: rejected because it changes Halfpipe semantics.
- Replace region/axis geometry with a general complete-area clearing algorithm:
  outside the approved targeted scope; coverage remains explicitly partial.
- Post-specific corrective paths: rejected; geometry belongs before postprocessing.
- Silently return incomplete paths: rejected; overcut/undercut are explicit errors.

## Consequences

No project schema/version or saved-mode migration. Manual cutting rates persist.
`carveMaxWidthMm` remains the legacy radius field, clearly labeled Radius cap.
Previously ignored nonzero starts/alternate schedules/refine options now refuse
output. Strict clearance can reject existing problematic Perimeter island offsets.
Path size can grow with progressive entries; bounded work fails rather than prunes.
Halfpipe source remains byte-identical. No new physical or paired-inlay capability.

## Validation

[Qualification](../VCARVE_QUALIFICATION.md) records A1–A4, continuous proof and error
budget, independent cone/coverage/output oracles, guard tests, regression suites,
persistence, actual screenshots and remaining limitations. Human gate ivac-1b75.
