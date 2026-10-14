# ShopBot modernization

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
