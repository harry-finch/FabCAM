# ADR-0004: Manual operation cutting values and global recommendation sources

Status: Proposed — FS3.5 implementation authorized; human checkpoint review pending
Date: 2026-10-12

## Context

FS3 deliberately retained tool cutting defaults for legacy inheritance and
applied rate roles for operation authority. Its mandatory recommendation snapshot
prevented a complete manual operation cutting block independent of the engine.
Source import and per-operation material picking obscured the job workflow.
See [FS3.5 audit](../feeds-speeds/FS3_5_AUDIT.md) for storage/serialization/consumer evidence.

## Decision

Extend the existing AppliedCutting model with an optional recommendation snapshot.
Its slots remain the authoritative operation RPM/feed/plunge; native step,
depth_list and pocket overlap remain sole geometry authority. A manual edit
snapshots legacy roles explicitly, with no fabricated recommendation or evidence.
Only blocks with a proposal have recommendation staleness. Existing recommended
blocks deserialize unchanged; generation never invokes the engine.

Portable new saves use ivac-project-cutting/version 3 whenever cutting context
is present. Accept legacy v1 and FS3 v2; distinct kind makes older FS3 applications
reject new manual-only blocks. No existing file is rewritten on load, no tool
value is removed/reinterpreted and non-milling inheritance is preserved.

Reuse job material Identity at Stock level. Optional portable display labels remain
UI metadata and are stripped from native matching identities. Persist recommendation source
configuration under existing per-installation ivac.settings, not project rates.
Load configured data without calculating or applying proposals automatically.
Machine/tool reference compatibility stays explicit optional frontend metadata;
legacy fsContext snapshots remain readable. No machine equivalence is inferred.
Source selection never broadens profile scope or invents cross-source priority.

## Alternatives

Removing tool defaults would break historical projects and non-milling tools.
A second resolved model would duplicate already authoritative slots. Fake
recommendation snapshots for manual entry would make provenance untrustworthy.
Keeping dataset import in each operation would continue the identified UX issue.

## Consequences and validation

Optional proposal is additive in native JSON. Portable files need the new guard;
manual and recommended round-trips, legacy inheritance, Apply output parity,
source persistence and non-milling behavior require regression evidence.
No engine arithmetic or ShopBot post changes. Human FS3.5 acceptance is required
before FS4; physical preset evidence remains separate.
