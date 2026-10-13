# FS3.9.1 — Recommendation UX Simplification

Engineering handoff (ivac-hcea); human visual review ivac-anp0 pending. FS4 remains deferred/nonblocking.

## Existing UX weaknesses found

The inline FeedsSpeedsSection stacked context, repeated engagement, source counts,
a calculate action, another heading, six quantities, adaptation details and separate
Apply/Cancel locations. Mode required another click. Same-diameter derived Pocket
references could falsely display diameter→same diameter. Lab capture was adjacent to
the assistant, and its form initially selected Tested even though evidence was unconfirmed.

## UI changes implemented

Reuse existing Modal for a focused440px desktop recommendation with concise material /
operation / diameter / flutes context, compact aligned values and a coherent sticky
Cancel/Apply footer. Existing inspector inputs remain the effective values. No new
cards, icons or source-selection system. Existing modal focus trap/Escape retained;
closing returns focus to the recommendation trigger.

## Information removed or moved

Chipload, display-only cutting speed, requested nominal engagement, calculation
context, adaptation/unknown-limit explanations move into Why these values?. Nominal
engagement stays primary if it differs from suggested stepover. Source/evidence remain
visible; raw original source/provenance remain in a separate secondary disclosure.
No domain concepts merged. Display rounding never changes the supplied values.

## Recalculation behavior

Opening and changing mode auto-calculates apply:false. Calculation is transient and
creates no history command. Each request owns a revision and context key; late results,
errors/finalizers, cancellation, reopen and component disposal cannot publish old data.
Apply remains an explicit apply:true call and the existing undoable cutting command.
Full-width confirmation is local until Apply; cancellation leaves operation untouched.
Loading is announced; stale or unavailable previews disable Apply. Retry remains on error.

## Recommendation-quality explanation fixes

Compare raw diameters, not formatted strings. Identical diameters never show a false
adapted-from arrow. Slot-to-pocket transfers have their own metadata-backed explanation;
unknown metadata stays neutral. Resolver quality/evidence/ranking remain unchanged.

## Warning presentation

One visible reminder covers unknown limits/assumptions; extrapolation and disagreement
retain prominent caution. All diagnostics remain accessible. Invalid request issues
stay visible with a blocking treatment and disabled Apply. No-reference states use a
short summary plus actual scope refusals under Why no recommendation?.

## Lab preset workflow decision

Capture action is now Save current values as local preset… in the operation's effective
parameters, outside the recommendation dialog. Form starts Draft. Drafts retain existing
FS3.8 semantics: stored but not eligible recommendations. Tested remains a deliberate
choice with existing physical-test/no-critical-failure confirmations and evidence checks.
No new reusable-unverified evidence category or preset system introduced. No preset
saved or physical evidence promoted during visual qualification.

## Screenshots

Before: [existing inline panel](../feeds-speeds/FS3_9-final.png).
After: [adapted Ø6](FS3_9_1-adapted.png), [same Ø6.35](FS3_9_1-same-diameter.png),
[explanation](FS3_9_1-explanation.png), [no reference](FS3_9_1-no-match.png),
[blocking invalid RPM limits](FS3_9_1-blocking.png), [small desktop](FS3_9_1-small.png),
[lab evidence form](FS3_9_1-lab-evidence.png).

## Regression tests

Frontend1117 passes/1 optional skip (121 files); check0 errors/warnings; lint/build pass.
Five new tests cover adaptation facts, display-only formatting, engagement distinction,
out-of-order three-mode responses and cancellation/reopen ownership. Existing tests
cover Apply/history/manual overrides/save-reopen/lab safeguards/modal keyboard behavior.
FS98 tests +1 doctest passed again, including six exact references unchanged.
No production Rust, schema, pack, formula, limits or post changes in FS3.9.1.

## Actual browser workflow checks

Prudent/Normal/Rapid Ø6 feeds1530/2040/2551, automatic unapplied preview. Cancel
leaves10000rpm/1000feed/500plunge and Undo disabled. Apply creates the existing
Apply cutting parameters history command. Rough17000/1530/765, finish10000/1000/500
retained independently. Manual roughfeed1200, Normal preview2040, Escape, Save and
reopen:1200 remains Modified in a v4 file, no automatic recalculation. Tab/Shift-Tab
wrap inside Modal; Escape/Cancel restore trigger focus. No browser console errors.
At1024×600 all primary values and footer fit. Crossing into the existing800px mobile
layout disposes the inspector and discards its transient dialog; viewport restored.
Blocking screenshot uses synthetic contradictory RPM limits, not real ShopBot limits.

Profile without full-width confirmation refuses; checking it recalculates1530feed.
Cancel/reopen restores unchecked and no-reference state, proving confirmation is transient.

## Files changed

FeedsSpeedsSection, RecommendationBasis, LabPresetDialog; transient recommendation-preview
helper/tests; en/de translations/generated keys and unit allowlist. Project models,
source packs and storage format unchanged. Existing uncommitted CAM work preserved.

## Documentation

ROADMAP, STATUS, DEVLOG, DESIGN_DIRECTION and this handoff record updated.

## Remaining issues

FS3.9 sensitivity/selection discontinuities and ShopBot RPM-transition issue ivac-2zll
remain separate. Mobile-layout switching can dispose/recreate the operation inspector;
transient previews are intentionally discarded. This milestone qualifies desktop workflow.
No commit/push or next milestone started. **STOP at human visual approval.**

## Inspector / advanced-entry continuation — 2026-10-13

Authorized focused continuation, ivac-m0qf, same human gate ivac-anp0. No new CAM milestone.

### Existing UX inconsistencies identified

The default inspector duplicated source counts, quality, derivation and provenance
already available in the recommendation Modal. Separate actions wrapped into unrelated
rows. Five identical suggested badges obscured meaningful manual exceptions. Pocket's
second Advanced depth and entry disclosure actually contained only low-level overlap.
Advanced numeric columns used different widths/spacing from the compact inspector.

### Cutting Parameters changes / removed redundant information

Removed the default provenance/quality/disclosure block. Effective inputs and safety /
stale/missing-source warnings remain. Technical data is still in the recommendation
Modal; a saved-provenance disclosure remains there when current recalculation cannot
produce a proposal. No saved metadata deleted.

### Recommendation / preset action layout

Save as preset… and Review recommendation… share one secondary action row immediately
below finishing controls. At the existing narrow width both are36px high on the same
row; review text wraps within its button. Below180px container width they stack without
overflow. Capture still reads actual effective values; review is unapplied and Cancel
preserves them. No icons or primary CTA added to the inspector.

### Suggested indicator policy

All relevant fields recommended: one Applied recommendation indicator. All manual:
one manual indicator. Mixed recommended/manual: shared baseline and only manual
exceptions marked. Unknown/legacy origins prevent a blanket recommended claim; retain
individual meaningful indicators. Finishing origins are considered independently.
Four new unit tests cover these classifications; existing edit/update logic continues
to mark step/overlap overrides manual without any changes to persistence semantics.

### Advanced depth and entry changes / shared components reused

Reuse inspector-disclosure, row, num-cell, control-height/unit tokens and the existing
EntrySupport component. Targeted depth-entry numeric alignment/tabular numerals; concise
metadata fonts and normal disclosure spacing. Pocket overlap uses the same disclosure
style and a precise name; low-level fractional/inheritance/reset control is retained,
while primary stepover remains a percentage. No labels or conversions merged in the model.
Native disclosure expansion state is preserved; no auto-expansion introduced.

### Safety and compatibility preservation

EntrySupport's existing blocking message is presented outside the collapsed advanced
section and omitted inside to avoid duplicates. Ramp informational/resolution detail
stays contextual. Direct/Ramp/Helix handlers, angle/radius values, defaults, authored
strategy and existing switch behavior unchanged. Helix warning remains visible even
with the section closed. No post/geometry/compatibility policy edits or silent fallback.

### Tests and regression results

Frontend1121 passes/1 optional skip (122 files), check0 errors/warnings, lint/build pass.
27 CPS-enabled shopbot_m2_post regressions pass, including UX1 Ramp/Helix, islands,
requested/final-output parity and CAM3 tabs/leads/directions. No expectations changed.
Actual browser: manual1200 reopen, existing Modal review/Apply1530, grouped status,
Draft capture, Ramp3° and Helix3°/4.5mm, closed Helix warning and1024×600 view checked.
The original FS3.9.1 Apply/cancel/manual/save-reopen/keyboard qualification remains valid;
engine math/schema/source packs and rate/entry change handlers were not edited.

### Before / after screenshots

- [Before default inspector](FS3_9_1-inspector-before.png).
- [Normal/collapsed](FS3_9_1-inspector-normal.png).
- [After explicit Apply](FS3_9_1-inspector-applied.png).
- [Manual override](FS3_9_1-inspector-manual.png).
- [Ramp expanded](FS3_9_1-entry-ramp.png).
- [Helix expanded](FS3_9_1-entry-helix.png).
- [Helix warning with disclosure closed](FS3_9_1-entry-collapsed-warning.png).
- [Small desktop1024×600](FS3_9_1-inspector-small.png).

### Documentation updated / remaining UX issues

STATUS, DEVLOG, DESIGN_DIRECTION and this same FS3.9.1 handoff updated; roadmap order
unchanged. Long labels wrap within the narrow inspector and the full advanced section
requires normal panel scrolling. Legacy strategy-switch behavior remains as before;
no new inactive-value storage added. Existing independent ShopBot RPM-transition issue
ivac-2zll and FS3.9 selection discontinuities remain outside this UI task.
FS4 deferred/nonblocking. No physical claim, local preset saved, commit or push.
**STOP — human review ivac-anp0 pending.**
