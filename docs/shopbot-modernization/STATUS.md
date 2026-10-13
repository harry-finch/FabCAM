# Status — 2026-10-14

## Repository transition — 2026-10-14

User authorized integrating `feature/shopbot-mvp` into the default branch `main`
and renaming the GitHub fork to `harry-finch/FabCAM`. The integration is a
fast-forward to c9d3f3b; the feature branch and upstream relationship are preserved.
Origin now uses `https://github.com/harry-finch/FabCAM.git`. Application/package
names and the local checkout directory remain unchanged. Existing qualification
gates and the CPS regression follow-up ivac-13go retain their recorded status.

## Publication authorization — 2026-10-13

User explicitly authorized committing and pushing all current repository changes
on `feature/shopbot-mvp`. This publication checkpoint includes the CAM/UX/FS
software and qualification records below. Pending human/native/physical review
gates retain their recorded status; publication does not qualify those gates.

## Current direction and next action

**UX2.2 — Application Shell & Welcome Experience**: Atelier implemented locally
(ivac-j6ba), following explicit user approval (ivac-dwdd closed). Welcome,
New/Close, shared Open/Import, project name/save state, active machine and
selected-operation tool are integrated. Lifecycle tests and browser review passed;
[qualification and screenshots](ux/UX2_2_QUALIFICATION.md). Next: human review and
native desktop qualification **ivac-ein1**. No native bundle rebuilt.
Final polish continuation **ivac-8u1p** complete: literal machine limitations
stay visible, Layers owns the primary import, generation messaging is contextual.
Frontend 1145 pass/1skip; [seven review captures](ux/UX2_2_POLISH.md), including
critical export rejection and edit → Save → stale → reopen. Await human approval
under ivac-ein1. Existing CAM4/FS gates remain independent; CAD1 remains separate.

**FS3.9.1 — Recommendation UX Simplification** engineering qualified (ivac-hcea);
human visual review **ivac-anp0** is next. [Handoff and screenshots](ux/FS3_9_1_QUALIFICATION.md).
Inspector/advanced-entry continuation also qualified (ivac-m0qf): no default provenance,
shared actions/origin indicator and consistent disclosure rows; blocking entry warnings
remain visible while closed. Frontend1121 passes/1skip and27 ShopBot regressions pass.
Compact existing Modal, automatic unapplied mode preview, guarded asynchronous results,
explicit Apply, local capture separated and Draft by default. Frontend1117 pass/1skip,
engine98 +1doctest; check/lint/build pass. No FS mathematics, source priority, storage
or ShopBot post changes. FS4 remains deferred/nonblocking; CAM ordering unchanged.


**FS3.9 — Approximate Matching & Coverage Expansion** is explicitly authorized
as a targeted exception to the FS freeze (ivac-02in). Software qualification
is complete; human gate **ivac-t0fk** is next. See [handoff](feeds-speeds/FS3_9_QUALIFICATION.md), [policy](feeds-speeds/FS3_9_MATCHING.md) and
[coverage](feeds-speeds/FS3_9_COVERAGE.md). FS4 remains deferred/nonblocking.
Independent known limitation: ShopBot CPS rejects a rough/finish RPM transition
(ivac-2zll); see the FS3.9 handoff for the conditional export check.
[Approved sensitivity audit](feeds-speeds/FS3_9_REVIEW.md) complete (ivac-v6pf):11997 samples,98 FS tests +1 doctest. Selection/interpolation jumps quantified; production smoothing requires a separate policy decision.

Milling software remains active. CAM3 continuation approved (ivac-wh2m closed).
**CAM4 A/B — Remaining Material Analysis & 2D Preview** (ivac-qk8y) is software
qualified within the explicit per-pocket/depth scope in
[the handoff](CAM4_QUALIFICATION.md). Human gate **ivac-tvox** is next.
Amber residual rings, area/depth metrics, toggle and detail use final linear output;
stale/unsupported contexts are explicit. CAD1/Phase C corner relief remains
separate scope/approval; no Rest Machining or physical validation.
User review follow-up fixes WASM checkpoint BigInt scrubbing, warning-chip alignment
and adds transient 3D Toolpaths visibility. Existing Spiral choice is clarified;
FS reference coverage unchanged in that earlier follow-up; targeted FS3.9 now adds derived coverage. [Evidence](ux/CAM4_REVIEW_FOLLOWUP.md).

Stock/WCS/imported-geometry dragging and snapping remain deferred (ivac-t68s).
UX2 numeric Z and two-sided CPS limits remain (ivac-u1ko / ivac-jtkj).

FS4 — Physical Feeds & Speeds Validation
- **Status: DEFERRED** (ivac-mwry).
- **Reason: requires physical lab / ShopBot access**, tools and materials currently
  unavailable.
- **Blocking further software development: NO**. Neither failure nor blocker.
- Resume later with existing FS3.8 capture/storage/resolver/history/validation
  workflow; no Feeds & Speeds architecture redesign required.

FS architecture through FS3.8 is sufficient for continued CAM work. Freeze FS
expansion: no new manufacturer imports, Sorbonne tuning, mathematics, physical
trials or FS UX redesign unless a separately approved feature needs integration.
External values remain ReferenceData; Sorbonne Tested/Validated require real cuts.
WJ/ProtoMAX and VINYL remain deferred. Historical FS human reviews remain recorded,
not automatically closed by track redirection or dependencies of milling software.

Future FabCAM/FabSBP ownership and unnumbered planning groups are recorded in
[the workflow plan](FABCAM_FABSBP_WORKFLOW.md) and ADR-0009. Documentation only:
FabCAM declares requirements; future FabSBP establishes physical state. No implementation in that documentation checkpoint; no FabSBP development authorization.

UX2.2 is now the selected next milestone; its first source/design checkpoint is
complete. [Specification](ux/UX2_2_PLAN.md), [audit](ux/UX2_2_AUDIT.md), human
design/scope gate **ivac-dwdd**. This selection does not rerun CAM3 or silently
close the pending CAM4/FS reviews. Production implementation awaits the concrete
design/lifecycle decision; no persistence/post/FS behavior changed.

## Current browser review session

User test URL `http://127.0.0.1:18767/?api=wasm` explicitly forces the standalone
WASM engine, which has no CPS support in the current package. For ShopBot review,
use `http://127.0.0.1:18767/?api=http%3A%2F%2F127.0.0.1%3A18772` with the current
CPS-enabled native server running on 127.0.0.1:18772. CORS allows the 18767 origin.
The server is deliberately left running for human testing (ivac-t79u); binary is
in /private/tmp/fabcam-cps-runtime, outside root target/debug. This is a local review
session, not a service installation or new native desktop bundle. Save/reopen your
own project when switching page URLs; no user project file was rewritten.

## Verified source and delivery

- Checkout: /Users/mugen/Documents/01_Projets/FabCAM/ivacam.
- Branch: main; origin harry-finch/FabCAM, upstream aalarchiv/ivacam.
- Previous committed/pushed software: **96e41ec4e8a3b593579079cbb6c051e070cd03ea**
  (FS3.7/FS3.8), explicitly published by user authorization on 2026-10-12.
- M4 baseline: e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1; M0–M4 names retained.
- Current historical native delivery: ../ivaCAM-M4-r2/ivaCAM.app. No FS3.8 native
  bundle or physical trial is claimed. target/debug cleaned by prior user request;
  release/WASM packages retained.
- M4 operator-reported Preview/air-cut/MDF exterior-square and VCarve comparison
  evidence remains in [M4 validation](M4_VALIDATION.md) and
  [comparison](M4_VCARVE_COMPARISON.md). Exact instrument/readings/tolerance and
  measured RPM remain unreported; this is not blanket milling qualification.

## Current evidence and limits

FS3.9: 94 FS tests +1 doctest,17 core FS adapter tests,1112 frontend passes/1 optional
skip; check/lint/fmt/Clippy/schema-check and optimized WASM/frontend build pass.
Original six public cutting rows numerically unchanged; no physical evidence
promotion, additional manufacturer data or private Sorbonne population.
[Qualification](feeds-speeds/FS3_9_QUALIFICATION.md).


CAM4 A/B: core1,062; M1 11; ShopBot27; snapshots4 pass. Frontend1,112 passes /
1 optional skip in120 files; check/lint/fmt/Clippy/schema-check and complete optimized
WASM/frontend build pass. Buffered response/type addition is compatible; project
serialization and machining output unchanged. Actual browser four-corner overlay,
strategy stale suppression even after Save, unsupported reason and Save/reopen
checked. New generation resets visibility. No native desktop bundle/physical trial.

Earlier CAM3 holding-tab and UX2.1 fixture/history evidence remains in their records.
A/B covers metric Cascade Pocket, one Endmill, straight closed targets, linear
returned execution moves and qualified Z0 stock depth. Tabs/allowance, curved or
analytic-arc targets/paths and broader contexts are Not analyzed, with reasons.
No cumulative job/physical stock-removal verdict.

Helix, Ramp+tabs/leads, short-path vertical fallback and ShopBot G55–G59 retain
explicit rejection. Numeric XY origin is fixed (ivac-vj11); Z refuses rather than
being ignored. Genuine two-sided CPS/translated registration refuses. Fixtures
warn when holder geometry is missing; no physical clearance/automatic avoidance
claim. FS4 does not block this software work.

[Milling audit](MILLING_CAPABILITY_AUDIT.md) remains the dated feature inventory.
Dogbones/T-bones, dedicated spoilboard surfacing and named operation templates are
future gaps; fixture creation is now available under Stock.
Known ivac-htc8 global pocket-cache parallel-test race retains serial testing;
ivac-5g51 partial WASM freshness guard limits retain complete package verification.

Redirection, UX1, CAM2/UX2, UX2.1, future workflow documentation, CAM3, CAM4 A/B and FS3.9 changes are uncommitted; publication requires authorization.
