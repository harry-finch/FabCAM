# ADR-0015: Reusable operation templates with explicit rebinding

Status: Accepted within bounded UX4.1 software handoff; ivac-si5w approved 2026-10-14 (supersedes ivac-51gg)
Date: 2026-10-14

## Context

Repeated milling jobs need named operation recipes. Existing workspace persistence,
OpEntry, saved cutting slots, source selection and command history already provide
the required boundaries. A project clone would carry drawing IDs, coordinates and
machine context; a cutting-knowledge preset describes a different concept.

## Decision

Keep a frontend-only, versioned library in the existing local workspace.
`state/operation-templates.ts` captures whitelisted portable settings for Profile,
Pocket and PlanarSurface, and the resolved rough/finish rate values. V1 uses front
side Endmill tools without Whirl. It excludes source IDs/layers, physical tool IDs,
zone coordinates, pattern/approach/manual-tab placements and cutting evidence.
Freeze inherited pass depth at capture; require explicit depth for distinct-tool
recipes rather than resolving ambiguous role defaults. Refuse unsupported recipes.

Reuse requires current geometry and current cutting/finish tools chosen explicitly.
Surfacing requires an explicit snapshot of the destination stock footprint.
Instantiate an ordinary independent OpEntry through addOperationCommand. Saved
rate slots have manual origins; no recommendation engine runs and no template link
remains. Existing build-project, generation, warnings, dirty state and Undo/Redo
then govern the operation. Tool/material/machine suitability still needs review.

Workspace schema 1 gets an additive operation_templates array; each entry has
version 1 and strict validation. Legacy missing arrays default empty; malformed or
duplicate entries are skipped. Writes serialize per store and transport failures
propagate to explicit library actions. This is not a cross-tab transaction system.
The library stays outside portable job files; instantiated operations use the
existing guarded cutting/surfacing formats. No new core/schema/HTTP/post API.

## Alternatives considered

- Copy a whole operation: carries stale geometry/context and private evidence.
- Merge with FS knowledge presets: conflates machining intent and cutting evidence.
- New Rust/database service: adds an unnecessary boundary for local frontend data.
- Live template-linked operations: makes later library edits silently alter jobs.
- Support every operation immediately: source-specific operations need separate
  portability policies; defer them rather than dropping their meaning.

## Consequences

Named recipes speed repeated work while jobs remain self-contained. Different
cutter diameters can be assigned, but retained values are not recalculated or
certified for that tool. Library rename/remove does not mutate existing operations.
Removal has an in-dialog restoration action; project history governs operation
creation separately. Browser storage is per origin/profile; native storage uses
the existing workspace file. Older application downgrade preservation, native
runtime, multi-window arbitration and library import/export are unqualified.
The fork delta is limited to the frontend workspace, pure adapter and shared UI.

## Validation

Eleven template/workspace/history cases exercise all three kinds, distinct rough and
finish roles, stale bindings, malformed data, actual manual values, deep copies,
source/evidence omission, ordered writes and errors. Full frontend suite passes.
Actual browser capture/reload/rename/remove/restore, explicit stock/tool binding,
Undo/Redo, saved v3/v6 reopen and ShopBot generation/export are recorded in
[UX4 qualification](../UX4_QUALIFICATION.md). Revisit V1 only with a qualified
portability policy for another operation/source/tool type.
