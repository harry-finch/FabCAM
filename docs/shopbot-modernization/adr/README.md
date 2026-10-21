# Architectural decisions

[Documentation index](../../README.md) · [Fork architecture](../ARCHITECTURE.md) ·
[Current gates](../../releases/V1_READINESS.md).

All 26 decisions are indexed below. The recorded state is quoted from each ADR,
including historical pending-review wording. An accepted architecture, implemented
code, software evidence and human checkpoint acceptance are separate states.
Use current readiness/explicit acceptance records to resolve later dispositions;
this index does not approve, revoke or rewrite a decision.

[V1-FREEZE acceptance matrix](../../releases/V1_FREEZE_READINESS_2026_10_20.md#4-compact-human-acceptance-matrix)
reconciles the current ADR-0024/0025/0026 implementation reviews without rewriting
accepted ADR text or promoting architecture approval to release acceptance.

Use the [versioned ADR template](../../../.agents/skills/project-documentation/assets/ADR-TEMPLATE.md).
Include context, alternatives, evidence, compatibility, validation and approval.
Preserve accepted reasoning and history; future WJ semantics require their own
qualification. M0–M4 are retained; later work uses named tracks, not global M5.

| ADR | Decision | Recorded state in the decision document |
|---|---|---|
| [ADR-0001](ADR-0001-shopbot-cps.md) | ADR-0001: ShopBot MVP through the existing CPS recorder/runtime | Accepted for the approved M2 implementation; milestone review pending |
| [ADR-0002](ADR-0002-feeds-speeds-boundary.md) | ADR-0002: Milling recommendation boundary and explicit applied parameters | Accepted — FS0 user approval on 2026-10-12; pure FS1 implementation authorized |
| [ADR-0003](ADR-0003-feeds-speeds-data-ownership.md) | ADR-0003: FS domain snapshots, scoped data and applied operation authority | Accepted — user approved FS0 on 2026-10-12; integration remains FS3 |
| [ADR-0004](ADR-0004-manual-operation-cutting-and-source-settings.md) | ADR-0004: Manual operation cutting values and global recommendation sources | Proposed — FS3.5 implementation authorized; human checkpoint review pending |
| [ADR-0005](ADR-0005-source-packs-and-cutting-knowledge.md) | ADR-0005: Generic source packs and deterministic cutting knowledge | Implemented under explicitly authorized FS3.7; human checkpoint approval pending |
| [ADR-0006](ADR-0006-local-preset-authoring-and-validation.md) | ADR-0006: Local preset capture and human validation | Implemented for FS3.8; human approval pending |
| [ADR-0007](ADR-0007-shopbot-resolved-preview.md) | ADR-0007: Bundled ShopBot resolved preview from emitted XYZ | Implemented in UX1; human review pending (ivac-cb64) |
| [ADR-0008](ADR-0008-work-origin-frames.md) | ADR-0008: Generic work coordinates and drawing-frame display | Implemented, human review pending (CAM2/UX2) |
| [ADR-0009](ADR-0009-job-intent-machine-setup.md) | ADR-0009: Job intent versus physical machine setup | Accepted design direction; implementation DEFERRED |
| [ADR-0010](ADR-0010-depth-scoped-remaining-material.md) | ADR-0010: Depth-scoped remaining-material diagnostics | Implemented for bounded CAM4 A/B; human review pending |
| [ADR-0011](ADR-0011-bounded-cutting-reference-adaptation.md) | ADR-0011: Bounded normalized cutting-reference adaptation | Implemented under authorized FS3.9; human checkpoint approval pending |
| [ADR-0012](ADR-0012-fabcam-product-identity.md) | ADR-0012: FabCAM product identity with stable compatibility identifiers | Accepted identity policy; local implementation awaiting human review |
| [ADR-0013](ADR-0013-persistent-corner-relief.md) | ADR-0013: Persistent CAD corner relief before CAM evaluation | Accepted architecture by user on 2026-10-14; software qualified; human CAD1 review pending |
| [ADR-0014](ADR-0014-explicit-planar-surfacing.md) | ADR-0014: Explicit planar surfacing operation | Implemented locally; human UX3 review pending (ivac-k4z3) |
| [ADR-0015](ADR-0015-reusable-operation-templates.md) | ADR-0015: Reusable operation templates with explicit rebinding | Accepted within bounded UX4.1 software handoff; ivac-si5w approved 2026-10-14 (supersedes ivac-51gg) |
| [ADR-0016](ADR-0016-command-palette-owner-dispatch.md) | ADR-0016: Command palette dispatch through existing action owners | Accepted by the user (ivac-2zh9); implemented locally, software review ivac-txve pending |
| [ADR-0017](ADR-0017-vcarve-continuous-planner.md) | ADR-0017: V-Carve-only continuous pass planner | Architecture approved by user; implemented locally, software review pending |
| [ADR-0018](ADR-0018-bounded-shopbot-post-reuse.md) | Bounded process-local ShopBot post result reuse | 2026-10-15. Accepted implementation direction by user authorization of PERF1-B |
| [ADR-0019](ADR-0019-remaining-material-kernel-reuse.md) | Bounded reuse of the remaining-material planar kernel | 2026-10-15. PERF1-B2 authorized after user acceptance of B1. |
| [ADR-0020](ADR-0020-cooperative-cps-cancellation.md) | Connect cooperative CPS cancellation to the existing host probe | 2026-10-15. User accepted PERF1-B2 and authorized the proposed cancellation work. |
| [ADR-0021](ADR-0021-edited-drawing-contract.md) | ADR-0021: Source-preserving drawing edits and stable CAM bindings | Accepted in principle — mandatory refinements formalized before implementation |
| [ADR-0022](ADR-0022-versioned-tab-width.md) | ADR-0022: Versioned holding-tab width | Architecture approved by user; implementation awaiting human review |
| [ADR-0023](ADR-0023-milling-program-publication-guard.md) | Validate milling programs before executable publication | ACCEPTED — implemented locally, human review pending — approved by user on 2026-10-18, 2026-10-18. |
| [ADR-0024](ADR-0024-full-program-simulation-qualification.md) | Full-program simulation qualification independent of playback | APPROVED architecture by the user — 2026-10-19; implemented locally, checkpoint review pending |
| [ADR-0025](ADR-0025-movement-tool-provenance.md) | ADR-0025: Explicit movement tool provenance and emitted-intent verification | APPROVED by the user — 2026-10-19; implemented locally; checkpoint review pending |
| [ADR-0026](ADR-0026-explicit-import-units.md) | ADR-0026: Explicit import units for unitless DXF and non-physical SVG | AUTHORIZED for bounded implementation (2026-10-20); implemented locally — human UX/architecture acceptance pending |
| [ADR-0027](ADR-0027-lab-and-reference-identities.md) | Independent lab identities and reference-pack compatibility | User-authorized v1.1 follow-up, 2026-10-21; UX acceptance pending |
| [ADR-0028](ADR-0028-preset-duplicate-adapt.md) | Strict preset copy permissions, compatibility and inactive Drafts | User-approved architecture 2026-10-21; implementation human review ivac-1oe9l OPEN |
| [ADR-0029](ADR-0029-manual-inactive-recipes.md) | Manually editable inactive recipes from presets | User-authorized policy 2026-10-21; supersedes ADR-0028 context/value restrictions; implementation UX acceptance pending |
