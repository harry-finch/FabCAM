# ADR-0009: Job intent versus physical machine setup

Status: Accepted design direction; implementation DEFERRED
Date: 2026-10-13

## Context

The user requests a durable future FabCAM/FabSBP responsibility boundary while
preserving current milling development and standalone use. FabCAM has persisted
machine/tool snapshots and qualified single-sided XY work-frame behavior;
[ADR-0008](ADR-0008-work-origin-frames.md) retains Z/two-sided limits. FabSBP and
cross-app metadata are not implemented by this checkpoint.

## Decision

FabCAM describes job requirements; FabSBP establishes machine state and executes.
Declaring work zero is distinct from establishing it physically. Model future XY
origin and Z reference independently. Do not emit zero-setting commands simply
because an origin is declared. Preserve geometry → CAM → generic toolpath → post.

Reusable future profiles separate capabilities, workshop setup and execution
workflow; FabCAM consumes only relevant capability/setup intent, not controller
procedures. Default fablab guidance is ON with explicit expert policy and visible
safety-critical state. Structured pre/post actions are normal; raw code is advanced.
Both apps remain standalone. Integration uses explicit requirements metadata,
never hidden coupling or a mandatory FabCAM project dependency.

The [workflow plan](../FABCAM_FABSBP_WORKFLOW.md) owns conceptual detail, future
acceptance and UX placement. Shared Professional Workshop UI governs appearance;
CAM and future machine-control UX provide separate domain rules.

## Alternatives considered

Putting homing/probing/I/O into CAM duplicates control configuration and mixes
intent with physical state. A second CAM editor in FabSBP duplicates job authorship.
A mandatory shared project format breaks standalone use. Origin declarations that
automatically establish zero assume unknown machine position. Separate beginner
and expert products create divergent workflows. None is the chosen direction.

## Consequences

No current schema, persistence, generation, postprocessor or UI behavior changes.
No FabSBP implementation, fixed metadata format or new milestone numbering.
Shared-model compatibility and metadata contracts need their own design checkpoint.
Current embedded snapshots and qualified transforms must remain valid; future
migration must cover existing projects, local profiles and all transports.
UX2.1 review → CAM3 → remaining milling sequence stays intact. FS4 is nonblocking.
The current fork delta is documentation only, with no upstream subsystem rewrite.

## Validation

Future checkpoints must demonstrate standalone raw-code use and standalone CAM
export, explicit metadata handling, independent XY/Z frames, no zero-setting from
intent alone, compatible saved projects and visible profile/job/live state.
Physical readiness/profile validation requires operator-confirmed machine tests;
software tests cannot award it. Revisit implementation details after profile,
Z-reference and interoperability design qualification, preserving this ownership
principle unless a later explicit decision supersedes it.
