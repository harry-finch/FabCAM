# Data and provenance

Cutting recommendations are engineering data. Treat provenance as part of the data model.

## Data categories

Useful conceptual categories:

### Reference data

Derived from a documented manufacturer, handbook, reputable source, or project-maintained baseline.

Not locally validated merely because it comes from a reputable source.

### Tested

Actually tried under a documented machine/material/tool/operation context.

### Validated

Meets the project's explicit validation criteria for that context.

Do not let `Validated` silently generalize beyond its scope.

## Scope

A preset may be scoped by some combination of:

- machine or machine family,
- material/material family,
- tool or tool family,
- diameter range,
- flute count,
- cutter type,
- operation,
- DOC range,
- radial engagement range,
- material thickness where relevant.

Avoid matching a narrow preset too broadly.

## Stable identity

Persistent data should have stable IDs.

Do not rely on display names as identity.

Useful metadata may include:

- schema version,
- profile ID,
- revision,
- source/reference,
- author/organization,
- creation/update date,
- notes,
- evidence level.

## Versioning

A project should persist the actual cutting parameters it used.

It may also persist recommendation metadata such as:

- dataset/profile ID,
- dataset revision,
- engine version.

If the profile later changes, reopening the project must not silently modify its stored values.

## Precedence

Define matching and precedence explicitly.

Questions FS0 must answer:

- What is an exact preset match?
- Can family profiles inherit?
- Can a validated machine-specific preset override generic reference data?
- What happens when two presets have equal specificity?
- How are ranges matched?
- What is the fallback when no useful profile exists?

Ambiguity should produce a deterministic rule or an explicit warning, not iteration-order-dependent behavior.

## File format

Do not select TOML/JSON/YAML solely from personal preference.

Evaluate:

- existing ivaCAM conventions,
- Rust tooling,
- schema validation,
- human editability,
- comments,
- versioning,
- distribution/update strategy.

A significant persistent-format choice should use the project's ADR process.

## Licensing

Do not copy proprietary feeds/speeds tables without permission.

Record the source/license or origin of imported reference data.

Prefer original project data, manufacturer-permitted data, open/licensed references, and locally measured presets.
