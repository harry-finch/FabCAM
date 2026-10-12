# Engineering log

## 2026-10-10 — M0 qualification

- Initial FabCAM folder contained the supplied skills pack, not a Git checkout.
  Cloned upstream history into `ivacam/`, preserving the original pack outside.
- Verified GitHub account with gh: harry-finch. Created its normal ivaCAM fork,
  configured origin/upstream and created `feature/shopbot-mvp` from upstream
  `9168104b4fc4e02e916740a54bd1e56b5019f662` (version 0.5.0).
- Applied github-upstream-workflow, ivacam-architecture, project-documentation
  and protomax-waterjet (architectural foresight only). Copied the supplied skill
  pack into the checkout for future sessions.
- Inspected upstream agent files, manifests, imports, models, pipeline/CAM,
  simulation, native posts/CPS runtime, frontend persistence/history/profile
  helpers, build scripts and CI. Recorded feature evidence and reuse decisions.
- ShopBot feasibility: existing post seams appear sufficient; final CPS/native
  decision deferred to M2. ProtoMAX assessment B: shared infrastructure reusable,
  explicit process/pierce/quality semantics absent in the current CPS IR.
- Ran existing Node/frontend checks. Results and environment failures are in
  TESTING_STRATEGY. Rust is unavailable; no claim of Rust CAM qualification.
  bd is unavailable; upstream issue records untouched.
- Created the persistent documentation baseline, including roadmap/approval
  policy. No application source, schema or machining behavior changed.
- Baseline commit `15b3d58667599af5a4e445d3cc52c0ae1b7d702a` pushed to the feature
  branch; separate handoff commit records its SHA in STATUS. HTTPS push used
  gh's credential helper transiently; no global Git config changed.
- M0 commit/push state is recorded in STATUS. No PR, default-branch merge, M1
  qualification, ShopBot implementation or waterjet implementation authorized.

STOP at M0; next milestone requires explicit human approval.


## 2026-10-10 — M1 software qualification

- User approved continuing after M0 and installing the tools. Applied
  cam-geometry, cam-validation and project-documentation; inspected existing
  imports, offsets, operation setup, preview and simulation rather than adding
  a new CAM subsystem.
- Installed pinned Rust/WASM tools, Node 24/pnpm 10, sccache and Beads. Repaired
  the pre-existing Homebrew Node after its dependency update. Baseline Rust
  suite with the restored environment: 1,136 pass. See TESTING_STRATEGY.
- Added all eight specified analytic shapes in SVG and DXF, and 11 qualification
  tests for import, profile compensation/Z passes, pocket preservation/coverage/
  allowance, drill depth/peck/retract, disconnected/open paths and determinism.
- Corrected endpoint-only arc bounds, inscribed island round-join clearance,
  and cascade/spiral terminal-strip coverage by capping ring stride to radius.
  Added two arc-bound unit tests and strengthened the existing island test.
  Reproductions, tolerances and limits are in M1_QUALIFICATION.
- Final Rust workspace: 1,149 pass; core without CPS: 1,060 pass. Headless build,
  Clippy, formatting, schema/version, release WASM, frontend check/test/build pass.
  Frontend: 1,044 pass, one optional locale skip. No machine/VCarve qualification.
- Beads installed and original 1,165 issue records imported. Closed only new
  issues ivac-h52u and ivac-n92a; appended their exported records without
  rewriting upstream issue history. Local runtime files excluded from Git.
- No schema, dependency or process-boundary changes; future ProtoMAX assessment
  remains B. More vertices/passes are possible; performance unmeasured.
- M1 work remains uncommitted. Master prompt requires explicit commit/push
  authorization after M0. No M2 implementation, PR or merge started.

STOP at M1; next milestone and Git publication require explicit approval.


## 2026-10-10 — M1 approval/publication and M2 implementation

- Human approved M1 commit/push and starting M2. Committed/pushed M1 as
  7737a2a5c8e3766b03869b8e6618e3e32c7e1f0f, origin/feature/shopbot-mvp.
- Applied ShopBot, validation, architecture and documentation skills. Compared
  existing CPS/native interfaces, recorded ADR-0001 and verified the emitted
  subset against official 2007 command reference and 2010 programming handbook.
  No third-party post source copied.
- Added authored bundled shopbot CPS post: metric XYZ, manual spindle/tool/zero
  prompts, arc linearization, explicit drill expansion, .sbp extension. Whole-IR
  preflight rejects unsupported motion/events/machine policies. Added generic
  optional metadata, no schema or generic CAM change. Browser CPS remains absent.
- Added six integration and three guard tests, five required full-text output
  goldens with analytic checks on the emitted motions. Manually reviewed numeric
  command forms, initialization/finalization, drill retracts and tool sequence.
  The 31,733-line pocket is a buffering/performance qualification limit.
- All-features suite 1,158 pass serial; core without CPS 1,060 pass. Clippy,
  format, schema/version, headless/WASM/frontend builds pass; frontend 1,044 pass,
  one optional skip. Verified CLI registry listing.
- Parallel global-cache count race recorded as ivac-htc8. Sandbox-interrupted
  WASM output exposed incomplete freshness detection, ivac-5g51; full rebuild
  repaired current packaging. Both remain explicit follow-ups, no scope creep.
- Closed M2 tracking issue ivac-732u after software qualification. Documentation
  and exported M2 issue records prepared for review. M2 is uncommitted; no PR,
  merge, M3 profile or physical validation started. ProtoMAX remains deferred.

STOP at M2; publication and M3 require explicit human approval.

## 2026-10-10 — M2 publication and M3 machine profile

- User approved M2 and M3; committed and pushed M2 as
  `047a75872ca7ea7d28077555c9859f92fa9a7411` on feature/shopbot-mvp.
- User supplied nominal gantry 120 × 60 cm, year 2011 and SB3.8; all other
  machine details remain unverified. Recorded provenance and converted XY to mm.
- Added a native version-1 machine snapshot and loading instructions using the
  existing import/profile UI. No production code, public schema or UI changes.
  Owned ShopBot CPS selection and manual policies preserved; no tooling presets.
- Zero clearance is an explicit unconfigured sentinel: frontend wire propagation
  and Rust CAM-to-post refusal tested. Unknown Z travel is zero and documented;
  omitted speeds/RPM and existing simulation defaults are not measured values.
- Actual import/save/reload path tested with desktop I/O/store doubles, including
  workspace profile round trip and CPS request. Both new integration tests pass;
  frontend type checking has zero errors/warnings. Physical qualification absent.
- Tracking: ivac-c5lt. M3 remains uncommitted pending approval; M4 not started.

STOP at M3; publication and M4 require explicit human approval.

## 2026-10-10 — M3 publication and runnable M4 software handoff

- User approved M3 publication and starting M4, requested explicit testing
  instructions and a fully compiled application. Published M3 at b8a073a.
- Installed Tauri CLI 2.12.1 locally under /private/tmp; built frontend and native
  release CPS-enabled macOS arm64 app 0.5.0. Repaired local bundle signing with
  an ad-hoc signature and verified it. Ready-to-launch copy in FabCAM/ivaCAM-M4.
- Prepared a self-contained square project and independently generated reference
  SBP, synthetic parameters clearly marked OFFLINE DEMO ONLY. Explicit svg layer
  source avoids also machining the selectable stock outline.
- Native launch/load/generate/settings/export check exposed toolbar .ngc label
  despite correct ShopBot output, falsely dirty hidden machine settings draft,
  and cosmetic 0.0.0 frontend release display. Fixed these bounded delivery bugs.
- Native UI export equals the 164-line reference byte-for-byte. One expected
  manual-spindle warning; no critical bounds warning on the final demo.
- Full frontend suite 1,046 pass/one skip; final adapter regression and type/lint
  checks pass. Release bundle ready, no source CAM or CPS changes.
- Prepared M4 ordered human validation protocol and evidence record. Preparation
  ivac-mwp7 closes at handoff; human software/physical evidence tracked in ivac-o4oy.
  M4 remains in progress; no SB3.8 Preview, air cut, machining or measurement.

Waiting for the user's explicit offline software test results. M4 preparation
remains uncommitted; no authorization to publish this milestone yet.

## 2026-10-10 — positive software acceptance

User replied “je valide !” to the offline software test handoff. Recorded positive
software acceptance without inventing individual observations or physical results.
Requested actual Z travel/jog/spindle data and operator-selected tool, sacrificial
material, feeds/plunge and clearance before preparing a dedicated machine file.
No application changes or rebuild; delivered 0.5.0 bundle remains current.
M4 controller/physical evidence remains pending in ivac-o4oy.

## 2026-10-12 — M4 spindle workflow correction

User confirmed successful r1 Preview and air cut, then reported SB3 PAUSE blocks
manual controls and spindle never starts. Inspected supplied VCarve programme and
lab macro archive as data only. Added opt-in reviewed spindle-1 relay/RPM mode
for preloaded tool 1, with no C9/zero/offset routines. Default manual M2 output
retained. See M4_SPINDLE_R2 for policy, scope, evidence and renewed human gates.

Software verification r2: CPS guards 5/5, actual CAM/golden integration 8/8, bundled metadata 1/1; formatting/diff clean. Release macOS build and local signature passed, native project load/generate/export verified. Exported XY/cutting/feed lines equal r1. Delivery BUILD.json identifies new executable and SBP; human revised gates pending.

## 2026-10-12 — r2 Preview and air-test acceptance

User supplied completed r2 Preview (XY 7..113, Z -5..+15, no offset) and then
confirmed “c’est un sans faute !” after the spindle-enabled air-test handoff.
Recorded overall acceptance without inventing measured RPM or air-origin data.
MDF cut, agreed tolerances/measurements and VCarve comparison remain pending.
No code, delivered programme or application changed; no commit/push performed.

## Human material-trial acceptance — 2026-10-12, r2

After the handoff explicitly requesting the real MDF trial with the same r2 SBP,
the user replied “c’est un deuxième sans faute, tout fonctionne parfaitement !”.
Record successful operator-reported material machining with no anomaly reported.
No measured dimensions, shoulder depth, instrument, pre-cut tolerance agreement
or equivalent VCarve comparison were supplied. M4-PART accepted by operator;
M4-MEASURE and M4-VCARVE remain pending. The application and SBP are unchanged;
no new implementation, milestone completion, commit or push follows this report.

## Operator target-conformity confirmation — 2026-10-12, r2

User states “tout est parfait, dimensions, profondeur cible, dégagement à 15mm
à la fin. l’usinage est propre. c’est nickel.” Record operator-confirmed conformity
to the nominal retained square 100 × 100 mm and 5 mm depth targets, clean machining
and final +15 mm clearance. No exact dimensional readings, measuring instrument
or quantified tolerance agreement were supplied; do not invent them or claim
a metrology certificate. M4-MEASURE has qualitative operator acceptance.
Equivalent VCarve comparison remains unreported and is the outstanding M4 gate.
No change to the tested application or SBP; no milestone commit/push or new track.

## VCarve reference received — 2026-10-12

Compared supplied M4-r2-carre-MDF_vcarve.sbp with tested r2. Nominal 100 mm square,
6 mm exterior compensation, final depth 5 mm and 18000 RPM agree. Reference is
translated (+1.246161,+0.409699 mm), uses -2.5/-5 passes, 4980/1794 mm/min feed/
plunge and safe/home Z 6/20 mm. Traversal opposite; CG versus linearized corners.
See M4_VCARVE_COMPARISON.md for hashes and evidence. Parameter alignment pending;
no defect inferred, no new cut or application change requested. M4 remains open.

## M4 qualification decision — 2026-10-12

Second VCarve export aligns square location/exterior D6 compensation and -3/-5
passes. Actual feed/plunge 5976/1992 differ by -0.4%; imposed safe/final Z 6/20,
opposite traversal, arc representation and XY parking documented and accepted
for semantic comparison. See M4_VCARVE_COMPARISON.md for source hashes/results.

Operator confirmed r2 Preview, spindle-enabled air cut, real MDF cut, target
dimensions/depth, clean finish and +15 final retract. M4 qualification completed
at this reported evidence level; no exact metrology readings/instrument or measured
RPM invented. ivac-o4oy closes. Explicit milestone commit/push approval still
required; no code/app/SBP changes or post-M4 governance/FS implementation performed.

## 2026-10-12 — M4 publication approval

User explicitly authorized commit/push (“oui tu peux push”) after the completed
physical qualification and VCarve comparison handoff. Publish the existing M4
changes on feature/shopbot-mvp, with exact tested r2 project/SBP and build manifest
retained as repository fixtures. No rebuild or modification of the delivered
app/SBP; no PR, merge, upstream synchronization or new implementation. FS0 and
the separate governance update remain subsequent work.
