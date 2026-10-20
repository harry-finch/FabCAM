# V-Carve Inlays

2026-10-14 · **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** as a complete qualified
paired inlay capability. Inherited helper/UI presence is distinguished below.
Unnumbered later advanced V-carving milestone, after a sufficiently qualified
V-Carve foundation and preserved approved/planned work. Two-Sided Machining has
earlier proposed relative priority, not a mandatory dependency for ordinary inlays.
Documentation checkpoint ivac-jqq5; historical inlay issue **ivac-rt1.7** is CLOSED
and remains historical, not reopened or duplicated by this plan.

## Objective, foundation and dependencies

Prepare female receiving cavity and male insert from shared lettering/logo/motif
or geometric source, with explicit depth-dependent cutter geometry, pair preview
and independent programs. This is not identical outlines with a simple 2D offset.

The active [V-Carve audit](VCARVE_AUDIT.md) is the prerequisite evidence record,
not a completed qualification. It found perimeter versus sampled medial-axis limits,
pass/entry defects and current-at-audit ShopBot refusal. Ongoing scheduler/validation
work is outside this documentation checkpoint and must complete its own evidence
and human HARD STOP; do not freeze its changing implementation as qualified here.
Full arbitrary-region removal and physical inlay fit remain unqualified.

Existing seams (repository-relative): `crates/ivac-core/src/cam/vcarve.rs`,
`cam/vcarve_emit.rs`, `pipeline/op_drivers/vcarve.rs`, VCarveParams and source inset;
`frontend/src/lib/components/op_properties/VCarveSection.svelte` has inherited
“Duplicate as inlay plug”, duplicating an operation and setting `sourceInsetMm: 0.1`.
An existing driver test checks inset, not complete mating/fit behavior. No dedicated
`cam/inlay.rs` was found. The closed inherited ivac-rt1.7 description and this button
do not prove a paired generator, valid topology, assembly clearance or actual fit.
The existing 0.1 mm UI value is not a universally qualified fit allowance.

KEEP qualified V-bit geometry, generic representations, existing V-Carve engine,
operation/history and final machine-output/preview seams. EXTEND pair intent and
validation only after audit; do not duplicate a V-Carve geometry engine. Prerequisites:
included-angle/flat-tip validation, reliable depth/envelope and pass increments,
achieved variable-depth paths, topology, machine capability/output compatibility,
tool physical limits and final preview parity. Source inset alone bypasses none.

## Geometry and fit scope to qualify

Define included angle, sharp/flat tip, usable diameter/flute length, maximum depth,
female cavity, male retained insert, start depth, required overlap/engagement,
clearance/fit allowance, glue gap, stock thickness, remaining material and trimming.
For an ideal straight cone, section radius grows with depth and half-angle; a flat
tip adds its radius. Explicitly derive the male/female relationship in their respective
cutting and assembled frames, with independent section/depth/offset tests. This
ideal cutter relation does not establish a complete inlay algorithm or physical fit.

Distinguish radial allowance, axial glue space, engagement and top-trim reserve;
do not collapse them into one undocumented “gap”. Narrow/acute features, inset
collapse/split, islands, disconnected/nested regions and impossible depths must be
handled explicitly. Preserve requested versus achieved depth and actual residual
material; do not imply complete clearing from a centreline or perimeter preview.
Define numerical/chord/output tolerances separately from manufacturing tolerance.
No universal material/cutter allowance: allow qualified explicit user adjustments
and record their meaning/evidence without automatic calibration or hidden changes.

## Workflow, preview and separate output

Source selection → V-Carve Inlay → V-bit → female cavity → male insert → explicit
depth/fit parameters → pair/section preview → separate operations → separate programs.
Clearly label male and female, retain source identity and each part's stock/tool/setup,
and distinguish cutting orientation from assembled orientation. Audit whether a
compatible paired-intent reference is needed; no project schema selected now.
Undo/Redo and save/reopen must retain the relationship without automatic project
migration or silently modifying existing operations/cutting values.

Preview candidates: female cavity, male insert, assembled cross-section, depth and
geometric clearance, insufficient-depth/impossible-feature warnings and residuals.
Show software geometric clearance separately from material behavior and manufacturing
tolerance; do not claim exact fit. Reuse appropriate preview infrastructure, auditing
sampled/voxel approximation limits. No inlay preview is added to current V-Carve scope.

Export separate female/male programs for their independent stock/reference setup;
do not mix both paths into one ambiguous job. Existing generic CAM, capability
resolution, ShopBot post and final emitted-path preview remain authoritative.
Guard stale output, safe Z, fixtures, physical tool limits and current blocking policy.
Unsupported topology/toolpaths refuse rather than silently convert. No new ShopBot
geometry, syntax, machine-control/zero-setting or tool database in this plan.

Use [$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md)
and [$cam-ux-ui](../../.agents/skills/cam-ux-ui/SKILL.md) for compact paired setup,
clear part/depth labels, useful section view and progressive disclosure. Keep normal
workspace controls usable; advanced fit parameters explain their geometric meaning.
FabCAM remains standalone; future FabSBP may consume separately approved setup
metadata, not calculate fit or become a CAM dependency.

## Future qualification and acceptance

| Matrix | Required evidence, all PLANNED |
| --- | --- |
| Pair geometry | Simple insert, lettering/complex motifs, acute/narrow features, nested/disconnected regions, female/male section and dimensional relationships |
| Cutter/depth | Varying included angles, nonzero tip diameter, start-depth variation, maximum depth, usable tool/stock limits and pass/entry correctness |
| Fit parameters | Explicit overlap, radial clearance, glue gap and trimming; invalid/impossible combinations refused; residual/unreachable material explained |
| Output/preview | Separate programs and setup identity, machine/post guards, independently checked final paths and pair/section preview parity |
| Lifecycle/UX | Shared source and pair intent, Undo/Redo, save/reopen and old-project compatibility, stale blocking; desktop accessibility/captures and human review |

Acceptance requires an architecture/capability audit first, prerequisite V-Carve
qualification for the promised scope, independently derived geometric oracles,
targeted and relevant regressions, explicit unsupported cases, separate-output
fixtures and representative pair/section captures. No promotion of the inherited
duplicate/inset shortcut to qualified fit on the strength of screenshots/tests alone.
End at human software approval before another milestone.

Physical fit validation is separate: supervised controller preview/air cut and
sacrificial pairs, measured cavity/insert/engagement/gap and assembly results under
recorded cutter, material, moisture/condition, stock, datum and machining settings.
No exact-fit claim or physical cutting authorization is made by this documentation.

Open decisions: supported V-Carve coverage/topology, female/male assembled-frame
geometry, flat-tip/start-depth semantics, overlap versus glue/fit allowances, trimming
and part release, residual proof, paired persistence/compatibility, separate-file
identity and calibrated material/cutter applicability. Resolve after active V-Carve
qualification; this milestone must not expand or interrupt its bounded scope.

**STOP — waiting for human approval before further work.**
