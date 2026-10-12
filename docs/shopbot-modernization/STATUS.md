# Status — 2026-10-12

M4 qualification completed at the operator-reported evidence level. r2 Preview,
spindle-enabled air cut and real MDF machining passed; operator confirms target
dimensions/depth, clean finish and final +15 mm clearance. The second
[VCarve reference comparison](M4_VCARVE_COMPARISON.md) passes geometry, exterior
compensation and -3/-5 pass schedule. Residual differences: -0.4% feeds, VCarve
safe/final Z 6/20, opposite traversal, arcs and XY parking. Exact measurement
readings/instrument/tolerances and measured RPM remain unreported.
M3 approved and pushed. On 2026-10-12 the user authorized the M4 commit/push.

## Git and delivery

- Checkout: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
- Fork/origin: `https://github.com/harry-finch/ivacam.git`.
- Upstream: `https://github.com/aalarchiv/ivacam.git`.
- Branch: `feature/shopbot-mvp`.
- M4 source base: `b8a073a` (approved M3); M2 is `047a758`.
- M4 baseline/publication: `e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1` on `feature/shopbot-mvp`.
- Latest published source: `f6d17dd89be9229c243709d7c2df06dc49df45fa` (governance and feeds-speeds skill); pushed on explicit user instruction 2026-10-12. HEAD and origin tracking ref match; FS0–FS3.6 work is uncommitted.
- No PR, merge or upstream synchronization is included in M4 publication.
- Current ready-to-launch macOS arm64 app: `/Users/mugen/Documents/01_Projets/FabCAM/ivaCAM-M4-r2/ivaCAM.app`.
- Current machine file: `/Users/mugen/Documents/01_Projets/FabCAM/ivaCAM-M4-r2/M4-r2-carre-MDF.sbp`.
- Older delivery is retained unchanged; r2 BUILD.json identifies the current baseline.
- Version 0.5.0; delivery was built from M3 plus the M4 changes now committed. Delivery `BUILD.json`
  records executable/project/SBP hashes. Local ad-hoc signature verifies; not notarized.

## Verified software result

Release Tauri bundle includes native CAM and owned ShopBot CPS. Node 24.21.0,
Rust 1.88.0, Tauri CLI 2.12.1. Native app launched, project loaded/regenerated,
settings displayed correctly and export picker offered `.sbp`. The actual UI
export matches the 164-line reference byte-for-byte. Only expected manual-spindle
CPS warning, no work-area/stock warning for the final square-only demo.

Three small delivery fixes: toolbar label uses post-declared extension;
optional checkbox defaults no longer dirty the hidden Machine draft and block
refresh after loading; build version falls back to the Tauri release version
when the private frontend package has no version. No CAM algorithm changes. r2 subsequently adds the opt-in lab spindle policy
recorded in M4_SPINDLE_R2.md; 14 targeted Rust checks and rebuilt native export pass.

Frontend 108 files: **1,046 pass, one optional skip**. Final demo-adapter check
also passes with selectable stock outline present and explicit svg-layer source.
Svelte/TypeScript zero errors/warnings; targeted ESLint, formatting and Git
whitespace checks pass. Release frontend and native desktop builds pass.
M2 broad Rust baseline remains 1,158 serial passes, core without CPS 1,060;
M3 clearance-refusal integration passes. Existing cache race ivac-htc8 and
partial WASM freshness guard ivac-5g51 remain open. No Windows/Linux run.

## Current planning and next action

Post-M4 governance now records FS/UX/CAD/WJ/VINYL named tracks. Priority:
FS foundation → priority milling UX/CAM improvements → WJ → VINYL.
The [FS contract](feeds-speeds/README.md) and
[ADR-0002](adr/ADR-0002-feeds-speeds-boundary.md) record a milling-only,
independent recommendation engine, CAM adapter and explicitly persisted applied
parameters. Aggressiveness is separate from evidence; manual edits remain
authoritative. Post-processors never invoke the recommendation engine.

The user authorized FS3.5 on 2026-10-12 to clean up FS3 UX/domain ownership.
[FS3.5 qualification](feeds-speeds/FS3_5_QUALIFICATION.md) and
[ADR-0004](adr/ADR-0004-manual-operation-cutting-and-source-settings.md) record
job material in Stock, explicit operation cutting values with optional proposal,
focused Suggest/Review, global source Settings, physical tool/fallback disclosure
and available-tools-first Machine tooling. New portable cutting saves use guarded
kind/version 3; legacy v1 and FS3 v2 load unchanged. No math or ShopBot post changes.

Frontend **1,065 pass, one optional skip**; ten core adapter tests and eight
ShopBot goldens pass. Svelte/TypeScript zero errors/warnings, native server/Tauri
check, Clippy, schema/format/lint guards and release WASM/frontend build pass.
Real browser Apply/manual/DOC-stale/save/reopen/source-disable-persistence passes;
actual v3 saved fixture and screenshots are retained. No new desktop bundle or
physical trial. FS3's broader serial Rust baseline remains 1,218 passes.

The six Sienci references remain narrow LongMill full-slot references, with no
ShopBot preset, Pocket transfer, material equivalence, invented limit or local
Tested/Validated evidence. Manual values and generation are independent of sources.

The user explicitly approved FS3.5 functional behavior and authorized **FS3.6**
on 2026-10-12. [FS3.6 visual qualification](ux/FS3_6_QUALIFICATION.md) records
shared compact field grids, quieter section/header hierarchy, compact stale state
and Apply emphasis. [Persistent design direction](ux/DESIGN_DIRECTION.md) and
the repo-local cam-ux-ui skill now guide future desktop UI work.

FS3.6 frontend: **1,065 pass, one optional skip**; Svelte/TypeScript zero
errors/warnings; targeted lint/format, whitespace and production build pass.
Real browser Profile/Pocket, 240/360 px side panels, mixed manual/suggested states,
proposal/provenance, long names, large values and expanded sections qualified.
The initial visual handoff kept 437 protected files unchanged. The subsequent
user-requested review revision adds transient native rejection explanations
without changing matching rules/math, data, placement, persistence or posts.
Stock now shows drawing/WCS corner readouts; disclosures and origin vignettes
were refined. Revision: **1,068 frontend pass / one skip**, 50 engine/data tests
plus doctest, 11 adapter tests, eight ShopBot goldens and five CPS guards pass;
check/lint/format/Clippy/schema and rebuilt WASM/frontend pass. No new native
bundle or physical trial.

Focused recommendation-panel review (ivac-rncu) is ready: default material,
engagement, mode and compact source; known inputs moved to closed Calculation
context; clear Calculate recommendation/Recalculate naming. Explicit Apply is
preserved; no engine/domain/persistence changes. Frontend 1,068 pass / one skip,
check/lint/build and browser disclosure/240 px/kept-manual-feed verification pass.
See the focused-review addendum and captures in FS3.6 qualification.

STOP at human **FS3.6 approval** (ivac-vgu4). Engineering ivac-n23x is complete;
FS3.5 human gate ivac-eh1g closed on explicit user approval. FS4 remains
unstarted. User explicitly authorized committing and pushing the accumulated FS0–FS3.6
work on 2026-10-12. This publication does not close the FS3.6 human qualification
gate. Review the qualification screenshots; visual approval is the next action.
