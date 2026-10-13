# ADR-0008: Generic work coordinates and drawing-frame display

Status: Implemented, human review pending (CAM2/UX2)
Date: 2026-10-13

## Context

Numeric work origin XY was persisted but ignored during geometry generation.
The inspector also exposed legacy bbox-relative stock translation as if it were
an absolute corner. Fixtures already used drawing XY and operation-depth Z.
UX1 requires preview to follow resolved emitted moves, with source identity intact.

## Decision

Normalize an owned request in generic pipeline/work_origin.rs before CAM. Subtract
numeric work XY from segments/centers, stock and fixture origins, text/relief origins,
explicit contour approach and polar Drill center; preserve relative dimensions.
Reset numeric XY on that request only, retaining WCS selection. Machine home/park
and tool stations stay in their authored output frame. The saved authored project
is never mutated. Postprocessors continue consuming generic CAM/toolpath data.

Returned toolpath and regions use work coordinates. The desktop work-frame adapter
adds the request's captured XY point for drawing display/simulation; emitted text,
source lines, Z and cutting rates stay untouched. Generation input identity still
invalidates output after edits, including Save, and asynchronous results use the
captured origin. The bundled ShopBot resolved execution path remains authoritative.

No persistence/domain/schema migration. Effective stock-corner UI compensates
existing bbox-relative offsets. Existing bbox-following behavior remains explicit.
Fixture CRUD reuses existing history and schema. Generic validation rejects invalid
geometry; resolved-path candidates and unknown-holder diagnostics use the existing
warning severity/export-setting mechanism. No path avoidance is added.

Single-sided milling XY is supported. Nonzero numeric Z rejects until depth and
clearance conventions are qualified. Non-milling XY and enabled raw G-code includes
reject ambiguous translation. UI offers qualified WCS slots (LinuxCNC G54..G59,
other current dialects G54); persisted unsupported setup stays visible for correction.
Genuine two-sided CPS or nonzero XY rejects before the historical wrapper can mix
G-code comments with CPS syntax or use unshifted registration references.

## Alternatives considered

Post-only subtraction would leave CAM, fixtures and warning regions in inconsistent
frames. Shifting saved geometry would destroy authored coordinates and persistence
compatibility. A new absolute stock schema is unnecessary for clarification and
would require a separate migration. Enabling Z or two-sided output without depth
and registration qualification would preserve an unsafe ambiguity.

## Consequences

Old zero-origin jobs retain output. Previously ignored nonzero XY now intentionally
changes output coordinates to match persisted work zero; users must regenerate.
Legacy stock stays in place on reopen and still follows future drawing bbox changes.
Warnings/simulation preserve physical XY placement through explicit frame conversion.
Fixture clearance remains limited by supplied geometry and unknown holder envelope.
Z and two-sided CPS are deliberate limitations, not silently accepted no-ops.
The fork adds generic normalization/validation plus small UI adapters, not new
ShopBot semantics in geometry or a future waterjet requirement in milling.

## Validation

[UX2 qualification](../ux/UX2_QUALIFICATION.md) records independent emitted SBP
translation tests, coupled polar/approach inputs, drawing preview and pocket region
mapping, collision invariance, persistence/history and actual export recovery.
Future Z/two-sided qualification or absolute stock anchoring requires separate
approval and explicit acceptance before these guards/model choices change.
