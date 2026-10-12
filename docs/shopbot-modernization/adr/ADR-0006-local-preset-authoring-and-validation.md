# ADR-0006: Local preset capture and human validation

Status: Implemented for FS3.8; human approval pending
Date: 2026-10-12

## Context

FS3.7 provides one SourcePack contract, immutable app-local storage, pure resolver
and private-safe applied knowledge summaries. Physical trials need an authoring
workflow that records actual operation values without changing those values or
confusing a proposal with machine evidence. FS4 is not authorized.

## Decision

Extend SourcePack to schema 2, retaining schema 1 read compatibility. Public
Sienci data remains schema 1 and unchanged. Optional RecordOrigin.lab metadata
and lab_history snapshots describe authoring; the existing Evidence enum remains
unchanged. Draft is an inactive authoring state with theoretical evidence, never
an eligible recommendation. Tested and Validated use the existing exact-preset
categories and resolver tiers.

Capture through a pure core adapter using cam_request and saved_rates, including
manual overrides and existing fallback resolution. Require stable identities;
Machine/Tool compatibility offers explicit UUID local identity creation when no
reference exists. This is a separate setup action, not part of Save, and uses the
existing fsIdentity fields. Never identify physical equipment by display names.
Keep exact machine/material/tool/version, operation, DOC, thickness, entry and
known full-width or nominal engagement. Unknown engagement remains an inactive
Draft. Pocket capture qualifies primary roughing only; secondary finishing rates
remain context, not an invented qualified finish preset.

Sorbonne records use generated UUID preset IDs. Writes create immutable pack and
profile revisions; stable preset IDs and creation dates persist. Previous profiles
and authoring metadata are embedded in current export history. Native writes and
Sorbonne imports share an exclusive marker lock; authoring also compares expected
head revision. Browser writes use one IndexedDB transaction with the same head
comparison. Stale saves are rejected. Only the current Sorbonne head is loaded;
older bodies remain stored. Source preferences follow that head while preserving
the enable choice. This never recalculates or rewrites operations.

Policy sorbonne-local-validation-1 requires dated physical success for Tested,
and a later explicit documented review for Validated. The review attests acceptable
finish and behavior, no overheating/chatter/overload, and suitability for normal
lab use in that exact context. There is no invented scientific sample count or
numerical tolerance. Software cannot verify a human's physical-use claim.
Disabling and confirmed deletion are reversible flags; revisions remain traceable.

## Alternatives considered

- A parallel preset database would duplicate FS3.7 identities and resolution.
- Adding Draft to cutting Evidence would conflate authoring with usable evidence.
- Immediate Validated in Save would weaken the deliberate review boundary.
- Overwriting records or embedding full private packs in projects would lose
  traceability or leak installation-local observations.
- Inferring ShopBot identity, engagement or cutter traits from names would broaden
  applicability without reliable physical context.

## Consequences

No mathematics, post-processing or Tool/CuttingParameters ownership changes.
Existing v1/v2/v3 projects and v4 private-safe saves remain supported. Missing local
knowledge never prevents saved-value generation. Local records have no public
redistribution grant and are never added to the bundled registry or Git.
Export is explicit and includes private notes and retained history. Storage limits
remain 2 MiB per pack, 32 distinct source IDs, 512 installed immutable revisions
(32 non-Sorbonne revisions); export before limits. Very long histories may require
future compaction tooling. Restore refuses an equal/older installed Sorbonne head;
restore on a fresh installation is supported, not destructive rollback/merge.
A crashed native writer can leave a lock marker: close all instances before manual
recovery, as the error explains. Windows/Linux runtime checks remain pending.

## Validation

Pure authoring, core capture, Rust loader/resolver and native storage tests plus
real browser Draft/save/manage/export/restore checks are recorded in
[FS3.8 qualification](../feeds-speeds/FS3_8_QUALIFICATION.md). All synthetic trials
are SOFTWARE ONLY. Human review and real physical trials remain separate gates.
