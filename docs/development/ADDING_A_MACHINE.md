# Adding a machine to FabCAM — initial developer integration guide

2026-10-18 · Documentation-only draft for [ARCH-MACH](../releases/ARCH_MACH_PLAN.md).
No new integration is authorized by this guide. Start with the
[specification template](MACHINE_INTEGRATION_TEMPLATE.md), existing
[architecture](../shopbot-modernization/ARCHITECTURE.md), project instructions and
relevant process/post/validation skills. Preserve existing machine support and GPL credit.

## Discover and choose the smallest extension

Read current modes/operations, profiles, pipeline/post/CPS IR, actual output validators,
preview/simulation, source evaluation/identity, persistence/transports/inspectors and
qualification tests. Classify KEEP/EXTEND/REPLACE/DEFER with source evidence. Same
supported process should normally add capabilities/profile/adapter/verification/fixtures;
a new process may need physical operations/UI. Do not duplicate geometry, scatter
machine-name conditionals or build generic plugins before proving a real need.

## 1. Identify machine and evidence

Record manufacturer/model/revision, process, firmware/controller, official preparation/
control software versions, operations/axes/units/origins/physical limits, formats/transfer,
manuals/SDKs and licensing/integration restrictions. Separate confirmed fact, inference
and unknown; unknown physical limits are not defaults or validated values.

## 2. Investigate official output references

Obtain lawful representative official-software programs and record file hash/provenance,
software/settings, intended geometry/operation and redistribution rights. Request controlled
missing files rather than inventing them. Candidate formats may be G-code/OpenSBP/OMX/
HP-GL/GP-GL or binary; names/extensions are not compatibility evidence.

Vary one feature at a time: line/arc, open/closed, units/origin, cut/traverse, process
parameters, leads, tool/operation transitions and sequencing. Compare bytes/structure
where appropriate, then semantics. For binary/proprietary files investigate lawful
headers/records/coordinates/metadata/version/integrity, with confidence for every claim.
Do not bypass encryption, access controls/protections or license restrictions. Keep
restricted fixtures local; never publish without rights/explicit authorization.

## 3. Establish a supported output contract — HARD STOP

List supported records/commands, units/frames/movement, process transitions, metadata,
precision/limits, version compatibility, unknowns and explicit refusals. Assign what CAM,
adapter, vendor software and operator own. Profiles cannot enable unsupported adapter
features. Stop if required semantics cannot be established; a plausible file is not proof.
Obtain human contract/architecture approval before a production adapter. For ProtoMAX,
use [OMX1](../releases/OMX1_PLAN.md): native OMX to MAKE, no LAYOUT preparation fallback.

## 4. Implement only after approval

Extend the smallest suitable process/adapter boundary. Reuse geometry/transforms/identity,
history, caches, guards and transport support; preserve unrelated manufacturing behavior.
New API/schema or IR requires consumer/version/compatibility assessment and ADR when
significant. Do not assume all processes fit existing milling motion/tool semantics.
No implementation performed or authorized by this documentation task.

## 5. Verify actual output independently

Decode/inspect the emitted program where feasible; compare coordinates, cut/traverse,
tool/kerf compensation ownership, order, depth/quality, process state, numeric/safety
limits and unsupported records with independent intent references. Preserve source-line
mapping and resolved-preview fidelity. If complete decoding is impossible, document a
narrower trustworthy alternative and blind spots; approval must match that scope.
Internal pre-post preview alone cannot establish final-program correctness.

## 6. Differential and negative qualification

Use reference semantics plus independent geometry oracles, positive/negative/malformed/
unsupported fixtures, deterministic repeats and lifecycle/cache/history/save tests.
Compare equivalent output behavior, not mandatory byte identity when encodings differ.
Never replace goldens just to silence a failure. Record commands, versions, seeds,
fixtures/results/skips, bounds and unrelated-machine regressions. Human software gate.

## 7. Verify manufacturer-software acceptance

When required, test exact installed version's file acceptance and interpreted movement/
process intent. An extension, our parser or successful structural import is insufficient.
Record target-software corrections/reordering/delegated calculations and limitations;
do not require a prohibited preparation step as a hidden workaround.

## 8. Separate physical and release qualification

Prepare operator-approved stages: file acceptance, non-cutting/safe equivalent where
applicable, simple geometry, dimensions, process behaviors/rejections and representative
workflows. Record actual machine/software/setup/tool/material, measurement method,
observations and anomalies. Human-supervised machine actions require separate authority.
SOFTWARE QUALIFIED ≠ PHYSICALLY TESTED ≠ RELEASE SUPPORTED; implementation alone
is none of those. Candidate/package identity and release matrix must match the evidence.

## AI-agent evidence and handoff rules

Inspect source first; keep observation/inference/unknown ledger. Ask for missing official
references when necessary. Never invent formats/commands, claim untested compatibility,
edit unrelated processes or confuse software with physical evidence. Propose architecture
before implementation, add scoped regressions and update durable docs. Stop when format/
machine semantics cannot be established. No automated machine motion or publication.

Standard completion report:

- Machine/process/adapter scope, exact supported and refused subset.
- Evidence/rights/versions and architecture decisions/changed files.
- Actual output validation and differential/negative/regression results with commands.
- Vendor acceptance, physical and release evidence **separately**, including NOT TESTED.
- Compatibility/known gaps, next checkpoint and specific human decisions.

Standard stop statement: **HARD STOP — awaiting human approval of [contract / architecture /
software results / physical protocol / release], with [unresolved evidence]. No further
implementation, machine action or publication authorized.**

Proposed future `$machine-integration` entry point is described in ARCH-MACH; no skill
is created/installed now. The guide and template remain authoritative documentation.
