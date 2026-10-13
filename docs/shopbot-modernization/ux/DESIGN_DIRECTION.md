# Desktop CAM design direction

The primary visual reference for the CAM is professional Adobe Creative Cloud
desktop software: compact density, contextual inspectors, restrained dark UI,
strong typography hierarchy, subtle separators and progressive disclosure.
This is a design-language reference, not a request to clone Adobe branding or
proprietary components. No Adobe assets, logos or proprietary icons are used.

## Working rules

Reuse app.css theme tokens and op_properties/op-properties.css before local
styling. Keep numeric fields in label / value / unit / status tracks; use tabular
numbers and fixed suffix widths. Ordinary desktop widths keep metadata in the
same row. Below 260 px of inspector content width, move secondary status below
its value without moving or hiding units. Keep full text/tool names accessible
through titles when a compact header truncates them.

The shared rhythm is 4 / 8 / 12 / 16 / 24 px with approximately 28 px controls.
Use small semibold section labels, normal field text and smaller muted metadata.
Keep existing theme colors and visible keyboard focus. Reserve strong accent
for selection, focus and Apply; warnings retain icon/text plus amber. Use subtle
section separators instead of repeated framed cards. A collapsed disclosure
keeps its label and keyboard affordance.

Applied values, proposed values and manual edits must remain distinguishable.
A stale recommendation is compact, actionable and explicitly says the cutting
values were kept; its full explanation remains available as a tooltip. Origin metadata may use a small tinted rectangle with icon and text, as in the
user-requested blue-star Suggested/violet-pencil Modified direction; avoid large
pills or success colors that suggest physical validation. Source
and evidence are secondary; full provenance stays in disclosure. No visual
polish changes matching, rates, persistence, sources or machine output.

Avoid dashboard/card layouts, mobile-size controls, oversized spacing, excessive
rounding, decorative borders, copied Adobe branding or exact proprietary layouts.
A contextual inspector should evolve within existing selection/workspace seams;
full geometry/machine/operation inspector unification is later UX work.

## Shared Professional Workshop UI skill

For new or significantly modified FabCAM UI, normally use both
[$professional-workshop-ui](../../../.agents/skills/professional-workshop-ui/SKILL.md)
and [$cam-ux-ui](../../../../.agents/skills/cam-ux-ui/SKILL.md).
The installed Professional Workshop UI skill owns shared desktop visual language;
CAM UX owns manufacturing workflows/interaction/safety. Keep them separate and
reuse current tokens/components; this policy introduces no current UI restyling.

Use one selected application SVG icon family, not emoji. Shared preference is
Lucide (or already consistent Tabler); confirm actual project adoption in a future
UI audit rather than claiming a library is installed or adding one in a docs task.
Preserve readable contrast, accessible names/tooltips and keyboard focus; semantic
state needs text/icon as well as color. Workspace remains dominant.

## Future remaining-material overlay

Future CAM4 should offer a compact Remaining material visibility control alongside
Toolpaths/Rapids where appropriate. Use an existing semantic warning token with
restrained amber/orange, optionally transparent; distinguish residuals from normal
stock, paths, selection and blocking errors. No isolated hardcoded color or large
warning cards. Compact example: “4 unreachable areas · 38 mm²”, plus a small
application warning icon; optional metrics/causes and depth/analysis limits belong
in disclosure/tooltips. Intentional tabs/allowances must not look like accidental
unreachable material. This is future guidance, not a current overlay capability.
See [remaining-material plan](../REMAINING_MATERIAL_PLAN.md).

UX2.1 adopts official `@lucide/svelte` for touched status controls. Critical fixture
feedback uses a compact red row, text and Show action; stale verdicts are not
rendered as current. Selected critical fixtures retain a dashed/thicker outline.
Direct fixture XY drag shares inspector coordinates and one release history entry;
precision remains numeric. Frame fixture bounds when diagnostics lack a precise
contact point. Avoid automatic floating-detail panels on Show. Snapping and stock,
WCS and imported geometry direct manipulation remain separately scoped future work.

Inspector accordion: opening a section may close another, but closing the active
section must not silently open a previous one. All sections may remain closed.
Explicit contextual reveals are idempotent. Short section bodies use content
height; only long content consumes available space and scrolls. Keep subsequent
headers adjacent to short bodies instead of pushing them to the bottom.

## Future FabCAM / FabSBP setup UX

DESIGN DIRECTION only; no current UI change. Use Professional Workshop UI for
both products, CAM UX for FabCAM and a future separate control/safety UX skill
for FabSBP. Do not duplicate shared design rules or create that skill now.

FabCAM Project/Stock/Job Setup declares independent XY origin, Z reference and
requirements. FabSBP separates persistent Machine profile, current Job Setup
readiness and live Machine control. Keep physical home/probe/I/O procedures out
of normal CAM inspectors; FabSBP is not a CAM editor. Origin intent must never
look like proof of physically established zero.

First-time profile setup uses an assistant; subsequent edits use a compact
inspector with small configuration indicators and collapsed Advanced. Current
physical readiness is a separate visible state. Default guided setup ON for
fablabs, machine defaults for daily jobs, exceptional collapsed job overrides.
One beginner/expert workflow uses progressive disclosure and explicit safety
policy. Structured pre/post actions are normal; raw code remains advanced.
Avoid large forms, duplicate controls and dozens of per-job options. See the
[future workflow plan](../FABCAM_FABSBP_WORKFLOW.md) for concepts and acceptance.

## CAM4 A/B implemented preview direction

Remaining material uses a compact generation-context row, operation/depth-scoped
metrics, shared warning-token fill and collapsed detail. Show in2D/Hide is explicit;
unsupported and stale contexts show concise text without a reassuring empty verdict.
View state is transient; no automatic cleanup. Intended and remaining rings share
cached canvas drawing, including holes. Analysis visibility is reset on generation;
current output/input identity, not Save/dirty, gates the overlay. See
[CAM4 A/B qualification](../CAM4_QUALIFICATION.md). CAD1/C stays separately scoped.

3D generated-path visibility has a compact Toolpaths checkbox. Hide lines/arrows
independently of imported geometry and stock simulation; retain global solid-mode
wire hiding. View changes do not mutate machining intent or recreate buffers.
Warning SVG/text use centered flex layout, independent of inline baselines.

## UX2.2 application shell and welcome (PLANNED)

[UX2.2 plan](UX2_2_PLAN.md) adds a focused command/location audit, compact header
hierarchy, project identity/status and a no-project welcome using the canonical
Professional Workshop UI skill plus CAM UX. Review global project commands,
contextual workspace controls and primary job actions according to actual usage;
these are responsibilities to audit, not mandatory additional toolbar buttons.
Reuse existing Lucide components/tokens and shared file/history/generation handlers.

Welcome is distinct from new-empty, geometry-without-operations and
operations-without-current-toolpaths workspaces. Reuse existing platform-local
recents and unsaved-work guards; old preview or successful Save never establishes
current export readiness. Qualification covers desktop density, accessible disabled
feedback, lifecycle cancellation/failure and representative screenshots.
No shared skill rules are duplicated here. This is documentation only: UX2.2
is not started, and roadmap placement/design approval precedes implementation.

## FS3.9.1 recommendation interaction

Use the existing focused Modal for proposed cutting values: concise material/operation/
diameter/flutes, aligned primary rates/depth/stepover, compact evidence, one Why these
values? explanation and secondary source provenance. Mode changes calculate transient
proposals; only Apply changes authoritative operation values. Cancel/Escape restore
focus and invalidate pending work. Local preset capture belongs to effective values,
defaults Draft and preserves the physical-evidence workflow. See FS3_9_1_QUALIFICATION.md.

### FS3.9.1 effective cutting inspector

Default inspector shows effective values, one common origin indicator when justified,
manual exceptions and the secondary preset/review action row. Full recommendation
context/provenance belongs to the existing Modal. Advanced depth-entry uses the shared
disclosure/numeric row tokens; incompatible entry warnings stay visible when closed.
Pocket overlap retains its lower-level inheritance control under an accurately named
secondary disclosure. No domain or compatibility changes.
