# ShopBot modernization

## Release planning — approved roadmap

- [FabCAM 1.0 / 1.5 / 2.0+ roadmap and approval decisions](../releases/RELEASE_ROADMAP.md)
- [V1 ShopBot scope and acceptance matrix](../releases/FABCAM_V1_SCOPE.md)
- [V1.5 ProtoMAX feasibility and acceptance matrix](../releases/FABCAM_V1_5_SCOPE.md)

Roadmap approved 2026-10-15 with macOS, Windows and Linux required for V1.
Existing engineering/physical evidence and independent human gates remain unchanged.

FabCAM is a university-fablab fork of [ivaCAM](https://github.com/aalarchiv/ivacam).
Public product identity and compatibility policy: [ADR-0012](adr/ADR-0012-fabcam-product-identity.md).
First target: ShopBot PRS Gantry, approximately
2011; future target: OMAX ProtoMAX. Windows, macOS and Linux remain required.
The first objective is a correct, reviewable program from simple SVG/DXF using
the existing engine, replacing the lab's VCarve 8 workflow progressively.

## Navigation

- [STATUS](STATUS.md): current handoff and Git baseline.
- [Feeds & Speeds planning](feeds-speeds/README.md): milling assistant contract and FS3.7 multi-source handoff.
- [Source packs](feeds-speeds/SOURCE_PACKS.md), [data license matrix](feeds-speeds/DATA_SOURCES.md) and [FS3.7 qualification](feeds-speeds/FS3_7_QUALIFICATION.md).
- [UX1 qualification](ux/UX1_QUALIFICATION.md): scoped milling entries, output guard and human handoff.
- [ROADMAP](ROADMAP.md): scope, milestone gates and approval policy.
- [Remaining-material future plan](REMAINING_MATERIAL_PLAN.md): CAM4 diagnostic analysis/overlay, coordinated corner relief and deferred Rest Machining.
- [Future FabCAM/FabSBP workflow](FABCAM_FABSBP_WORKFLOW.md): job intent versus physical setup, shared-profile direction and standalone interoperability; no implementation.
- [ARCHITECTURE](ARCHITECTURE.md): verified module map and boundaries.
- [UPSTREAM_ANALYSIS](UPSTREAM_ANALYSIS.md): feature evidence and dispositions.
- [TESTING_STRATEGY](TESTING_STRATEGY.md): build procedure and executed baseline.
- [CAM_VALIDATION](CAM_VALIDATION.md): qualification and physical evidence status.
- [M2 qualification](M2_QUALIFICATION.md): OpenSBP contract, output evidence and limits.
- [M4 validation](M4_VALIDATION.md): software handoff, operator qualification and evidence limits.
- [M3 qualification](M3_QUALIFICATION.md): loadable machine profile and unverified parameters.
- [Sorbonne machine record](shopbot/SORBONNE_MACHINE.md): reported facts and physical verification requirements.
- [M1 qualification](M1_QUALIFICATION.md): corpus, defects, fixes and limits.
- [FUTURE_PROCESSES](FUTURE_PROCESSES.md): deferred ProtoMAX assessment.
- [ShopBot post audit](shopbot/POST_PROCESSING_AUDIT.md): existing extension seams.
- [DEVLOG](DEVLOG.md): engineering history.
- [ADRs](adr/README.md): decision policy.

Repository-local skills live in `.agents/skills/`. Apply only relevant skills.
This documentation supplements upstream `docs/`; it does not replace it.
Evidence is pinned to upstream `9168104b4fc4e02e916740a54bd1e56b5019f662`.
Source presence and passing UI tests do not establish machining correctness.

- FS3.8: [capture qualification](feeds-speeds/FS3_8_QUALIFICATION.md),
  [preset authoring](feeds-speeds/PRESET_AUTHORING.md),
  [local validation policy](feeds-speeds/LOCAL_VALIDATION_POLICY.md).

- [2026-10-13 milling capability audit and redirection](MILLING_CAPABILITY_AUDIT.md): deferred nonblocking FS4, frozen FS expansion and proposed UX1 awaiting approval.

- [CAM2/UX2 qualification](ux/UX2_QUALIFICATION.md): stock corner, single-sided
  XY work zero, fixture authoring and collision/export recovery; human review pending.

- [UX2.1 qualification](ux/UX2_1_QUALIFICATION.md): optional 2D fixture dragging,
  compact critical collision feedback and typed notifications; human review pending.

- [CAM3 qualification](CAM3_QUALIFICATION.md): holding tabs, curved ramps,
  entry/lead-out corrections, output/preview matrix and bounded human handoff.

- [CAM4 A/B qualification](CAM4_QUALIFICATION.md): planar residual geometry,
  fresh 2D overlay and explicit depth/operation/unsupported scope; CAD1/C deferred.

- [UX2.2 application shell and welcome plan](ux/UX2_2_PLAN.md): **PLANNED**,
  command/state audit, welcome/navigation safeguards and future acceptance;
  sequence conflict preserved, human approval required before implementation.

- [UX5 Command Palette & Quick Actions plan](ux/UX5_PLAN.md): **PLANNED — NOT
  IMPLEMENTED**, local command/tool search, future shared registry and guarded
  dispatch; after CAD1/UX3/UX4, subject to separate human approval.

- [UX3 planar surfacing qualification and review](UX3_QUALIFICATION.md): locally
  implemented explicit flat-cutter raster, saved v6 fixture and evidence limits;
  human gate ivac-k4z3 is the next action before UX4.

- [UX4 qualification](UX4_QUALIFICATION.md): local operation models, explicit geometry/tool binding, retained manual values and human review ivac-51gg.

- [UX4.1 refinement](UX4_1_QUALIFICATION.md): focused template workflows, sticky section context, adjacent counters and human review ivac-si5w.

- [UX5 action audit/design](ux/UX5_AUDIT.md): completed first checkpoint; proposed architecture and concept await ivac-2zh9 before palette implementation.

- [UX5 command palette](UX5_QUALIFICATION.md): local owner-bound quick actions,
  origin-aware tools, templates, keyboard/focus and guarded export; human gate ivac-txve.

- [Future Two-Sided Machining & Registration](TWO_SIDED_PLAN.md): inherited foundation,
  open ivac-jtkj, frame/pin/setup/output and separate physical qualification plan.
- [Future V-Carve Inlays](VCARVE_INLAYS_PLAN.md): paired depth/fit geometry and separate
  output after qualified V-Carve prerequisites. Both **PLANNED — NOT IMPLEMENTED /
  NOT QUALIFIED** in their promised complete scope; current V-Carve priority retained.

- [V-Carve software qualification](VCARVE_QUALIFICATION.md): targeted planner, A1–A4, bounded modes, fixtures/screenshots; human review pending.

- [Geometry preparation shared architecture](GEOMETRY_PREPARATION_PLAN.md): future
  source/edit/region separation and generic waterjet-compatible tolerance/identity policy.
- [GEOM1 cleanup/repair](GEOM1_PLAN.md), [GEOM2 basic 2D editing](GEOM2_PLAN.md),
  [GEOM3 optional utilities](GEOM3_PLAN.md): **PLANNED — NOT IMPLEMENTED**;
  active V-Carve review and existing milestones/gates preserved.

- [Geometry transformations GEOM2-A](GEOM2_A_PLAN.md) and
  [basic drawing GEOM2-B](GEOM2_B_PLAN.md) refine the former GEOM2 umbrella.
- [Sheet-layout architecture](NESTING_PLAN.md), [manual NEST1](NEST1_PLAN.md) and
  [automatic true-shape NEST2](NEST2_PLAN.md): **PLANNED — NOT IMPLEMENTED**;
  source/instance/CAM/stock boundaries and independent future qualification.

- [PERF1 CAM performance plan](PERF1_PLAN.md): **PLANNED** profiling/benchmark phase
  A, report/HARD STOP, then separately approved targeted phase B; historical evidence
  preserved, reported V-Carve text latency unmeasured, numerical/output guards retained.

- [UX6 machine/tool management plan](ux/UX6_PLAN.md),
  [UX6-A profiles](ux/UX6_A_PLAN.md), [UX6-B tool library](ux/UX6_B_PLAN.md):
  **PLANNED — NOT IMPLEMENTED**; field/action architecture audit and separate UX gates,
  existing snapshots/FS/output safeguards preserved, no active-priority displacement.

- [GEOM2-A1 precise imported-file dimensions](GEOM2_A1_PLAN.md): **PLANNED — NOT
  IMPLEMENTED**, linked local Width/Height via existing Scale, original GEOM1 pivot
  and source/CAM safeguards; early subtask without selection transforms or nesting.
