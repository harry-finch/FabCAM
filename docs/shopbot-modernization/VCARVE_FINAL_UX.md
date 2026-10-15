# V-Carve — final UX and diagnostics qualification

2026-10-14 · User-approved final polish, ivac-8d21.
**HARD STOP: final human acceptance ivac-1b75 remains open.**
Software only. No physical validation, publication or subsequent milestone.
The [numerical qualification](VCARVE_QUALIFICATION.md) remains authoritative;
the historical audit and its evidence have not been rewritten.

## Unsupported-control audit and final inspector

| Control | Class | Final presentation / stored-value handling |
| --- | --- | --- |
| Final depth, Step/pass, tool, source selection, combination | A: supported | normal editable controls, same values and generation |
| Perimeter / Full medial axis | A with bounded semantics | same identifiers/default; contour groove / sampled spines, neither a complete pocket |
| Start depth | B: only 0 mm | read-only actual value; normal 0 mm labeled stock top; nonzero saved value also appears in incompatible settings |
| Radius cap | B: positive, greater than flat-tip radius | retained in Advanced; existing planner validation unchanged |
| Source inset | B: single contour, no split/collapse/islands | retained in Advanced; help explains restriction and that this is not qualified inlay fit |
| Through depth / Depth list | D: unsupported nondefaults | removed from normal V-Carve depth fields; every saved nondefault shown with explicit Clear |
| Finish step | E/D: contour-operation parameter, unsupported here | no normal V-Carve control; saved value shown, even zero; explicit Clear |
| Refine pass | C/D: persisted flag, unimplemented; true rejected | removed misleading permanently-unchecked checkbox; saved true shown as Enabled with Clear |
| Generic Direct/Ramp/Helix, approach, lead/tabs/directions | E: other operation controls | existing kind guards already exclude V-Carve; unchanged |
| V-bit entry angle in tool data | B: existing V-Carve entry setting | no new operation-level entry strategy; existing validation unchanged |
| Inlay shortcut | C: unqualified partial behavior | disabled in Advanced with explicit explanation; no qualified fit implied |

The compact incompatible-settings fieldset appears **before Source**, outside any
collapsed Advanced section. Values are not normalized on read/import. Each Clear
is an ordinary undoable operation patch; it changes only the named field. Backend
refusal remains authoritative. Switching selection or operation kind does not
clear stored parameters. The depth editor for unrelated operations is unchanged.

An adapter edge case was found: finish step zero and negative through depth could
previously be omitted by generic wire filtering. V-Carve now forwards these authored
values to the existing refusal, rather than silently accepting them. Other kinds
retain their existing filtering. This changes only invalid input handling, not
qualified CAM values/output. No project version or serialized identity migration.

## Inlay action decision

The historical handler duplicates the current operation, renames it with `(plug)`
and sets `sourceInsetMm = 0.1`. It does not compute a paired female/male fit, adjust
depth or establish an engagement/glue allowance. The handler/data remain present,
but the normal action is disabled. Explanation: **“Paired inlays are not qualified.
Plug duplication is unavailable.”** The old production-like fit instructions were
removed from help. Source inset remains independently useful and available within
its qualified single-contour limits. Full paired inlays stay in VCARVE_INLAYS_PLAN.

## Exact warning audit

Observed directly after generation of both software fixtures in the local browser.
All three have existing severity **warning**, non-critical/non-blocking. No severity,
critical-kind list, count or export policy has been lowered/changed.

### 1. `vcarve_target_capped` — operation 1, V-Carve driver

Original/backend message (still available in expanded JSON):

> V-Carve reaches the explicitly capped target; capped regions are not fully cleared. Reduce radius limits only if that is your intended geometry.

Final displayed message:

> Target limited by maximum depth, radius cap or cutter reach. The capped paths are complete; some material remains.

Expected for the fixtures: requested maximum depth is 1 mm, below the uncapped
geometrical target. Action: review maximum depth/radius/tool if a deeper target
was intended, not blindly reduce a radius limit. This warning concerns target
clipping; it is not evidence of scheduler undercut. The old “Reduce radius limits”
suggestion was misleading as a general recovery direction. English/German
presentation is localized through the existing warning renderer; raw evidence stays.

### 2. `vcarve_scope` — operation 1, V-Carve driver

Perimeter exact message:

> Perimeter: V-bit contour groove at constant final depth; the interior is not cleared.

Full medial exact message:

> Full medial axis: completed sampled spine paths (0.1 mm boundary spacing), not a complete-area pocket or exact swept-volume simulation.

Expected mode limitation, not an error to fix. Action: verify this is the intended
machining operation. It is related to but not a duplicate of the cap warning:
the mode itself is not a complete pocket even without a user cap. Retained as a
warning, with the affected operation name replacing the technical kind in the
collapsed row. The kind and full payload remain accessible in details.

### 3. `cps_post_warning` — bundled ShopBot post, program scope

Exact message:

> ShopBot MVP uses manual spindle start/stop and RPM; verify every operator pause. Arcs linearized at 0.01 mm. No controller/hardware qualification.

Expected for this manual-spindle post. Actionable operator/spindle/setup caution,
not a duplicate of either geometric warning. Preserved verbatim and at warning
severity. The arc clause describes the existing post policy; these V-Carve paths
are linear XYZ, not newly authorized V-Carve arcs. No physical qualification is
inferred. The program-level warning correctly has no single operation association.

## Stale/refusal behavior

Current valid output retains its normal available download. An edit marks the
previous result stale: Regenerate button, stale-warning label and disabled download.
A failed regeneration retains that previous preview and the stale state, while
showing the backend reason and “No qualified output was generated.” Structured
V-Carve misconfiguration toasts now remain until explicitly dismissed; other
operations retain their existing toast behavior. No parallel diagnostic system.

Browser scenario: generate medial (5,862 segments), set Source inset to 100 mm,
regenerate → collapse/split refusal, previous paths visible, download disabled.
Refusal remains visible beyond the previous eight-second timeout. An incompatible
saved project reveals all five fields before generation; generation refuses.
Clear Start depth → Undo restores -0.5 mm → Redo clears only it. Explicit clearing
of all remaining incompatible fields then generates the normal 130-segment
Perimeter result. No automatic correction occurs.

## Tests and numerical preservation

- Frontend: **1,241 passed, 1 skipped**, 136 test files; check 0 errors/warnings,
  lint passes, production build passes (existing size/dynamic-import notices).
- New tests cover both mode defaults, all stored unsupported fields, immutable
  inspection, explicit individual clearing, Undo/Redo, kind-change retention,
  refusal persistence, unchanged warning severity and wire forwarding to refusal.
- Existing persistence, tool-change stale-output, export-policy and failed-generation
  tests pass in that suite. No golden values updated.
- Rust V-Carve qualification: **10 passed**, including A1–A4 and independent decoded
  OpenSBP/preview parity. Reference matrix counts and coverage remain identical.
- Halfpipe: **15 passed**; CPS ShopBot guards: **7 passed**.
- SHA-256 comparison confirms the nine audited geometry/planner/driver/CPS-post
  files are byte-identical before/after this UX pass. [Hashes](vcarve/UX_CORE_HASHES.json).
- Browser reference results unchanged: Perimeter 130 moves/153 lines; medial
  5,862 moves/5,891 lines. Source profiles, modes and cutting values unchanged.
  The full 1,074-core and 51-regression baseline from the numerical milestone
  remains historical evidence; those larger suites were not rerun for this UI pass.

## Before/after visual evidence

Actual macOS browser, 1280×720 desktop viewport. No new renderer or simulation.
[Before Perimeter](ux/VCARVE-perimeter.jpg), [before medial](ux/VCARVE-medial.jpg),
[before refusal](ux/VCARVE-refused.jpg).

- A: [Perimeter supported controls](ux/VCARVE-final-A-perimeter.jpg).
- B: [Full medial supported controls](ux/VCARVE-final-B-medial.jpg).
- C/G: [Advanced settings and unavailable inlay](ux/VCARVE-final-C-G-advanced.jpg).
- D: [Warning details](ux/VCARVE-final-D-warnings.jpg).
- E: [Saved incompatible values and refusal](ux/VCARVE-final-E-incompatible.jpg).
- F: [Failed regeneration with stale preview/export blocked](ux/VCARVE-final-F-stale-refused.jpg).

A visual iteration corrected clipping of the incompatible-settings explanation;
it now wraps, stays outside Advanced and displays actual stored values. No oversized
card. Long warning rows retain expandable details and operation context. Ordinary
inspector scrolling remains; critical legacy information is placed near the top.
Fixture: [incompatible saved values](VCARVE-software-incompatible.ivac-project.json).
These are synthetic software references, never production cutting recommendations.

## Remaining limits and recommendation

Retain all geometric limits from VCARVE_QUALIFICATION: sampled medial coverage,
Perimeter island refusals, stock-top starts, bounded schedules and partial inset.
No physical accuracy, paired inlays, new entry strategy or generic Engrave export.
Future visualization ideas (cut/travel distinction, resolved 2D paths, V-bit removal)
are recorded under ivac-fqp3, **not implemented or authorized here**.

Recommend final **software acceptance of the bounded V-Carve milestone**, subject
to human review of this final UX. No numerical algorithm or post semantics changed.
**STOP — waiting for final human acceptance, ivac-1b75.**

## Text demo feedback — inspector and relief polish (2026-10-14)

User confirmed the text example works, then requested two presentation refinements.
The shared inspector segmented control now sizes to its buttons rather than stretching
across the grid column (also benefits the existing Tabs control); max-width and word
wrapping preserve narrow layouts. V-Carve scope help has 8 px top/bottom spacing and
1.45 line height. Mode buttons expose their existing selected state via aria-pressed.

3D scene lighting now uses ambient 0.35 and directional 1.5 at (100,-100,120), previously
0.7 / 0.8 at (100,-100,200). Reduced ambient fill and a lower key light distinguish
shallow walls while keeping top-face illumination similar. This is global preview
lighting only: no depth exaggeration, mesh/simulation edits, new shadows or CAM changes.

Verified on the saved Test fixture in the actual browser with Toolpaths hidden:
18,874 moves; OpenSBP byte-identical to the pre-polish export (`cmp`). Inspector
checked at approximately 360 and 550 px sidebar widths. Button group measured
140.2 px = buttons 138.2 px + border 2 px. Scope margins measured 8 px.
23 targeted existing tests pass (heightfield mesh 17, V-Carve controls 6),
Svelte/TypeScript check 0 errors/0 warnings, targeted ESLint/Prettier pass,
Vite production build passes with existing chunk/import warnings.
Evidence: [inspector](ux/VCARVE-polish-inspector.jpg),
[relief](ux/VCARVE-polish-relief.jpg). Software only. Existing text-only 2D issue
ivac-vt6e remains open; this does not start another milestone.

## Strategy terminology continuation (2026-10-14)

The user-requested strategy clarification is implemented (ivac-klww): Outline
groove (Perimeter) / Variable-depth engraving (Full medial axis), compact native
selection, contextual depth/pass help and preserved authored values/identities.
[Current screenshots, exact output comparisons and tests](VCARVE_STRATEGY_UX.md).
The prior numerical and UX qualification records above remain historical evidence.
Existing final human gate ivac-1b75 remains open; no later milestone/publication.
