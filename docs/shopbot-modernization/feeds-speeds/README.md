# Feeds & Speeds Assistant — planning contract

Status: post-M4 planned track; FS0 not started. This document captures the
user's requirements, not a qualified data model or implemented engine.
See [ROADMAP](../ROADMAP.md) for FS0–FS5 scope and approval gates and
[ADR-0002](../adr/ADR-0002-feeds-speeds-boundary.md) for boundary decisions.

## Purpose and flow

A simple, conservative and pedagogical CNC milling assistant, functionally
integrated with CAM but architecturally independent. The pure Rust engine
must be independent of UI and toolpath geometry generation, independently
testable and potentially reusable as a standalone library/tool.

```text
CAM operation context → Feeds & Speeds adapter → Feeds & Speeds engine
  → Recommendation → User accepts or modifies → Resolved cutting parameters
  → Project / CAM operation → PostProcessor
```

The post-processor must never call the recommendation engine dynamically.
Applying a recommendation writes explicit cutting parameters into the project.
Reopening must not silently change feeds/speeds when data or algorithms evolve.
Manual overrides remain possible and authoritative until the user deliberately
requests recalculation; recalculation must not silently overwrite them.

## Context and recommendation

Expected inputs: machine profile, material, tool diameter, flute count, cutter
type, operation type, DOC, and radial engagement/stepover when available.
Likely outputs: RPM, feed, plunge, recommended DOC/stepover, chipload, warnings,
aggressiveness, evidence/confidence and recommendation provenance. Exact types,
units, optional fields and unsupported/missing-context behavior belong to FS0.

`Feed = RPM × flute_count × chipload` is a fundamental relationship, not the
whole feature. Recommendations must account for material/tool/machine/operation
context. This planning document supplies no cutting values or algorithms.

Aggressiveness (for example Prudent / Normal / Rapid) and evidence (Theoretical /
Reference data / Tested / Validated) are independent concepts. Conservative
settings do not imply high reliability. Evidence/confidence meaning and
validation criteria must be explicit; no validated label without documented
criteria and qualifying results.

## Data and authority

Use configurable cutting data, not hardcoded material/tool values in algorithms.
Allow general reference profiles, local tested presets, and presets validated
for a specific machine/material/tool/operation combination. Record sources,
permissions, versions, applicability limits and provenance; do not copy
proprietary data without permission. M4's successful single MDF trial must not
be silently promoted into a generally validated recommendation preset.

FS0 must qualify physical Tool vs cutting-preset identity, material ownership,
request/recommendation/resolved models, schema/versioning, preset precedence,
fallback behavior and project compatibility. None is selected here. FS0 must
also plan deterministic engine tests, data validation, adapter coverage,
save/reopen reproducibility and manual-override protection.

The engine remains milling-specific. Future ProtoMAX/waterjet and drag-knife
processes may share infrastructure but must not depend on FS.

Create further architecture/data/validation/preset-authoring documents only when
qualified decisions or evidence make them useful; do not prefill speculative files.
