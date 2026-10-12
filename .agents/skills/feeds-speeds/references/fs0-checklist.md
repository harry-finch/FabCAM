# FS0 checklist — Architecture and data-model qualification

FS0 is analysis/design. Do not implement the recommendation engine.

## 1. Inspect current ownership

Trace where each currently lives:

- RPM,
- feed,
- plunge,
- stepdown/DOC,
- stepover,
- tool geometry,
- flute count,
- cutter type,
- machine limits,
- stock/material identity,
- operation type.

For each value identify:

```text
source
serialized location
UI editor
runtime consumer
post-processor consumer
default/fallback behavior
```

## 2. Identify conflicts

Look for:

- values duplicated in Tool and Operation,
- hidden defaults,
- frontend-only values,
- post-specific fallbacks,
- machine limits that are not centrally modeled,
- ambiguous material representation.

Document before proposing migration.

## 3. Crate/module boundary

Evaluate whether the engine should be a dedicated crate.

Desired property:

```text
pure domain models + deterministic logic
```

Avoid dependencies on:

- UI,
- post-processors,
- ShopBot,
- waterjet,
- geometry-generation internals.

A thin ivaCAM adapter is acceptable.

## 4. Minimum domain types

Propose minimal semantics for equivalents of:

```text
FeedsSpeedsRequest
FeedsSpeedsRecommendation
ResolvedCuttingParameters
```

Do not overdesign.

Decide whether machine/material/tool/operation context should be owned values, references, IDs, or small copied domain snapshots.

## 5. Recommendation lifecycle

Define:

```text
request
→ recommendation
→ apply
→ manual override
→ persist
→ context changes
→ stale/recalculate behavior
```

No silent mutation.

## 6. Material model

Answer:

- Does ivaCAM already model material?
- Is Stock only geometry/dimensions?
- Where should material identity live?
- How does material serialize?
- How do future non-milling processes avoid depending on milling feeds/speeds data?

Use an ADR if this changes a core project model.

## 7. Tool model

Audit existing defaults:

- feed,
- plunge,
- RPM,
- stepdown,
- stepover.

Decide how they coexist/migrate with presets.

Do not keep two equally authoritative values without a rule.

## 8. Data schema

Design requirements for:

- material profiles,
- tool families,
- local presets,
- validated presets,
- provenance,
- evidence,
- schema/version IDs.

Choose file format only after inspecting project conventions.

## 9. Matching and precedence

Define:

- exact match,
- family match,
- range match,
- machine-specific match,
- conflict resolution,
- fallback behavior.

Make it deterministic and testable.

## 10. Operation context

Map current CAM operations to recommendation context.

Especially determine whether the CAM can reliably provide:

- full-width slotting,
- DOC,
- stepover,
- finishing state,
- surfacing.

Do not promise engagement knowledge the toolpath engine does not expose.

## 11. Reproducibility

Decide what gets persisted after Apply.

At minimum actual resolved cutting values must persist.

Decide which recommendation metadata also persists.

## 12. Initial tests

Define deterministic reference cases before FS1, including:

- plywood + Ø6 2F profile,
- same combination full-width slot,
- pocket at known stepover,
- machine RPM clamp,
- machine feed clamp,
- unknown material,
- unsupported tool,
- exact validated preset,
- manual override persistence.

Real physical cutting values can remain placeholders until legitimate source data exists.

## FS0 output

FS0 should leave:

- architecture documented,
- source-of-truth map documented,
- domain types proposed,
- data/provenance model proposed,
- precedence documented,
- validation plan written,
- ADRs where genuinely necessary,
- FS1 implementation scope precise.

Then STOP.
