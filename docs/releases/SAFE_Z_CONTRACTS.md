# SAFE-Z — approved common-frame contracts

Current state — 2026-10-18: implemented bounded software evidence consolidated on
pipeline61; [current matrix and limits](FINAL_CONSOLIDATION_2026_10_18.md). Human review ivac-l6ouh OPEN;
TEST1/SAFE-Z overall remain open pending scoped acceptance. No commit/publication.
The contracts below retain their original numerical budgets and scope. Historical
proposal/reproduction text is context, not an assertion that implementation is absent.

## Established coordinate contract

`pipeline/work_origin.rs` subtracts authored XY work origin from sources, stock and
fixtures before CAM. Z is unchanged; nonzero numeric work-origin Z is refused.
`setup_resolver.rs` forwards start/depth/fast_move_z without adding stock top.
`StockConfig` defines stock Z as `[top_z_mm-thickness_mm, top_z_mm]` and explicitly
documents the spoilboard convention. These are absolute coordinates in the same
program work frame. A stock-top shift is not a numeric work-origin translation.
The saved authoring project must remain unchanged. Controller zero establishment and
unknown initial tool position still require operator verification.

## Reproduction on pipeline58 (not a physical test)

A6mm flat cutter profiles a20x20 square within100x100x18mm stock. `top_z_mm=18`,
`start_depth=18`, final17, rapid5, work-origin Z0. Generation succeeds and emits
`JZ,5.0000`, then `J2,20.0000,17.0000`; the declared stock occupies Z0..18.
The final known-position rapid goes from Z17 to5 inside stock. This is a confirmed
unsafe clearance, not a toolpath-depth convention guess.

Default work-area checking warns about positive-Z **cuts**; it does not establish
rapid clearance. With work-area Z unset (0, existing unchecked-envelope convention),
only the generic CPS operator warning remains. Thus another warning/default UI
setting is not an adequate clearance guard. Correcting rapid to23 avoids this
specific conflict but does not qualify the whole positive-Z workflow or initial motion.
Negative clearance with top0 is already refused by ShopBot; nonzero numeric origin Z
is also refused. Preserve both guards. Probe results are observations, not passing
safety tests. Initial XY is unknown; the probe does not claim to reconstruct it.

## Recommended bounded implementation for approval

Add shared milling preflight validation using resolved common-frame coordinates,
not a ShopBot-specific patch in geometry. For ordinary air-clearance transitions
without a qualified cleared-region proof, require finite clearance strictly above
known stock top (and relevant known fixture bounds); reject inconsistent configuration
before returning or streaming executable output. Never auto-raise stored values.
Keep diagnostics actionable: operation, values, frame and correction needed.

Check actual relevant emitted rapid segments as well as configuration. Include
inter-operation transitions, tool-change/park paths and initial-position uncertainty.
A universal `all rapid Z > stock top` assertion is wrong: a qualified retract can
start inside a previously cut channel. Permit that only with the operation-specific
cleared-channel contract; distinguish retract from an unproved lateral/descending
rapid through uncut material. Do not infer previous removal from a pretty preview.
If this proof exceeds the bounded V1 scope, refuse the ambiguous configuration and
report the remaining limit for approval; do not implement a new stock-removal engine.

Generation/streaming must fail closed for the reproduced known-stock conflict,
independent of UI warning preferences. HTTP, native, WASM and CPS must return the
same failed operation state; export must not reuse older output. Unknown stock,
unknown fixture dimensions or unknown initial position are distinct **unqualified**
conditions; retain explicit operator prerequisites and existing refusal policies,
never relabel them safe. Any stronger global prohibition requires a separately
reviewed compatibility decision. Do not add numeric Z translation or relax two-sided,
raw included command, spindle or unsupported strategy guards.

## Required cases and oracle

| ID | Case | Expected acceptance evidence |
| --- | --- | --- |
| Z01 | top0, rapid+5; profile/pocket/drill | Existing supported output retained; decoded safe lateral transitions and qualified retract semantics. |
| Z02 | Same job with XY origin translation | Stock/source/fixture/output share translated XY; Z unchanged; source project unchanged. |
| Z03 | Numeric origin Z nonzero or nonfinite | Existing explicit refusal, no usable generated/streamed/exported output. |
| Z04 | top18, start18, final17, rapid5; known and unknown work area | Historical red case, now regression-tested: generation refusal independent of critical-warning preference. |
| Z05 | Same stock rapid23; negative/zero/equal-top/nonfinite rapid variants | Positive control only after all relevant paths are checked; equality/no margin must not silently qualify. Existing negative-clearance refusal retained. |
| Z06 | Known fixture above stock, unknown fixture/holder, lateral crossings | Cutter/holder swept envelope and bounds in common frame; collision fails under approved guard, unknown remains explicitly unqualified. No whole-machine collision promise. |
| Z07 | Two operations, distinct clearances, tool change, park, initial axes unknown | Check transition semantics and decoded path, not operation-local end points alone; unknown initial segment remains an operator prerequisite, never invented evidence. |
| Z08 | Edit stock/top/clearance after generation; Undo/Redo, cancelled/failed regeneration, cache hit; streaming | Old output cannot export; new refusal cannot yield success/partial usable output; same contracts across adapters. |

Oracle: independent analytic segment-versus-box/cylinder swept intersections using
known cutter/holder envelope; include the interior of segments, not just endpoint
checks. Fixed analytic witnesses accompany generated cases. Coordinates inmm;
numerical budget1e-8mm for analytic bounded fixtures, OpenSBP rounding0.00005mm/axis
plus arithmetic budget accounted conservatively. No physical clearance margin is
chosen by this numerical tolerance. A lower-bound clearance within the uncertainty
band is unresolved/refused, not safe. Separate guaranteed air paths from qualified
cut-channel retracts and unknown initial poses.

Release gate: fix and independent regressions for Z04, verify Z01–Z08 within agreed
supported scope, review UX diagnostics/export and native/WASM parity, then human
software acceptance. Platform and actual setup/ShopBot trials remain required.

**HARD STOP — review the current consolidation before the next checkpoint.**
