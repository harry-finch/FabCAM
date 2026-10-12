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
- M4 baseline/publication: `e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1` on `feature/shopbot-mvp`; HEAD and local origin tracking ref match.
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

This update is documentation only; the user authorized its local commit; no FS0 audit,
implementation, algorithm, CAM behavior, schema or application change.
No feeds-speeds skill exists in the inspected repo-local skill catalog; none
created. Governance tracking: ivac-x4bi.

STOP for user approval. Exact next step after explicit authorization: start
**FS0 architecture and data-model qualification** from the M4 baseline, using
the architecture and documentation skills. Audit serialized parameter ownership
and all consumers, then propose boundary/models, preset precedence/fallback,
reproducibility/migration and a test strategy. Implement no recommendation
algorithm during FS0. Local commit authorized by the user; push still requires explicit
authorization under ROADMAP Git policy.
