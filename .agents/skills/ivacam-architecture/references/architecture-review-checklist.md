# Architecture review checklist

Use this for changes to shared models, crate boundaries, serialized project types, or cross-process abstractions.

## Existing state

- Where is the source of truth?
- Is there already a similar type or extension point?
- Which crate owns the behavior?
- Which public/serialized types are involved?
- Which generated frontend/API types depend on it?

## Compatibility

Check impact on project file loading/saving, schema generation, frontend forms/state, simulation, post-processors, CLI/server/WASM/Tauri transports, tests and fixtures.

## Process separation

Ask whether the concept is truly generic, milling-specific, or machine-specific. Consider whether laser/plasma/drag-knife/waterjet need different semantics.

Do not make a generic enum or struct absorb process-specific fields merely to avoid creating a proper process-specific type.

## Upstream strategy

Prefer existing extension point, then additive extension, then isolated adapter, and only then an invasive fork change.

Document invasive changes in the fork-delta section of the project documentation.
