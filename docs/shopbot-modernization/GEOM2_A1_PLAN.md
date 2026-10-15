# GEOM2-A1 — Precise Imported-File Dimensions

2026-10-15 · **PLANNED — NOT IMPLEMENTED**. Early bounded subtask of
[GEOM2-A](GEOM2_A_PLAN.md), not a replacement/renumbering of GEOM2-A/B.
Documentation checkpoint ivac-hx3xq. May precede full selection/direct manipulation
if the implementation audit proves compatible reuse. Current GEOM1 software/UX gate
**ivac-e2jnb** and all approved priorities remain intact; no implementation starts here.

## Product goal and current foundation

Resize an entire imported drawing to exact Width or Height in mm without manually
calculating a scale factor. Example: local 83×50 mm at scale 1, desired W100 →
scale 100/83 and H≈60.2409638554 mm. The conceptual 100×60 mockup is not the expected
result for this example. Rotation/mirror/translation values are preserved.

Source audit (paths repository-relative): `frontend/src/lib/components/LayerList.svelte`
renders per-import File transform X/Y, angle, uniform Scale and mirrors. It calls
`project.patchFileTransformForImport` through the existing throttled patch/history flow.
`state/file-transform.ts` documents scale → mirror X → mirror Y → rotate → translate
about original-file bbox centre. Current TS helpers remain the legacy path; edited
GEOM1 geometry uses authoritative Rust `crates/ivac-core/src/drawing/` evaluation and
`frontend/src/lib/state/drawing.ts` adapter. Reuse current owners, not a second transform.

## Uniform-scale synchronization — normative planned behavior

Existing `fileTransform.scale` remains the only persisted scale authority. Width and
Height are derived from effective local whole-import extents before file transformation.
Let finite positive local extents be W0,H0 and positive uniform scale s:

`W = s × W0`, `H = s × H0`; Width request Wt sets `s = Wt/W0`, Height request Ht
sets `s = Ht/H0`. Recompute both display values from that same scale. Editing Scale
also refreshes W/H. Derive from the unchanged accepted local geometry and full-precision
scale, not the rounded display or repeatedly scaled segments; avoid cumulative drift.

Do not persist independent W/H values, introduce Scale X/Y, or add an interactive
unlock when nonuniform scaling is absent. Linked proportions are a truthful fixed
indicator. Existing files need no format migration if the current scale encodes it.
Positive finite requests only; preserve internal precision beyond formatted decimals.

## Bounds, rotation and source/edit semantics

File dimensions use **the one import being transformed**, not the selected contour
or combined project bbox. Include authoritative source membership irrespective of
view-only hidden layers. Distinguish immutable original import, accepted effective
local GEOM1 snapshot, transformed whole-file AABB and combined project bounds.

Chosen planning direction: W/H describe effective local dimensions **after uniform
scaling, before mirrors/rotation/translation**. Rotation45° of local100×50 leaves
editable100×50; its drawing-axis AABB differs and may be separately labeled read-only.
Mirrors preserve absolute W/H. No oriented minimum bbox or confusing mixed frames.

Audit curve-aware extrema/bounds and point membership before implementation. Legacy
frontend bbox helpers scan segment endpoints, while GEOM1 has authoritative evaluated
bounds; never assume either combined world bbox or inverse-rotated AABB recovers the
required local import extent. Reuse appropriate existing authoritative local geometry/
bounds, or approve the smallest shared read-only derivation. No frontend edit replay,
curve flattening change or duplicated Rust transformation algorithm for this UI.

## Original pivot, ADR-0021 and GEOM1 compatibility

[ADR-0021](adr/ADR-0021-edited-drawing-contract.md) and
[GEOM1 contract](GEOM1_CONTRACT.md) remain authoritative: original coordinates/pivot
→ accepted local edits → existing file similarity transform → applicable relief →
shared effective drawing. Original bbox centre remains the pivot even if a repair
removed an extremum and changed the effective local dimensions. Extent measurement
does not redefine that pivot.

Patch only uniform scale: retain translate XY, rotateDeg and mirrors. Preserve the
same qualified pivot behavior as editing Scale directly; do not compensate with a new
translation to keep an arbitrary visible corner fixed. Scaling off-centre edited
geometry can move its envelope about the preserved pivot; explain that consequence
rather than promising unchanged world position. No implicit work-zero or stock-setting
change; inherited auto-stock/bounds behavior remains observable, not newly redesigned.

Preserve source revision/entity/contour lineage and binding rules. Audit GEOM1 transform
patch/re-evaluation, pending proposals/source-conflict validation and relief/tab/approach
guards; do not bypass refusals or expand cleanup scope. Stale async results cannot
overwrite newer dimensions/geometry. Independent proof is required before early delivery.

## Numerical input and UX policy

Handle empty/point-only/zero-width/zero-height/degenerate imports explicitly, with no
fabricated size, division by zero or silent invalid commit. Audit valid positive-axis
resizing for a line with one zero extent; document supported edit versus disabled reason.
Reject zero/negative/nonfinite targets, overflowing/underflowing or unsupported scale.
Persisted legacy scale values need honest display/refusal according to existing semantics,
not normalization on read. Test small/large geometry, units and threshold/rounding behavior.

Mandatory [$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md)
+ [$cam-ux-ui](../../.agents/skills/cam-ux-ui/SKILL.md): compact W/H mm rows, fixed
linked-proportions cue, existing angle/Scale/mirror controls and aligned short values.
Keep sidebar density, keyboard/focus and clear invalid/disabled explanations; no cards
or new CAD panel. Layout is provisional, coordinate/authority policy explicit.

## Persistence, CAM and measurement boundary

Use the existing file-transform command/history and serialization, retaining import
identity, accepted GEOM1 lineage and existing CAM references. Explicit dimension edits
are coherent Undo/Redo actions with normal coalescing policy; qualify repeated edits,
cancel/incomplete numeric entry and save/reopen. Preserve current geometry identity,
generation invalidation/export preflight and final output/preview parity.
Do not scale the physical cutter, change depth/feed/tool assignment or recalculate FS.
Original SVG/DXF files remain untouched; no operation silently rebinds another contour.

Existing status-bar selected-object bounds/dimensions partially cover measurement;
when no selection it falls back to import-wide information, which can be combined
project geometry, not this per-file control. Do not duplicate it or use it as file
extent authority. No MEAS1 milestone/specification found in the current roadmap.
Record the requested future MEAS1 direction without inventing completion/priority:
interactive point-to-point distance, radius/diameter, angles, contour length and useful
additional bbox overlays. Its future scope should build on existing selection dimensions.
Editable file size is a transformation control, not all-purpose measurement. No MEAS1
implementation or new schedule dependency is introduced here.

File, future selected-group and future nesting-instance transforms may share primitives,
but retain separate ownership/persistence. This subtask includes no selection resizing,
handles, nonuniform scaling, primitives, nesting, full CAD dimensioning, oriented minimum
bbox, GEOM1 repair changes or machine-output capability changes.

## Future qualification and acceptance

All cases PLANNED, no implementation PASS claimed:

1. Width→Scale/H, Height→Scale/W, Scale→W/H through one authority; aspect preserved.
2. Independent83×50→W100 and height-driven references; repeated edits preserve precision.
3. Whole-import multi-shape relative layout, with selected-object/other-import bounds
   intentionally different; hidden layers do not silently change membership.
4. Rotation and mirrors retain local dimensions; optional AABB labeled separately.
5. Translate/angle/mirrors unchanged; original pivot before/after GEOM1 extrema repair
   independently checked, including expected envelope movement about that pivot.
6. SVG/DXF units, arc/circle extrema, tiny/large/empty/point-only/line/degenerate inputs,
   nonfinite/zero/negative targets and unsupported scales give correct result/refusal.
7. One coherent transform history, cancel/no-op, Undo/Redo and save/reopen; no new format.
8. Import/entity lineage and operation source bindings unchanged; manual values/tools
   retained, current output invalidated, explicit regenerate and final preview parity.
9. Authoritative edited/legacy evaluation agrees with displayed size; async/proposal/
   source change and existing relief/marker conflict guards remain correct.
10. Relevant automated geometry/lifecycle regressions and actual desktop keyboard/
    narrow-sidebar screenshots reviewed by a human before next milestone.

Open implementation-audit decisions: authoritative per-import effective local-bounds
access, supported degenerate-axis policy, curve extrema agreement, validation limits/
display precision, history commit cadence, async draft handling and exact early-delivery
scope. Preserve accepted ADR-0021 instead of reopening it; new bounds/pivot semantics
cannot be silently introduced. Human scope approval required before implementation.

**STOP — waiting for human approval before implementation.**
