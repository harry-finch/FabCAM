---
name: feeds-speeds
description: Review FabCAM authored milling cutting values, machine/tool validation, tested preset collections and reviewed numerical reuse; calculator material is historical reference only.
---

# Milling cutting values and presets

Use this skill for current FabCAM authored RPM/feed/plunge/DOC, actual tool/stage
ownership, exact material identities, local tested presets and advisory value reuse.
[Current cutting contract](../../../docs/CUTTING_CONTRACT.md) and
[preset workflows](../../../docs/TESTED_PRESETS.md) are authoritative.

## Current boundaries

- FabCAM contains no integrated Feeds & Speeds recommendation engine or active
  source-pack runtime. Do not recreate `ivac-feeds-speeds`, Apply recommendation,
  automatic chipload conversion or recipe/Trial promotion during ordinary work.
- Project/cutting V10 and collection V2 are current. New Profile/Pocket use Main
  Cutting and optional Finishing under ADR-0032/0033, not legacy Rough/Finish.
- Tool defaults seed operation creation once; generation validates explicit values,
  actual stage/tool assignments, known limits and independent geometry proofs.
- Strict tested preset matching remains distinct from ALPHA-4's explicit selective
  reuse of starting values. Source attestation never transfers; stepover reuse is
  unsupported. No automatic scaling, normalization or context migration.
- FabSpeed is independent. Optional one-way integration is a proposal requiring its
  own approved contract; CAM/export must not depend on a live calculator.

Use architecture, validation, UX and documentation skills when those boundaries
are affected. Keep non-milling parameters in their own process contracts.

## Preserved reference material

The following material predates ADR-0030. Its formulas and safety explanations are
useful reference, **not current implementation instructions or qualified machining
recommendations**. Do not restart FS0 or use its old data models as current API:

- [Calculation formulas](references/calculation-model.md)
- [Historical data/provenance](references/data-and-provenance.md)
- [Historical FS0 checklist](references/fs0-checklist.md)
- [Operation engagement reference](references/operation-context.md)
- [Validation and physical calibration reference](references/validation-and-calibration.md)

Preserve units, attribution and physical-test boundaries. A mathematical relationship
or another workshop's record does not qualify the target machine/material/tool.
