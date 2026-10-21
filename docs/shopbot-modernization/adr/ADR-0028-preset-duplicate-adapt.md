# ADR-0028: PRESET-2 copy permissions, compatibility and incomplete drafts

Status: ACCEPTED for bounded implementation — user approved the strict framework, 2026-10-21
Date: 2026-10-21
Track: PRESET-2, `ivac-h5nln`, branch `FabCAM-v1.1`

> Follow-up 2026-10-21: [ADR-0029](ADR-0029-manual-inactive-recipes.md) supersedes
> unchanged-value and machine/material-match restrictions for new inactive recipes,
> following an explicit user request. The original accepted decision below is retained.

## Context

The user accepted PRESET-1 and requested the next milestone. PRESET-2 is authorized
as a workflow, but its canonical contract explicitly requires human review of exact
compatibility boundaries and incomplete-draft behavior before implementation.
This proposal makes those decisions concrete. PRESET-1's read-only projection and
existing local pack authoring/storage remain the foundation; do not restart FS or
introduce a competing recommendation engine, persistent material taxonomy or CAM.

## Proposed bounded decision

1. The copy targets a selected editable private lab pack and the **current physical
   milling machine**, a chosen existing physical library/project cutter, and an
   explicitly selected material. No selected operation is required. The dialog does
   not stock tools, change the job material or change any operation.
2. Required physical machine/tool identities must already exist through ADR-0027
   creation/load workflows; material has an explicit ID/revision. Creating or fixing
   a machine/tool association uses its existing editor, then reopens the copy. The
   material selector may author its existing custom identity in the dialog's local
   form only. Legacy ambiguous identities are refused rather than split/inferred.
3. Process must declare milling. Source/current tool cutting families must agree;
   a shared generic `other` family alone cannot establish geometric compatibility.
   Machine scope must explicitly match the physical identity for a local trial, or
   the current reference identity/recorded membership for a reference. No transfer
   to a different machine merely because its name, work area or spindle looks similar.
4. Material must be the same ID/revision, or belong to a relation expressly declared
   by the existing source scope/membership/adaptation data. Textual similarity,
   aliases, general wood/plastic labels or a new custom name do not establish this.
   Changed compatible diameter/flutes are allowed only as clearly flagged untested
   targets within the same known cutting family. No numerical scaling is performed.
5. **No incomplete saved copy in this first slice.** Unknown required identity,
   incompatible context, unresolved geometry/family or missing permission blocks
   save with an explanation. The editable form can remain incomplete without any
   storage write. Wider draft permissiveness requires a subsequent reviewed decision.
6. A reference source requires explicit transformation permission in its validated
   manifest. Installation-local storage is not permission. A locally authored private
   preset whose manifest does not grant transformation requires a deliberate operator
   ownership/authorization attestation for this copy; it does not rewrite the source
   manifest or grant public redistribution. Generic references with absent/negative
   permission stay blocked. Source license, attribution and restrictions are retained.
7. Save creates a new independent UUID record as **Draft / theoretical / disabled**.
   Clear test dates, physical-success booleans, reviews, evidence records and lab
   history. Do not inherit Tested/Validated eligibility. Preserve original authored
   values (including chipload-only data without inventing a feed), show before/after
   context and require explicit review. No auto calculation, reduction, apply or CAM.
8. Store the source pack ID/revision, record ID/revision, original source reference,
   author, attribution/license and changes as explicit provenance in the existing
   source/notes fields. New target context is exact; original constraints are retained
   as source information, never represented as target qualification. No new serialized
   fields are proposed in this slice. If inspection shows existing fields cannot retain
   adequate provenance, stop for a separate schema/migration decision.
9. Pin the selected source and destination revision and form context. Recheck both
   before writing and use the existing destination compare-and-append command.
   Stale source/destination/context requires reopening/review; no silent retry on a
   new head. Cancel writes no preset and mutates no source or operation. Existing
   explicitly-created empty packs may remain if the user created one separately.

## Intended interaction

Presets → select record → Duplicate & adapt → inspect permission/compatibility →
choose local pack, cutter and material → inspect copied values/context differences →
confirm review and any local ownership attestation → Save new Draft.

The source is always immutable in this workflow. Existing lab evidence promotion
stays a separate manual process with its current physical-use/review requirements.
No new Test/Validate shortcut is part of the copy dialog.

## Review matrix prepared for implementation

| Case | Proposed result |
|---|---|
| Complete same compatible context | Independent inactive Draft; original unchanged |
| Same known cutting family, changed diameter/flutes | Visible differences/review flags; unchanged copied values; inactive Draft |
| Explicitly related material | Visible material difference; inactive Draft; no inferred equivalence |
| Similar material label only | Refuse until a valid recorded association exists |
| Licensed transformable reference | Preserve source/reference/attribution/license; new Draft only |
| Private authored preset without redistribution permission | Require explicit local ownership/copy authorization; do not grant redistribution |
| Generic reference permission missing/false | Refuse copying |
| Missing identity/non-milling/wrong machine or cutting family | Refuse save; explain missing association |
| Source/destination head changes | Refuse stale save; no partial mutation |
| Cancel | No preset write, source/operation unchanged |

## Compatibility and evidence boundary

Use existing source-pack schema 2 and immutable native/IndexedDB storage. Public
reference catalogs, old Sorbonne records, named packs, existing identities and
PRESET-1 changes are retained unchanged. No engine eligibility or release evidence
is inferred from a successful draft copy. PRESET-2 software tests, UI review and
physical qualification remain separate. The strict framework was explicitly approved through the in-chat architecture question.
Implementation and software/UX acceptance remain separate checkpoints.
