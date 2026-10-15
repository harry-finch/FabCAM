# V-Carve strategy clarification — software UX handoff

2026-10-14 · follow-up ivac-klww; existing human gate **ivac-1b75** remains open.
This is a presentation refinement of the qualified V-Carve operation, not a new
CAM milestone. The numerical [qualification](VCARVE_QUALIFICATION.md),
[final diagnostics audit](VCARVE_FINAL_UX.md) and ADR-0017 remain authoritative.

## Verified semantics and labels

The former Perimeter / Medial axis segmented control named algorithms. Users had
to infer the machining result, and longer meaningful labels would crowd its row.

| User-facing strategy | Existing internal identity | Intended result | Depth presentation |
| --- | --- | --- | --- |
| **Outline groove (Perimeter)** | `fullMedialAxis` absent/false; wire `full_medial_axis=false` | Offset V-shaped contour groove, constant final depth and qualified progressive passes; interior is not cleared | **Final depth**, subject to cutter reach/radius limits |
| **Variable-depth engraving (Full medial axis)** | `fullMedialAxis=true`; wire `full_medial_axis=true` | Sampled internal paths with depth adapted to local width and cutter shape; useful for lettering/motifs; no complete-pocket or arbitrary-area coverage guarantee | **Maximum carving depth**; local targets may be shallower |

Verified against `pipeline/op_drivers/vcarve.rs` and `cam/vcarve_plan.rs`.
The negative-Z convention is unchanged. **0 retains its existing automatic
geometry-limited meaning**, so both contextual depth tooltips explicitly explain
it. No new depth field, reset or recalculation occurs on a strategy change.

Step/pass retains its current control, inheritance and authored value. Tooltip:
“Maximum additional depth per pass along the carving path, relative to the
previously cleared groove. Not a volume or absolute ramp-angle limit.” This
matches the qualified piecewise-linear envelope; later passes can have an
absolute XYZ slope steeper than the entry angle.

## Selector, explanations and accessibility

A full-width native select displays the two result names, followed by one
contextual description. The existing V-Carve fieldset, numeric rows, hint styles
and Advanced disclosure are reused; only a scoped grid/gap rule is added. No
large cards, badges, illustration or new renderer. Concise text suffices, so the
optional conceptual illustrations are omitted.

Select height plus label is46.5px; measured inner width220px at the approximately
280px sidebar,294px at360px,485px at550px. Names fit, descriptions wrap naturally,
and the max-depth label wraps without overlapping its numeric field. The select
is labeled Strategy and describes itself through the selected explanatory text.
Native focus/Tab semantics remain; key propagation is stopped locally so the
operation list cannot intercept Home/End/arrows or collapse the inspector.
Keyboard focus and isolation were checked in the browser. Native OS popup option
navigation remains a manual acceptance check; automation's injected native
select keys did not establish that behavior across platforms.

The existing V-Carve operation tooltip now describes both intentions. English
and German catalogs are updated. Selected strategy changes descriptions and
depth label/help only; Advanced controls keep their existing semantics. Radius
cap, restricted Source inset and the unavailable paired-inlay action remain.

## Saved values and unchanged boundaries

The same ordinary `patch('fullMedialAxis', true/undefined)` is used. Default
Perimeter stays unchanged, old absent/false projects display Outline groove,
and true displays Variable-depth engraving. HTML option values are presentation
only; no serialized enum or project version is renamed. V-Carve remains outside
the existing UX4 template capture scope; supported templates are unchanged.

Actual browser switching, Undo/Redo, Save and reopen preserve depth−1mm and
step−0.2mm. Comparing real before/after Save downloads finds **only the existing
mode field changed in the operation**; all other authored operation values agree.
The saved Outline project reopens with Outline groove and the same numeric values.
Pure tests also cover legacy/default/true mapping, localized explanations, null
selection, ordinary-operation depth wording, history/JSON persistence and the
unchanged template support boundary.

No geometry, sampling, continuous-clearance, scheduling, generic path, CPS post,
preview resolver, API, model, serialization, diagnostic policy or export guard
code was changed. [Nine protected source hashes](vcarve/STRATEGY_UX_CORE_HASHES.json)
match both the pre-change capture and the earlier qualified hashes. Output
comparison below confirms exact programs, not merely visually similar previews.

`vcarve_target_capped`, `vcarve_scope` and `cps_post_warning` remain visible and
at warning severity; incompatible settings/refusals and stale export remain.
The selector deliberately does not replace the scope diagnostic. Its similar
scope wording could be consolidated in a separately reviewed presentation change;
no diagnostic is removed or downgraded here.

## Tests and numerical regression

- Frontend: **1249 passed,1 optional skip,137 files**; check0 errors/warnings,
  ESLint/Prettier and production build pass; unchanged component budgets and
  `git diff --check` pass. Existing chunk/static-dynamic import warnings remain.
- V-Carve qualification: **10 passed**, including decoded OpenSBP/preview parity,
  A1–A4, topology, cone/caps, continuous clearance and fail-closed inputs.
- Halfpipe: **15 passed**; CPS ShopBot guards: **7 passed**.
- [Actual downloaded output comparisons](vcarve/STRATEGY_UX_OUTPUT_COMPARISON.json):
  **byte-identical** before/after for both strategies. Outline remains130 moves/
  153 UI lines; variable remains5862 moves/5891 UI lines. File `splitlines()`
  counts are152/5890, excluding the UI's trailing empty line. No goldens updated.

Reproduction:

```sh
npm --prefix frontend test
npm --prefix frontend run check
npm --prefix frontend run lint
npm --prefix frontend run build
CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test vcarve_qualification -- --nocapture
CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps halfpipe --lib
CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime RUSTC_WRAPPER='' cargo test --offline -p ivac-cps --test shopbot_guards
```

## Actual visual evidence

macOS in-app browser, local frontend18781 / HTTP-CPS18792,1280×720 desktop.
These are software fixtures, never production machining recommendations.

- A/C: [Outline groove, Final depth and contextual description](ux/VCARVE-strategy-A-outline.png).
- B: [Variable-depth engraving](ux/VCARVE-strategy-B-medial.png).
- C: [Maximum carving depth and selected explanation](ux/VCARVE-strategy-C-depth-medial.png).
- D: [Expanded Advanced controls](ux/VCARVE-strategy-D-advanced.png).
- E: [Narrow Outline inspector](ux/VCARVE-strategy-E-narrow.png),
  [narrow variable-depth inspector](ux/VCARVE-strategy-E-narrow-medial.png).
- F: [Preserved warnings and inspector](ux/VCARVE-strategy-F-warnings.png).

## Remaining limitations and next action

All numerical/geometric/unsupported-setting limitations of the existing V-Carve
qualification remain: sampled medial paths, incomplete area clearing, strict
Perimeter island refusals, stock-top starts and restricted inset. No paired
inlays, flat-bottom clearing, physical accuracy or new controller qualification.
Final human review should confirm native select keyboard behavior and wording.
No commit/push/merge/release or next milestone started. **STOP at ivac-1b75.**
After acceptance, the recorded future CAM plan is Two-Sided Machining &
Registration; its scoped launch remains a separate decision.
