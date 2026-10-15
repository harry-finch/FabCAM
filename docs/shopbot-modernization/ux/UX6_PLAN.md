# UX6 — Machine Profiles & Tool Library UX Redesign

2026-10-15 · **Phase 0 and A accepted; B implemented, human review ivac-y2aln pending.**
Planning record ivac-afey9; audit ivac-4df39; human gate ivac-tjgt1.
[Completed audit, proposed contracts and layouts](UX6_PHASE0_AUDIT.md).
UX6 is available after UX5; existing milestone IDs/status are retained. This is a
coordinated task-clarity/information-architecture initiative, not just visual polish.
Phase 0 was audit-only. [A results](UX6_A_RESULTS.md) describe the implemented UI;
project formats and core CAM/FS/post behavior remain unchanged.

## Goal, placement and checkpoints

Configure machines carefully once, manage tools efficiently and keep routine CAM
simple. Explain output effects, limits versus estimation, tool geometry/availability
and operation consequences without duplicate cutting-value models.

Drawings/Layers review ivac-ena7m accepted; preserve independent visual review
ivac-1drzo and PERF cancellation review
ivac-nh6q, remaining latency work and all GEOM/NEST/CAM priorities. UX6 does not
interrupt approved work or become the next task automatically. User authorized A on 2026-10-15, accepting Phase 0. User accepted A and explicitly authorized B; [B results](UX6_B_RESULTS.md)
now await human acceptance ivac-y2aln.

| Checkpoint | Future deliverable | Gate |
| --- | --- | --- |
| Phase 0 — architecture/UX audit | Field ownership/behavior matrix, action/persistence audit, selection/assignment model, information architecture and representative layouts | Accepted; ivac-tjgt1 closed |
| [UX6-A — Machine Profiles](UX6_A_PLAN.md) | Compact browser/editor, contextual output, safe edit/apply and machine-tools clarity; tests, actual screenshots/docs | Accepted; ivac-osa4z closed |
| [UX6-B — Tool Library](UX6_B_PLAN.md) | Search/list/contextual detail, physical schematic if justified, type-specific/legacy fields, association clarity; tests, actual screenshots/docs | **HARD STOP** for human acceptance |

Both A/B remain separately testable and approvable; no massive rewrite, automatic
autosave conversion or domain/schema redesign assumed. Mandatory future visual
language: [$professional-workshop-ui](../../../.agents/skills/professional-workshop-ui/SKILL.md)
plus [$cam-ux-ui](../../../.agents/skills/cam-ux-ui/SKILL.md). Shared skills remain
canonical rather than duplicating their full rules here.

## Preliminary source-backed findings — not the full Phase 0 audit

Paths repository-relative. KEEP authority and compatibility; EXTEND presentation
only after Phase 0 confirms semantics; DEFER domain/persistence/control changes.

| Existing seam | Verified ownership/behavior and planning consequence |
| --- | --- |
| `frontend/src/lib/state/workspace.ts`, `workspace.svelte.ts` | Per-installation machine_profiles/tool_inventory; native app-data workspace file or browser localStorage. Not the same persistence scope as project snapshots/history |
| `state/machine_profiles.ts` | Named machine+tools bundles; profilePayload copies into project; named profile fallback can differ from machine.name. KEEP independent embedded project snapshots |
| `components/MachineDialog.svelte` | Draft editor; Apply calls project.setMachine, Revert reseeds from committed project. Current profile picker applies/detaches immediately, not independent browsing |
| `state/project-machine-ops.ts`, `commands.ts` | applyMachineProfile replaces machine/tools/reference in one undoable step, clears output and assesses mode compatibility. KEEP; UX must not imply arbitrary profile switching preserves every tool binding without review |
| `services/workspace-session.svelte.ts::mirrorMachineProfileOnChange` | Referenced project machine/tools changes mirror into local profile. KEEP current behavior; expose consequences and audit Undo/mirror interactions before relabeling actions |
| `components/AppNavigation.svelte` | Header identity reads project.data.machine.name; not a distinct viewed-profile selection |
| `components/ToolLibraryDialog.svelte`, `state/tool_inventory.ts` | Inventory versus project editor source; committed inventory edits sync same-ID stocked copies via undoable project.replaceTools, which can mirror to referenced machine profile. Project-local/legacy copies remain distinct |
| `components/MachineTooling.svelte`, `state/tool_picker.ts`, `op_tool_constraint.ts` | Existing loadout/compatibility/assignment seams; KEEP, audit association removal/deletion and operation references |
| `components/dialog-draft.svelte.ts`, `services/file_ops.ts` | Existing drafts and toolset/machine import/export pathways; KEEP format IDs and explicit commit boundaries |
| `state/project-types.ts`, core project/tool/machine, `api/build-project.ts` | Physical/legacy metadata and wire mapping; trace actual consumers, not labels, before classification |
| `generation-context.ts`, `generated.svelte.ts`, export preflight | Actual-input freshness/export authority; no new UI readiness/compatibility truth |

The profile picker currently means project assignment. A future left-list inspection
model is a proposed behavior distinction, not already implemented. Browsing must not
mutate the active project. Introducing “Use for this project” or separate draft
inspection requires Phase 0 approval and shared existing transition handlers, not a
parallel authoritative UI machine store. Actual labels/state must be demonstrated.

“Mill 200×300” can be a generated profile display name while machine.name is empty,
making the header's “Unnamed machine” possible without separate viewed state. This
explains a possible path, not the exact screenshot or a proven bug. Reproduce name,
profile reference, committed snapshot and draft transitions, including stale drafts,
before proposing a consistency fix. No screenshot was supplied with this attachment.

## Phase 0 required field ownership and behavior matrix

Audit every important field, including default/unset/unsupported values. For each
record: UI/wire field and unit; meaning; owner and persistence; copy/reference/mirror
behavior; producer/consumers; process/post applicability; output/CAM-validation/
estimation/simulation/legacy effects; operation impact; draft/Apply/Revert/history;
visibility/grouping and source evidence. A field may have several effects; do not
assign by current label or hide safety-critical fields merely because uncommon.

Candidate inventory, **not completed classification**:

- Machine identity, unit/process/capabilities/envelope, spindle/other known limits,
  safe/fast Z, arcs/formatting, selected dialect/CPS script/options, tool-change,
  warmup/parking/end program, rapid/acceleration/jerk/toolchange-time assumptions.
- Tool kind/diameter/flutes/flute length, V included angle/tip, shank/body/holder/
  stickout/length where modeled; Z shift, spindle warmup/direction, whirling, wear,
  reference compatibility, calibration and legacy cutting defaults.

General / Program output / Tools / Advanced and Cutting geometry / Mounting /
Availability / Notes/Legacy are provisional groupings. Trace fields into build-project,
core validation/emission, post properties, simulation/timing and FS adapter first.
Never imply estimate-only acceleration configures the controller, unknown limits are
verified, or a visible capability checkbox makes output supported.

## Data and action contracts to preserve

PhysicalTool means geometry/construction/physical constraints; applied CuttingParameters
belong to an operation; Recommendation proposes from explicit context. These are domain
roles, not new types mandated here. [ADR-0004](../adr/ADR-0004-manual-operation-cutting-and-source-settings.md)
preserves manual/saved operation rates and legacy defaults as compatibility fallback.
Do not merge tool library with FS engine, introduce universal recipes or silently
recalculate operation values after editing tool/machine. FS mathematics/data/provenance
and existing fallback precedence stay unchanged; contextual stale guidance stays honest.

Library existence ≠ machine association ≠ operation assignment ≠ physically installed
or verified compatible. Current profiles contain copied tool bundles, not a newly
assumed reference-only schema. Add to machine / Remove from this machine / Open tool
details / Delete from library need action-specific semantics audited first. Global
inventory delete, loadout removal and referenced-operation repair are different.

Audit Apply, Revert, Save/Load, replace/add import, New/Duplicate/Delete and profile
selection: what changes draft, workspace, referenced profile, project, disk and history?
Apply and Revert do not imply universal transaction/rollback across workspace and
project. Import/Export are file exchange; Assign is project transition. Preserve exact
technical IDs/migrations and current auto-mirroring; no automatic-save change for easier
copy. Destructive replacement/deletion require scope, appropriate confirmation and
existing Undo where supported; do not promise workspace-wide Undo without evidence.

## Safety, product boundary and shared qualification

Qualify diameter/type/V-bit edits, machine limits/post changes, association changes,
referenced deletion, replace/add import, Undo/Redo and save/reopen. Keep authoritative
generation identity and preflight; stale/unsupported output cannot become exportable.
Unknown dimensions stay unknown; sketches/profiles/association are not physical proof.
No reinterpretation/removal of unsupported persisted fields; keep incompatibility
visible even if ordinary control is outside selected process context.

FabCAM retains CAM envelope/limits/output capabilities and legitimate program-level
park/spindle/tool-change declarations for standalone output. Future FabSBP owns live
connection/jog/home/probe/spindle/execution automation/recovery, not this editor.
No controller controls, new post capability or FabSBP implementation in UX6. See
[ownership plan](../FABCAM_FABSBP_WORKFLOW.md) and ADR-0009. Profile validation labels
need authoritative evidence: saved/configured/software-qualified/physical are distinct;
no new unsupported “validated” badge or lifecycle calculation.

Actual desktop screenshots and keyboard/focus review supplement tests. Cover realistic
normal/reduced widths, long names, unknown/incompatible values, drafts/import replacement
and associations. Use compact controlled widths, aligned short numeric inputs/units,
Lucide/contextual explanations and progressive disclosure. Advanced remains discoverable;
frequent/consequential settings and safety/export restrictions cannot disappear.

Exclude unrelated navigation/operation/FS redesign, CAM math/new processes, tool-model
rewrite, format migration without separate decision, active performance changes,
machine control or generic frontend rewrite. No ADR freezes a new model in this plan.

## Phase 0 open questions and acceptance direction

Resolve inspection versus immediate assignment and draft switching, name consistency,
write-back/mirroring, workspace versus project history, field consumers/multiple effects,
library/stocked/project-local IDs and delete/import conflicts, precise unit/type labels,
supported diagram dimensions, assistant drafts/evidence and final sections.

Phase 0 deliverable is source-backed matrix + state/action diagrams + bounded layout
proposals and compatibility/test plan. Human architecture/UX approval must precede A;
A/B acceptance criteria and actual screenshot matrices are in their specifications.
No Phase 0 implementation audit, mockup build or new behavior is claimed by this
preliminary roadmap inventory. Current approved work and independent gates stay intact.

**Phase 0/A accepted. B implemented; human software/visual review ivac-y2aln
is the current HARD STOP before another milestone.**
