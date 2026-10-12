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
- Published foundation: `8271dd3bc892559fecbcaf3d80a64abff59aa299` (FS0–FS3.6). Latest publication: FS3.7/FS3.8 on `feature/shopbot-mvp`, explicitly authorized for commit/push on 2026-10-12; the publication commit is identified in branch history. Publication does not close human qualification gates or authorize FS4.
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

FS3.6 human review (ivac-vgu4) remains recorded separately. Engineering ivac-n23x is complete;
FS3.5 human gate ivac-eh1g closed on explicit user approval. FS4 remains
unstarted. User explicitly authorized committing and pushing the accumulated FS0–FS3.6
work on 2026-10-12. This publication does not close the FS3.6 human qualification
gate. The subsequent user request explicitly authorizes FS3.7 phases A–G.


## FS3.7 — source packs handoff (2026-10-12)

The explicitly requested [FS3.7 audit](feeds-speeds/FS3_7_AUDIT.md) and
[ADR-0005](adr/ADR-0005-source-packs-and-cutting-knowledge.md) guide one shared
JSON pack loader and pure multi-source resolver. Existing numerical engine rules
remain intact. Reviewed public packs use an explicit registry; imported packs
use Tauri app-data files or origin-local IndexedDB, never the HTTP transport.
Preferences retain IDs/versions and enablement. Legacy imports migrate only after
successful validated storage. Generic Settings and compact operation summaries
retain collapsed calculation context and detailed record provenance.

The application-owned material taxonomy supports explicit parent fallback and
exact preference; composite Sienci categories are deliberately unmapped. Original
values/classifications remain traceable beside normalized quantities. The resolver
prioritizes scoped local trial presets, ranks specificity and reconciles only
comparable authored same-mode records without averaging. Noncomparable contexts
remain ambiguous; disagreement is visible and never promotes evidence.

[License matrix](feeds-speeds/DATA_SOURCES.md): APPROVED_PUBLIC is only the pinned
six-row Sienci CC BY-SA subset. LOCAL_ONLY is the framework for user-entitled
original measurements; no production private pack is populated. FabCAM/Sorbonne
public records, Onsrud, Amana, DATRON and Sorotec require licensing review. No new
lab-data license is chosen. Source licenses remain separate from application GPL.

[Qualification](feeds-speeds/FS3_7_QUALIFICATION.md): 69 engine/data/pack tests plus
one doctest, 13 adapter tests, two native storage tests, eight ShopBot goldens and
four CPS integration tests pass. Frontend 1,074 pass / one optional skip;
Svelte/TypeScript zero errors/warnings, lint/format/Clippy/schema and production
WASM/frontend build pass. Real browser import/refusal/duplicate, two-source
proposal/explicit Apply, actual v4 save and missing-local-pack reopen/generation
pass. Local data bodies are absent from the saved project. No new desktop bundle,
Windows/Linux execution, physical trial or lab-qualified preset is claimed.

New knowledge saves use guarded version 4; v1/v2/v3 files and legacy dataset
requests remain supported. Missing local data never changes saved rates or blocks
generation. See [source-pack contract](feeds-speeds/SOURCE_PACKS.md).

FS3.7 human review ivac-tm7u remains recorded. The subsequent explicit user
request authorizes FS3.8 engineering; it does not authorize FS4.


## FS3.8 — local capture and validation handoff

Explicit user checkpoint request authorizes FS3.8. The pure core capture adapter
reads actual generation rates including manual edits, exact stable identities and
native depth/entry/engagement. Secondary Save as lab preset writes only private
Sorbonne knowledge using FS3.7 storage/resolution. Existing Tool/CuttingParameters,
recommendation mathematics and ShopBot posts remain unchanged.

Schema 2 adds optional authoring metadata and retained revision snapshots; schema 1
public references remain readable and unchanged. Draft is inactive, Tested requires
dated physical-use/success attestation, Validated requires later deliberate review
under sorbonne-local-validation-1. Unknown engagement cannot be qualified. Revisions
are immutable with stale-head refusal; deletion is confirmed and reversible.
Settings supports search/context/trial/history/notes/disable/restore/promotion and
explicit private backup/restore. Missing local packs still preserve saved values.

Read [qualification](feeds-speeds/FS3_8_QUALIFICATION.md),
[authoring guide](feeds-speeds/PRESET_AUTHORING.md),
[validation policy](feeds-speeds/LOCAL_VALIDATION_POLICY.md) and
[ADR-0006](adr/ADR-0006-local-preset-authoring-and-validation.md).
Software tests and browser captures do not constitute a physical trial. No new
macOS desktop bundle or Windows/Linux runtime qualification is claimed.
Software checks: 77 engine tests + doctest, 16 core adapter, 3 native storage,
1,087 frontend passes/1 optional skip and 8 ShopBot post regressions; check/lint/
Clippy/optimized build pass. Browser capture/manage/export/restore and identical
before/after project-operation checks pass.
STOP — human approval ivac-7yii is required before starting FS4.
