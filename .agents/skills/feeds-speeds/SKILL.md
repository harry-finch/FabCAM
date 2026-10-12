---
name: feeds-speeds
description: Design, implement, validate, or integrate the CNC milling feeds & speeds recommendation engine, including chipload, machine/material/tool/operation context, presets, provenance, confidence, and manual overrides.
---

# CNC milling Feeds & Speeds

Use this skill for the milling-specific Feeds & Speeds Assistant.

Use it for:

- FS0 architecture/data-model qualification,
- recommendation-engine design,
- chipload/feed/RPM calculations,
- operation-aware adjustments,
- machine/material/tool context,
- preset and provenance design,
- local fablab calibration,
- CAM integration of recommended cutting parameters.

Do NOT use this skill for waterjet, laser, plasma, or drag-knife process parameters.

For architectural changes, combine with `$ivacam-architecture`.
For correctness and regression work, combine with `$cam-validation`.
For user-facing workflow, combine with `$cam-ux-ui`.
For persistent project records, combine with `$project-documentation`.

---

## Product intent

This is not an expert machinist calculator.

The product goal is a simple, pedagogical, deterministic and conservative assistant for CNC routers used in fablabs, initially focused on a ShopBot-class machine while remaining generic for other milling machines.

The normal workflow is:

```text
Machine
→ Material
→ Tool
→ Operation
→ Recommendation
```

The engine should return understandable recommendations such as:

```text
Spindle       18 000 rpm
Feed           3 600 mm/min
Plunge           900 mm/min
Depth/pass          4 mm
Stepover           40 %
Chipload          0.10 mm/tooth

Mode: Prudent
Evidence: Tested
```

The user must remain able to edit the applied values manually.

---

## Fundamental architecture rules

The recommendation engine must remain independent of:

- UI frameworks,
- toolpath geometry generation,
- ShopBot/OpenSBP,
- post-processors,
- non-milling processes.

Prefer a pure Rust crate/module with small input/output domain models.

Conceptually:

```text
CAM operation context
        ↓
FeedsSpeedsRequest
        ↓
Recommendation engine
        ↓
FeedsSpeedsRecommendation
        ↓
User accepts / modifies
        ↓
ResolvedCuttingParameters
        ↓
Saved CAM operation/project
        ↓
PostProcessor
```

The post-processor must NOT call the recommendation engine dynamically.

Applied cutting parameters are saved explicitly in the project.

A project reopened after the recommendation engine or dataset changes must continue to use its previously saved cutting values unless the user deliberately requests recalculation.

---

## Separate recommendation from resolved values

Never conflate:

```text
recommended values
```

with:

```text
values actually applied to the operation
```

The engine proposes.

The user/project resolves.

The post-processor consumes only resolved values.

Manual user overrides must never be silently overwritten.

If tool/material/machine/operation context changes after a recommendation was applied, mark the recommendation as stale or offer recalculation rather than silently changing it.

---

## Separate aggressiveness from evidence

These are independent dimensions.

### Aggressiveness

Example user intent:

- Prudent
- Normal
- Rapid

### Evidence

Example data quality/provenance:

- Theoretical
- ReferenceData
- Tested
- Validated

A conservative recommendation can still have weak evidence.

A locally validated recommendation can still intentionally use a prudent aggressiveness mode.

Do not derive UI confidence solely from aggressiveness.

---

## Calculation discipline

The basic milling relationship is:

```text
feed = rpm × flute_count × chipload
```

and therefore:

```text
chipload = feed / (rpm × flute_count)
```

when RPM and flute count are non-zero.

This equation is necessary but insufficient.

The useful recommendation must also account for relevant:

- machine constraints,
- material,
- tool/family,
- operation type,
- axial engagement / DOC,
- radial engagement / stepover,
- full-width slotting,
- finishing intent,
- verified presets,
- tool/manufacturer constraints when available.

Do not build an industrial cutting-force simulation for the MVP.

Prefer transparent rules and traceable data.

Read `references/calculation-model.md` before changing recommendation mathematics.

---

## Operation awareness

Do not treat every milling path the same.

At minimum distinguish where supported:

- profile,
- pocket,
- full-width slotting,
- finishing,
- surfacing,
- engraving.

Known full-width slotting must not accidentally receive a recommendation intended for low radial engagement.

Use actual CAM context when available.

Do not ask the user to re-enter data the CAM already knows.

Do not infer exact physical engagement from geometry unless the CAM actually provides enough information.

Read `references/operation-context.md` when adding or modifying operation-specific behavior.

---

## Machine limits

Respect known hard machine/tool constraints.

Examples:

- spindle min/max RPM,
- maximum feed,
- maximum plunge,
- tool maximum RPM when known,
- other explicitly modeled limits.

Unknown limits must remain unknown.

Never invent a ShopBot or other machine limit merely to make the calculation complete.

If a required constraint is missing:

- emit a warning,
- reduce evidence/confidence where appropriate,
- or return a recommendation with clearly documented assumptions.

Integrate hard-limit warnings with the CAM's existing validation/safety system rather than creating a competing isolated warning framework.

---

## Data must not be hidden inside algorithms

Do not hardcode material-specific chiploads, DOCs, or feeds directly into recommendation logic.

Cutting data should live in configurable/versioned data structures.

The architecture should support:

- general reference profiles,
- tool-family profiles,
- machine-specific adjustments,
- tested local presets,
- validated local presets.

Read `references/data-and-provenance.md` before designing persistent datasets or preset precedence.

---

## Material and stock are distinct concepts

Do not assume `Stock` and `Material` are identical.

Conceptually:

```text
Stock
= physical raw-workpiece dimensions and placement

Material
= cutting identity/properties used for recommendations
```

They may be related in the project model, but they serve different responsibilities.

Before adding a new Material type, inspect the current ivaCAM model and use `$ivacam-architecture`.

---

## Tool data ownership

Audit existing ivaCAM tool fields before adding new cutting-parameter fields.

A physical tool may contain geometry and manufacturer limits.

A cutting preset may depend on:

- tool,
- material,
- machine,
- operation,
- engagement.

Avoid creating multiple silent sources of truth for RPM/feed/plunge/stepdown/stepover.

Existing tool defaults may become:

- fallback values,
- migrated legacy values,
- generic defaults,

but this must be decided explicitly during FS0.

---

## Preset precedence

Recommendation precedence must be explicit and testable.

A possible conceptual hierarchy is:

```text
exact validated preset
        ↓
tested local preset
        ↓
machine/material/tool-family profile
        ↓
general reference data
        ↓
theoretical fallback
```

Do not implement this exact hierarchy blindly.

FS0 must decide the actual policy based on the project data model.

Whatever policy is chosen must:

- be deterministic,
- expose provenance,
- be covered by tests,
- avoid accidental broad matching of narrowly validated presets.

---

## Provenance

A recommendation should be explainable.

Useful metadata may include:

- data/profile ID,
- source,
- engine version,
- dataset version,
- evidence level,
- applicable machine/material/tool/operation scope,
- applied correction rules,
- whether a machine limit clamped the result.

Do not create opaque recommendations with unexplained "AI" confidence.

This subsystem should remain deterministic and explainable unless the project explicitly chooses a different architecture later.

---

## Validation

Every recommendation-engine change needs tests appropriate to its semantics.

At minimum consider:

- chipload/feed equation,
- positive/valid ranges,
- machine-limit clamps,
- deterministic output,
- preset precedence,
- unknown-data behavior,
- full-slot behavior,
- serialization round-trips,
- manual-override persistence at integration level.

Read `references/validation-and-calibration.md`.

Do not claim a preset is physically `Validated` unless the documented human machine-validation criteria have actually been met.

---

## FS0-specific workflow

When this skill is used for **FS0 — Architecture and data-model qualification**, do NOT implement the recommendation engine yet.

FS0 should:

1. inspect existing tool, machine, stock/material and operation models;
2. trace current RPM/feed/plunge/stepdown/stepover ownership and data flow;
3. identify current post-processor inputs;
4. identify conflicts or duplicated sources of truth;
5. propose the engine crate/module boundary;
6. propose minimal request/recommendation/resolved types;
7. decide whether a first-class Material model is needed;
8. design preset/provenance/evidence concepts;
9. design precedence/fallback semantics;
10. define saved-project reproducibility semantics;
11. define initial deterministic validation cases;
12. document decisions and create ADRs only where warranted.

FS0 should end with an architecture that can be implemented in FS1, not with speculative production code.

Read `references/fs0-checklist.md` during FS0.

---

## Scope discipline

Do not implement during the initial engine work:

- automatic ML-based recommendations,
- chatter simulation,
- tool deflection simulation,
- cutting-force finite-element models,
- automatic optimization from machine telemetry,
- diagnostic troubleshooting assistant,
- waterjet process parameters,
- vinyl/drag-knife parameters.

These are separate future concerns.

The future diagnostic assistant is explicitly deferred until after the recommendation engine and fablab validation are established.
