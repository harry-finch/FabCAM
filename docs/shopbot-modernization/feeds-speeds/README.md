# Feeds & Speeds Assistant — planning contract

Practical workshop tutorial: [Create your own cutting-data sources](../../feeds-speeds/CUSTOM_DATA_SOURCES.md)
(local presets, external JSON, validated synthetic examples, backup and limitations).

## Current delivered scope

FS0–FS3.9 are delivered with bounded software qualification; the
[current milestone matrix](../../releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md#feeds--speeds-milestone-matrix)
records source evidence and actual tracker decisions. FS0/FS1/FS2 are accepted,
FS3's review was superseded by accepted FS3.5, and FS3.7/3.8/3.9 reviews were accepted
on 2026-10-18. **FS4 physical calibration is deferred/nonblocking and unstarted here.**
No software approval establishes measured machining suitability or V1 acceptance.
The approved release plan still requires minimal selected trials before recommending
real ShopBot cutting settings; deferred software work does not waive that boundary.

The sections below preserve the planning contracts and subsystem responsibilities;
future-tense FS0/FS1 wording describes their original phase boundaries. For current
remaining work use [ROADMAP](../ROADMAP.md); original FS0–FS5 scope remains in the
[historical roadmap](../history/ROADMAP_2026_10_20.md).
[FS0 qualification](FS0_QUALIFICATION.md), [ADR-0003](../adr/ADR-0003-feeds-speeds-data-ownership.md)
and [ADR-0002](../adr/ADR-0002-feeds-speeds-boundary.md) retain design decisions.

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
fallback behavior and project compatibility. The approved contracts are documented in
FS0. FS1 implements the independent engine and its synthetic tests; FS2 supplies an explicit
JSON loader and licensed, narrowly scoped source references. FS3 implements the adapter, save/reopen reproducibility and manual-override
protection; see [FS3 qualification](FS3_QUALIFICATION.md). FS3.5 implements operation ownership and UX; its software acceptance is recorded
in the current matrix. Physical calibration remains FS4.

The engine remains milling-specific. Future ProtoMAX/waterjet and drag-knife
processes may share infrastructure but must not depend on FS.

Create further architecture/data/validation/preset-authoring documents only when
qualified decisions or evidence make them useful; do not prefill speculative files.

## FS3.5 workflow and domain distinction

Tool != CuttingParameters != Recommendation. Physical cutter definitions retain
legacy cutting fallbacks; operations own resolved rates and native DOC/overlap.
Recommendations are optional proposals with provenance, applied only explicitly.
Material is job context edited at Stock; source configuration is global Settings.
Read [FS3.5 audit](FS3_5_AUDIT.md), [qualification](FS3_5_QUALIFICATION.md),
[human review](FS3_5_TEST_LOGICIEL.md) and ADR-0004. Human FS3.5 approval precedes FS4.


## FS3.7 source-pack architecture

Read [source-pack contract](SOURCE_PACKS.md), [licensing matrix](DATA_SOURCES.md),
[phase audit](FS3_7_AUDIT.md), [qualification](FS3_7_QUALIFICATION.md) and
[ADR-0005](../adr/ADR-0005-source-packs-and-cutting-knowledge.md). One engine handles
reviewed public and user-local data. Existing cutting math remains intact; v4 saves
protect private source bodies while retaining reproducible values. Human FS3.7
approval precedes FS4; no physical calibration or new public lab-data license.

## FS3.8 local authoring

See [preset workflow](PRESET_AUTHORING.md), [validation policy](LOCAL_VALIDATION_POLICY.md),
[audit](FS3_8_AUDIT.md), [qualification](FS3_8_QUALIFICATION.md) and ADR-0006.
Current operation values become private exact Sorbonne records through explicit
capture; no automatic application or physical-evidence promotion. FS4 awaits approval.

## FS3.9 targeted reopening

[Bounded matching policy](FS3_9_MATCHING.md) and [generated coverage](FS3_9_COVERAGE.md)
record the separately authorized approximate-matching checkpoint. FS4 stays
DEFERRED / NONBLOCKING; no physical validation or new proprietary datasets.

### FS3.9.1 UI interaction

The compact recommendation Modal automatically previews opening/mode changes; Apply
remains explicit. Full-width confirmation is transient until acceptance. Details and
source provenance remain accessible. Local capture stays separate, defaults Draft,
and retains existing evidence eligibility. No resolver or saved-value semantics change.
[Qualification](../ux/FS3_9_1_QUALIFICATION.md); FS4 remains deferred/nonblocking.
