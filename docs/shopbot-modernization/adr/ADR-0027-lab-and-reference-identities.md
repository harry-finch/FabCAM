# ADR-0027: Independent lab identities and reference-pack compatibility

Status: User-authorized implementation; UX acceptance pending
Date: 2026-10-21

## Context

The v1.1 follow-up explicitly requires one machine to retain its lab identity and
reference-pack compatibility simultaneously, with lab priority and visible provenance.
Previously `fsIdentity` served both roles; Enable local presets replaced the reference.
The existing milling resolver already prioritizes eligible local Tested/Validated
exact presets. Extend that resolver; keep geometry, posts and saved-rate authority intact.

## Decision

Retain `fsIdentity` as the explicit reference compatibility link. Add `fsLocalIdentity`
to machine/tool frontend state. Generate it at creation/load boundaries and persist it
in projects, machine profiles and the tool inventory. Request construction and rendering
never generate identities. Machine/tool display labels do not establish compatibility.
The UI independently selects the lab entry and optional reference, with no activation button.
Selecting an existing lab entry asserts the same physical machine/tool, not just a similar model.

Add optional `machine_local_identity` / tool `local_identity` to native `FsContext`,
and optional `local_identity` to FS machine/tool request snapshots. The reference stays
in the existing identity field. Lab capture and authored lab-record evaluation explicitly
resolve the local identity; ordinary reference records and bounded adaptations retain
the existing reference matching. Both paths use the same numerical limits and resolver.
No combinatorial alias search, source-content rewriting or duplicated recommendation math.

Eligible tested/validated exact local presets retain their existing first ranking tier.
Draft, disabled, deleted or incompatible records cannot win. Reference data remains the
fallback when no eligible local preset matches. The suggestion and saved operation show
the selected source; existing detailed provenance and privacy rules remain authoritative.
An imported generic local source is labelled as a reference pack, not as a lab trial.

## Compatibility and consequences

Fields are optional and omitted when absent. Native legacy requests keep single-identity
behavior. Frontend legacy identities (including inherited fsContext bindings) are retained
as local identities so previously captured presets still match. An identity previously
shared by several machines stays shared: migration cannot infer their physical separation.
New entities receive independent UUIDs. Existing setup copies and profile variants keep
the same identity intentionally. Save migrated projects to retain newly allocated identities.
Legacy context references are materialized in the loaded machine/tool. A new
physical identity does not fall back to an unrelated previous project reference.
Changing a reference thereafter never changes the local identity. Material remains an
explicit grade/condition identity and is still required for reusable presets.

Workspace migrations are persisted only after successful parsing of the known schema.
Older applications are not qualified to read the additive native request fields; no release
or backward-writing guarantee is asserted. New frontend and native/HTTP engines must be
updated together. Applied/manual rates remain unchanged; context changes require review.

## Alternatives considered

Replacing the sole identity caused the reported loss of matching. A generic alias set
would also let lab trials leak between distinct machines sharing a reference model.
Duplicating every request per source would fragment ranking, adaptation and provenance.
The additive physical identity makes the distinction explicit without changing source packs.

## Validation

Engine tests cover simultaneous distinct identities, source-order independence, local
priority, disabled fallback, reference removal, different physical machines/tools sharing
one reference, malformed identity refusal and provenance. Adapter tests verify local capture,
no project mutation and round trips. Frontend tests verify automatic stable IDs, independent
reference edits, legacy identity retention, workspace/project persistence and unchanged
physical tool data. Full frontend checks and browser smoke complement those proofs.
No physical machining or new release qualification is implied.
