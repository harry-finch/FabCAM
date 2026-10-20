# Two-Sided Machining & Registration

2026-10-14 · **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** as a new qualified
capability. Existing inherited code is partial foundation, not readiness evidence.
Unnumbered future general CAM milestone; no new conflicting CAM/UX label assigned.
Reuse open **ivac-jtkj** for CPS/translated-registration qualification; ivac-13go
is a separate open regression-contract issue, not feature approval. Documentation
checkpoint ivac-jqq5 does not implement or close either issue.

## Priority, objective and existing foundation

Keep active [V-Carve Qualification](VCARVE_AUDIT.md) and its human HARD STOP first.
Preserve existing approved/planned sequence and independent gates. Propose this
general 2.5D double-sided milling capability afterward, ahead of later
[V-Carve Inlays](VCARVE_INLAYS_PLAN.md) in relative priority, subject to explicit
scheduling approval. Inlays do not inherently require double-sided machining.

Target practical Side A/Side B stock setup, explicit physical flip and repeatable
registration, side-assigned operations, independent generation/preview/programs
and clear operator setup instructions for a ShopBot-class router.

Read-only foundation inventory (paths repository-relative):

| Seam | Existing behavior and disposition |
| --- | --- |
| `crates/ivac-core/src/project.rs`, `project/op.rs` | Stock flip X/Y, optional dowel count/diameter/margin and Front/Back operation side; KEEP compatible single-sided defaults; EXTEND only after model audit |
| `crates/ivac-core/src/cam/flip.rs` | Stock-centred XY reflection, arc/bulge handedness and mid-plane Z mapping; KEEP mathematical helpers, qualify their composed setup meaning |
| `crates/ivac-core/src/cam/two_sided.rs` | Bounding-box/removal-depth conflict guard with documented approximation limits; KEEP protections, not exact cumulative material proof |
| `crates/ivac-core/src/pipeline/two_sided_emit.rs` | Split operations, mirror back geometry, native front/back programs and automatic dowel preparation; EXTEND/REPLACE unsafe scaffold/preparation only after audit |
| `pipeline/work_origin.rs`, [ADR-0008](adr/ADR-0008-work-origin-frames.md) | Qualified single-sided XY normalization; KEEP frame ownership and explicit unsupported guards |
| `frontend/src/lib/scene3d/flip_gizmo.ts`, `lib/sim/two_sided_conflict.ts` | Inherited flip visualization/diagnostics; KEEP as candidates, not registration accuracy evidence |

Genuine two-sided CPS or nonzero work-offset XY is still rejected before output by
`two_sided_emit.rs`. [CAM2/UX2 evidence](ux/UX2_QUALIFICATION.md), scenario UX2-09,
and `tests/shopbot_m2_post.rs::ux2_unqualified_two_sided_shopbot_scaffold_blocks_before_invalid_output`
record that protective contract. Native unshifted behavior is historical, not newly
qualified for this fork. Existing flip/wrapper tests are inventory only; not rerun
or promoted to new qualification here. Closed inherited two-sided issues do not
supersede open ivac-jtkj or establish current ShopBot/physical readiness.

## Coordinate and registration architecture to audit

Explicitly distinguish drawing, stock, Side A work, Side B work and machine frames,
plus physical registration points. Define units, axis direction, transform order,
origin and datum ownership. Compose a mathematically specified physical 180° flip
with placement into each side's work frame; no ad hoc sign changes or post-only
coordinate repair. Account for stock dimensions/thickness, chosen X/Y flip line,
registration origin, front/back feature relationship, handedness/compensation and
each side's Z reference. Validate inverse/round-trip and pin correspondence against
independent geometric references, including nonzero stock and work origins.

Current helper convention: X flip preserves X and reflects Y about stock centre;
Y flip preserves Y and reflects X. `flip_z` maps stock top/bottom about the mid-plane.
The inherited output instead authors each side's depths relative to its own face
and expects physical Z re-establishment; applying `flip_z` to those emitted depths
would double-transform. Audit this distinction, datum/clearance conventions and
fixture transformation before selecting a future contract. A visual mirror is
not proof of physical alignment, and machine coordinates are not inferred from CAD.

Initial registration candidate: alignment pins/holes, possibly two when mechanically
appropriate. Qualify count, pin versus hole diameter/clearance, positions, separation,
stock/fixture references, seating/clamping, 2D/3D visibility and collision/keep-outs.
Do not assume arbitrary two-pin layouts are valid or transplant legacy defaults
as qualified tolerances. Registration drilling must be an explicit understandable
preparation step, with authored target/depth; no automatic spoilboard/fixture drilling.
Preserve fixture/holder uncertainty and opposing-cut/material-retention safeguards.

## Workflow, output and UX

Stock → front/back faces → physical X/Y flip → registration → assign operations
→ generate/preview Side A → inspect required flip → generate/preview transformed
Side B → separate programs and setup instructions. Reuse shared source/stock and
existing Front/Back fields; avoid copying whole projects. Additional setup categories
require explicit support. Switching active view/side must not change depth, feed or
tool assignments. Preserve single-sided load/save/Undo/Redo compatibility.

Prefer independent **Side A — Front** and **Side B — Back** outputs, plus separately
qualified optional preparation. Each must be valid for its own setup, work origin,
safe Z, capability/post selection, final emitted-path preview and warning/export
policy. Retain freshness protections across either side/setup edit. Do not qualify
a single combined file merely by adding a pause, or establish machine zero from CAM.
Keep ShopBot syntax in post/adapter layers; generic geometry remains machine-neutral.
No OpenSBP command additions or capability changes in this documentation task.

Use [$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md)
and [$cam-ux-ui](../../.agents/skills/cam-ux-ui/SKILL.md): compact Machining setup,
Single/Two-sided, A/B, X/Y flip, registration and per-side XY/Z with progressive
disclosure. Show reference edge/line, physical turning motion, final B orientation
and registration correspondence; avoid unexplained “Flip horizontal”. Reuse current
preview/inspector patterns, with advanced transforms outside the default view.

Conceptual operator stages: A placement/clamping and XY/Z references; configured
registration preparation and A program; stop/retract, safely turn about specified
axis, reseat/reclamp and verify; B XY/Z verification, clearance and B program.
These are future instruction requirements, not an authorized machine procedure.
FabCAM declares intent; operator/future FabSBP physically establishes references.
Standalone instructions remain usable without FabSBP; future structured metadata
needs its own contract approval ([ownership plan](FABCAM_FABSBP_WORKFLOW.md)).

## Future qualification and acceptance

| Matrix | Required independent evidence, all PLANNED |
| --- | --- |
| Geometry/frames | Rectangular and non-square stock, asymmetric features, X and Y flips, translated stock/work origins, pin transforms, holes/islands, round-trip and registration invariants |
| CAM/depth | A/B differing depths, side-specific Z/clearance, handedness, arcs, cutter compensation and retained material/opposing cuts |
| Setup guards | Invalid pin layouts, thickness/datums, inconsistent side/flip state, collisions/unknown holders and unsupported machine/post setups explicitly refused |
| Lifecycle | Shared-model integrity, unchanged single-sided fixtures, explicit side editing, Undo/Redo, save/reopen and stale generation after setup edits |
| Output/preview | Two independently valid ShopBot files, optional explicit preparation, final output/preview parity for each side, current critical-warning export gates |
| UX | Compact side/flip diagrams, clear registration/Z references and operator instructions; desktop keyboard/accessibility and human review |

Acceptance requires architecture/capability audit first, independent geometry oracles,
targeted and relevant regression tests, recorded per-side output and representative
screenshots, and a human software gate before any subsequent milestone. Numerical,
tessellation, output-rounding and physical-fit tolerances remain distinct.

Physical registration accuracy needs a separate supervised actual-machine protocol:
controller preview, air cut, sacrificial stock, measured front/back feature and pin
alignment, seating repeatability and errors, with machine/tool/stock/datum/conditions
recorded. No physical PASS is claimed or cutting authorized here.

Open decisions: actual frame/placement contract, per-side Z/datum ownership, pin
layout/clearance/mechanics, explicit preparation ownership, fixture positions after
flip, side-specific freshness, compatible persistence extension and CPS scaffold
without automatic zero-setting. Resolve before enabling existing blocked output.

**STOP — waiting for human approval before further work.**
