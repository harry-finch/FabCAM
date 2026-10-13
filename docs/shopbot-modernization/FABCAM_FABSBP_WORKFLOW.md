# FabCAM ↔ FabSBP — future setup and execution workflow

Status: DESIGN DIRECTION / FUTURE; documentation only, 2026-10-13.
FabSBP is a future application, not an implemented subsystem in this repository.
This plan does not authorize development or move current milestones aside:
UX2.1 human review → CAM3 → remaining milling roadmap. FS4 stays deferred and
nonblocking; Feeds & Speeds expansion stays frozen. No waterjet/vinyl work here.

## Durable ownership

**FabCAM describes what the job requires. FabSBP establishes the required
machine state and executes the job.** FabCAM may declare a work zero; FabSBP
establishes that zero physically. See [ADR-0009](adr/ADR-0009-job-intent-machine-setup.md).

| Concern | FabCAM job intent | Future FabSBP setup/execution |
| --- | --- | --- |
| Machining | Stock, geometry, physical tools, operations and toolpaths | Verify installed tool/spindle; execute the prepared job, not edit CAM |
| Coordinates | XY work origin, independent Z reference, relationship to drawing/stock | Home, know machine coordinates, jog, physically register XY, probe/touch off Z and establish work zero |
| Safety | Required capabilities and relevant job requirements; CAM warnings/preview | Visible machine readiness, fixture confirmation and execution safety checks |
| Execution | Produce machine output and setup information | Spindle/accessory control, machine pauses, run/pause/resume, live feed override and park |

Normal FabCAM setup must not grow controller connection, probe configuration,
homing actions, spindle I/O, dust extraction or park movement controls. FabSBP
must not become a second CAM editor. Preserve geometry → CAM operation → generic
toolpath → postprocessor → machine-specific output. Execution/setup semantics
must not enter generic geometry; future non-milling processes must not distort
milling semantics.

## Current implementation and disposition

| Existing seam | Disposition | Future implication |
| --- | --- | --- |
| `Project.work_offset` / `WorkOffset` in `crates/ivac-core/src/project.rs`, generic normalization in `pipeline/work_origin.rs` | KEEP | Preserve qualified single-sided milling XY transform and authored project coordinates |
| Machine settings in `crates/ivac-core/src/project/machine.rs`; named machine/tools bundles in `frontend/src/lib/state/machine_profiles.ts` | KEEP now / EXTEND only after design | Existing embedded machine/tools snapshot keeps projects independent of local profile availability; not yet the shared cross-app model |
| Project serialization, generation transports and postprocessors | KEEP | No schema, persistence, API or post changes in this checkpoint |
| Shared profile, setup manifest and FabSBP execution workflows | DEFER | Separate design/compatibility checkpoints before implementation; no parallel engine or control subsystem now |

## XY work origin and Z reference

Treat these as distinct declarations, not necessarily the same physical origin.
XY choices may be stock bottom-left, stock center, drawing origin or a custom
point (including a fixture registration point). Z choices may be stock top,
spoilboard, fixture plate or a custom reference surface. Example: XY at stock
bottom-left with Z on spoilboard; XY at fixture registration with Z on stock top.
These are future UX choices, not claims of current support.

The existing coordinate contract remains:

```text
drawing coordinates → FabCAM work-origin transform → job coordinates → machine output
```

[ADR-0008](adr/ADR-0008-work-origin-frames.md) still governs the qualified XY
behavior and drawing-frame preview. Nonzero numeric Z and unqualified two-sided
output remain guarded. The future Z-reference design must qualify operation
depths, stock thickness, safe clearance, fixtures, output and preview together
before enabling it; merely naming a reference surface does not resolve them.
Future UI must show the declared origin, its stock/drawing relationship and the
separate need to establish that reference physically.

**Do not automatically emit zero-setting commands such as Z2/ZZ merely because
FabCAM declares an origin.** Physical zero establishment depends on actual
machine position and belongs primarily to setup/execution. This is an ownership
rule, not a command syntax specification or a change to the current ShopBot post.

## Shared Machine Profile Model — conceptual target

Configure carefully once, reuse across jobs, and rarely edit. Separate:

| Profile area | Conceptual content |
| --- | --- |
| Capabilities | Identity, envelope XYZ, feed/spindle limits, supported WCS, controller/dialect/post capabilities |
| Workshop setup | Fixed stops/registration points, probe/touch plate, park position, accessories, known fixture locations |
| Execution workflow | Guided policy, structured pre/post actions, zeroing strategy and safety checks |

FabCAM may share the CAM-relevant identity, envelope, feed/spindle limits,
post/controller capability and WCS subset. Controller connection, physical homing
procedure, probe inputs, I/O, dust extraction, park commands and execution safety
procedures belong to FabSBP. Sharing a concept does not require both applications
to understand every field or expose the same configuration UI.

Future lifecycle ideas: **Draft → Configured → Software validated → Physically
validated**. These are proposed machine-profile states, not existing FS evidence
labels. Configured does not mean tested; software checks do not prove physical
readiness. Guided checks should cover connection, homing, axis directions,
envelope, spindle, probe input, accessories and park. Physical-motion tests require
explicit operator confirmation and recorded evidence. No Sorbonne dimensions,
controller behavior or profile qualification is inferred from illustrative UI.

## Guided Machine Setup and daily use

Default guided setup ON for fablab use, driven by the machine profile:

```text
Home → Load job → Review required setup → Establish XY → Establish Z
→ Verify fixtures → Verify tool/spindle → Start required accessories
→ Run → Post-job actions
```

One workflow serves beginners and experts through progressive disclosure. Experts
may skip already-satisfied steps, simplify confirmations, use fixed coordinates
or advanced raw code under explicit policy. Safety-critical readiness must remain
visible and must not be silently bypassed. A declared origin or imported manifest
is a requirement, never proof that the machine is ready.

Normal jobs use machine defaults. Exceptional changes live under a collapsed
“Override for this job” section; do not expose dozens of per-job machine toggles.
Persistent profile configuration, current-job readiness and live execution state
remain distinct.

First-time profile creation uses an assistant:
Machine → Motion & limits → Spindle → Homing & coordinates → Z probing →
Accessories/I/O → Pre-job → Post-job → Safety → Test & save.
Daily edits use a compact inspector: machine identity/capabilities, small text/icon
home/probe/park configuration indicators, guided policy, named pre/post workflows
and collapsed Advanced. Configuration indicators must not imply current physical
readiness; the Job Setup area owns that state.

Advanced repeatable-fixture direction: repeatable home → known machine frame →
known fixture/stock location → declared CAM origin → automatic or semi-automatic
physical work-coordinate establishment. Requires verified registration, repeatable
home and explicit safety policy; no guessed machine coordinates or automatic
zeroing are introduced now.

## Pre-job and post-job workflows

Prefer structured profile actions rather than raw code in the normal interface.
Pre-job examples: home where applicable, verify XY, establish Z, check clamps,
verify tool, start extraction and confirm spindle. Post-job examples: safe retract,
stop spindle/accessories, move to park and show completion state. Future design
must specify prerequisites, ordering, safe failure/interruption and readiness
invalidation; a list of configured actions alone is not a safety guarantee.

Machine-specific pre/post code, potentially OpenSBP snippets, remains a secondary
advanced option with explicit policy. Normal operators should not write machine
code. No snippet syntax, action schema or execution implementation is specified.

## Standalone operation and explicit interoperability

FabCAM remains usable without FabSBP. Future human-readable Job Setup instructions
show XY origin, Z surface and requirements such as home/XY/Z establishment; they
may accompany exported machine code. Using those files must not require FabSBP.

FabSBP remains usable without FabCAM. A raw `.sbp` without metadata still supports
manual setup, guided generic setup and machine-profile defaults. Present choices
such as “Machine already prepared” and “Guide me through setup”; missing metadata
must stay explicit and must not be interpreted as satisfied requirements. FabSBP
must not require FabCAM project files.

**Job Setup Manifest / Job Setup Metadata** is a future lightweight requirements
contract, not an execution script. Conceptual content: independent XY intent and
Z surface, stock dimensions, expected machine identity/capabilities, and home/XY/Z
setup requirements. The receiving application checks requirements against its
profile and physical state rather than blindly executing declarations.

No serialization, schema, filename, sidecar convention or version is frozen.
A dedicated interoperability design checkpoint must decide units/frames,
identity/capability matching, unknown/missing/stale metadata handling, versioning,
compatibility, validation and explicit handoff. There is no hidden shared project
state or mandatory runtime coupling. Export/consumption follow that checkpoint;
current saved-value and profile persistence remain unchanged.

## UX placement and shared visual language

| Application area | Future responsibility |
| --- | --- |
| FabCAM Project / Stock / Job Setup | XY origin, Z reference and job setup requirements; not unrelated operation inspectors |
| FabSBP Machine profile | Persistent reusable configuration |
| FabSBP Job Setup | Current preparation procedure and visible readiness |
| FabSBP Machine control | Live jogging, run/pause/resume, feed override and machine state |

Both products use [$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md)
as shared visual/interaction guidance. [$cam-ux-ui](../../.agents/skills/cam-ux-ui/SKILL.md)
adds CAM-specific rules. A future FabSBP UX skill should add machine-control and
safety behavior, without duplicating shared design rules; none is created here.
Use compact contextual inspectors, concise copy, small semantic text/icon states,
existing tokens/icons and collapsed advanced detail. Avoid enormous forms,
duplicate configuration, extra cards and normal-flow raw-code fields. See
[desktop design direction](ux/DESIGN_DIRECTION.md).

## Future checkpoint dependencies and acceptance

The unnumbered [roadmap groups](ROADMAP.md#future-fabcam--fabsbp-workflow) are design
directions, not development authorization. Shared profile design precedes shared
profile consumers and guided policy. Qualified origin/Z semantics precede accurate
setup instructions and metadata export. The explicit metadata design precedes
export/consumption, but standalone FabSBP generic setup does not depend on it.
Guided setup depends on profile, zeroing/probe safety and structured pre/post
contracts; repeatable fixture automation follows physical registration evidence.

Each future checkpoint requires scoped human approval. Acceptance must demonstrate
both standalone modes, explicit ownership and visible safety state; compatible
old-project/profile load-save behavior; independent XY/Z semantics; no zero-setting
from declarations alone; default guided flow with explicit expert policy; and a
compact shared UI with separate persistent/job/live contexts. Physical profile
validation requires actual machine evidence. Existing milling and FS evidence is
not broadened by this document.
