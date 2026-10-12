# Architectural decisions

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
