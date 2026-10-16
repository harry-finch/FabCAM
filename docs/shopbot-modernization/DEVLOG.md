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


## 2026-10-12 — FS3.7 multi-source source packs

- Explicit user checkpoint request authorizes phases A–G. Audited special Sienci
  paths, imported preference bodies and full applied snapshots before design.
- ADR-0005 defines shared schema/loader, explicit material taxonomy, conservative
  comparable-record policy, reviewed public registry and private-safe persistence.
- Migrated the six Sienci references without changing cutting values. Reverified
  pinned LICENSE/handbook/PDF and original SHA-256; kept CC BY-SA separate from GPL.
  Candidate manufacturers remain licensing-review-only; no proprietary data copied.
- Native imports use app_data_dir path APIs, immutable safe filenames, bounded
  validated JSON and Unix 0600 files. Browser imports use local IndexedDB/WASM;
  no private HTTP calculation, telemetry, sync or automatic export.
- Generic Settings manage built-in/local pack enablement. Operations retain compact
  material/engagement/mode and aggregated sources, with advanced provenance/rank
  explanation and visible disagreement. Apply remains explicit.
- New guarded v4 projects retain safe source identities and resolved rates; legacy
  project versions and datasets remain supported. Actual browser save/reopen with
  missing local pack retains rates and generates successfully.
- Analytical/native/frontend/post checks and captures are recorded in qualification.
  Native desktop rebuild and Windows/Linux runtime checks were not performed;
  physical calibration remains unstarted. Engineering ivac-3957 handoff stops at
  human review ivac-tm7u; no current checkpoint commit/push is authorized.

## 2026-10-12 — FS3.8 local preset capture and validation

- Audited FS3.7 contracts/storage/evidence/resolver before extending them. No
  parallel preset DB or new cutting-evidence enum.
- Core capture shares generation's saved_rates and native request context; manual
  values remain authoritative, and Save never changes the current operation.
- SourcePack schema 2 adds local authoring/history; public schema-1 data untouched.
- Sorbonne immutable revisions, native import/save lock and optimistic head checks;
  browser IndexedDB transactions. Current head loads without old-body bloat.
- Compact Save dialog, explicit physical-use/success/date guards; later Settings
  promotion, notes/history/search/disable/confirmed reversible delete/backup.
- Explicit one-time UUID machine/tool identity setup supports initial lab use;
  display renames keep physical identity. Browser QA caught empty legacy names;
  stable-ID labels now prevent that capture failure.
- Tests/docs in FS3_8_QUALIFICATION. No FS4, real physical evidence or publication.

- Browser before/after project downloads confirm identical operations/tools/machine
  and no private capture observations. Draft backup restores on a fresh origin;
  duplicate import and stale capture refused. Rust JSON ordering/nullable fields
  are normalized for duplicate/catalog comparison; reactive-proxy and normalization
  regressions added. Engineering handoff gates FS4 at ivac-7yii.

## 2026-10-12 — FS3.7/FS3.8 publication and debug cleanup

Explicit user instruction authorizes commit/push of the qualified source changes
on feature/shopbot-mvp. Human review gates remain separate; FS4 is unstarted.
Oversized rebuildable debug artifacts are cleaned with cargo clean --profile dev;
release builds, WASM release package and delivered applications are preserved.

## 2026-10-13 — defer FS4 and resume milling CAM audit

User explicitly redirects the track. FS4 is DEFERRED for unavailable physical lab/
ShopBot/tool/material access, not a failure or software blocker. Keep FS4 on the
roadmap; existing FS3.8 architecture supports later resume without redesign. Freeze
FS datasets/preset tuning/math/UX; ReferenceData remains distinct from real trials.
WJ and VINYL remain deferred.

Inspected milling implementation and test assertions: Profile/Pocket/Drill/V-carve,
ramp/helix, winding, tabs/leads, stock/WCS, warnings, dogbones/T-bones, surfacing,
operation reuse, preview/simulation and fixtures. Found generic helical circular
records vs ShopBot planar-arc preflight mismatch, bbox-centered manual stock offsets,
unsupported ShopBot multi-WCS choices and missing ordinary fixture-creation UI.
No dogbone/T-bone generator or named reusable CAM operation-template library found;
ballnose relief/spiral overlay do not constitute planar spoilboard surfacing.

Published an evidence-scoped feature matrix, priority/dependency order and proposed
UX1 entry/post qualification with UX1-01..08 acceptance. No application code changed;
124 fresh frontend checks across 9 files pass. Rust coverage inspected, not rerun;
no physical evidence or newly reproduced numerical bug claimed. STATUS condensed
to current direction, source/delivery and limits; historical qualification remains
in dedicated records. FS4 issue ivac-mwry deferred; milling approval ivac-s7zo open.
Stop before implementation. This checkpoint has no new commit/push authorization.

## 2026-10-13 — UX1 software entries and handoff

Explicit user approval starts UX1 after the milling audit. Reproduced circular Ramp
rejection at ShopBot varying-Z arc preflight; enabled scoped Ramp through existing
0.01 mm CPS XYZ linearization. Generic planning unchanged. Transient section inputs
allow explicit refusal of Helix/Trochoidal/path helix, Ramp+tabs/leads and emitted
vertical fallback, preserving authored rates/parameters. Corrected curved-ramp
boundary warning and added compact inspector support/refusal explanation.

Discovered numeric work-origin XYZ does not translate generic output coordinates;
ShopBot now rejects those offsets rather than implying support. Explicit geometry/
stock translation with numeric zero G54 tested. Recorded separately scoped UX2
follow-up ivac-vj11. Browser testing also exposed downloadable stale programs and
Save clearing dirty; runtime wire-input identity now guards shared export, including
recheck after native save picker. No persisted schema or FS changes.

Native core/M1/M2/snapshot checks 1,054/11/14/4 pass, strengthened final UX1 variants
6 pass; frontend 1,092 pass/1 optional skip in 115 files; check/lint/fmt/Clippy and
optimized WASM/frontend build pass. Real local browser verifies generation/export,
manual 1500 Save/reopen, stale edit/save refusal and explicit Helix failure/recovery.
See ux/UX1_QUALIFICATION.md for durable acceptance evidence. Scratch builds avoided
root target/debug; stopped test servers/tab and cleaned 1.5 GiB of temporary Cargo
artifacts after qualification. No new desktop bundle, physical test, commit or push. Stop at
human gate ivac-cb64; FS4 remains deferred/nonblocking and UX2 unstarted.

## 2026-10-13 — Continue UX1: capability resolution and output parity

User refines UX1 scope; retain existing work, reopen engineering ivac-uigy. Name
remains UX1, covering CAM/post compatibility as well as UX. Real clamp/keep-out
creation, stock reference and numeric WCS repair remain CAM2/UX2; concave/island
drawing tests remain generic CAM geometry. No next milestone started.

Found CPS preview used ideal generic arcs and combined rapid axes while ShopBot
emits linearized XYZ/split jogs. Formalized existing preflight/kernel linearize as
capability resolution and added isolated owned-output preview adapter (ADR-0007).
Final preview, sim, timing and envelope checks consume rounded emitted movements,
with source-line mapping; unknown output fails instead of falling back to IR.
Initial operator trajectory/manual pauses remain explicitly unmodeled. No schema
change. Per-op existing diagnostic params report requested/resolved representation;
compact inspector uses informational status and collapsed Machine compatibility.

Generic Helix geometry fallback now emits an explicit resolution comment, with no
machine-specific planner branch or intent rewrite. ShopBot distinguishes geometric
fallback from true Helix, rejects both before output; full-region/island safety and
subsequent contour re-plunge are not qualified. Corrected misleading Helix tooltip.
A small pocket at 3° reproduced vertical fallback; guard retained. Independent SBP
decoder proves exact known-state preview endpoint/motion/source-line parity for
Direct/Ramp Profile/Pocket; analytic circle/offset/direction tolerances retained.

Final native 1,056 core/11M1/17ShopBot/4snapshots pass serial; frontend 1,092/1skip
in 115 files, check/lint/fmt/Clippy/WASM/frontend/server builds pass. Browser reopens
saved Helix unchanged, keeps 1500 manual feed, refuses it, explicitly restores Ramp
and exports SBP (160 displayed lines); final preview has 136 actual emitted moves instead of 101
generic moves. Documentation/ADR/qualification/screenshots updated. No physical
trial, desktop bundle, FS change, commit/push or CAM2/UX2 implementation. Stop at
human review ivac-cb64; FS4 deferred/nonblocking.

UX1 continuation cleanup: stopped local test servers/tab and removed 1.5 GiB of
Cargo scratch artifacts; root target/debug stayed absent, release/WASM retained.

## 2026-10-13 — Future remaining-material roadmap (documentation only)

User requests a future diagnostic Remaining Material Analysis / Rest Material
Visualization feature. Preserve UX1 software completion/output parity/Helix refusal,
requested intent and deferred nonblocking FS4; CAM2/UX2 remains next. Record CAM3
broader entry/tabs/leads qualification ahead of CAM4 analysis/overlay and coordinated
CAD1 dogbones/T-bones. Keep existing UX3/UX4 labels for later surfacing/templates;
V-carve qualification and Rest Machining remain later advanced work.

Future plan separates generated cutter-sweep vs intended removal at qualified
stock/reference/depth context, intentional tabs/allowance vs accidental residuals,
generic vs resolved path questions and provisional analytical acceptance. Phase A
analysis is useful without dogbone generation; B preview and C corner relief follow.
V1 is diagnostic/visual, not generalized simulation or smaller-tool cleanup.

Confirmed professional-workshop-ui is already installed in repository .agents/skills;
read it and relevant icon/color/interaction guidance, did not create/edit a competing
skill. Record shared visual language + cam-ux-ui pairing, compact amber/token-based
overlay and consistent SVG icons in durable design guidance. Updated ROADMAP,
STATUS, milling planning/audit, design guidance, future plan and documentation index.
No application/post/UX1 evidence change, implementation, rebuild or physical test;
document links/content and unchanged-code/evidence checks only. No commit/push.

## 2026-10-13 — CAM2 / UX2 software handoff

User approval of UX1 and instruction to continue authorized bounded setup work.
Implemented effective stock minimum corner controls without moving legacy jobs,
once-before-CAM numeric XY normalization and drawing-frame resolved preview.
Fixture box/cylinder/polygon editing reuses history/persistence; resolved-path
candidate warnings integrate with the existing critical export setting. Unknown
body/holder clearance remains explicit and conservative for known cutter footprint.
Nonzero Z rejects; discovered legacy two-sided CPS scaffold/shifted-registration
limitations now reject before output (future ivac-u1ko / ivac-jtkj).

Native/core and frontend checks, complete WASM build and actual browser
collision/refusal/recovery + Save/reopen pass. Manual feed and Ramp intent retained.
No FS math/data changes, physical test or native bundle. See
[UX2 evidence](ux/UX2_QUALIFICATION.md) and [ADR-0008](adr/ADR-0008-work-origin-frames.md).
Engineering ivac-69ak complete; human gate ivac-97fb pending. Stop before CAM3.
Publication remains unauthorized. Root debug stays absent; scratch build is cleaned.

## 2026-10-13 — UX2.1 direct manipulation and collision feedback

Requested follow-up preserves CAM2 architecture. Fixed actual Generate/reportError
object-coercion paths using whitelisted user-facing errors and preserved backend
localization metadata. Added compact critical collision row and official Lucide
icons; Show selects/reveals/frames a fixture using live 2D container bounds rather
than hidden canvas dimensions. Selected colliding fixtures use red dashed outlines.

Mouse/pen fixture dragging uses the existing model, a captured drawing transform
and one release command; no visual-only placement/history-per-move. Existing
snapshot and generation identity preserve saved XY and stale export refusal.
Actual box/cylinder/polygon drag, Undo/Redo, Save/reopen, moved-polygon collision
and numeric correction/recovery verified. Tests/check/lint/build pass; no collision
math, FS, persistence migration, native bundle or physical trial. Snapping and
stock/WCS/CAD interactions deferred. Engineering ivac-yfg8 closed; human gate
ivac-svnf before CAM3. See [evidence](ux/UX2_1_QUALIFICATION.md).

## 2026-10-13 — CPS availability message during fixture testing

User reported cps_unavailable while expecting collision feedback. This is an
engine capability refusal, not a fixture verdict: standalone production browser
mode defaults to WASM without CPS, while UX2.1 was qualified against a CPS-enabled
native HTTP server. Desktop/server default CPS features remain enabled. Clarified
localized error and recovery hint: generation stopped, ShopBot .sbp requires CPS,
and resolved collision checks require successful path generation. No transport,
post, fixture mathematics or file format change. Test environment (browser URL or
native app) requested; no unverified runtime cause or new bundle is claimed.

## 2026-10-13 — CPS browser review environment restored

User confirmed port 18767 with ?api=wasm. Built current native CPS server offline
in /private/tmp/fabcam-cps-runtime and started it on 127.0.0.1:18772 with matching
18767 CORS. Prepared the same frontend with explicit HTTP API query. Kept the
server available for human review; no automatic fallback, post-format change,
user-file overwrite or service installation. Standalone WASM remains available
for its qualified supported posts. Root target/debug stays absent. Local URL and
session boundary are recorded in STATUS (ivac-t79u).

The configured 18767 page generated the ShopBot software collision fixture:
136 resolved movements, 160 output lines, 7 warnings including one critical
fixture_path_collision naming Clamp 1. No cps_unavailable refusal. Browser proof
is ux/UX2_1-cps-browser.png; page retained as a human-review deliverable. Runtime
server session is intentionally retained until review completes.

## 2026-10-13 — Inspector accordion collapse and natural content height

Removed active/previous-panel bounce after user's report. Inspector permits all
sections closed, keeps explicit contextual reveals, and uses natural-height flex
hosts with scrolling only when content exceeds available height. Updated obsolete
collapse tooltips. Verified Layers/Operations closure, compact empty Text and short
Layers/Operations, long Stock/operation scrolling, keyboard Enter and fixture
selection revealing Stock in the actual desktop browser. Frontend 1107 pass /
1 optional skip, zero check warnings, lint/build pass. No CAM/FS/storage change,
CAM3 still unstarted. See ux/INSPECTOR_ACCORDION_FIX.md (ivac-16yy).

## 2026-10-13 — Future FabCAM / FabSBP setup ownership (documentation only)

Recorded user-directed product boundary in ADR-0009 and FABCAM_FABSBP_WORKFLOW:
FabCAM declares intent; future FabSBP establishes physical state and executes.
Separated future XY origin/Z reference, reusable profile capability/setup/workflow
areas, standalone behavior and explicit unfrozen requirements metadata. Added
unnumbered future roadmap groups, profile assistant/validation, guided default-ON
setup, structured pre/post actions and shared Professional Workshop UI placement.
Zero-setting cannot follow merely from origin declaration. Existing machine/tools
snapshots and qualified XY behavior stay intact; Z/two-sided guards unchanged.
UX2.1 human gate remains pending, CAM3 next and unstarted; FS4 deferred/nonblocking.
No source, post, schema, UI or FabSBP implementation changes. Documentation links
and whitespace checked; no new software/physical qualification claimed (ivac-22df).

## 2026-10-13 — CAM3 bounded tabs/leads qualification

User validated documentation and requested progression; closed UX2.1 gate ivac-svnf
and started CAM3 ivac-sa8q. Independent output tests reproduced configured tab
width being replaced by tool diameter, ignored taller intermediate-pass overrides,
chord projection of curved tabs, discontinuous/coarse curved tab ramps, a lead-out
recutting a tab and direct entry plunging through a manual tab. Corrected generic
attachment/emission; no ShopBot syntax, FS, schema or new UI feature. Added four
CAM3 tests including 48 simple-shape combinations and reviewed intentional width
snapshot changes. Complete build and regression results in CAM3_QUALIFICATION.
Conservative rectangular arc lifts and remaining topology/physical limits stay
explicit; Ramp-entry+tabs/leads refusal retained. Human gate ivac-wh2m; CAM4 not
started. CPS review server rebuilt in retained scratch target; no app bundle.

## 2026-10-13 — CAM4 A/B remaining-material analysis and 2D overlay

User approved CAM3 and requested progression, later asked to resume ongoing work.
Closed ivac-wh2m; continued ivac-qk8y without restarting. Added bounded per-pocket
final-depth capsule/disk difference using existing Clipper, intended regions and
final returned linear movements. Reused hole conversion; kept arbitrary CPS IR,
analytic arcs/curved targets, tabs/allowance and unqualified depth/tool contexts
explicit. Added optional buffered response/schema/types, work/drawing frame mapping,
compact translated overlay/metrics/detail and generation-identity stale suppression.
Extracted canvas region-layer logic to preserve the component line budget instead
of raising its guard. No toolpath/post/FS/persistence change or rest machining.
Analytical and emitted-path tests, complete regression/check/lint/build and browser
stale-after-Save/unsupported/overlay evidence in CAM4_QUALIFICATION. Retained updated
native CPS review server on18772; no desktop bundle. A/B gate ivac-tvox; CAD1/C
corner relief remains separate scope/approval.

## 2026-10-13 — CAM4 review: WASM checkpoints and 3D path visibility

Fixed simulator i64/bigint→number checkpoint boundary after user scrub errors;
invalid/sentinel indices use full replay. Added two regression tests through
backward replay planning. Aligned warning-chip SVG/text and added transient 3D
Toolpaths checkbox using visibility-only scene updates. Audited existing Spiral
and added compact choice guidance, without changing Cascade defaults or geometry.
Actual browser Cascade/Spiral rewind and final visibility checked; emitted listing
J2=13→2/JZ=16→5 on the software test, Spiral residual analysis still unsupported.
Explained restrictive Sienci reference coverage for metric softwood pockets; FS
math/data remain frozen. Frontend1112 pass/1skip, check/lint/build pass; four existing
Spiral tests pass. See ux/CAM4_REVIEW_FOLLOWUP.md (ivac-ubcr).

## 2026-10-13 — FS3.9 targeted approximate-matching reopening

User explicitly reopened FS for bounded matching/coverage, preserving deferred
nonblocking FS4. Audited six narrow LongMill single-flute full-slot references.
Kept legacy/exact math; added pure normalized nearest/extrapolation and unique
within-series interpolation with explicit router/material/tool/slot-to-pocket
transfers, graph taxonomy, provenance, deterministic trace and accurate refusal
reasons. Same-mode reconciliation/disagreement and exact six-row outputs retained.
No new cutting rows or private data imports; reviewed public schema3 metadata only,
original CC BY-SA profiles untouched. Router class cannot masquerade as an
individual physical trial scope. Compact inspector explains quality/transfers under
Why these values?; saved values remain authoritative and private snapshots drop.

94 FS tests +1 doctest,17 core adapter tests,1112 frontend passes/1 optional skip;
check/lint/fmt/Clippy/schema-check/build qualified. Actual browser Ø6/Ø6.35 Pocket,
extrapolated Ø2.2, explicit Apply and manual1200 save/reopen verified. CPS review
server rebuilt/restarted for additive provenance; no native desktop or physical
qualification. [Handoff](feeds-speeds/FS3_9_QUALIFICATION.md),
[policy](feeds-speeds/FS3_9_MATCHING.md), [coverage](feeds-speeds/FS3_9_COVERAGE.md).
Human gate ivac-t0fk; CAM4 gate ivac-tvox remains pending independently. No commit
or push authorized for this checkpoint. Stop before further FS work.

### FS3.9 resumed handoff and analysis

Final frontend check/lint/build passed; 1112 tests passed/1 optional skip. Native
browser Ø6 reproduced17000/1530/765/DOC1; Ø5 interpolation and Ø2 refusal also
recorded. ShopBot rough17000/finish10000 exposes existing unsupported spindleSpeed
record (ivac-2zll). Explicitly concordant RPM software export produced77moves/133lines
and preserved manualfeed1200; no general post or physical qualification implied.
User requested analysis/proposal after completion: recommend a deterministic diameter
sensitivity sweep around policy and anchor-selection boundaries, before considering
any smoothing. Proposal only, no new resolver mathematics implemented. See
[review](feeds-speeds/FS3_9_REVIEW.md). Human gate ivac-t0fk remains open.

## 2026-10-13 — Approved FS3.9 sensitivity audit

Added developer CSV sweep and stdlib report generator:11997 samples (1333 diameters,
three modes, DOC1/DOC10/synthetic caps). Measured Prudent feed jumps≈10% at the
small-tool near boundary and≈57% at interpolation entry; close-match selection also
reduces DOC≈27%. Added four regressions for boundary refusal, both numeric transitions,
all-mode order invariance, evidence and hard constraints. FS98 tests +1 doctest pass;
Clippy/fmt pass. No production math, UI, source or persistence edits. Results and
revised continuity proposal in FS3_9_REVIEW/FS3_9_SENSITIVITY; FS4 remains deferred.

## 2026-10-13 — UX2.2 shell and welcome roadmap (documentation only)

Added PLANNED UX2.2 specification, roadmap row/placement note, status pointer,
UX design direction and navigation link (ivac-3267). Applied repository-local
Professional Workshop UI, CAM UX, architecture and documentation skills. Read-only
source inventory confirms existing RecentMenu/workspace local storage, shared
file-operation unsaved-work guards and generation-context export freshness; these
must be reused, not replaced by header-only truth or a second recent store.

Requested UX2.1 → UX2.2 → CAM3 order conflicts with approved UX2.1/CAM3 continuation
and software-qualified CAM4 A/B. Kept history intact; propose the shell audit/design
checkpoint after current handoff reviews, subject to human planning gate ivac-dwdd.
Recorded distinct no-project/new-empty/no-operations/no-current-path states,
command grouping, safe cancellation/switching, planned acceptance, targeted tests,
desktop screenshots and human review. FS3.9 and deferred/nonblocking FS4 unchanged.
No source/UI/algorithm/engine/project-format/post behavior changed; no implementation,
new software qualification, commit or push. Local added links and whitespace
checked; frontend/core/schema/data hashes unchanged from the session baseline.
No software tests/build needed for documentation only. Stop pending human approval.

## 2026-10-13 — FS3.9.1 recommendation UX simplification

User authorized UI-only checkpoint. Audited inline assistant and FS3.8 capture; reused
Modal for compact values/context, auto mode preview and coherent footer. Added transient
request ownership with cancellation/disposal/key guards, local full-width confirmation
committed only by Apply, trigger focus restoration and raw-diameter presentation facts.
Kept source/diagnostics accessible, blocking issues visible, no-match Apply disabled.
Moved local capture to effective values and defaulted existing form to Draft; Tested
still requires physical evidence. No math/source/storage/domain/post edits.
Frontend1117 pass/1skip, engine98 +1doctest, check/lint/build; actual mode/Cancel/Apply,
manual1200 save/reopen, keyboard, matching/refusal/invalid-limits and1024×600 checked.
Handoff/screenshots: ux/FS3_9_1_QUALIFICATION.md. FS4 deferred/nonblocking; stop for
human visual review. No commit/push authorized.

## 2026-10-13 — FS3.9.1 inspector / advanced-entry continuation

Removed duplicate default recommendation/provenance block; retained saved details in
Modal for unavailable recalculation. Shared secondary preset/review row and origin
baseline/manual exceptions. Reused inspector disclosure/numeric tokens; Pocket overlap
renamed precisely, original values/handlers unchanged. Existing blocking EntrySupport
message now visible outside closed depth-entry disclosure; no policy/post changes.
Frontend1121 pass/1skip/check/lint/build;27 CPS ShopBot regressions pass. Actual before/
after/apply/manual/Ramp/Helix/closed-warning/small-desktop screenshots in existing
FS3_9_1_QUALIFICATION. Same human gate ivac-anp0; FS4 deferred/nonblocking; no new CAM
milestone, commit or push. Engineering task ivac-m0qf complete.

## 2026-10-13 — UX2.2 selected; first design checkpoint

User selected UX2.2 as the next development milestone. Completed source-backed
command inventory, lifecycle ownership audit, Atelier/Document interactive design
alternatives and targeted qualification plan (ivac-7737). Recommended Atelier.
Identified existing empty-project discard, save identity and load-helper reset
limits; proposed bounded New/Close scope in ux/UX2_2_AUDIT.md. Actual source remains
unchanged. Mockup actions are simulated, not software qualification. Human design/
lifecycle gate ivac-dwdd remains before implementation; earlier CAM4/FS gates stay
independent. No CAM, FS, post, persistence change, commit or push.

### UX2.2 bounded machine/tool navigation continuation

User requested active machine (and operation tool) visibility in Atelier menu bar.
Added reactive machine snapshot name to existing tab and separate primary operation
tool shortcut, with full accessible names and bounded ellipsis. Tool shortcut reveals
the selected operation, avoiding false identity matches with the shop inventory;
library remains independently accessible. Unnamed/missing fallbacks translated EN/DE.
Check/lint/build pass; 19 i18n/sidebar tests pass, 1 optional skip. Actual browser
fixture selection/long-name/small-window review recorded in UX2_2_AUDIT.md.
No lifecycle/CAM/FS/post changes, commit or push; broader UX2.2 remains in progress.

## 2026-10-13 — UX2.2 Atelier implementation

- Explicit user approval closes ivac-dwdd; implemented ivac-j6ba on the existing
  feature branch without commit/push. Welcome and compact file menu reuse existing
  Open/Import/Save/example/recent paths. New/Close retain machine/tool context and
  clear job geometry, operations, selections and both generated sides.
- Added runtime document identity and saved snapshot baseline: an empty edited
  project is protected, and generation cannot incorrectly mark a job saved.
  Failed replacement/conversion restores the prior document/history. Async
  generation results are tied to a document epoch.
- Active machine is always visible in desktop navigation; selected-operation tool
  links to its inspector. Shared UI tokens/Lucide and EN/DE messages reused.
- Frontend lifecycle coverage and browser evidence recorded in
  [UX2.2 qualification](ux/UX2_2_QUALIFICATION.md). Human/native gate ivac-ein1;
  existing CAM4/FS gates unchanged. No CAM/FS/post or physical qualification claim.

## 2026-10-13 — UX2.2 final shell polish (ivac-8u1p)

- Preserved Atelier. Compact machine presentation uses the persisted name; explicit
  SOFTWARE ONLY/unvalidated name qualifiers remain visible with a small Lucide
  caution icon. Full identity remains in title/accessible name. No new validation
  field, inferred positive certification or profile-selection behavior.
- Kept Layers as primary import; removed the competing empty-canvas button.
  File import and Layers Add still support additive geometry.
- Idle generation summary appears only when operations exist. Generated diagnostics
  remain visible after editing/removing operations; shared stale calculation now
  includes output awaiting simulation. Critical count/export handlers unchanged.
- Frontend 1145 passes/1skip, check 0 errors/warnings, lint/build and diff checks pass.
  Browser tested primary/secondary imports, native-server CPS collision generation,
  blocked export and edit/save/reopen consistency. Seven screenshots and limitations
  in [polish review](ux/UX2_2_POLISH.md). Stop for human review ivac-ein1; no new milestone.

## 2026-10-13 — Publication checkpoint authorized

User explicitly requested committing and pushing all current repository changes
to origin/feature/shopbot-mvp. Includes CAM/ShopBot improvements, remaining-material
analysis, FS3.9/FS3.9.1, Atelier UX2.2 and their qualification records, fixtures,
screenshots and regression tests. Frontend publication checks: 1145 tests pass,
1 skipped; type checks, lint and production build pass. Existing human/native/
physical review gates remain independent and pending as recorded above.
Rust publication checks: 1063 core unit tests and 98 feeds/speeds tests plus
1 doctest pass. The broad CPS integration run stops at the legacy
`cps_posts::two_sided_runs_both_sides` success expectation: current code deliberately
rejects that unqualified capability (ivac-jtkj). Regression alignment follow-up
is recorded as ivac-13go; the publication checkpoint does not claim a fully green
broad Rust suite.

## 2026-10-14 — GitHub fork renamed FabCAM; ShopBot integrated into main

User explicitly authorized merging feature/shopbot-mvp into main and renaming
the GitHub repository. Main had no unique commits and advanced by fast-forward
from 9168104 to c9d3f3b, preserving all 11 feature commits and their source tree.
The fork is now harry-finch/FabCAM; origin was updated locally and upstream remains
aalarchiv/ivacam. Feature branch retained. Governance references updated in both
AGENTS.md and CLAUDE.md. Application branding and package identifiers unchanged.
No new code behavior or physical/native qualification; prior publication test
results apply to the identical merged source, including known CPS follow-up
ivac-13go. Tracking: ivac-wesn.

## 2026-10-14 — FabCAM A1 Optical Balance asset finalization

User supplied authoritative symbol/app-icon/lockup SVGs and requested an
asset-only finalization with professional-workshop-ui and project-documentation.
Copied all three originals byte-identically into assets/branding/fabcam/source;
canonical geometry/palette unchanged. Added four monochromes and a separately
named micro candidate enlarging only the inner opening for 16–20 px review.
Rendered eight symbol sizes; practical canonical minimum 24 px, app tile 32 px.

The original font stack has no author-rendered font proof; local resolution was
Verdana. Temporarily used official Inter 4.1 Medium/Bold, the first declared
family, for accurate Inkscape text-to-path. Editable original retained, no font
files bundled. Text/outline bounds match at 4×; 300/412160 edge pixels differ.
Outlined rendering is identical with/without the temporary font configuration.

Validated nine SVGs, palette/geometry/alpha/bounds, 22 PNG dimensions, seven
ICO entries matching source pixels, and native-decodable ICNS. Lossless PNG
entries replace legacy 16/32 px ICNS data; Pillow decodes every entry exactly.
Native iconutil extraction still alters 16/32 px antialiased RGB edges while
preserving alpha; the other eight slots match. Reported this limitation rather
than claiming installed-platform visual qualification. README, comparison sheets
and validation.json are the handoff. No unrelated application source, Tauri
configuration, existing branding assets, release, commit or push changed.
Engineering ivac-79bq complete; human typography/micro/integration gate ivac-8k3o.

## 2026-10-14 — A1 branding integration into the UX2.2 shell

User explicitly authorized integration of the production A1 masters. Closed
ivac-8k3o for canonical artwork/outlined Inter lockup; retained micro as a separate
unapproved candidate. Header adds a 24 px symbol beside the existing FabCAM name;
measured navigation height unchanged at 30.2890625 px. Welcome h1 uses the outlined
lockup at 258 px width; existing actions/recents/lifecycle unchanged. About changes
only the 80 px logo reference and EN/DE alt text, preserving identity/legal/version
information and links. SVG plus 16/32/64/128/256 PNG favicons; Apple touch 180 px.

Extended the existing master-export script with a font-free --integrate command:
canonical runtime copies at frontend/public/branding and existing Tauri icon paths,
reviewed ICO/ICNS, direct SVG rasterization of Store sizes, exact dimensions/alpha
checks and integration manifest. No Tauri config/identity/security or runtime
graphics dependency change. Removed only the confirmed-unused public ivacam-logo
PNG; preserved historical artwork/fixtures and Android's independent pipeline.

Frontend 1145 pass/1skip, check 0 errors/warnings, lint and production build pass.
Local optimized macOS app bundle built; actual Welcome/About and Finder icon-view
captured. Info.plist identity stable and bundled icon bytes match reviewed ICNS.
Nine branding/favicon resources served with source-identical bytes and HTTP 200
in dev/production. Windows/Linux native build/runtime and Dock/small OS scale
behavior remain untested; native ICNS extraction caveat remains documented.

Engineering ivac-7cdj complete; existing UX2.2 human/native gate ivac-ein1 stays open
for broader checks. [Handoff](ux/A1_BRANDING_INTEGRATION.md), seven genuine captures,
app icon size sheet, brand README and BUILDING/UI docs updated. No new CAM milestone,
CAM/FS/post/schema/project changes, commit, merge, push, release or publication.

## 2026-10-14 — FabCAM public product identity and attribution

User authorized public product rename, About/origins and current documentation.
Audited references by public identity, upstream attribution, internal namespaces,
persisted/install contracts and historical records. UI/browser/window titles,
EN/DE text, report/picker labels and startup diagnostic now use FabCAM. Cargo's
public repository metadata points to the existing fork; names/authors/version
are retained. Root READMEs and active guides describe verified current features
and limits, not hypothetical roadmap capability.

About now shows A1 symbol, FabCAM and authoritative 0.5.0, upstream ivaCAM by
Soenke J. Peters, GPL/fork source/status and preserved acknowledgements/dependency
table in accessible disclosures. Official upstream Cargo/LICENSE/ABOUT verified;
LICENSE unchanged. Private data/provenance are untouched. No logo changes.

Tauri window/macOS display metadata use FabCAM. productName=ivaCAM remains because
default MSI upgrade identity and installation names depend on it; bundle filename,
ivac-desktop, org.opcenter.ivacam, permissions/security/signing/keys/extensions
are preserved. ADR-0012 records policy; ivac-snqn is a proposed future installed-
upgrade qualification, not an implemented migration.

Frontend1145 pass/1skip/check 0 errors/warnings/lint; Rust29 project tests pass.
Optimized local macOS bundle built with correct display metadata; actual runtime
reopened existing M4 project via recents and read settings. Browser identity/meta,
keyboard links/disclosures and seven genuine captures recorded in
ux/PRODUCT_IDENTITY.md. Protected baseline:361 source/data/license files identical;
only Cargo repository metadata differs by one verified line. No CAM/FS/post/schema
or serialization change. Windows/Linux native packages/upgrade untested; physical
gates/FS4 remain unchanged. Engineering ivac-9td7; human review ivac-7pul pending. No commit,
push, merge, GitHub rename, release or publication.

## 2026-10-14 — Branding and product identity publication approval

User accepted the A1 branding, FabCAM public identity, About attribution and
documentation, then explicitly requested commit/push to `main`. Human identity
review ivac-7pul is closed as accepted. Frontend 1145 pass/1skip, check/lint,
production build, Rust 29 project tests and the local macOS bundle checks passed
at the engineering checkpoint; publication introduces no further code changes.
The separate micro candidate stays excluded. Broader UX2.2/CAM/FS qualification,
Windows/Linux native qualification and proposed installer migration ivac-snqn
retain their recorded status. No release or binary publication is included.


## 2026-10-14 — CAD1 architecture and prerequisite audit

User explicitly selected CAD1 / CAM4 C and requested important architecture
decisions before code. Audit ivac-g9gn inspects core lines/arcs/chaining/offsets,
imports/transforms, shared effective geometry, project persistence and history.
Proposed source-preserving modifier with one accepted evaluated geometry for
view and CAM; identified tab reparameterization, transform/diameter, arc bounds
and multi-import object identity risks. See [audit](CAD1_AUDIT.md).

On f6b88c9, 6 residual unit tests, 1 emitted ShopBot integration and 34 frontend
prerequisite tests pass. A/B human gate ivac-tvox remains open; curved targets
remain outside residual diagnostics. Decision ivac-alch records the model/scope
and prerequisite checkpoint. No production implementation, full-suite claim,
physical review, commit or push. UX3/UX4 unstarted; CAD1 ends at human HARD STOP.

## 2026-10-14 — UX5 Command Palette & Quick Actions planning

Documentation only (ivac-70y7): added UX5_PLAN and roadmap/status/architecture/UX
notes plus navigation link. Future CAD1 → UX3 → UX4 → UX5 sequence preserves
existing labels, actual CAD1 architecture checkpoint, implemented UX2.2 and all
independent qualification gates. No current next-action replacement.

Read-only source audit distinguishes app-menu shortcut resolution/App effects from
commands/history mutations, identifies shared file/generation safeguards and shop
inventory versus stocked/project-tool ownership. Proposed thin future CommandRegistry,
local deterministic command/tool search, context availability and execution recheck,
explicit assignment, focus/shortcut audit and conditional approved UX4 integration.
Schema/shortcut details remain unfrozen. Fifteen future acceptance criteria require
regression, keyboard/accessibility, real metadata/performance and desktop review.

PLANNED — NOT IMPLEMENTED; human planning gate ivac-2zh9 precedes UX5 coding.
No application UI, CAM/FS/post, project format, operation or tool-management behavior
changed. No code tests/build, physical qualification or commit/push claimed.
Local added links/whitespace checked; all baseline source/config/data hashes
unchanged. An unrelated new CAD1 probe appeared during verification and was left
untouched. Stop pending human approval.

## 2026-10-14 — CAD1 implementation and software qualification

After user approval of the architecture checkpoint and bounded CAM4 A/B prerequisite,
implemented pure exact-radius dogbone/T-bone geometry, explicit batch preview/selection,
source-preserving v5 project modifier and atomic Undo/Redo. HTTP/Tauri/WASM share the
same evaluator; ordinary CAM/preview/ShopBot consume accepted evaluated contours.
Source/tool changes block stale generation/export; existing tab/approach bindings
and unsupported operation contexts are refused. Acute corners requiring isolated
plunges/access enlargement are refused without introducing post-specific geometry.

1070 core + 1151 frontend tests pass (1 frontend skip); 18 emitted-motion CAD1 cases,
42 existing ShopBot/snapshot tests, check/lint/Clippy/schema, Tauri compile and fresh
WASM/frontend build pass. Browser review covers Apply/Cancel, save/reopen, Undo/Redo,
ShopBot generation, stale-source blocking and compact desktop layout. No physical
cutting or new native-bundle runtime qualification. [Evidence, limitations and human
review](CAD1_QUALIFICATION.md), [ADR-0013](adr/ADR-0013-persistent-corner-relief.md).
Human gate ivac-7r7r remains open; HARD STOP before UX3/UX4. Uncommitted local delivery.

### CAD1 human-review fix — ivac-qt54

Fixed the orphaned corner-selection mode when changing sidebar panels. Unmount now
closes the draft and invalidates pending preview callbacks; empty state includes
contour-selection instructions and a recovery action. Verified actual browser
Operations/ Layers round trips with saved and unapplied reliefs, edge selection
and four-corner reopening. Frontend 1151 pass / 1 skip, check/lint/build pass.
Evidence appended to CAD1_QUALIFICATION.md. HARD STOP still at CAD1 human review.

## 2026-10-14 — UX3 planar surfacing implementation and handoff

User requested the next milestone. Recorded CAD1 continuation acceptance under
ivac-7r7r without inventing physical/extended geometry evidence; opened ivac-gepq
and human gate ivac-k4z3. Audited ReliefMill/drop-cutter and face-mill overlay:
kept their existing roles; added a pure planar raster and specialty driver instead
of synthetic imported contours. ADR-0014 records persisted rectangle/stock
snapshot, mm parameters, margin/radius/extra-overtravel distinction and v6 guard.

Qualified first/last transverse edges, distributed spacing, both scan directions,
shallow negative multi-pass depths and safe-Z travel. Manual rates reuse existing
saved roles while the recommendation engine remains frozen/unsupported here.
Explicit `planar_surface` ShopBot strategy preserves rejection of `parallel`/3D
and requires a flat cutter. No new OpenSBP command or physical-machine capability.

Stock XY excursions are scoped to validated surfacing, with a visible edge-travel
warning and retained Z/fixture/travel checks. Surfacing-only jobs no longer inherit
an imported-contour zero-inside-bbox heuristic; mixed contour jobs retain it.
Preview/UI review corrected missing-drawing messages in the operation row and 3D
viewport; sizing carries a bbox without injecting source chains. The independent
operation does not consume/rebind an existing CAD1 modifier.

Analytical coverage, decoded ShopBot coverage/parity, clearance/depth/rates,
17 refusal contexts, fixtures and legacy contour safeguards are exercised.
1074 core and1161 frontend passes/1skip,8 UX3 integrations,27 existing ShopBot
regressions,18 CAD1 contexts,14 server tests; check/lint/Clippy/schema/build and
CLI/WASM/Tauri compilation pass. Existing server handlers were moved before its
test module to resolve the all-targets Clippy ordering failure; behavior preserved.
[Qualification and browser captures](UX3_QUALIFICATION.md) record actual standalone
export, stale edit, v6 save/reopen of manual Y/1500 values and rejected stepover.
No new native bundle/physical cut/flatness measurement. Local uncommitted handoff;
UX3 human review ivac-k4z3 is next; HARD STOP before UX4.

## 2026-10-14 — UX4 local operation templates

User requested UX4; accepted UX3 continuation ivac-k4z3, engineering ivac-epg2.
Audited existing workspace, OpEntry, rates, history and job persistence. Added a
pure versioned whitelist model for Profile/Pocket/planar surfacing with explicit
current geometry/stock and rough/finish tool binding. Captured values stay manual,
step defaults freeze at source, private evidence/source coordinates are omitted,
source-bound/unsupported contexts refuse capture. No FS engine or CAM/post change.

Local library reuses workspace persistence; invalid/duplicate models skip on load,
ordered writes and transport failures are handled. Existing Modal/inspector styles
provide capture/rename/remove/restore and creation via addOperationCommand.
Browser review found and fixed numeric option values clearing a string-based tool
selection. Actual surface v6 and different-tool pocket v3 downloads retain values;
Undo/Redo, reload, generation/export and stale guards verified. Profile source is
disabled in the contour fixture to isolate pocket qualification from its expected
stock/work-area warnings. 1172 frontend passes/1skip, check/lint/build pass.
[Evidence and human review](UX4_QUALIFICATION.md); human gate ivac-51gg, hard stop
before UX5. Local uncommitted delivery; no new native/physical evidence.

## 2026-10-14 — UX4.1 templates and sidebar refinement

Human review requested focused UI corrections; engineering ivac-01ux, consolidated
review ivac-si5w supersedes ivac-51gg without functional/physical certification.
Audited library/OpEntry/command persistence and nested sidebar scroll ownership.
Kept template wire/manual values and CAM/FS/posts; added reuse-first search/list,
separate capture and contextual rename/confirmed delete, stored preview and known
cutter-diameter review. Moved modal outside clickable header to fix bubbling;
Create reveals the destination inspector. Pure helpers retain authoritative
instantiate validation. Shared title/count grouping fixes the five-child/four-grid
mismatch; total operation count remains direct project data, disabled included.

Active hosts own sidebar scrolling; sticky section heads reuse existing surfaces
and stacking tokens, with direct Stock padding compensation. Native keyboard
scrolling, resize and1100×660 review exposed/fixed top-edge bleed and disappearing
Create button in expanded preview; local sticky footer keeps it visible. Actual
v3 save/reopen and SBP/manual rates/stale output verified; dense fixture covers
32 operations/24 layers/12 text entries and all section transitions. 1178 frontend
passes/1skip, zero check diagnostics, lint/build pass. [Screenshots and limits](UX4_1_QUALIFICATION.md).
Local uncommitted handoff; human review next, no UX5 or publication.

## 2026-10-14 — UX4.1 accepted; UX5 audit/design checkpoint

User validated UX4.1, closed ivac-si5w within recorded limits, and requested the
next milestone. Executed UX5_PLAN's first checkpoint under ivac-some. Audited live
shell/menu/shortcut/service/operation/tool/generation/export/template/CAD owners,
context and keyboard handling. Found split export guards (GenerateBar aggregate
sim/pipeline/work area versus file service freshness/fixture/native Save), tool
origin ID collisions, ID-bound saved rates and inventory-editor first-use seeding.
Proposed owner-bound shared dispatch, target revalidation, origin-aware read-only
search and explicit qualified assignment/template navigation. Kept mutation
history and CAM/FS/post/state formats untouched. Source inventory/matrix,
search/ranking/numeric corpus, prospective latency budget, focus design and concept
SVG provided in [UX5 audit](ux/UX5_AUDIT.md); ADR-0016 remains Proposed.143 existing
seam tests pass in12 files; no palette implementation/runtime/latency claim.
Architecture/design gate ivac-2zh9 is the next action before coding, per the plan.
No publication. User requests1–2 completion lines describing the next milestone.

## 2026-10-14 — UX5 command palette software handoff

User accepted the concrete UX5 audit/design (ivac-2zh9) and authorized implementation
(ivac-0vbt). Added Cmd/Ctrl+K and menu/Welcome entry; prepared local command/tool/
template search with numeric/alias/typo ranking and source-qualified identities.
Disposable owner bindings recheck context and availability, preserve history/discard
handling, and route generation/export through GenerateBar's complete shared policy.
Tool inspection has no mutation; explicit assignment checks stock/kind/mode/saved
roles. Templates reuse UX4.1's binder and existing manual slots/project formats.

1220 frontend tests pass/1skip; check0 errors/warnings, lint/build and diff check pass.
Browser verifies focus, no nested palette during discard confirmation, cancel leaving
job intact, legacy assignment Undo, saved-role/inventory-collision refusal, history
actions, actual ShopBot export, stale/critical blocking and template Save/reopen.
5250-record synthetic metadata corpus: cold7.838ms, maximum query p951.303ms;
measurement excludes UI paint/CAM/native dialogs. Actual captures and saved fixture
in [qualification](UX5_QUALIFICATION.md). No new Rust/FS/CAM/post or physical/native
certification/publication. Human gate **ivac-txve** is next; later milling/V-carve
qualification needs its scoped continuation after UX5 review.

## 2026-10-14 — UX5 discoverability presentation polish

Human UX review found the Welcome palette hint competing with the three primary
actions. Removed that element and unused Welcome callback while preserving A1
branding/content/layout. Kept the existing File/Project menu as the sole launcher,
with Command Palette… label and a display-only platform shortcut helper. Expanded
the existing Help definition with result navigation, execution and Esc. No second
shortcut handler, registry/search/availability/owner/history/safety changes.

Refinement ivac-snsy remains part of UX5; no roadmap or later milestone change.
1232 frontend tests pass/1skip, check0 diagnostics, lint/build/diff check pass.
Actual browser verifies five document/selection contexts, keyboard menu/focus,
field/modal guards and byte-identical before/after Save downloads. Five captures
and native evidence limits are recorded in [UX5 qualification](UX5_QUALIFICATION.md).
Existing human gate ivac-txve remains pending; no publication or next milestone
authorized by this refinement.

## 2026-10-14 — V-Carve Phase A evidence audit

User supplied a complete software qualification scope. Audited perimeter and sampled
medial-axis modes, V-bit model, ratchet emitter, normal CPS strategy and ShopBot
fail-closed gates, conical simulation and inspector/persistence wiring. 26 targeted
existing core tests pass. Retained read-only audit probe reproduces a skipped pass
limit, a ramp point 0.49 mm below its local target, shallow short-chain completion
without result metadata, and ShopBot rejecting strategy engrave. Recommended bounded
V-Carve-only scheduling replacement to preserve Halfpipe behavior; architecture gate
before production changes per user's instructions. See VCARVE_AUDIT.md. Full
qualification, UI captures and physical validation are not claimed. No commit/push.

## 2026-10-14 — Future Two-Sided Machining & Registration / V-Carve Inlays

Documentation only (ivac-jqq5). Added concise unnumbered future specifications and
roadmap/status/architecture/UX/navigation references, plus V-Carve future-scope note.
Applied documentation, architecture, geometry, validation, ShopBot, CAM UX and shared
Professional Workshop UI skills. Active V-Carve qualification and human HARD STOP
remain the priority; no code edits, next-milestone start, physical cutting or publication.

Read-only audit confirms open ivac-jtkj, rejected genuine CPS/nonzero-XY two-sided
wrapper, inherited stock-centred flip/front-back/native/dowel scaffolding and
approximate conflict tests. Open ivac-13go is distinct regression-contract work.
Closed inherited inlay ivac-rt1.7 and source-inset/duplicate-as-plug shortcut do not
prove qualified mating geometry or fit; no duplicate feature issue created.

Plans distinguish explicit drawing/stock/side/machine/registration frames and
physical references, explicit pin preparation, independent A/B outputs, female/male
angle/tip/depth/engagement/glue relationships and separate programs. Record audit-first
architecture gates, software matrices and distinct future physical registration/fit
validation. Both PLANNED — NOT IMPLEMENTED / NOT QUALIFIED in their complete scope;
no guard weakening, CAM/post/project/tool behavior or serialization changes by this
checkpoint. Stop pending human approval; current V-Carve work remains independent.

Validation documentaire : nouveaux liens locaux et espaces de fin de ligne vérifiés;
aucun contenu préexistant supprimé dans les documents modifiés. Les empreintes des
sources/configurations/données du début de ce checkpoint sont inchangées. Pas de
nouveau test/build logiciel requis ni de résultat V-Carve revendiqué par ce travail.

## 2026-10-14 — V-Carve targeted planner and software handoff

User approved the Phase A recommendation and continuous-envelope/completion requirements. Added red A1–A3 references before the V-Carve-only replacement, continuous disk-clearance proof, bounded complete sweeps and explicit overcut/undercut errors. Preserved both saved modes, manual values, Halfpipe and geometry/post separation. Narrow ShopBot `vcarve`/V-bit linear XYZ gate qualified by independent A4 decoder and preview comparison. Actual UI review verifies scope hints, stale export, staged-start refusal and Undo; alternate generic depth schedules now explicitly refused. Full medial coverage quantified as partial; Perimeter square island refused. See VCARVE_QUALIFICATION.md and ADR-0017 for test counts, error budget, screenshots and limits. HARD STOP ivac-1b75; no hardware, publication or later milestone.

## 2026-10-14 — GEOM drawing cleanup and lightweight editing roadmap

Documentation only (ivac-gpn7): added shared architecture/source inventory and
GEOM1 HIGH, GEOM2 MEDIUM, GEOM3 FUTURE/OPTIONAL specifications. Retained CAD1 and
existing milestone numbering; no GEOM conflict found. V-Carve now has bounded
software handoff, HARD STOP ivac-1b75; this checkpoint leaves it and other priorities
intact. Proposed GEOM1 after active qualification/preexisting approvals and before
serious WJ integration; GEOM2 after stable shared model; GEOM3 individually scoped.

Read-only source audit distinguishes SVG normalized/transformed paths/common layer
from DXF named layers/units/entity/point conversion and existing short-line filtering.
Reuse shared Segment/import metadata, transforms/transformedImport, CAD1 source
preservation, commands/history and generation identity. Multi-import IDs and relief/
tab/approach bindings require explicit topology-edit handling. CAM region booleans
are not persistent drawing editing. Planned analysis/preview/explicit batch Apply,
unit-specific tolerances, operation-aware diagnostics, numeric primitives and source
protection avoid full CAD scope. Generic cleanup is reusable by waterjet; future
palette integration reuses implemented UX5 authority, with no commands added now.

Shared geometry/architecture/validation/CAM UX/Professional Workshop UI and ProtoMAX
skills applied. Roadmap/status/architecture/future-process/UX/index updated; independent
geometry matrices and lifecycle/accessibility evidence remain future acceptance.
PLANNED — NOT IMPLEMENTED; no production code, importer/schema/post/CAM change,
physical work, commit, push, merge, release or next-milestone start authorized.
Stop pending human approval before any geometry-editing implementation.

Documentation validation: added local links/whitespace checked and preexisting
content retained. Source-baseline comparison detected an independent change to
pipeline/op_drivers/vcarve.rs during this checkpoint; this documentation task did
not edit it. No source-baseline equality or V-Carve test result is claimed here.
No software tests/build required for these documentation changes.

## 2026-10-14 — GEOM2 transformations/drawing split and sheet nesting plan

Documentation only (ivac-ms20). Refined GEOM2 umbrella into GEOM2-A HIGH/MEDIUM
transformations and GEOM2-B MEDIUM primitives, retained GEOM1/3 and existing IDs,
added NEST1 MEDIUM manual layout and NEST2 FUTURE/ADVANCED true-shape optimization.
Current V-Carve ivac-1b75 HARD STOP and existing approved priorities remain intact.
No conflicting new IDs found. NEST1 needs qualified A/reliable source preconditions,
not all B; NEST2 builds on NEST1. GEOM3 may progress independently per prerequisites;
drawing mirror moves to candidate A/subphase, not duplicated advanced scope.

Read-only source audit identifies per-import TS transforms and Rust segment helpers,
not completed selected-part/instance editing; containment nesting is not sheet packing.
Plans separate drawing/source edit, coherent part and placed instance, sheet/stock and
CAM references, transform order/WCS, nonuniform arc policy, stable identities/history/
persistence and stale-generation behavior. Require GEOM2-A/NEST1 architecture reviews,
NEST2 algorithm/license/performance research, independent collision/metric proofs,
explicit optimizer acceptance and manual control. No schema or library selected.

Process-independent placement remains distinct from milling cutter/tab/lead/hold-down
or waterjet kerf/pierce/quality constraints. Common-line cutting is a separate future
extension; multi-sheet phased later. Shared UI/CAM UX plus ProtoMAX guidance used.
Roadmap/status/architecture/geometry/UX/future-process/index updated. All capabilities
PLANNED — NOT IMPLEMENTED; no production/import/project/CAM/post change, new
palette command, GitHub issue mutation, physical test, commit/push/merge/release.
Stop pending human approval before implementation.

Documentation checks: added local links and whitespace verified; prior history and
architecture/context text retained. All baseline source/config/data hashes unchanged.
No software tests/build or new qualification claims for documentation-only work.

## 2026-10-14 — V-Carve final UX and diagnostics polish

Human screenshot review authorized a bounded refinement (ivac-8d21). Read-only start, contextual unsupported saved-value list with explicit undoable clearing, inactive/unqualified inlay shortcut, restricted source-inset help, clearer cap message and operation-context warning rows. Existing three warnings audited and severities preserved. Structured V-Carve refusals remain visible until dismissed. Legacy zero finish-step/negative through-depth wire filtering now forwards values to existing rejection; no planner/post modification. 1241 frontend/1skip, 10 V-Carve, 15 Halfpipe, 7 CPS guards pass; check/lint/build pass; nine core files hash-identical. See VCARVE_FINAL_UX.md and A–G captures. Visualization deferred ivac-fqp3; final human gate ivac-1b75 open. No hardware or publication.

## 2026-10-14 — PERF1 CAM performance roadmap (documentation only)

Added planned PERF1-A profiling/benchmarking and separately gated PERF1-B optimization
(ivac-yrk8), dedicated plan and roadmap/status/architecture/UX/index references.
Current V-Carve numerical/final UX software handoff and human ivac-1b75 stay intact;
propose A near-term after acceptance without displacing approved GEOM/NEST/CAM work.
User-reported approximately 5–10 s small-text generation is informal, not a baseline.

Source inventory consolidates existing core DXF/Criterion and CPS/GRBL benchmarks,
scoped V-Carve topology wall/RSS evidence and separate UX5 search benchmarking.
Historical numbers/qualification are untouched. Plan actual pipeline/transport/render
attribution, equivalent backend/build/cache comparison, repeated median/variability,
CPU/memory/counts, ranked bottlenecks and measured targets before any optimization.
Existing generation/progress/cancel ownership and open global-cache race ivac-htc8
are audit requirements. Candidate indexes/caches/parallelism remain hypotheses;
continuous predicates, sampling/error budgets, pass completion, fail-closed behavior,
determinism, source mapping and ShopBot final-output parity cannot be weakened.

Canonical documentation/architecture/geometry/validation/CAM UX/Professional Workshop
UI and ShopBot skills used. Both phases PLANNED, not implemented/qualified. No
instrumentation/benchmark run, production algorithm/tolerance/preview/post/FS change,
new dependency, GitHub issue mutation, physical validation, commit/push/merge/release.
PERF1-A report ends at human HARD STOP before B. Stop awaiting approval to start A.

Documentation validation: new local links and whitespace checked, previous text
retained; baseline source/config/data and historical benchmark/V-Carve qualification
hashes unchanged. No software tests/build or performance measurement claimed here.

## 2026-10-14 — V-Carve strategy intention clarification

User requested a focused presentation follow-up to the qualified V-Carve planner
(ivac-klww). Verified actual mode, radius/depth0-auto and incremental-pass semantics.
Replaced algorithm-named segmented buttons with a compact full-width native select
and selected-result explanation. Outline groove retains Final depth; Variable-depth
engraving uses Maximum carving depth. Authored depth/step/other fields remain; the
ordinary fullMedialAxis patch, defaults and serialized identifiers are unchanged.
Local key isolation prevents the operation list intercepting select navigation.
No illustrations or domain rewrite; warning policy and inlay refusal remain.

1249 frontend passes/1skip; check/lint/build pass. V-Carve10, Halfpipe15 and
ShopBot guards7 pass. Nine qualified core/post hashes match, and actual before/
after OpenSBP programs are byte-identical for both strategies. Browser checks
UI history/save-reopen, Advanced/warnings and280/360/550px sidebars; native popup
keyboard navigation is left explicit for human confirmation. [Evidence, mapping,
captures and limitations](VCARVE_STRATEGY_UX.md). Existing gate ivac-1b75 remains
pending. No new CAM milestone, publication or future work started.


## 2026-10-14 — Geometry source and contour interpretation UX refinement

Audited actual wire precedence/empty fallback, imported drawing versus layer versus
chained-contour identity, SVG single-layer flattening, DXF named layers, equal-name
multi-import merging, text/stock sources and consumer-specific booleans. Extracted
GeometrySourceSection from the crowded shared inspector; retained existing commands,
CSS, selectors and enum values. Added contextual wording/provenance, saved multi-layer
presentation, pending unbound choices without source mutation and one-command binding.

Normal Pocket Auto and the shared region kernel differ for deeper selected nesting;
XOR ends on empty intermediate output. Recorded separate ivac-dli7/ivac-j2po rather
than changing CAM. Deferred authoritative source highlighting as ivac-9oty; no extra
frontend resolver or conceptual diagrams. GEOMETRY_SOURCE_UX.md records the exact
architecture/limitations and visual review requirements.

1260 frontend passes/1skip; check0 diagnostics; lint/build pass. Seven new core audit
fixtures include real standalone SVG/DXF imports; combine9/selection9/offset54/input41
reported passes (two optional DXF corpus early returns documented), V-Carve10,
Halfpipe15 and ShopBot7 pass. 120 protected production files unchanged; actual SBP
bytes equal to earlier output for both V-Carve strategies. Browser validates source
mode and layer changes, Undo/Redo, duplication, actual Save/reopen, missing-source
critical export refusal and 1280×720/1366×768 layouts with narrow panels. Native popup
windows/arrow-key navigation remain manual acceptance checks.

Engineering ivac-uebs complete; focused human review ivac-xy3w pending. Existing
V-Carve acceptance ivac-1b75 remains open and unchanged in scope. No publication,
new CAM milestone, physical/native qualification or geometry fixes performed.


## 2026-10-15 — PERF1-A profiling and reproducible baselines

Human software acceptance closes Geometry UX ivac-xy3w, V-Carve ivac-1b75 and UX5
ivac-txve with existing limits. Published snapshot2032848 pushed before this phase.
User authorizes PERF1-A and explicitly chooses saved Test as representative, not
the unknown reported5–10-second case. Added opt-in native spans/counts, version-bound
frontend console endpoints and fixture/native/WASM/summary developer harnesses.
No CAM algorithm, tolerance, cache policy or post optimization.

18 native request/post cases each measured control/profile cold+warm,9 generic
WASM cases plus explicit unsupported CPS, actual production HTTP UI, imports,
CPU/RSS and cancellation. Control Test CPS warm904ms; profiled post98.4% of total.
Pocket remaining-material analysis83%; many-segment CPS99.6%. Browser first frame
median935ms, simulation2310–4051ms including existing debounce. CPS cancellation
inside post waits another730–739ms; no partial output. Exact native output hashes
match instrumentation control; browser SBP matches native; generic WASM programs
match (one preview-coordinate difference2.84e-14mm). Refusals and empty-source
diagnostics are separate, not speedups. Debug timeout/late result is exploratory.

1262 frontend passes/1skip;1075 core unit passes serial; geometry-source7,
V-Carve10, ShopBot7 pass; check/lint/build/strict fixture TS/Clippy/rustfmt pass.
[PERF1_A_RESULTS.md](PERF1_A_RESULTS.md) retains methods, raw evidence, build limits
and proposed numerical B targets. Engineering ivac-sgnd complete; human report/
scope gate ivac-xzys open, HARD STOP before B. Existing CPS reuse ivac-rnsg gets
evidence; cancellation ivac-sxmw and simulation attribution ivac-lf1e deferred.
No new physical evidence, optimization, commit or push in this phase.

## 2026-10-15 — PERF1-B1 implementation after explicit user approval

Following PERF1_A_AUDIT, human A gate ivac-xzys accepted; B1 ivac-rnsg implements
bounded, process-local, exact-input bundled ShopBot PostOutput reuse in the existing
PipelineCache. ADR-0018 records the boundary and limits. Evidence helpers now
reject mixed outcomes, verify request/response hashes, expose absent comparisons,
and avoid truncated path-delta claims. Frontend opt-in diagnostics separate debounce,
configuration, replay, LOD and first rendered relief. No geometry/post semantics,
Pocket analysis optimization or physical qualification. Qualification underway;
HARD STOP before any further milestone or publication.

### PERF1-B1 software handoff — 2026-10-15

[Results](PERF1_B_RESULTS.md): exact bounded bundled ShopBot reuse qualified.
Native repeat Test832→15ms; HTTP first frame899→141ms; relief2259→1516ms.
Full output equality, refusal/invalidation/cancellation/cache-bound tests and
frontend/core/ShopBot/V-Carve gates pass. Simulation boundaries now measured;
no simulation algorithm change. ivac-rnsg/ivac-lf1e closed; **ivac-sf7y OPEN**
for human review. HARD STOP before B2/further work; no publication/physical claim.

### PERF1-B2 authorization and implementation — 2026-10-15

User accepts B1 and authorizes continuation; ivac-sf7y closed. B2 ivac-dlrm
extends PipelineCache with bounded exact-input remaining-material kernel reuse.
Live eligibility/stock/provenance and diagnostic checks retained. ADR-0019
records the boundary; paired benchmarks, output equivalence and regression
qualification in progress. HARD STOP for human results review, no publication.

### PERF1-B2 software handoff — 2026-10-15

[Results and limits](PERF1_B2_RESULTS.md): pure remaining-material kernel reused
with live guards. Pocket repeated analysis337.845→0.222ms, total338.370→0.625ms;
first/new input still computes. All18 native outcomes unchanged; WASM parity,
1085unit/38integration tests and actual UI freshness/overlay reviewed. Small
CAM4 UI case has no material latency gain; no universal end-to-end claim.
ivac-dlrm complete, **human gate ivac-s5xy OPEN — HARD STOP**. No publication or
physical claim. Bounded CPS interruption ivac-sxmw remains separately deferred.

### CPS cancellation follow-up — 2026-10-15

User accepted B2 and authorized ivac-sxmw. [Results](PERF1_CPS_CANCELLATION_RESULTS.md):
existing CPS host probe now reads real atomic cancellation; no partial successful
output or successful-cache insertion. Native Test750→213ms, dense contour1181→193ms,
HTTP743→207ms medians. Target p95≤200ms is **not met** (216/218ms Test native/HTTP).
44 CPS/1086 core unit tests pass, all18 full output/refusal outcomes unchanged,
UI cancel/export-disabled/retry verified. An obsolete two-sided CPS integration
expectation was corrected to assert the existing CAM2 refusal, not remove it.
Review gate ivac-nh6q OPEN; remaining latency attribution ivac-q1nz1 deferred.
HARD STOP before any further implementation, publication or physical claims.


## 2026-10-15 — Visual hierarchy and contrast continuation

User requests presentation-only polish under Professional Workshop UI. Actual
1366×768/1024×768 baseline captured first. Four CSS-only source changes: derived
navigation and section-text tokens, existing-context border cue and smaller
context prefixes, dark minor grid38→34 gray. Keep toolbar/sidebar/canvas surfaces,
active/selected styling, spacing, warning colors and grid threshold/math.

[Qualification](ux/VISUAL_HIERARCHY_REFINEMENT.md) retains before/after welcome,
workspace, inspector, warnings, three grid views and small-desktop captures.
Long-name stress, keyboard focus/navigation, sticky header, real drawing selection,
Save, stale edit/Undo/Redo, critical export refusal and incompatible V-Carve alert
checked. Critical Download remains visually enabled after settling in the existing
design, with the action guard refusing export; stale export is actually disabled.
No export-policy change. Original theme and viewport restored.

775/779 captured source files unchanged; four exceptions CSS only, Svelte non-style
sections identical.1263 frontend passes/1skip; check zero errors/warnings; lint/build
pass. No CSS-mirroring tests or CAM requalification added. Design direction,
UX2.2 qualification and STATUS updated; functional ROADMAP untouched. Engineering
ivac-28e92 complete; visual review ivac-1drzo OPEN, STOP. Existing PERF gates,
including cancellation ivac-nh6q, remain unchanged. No commit/push/release.


## 2026-10-15 — Drawings and layers UX clarification

User requests focused imported-file/layer clarification. Pre-implementation audit
finds exact-name layer union, first metadata color, view-only visibleLayers,
read-only swatch inside the old checkbox label, and an incorrect tooltip claiming
CAM inclusion. Portable saves intentionally omit visibility preferences; workspace
view-state/legacy defaults remain unchanged. Layer deletion removes real geometry
across imports; file/layer undo and visibility pruning have separate boundaries.

Consistent Drawings/import count, Imported files versus Layers grouping, eye/off
buttons, passive 14 px swatches, view-only CAM reminder, mixed master indication,
provenance tooltips and narrow count formatting. Reuse existing ACI resolver
(including 9), not a new palette. Preserve file transform markup/handlers and
file-removal availability. Add accurate empty-layer state when file records remain.
English/German catalogs and generated keys updated; no dependency/model migration.

[Qualification](ux/DRAWINGS_LAYERS_CLARIFICATION.md): paired actual screenshots,
240px sidebar, keyboard focus/Space, aggregate 12/4 counts, 1535-move generation
with imported layers hidden, stale export after deletion and Undo restoration.
Actual Save/reopen retains exact imports/colors/transforms and operation source
references; portable all-hidden retention is not claimed. Seven new actual
state-command/CAM-identity/data tests; 1270 frontend passes/1skip; check/lint/build
pass. 774/779 incoming existing source files unchanged; exceptions are presentation,
locales and generated keys, plus new UI helper/tests. Core/state/serialization
untouched. No physical qualification or numerical expectation changes.

Engineering ivac-yjcv2 complete; human visual review ivac-ena7m OPEN — STOP.
Functional ROADMAP, earlier visual gate ivac-1drzo and PERF gate ivac-nh6q unchanged.
No commit/push/merge/release or unrelated next work.

## 2026-10-15 — UX6 Machine Profiles & Tool Library roadmap planning

Documentation only (ivac-afey9): added shared UX6 plan/preliminary ownership inventory
and UX6-A/B specifications, roadmap/status/architecture/UX/index and FabSBP boundary
references. UX6 numbering available; Phase 0 field/action matrix + architecture/layout
approval precedes A, A human gate precedes B, B ends at review. Current visual and PERF
cancellation reviews/remaining latency objective and existing priorities stay intact.

Source inventory confirms profile selection currently applies immediately; named
workspace bundles copy machine/tools/reference into project with history, then mirror
project edits. Header reads project machine.name while profile names may be generated;
this is a possible explanation, not proof of screenshot defect. Inventory Apply syncs
same-ID stocked copies; workspace and project history have different scopes. Preserve
embedded snapshots, local-only copies, FS saved/manual rates/fallbacks and stale/export
preflight. Full field effects/classification, browse/edit/assign model, delete/import
relationships and exact persistence semantics require Phase 0, not guessed from labels.

Plans cover compact grouped machine editor, post-aware output/limits/estimates, profile
assistant with honest evidence, searchable tool/contextual type editor, optional actual-
dimension 2D schematic, secondary legacy defaults and explicit machine associations.
Program output declarations remain FabCAM; live execution/setup control remains future
FabSBP. Canonical documentation/architecture/CAM UX/Professional Workshop UI/validation/
feeds-speeds skills plus ShopBot boundary guidance used. No new rate/association model,
autosave policy, importer/project format, FS/CAM/post/optimization/UI change, physical
validation, GitHub issue mutation or commit/push/merge/release. Both A/B PLANNED — NOT
IMPLEMENTED. Stop before UX6 audit/implementation pending human approval.

Documentation checks: new links/whitespace verified, existing document history and
current state retained; baseline source/config/data hashes unchanged. No tests/build
or new software/physical qualification required or claimed for documentation only.

## 2026-10-15 — Drawings/Layers accepted; UX6 Phase 0 audit delivered

User accepted ivac-ena7m and requested next UX checkpoint. Completed ivac-4df39:
[UX6 Phase 0 audit](ux/UX6_PHASE0_AUDIT.md), 30 machine + 48 tool fields traced
through persistence, adapters and actual consumers; draft/action/history and same-ID
copy/mirror contracts; static machine/tool layout sheet and A/B compatibility plan.
Current UI captures include actual dirty machine draft surviving external project
Undo, then restored with Revert in a blank unlinked project. No authored files or
workspace profile/inventory changes. Existing source comments conflict with actual
jerk, laser-lead, compression-warning and kerf consumers; discrepancies recorded,
not numerically changed. Proposed inspection/assignment and edit scopes need approval.

140 targeted existing tests pass (8 files); no production source changes relative
to incoming dirty tree, no new schema/domain/FS/post/CAM behavior. Static SVG rendered
and visually reviewed; not an implemented screen or packaged native qualification.
Human gate ivac-tjgt1 OPEN — HARD STOP before UX6-A. Other visual/PERF/UX2.2 gates
unchanged. No commit/push/PR/release. After A's independent review, B covers contextual
tool inventory, geometry/association clarity and secondary legacy defaults.


## 2026-10-15 — UX6-A machine profile workspace

User accepted Phase 0 and authorized A. Implemented independent searchable profile
inspection, explicit reviewed project assignment, active/inactive draft ownership,
conflict detection after external changes/Undo, staged creation and machine-file I/O.
Grouped General/output/tools/estimation fields with compact numeric inputs and honest
ShopBot/unknown-limit/jerk context. Existing shared project commands and formats retained.
[Results](ux/UX6_A_RESULTS.md): 1277 frontend passes/1 existing skip, check/lint/build pass;
actual browser desktop scope/history/stale-output/conflict/creation/keyboard/narrow review.
781-file baseline check: 8 existing frontend files changed, two helper/test files added;
no core/post/schema edits. No packaged Tauri or physical claim, commit or publication.
Engineering ivac-jhfns complete; human gate ivac-osa4z OPEN. HARD STOP before UX6-B;
independent hierarchy/PERF gates unchanged.


### UX6-A review polish — 2026-10-15

User requested clearer section selection and library-add action. ivac-26854 fixes
button-style specificity, adds a persistent accent underline, and gives Add tool
from library a Lucide Plus and 12px list separation. Actual UI reviewed with focus
away from the section; inventory disclosure works. 1277 frontend passes/1 skip,
check/lint/build pass. [Evidence](ux/ux6-a/tools-polish.png). Existing A human gate
retained; no UX6-B, domain behavior or publication.


## 2026-10-15 — UX6-B tool library redesign

User accepted A and authorized B. Replaced multi-column editable table with a compact
searchable list and selected contextual detail while preserving whole-library drafts.
Added explicit same-ID/project-operation review, staged replace/add import for both
entry scopes, external-tool-change conflict guard, copy availability and separate
library deletion. Existing inventory/project commands, history, post guards and saved
operation rates retained. Type changes preserve unknown/hidden fields; auto naming
no longer invents an unentered conical angle. Optional schematic not implemented.
[Results](ux/UX6_B_RESULTS.md): 1284 tests pass/1 existing skip; check/lint/build pass;
actual desktop generation/stale export/Undo/conflict, imports, copy removal, type,
unknowns, keyboard/filter and 1024px review. Software only; no packaged native or
physical qualification, Git publication or next milestone. Engineering ivac-f8w26
complete; human gate ivac-y2aln OPEN — HARD STOP. Independent earlier gates unchanged.

## 2026-10-15 — PERF1 and UX source publication authorized

User explicitly requested committing and pushing all pending work to `main`.
Publication includes PERF1 profiling/cache/cancellation improvements and evidence,
visual hierarchy, Drawings/Layers clarification, UX6 machine profiles and tool
library work with their qualification records. Source publication does not close
pending human gates, qualify native/physical behavior, satisfy the remaining CPS
cancellation latency target or authorize another milestone or binary release.

## 2026-10-15 — UX6-B Availability visual harmonization

User screenshot prompted focused correction ivac-b54jh: document-sized paragraphs/lists
replaced by scoped compact inspector presentation in ToolLibraryDialog. Existing script,
messages/conditions/actions verified unchanged; wrapping action row and quiet copy notice.
Actual before/after and1024px captures in ux/ux6-b; keyboard focus checked. Svelte zero
diagnostics, lint/build pass and32 targeted tests pass. No inventory/profile/association
mutations, core/FS/CAM/schema changes or publication. UX6-B human gate ivac-y2aln open.

## 2026-10-15 — UX6-B accepted

User accepted tool-library UX6-B and Availability polish; ivac-y2aln closed as
software/visual acceptance. No new milestone launched. Roadmap has no defined UX7;
GEOM1 Drawing Cleanup & Repair (high priority) proposed as next geometry checkpoint,
beginning with architecture/tolerance/reference audit. Exact scheduling remains
open against existing CAM/PERF priorities; independent reviews and physical/native
limits retained. No new publication authorization.

## 2026-10-15 — Adaptive2D grid and central-button3D pan

Focused user request ivac-tgxsn: grid decimal spacing adapts to zoom instead of
vanishing under6px/mm; origin-anchored screen-space drawing bounds line count.
3D OrbitControls middle button pans; left orbit/right pan/wheel zoom retained and
help updated.42 targeted tests pass, Svelte0 diagnostics, lint/build pass. Actual
browser demo zoom out/in, fit,3D and help checked; central drag mapping source-
verified but gesture not automated (CUA drag has no button parameter). No CAM/snap/
file-format or machine/tool changes. GEOM1 not launched; no new publication.


## 2026-10-15 — GEOM1 launch and architecture audit

User selected GEOM1 after UX6-B acceptance. ivac-cbgkk audited imports, source
identity, global CAM rechaining, transform pivots, relief, history, watcher and
persistence. GEOM1_AUDIT.md and proposed ADR-0021 recommend a preserved original
plus edited snapshot, stable source binding and guarded format before bounded
repairs. 86 existing frontend and 7 core source tests pass (compiler cache disabled
after sccache permission failure). Human decision ivac-rcmta. No production code
or user files changed.
HARD STOP at the architecture decision; no GEOM2 or publication authorization.


## 2026-10-15 — GEOM1 approved architecture and bounded software delivery

- Integrated all four ADR-0021 refinements before structural work. Implemented pure
  Rust drawing identity/provenance/lineage/evaluation, conservative line repairs,
  exact fallback topology predicates and one shared source-to-CAM adapter.
- Added explicit preview/Apply/reset with coherent history and source-review guards;
  actual screenshots led to changed-endpoint framing and compact styled controls.
- Guarded v7 preserves originals and compatible data; old reader/endpoint refuse.
  Final review closed reset-after-source-change, original operation-ID reuse and
  orphaned binding downgrade counterexamples. No automatic rebase or guessed binding.
- 1198 full core / 23 final focused (overlapping) / 44 CPS / 1312 frontend tests pass
  (1 existing skip). Check/lint/clippy/schema/build and compiled WASM/native parity
  pass. Actual UI save/reopen/generate/output download, Undo/Redo, stale export,
  pending preview cancellation and narrow keyboard review captured.
- See GEOM1_RESULTS/CONTRACT/SERIALIZATION and geom1/checks.json/logs for exact
  scope, commands and limitations. Engineering ivac-8rura complete; human gate
  ivac-e2jnb open. No physical validation, commit/push/merge/release or next milestone.

## 2026-10-15 — GEOM2-A1 precise imported-file sizing plan

Documentation only (ivac-hx3xq): early GEOM2-A subtask, existing numbering/priorities
retained. Added GEOM2_A1_PLAN, parent/shared/roadmap/status/architecture/UX/index
references. Source audit confirms per-import X/Y/angle/uniform Scale/mirror controls,
legacy TS transforms and authoritative edited Rust drawing with original pivot under
ADR-0021. Current GEOM1 software/UX human ivac-e2jnb remains open/next.

Planned W/H derive from effective local whole-import extents before rotation and one
persisted scale; no contour-selection/project bbox substitution or shadow size storage.
Accepted local edits change measured bounds without redefining original pivot. Preserve
translate/angle/mirror and normal scale displacement, bindings/lineage/history/v7,
source membership and CAM freshness; no physical tool/rate/FS change. Empty/point/line,
curve extrema, small/large units and rounding/drift matrix requires independent proof.
MEAS1 not found in current planning; recorded requested future residual measurement
scope, distinguishing existing status selection dimensions from editable file sizing.

Documentation/architecture/geometry/validation/CAM UX/Professional Workshop UI applied.
PLANNED — NOT IMPLEMENTED; no code, source rewrite, GEOM1 repair, format/post change,
new GitHub issue, commit/push/merge/release or next milestone authorized. Stop pending
human approval before implementation.

Documentation checks: added links/whitespace and retained prior content verified;
baseline source/config/data hashes unchanged. No software tests/build or new
qualification evidence for this documentation-only checkpoint.


## 2026-10-15 — Release-oriented roadmap proposal (documentation only)

User requested a shorter release-led route: FabCAM 1.0 ShopBot milling, 1.5 ProtoMAX,
2.0+ advanced manufacturing. Read current STATUS, historical ROADMAP/DEVLOG, ADRs,
CAM/V-Carve/GEOM1/FS/UX6/PERF records, machine/process architecture and packaging
configuration/guides. Applied project-documentation, ivacam-architecture,
cam-validation, cam-geometry, shopbot-opensbp, protomax-waterjet, feeds-speeds,
cam-ux-ui and professional-workshop-ui skills. No architecture or code changed.

Added docs/releases/RELEASE_ROADMAP.md, FABCAM_V1_SCOPE.md and FABCAM_V1_5_SCOPE.md;
prepended concise release summary to ROADMAP while retaining its prior text verbatim.
Updated STATUS, documentation index and FUTURE_PROCESSES placement note. Classified
existing milestone families with rationale/remaining effort, linked evidence and
separate software/human/physical status. Preserved narrow M4 r2 physical acceptance;
current release workshop validation remains pending. Identified existing tool-identity,
nested Pocket/XOR, text-reopen, RPM-refusal and package-freshness issues for triage,
without asserting every old ticket still reproduces or weakening refusal policy.

Proposed macOS Apple Silicon first (human confirmation required), conditional bounded
V-Carve/surfacing/relief and GEOM2-A1, minimal workshop/FS4 campaign and early WJ
output-contract research. Defined explicit release gates, dependency sequence and
scope-change control. Next recommendation is V1 Scope Freeze & Release-Blocker Triage,
not immediate implementation or a new large feature program.

Verified local Markdown links, preserved historical roadmap payload and whitespace.
No test/build rerun for this documentation task; cited counts are prior evidence.
Local Beads ivac-7m4al tracks this proposal; no existing human gates closed and no
GitHub issues created/modified. Working-tree source preserved. No release branch,
version bump, commit, push, merge, publication or physical action.

HARD STOP — waiting for human approval of the FabCAM 1.0 / 1.5 release roadmap.


## 2026-10-15 — Roadmap approved; Windows and Linux required for V1

User approved the release roadmap with explicit Windows and Linux inclusion alongside
macOS. Supersedes the initial optional Windows/Linux and macOS-first recommendation.
Updated release strategy, V1/V1.5 scope headers, acceptance/packaging matrices,
dependency graph, ROADMAP, STATUS, index and future-process placement note.
All three desktop platforms require actual installation/upgrade/runtime and supported
ShopBot workflow qualification; exact OS/architecture/distribution/package variants
remain to be pinned. Configuration/build availability is not platform acceptance.

Independent GEOM1/FS/UX/cancellation and physical gates remain unchanged. Next
checkpoint remains V1 Scope Freeze & Release-Blocker Triage, not started here.
Documentation-only checks: links/whitespace and removal of superseded optional-platform
policy. No code/build/hardware/GitHub/version/commit/push/release action. ivac-zgmds.


## 2026-10-15 — V1 scope freeze and release-blocker triage

Authorized checkpoint ivac-oznar. Recorded three-platform matrix and bounded
stabilization lot in docs/releases/V1_SCOPE_FREEZE_TRIAGE.md, with logs and source
hashes. User confirmed Windows/Linux Intel x64, latest Ubuntu, and GEOM1 not yet
reviewed. Official Rust/WebView2 compatibility research identified Windows 7 conflict
with current standard build; user explicitly selected Windows 11 x64 and finding a
test environment. Ubuntu latest pinned to 26.10 using Canonical announcement.

Executed 7 geometry-source, 22 frontend and 7 CPS guard tests; XOR characterization
still reproduces wrong empty result. Isolated copied WASM freshness script accepts
incomplete pkg. Static audits retain Pocket nesting, tool shortcut identity and
text warning risks; no claim of complete UI/path reproduction. Proposed bounded
geometry, tool/text and package integrity fixes before multi-platform qualification.
No production code, machine action, installer build, GitHub issue or publication.
Independent gates unchanged; HARD STOP before stabilization implementation.


## 2026-10-15 — V1 bounded stabilization corrections

Explicit user approval, ivac-chhz0. Added failing XOR and deep Pocket emitted-path
counterexamples before fixes; corrected fold continuation and selected-ancestor
parity without changing ShopBot semantics. Added three/four-level and partial-selection
matrix plus independent OpenSBP decoding and fresh native/WASM program parity.
Reproduced and corrected project-tool/inventory ID confusion; actual Apply/Cancel,
Undo/Redo and saved-text/project-tool reopen reviewed. Text-only fit now frames text
with stable placement math instead of the entire machine bed; false no-drawing
warning removed. Production WASM completeness/content-fingerprint guard fails closed,
with locked rebuild and ten guard tests wired into npm test.
[Report, commands, counts and limitations](../releases/V1_STABILIZATION_RESULTS.md).
Known cache-count race ivac-htc8 reproduced in parallel tests; serialized qualification
retained separately. No package installation, physical testing or publication.
Independent GEOM1/human gates remain; HARD STOP before next checkpoint.


## 2026-10-15 — Version 0.9.0-beta.1 and source publication

User requested a maturity-based version update, publication of all pending work,
preview shutdown and debug cleanup. Selected 0.9.0-beta.1 for implemented bounded
V1 scope undergoing stabilization; 1.0 and RC remain gated on integrated acceptance,
three-platform installation and workshop evidence. Cargo workspace and Tauri version
are aligned; browser/window/About inherit the Tauri version at build time. Current
READMEs, release roadmap and scope reference the decision. Historical logs retain
the versions actually tested. No installation identity or project format is changed
by this version bump. Prior GEOM1/stabilization changes are included in publication;
their evidence and remaining review limits are preserved.

Publication checks under 0.9.0-beta.1: 1,313 frontend tests (1 existing skip),
10 WASM guard tests, Svelte/TypeScript (zero diagnostics), lint and Rust formatting
pass. Serialized core library with cps+profiling: 1,087 pass; focused GEOM1,
geometry-source and V1 stabilization integration: 33 pass. Production UI and WASM
rebuilt with the new version; initial input-fingerprint refusal required a retry,
which succeeded with stable inputs. Existing chunk/dynamic-import warnings remain.
Seven identified Vite/API preview services stopped. Removed only Cargo debug build
artifacts from target/debug and /private/tmp/fabcam-cps-runtime/debug (about
12.3 GiB allocated); source, release outputs and qualification evidence retained.

Local optimized macOS app rebuilt successfully (2m23s, 21.73 MiB). Bundle display
name is FabCAM, both bundle version fields are 0.9.0-beta.1 and identifier remains
org.opcenter.ivacam. This build/metadata verification does not replace the pending
installation/upgrade or runtime qualification on the three required platforms.


## 2026-10-16 — desktop candidate preparation

User accepted bounded stabilization and authorized macOS/Windows 11/Ubuntu preparation.
Added manual, read-only-permission candidate workflow with explicit platform targets,
locked build and checksummed provenance. Fixed retired macOS runner, universal rustup
and artifact paths, missing-artifact errors and partial-matrix draft assembly in legacy
release workflow. CLI pinned to npm 2.12.3. Four collector regressions and actionlint pass.
Local arm64 0.9.0-beta.1 app/DMG built; sandbox hdiutil failure resolved by authorized
unrestricted rerun, image checksum VALID, ad-hoc signature limitation recorded.
No CAM changes, platform install/upgrade/runtime claim, commit, remote dispatch or release.
User Windows/Ubuntu hosts unavailable until Monday 19 October. Parent ivac-2p53v
remains open; preparation ivac-2p53v.1 delivered. [Results](../releases/desktop-preparation/RESULTS.md).

### 2026-10-16 — Desktop candidate publication and dispatch

User authorized commit/push and manual build without release. Preparation commit
`5ac4023` pushed via existing SSH; manual run `38863177268` reached compilation on
all three runners. Artifacts and installation remain pending; no tag or Release.
See [dispatch record](../releases/desktop-preparation/RESULTS.md).
