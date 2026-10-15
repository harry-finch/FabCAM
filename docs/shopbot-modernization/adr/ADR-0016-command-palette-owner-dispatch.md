# ADR-0016: Command palette dispatch through existing action owners

Status: Accepted by the user (ivac-2zh9); implemented locally, software review ivac-txve pending
Date: 2026-10-14

## Context

The user accepted UX4.1 and requested UX5 continuation. UX5_PLAN explicitly
starts with a live action/shortcut audit and design checkpoint before coding.
The application has pure shortcut decisions, App-local effects, component-owned
generation/export/template/CAD workflows, shared file services and mutation
history. A second independent callback/guard collection would diverge from them.

The audit found two export policy layers, origin-dependent tool identities,
tool-ID-bound saved cutting slots and first-use inventory editor seeding.
[Verified seams and proposed design](../ux/UX5_AUDIT.md).

## Decision

Use pure localized descriptors/search/context plus thin bindings to existing
shell/component/service actions. Search indexes whitelisted metadata from current
stores, never geometry or cutting evidence. Mutation history remains separate.
The palette owns only transient query/selection/focus. Execute rechecks the
captured document/target and current authority; unavailable/changed/disposed
owners refuse explicitly. No hidden action replay on another document.

GenerateBar retains run/cancel and the complete guarded export action. File I/O
retains native-picker freshness checks. Template results open UX4.1's binder;
no template instantiation semantics move into the palette. Tool results encode
inventory/project origin and cannot implicitly stock/assign. Explicit assignment
uses current operation commands and existing compatibility/rate-role authority;
missing saved slots route to review rather than fabricating values. Search has
no FS/CAM/post call. Other-process tool families remain metadata; no new waterjet
or machine-control behavior is introduced.

The reviewed contract is implemented with pure search/dispatch/keyboard/export-
policy helpers, dedicated shell/component bindings and transient palette UI.
Actual names precede generic aliases. Stable origin-qualified ids and captured
owner/context tokens guard execution; repeated Generate allows explicit Cancel.
The shared modal trap retains Tab and the destination owner receives focus.

## Alternatives considered

- Palette-local callbacks and guards: creates menu/shortcut/export divergence.
- Reuse raw file export alone: omits owner-level sim/pipeline/work-area policy.
- Search command history as an app registry: mixes mutation and navigation.
- New editable tool/template database: risks origin collisions and store drift.
- Stock/assign on result selection: changes job data without explicit action.
- Broad App or CAM refactor: outside the bounded frontend milestone.

## Consequences

Small frontend extension, source-qualified identity and testable action parity.
Owner registration/teardown and focus handoff need explicit lifecycle tests.
No serialization/schema migration, new Rust/API engine, FS expansion or post
change. Same-name or same-ID origin collisions remain honest, visible data.
Browser shortcut/focus and synthetic search latency evidence are recorded in
[UX5 qualification](../UX5_QUALIFICATION.md). Native cross-platform/IME/picker and
physical qualification are not implied by this accepted software decision.

## Validation

The audit checkpoint had143 existing seam regression passes across12 files.
The completed software handoff has1220 frontend passes/1skip, check/lint/build
passing, plus actual local-browser command/template/history/focus and protected
ShopBot export checks. The implementation adds42 tests for search/numeric/role/
dispatch/keyboard/export-policy decisions and records the5250-entry latency corpus.

Back-side/work-area policy and late native-picker freshness retain software
coverage; no new native two-sided/save-picker runtime proof, Windows/Linux
shortcut conflict or native IME qualification is claimed. Detailed observations
and limits are authoritative in [UX5 qualification](../UX5_QUALIFICATION.md).
Human gate ivac-txve precedes later milling work and any publication.
