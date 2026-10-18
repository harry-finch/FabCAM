# Machine integration specification — template

Copy only when a real integration is authorized. Keep unknowns explicit; do not populate
with invented device/protocol values. See [guide](ADDING_A_MACHINE.md) and
[ARCH-MACH](../releases/ARCH_MACH_PLAN.md). No integration specified by this blank template.

## 1. Machine identity

Manufacturer/model/hardware revision, process, operator/source, record date.

## 2. Manufacturing process

Existing or new process; required operations/state semantics and core reuse boundaries.

## 3. Software and firmware compatibility

Exact official preparation/control software, firmware/controller and tested OS/transfer.

## 4. Capability matrix

For each operation/feature: process support, profile declaration, adapter representation,
backend availability, validator evidence, qualified limits and explicit refusal.

## 5. Input/output formats

Native output/transfer objective, units/frames/version, required vendor steps and ownership.

## 6. Reference file inventory

Fixture ID/hash, provenance/settings/intended behavior, rights/storage and expected result.

## 7. Output-format findings

Supported records/fields/integrity/precision, facts versus hypotheses/unknowns, confidence.

## 8. Process-specific semantics

Cut/non-cut transitions, tool/kerf/quality/pierce/force/depth as applicable; delegated behavior.

## 9. Machine-profile fields

Owner/unit/storage/consumer, effect on CAM/output/estimate/preview and validated limits.

## 10. Adapter architecture

Existing seam/reuse, missing minimum, capability gating, API/schema/transport implications
and ADR/compatibility decision. No speculative interface merely to fill this section.

## 11. Independent validation strategy

Final-output decoder/alternative, intent oracles, mapping/preview checks and limitations.

## 12. Positive and negative fixtures

Domains/preconditions, invalid/refused/malformed cases, repeatability and regression commands.

## 13. Physical test protocol

Human authority, staged trials, actual configuration/measurement and stop conditions.

## 14. Unsupported features and known limits

Explicit refusal/delegation, untested versions, unknown dimensions and verification gaps.

## 15. Qualification evidence

Separate IMPLEMENTED / SOFTWARE QUALIFIED / PHYSICALLY TESTED / RELEASE SUPPORTED,
with dated exact-source/artifact/machine references; absent evidence is NOT TESTED.

## 16. Open decisions and approval

Required evidence, owner, next gate and human approval record; no automatic continuation.
Use the completion/HARD STOP format in the guide. Distribution/publication requires
separate authorization and appropriate rights.
