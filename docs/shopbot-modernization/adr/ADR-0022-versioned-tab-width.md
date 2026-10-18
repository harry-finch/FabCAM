# ADR-0022: Versioned holding-tab width

Status: Architecture approved by user; implementation awaiting human review
Date: 2026-10-18

## Context

Audit11 identifies that the historical tab width describes a cutter-centre
zone, not retained material. Adding cutter radius silently would alter saved
jobs. Rectangle tabs also lifted entire intersecting arcs. User explicitly
approved versioning and explicit conversion, not automatic migration.

## Decision

Persist `tabs.width_mode`: missing/`legacy_center` preserves the old emitter;
`material_v1` protects a disc of radius width/2 at each projected tab anchor.
The cutter-centre exclusion radius is width/2 + cutter radius, with a separate
conservative numerical retention allowance. Per-tab width/height overrides
remain authoritative. New Profile/Pocket operations created in the UI select
`material_v1`. Backend defaults remain legacy for existing callers.

Old projects show a compatibility explanation and an explicit conversion
button announcing toolpath changes. Conversion uses the existing command
history, persists in project and operation-template data, and invalidates output.
There is no implicit conversion on load, save, duplicate or generation.

Physical-width rectangles split lines/arcs at exact footprint intersections.
Physical-width ramps have a retained plateau with approaches outside it;
legacy triangular ramps are unchanged. Variable-Z curved approaches use
bounded XYZ chords; planar portions retain arcs. No postprocessor exceptions.
Pipeline cache version is 56, public version stays 0.9.0-beta.2.

## Alternatives considered

- Global radius addition: rejected because it silently changes existing jobs.
- Keeping ambiguous width for all new work: rejected because bridge material
  can disappear when cutter diameter meets or exceeds nominal legacy width.
- Automatic numeric conversion to preserve old motion: does not solve whole-arc
  legacy behavior or ramp semantics and can produce nonpositive material width.

## Consequences

Saved legacy behavior includes its known limitations; it is not newly qualified
as physical holding. Conversion intentionally regenerates different motion.
The retained disc is a local geometric lower bound, not a proof of mechanical
holding strength or protection against another operation cutting the bridge.
Older software that ignores the new field cannot safely interpret converted
jobs: reopen converted files only in pipeline56 or later compatible builds.
No project schema migration or installer identity/data change is needed.

## Validation

See [audit results](../HOLDING_PREVIEW_CONTRACT_AUDIT_2026_10_18.md) for cutter-sweep
checks, both arc directions, overlaps/seams, decoded output, compatibility,
Undo/Redo and browser review. Physical machining remains UNQUALIFIED.
