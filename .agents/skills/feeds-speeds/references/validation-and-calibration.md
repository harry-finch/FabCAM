# Validation and fablab calibration

## Automated validation

The pure engine should have deterministic unit tests.

At minimum include cases for:

- feed/RPM/flute/chipload relationship,
- invalid zero flute count,
- invalid zero/negative inputs where applicable,
- spindle min/max constraints,
- feed/plunge constraints,
- deterministic preset matching,
- full-slot conservative policy,
- known stepover context,
- unknown material,
- unknown tool family,
- unsupported tool type,
- exact tested/validated preset,
- serialization round-trip,
- provenance retention.

## Property/invariant examples

Where appropriate:

```text
feed >= 0
rpm > 0
chipload > 0
```

for successful recommendations.

If hard limits are known:

```text
rpm <= machine.max_rpm
feed <= machine.max_feed
plunge <= machine.max_plunge
```

For the same supported context and policy, result generation must be deterministic.

Do not impose simplistic monotonic rules unless physically justified.

## Comparison tests

Compare against:

- known local historical settings,
- tool manufacturer guidance,
- VCarve-era settings where available,
- trusted reference calculators where licensing/semantics permit.

Comparison does not imply that another calculator is ground truth.

Investigate differences.

## Physical calibration

Human validation should record:

- machine ID/profile,
- material and thickness/grade,
- exact tool,
- flute count/type,
- operation,
- DOC,
- WOC/stepover,
- RPM,
- feed,
- plunge,
- hold-down,
- observed sound/vibration,
- chip formation,
- heat,
- surface quality,
- dimensional result,
- tool condition,
- operator notes.

## Evidence promotion

Define project criteria before promoting data.

Example concept:

```text
ReferenceData
→ Tested
→ Validated
```

A single successful cut may justify `Tested`; it should not automatically mean universally `Validated`.

Validation remains scoped to the documented context.

## Conservative first rollout

During early fablab deployment:

- prefer lower-risk/common materials,
- use common known-good tools,
- use controlled test pieces,
- compare engine recommendations with current lab practice,
- retain manual operator authority.

Do not turn the assistant into an automatic machine-parameter optimizer before sufficient evidence exists.
