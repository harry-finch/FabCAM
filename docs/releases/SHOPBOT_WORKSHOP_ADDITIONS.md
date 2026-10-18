# ShopBot V1 workshop-readiness additions

> SAFE-Z assessment — 2026-10-18: [combined results, confirmed blockers and approval boundary](POST_AUDIT_CHECKPOINT_2026_10_18.md); [TEST1 contracts](TEST1_CONTRACTS.md), [SAFE-Z contracts](SAFE_Z_CONTRACTS.md). Assessment complete, implementation awaiting approval.

2026-10-18 · **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** for these new checkpoints.
Preserve audit-correction reviews, bounded TEST1, conditional GEOM-CORE, package/platform/
physical gates and explicit V1 acceptance. No new Z workflow or machine capability here.

## SAFE-Z — Clearance and Z-reference qualification — V1 REQUIRED assessment

Audit supported stock thickness/top Z, work reference, start/final depth, rapid/fast Z,
actual emitted OpenSBP, fixture envelopes and current preflight/export guards in one
explicit frame. Reuse [ADR-0008](../shopbot-modernization/adr/ADR-0008-work-origin-frames.md)
and qualified work/output adapters. Spoilboard zero +18 mm stock/+5 mm rapid is a
potential counterexample to investigate, **not a claim that this setup is supported**.
Never add `fast_move_z > stock.top_z_mm` without proving common reference/semantics.

Map authored/drawing/stock/work/output coordinates and actual supported Z convention.
If spoilboard-based Z is unsupported, keep explicit refusal or unmistakable unsupported
warning; do not present it as usable. Proven unsafe supported motion must fail closed
or be corrected and requalified. Diagnostic warning is not permission to export an unsafe
supported case. Do not enable another Z-zero convention to satisfy this assessment.

Acceptance: supported/unsupported Z contract, independent stock/work/rapid coordinates,
decoded OpenSBP checks for clear and inside-stock counterexamples, nonzero top/start
and authored unsupported offsets, known fixtures/unknown holder data, cache/edit/Undo/
stale/critical export behavior. State exact software/physical limits. Reproduce/triage
any confirmed safety failure as release-blocking; no code or hardware action now.

## UX8 — ShopBot Setup Sheet — V1 HIGH VALUE / strongly recommended

Bounded printable/displayable view, not a large report/PDF subsystem. Include project/
generated-program identity, machine profile, stock dimensions/intended position, declared
XY origin and explicit supported Z reference; ordered tools with known geometry, effective
cutting parameters/depths, resolved-output XY/Z envelope, relevant fixture/clearance warnings
and pre-execution operator checklist. Tie it to captured current generation/context; stale
or unknown fields must not look current. It describes intent, not established physical state.

Never imply tool mounted, zero established or fixture checked from configuration. Actual
stickout/holder dimensions remain unknown when not evidenced. Compact Professional
Workshop UI + CAM UX, legible print and labels; optional FabSBP remains separate. A short
safe OpenSBP comment summary may be investigated later, with syntax/parity qualification;
no post/comment change now. Do not block V1 disproportionately on cosmetic/export scope.

Acceptance: authoritative identity/data/envelope source, ordered tool/depth values,
visible unknown/stale/unsupported states, complete bounded checklist, actual desktop/
print review and independent value/output comparison; existing project/output unchanged.
Human decides final bounded inclusion if implementation cost conflicts with V1 delivery.

## TAB-SAFE — Holding-tab entry qualification — V1 CONDITIONAL

Release-critical only if unsafe supported behavior is confirmed. Compare rectangular and
ramped tabs, vertical re-entry, known/unknown centre-cutting capability, plunge/ramp limits,
curved contours, effective retained material and decoded OpenSBP. Preserve current defaults
and [versioned holding contract](../shopbot-modernization/HOLDING_PREVIEW_CONTRACT_AUDIT_2026_10_18.md),
ADR-0022 and legacy/material-width intent; existing corrections do not qualify every tool.
Do not claim ramped universally safer or change saved/default style from suggestion alone.

Acceptance: independent cutter-swept holding/entry references for supported combinations,
explicit unknown/refused cases, material/geometry/depth/parity matrix, current defaults
retained unless comparative evidence and human approval justify change. Confirmed unsafe
supported cases become V1 blocker; style preference alone remains conditional.

## OP-ORDER — Inner-before-outer analysis — V1 CONDITIONAL / V1.5 REQUIRED assessment

Audit current warnings against actual geometric/machining dependencies: internal holes/
pockets/profiles, external through versus partial cuts, stock/fixtures, tabs/holding and
authored sequence. Nested contours alone do not prove detachment or safety. Preserve
operation order; no silent reordering. Prefer explicit understandable warning for proven
risk and indicate uncertainty when holding is unknown. Waterjet integrates this assessment
with WJ3 process-specific sequencing, not a competing analyzer.

Acceptance: safe and risky order references, partial versus through depth, held/unknown/
unheld conditions, open/nested contours, correct warning severity/explanation and unchanged
authored order/output. V1 adjustment only if demonstrated supported-workflow risk; broader
analysis remains bounded. Existing safe rejection/unsupported scope stays authoritative.

## Shared evidence/gates

Begin with source/contract audit, independent numeric/intent fixtures, lifecycle and
decoded-output comparisons; actual UX/print evidence for UX8. Software review does not
replace authorized physical ShopBot qualification. No release dates, defaults/data/schema/
CAM/post/golden changes or physical actions in this documentation update.

**HARD STOP — awaiting human approval of scope before implementation.**
