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
| FS — Feeds & Speeds | Conservative, pedagogical milling assistant; independent engine and CAM adapter | FS0 approved; FS1 approved; FS2 approved; FS3.5 approved; FS3.6 visual handoff ready, human approval pending; foundation first |
| UX — Milling workflow | Priority setup, warnings, tabs/ramps and milling CAM/UX improvements | Deferred until FS foundation review; individual milestones need scoping/approval |
| CAD — Geometry preparation | Dogbones and geometry preparation; no full CAD rewrite implied | Deferred; scope and scheduling require approval |
| WJ — ProtoMAX / Waterjet | Architecture → process semantics → OMX post → corpus → MAKE validation → physical validation | Deferred after priority milling improvements; replaces former M5–M10 labels |
| VINYL — Drag knife / vinyl | Improvements to existing drag-knife workflows | Deferred, lower priority after WJ |

Intended order: **Feeds & Speeds foundation → priority milling UX/CAM
improvements → ProtoMAX/waterjet → lower-priority vinyl work**. This order
is not authorization to start or to finish all FS milestones before any UX work.
CAD scheduling remains open. Each track milestone needs its own approval gate.

## FS — Feeds & Speeds Assistant

The [planning contract](feeds-speeds/README.md) defines subsystem constraints;
[ADR-0002](adr/ADR-0002-feeds-speeds-boundary.md) records independence,
persistence and process separation. FS0/FS1 are approved; FS2 is approved; FS3 has an integration handoff; FS3.5 cleanup is approved; FS3.6 visual polish awaits human review; FS4 and later remain unstarted.

| Milestone | Goal and included scope | Excluded scope | Acceptance criteria | Status |
| --- | --- | --- | --- | --- |
| FS0 — Architecture and data-model qualification | Audit existing models and decide crate/module boundary; RPM/feed/plunge/stepdown/stepover ownership; material; physical Tool vs cutting presets; request/recommendation/resolved models; preset/data schema; precedence and fallback; aggressiveness, evidence, provenance; reproducibility and test strategy | Recommendation algorithms, engine implementation, UI integration | Source-backed ownership/consumer audit; documented decisions and unresolved questions; compatibility/migration and test plan; ADRs where needed; human approval | Approved 2026-10-12; see [FS0 qualification](feeds-speeds/FS0_QUALIFICATION.md) and accepted ADR-0003 |
| FS1 — Pure recommendation engine MVP | Independently testable Rust engine; deterministic chipload/feed relationship; simple machine constraints; operation context and profile/pocket/slotting distinctions; conservative policies; unit tests | Deep UI integration; diagnostic assistant | Deterministic tests cover units, constraints, missing/invalid context, operation distinctions and conservative policies; no UI/geometry dependency; human approval | Approved 2026-10-12; see [FS1 qualification](feeds-speeds/FS1_QUALIFICATION.md) |
| FS2 — Initial data profiles and provenance | Small configurable dataset relevant to fablab materials/tools; documented sources, provenance and evidence levels | Unauthorized proprietary copying; hidden cutting constants inside algorithms | Versioned data with traceable sources and evidence; data validation/loading tests; reference vs local tested/validated profiles distinguishable; human approval | Approved 2026-10-12; see [FS2 qualification](feeds-speeds/FS2_QUALIFICATION.md) |
| FS3 — CAM integration and UX | Adapter supplies known CAM context; simple Machine → Material → Tool → Operation → Recommendation flow; separate advanced view; explicit application and manual overrides | Dynamic post-time recommendation; silent recalculation | Applied values persist across save/reopen independent of engine/data changes; manual edits survive until deliberate recalculation; adapter and round-trip tests; human approval | Integration handoff ready; human approval pending; see [FS3 qualification](feeds-speeds/FS3_QUALIFICATION.md) |
| FS3.5 — Feeds & Speeds UX & Domain Model Cleanup | Job material; operation cutting values; explicit focused assistant; global source settings; physical tool and available-tools-first UI | Engine math rewrite; ShopBot post changes; physical tests; WJ/VINYL | Manual/recommended save-load, no automatic recalculation, clear stale state, source persistence, unchanged output and human usability approval | Approved by user 2026-10-12; [qualification](feeds-speeds/FS3_5_QUALIFICATION.md) |
| FS3.6 — Adobe-inspired UI polish | Shared compact inspector styles, field alignment, hierarchy, compact stale state, restrained actions and persistent visual guidance | FS math/rules/data changes; domain/persistence/post changes; full application redesign; FS4 | Profile/Pocket/manual/suggested/stale/narrow/long names/large values/advanced visual review; accessibility and frontend checks; human approval | Engineering handoff ready; [qualification](ux/FS3_6_QUALIFICATION.md); human approval pending |
| FS4 — Fablab validation and calibrated presets | Controlled physical tests recording machine/material/tool/operation and observations; local tested/validated presets | Unsupported claims of validation | Documented validation criteria, test conditions/results and preset applicability limits before a validated label; human approval | Planned |
| FS5 — Diagnostic assistant | Possible symptoms: hot tool, melting plastic, chatter, overloaded sound, fine chips, poor finish; possible adjustments to feed/RPM/DOC/stepover/strategy | Any current implementation | Future scope, evidence and acceptance criteria must be qualified and approved before work | Deferred future work |

FS0 was approved and FS1 authorized on 2026-10-12. FS1 adds an isolated library
without changing CAM/project schemas or behavior. FS1 was approved and FS2 authorized on 2026-10-12. FS2 adds an explicit JSON
loader and narrowly scoped licensed external references; it stops at human review.
FS2 was approved and FS3 authorized on 2026-10-12. User authorized FS3.5 cleanup on 2026-10-12; FS3.5 functional approval and FS3.6-only authorization are recorded in the user checkpoint request; FS3.6 human review now gates FS4, which remains unstarted. WJ and VINYL must not depend on the milling FS engine.

## Git policy

Preserve upstream history. `origin` is the user fork; `upstream` is
`aalarchiv/ivacam`. Work on `feature/shopbot-mvp`. No force-push, automatic
default-branch merge or PR creation. The user's M0 instruction authorizes fork,
remotes, branch, baseline documentation, commits and push. After M0, commits and
pushes require explicit authorization again. These instructions override the
upstream agent files' local-only/main-branch assumptions.
