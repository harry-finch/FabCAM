# ADR-0003: FS domain snapshots, scoped data and applied operation authority

Status: Accepted — user approved FS0 on 2026-10-12; integration remains FS3
Date: 2026-10-12

## Context

[ADR-0002](ADR-0002-feeds-speeds-boundary.md) requires a pure milling assistant
and explicitly persisted user choices. [FS0 qualification](../feeds-speeds/FS0_QUALIFICATION.md)
finds tool-owned RPM, operation feed/plunge overrides, signed step and overlap
inheritance, no cutting material identity, and independently resolved CPS section
metadata. Portable frontend files differ from Rust pipeline request shapes.

## Decision

Use a dedicated `ivac-feeds-speeds` crate, independent of core/geometry/CPS/UI,
using small copied machine/material/tool/operation snapshots. A core adapter maps
context and domain diagnostics into the existing pipeline warning system.

Add optional process-neutral project material identity at FS3; keep stock geometry
and milling cutting properties separate. Future configurable datasets use
versioned JSON envelopes with immutable IDs/revisions, source permissions,
evidence and explicit scope. Eligible exact validated/tested presets precede
machine/family and reference/theoretical profiles. Strict scope containment
resolves specificity; equal/incomparable top candidates are explicitly ambiguous.
No source means no recommendation. Missing constraints remain unknown.

At FS3, one optional milling operation block owns complete applied rate slots for
each used tool/pass. Existing signed step/overlap/schedule fields remain sole
geometry authority, written explicitly on Apply. Do not store duplicate DOC/WOC
values. Preserve absent-block legacy inheritance and never migrate implicitly.
Manual edits, proposal snapshots and per-field origins persist. Context changes
mark stale without changing applied values; unresolved new tool roles require
explicit application. Post, CPS metadata, simulation and headers consume a single
runtime resolution; no dynamic recommendation during generation/export.

FS-bearing portable files require a guarded new version; v1 remains readable by
new apps, but old apps cannot be claimed to safely read v2. Dataset changes cannot
change saved settings. Final machine clamps remain separate, visible runtime
constraints. FS0 approved these design decisions; no public model changed during FS0.
FS1 implements the isolated engine only; core project integration remains FS3.

## Alternatives considered

- Engine in ivac-core: smaller initial layout, but couples independent consumers
  to geometry/process dependencies; dedicated crate provides enforceable boundary.
- Apply to tool library: changes unrelated operations and loses per-operation
  manual authority. Keep defaults for legacy workflows, snapshot applied roles.
- Duplicate all values in a resolved object and existing geometry fields: creates
  two editable authorities. Keep native geometry fields and a single rate block.
- Material as a stock label or milling-dependent generic type: confuses geometry,
  identity and process data. Optional generic identity is sufficient.
- YAML/TOML or arbitrary ID tie-breaks: adds conventions or silently chooses
  conflicting cutting data. JSON matches persistence; ambiguity is inspectable.
- Treat tool defaults as theoretical fallback: the equation cannot establish
  safe material cutting values. Manual values remain usable without evidence claims.

## Consequences

Later changes are additive but span core models, generated schema, the portable
file guard, state/undo, adapter, resolver/header/dual-tool/CPS consumers and cache
keys. Keep geometry algorithms, native post interfaces and non-milling paths.
Applied rates must not depend on live presets or library changes. Complete role
snapshots and strict matching cost some data authoring effort and may return
Unavailable; this is preferable to invented limits or broadened validation.
No upstream source/dependency delta is introduced by this documentation.

## Validation

FS-01 through FS-16 in FS0 qualification define later deterministic, persistence
and consumer contracts. Existing resolver baseline: 19 targeted tests passed.
Revisit role storage if integration demonstrates a smaller equally authoritative
extension; preserve explicit persistence, manual authority and no post-time engine.
Validated evidence requires approved FS4 protocol and scoped human results.
