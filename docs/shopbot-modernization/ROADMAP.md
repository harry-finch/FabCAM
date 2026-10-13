# Roadmap and approval gates

Every milestone ends with a human approval gate. Never proceed automatically.

| Milestone | Included work / acceptance | Status |
| --- | --- | --- |
| M0 | Fork/remotes/feature branch; code and feature audit; known build/test baseline; post and future-process assessment; committed/pushed documentation | Complete with recorded environment limitations; see STATUS |
| M1 | Deterministic SVG/DXF corpus; analytical profile outside/inside/on-line, pocket and drill checks; regressions and unresolved issues recorded | Approved, committed/pushed at 7737a2a; see M1_QUALIFICATION |
| M2 | CPS vs native ADR; authoritative OpenSBP research; minimal deterministic post; unsupported semantics explicit; output tests | Approved, committed/pushed at 047a758; see M2_QUALIFICATION |
| M3 | Actual lab machine profile; known values sourced; unknown values marked TO VERIFY ON MACHINE | Approved, committed/pushed at b8a073a; see M3_QUALIFICATION |
| M4 | Human textual review, Preview Mode, supervised air cut, sacrificial machining, measurement, VCarve comparison; no unexpected motion | Qualified through operator-confirmed r2 physical trial and documented VCarve comparison; commit/push approved 2026-10-12; see M4_VALIDATION |

M1 corpus minimum: `square_100`, `rectangle_100x50`, `circle_d50`,
`concave_L_shape`, `rectangle_with_hole`, `rectangle_with_island`,
`multiple_closed_contours`, `open_polyline`. Use a known Ø6 mm cutter and test
depth passes, compensation, pocket islands/concavity/stepover/stepdown and
available allowance; drill depth and peck semantics. Compare intent/invariants,
not byte-identical VCarve paths.

M2 may deliberately restrict output to metric, one spindle and manual changes,
without probing/ATC/advanced cycles. Every emitted command needs verified syntax.
Do not copy third-party CPS source without a compatible license.

M3 records envelope, rapid/feed/plunge limits, spindle/RPM, tool changes, safe Z,
parking, work zero, arcs and controller/software assumptions. Invent no values.

M0–M4 exclude waterjet implementation, nesting, multiple sheets, full CAD, PDF
import, inlays, advanced 3D/adaptive/rest machining, plugin architecture, major
frontend rewrite and direct machine control unless explicitly authorized.

## Post-M4 named tracks

M0–M4 remain the completed ShopBot foundation; no global M5/M6 sequence follows.
M4 baseline: `e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`, qualified at the
operator-reported evidence level documented in M4_VALIDATION.

| Track | Scope | Status / priority |
| --- | --- | --- |
| FS — Feeds & Speeds | Conservative, pedagogical milling assistant; independent engine and CAM adapter | FS0 approved; FS1 approved; FS2 approved; FS3.5 approved; FS3.6 visual handoff ready; Architecture through FS3.8 ready for continued CAM work; targeted FS3.9 approximate matching authorized; FS4 DEFERRED, nonblocking |
| UX — Milling workflow | Priority setup, warnings, tabs/ramps and milling CAM/UX improvements | UX1/CAM2/UX2.1/CAM3 continuation approved; CAM4 A/B human handoff; independent of deferred FS4 |
| CAD — Geometry preparation | Dogbones and geometry preparation; no full CAD rewrite implied | Proposed CAD1 corner relief coordinated with CAM4 Phase C, after CAM2/UX2 and CAM3; separate scope/approval |
| WJ — ProtoMAX / Waterjet | Architecture → process semantics → OMX post → corpus → MAKE validation → physical validation | Deferred after priority milling improvements; replaces former M5–M10 labels |
| VINYL — Drag knife / vinyl | Improvements to existing drag-knife workflows | Deferred, lower priority after WJ |

Intended order: **Feeds & Speeds foundation → priority milling UX/CAM
improvements → ProtoMAX/waterjet → lower-priority vinyl work**. This order
is not authorization to start or to finish all FS milestones before any UX work.
CAD1 corner relief is coordinated with future CAM4 after CAM2/UX2 and CAM3. Each track milestone needs its own approval gate.

## FS — Feeds & Speeds Assistant

The [planning contract](feeds-speeds/README.md) defines subsystem constraints;
[ADR-0002](adr/ADR-0002-feeds-speeds-boundary.md) records independence,
persistence and process separation. FS0/FS1 are approved; FS2 is approved; FS3 has an integration handoff; FS3.5 cleanup is approved; FS3.6 visual polish has a review record; FS3.7 multi-source handoff awaits human review; User explicitly authorized FS3.8 capture/validation workflow; FS4 is now explicitly deferred for unavailable physical lab access, not a dependency of milling software work.

| Milestone | Goal and included scope | Excluded scope | Acceptance criteria | Status |
| --- | --- | --- | --- | --- |
| FS0 — Architecture and data-model qualification | Audit existing models and decide crate/module boundary; RPM/feed/plunge/stepdown/stepover ownership; material; physical Tool vs cutting presets; request/recommendation/resolved models; preset/data schema; precedence and fallback; aggressiveness, evidence, provenance; reproducibility and test strategy | Recommendation algorithms, engine implementation, UI integration | Source-backed ownership/consumer audit; documented decisions and unresolved questions; compatibility/migration and test plan; ADRs where needed; human approval | Approved 2026-10-12; see [FS0 qualification](feeds-speeds/FS0_QUALIFICATION.md) and accepted ADR-0003 |
| FS1 — Pure recommendation engine MVP | Independently testable Rust engine; deterministic chipload/feed relationship; simple machine constraints; operation context and profile/pocket/slotting distinctions; conservative policies; unit tests | Deep UI integration; diagnostic assistant | Deterministic tests cover units, constraints, missing/invalid context, operation distinctions and conservative policies; no UI/geometry dependency; human approval | Approved 2026-10-12; see [FS1 qualification](feeds-speeds/FS1_QUALIFICATION.md) |
| FS2 — Initial data profiles and provenance | Small configurable dataset relevant to fablab materials/tools; documented sources, provenance and evidence levels | Unauthorized proprietary copying; hidden cutting constants inside algorithms | Versioned data with traceable sources and evidence; data validation/loading tests; reference vs local tested/validated profiles distinguishable; human approval | Approved 2026-10-12; see [FS2 qualification](feeds-speeds/FS2_QUALIFICATION.md) |
| FS3 — CAM integration and UX | Adapter supplies known CAM context; simple Machine → Material → Tool → Operation → Recommendation flow; separate advanced view; explicit application and manual overrides | Dynamic post-time recommendation; silent recalculation | Applied values persist across save/reopen independent of engine/data changes; manual edits survive until deliberate recalculation; adapter and round-trip tests; human approval | Integration handoff ready; human approval pending; see [FS3 qualification](feeds-speeds/FS3_QUALIFICATION.md) |
| FS3.5 — Feeds & Speeds UX & Domain Model Cleanup | Job material; operation cutting values; explicit focused assistant; global source settings; physical tool and available-tools-first UI | Engine math rewrite; ShopBot post changes; physical tests; WJ/VINYL | Manual/recommended save-load, no automatic recalculation, clear stale state, source persistence, unchanged output and human usability approval | Approved by user 2026-10-12; [qualification](feeds-speeds/FS3_5_QUALIFICATION.md) |
| FS3.6 — Adobe-inspired UI polish | Shared compact inspector styles, field alignment, hierarchy, compact stale state, restrained actions and persistent visual guidance | FS math/rules/data changes; domain/persistence/post changes; full application redesign; FS4 | Profile/Pocket/manual/suggested/stale/narrow/long names/large values/advanced visual review; accessibility and frontend checks; human approval | Engineering handoff ready; [qualification](ux/FS3_6_QUALIFICATION.md); human approval pending |
| FS3.7 — Multi-source cutting knowledge & source packs | One pack schema/loader, reviewed public registry, private app-data storage, vendor-independent taxonomy, provenance, conservative deterministic resolution and source Settings | Proprietary copying; unlicensed public lab data; blind averaging; private data export/sync; physical calibration; FS4 | Manifest/licensing/storage/taxonomy/matching/reconciliation/privacy/save-reopen tests, license matrix and human architecture/UX approval | Engineering handoff ready; [qualification](feeds-speeds/FS3_7_QUALIFICATION.md); human gate ivac-tm7u open |
| FS3.8 — Local Preset Capture & Validation Workflow | Read-only resolved-value capture into private Sorbonne pack; Draft/Tested/later Validated, duplicate/revision safeguards, management and backup/restore | Mathematics/post changes; direct casual validation; public lab-data publication; FS4 physical work | Actual overrides/context preserved, stable identity/history, existing resolver/storage, unchanged operation, compact repeated-use workflow, tests/docs and human review | Engineering qualification; [record](feeds-speeds/FS3_8_QUALIFICATION.md); human gate ivac-7yii open |
| FS3.9 — Approximate Matching & Coverage Expansion | Bounded normalized diameter matching, explicit reference transfers, within-series interpolation, deterministic explanations and public coverage audit | Physical validation; evidence promotion; proprietary imports; automatic value changes; further FS milestones | Ø6/6.35 Pocket, conservative or refused Ø2/3, six exact rows unchanged, strict families, taxonomy, trace, persistence/privacy tests and compact UX; human approval | Engineering qualified (ivac-02in); human gate ivac-t0fk pending; [handoff](feeds-speeds/FS3_9_QUALIFICATION.md); [policy](feeds-speeds/FS3_9_MATCHING.md), [coverage](feeds-speeds/FS3_9_COVERAGE.md) |
| FS3.9.1 — Recommendation UX Simplification | Compact recommendation dialog, auto unapplied mode preview, accurate derivation copy, warnings/provenance and local capture hierarchy | Resolver mathematics/priority; evidence/storage changes; ShopBot post; unrelated panels | Preview/cancel/manual/Apply/no-match/focus tests, representative desktop screenshots and human visual review | Engineering handoff (ivac-hcea); [qualification](ux/FS3_9_1_QUALIFICATION.md); human gate ivac-anp0 pending |
| FS4 — Physical Feeds & Speeds Validation | Controlled physical tests recording machine/material/tool/operation and observations; local tested/validated presets | Unsupported claims of validation | Documented validation criteria, test conditions/results and preset applicability limits before a validated label; human approval | DEFERRED: requires physical lab / ShopBot access; blocking further software development: NO (ivac-mwry) |
| FS5 — Diagnostic assistant | Possible symptoms: hot tool, melting plastic, chatter, overloaded sound, fine chips, poor finish; possible adjustments to feed/RPM/DOC/stepover/strategy | Any current implementation | Future scope, evidence and acceptance criteria must be qualified and approved before work | Deferred future work |

FS0 was approved and FS1 authorized on 2026-10-12. FS1 adds an isolated library
without changing CAM/project schemas or behavior. FS1 was approved and FS2 authorized on 2026-10-12. FS2 adds an explicit JSON
loader and narrowly scoped licensed external references; it stops at human review.
FS2 was approved and FS3 authorized on 2026-10-12. User authorized FS3.5 cleanup on 2026-10-12; FS3.5 functional approval and FS3.6-only authorization are recorded in the user checkpoint request; User subsequently explicitly authorized FS3.7 phases A–G; its human architecture/licensing/UX review now gates FS4, which remains unstarted. The FS3.6 visual review record is retained separately. WJ and VINYL must not depend on the milling FS engine.

## Git policy

Preserve upstream history. `origin` is the user fork; `upstream` is
`aalarchiv/ivacam`. Work on `feature/shopbot-mvp`. No force-push, automatic
default-branch merge or PR creation. The user's M0 instruction authorizes fork,
remotes, branch, baseline documentation, commits and push. After M0, commits and
pushes require explicit authorization again. These instructions override the
upstream agent files' local-only/main-branch assumptions.


## 2026-10-13 redirection: resume milling software

FS4 remains on this roadmap with **Status: DEFERRED**, **Reason: requires physical
lab / ShopBot access**, **Blocking further software development: NO**. This is not
failure or blocker. FS3.8 architecture supports later resumption without redesign.
FS software expansion is frozen: no further manufacturer datasets, Sorbonne tuning,
recommendation math, physical trials or FS UX redesign in this phase unless an
approved other feature requires a narrowly scoped integration. External sources
remain ReferenceData; Sorbonne evidence requires actual physical tests.

See [milling capability audit](MILLING_CAPABILITY_AUDIT.md) for feature classifications,
priorities, dependencies and verified evidence limits. WJ and VINYL remain deferred.

| Suggested milestone | Scope and acceptance | Status |
| --- | --- | --- |
| UX1 — ShopBot-compatible milling entry strategies and preview/output parity | Profile/cascade Pocket Direct/Ramp/Helix intent; geometric validity before explicit CPS capability resolution; final machine-resolved preview/simulation and SBP parity, requested strategy persistence, direction/tab/lead regressions; no stock/fixture authoring | SOFTWARE QUALIFIED; scoped Ramp/Direct and explicit unsupported requests; [qualification](ux/UX1_QUALIFICATION.md); human review ivac-cb64 approved |
| CAM2 / UX2 — Stock, Origins, WCS & Fixtures | Qualify numeric origin transforms across generic movement/output/preview (ivac-vj11); stock corner vs centered offsets, supported ShopBot zero choices, warnings and obstacle declaration; no automatic avoidance claim | Software-qualified single-sided XY/fixtures; accepted for UX2.1 follow-up; Z/two-sided limits explicit |
| UX2.1 — Direct Manipulation & Collision Feedback | Typed error notifications, compact red critical fixture state and Show collision; 2D fixture XY drag through existing model/history, precise inspector, save/reopen and stale export. No collision math, stock/WCS/CAD drag or schema change | Approved for continuation 2026-10-13; snapping deferred ivac-t68s |
| UX2.2 — Application Shell & Welcome Experience | Audit existing commands; compact header/project status, welcome and distinct workspace empty states, local recents reuse and guarded navigation; authoritative generation/export state; targeted tests, desktop screenshots and human review. No CAM/FS/post or persistence-semantic changes | **IMPLEMENTED locally**, automated/browser evidence; [qualification](ux/UX2_2_QUALIFICATION.md); human/native review ivac-ein1. Explicit Atelier approval ivac-dwdd closed; not a retroactive CAM3 prerequisite |
| CAM3 — Tabs / Lead-in / Lead-out qualification | Broader entry × tabs × leads matrix, intentional residual stock/allowance, direction and safe transitions; retain UX1 parity and requested intent | Bounded software qualification; reproduced holding-tab defects corrected; continuation approved (ivac-wh2m closed); see [CAM3 record](CAM3_QUALIFICATION.md) |
| CAM4 — Remaining Material Analysis / Corner Relief workflow | Diagnostic 2.5D generated-path residual analysis (A), compact preview overlay/diagnostics (B), coordinated dogbone/T-bone assistance (C, retained CAD1 workstream); no generalized simulation or Rest Machining in V1 | A/B bounded software handoff; [qualification](CAM4_QUALIFICATION.md), human gate ivac-tvox; CAD1/C separate scope/approval, not started; [phased plan](REMAINING_MATERIAL_PLAN.md) |
| UX3 — Planar spoilboard surfacing | Bounded flat-cutter stock raster with explicit margins/overtravel, work area/clearance and coverage proof | Proposed later milling (CAM5+ direction); after CAM4/setup qualification; existing UX3 label retained; separate approval |
| UX4 — Reusable operation templates | Named operations with explicit geometry/tool rebinding and saved-value/undo invariants; distinct from FS knowledge packs | Proposed later milling (CAM5+ direction); existing UX4 label retained; separate approval |
| Later milling — V-carve qualification | Dedicated V-carve geometric/output/preview qualification, beyond existing partial evidence | Future scoped milestone after priority CAM work; not started |
| Rest Machining | Smaller-cutter cleanup operation built on reliable remaining-material analysis | Later advanced capability; excluded from CAM4 V1; not designed/implemented |

UX1 is software-qualified and its human gate was approved by the user's instruction
to continue. CAM2/UX2 engineering ivac-69ak is complete and accepted for the requested UX2.1
follow-up. UX2.1 continuation was approved on 2026-10-13 (ivac-svnf closed).
CAM3 continuation is approved. CAM4 A/B bounded analysis/preview is software-qualified;
human gate ivac-tvox precedes broader coverage or separately scoped CAD1/C relief.
Current handoff ends at A/B; later milling is not started.
FS4 is not a software dependency.

### UX2.2 — Atelier implementation and review

2026-10-13: user selected UX2.2, approved the Atelier proposal and explicitly
requested the active machine in the top bar. Design gate ivac-dwdd closed;
implementation ivac-j6ba is locally complete. The [qualification](ux/UX2_2_QUALIFICATION.md)
records welcome, shared file actions, explicit document state, New/Close safeguards,
rollback on failed loads and generation isolation across document changes.
Human/native review ivac-ein1 remains before subsequent milestones. CAM4/FS reviews
remain independent; CAD1, FS4 and physical validation remain outside this scope.
The earlier planning order UX2.1 → UX2.2 → CAM3 does not rewrite already completed
CAM3/CAM4 work. No automatic publication or next milestone is authorized.

### CAM2 / UX2 — software-qualified bounded scope

Effective stock corner editing retains bbox-relative persistence. Single-sided
milling XY work-zero normalization, drawing-frame preview and fixture
box/cylinder/polygon CRUD/history/save/reopen are qualified. Resolved execution
path supplies obstacle warnings and default critical-warning export refusal;
missing holder geometry stays explicit. No automatic avoidance or physical claim.
See [acceptance and limits](ux/UX2_QUALIFICATION.md) and
[coordinate decision](adr/ADR-0008-work-origin-frames.md).

Nonzero Z (ivac-u1ko), genuine two-sided CPS or shifted registration (ivac-jtkj)
remain blocked pending separate qualification. Legacy stock still follows drawing
bounds. CAM3 now qualifies bounded entry × tabs × leads behavior; CAM4
remaining-material A/B analysis/overlay is the current bounded handoff. CAD1/C
corner relief remains separate scope/approval.

### Remaining Material Analysis / Rest Material Visualization — phased CAM4

Diagnostic/visual V1: target removal region minus actual cutter-swept region,
using generated machining geometry rather than input drawing alone. Reliable
stock/reference/depth context (CAM2/UX2) and intentional tabs/lead qualification
(CAM3) precede analysis. Phase A geometry analysis → Phase B toggleable preview
and concise diagnostics → Phase C coordinated CAD1 dogbone/T-bone assistance;
Phase A does not depend on dogbone generation. See [scope, provisional acceptance
and deferred decisions](REMAINING_MATERIAL_PLAN.md). For final machine residuals,
prefer resolved execution path, retaining UX1 ShopBot output/preview parity; audit
generic vs resolved use and representation/tolerances during CAM4. Intentional
tabs/allowances must remain distinguishable from unreachable/unmachined areas.

New or significantly revised UI should normally use both `$professional-workshop-ui`
(shared desktop visual language; installed) and `$cam-ux-ui` (CAM-specific UX).
Reuse shared warning tokens and chosen application icon family; no emoji/new local
color system. Compact amber residual overlay/status, disclosure and workspace
dominance follow [design guidance](ux/DESIGN_DIRECTION.md). Rest Machining is a
separate later cleanup-operation capability, not diagnostic V1. Current bounded
A/B evidence is in CAM4_QUALIFICATION; broader scope/physical accuracy is not claimed.

### Future direct manipulation (deferred)

UX2.1 enables fixture XY dragging only. Future snapping (grid, stock edges/corners,
geometry bounds, work zero and disable-snap modifier), explicit Set work zero click,
stock manipulation and imported-geometry/CAD placement require separate scope and
approval (ivac-t68s). No CAD constraints now. CAM3 continuation is approved;
CAM4 A/B is the current milling handoff. See [follow-up qualification](ux/UX2_1_QUALIFICATION.md).

## Future FabCAM / FabSBP workflow

**DESIGN DIRECTION / FUTURE — unnumbered, not started.** Preserve approved
UX2.1/CAM3 continuation and the existing remaining milling sequence; planned
UX2.2 placement is separately reviewed above. These groups do not authorize
FabSBP development, change current CAM/post behavior or reopen frozen FS work.
See [workflow plan and dependencies](FABCAM_FABSBP_WORKFLOW.md) and
[ADR-0009](adr/ADR-0009-job-intent-machine-setup.md).

| Future group | Checkpoint / included scope | Acceptance direction / excluded scope |
| --- | --- | --- |
| Shared architecture | Shared Machine Profile Model: capabilities, workshop setup, execution workflow | Defined ownership/CAM subset, compatible existing snapshots; no controller details forced into CAM |
| Shared architecture | Job Setup Metadata / interoperability design | Explicit lightweight requirements, independent XY/Z, identity/capability and compatibility handling; no execution scripts or frozen format before this checkpoint |
| FabCAM | Job Origin / Z Reference UX refinement | Clear drawing/stock relationship, separate XY/Z; qualify depth/clearance/output/preview before enabling Z; no physical zero-setting from declarations |
| FabCAM | Standalone setup instructions | Human-readable requirements alongside usable machine output; no FabSBP dependency |
| FabCAM | Job setup metadata export | Conforms to separately approved contract; no hidden coupling or project-format dependency |
| FabSBP | Machine Profile Setup UX | First-time assistant, compact daily inspector, machine defaults and exceptional job overrides; no enormous form |
| FabSBP | Guided Machine Setup | Profile-driven fablab default ON, visible readiness and explicit expert policy; one progressively disclosed workflow |
| FabSBP | Pre/Post Job Workflows | Structured actions/prerequisites/failure behavior; advanced raw machine code secondary, not normal UX |
| FabSBP | Zeroing / Probe Workflow | Physically establish independent XY/Z safely; repeatable fixture automation only after verified home/registration |
| FabSBP | Job setup metadata consumption | Validate requirements against machine/setup; raw .sbp remains usable with manual/generic guided setup |
| FabSBP | Machine-profile validation | Draft/Configured/Software validated/Physically validated lifecycle; guided checks and explicit confirmation for physical motion; evidence required |

Shared profile design precedes consumers/guided policy; Z semantics precede accurate
Z instructions/export; metadata design precedes export/consumption. Standalone
FabSBP generic setup remains independent of metadata. Profile validation and
zeroing safety precede repeatable-fixture automation. All future UI follows shared
Professional Workshop UI plus product-specific domain guidance. Scope and final
milestone numbering require separate approval; no current implementation claim.

### FS3.9 approved sensitivity follow-up

User approved developer-only diameter audit (ivac-v6pf). Completed11997 samples,
three modes and three DOC/limit contexts, deterministic CSV/report and four additional
regression tests. [Measurements](feeds-speeds/FS3_9_SENSITIVITY.md).
No production mathematics changed. Selection/interpolation continuity is the next
proposal for review, not an authorized implementation. FS4 remains deferred/nonblocking.
