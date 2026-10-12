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

## 2026-10-12 — Post-M4 governance and Feeds & Speeds planning

- Verified clean initial feature/shopbot-mvp checkout, M4 HEAD and local origin
  tracking ref both `e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`; no network
  fetch or upstream sync performed. Read M4 qualification/spindle/reference
  records; retain operator-reported evidence limits and unchanged artifacts.
- Used local architecture/documentation skills; no feeds-speeds skill found or
  created. Read existing model entry points only to ground future boundaries;
  the FS0 ownership/consumer audit has not started.
- Updated mirrored AGENTS/CLAUDE fork governance and roadmap: retain completed
  M0–M4, use FS/UX/CAD/WJ/VINYL tracks, FS foundation before priority milling
  improvements, then WJ and lower-priority VINYL; CAD scheduling remains open.
- Recorded FS0–FS5 scope, exclusions, acceptance and approval gates; one useful
  FS planning contract and proposed ADR-0002 capture independent milling engine,
  adapter, explicit persisted application, manual authority, configurable
  provenance-bearing data and separate aggressiveness/evidence.
- Updated architecture/upstream/deferred-process navigation and current STATUS.
  Documentation only: no code, CAM behavior, algorithms, schema, skill, fixture,
  application or SBP changes. No FS0 implementation or audit, no FS5 work.
- Tracking ivac-x4bi. Documentation verification: whitespace, relative Markdown
  links and changed-file scope. No code tests/build needed for this change.
  Stop for user review; no commit/push or subsequent milestone start authorized.

## 2026-10-12 — Governance commit approval

User authorized committing the documentation/governance changes. Commit locally
on feature/shopbot-mvp; no push or FS0 start authorized. Documentation whitespace
and changed-file scope verified before commit.


## 2026-10-12 — FS0 architecture and data-model qualification

User explicitly requested push of the existing last commit, then FS0. Push
published governance plus skill through f6d17dd on feature/shopbot-mvp. Read
feeds-speeds first, persistent project context, architecture/validation/docs
skills and their references. No upstream synchronization or new commit performed.

Audited upstream-derived tool/machine/stock/operation models, both frontend-file
and Rust wire persistence, editors, setup/header/dual-tool resolution, CPS IR
metadata, native/ShopBot posts and simulation seams. Found no per-op RPM, signed
DOC and complementary overlap conventions, absent cutting material identity,
and CPS metadata that bypasses operation feed overrides/clamps.

Created FS0_QUALIFICATION and proposed ADR-0003: independent crate/snapshots,
material identity, scoped versioned JSON data, deterministic ambiguity/fallback,
one applied pass-rate authority with native geometry ownership, guarded project
version, manual/reopen invariants, 16 future validation cases and precise FS1
scope. Updated current navigation/status/upstream disposition and validation
index. No engine, final UX, algorithms, material tables or production schemas.

19 existing resolver tests passed after sandboxed sccache failed before compile
and approved wrapper-free rerun succeeded. Documentation links/whitespace/scope
checked at handoff. Audit ivac-c943; gate ivac-9ma3 awaits human approval. FS1 unstarted;
new FS0 commit/push authorization remains pending.


## 2026-10-12 — FS0 approval and FS1 pure engine

User approved FS0 and requested proceeding. Closed human gate ivac-9ma3 and
accepted ADR-0002/0003; started FS1 as ivac-86b8. No new commit/push authorized.

Added independent ivac-feeds-speeds workspace crate: versioned copied context,
validated complete supplied profiles, explicit family matching and strict scope
containment, deterministic tier/ambiguity policy, chipload-target or exact-value
arithmetic, independent machine/tool constraints and transparent rounding,
source evidence, applicability and adjustment provenance. No geometry/core/CPS/UI
dependency, production material data, adapter or project-schema changes.

38 qualification tests plus compiled API doctest pass; targeted Clippy with
warnings denied, WASM compile, workspace formatting, schema drift and Serde-only
normal-dependency checks pass. Serial all-features workspace tests excluding
Tauri pass with initial 34 FS cases; final four cases/doctest passed separately.
After the workspace run, provenance additionally snapshots selected family memberships;
arithmetic/matching behavior is unchanged and final targeted checks passed. Updated qualification/status/
architecture/roadmap/validation/upstream docs. FS1 awaits human approval; FS2
and CAM integration remain unstarted; no physical evidence claim or app rebuild.
Engineering ivac-86b8 closes; pending human FS1 gate is ivac-406n.


## FS2 — 2026-10-12

User approved FS1 and authorized FS2. Added format-1 pure JSON loader, immutable
loaded document/engine, catalogs and atomic semantic validation, using existing
serde_json version. Six licensed Sienci reference rows transcribed and visually
checked against page 1 of the pinned metric PDF. Exact source context retained;
no lab machine/material/tool equivalence, local evidence or missing cutting
parameters invented. 38 engine + 11 data tests and 1 doctest pass; Clippy, WASM,
formatting/schema guards pass. FS3 unstarted; FS2 human approval and new commit/
push remain pending. See FS2 qualification for source, policy and limitations.

## FS3 — 2026-10-12

User approved FS2 and authorized FS3. Implemented core milling adapter, saved
cutting roles/provenance and explicit HTTP/Tauri/WASM endpoints. Added optional
schema feature to pure engine; native geometry remains DOC/overlap authority.
UI now loads data explicitly, calculates a preview, applies atomically, edits
manual rates and warns stale context. Portable v2 uses a distinct root kind
because old readers ignored version. Corrected native/CPS initial RPM and
active-pass feed/plunge metadata; no dynamic post-time recommendation.
Serial broad Rust and frontend tests, focused adapter/CPS regression checks
and real browser workflow qualify software behavior. Browser review fixed
release WASM import resolution and disclosure edit state. FS3 human gate
remains pending; FS4 and commits/push remain unauthorized.

## 2026-10-12 — FS3.5 UX/domain cleanup

- User explicitly requested FS3.5 only. Applied feeds-speeds, cam-ux-ui,
  ivacam-architecture, cam-validation and project-documentation. Audited ownership
  and compatibility before code; tracked ivac-x5pw and proposed ADR-0004.
- Made existing recommendation snapshot optional for manual-only operation rates,
  retaining tool fallback and native geometry semantics. Guarded portable cutting
  v3 preserves old reader refusal; v1/v2 remain readable. Job material now lives
  in Stock with optional readable display metadata stripped from native matching.
- Replaced permanent expert form with current cutting values + focused explicit
  Suggest/Calculate/Apply. Added plain Modified/stale states, progressive finishing
  and provenance/depth disclosures. Sources moved to persistent global Settings.
  Tool Library prioritizes physical fields; Machine prioritizes available tools.
- Frontend 1,065 pass/one optional skip, core FS ten pass, ShopBot eight goldens
  pass; native check, Clippy/schema/format/lint and WASM/frontend release pass.
  Real browser workflow and actual v3 save/reopen/source-disable confirmed;
  screenshots and a human protocol retained. Fixed conflicting global row CSS,
  added an explicit missing-cutter-rates state and validation before saving
  incomplete manual blocks. Deferred UX labels/source isolation tracked in
  ivac-xz59/ivac-xf9w.
- No math/post behavior change, no physical work, no new native bundle or push.
  FS3 review superseded by FS3.5 human gate ivac-eh1g. STOP before FS4.

## 2026-10-12 — FS3.6 compact desktop UI polish

- User declares FS3.5 functionally satisfactory and authorizes FS3.6 only.
  Closed FS3.5 human gate ivac-eh1g; applied cam-ux-ui/project-documentation.
  Audited existing theme tokens, shared property CSS and adjacent workspaces.
- Added a small shared spacing/density layer and label/value/unit/status grid.
  Reduced decorative nesting/indentation, quieted headings and provenance,
  separated operation name/cutter hierarchy and emphasized only Apply locally.
  Compact stale state retains kept-values text and full explanation tooltip.
- Browser review exposed narrow-width label/value competition; shared row grid
  now preserves numeric values and units, moving metadata beneath values when
  inspector content is below 260 px. Qualified 240/360 px desktop side panels,
  Profile/Pocket, mixed origins, proposal, long names, six-digit values, advanced
  depth, finishing and provenance. Keyboard Tab/focus remains visible.
- Frontend 1,065 pass/one skip, check zero errors/warnings, targeted ESLint/
  Prettier, production build and whitespace pass. 437 protected source/data files
  unchanged against the pre-checkpoint hash snapshot. No new native/physical run.
- Added ux/DESIGN_DIRECTION.md and ux/FS3_6_QUALIFICATION.md; extended cam-ux-ui
  visual guidance, preserving the existing user-authored Adobe reference.
  Tracked stock-offset explanation ivac-703e, no-match reason ivac-evl0 and
  broader contextual inspector ivac-dqax separately. FS3.6 human gate ivac-vgu4
  awaits review. No domain/math/source/post changes, no commit/push, no FS4.

## 2026-10-12 — FS3.6 human review revisions

User authorized clarification of stock frame/no-match and disclosure/status polish.
Resolved ivac-703e/ivac-evl0 and style ivac-3d53. Reused computeFootprint for drawing/
WCS corner readouts; stock placement unchanged. Shared smaller disclosure now has
8 px separation. Blue-star Suggested and violet-pencil Modified vignettes retain
text, keyboard focus and light/dark colors. No physical-validation implication.

Added optional transient scope_rejections response metadata from the shared native
scope predicate, with unchanged matching/calculation/precedence and saved formats.
Actual DOC 1 versus required 2.8 shown; context changes hide outdated refusals.
Existing engine qualification plus new explanatory/compatibility tests pass:
1,068 frontend/one skip, 50 engine/data plus doctest, 11 core adapter, eight ShopBot
goldens, five CPS guards. Check/lint/format/Clippy/schema and final release WASM/
frontend pass; used cached offline tools after partial pkg freshness issue.
Browser stock/WCS, exact no-match, disclosure keyboard and 240/360 px status layout
verified; captures/addendum retained. FS3.6 gate ivac-vgu4 remains open; no FS4,
native bundle, machining, commit or push.


### 2026-10-12 — FS3.6 focused recommendation disclosure (ivac-rncu)

Moved automatic machine/cutter/operation/pass-depth context into a native closed
Calculation context disclosure. Material, relevant engagement, cutting mode,
compact source/evidence and explicit calculation remain visible. New/existing
states now say Calculate recommendation/Recalculate; duplicate opener and source
paragraphs removed. Previous-source and pending-evidence wording avoids claiming
an applicable profile before calculation. No math, domain or persistence changes.

1,068 frontend tests pass / one skip; check/lint/build pass. Browser verifies
keyboard disclosure, Pocket nominal engagement, 240/360 px density and manual
feed 1,500 retained beside unapplied suggestion 1,620. Protected audit adds zero
changes beyond prior seven diagnostic files. Qualification/captures updated;
FS3.6 human gate stays open, FS4 unstarted. No commit or push.


### 2026-10-12 — Accumulated FS0–FS3.6 publication authorized

User explicitly requested commit and push of all work to date on
feature/shopbot-mvp. Publish the pure engine, versioned licensed reference data,
CAM integration, physical-tool/operation cutting ownership, source settings,
inspector refinements, qualification records, fixtures and screenshots together.
Existing frontend/native qualification remains the evidence baseline; publication
does not imply physical validation or close the FS3.6 human gate. FS4 remains
unstarted.
