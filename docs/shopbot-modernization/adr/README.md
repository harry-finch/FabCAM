# Architectural decisions

- [ADR-0012: FabCAM product identity](ADR-0012-fabcam-product-identity.md) — public name and preserved upstream/compatibility contracts; implementation human review pending.

Use `.agents/skills/project-documentation/assets/ADR-TEMPLATE.md`; include
context, alternatives, evidence, decision, compatibility and validation impact.

- [ADR-0001 — ShopBot CPS](ADR-0001-shopbot-cps.md): implemented and approved
  through M2; no native post or new IR.
- [ADR-0002 — Feeds & Speeds boundary](ADR-0002-feeds-speeds-boundary.md): implemented through the approved FS0–FS3 foundation; pure milling engine and explicit CAM/persistence boundary.

Future WJ architecture qualification must assess explicit waterjet semantics
before implementation and record significant decisions. Global M5 numbering
is superseded by named tracks in ROADMAP.

- [ADR-0005 — Source packs and cutting knowledge](ADR-0005-source-packs-and-cutting-knowledge.md): implemented FS3.7, human approval pending; shared schema/engine, explicit taxonomy, conservative reconciliation and private-safe storage/projects.

- [ADR-0006 — Local preset authoring and validation](ADR-0006-local-preset-authoring-and-validation.md): implemented FS3.8, human approval pending; resolved-value capture, inactive Draft, later validation, immutable private history.

- [ADR-0007 — ShopBot resolved preview](ADR-0007-shopbot-resolved-preview.md): UX1 final preview/simulation follows emitted XYZ; generic intent preserved, post resolution explicit, no universal capability framework.

- [ADR-0008 — Work-origin frames](ADR-0008-work-origin-frames.md): CAM2 single-sided
  XY normalization before CAM, drawing display and compatible stock persistence;
  explicit Z/two-sided limits, human review pending.

- [ADR-0009 — Job intent and physical setup](ADR-0009-job-intent-machine-setup.md):
  accepted future design direction, implementation deferred; standalone FabCAM /
  FabSBP, independent XY/Z intent, explicit metadata and reusable profiles.

- [ADR-0010 — Depth-scoped remaining material](ADR-0010-depth-scoped-remaining-material.md):
  CAM4 A/B bounded per-pocket diagnostic, final path provenance, additive response,
  explicit unsupported context and fresh drawing-frame overlay; human review pending.

- [ADR-0011: Bounded cutting-reference adaptation](ADR-0011-bounded-cutting-reference-adaptation.md) — FS3.9; human approval pending.

- [ADR-0013 — Persistent CAD corner relief](ADR-0013-persistent-corner-relief.md): accepted source-preserving modifier, pure geometry evaluator and normal CAM boundary; CAD1 bounded continuation accepted for UX3 (ivac-7r7r).

- [ADR-0014 — Explicit planar surfacing](ADR-0014-explicit-planar-surfacing.md):
  operation-owned raster rectangle, shared XYZ emission, scoped edge diagnostics
  and guarded v6; implemented locally, UX3 continuation accepted ivac-k4z3.

- [ADR-0015 — Reusable operation templates](ADR-0015-reusable-operation-templates.md):
  bounded local recipes, explicit destination binding and independent manual-value
  operations; UX4 implemented locally; UX4.1 software review ivac-si5w accepted, superseding ivac-51gg.

- [ADR-0016 — Command palette owner dispatch](ADR-0016-command-palette-owner-dispatch.md):
  accepted UX5 design ivac-2zh9; owner-bound palette implemented with origin-aware
  tools/templates and existing guards; human software review ivac-txve.

- [ADR-0017 — V-Carve continuous planner](ADR-0017-vcarve-continuous-planner.md): approved architecture, bounded software implementation; human review pending.

- [ADR-0021 — Edited drawing contract](ADR-0021-edited-drawing-contract.md): GEOM1 approved architecture with mandatory refinements; stable bindings, Rust evaluation and guarded v7, bounded implementation under final software review.
