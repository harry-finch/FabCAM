# PRESET-3 — recommendation audit and implementation boundary

Date: 2026-10-21. Inspected branch **FabCAM-v1.1**, HEAD **df6eb1f** plus
existing uncommitted PRESET-1/2 implementation. Audit task `ivac-18u82`; human schema decision **`ivac-ms6rz` OPEN**.
**Phase A complete; implementation stopped before the schema boundary.**

The submitted PRESET-3 brief requires: “If a schema or source-pack format change
appears necessary, explain why and HARD STOP before implementing it.” That condition
is met. No production code, tests, schemas, source packs, CAM, post-processors,
security policies, versions, V1 source or release artifacts were changed here.

## Decision

PRESET-3 is **primarily existing capability requiring a new explicit-discovery UX,
with a genuine missing record-selection contract**. It is not a new recommendation
engine and is not redundant with the read-only PRESET-1 library. Most requested
compatibility, parameter resolution, persistence and application behavior already
exists. Rebuilding those capabilities would substantially duplicate working code.

Recommended architecture: one compact selector with **Suggest** and **Choose preset**
as discovery modes, converging on the existing authoritative Rust adapter and
`applyCuttingCommand`. The library supplies a pinned record and an existing target
operation to that same selector. No alternate operation-patching pipeline.

The current `FsCamRequest` has no record selector and rejects unknown fields. A
proper explicit selection requires a reviewed additive API/schema contract. Private
applied provenance also lacks a distinct profile-revision field; pack revision and
record ID are retained, but full offline selected-profile identity needs a decision.
No schema workaround or implementation was attempted.

## Inspected source and coordination

Read AGENTS/CLAUDE, current STATUS/ROADMAP and ADR-0027/0028. Applied all six requested
skills. `feeds-speeds` and `professional-workshop-ui` exist under the repository's
`.agents/skills/`, although absent from the host catalogue; their relevant contracts
were read directly. No skill was fabricated or replaced.

Only this root agent is live in the current agent tree. The tracker has no active
PRESET implementation by another agent; the older `ivac-tnpyz` lease is expired and
its notes describe work already committed or awaiting human review. Other V1
worktrees/tracker items were inspected only as coordination context and left alone.
This does not assert knowledge of every external editor/process. A hash inventory
pins the actual starting worktree and verifies source/evidence preservation.

PRESET-1 acceptance is already recorded. PRESET-2 is **implemented, not a future
unstarted milestone**, and its human checkpoint `ivac-1oe9l` remains OPEN. This brief
does not retroactively accept it or authorize restarting it. The roadmap places
PRESET-3 logically after PRESET-1 and before PRESET-2 while retaining that history.

## 1. Actual Suggest call path

1. [FeedsSpeedsSection](../../frontend/src/lib/components/op_properties/FeedsSpeedsSection.svelte)
   exposes **Review recommendation…** (the current label for the Suggest workflow).
   Opening the review automatically calls `calculate()` when context/material/sources
   are ready. `buildProject()` supplies the real operation/tool/machine/material;
   `referenceContext()` preserves physical and reference bindings independently.
2. [RecommendationSources.refresh](../../frontend/src/lib/state/feeds-speeds-sources.svelte.ts)
   validates bundled/local packs, migrates legacy preference bodies through the local
   store, resolves local heads and applies project source settings.
   [enabledPacks/packCollection](../../frontend/src/lib/state/feeds-speeds-sources.ts)
   include only enabled chosen revisions and reject duplicate enabled pack IDs.
3. `calculate()` sends **one collection-wide request**, `apply:false`, through
   `cuttingKnowledgeClient()` → `WasmWiacClient.feedsSpeeds` or
   `TauriWiacClient.feedsSpeeds`. Those call the same
   [core feeds_speeds](../../crates/ivac-core/src/feeds_speeds.rs).
   The HTTP client/server route also reaches this adapter; this UI selects WASM/Tauri.
4. Core `cam_request()` builds the milling domain request. `feeds_speeds()` chooses
   `recommend_packs()` for a pack collection, or legacy `LoadedDataset.engine().recommend()`
   for a legacy dataset. It never runs implicitly during toolpath generation.
5. [recommend_packs_traced](../../crates/ivac-feeds-speeds/src/source_pack.rs) validates
   each pack, checks record eligibility and scope, evaluates records through the same
   `Engine`, optionally uses existing FS3.9 reference derivation, then ranks results.
6. `calculate()` currently stores a one-element response list named `cutting-knowledge`.
   `suggestions` filters it for availability. The UI's `each suggestions` is therefore
   **not** a selectable list of every preset or pack. Its Apply button uses `suggestions[0]`.
7. `apply()` calls the same endpoint again with `apply:true`. Core resolves/validates
   again and returns `applied_op`; the frontend commits the supported patch through
   `project.history.exec(applyCuttingCommand(...), project.target())`.

## 2. Resolution, compatibility and identities

**Single or multiple?** One final available recommendation or an unavailable reason.
`KnowledgeExplanation.records` contains compatible **best-rank retained records**, not
all installed/eligible records: the resolver discards lower ranks before assembling
it. Same-rank comparable records are ordered deterministically by chipload then
pack/record identities; prudent/normal/rapid select the low/middle/high authored
candidate. Non-comparable same-rank RPM/DOC/engagement/reduction contexts return
`ambiguous_profiles`, even when numerical clamps happen to equalize them.
`matching_trace()` is an existing diagnostic that can inspect accepted/rejected
records, but is not an application API and does not return applied operations.

**Sources?** Recommendations already come from the stored source-pack profiles.
Named authored lab records and manufacturer references share `SourcePack`, `Profile`
and `RecordOrigin`, with explicit lab metadata and distribution/rights differences.
A local exact Validated preset ranks before local exact Tested, then other records.
Draft/disabled/deleted lab records are excluded; `external_reference` distribution
is not eligible for recommendation. An imported manufacturer's eligible `local_only`
pack is distinct from that reference-only distribution and from an authored lab trial.

**Can users choose a record?** No. They can change aggressiveness, actual context,
full-width classification or source enablement, but cannot pin a particular record
for preview/apply. Inspecting a record in PRESET-1 does not select it for Suggest.

**Authoritative checks:**

- Core `cam_request()` requires primary **milling mode**, an existing tool, and a
  supported Profile or Pocket operation. Planar surfacing may consume saved/manual
  cutting slots elsewhere; that does not make it supported by this assistant.
- Adapter maps Endmill to `flat_endmill`, other cutter kinds to `other`; having a
  ball-nose/compression/V-bit in the tool library is not automatic FS applicability.
- [matching::mismatches](../../crates/ivac-feeds-speeds/src/matching.rs) checks mode
  (aggressiveness), exact/family/any machine and material IDs **and revisions**, optional
  exact tool identity, cutting family, effective diameter, flute count, operation,
  DOC, stock thickness, entry and engagement. Profile full-width requires explicit
  `fsFullWidth`; Pocket uses nominal configured overlap, not measured engagement.
- The engine validates inputs and respects known machine/tool limits; unknown limits
  remain unknown. Exact local Tested/Validated records are not generalized.
- Existing [FS3.9 adapt_profile/add_interpolations](../../crates/ivac-feeds-speeds/src/adaptation.rs)
  can normalize certain permissioned reference records across diameter/context or
  interpolate an authored series. This is **existing Suggest behavior**, not a license
  for PRESET-3 to scale manually chosen records. Preserve Suggest; explicit-use mode
  should reject required numerical adaptation and offer inspection/duplication instead.
- ADR-0027: frontend `fsLocalIdentity` is the physical identity, `fsIdentity` the
  optional reference. `referenceContext()` sends both; core capture and authored lab
  evaluation call `use_local_identities()`, while references keep reference matching.
  Two physical machines sharing a model reference must not share a lab trial by inference.
- PRESET-2's `adaptationProblems()` answers **whether a new inactive Draft may be
  copied**, including rights. It is intentionally not authoritative applicability
  validation for applying that record to an operation; do not reuse it as that guard.

## 3. Application, persistence, overrides and stale state

Core `feeds_speeds(apply:true)` clones the existing operation, checks explicit depth
schedule consistency, writes DOC to `step`, maps supported Pocket stepover to
`xy_overlap`, and snapshots RPM/feed/plunge into tool/pass slots. It preserves
unqualified secondary Pocket roles and labels their origin accurately. It clears
legacy feed/plunge overrides and runs `validate_applied_project()` before returning.
It does not replace machine, tool, material or geometry bindings.

`FeedsSpeedsSection.apply()` takes the validated cutting block/step/overlap, builds
`appliedContextKey`, and reuses [applyCuttingCommand](../../frontend/src/lib/state/commands.ts).
That command records old/new operation, material and FS context atomically for Undo;
[History](../../frontend/src/lib/state/history.ts) supplies Redo. For a future explicit
selector, pass unchanged current material/context and preserve this single command
boundary, rather than assigning the selected record's machine/tool/material.

`editOperationRate()` and `preserveManualCutting()` keep applied slots authoritative
and mark edited fields `manual`; the accepted proposal remains historical provenance.
Manual step/overlap edits also mark origins. `saved_rates()` consumes saved slots and
fails on missing/duplicate roles rather than silently falling back to a changed tool.
A tool/material/machine/context change marks a proposal stale, retains saved values,
and requires review; invalid cutter/process/slot combinations can block generation.
The pipeline emits `feeds_speeds_stale` without recomputing.

Public applied proposals retain full profile identity/revision and provenance.
Private/local sources deliberately persist **values plus KnowledgeSummary only**,
not private record bodies/notes. That summary retains selected pack ID/revision,
source record ID, evidence, engine/policy, contributors and match quality. It does
**not** separately persist the selected profile ID/revision or a user-selected versus
automatic discovery flag. Source pack revision pins the historical context; do not
claim every profile revision is available offline after removing a private pack.
[Project validation](../../frontend/src/lib/state/feeds-speeds-persistence.ts),
`buildProject`, save/reopen and native tests cover this snapshot boundary.

Changed/deleted/disabled sources do not alter saved rates or trigger live linkage.
The UI can show missing provenance; recommendation eligibility changes on refresh.
`PreviewRequests` plus context/source-content fingerprints reject outdated asynchronous
responses and changed in-memory previews. **Limit:** existing Apply uses cached pack
contents and does not reread the installation store immediately before application.
Cross-window shelf deletion/update is not a proven atomic apply-time guard. The future
shared flow needs a fresh pinned-source availability check before both preview/apply;
source freshness and a portable historical operation snapshot are different concerns.

Applying changed effective parameters alters
[generationContextKey](../../frontend/src/lib/state/generation-context.ts).
Old geometry may remain visible, but [qualificationKey/qualificationPhase](../../frontend/src/lib/state/program-qualification.ts)
make prior simulation stale. [exportGeneratedGcode](../../frontend/src/lib/services/file_ops.ts)
checks current inputs before export and again across the save flow. File saving or
setting `dirty=false` does not qualify an old program. Reuse these freshness guards;
do not claim that `applyCuttingCommand` physically clears every cached result itself.

## 4. Capability map

| Capability | Already exists | Missing | Reusable implementation |
|---|---|---|---|
| Automatic recommendation | Yes: collection-wide deterministic winner, priority/fallback/ambiguity | No new algorithm needed | `cam_request`, `recommend_packs`, `Engine` |
| Explicit preset selection | PRESET-1 browsing only | Pinned-record request and two contextual entry points | `presetRows/filterPresets`, shared selector, Rust per-record evaluation |
| Source compatibility checking | Yes, authoritative Rust scopes/limits and physical/reference split | Expose result for an explicit record, excluding numerical adaptation | `matching`, `Engine`, `matching_trace` diagnostic concepts |
| Parameter preview | Yes: resolved proposed values, diagnostics and provenance | Current→proposed field differences and clear manual-overwrite acknowledgement | `FeedsSpeedsSection`, `RecommendationBasis`, `operationRates` |
| Apply to operation | Yes: validated adapter returns op patch | Invoke same path with explicit pinned selection | `feeds_speeds(apply:true)`, `applyCuttingCommand` |
| Provenance persistence | Yes: public snapshot / private summary | Decide exact private profile revision and discovery-mode metadata | `AppliedCutting`, `KnowledgeSummary`, project round-trip validation |
| Undo/Redo | Yes, command/history infrastructure | Shared integration regression for both future entry points | `applyCuttingCommand`, `History` |
| Stale-context invalidation | Yes: request tokens, context hashes, generated/export qualification | Fresh cross-window source availability check and pinned target ownership | `PreviewRequests`, `cuttingContextKey`, `generationContextKey`, export checks |

Save as preset already uses `capture_lab_preset()` to capture **actual applied/manual
rates**, not just the original recommendation, then `LabPresetDialog`,
`saveCapturedPreset()` and `persistLabPack/commitLabPack` create immutable revisions in
named private packs. Tested/Validated promotion keeps physical attestations/reviews.
PRESET-1 displays/searches/filter records and provenance; PRESET-2 adds inactive Draft
copying. Neither offers direct use in an operation.

## 5. Proposed bounded contract — not implemented

1. **One selector/application owner.** Operation inspector opens a compact selector
   for that operation; library **Use in operation** supplies a pinned record and asks
   for an existing target. Both show current→proposed values and use the same apply
   service/command. No duplicate full library inside a modal and no tool stocking.
2. **Explicit selector contract for review:** optional selection containing pack
   identity/revision, original record ID and profile identity/revision in `FsCamRequest`
   (final field names not chosen). Absence retains current Suggest behavior. A selected
   request validates the complete original enabled pack and evaluates only the pinned
   eligible record through the existing math/scope logic; no fallback to another record.
   Preview and Apply must prove the same record and unchanged target/source context.
3. **No rewriting source bodies to force a winner.** Filtering `dataset_json` profiles,
   changing source identity/revision, using the private record ID as another field, or
   toggling global source settings to force ranking would hide ambiguity/provenance
   changes and do not constitute an honest explicit-selection contract.
4. **No direct numerical adaptation.** Keep current Suggest ranking/FS3.9 behavior.
   Explicit-use mode offers similar/derived records for inspection only if application
   would scale/interpolate or transfer beyond the reviewed direct scope. PRESET-2
   remains an explicit new-Draft workflow, never an applicability override.
5. **Drafts remain inspect/duplicate only** in the proposed first slice. Current
   resolver excludes them. A checkbox must not make a Draft eligible by altering its
   evidence; allowing direct manual Draft use needs a distinct reviewed policy/schema.
6. **Provenance decision:** optionally add selected profile identity/revision to the
   privacy-safe saved summary. Assess older native readers (`deny_unknown_fields`),
   generated schemas/transports and portable project compatibility before any change.
   No project-root format bump or migration is assumed authorized.
7. Existing explicit Apply is a deliberate action, but lacks the requested per-field
   current/proposed overwrite review. Add clear acknowledgement for replacing manual
   values without silently mutating setup. Keep source evidence descriptive; an
   operation using a Validated source does not become physically validated itself.

At least item 2 changes the generated API schema
([FsCamRequest](../../schema/openapi.yaml)); item 6 may also touch persisted metadata.
**This is the required schema HARD STOP, not a model/tool limitation.** No ADR is
silently accepted and no schema/implementation commit has been made.

## 6. Regression evidence and unperformed work

Log receipts normalize trailing whitespace only.
All checks below rerun on the actual pre-existing PRESET-1/2 worktree; they qualify
existing behavior and the audited baseline, **not a nonexistent PRESET-3 implementation**.

| Command/check | Result |
|---|---|
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-feeds-speeds` | **113 integration + 1 doctest PASS**; local priority, physical/reference identities, drafts/deletion, ambiguity/order, reference derivation, rights/data validation — [log](preset3-audit-2026-10-21-evidence/feeds-speeds-tests.log) |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --lib feeds_speeds` | **19 PASS**; preview immutability, explicit apply, private provenance, round trip, manual roles, stale context, no live inheritance — [log](preset3-audit-2026-10-21-evidence/adapter-tests.log) |
| `npm test` (Node 24 PATH) | **1488 PASS / 1 existing locale skip / 0 FAIL**, 168 frontend files, plus **10 WASM guards PASS** — [log](preset3-audit-2026-10-21-evidence/frontend-tests.log) |
| `npm run check` | Svelte **0 errors / 0 warnings**, node TypeScript PASS — [log](preset3-audit-2026-10-21-evidence/frontend-check.log) |
| `npm run build` with existing offline Rust/WASM toolchain PATH | PASS; existing bundle-size/dynamic-import/timing notices retained — [log](preset3-audit-2026-10-21-evidence/frontend-build.log) |
| Documentation links and scoped diff | Checked for this audit; [links](preset3-audit-2026-10-21-evidence/doc-links.json), [preservation](preset3-audit-2026-10-21-evidence/preservation.json) |

The current unchanged component-size guard **passes** in the rerun. Historical V1.1
failure (OperationsList1008/FeedsSpeedsSection1010) was fixed by the preceding
PRESET-1 extraction, with budgets unchanged. No hidden waiver or expanded budget.
Frontend tests include existing command/history, source/preset, preview cancellation,
project serialization, generation key and mocked export-service guard regressions.
The direct FS command test asserts apply/revert; the general History suite covers
Redo. No claim of a new end-to-end selected-preset Undo/Redo test.

Because implementation stopped: no two-entry-point state equivalence, selected-record
preview/apply equality, Draft acknowledgement flow, explicit overwrite confirmation or
new chooser UI has been tested. No new browser/native GUI walkthrough was necessary
for this static audit. No physical cutting, installed-platform or V1 release acceptance.
No production-code lint changes; no release/pre-release workflow launched.

## 7. Git and next decision

Only this report, its receipts, and narrowly reviewed audit documentation additions
are eligible for a local documentation commit. Existing PRESET-1/2 implementation,
reports/evidence and their dirty documentation edits are preserved outside that commit.
The commit hash is reported in the chat handoff; the audited starting revision above
remains the source of the test results. No push, merge, rebase, reset or V1 modification.

The next V1.1 action is **human review of the minimal PRESET-3 request/provenance
contract**, followed by explicit implementation authorization. The new schema gate
is separate from still-open PRESET-2 review. MAT-1 remains planned/unstarted; do not
launch it, restart PRESET-2, or infer acceptance of existing release/workshop gates.
