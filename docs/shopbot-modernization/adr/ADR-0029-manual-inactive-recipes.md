# ADR-0029: Manually editable inactive recipes from presets

Status: ACCEPTED policy — explicit human instruction, 2026-10-21; implementation and UX acceptance separate
Date: 2026-10-21
Track: V1.1 recipe authoring (`ivac-fnkwz`)

## Context

The user explicitly requested creating a recipe from a preset, editing speeds and
depths, and saving an inactive Draft even when machine or material differ. The
strict unchanged-value/context-copy scope in ADR-0028 does not meet that workflow.
This decision supersedes those two restrictions, preserving the original ADR as history.

## Decision

- Replace the visible “Duplicate & adapt” entry with “Create recipe from preset”.
  Source values remain immutable and visible beside manually editable recipe values:
  RPM, feed OR chipload (preserving source kind), plunge, depth/pass and existing
  stepover. No automatic diameter/flute/material scaling, calculation or CAM apply.
- Different machine/material applicability becomes explicit review information.
  Save does not require the source scope to match the new target or create a source
  compatibility association. Keep exact target identities, milling support, known
  cutting-family compatibility, valid geometry, rights and explicit review required.
- Reject non-finite/non-positive required values, invalid stepover, depths beyond
  known flute length, and values outside known machine limits. Preserve catalog
  collision guards and recorded destination-tool RPM limit. Never clamp silently.
- Save a new UUID record, theoretical category/evidence, Draft, enabled=false,
  with no physical test/review or history inherited. Source and job/operation remain
  unchanged. Existing source/destination revision checks and native/IndexedDB CAS
  remain authoritative; cancellation creates no recipe.
- Keep original source manifest, identity/revision, values, scope, reductions,
  license/attribution and source evidence classification in provenance only. Record
  authored values and source applicability differences there as well. Target DOC and
  nominal stepover scope match edited values; do not inherit source stock thickness
  as a fact about a new target. Operation/entry/engagement intent remains source-based.
- Do not add serialized fields, a schema version, a new storage path or engine rules.
  Existing source notes carry provenance. Existing Rust validation requires Drafts
  to remain inactive/theoretical; recommendation/apply still rejects inactive Drafts.

## Alternatives considered

Keeping strict copy and merely renaming it would not fulfill the requested authoring
workflow. Automatically rescaling values is outside this instruction and would require
separate numerical policy and qualification. Saving an active or qualified transferred
recipe would misrepresent its evidence and is not authorized.

## Consequences and validation

Recipe authoring becomes usable across workshop contexts without granting source
compatibility or qualification. Required target identities and rights still need setup.
Old packs and applied projects remain readable and unchanged. No Rust, schema,
postprocessor or security-policy change is needed. Verify manual edits, invalid values,
context warnings, provenance and immutable inputs, real WASM pack validation, Draft
ineligibility, source/destination guards, UI review invalidation and persistence/reload.
Human UX review and any physical trials remain separate acceptance gates.
