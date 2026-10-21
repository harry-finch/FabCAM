# PRESET-4 — Experimental recipe application

**2026-10-21: Phase A audited; HARD STOP before persistence/API implementation.**
No trial application has been implemented or qualified. Human architectural approval
is required by the submitted brief, Phase A and Phase C. Audit: `ivac-21mqf`;
human architecture gate: `ivac-knihp` OPEN.

## Snapshot and scope

Inspected `FabCAM-v1.1`, HEAD `17581be103c374b3b11a862ac26a7e8aaa1c3ae6`,
plus the existing uncommitted manual recipe authoring and compact browser changes.
This is a working-tree audit, not qualification of a reproducible committed release.
All prior source changes, untracked reports and local evidence were preserved.
No production code, schemas, generated clients, source packs, security policies,
machine output or accepted ADRs were changed for PRESET-4.

Relevant contracts: [PRESET-3](FABCAM_PRESET3_AUDIT_AND_IMPLEMENTATION.md),
[recipe authoring](PRESET_RECIPE_AUTHORING_2026_10_21.md),
[ADR-0027](../shopbot-modernization/adr/ADR-0027-lab-and-reference-identities.md),
[ADR-0028](../shopbot-modernization/adr/ADR-0028-preset-duplicate-adapt.md),
[ADR-0029](../shopbot-modernization/adr/ADR-0029-manual-inactive-recipes.md).

## Existing mechanisms audited

| Mechanism | Confirmed behavior and implication |
|---|---|
| Manual operation values | `frontend/src/lib/state/feeds-speeds.ts`: `editOperationRate` requires positive u32 integers; `preserveManualCutting` marks edited rate/DOC/stepover origins manual. Saved slots override tool defaults. This is a numeric path, not an experimental-provenance path. |
| Rust generation | `crates/ivac-core/src/feeds_speeds.rs`: `saved_rates` rejects zero/missing/duplicate roles; `validate_applied_project` rejects incompatible processes, operations and cutter families. Manual-only round-trip/generation has a direct passing test. |
| Recommended / Browse | `feeds_speeds` supplies preview/apply; pinned selected records use the same resolver checks. Frontend `cutting-selector.ts` rereads source fingerprints and checks current context around Rust validation before synchronous commit. Draft is inspect/copy only. |
| Source storage / eligibility | `source_pack.rs::validate_lab` requires Draft to be theoretical, inactive, without review. `resolve_packs` excludes Draft/disabled/deleted/reference-only records, including explicitly selected records. Authoring does not promote them. |
| Atomic application | `commands.ts::applyCuttingCommand` stores the prior operation/material/context for Undo/Redo and applies the operation patch synchronously. A future trial must preserve the existing job material/tool and recheck context, rather than changing them. |
| Portable projects | `AppliedCutting` in Rust has `deny_unknown_fields`; `CuttingOrigin` only has recommendation/manual/legacy_snapshot. Project validation explicitly allows those three origins. Saved format kind/version is guarded (currently 1–7). |
| Source privacy | `KnowledgeSummary` saves compact source identities/evidence, without source bodies/notes. Private recommendations omit the full proposal. This summary describes resolver output, including engine/policy and compatible-source counts; it has no experimental application discriminator. |
| Invalidation | `generation-context.ts` fingerprints the built project including cutting inputs. `program-qualification.ts` rejects missing/stale generation and stale verification keys. `services/file_ops.ts` rechecks export safety/current inputs, including after the save dialog. Saving alone cannot make stale generation current. |
| Physical review | `lab-presets.ts::saveCapturedPreset` requires dated successful physical-use attestations and known engagement/entry for Tested. `revisePreset` allows Tested → Validated only with review notes and all review criteria. Rust source-pack validation independently enforces the evidence policy. |

**Can manual values produce a qualified program?** They can generate machine output
without a recommendation or live source dataset. The ordinary mandatory simulation,
postprocessor and export gates still apply; this does not establish physical safety,
physical qualification or acceptance on an installed platform.

**Must Draft eligibility change?** No. Exact numeric values could enter a separate
explicit manual trial path. Making Draft eligible for the resolver is unnecessary
and forbidden by this checkpoint.

**Which operations/roles?** The recommendation adapter supports milling Profile
and Pocket. Saved manual slots additionally support PlanarSurface, with the cutter
family exceptions explicitly checked in `validate_applied_project`. Slots distinguish
tool ID and Rough/Finish; Drill is refused. All required tool/pass slots must exist
exactly once. Proposed first trial slice is Profile/Pocket, one assigned cutting
tool, without a distinct finish tool; it must explicitly populate every required
role without presenting this as support for multi-tool recipes. Surfacing and other
operations remain outside that first slice.

**Are current provenance structures sufficient?** No. Saving only manual slots loses
the requested experimental origin; reusing `Recommendation` or `KnowledgeSummary`
would suggest an engine evaluation that never occurred. A magic policy string,
context key, operation name, note or opaque extension would not establish a typed,
validated experimental state across Rust/frontend persistence and export. Arbitrary
fields inside `AppliedCutting` are not an extension point. The persistent
“Experimental recipe · Not tested” requirement therefore crosses the expressly
reserved project-persistence boundary.

## Minimal proposed architecture — NOT approved or implemented

1. Add one optional typed `experimental_recipe` snapshot to `AppliedCutting`.
   Absence preserves ordinary manual/recommendation semantics. Keep the existing
   per-field origins for numeric manual edits; the separate marker describes the
   untested source, not a new evidence rank or recommendation.
2. Snapshot only exact pack identity/revision, record ID, profile identity/revision,
   theoretical trial kind, target identity/context at application, and original
   applied cutting values. Existing slots/DOC/stepover remain authoritative. The
   original-value baseline permits truthful “modified since trial application”
   reporting. Do not embed pack contents, author/trial notes, local file paths,
   source licenses' private text, or fabricated evidence. Source-context inspection
   can use the installed pack transiently; missing packs must not erase provenance
   or change saved values. Historical upstream attribution remains in source packs.
3. Introduce a guarded project format for experimental snapshots (proposed version
   8), with all existing optional geometry/surfacing features supported. New readers
   continue reading versions 1–7 unchanged; old readers must refuse the new format
   explicitly, rather than silently losing experimental status. This is backward
   readability, **not** a claim that old software can open new experimental files.
   The version-7 edited-drawing equivalence checks must be updated deliberately.
4. Add a separate Rust preview/apply trial contract, shared by WASM/native
   transports. Resolve the exact local Draft and revisions; validate its existing
   source-pack contract, authorization, stable target identities, supported roles,
   numeric values, known machine/tool limits, flute length/DOC and overlap. Reuse
   existing setup/numeric validation where possible, without calling the Draft
   recommendation resolver or implementing another recommendation engine.
   Invalid/stale/deleted/missing sources or changed targets refuse atomically.
5. Preserve exact authored numbers: existing operation rate slots are integers,
   so fractional RPM/feed/plunge are refused with an explanation, never rounded.
   Chipload-only recipes require an explicitly authored feed before trial use;
   this slice must not derive it. Existing generation can clamp RPM/feed and emit
   warnings (`pipeline/setup_resolver.rs`); **trial preview/apply must reject values
   needing these clamps**. Preserve those existing pipeline safeguards unchanged.
   Unknown limits stay unknown; acknowledgment cannot override a known limit.
6. Reuse PRESET-3's selector, comparison, context/source refresh and atomic command.
   Add the deliberate “Use for trial…” action from both entry points, unchecked
   theoretical-use acknowledgment and compact persistent label. Keep ordinary
   Recommended/eligible selected-record behavior unchanged. Value/context edits
   invalidate confirmation; manual edits retain untested ancestry and mark changes.
7. Revalidate trial metadata and hard limits in Rust generation, so direct API/file
   input cannot bypass the trial contract. Preserve SAFE-Z, numeric budgets,
   operation/tool validation, ShopBot/CPS restrictions, tool provenance, program
   simulation and fail-closed export controls. Include experimental origin in the
   existing export feedback, without adding a second export route or suggesting
   that software verification is physical validation.

No source-pack format, eligibility rank, physical-evidence policy or existing CAM
safety invariant needs to be relaxed. The proposed API/persistence addition itself
requires approval; implementation details and its negative tests must be reviewed
before calling it safe. Generated API/schema artifacts must come from Rust tooling.

## Physical feedback lifecycle

The existing capture flow can record manually resolved operation rates as a dated
Tested record, and the existing manager reviews Tested records for Validated.
Neither action is a consequence of generating/exporting/applying a recipe.
The current manager does not provide a dedicated trial ledger linking an experimental
application to multiple observed outcomes and exact program/configuration snapshots.
Do not claim that complete end-to-end lifecycle is implemented.

After PRESET-4 acceptance, propose a bounded workshop feedback checkpoint: explicitly
record actual configuration, program reference, date/operator, observations and
successful/failed outcome; review how it links to existing capture/revision history.
Only independent human attestations/review may promote evidence. No automatic
qualification engine, hardware test or promotion is authorized here.

## Verification actually run at this checkpoint

| Command / scope | Result |
|---|---|
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --lib feeds_speeds::tests -- --nocapture` | 20 PASS, 0 failed/ignored; 1173 unrelated tests filtered. Includes manual-only round-trip/generation, saved-value independence, private-body exclusion, explicit-record application and missing/duplicate/incompatible roles. |
| Frontend `vitest run` on feeds-speeds, source-packs, cutting-selector, cutting-selector-wasm, lab-presets, program-qualification, qualification-export, recipe-values and history test files | 97 PASS across 9 files, no skips. Real WASM test retains Draft rejection; unit checks cover review/eligibility, context changes, Undo/Redo and qualification/export guards. |

These are existing regression tests against the inspected working tree. No PRESET-4
implementation tests, UX replay/screenshots, all-features qualification, type/lint,
generated-contract regeneration or build are claimed: implementation stopped before
the schema boundary. No tests were weakened or skipped. Existing reports' broader
test results remain historical and are not substituted for this run.

## Handoff and human decision

Approve or amend the optional experimental snapshot, guarded project version and
separate Rust trial contract above before Phase B–G implementation. Then qualify
exact application/refusal, privacy, edits/staleness, source revisions, round-trip,
Undo/Redo, generation/export invalidation and both compact-selector entry points,
including narrow viewport screenshots. No physical acceptance follows from tests.

This audit adds this report and PRESET-4 navigation/status only. Existing recipe/UI
changes remain uncommitted. Any audit-only commit is identified in the chat handoff;
this document's inspected code revision remains `17581be` regardless of a subsequent
documentation commit. No push, merge, rebase, release, version promotion or hardware
operation. **HARD STOP for architectural approval.**
