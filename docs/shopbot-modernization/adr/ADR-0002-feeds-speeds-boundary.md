# ADR-0002: Milling recommendation boundary and explicit applied parameters

Status: Proposed — recorded for post-M4 governance review; implementation not authorized
Date: 2026-10-12

## Context

M4 is complete at `e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`.
The requested FS track needs CAM integration and independent reuse/testing.
Existing Tool models already carry cutting defaults (`project/tool.rs`),
operations carry feed/plunge overrides (`project/params.rs`), and machine models
carry constraints (`project/machine.rs`). These are integration references,
not a completed ownership/compatibility audit; that work is FS0.

## Decision

Record the requested architectural constraints: a milling-specific pure Rust
recommendation engine independent of UI and geometry generation, with a CAM
adapter supplying context. Recommendations are proposals; user acceptance or
modification produces explicit parameters persisted in the project/operation.
Post-processors consume applied parameters and never dynamically call the engine.
Manual edits remain authoritative until deliberate recalculation; no silent
overwrite. Algorithm/data changes must not silently alter reopened projects.
Aggressiveness and evidence remain independent; configurable reference/local
presets carry provenance and documented evidence criteria. WJ/VINYL have no
FS dependency.

Crate/module placement, exact models, units, schema, precedence, fallback,
versioning and migration remain undecided until FS0 qualification. This ADR
records boundaries, not a new universal process layer or a chosen API.

## Alternatives considered

- UI-only calculator: insufficient for independent Rust reuse and headless testing.
- Post-time calculation: would couple output to changing algorithms/data and bypass
  persisted user choices, undermining reproducibility.
- Universal cutting engine for milling/waterjet/drag knife: mixes distinct process
  semantics and is not justified by the current scope.
- Fixed cutting values in algorithms: prevents configurable, traceable local data.

## Consequences

The adapter must bridge existing models without duplicating geometry generation.
Application/persistence is a deliberate action distinct from recommendation.
Future serialized changes need compatibility, schema/codegen and consumer review.
FS0 must assess old-project defaults, tool/operation precedence, existing machine
constraint resolution, transports, simulation and post consumers. No source,
schema, dependency or upstream code delta occurs in this documentation update.

## Validation

FS0 supplies source-backed ownership/consumer and compatibility findings, and
qualifies a test strategy before implementation approval. Later milestones test
engine determinism and units, machine/context limits, configurable data/provenance,
adapter isolation, save/reopen stability across engine/data changes and preservation
of manual edits. Physical validated presets require documented FS4 criteria.
Revisit model placement if that audit demonstrates coupling or migration costs;
preserve the user-required reproducibility and process boundaries.
