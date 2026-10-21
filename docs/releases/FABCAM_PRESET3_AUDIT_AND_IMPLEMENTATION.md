# PRESET-3 — Unified Cutting Parameters Selector

**Current checkpoint (2026-10-21): selector software/UX accepted and integrated locally.**
API `5ca56321bd7201eeda6dd5b777966c3e82882e8b` is preserved; native guard `bed3a92`
and frontend `b8deaef` integrate PRESET-1/2/3 and the approved UX refinement.
[Acceptance, commits and current qualification](PRESET_INTEGRATION_2026_10_21.md). No push or release;
MAT-1 remains unstarted. Sections below retain historical source/approval snapshots;
statements about pending review or uncommitted work are superseded by this checkpoint.

## Historical Phase A audit

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


## Approved implementation contract — 2026-10-21

The maintainer now authorizes the minimal additive request extension. Starting HEAD
1935878c305df02cccf526a5899207e05efc78fc on FabCAM-v1.1, with existing uncommitted
PRESET-1/2 changes preserved in a local fingerprint/patch/byte snapshot.

FsCamRequest gains optional `selected_record: FsSelectedRecord` (omitted/null means
unchanged automatic recommendation). FsSelectedRecord uses `pack: FsIdentity`,
`record_id: string`, `profile: FsIdentity`; both identities include id and revision.
No source-pack, project-root or AppliedCutting/KnowledgeSummary format changes.
Old requests deserialize unchanged; old servers reject the new field, so generated
clients and bundled WASM/native contracts must be updated together. Existing private
provenance retains values/pack revision/record ID without claiming full offline
profile-revision identity. Human schema gate ivac-ms6rz remains open.

Explicit selection validates original packs, requires a unique exact pack/record/
profile match and existing eligibility, evaluates that record through the same Engine,
and bypasses only candidate ranking, reference adaptation and interpolation. No pack
rewriting or fallback. Existing hard machine/tool clamps still apply and are shown.
Preview uses the loaded, validated source snapshot. Apply rereads stored sources before
and after authoritative validation and compares source/context fingerprints before
committing the existing atomic applyCuttingCommand. Detectable
changes fail closed. Cross-backend read→apply is not a storage transaction; a write
after final read remains outside the guaranteed detection window, never a live link.


## Implementation results — 2026-10-21

### Architecture and reuse

The optional `selected_record` carries canonical pack and profile identities (each
ID + revision) and the original record ID. Rust resolves that original record only,
without rewriting source bodies, reference adaptation, interpolation or fallback.
The absent-selector path retains existing automatic ranking and derivation. Both
paths retain pack validation, exact compatibility, hard tool/machine clamps and the
existing CAM adapter. OpenAPI and generated TypeScript were regenerated together.
No pack format, persisted project schema, dependency, output contract or V1 change.

The inspector now opens **Choose cutting parameters…**, with **Recommended** and
**Browse presets** sharing the same comparison, Rust preview/apply and operation
command. Browse reuses PRESET-1 filtering/details and PRESET-2 Duplicate & adapt.
Library **Use in operation** offers existing compatible Profile/Pocket operations
and opens that same selector with the original record pinned. Draft, disabled,
deleted and external-reference records cannot be applied. Incompatible records
remain inspectable; source context is not substituted into the operation.

New small modules handle selection identity, fresh source reads and the shared
apply lifecycle; comparison and explanatory UI were extracted from the existing
inspector to preserve its unchanged component-size budget. Manual-value replacement
requires acknowledgment. The existing apply command had dropped explicit undefined
patch fields during JSON cloning, resurrecting cleared manual overrides. Its patch
copy now preserves those fields; regression tests cover application, Undo/Redo,
reopen and generation-context invalidation. No alternative parameter engine added.

### Integrity, compatibility and privacy

Apply verifies the selected source snapshot and original operation object/context,
rechecks sources before and after Rust validation, and rejects detected modification,
removal, cancellation or replacement without committing operation values. Stored
cutting values remain snapshots, never live source links. Private source bodies and
trial notes are not copied into portable projects. Existing KnowledgeSummary does
not retain complete offline profile-revision identity; no stronger claim is made.

Cross-backend reads and operation commit are not a storage transaction: a concurrent
write after the final read can escape detection. This residual window is explicit.
Legacy requests without the optional field keep their behavior; older backends do
not support the new explicit selector and must be upgraded with their client.

### Qualification

Evidence: local `dist-test/preset3-implementation-2026-10-21/`, including logs,
source fingerprints, baseline snapshots, screenshots and isolated UI patch.

| Check | Result |
|---|---|
| Full `ivac-feeds-speeds` tests | 119 tests + 1 doctest PASS |
| Full `ivac-core --all-features` tests | 1434 PASS, including existing output/ShopBot regressions |
| Latest adapter tests after private-provenance assertion | 20 PASS |
| Clippy, both crates/all targets/all features, warnings denied | PASS |
| Server + Tauri compile | PASS |
| Schema check and generated-client reproduction | PASS |
| Frontend | 170 files, 1498 PASS, 1 pre-existing skip, 0 failures |
| Svelte/TypeScript and lint | PASS; 0 errors / 0 warnings |
| WASM freshness and production build | PASS; existing bundle-size/import/timing notices retained |

Targeted tests cover automatic/explicit equality, lower-ranked direct selection,
Tested/Validated eligibility, unknown/stale revisions, disabled/deleted/Draft
refusal, incompatible context, no reference adaptation, manual acknowledgment,
source and context races, save/reopen, privacy and output-freshness invalidation.
A real WASM regression exercises the shared request adapter. HTTP/Tauri forwarding
is compile-checked; no new installed-native or live HTTP runtime qualification is
claimed. Full core tests preceded the final test-only privacy assertion, which was
then verified in the 20-test adapter rerun. No physical cutting or export execution.

Visual walkthrough used a disposable SOFTWARE ONLY project and the real WASM
backend: inspector recommendation/comparison, manual acknowledgment, Apply,
Undo/Redo, library entry/preselection, incompatible selection and disabled Apply,
plus narrow and desktop layouts. The last layout pass corrected shrinking result
rows. Screenshots are review evidence, not human UX acceptance. The final library
screenshot shows the shared comparison before application.

### Git boundary and human handoff

Starting source: `1935878c305df02cccf526a5899207e05efc78fc` on `FabCAM-v1.1`.
Focused API commit: **`5ca56321bd7201eeda6dd5b777966c3e82882e8b`** — seven files:
Rust adapter/resolver, three Rust test files, OpenAPI and generated API types.

By explicit maintainer decision, UI, command fix, frontend tests and documentation
remain uncommitted pending PRESET-1/2 integration. The isolated UI patch is relative
to the preserved dirty PRESET-1/2 baseline, not a standalone patch against HEAD.
Concurrent Splitter/library resizing work was preserved and excluded from that
patch and the API commit. No push, merge, tag, release, packaging or V1 changes.

**HARD STOP:** human PRESET-3 UX/schema review remains pending (`ivac-ms6rz` open),
independent of PRESET-2 review (`ivac-1oe9l`). Next proposed step: review this selector
and authorize later frontend integration once PRESET-1/2 are committed. MAT-1 and
future parameter-scaling investigation remain unstarted.

## Unified selector UX refinement — 2026-10-21

**Implemented and frontend/WASM-qualified; human UX acceptance pending.**
Task `ivac-vyuab`. Source: `FabCAM-v1.1`, HEAD
`5ca56321bd7201eeda6dd5b777966c3e82882e8b` plus the existing dirty PRESET-1/2/3 tree.
The API commit is preserved. No Rust, schemas, source formats, tests of machine
output, security policies, versions or release workflows were modified in this pass.
[Source fingerprints](preset3-selector-ux-2026-10-21-evidence/source-fingerprints.json)
and [tracked Git state](preset3-selector-ux-2026-10-21-evidence/git-status-tracked.txt)
identify this checkpoint; these are not a clean-checkout qualification.

### Problems and changes

The previous selector repeated long context descriptions, used large three-line
preset cards, exposed recommendation aggressiveness while browsing a pinned record,
and repeated “Unchanged” in a fourth comparison column, including as a proposed
numeric value. Its material editor applied a separate job-wide command before the
parameter transaction.

- `CuttingSelectorFrame.svelte` owns a compact context header, independently scrolling
  content and a persistent shared footer. Width is capped at 960px and height at 85vh;
  height remains content-driven. Measured Recommended at 1280×720: **960×562px**, with
  content height/scroll height both 444px (no internal scrolling needed).
- `FeedsSpeedsSection.svelte` retains preview/application ownership. Both modes use
  the same comparison and **Cancel | Apply parameters** footer. Recommended retains
  its mode picker; Browse restores the selected record's authored mode even after
  visiting Recommended. Machine/tool/material context is visible in the header;
  full labels and details remain accessible. Source headings truncate rather than
  expanding indefinitely. Component size is **938 lines**, within the
  unchanged 1000-line budget.
- Material is now read-only in the selector. The explicitly authorized fallback in
  the brief is used because the prior material action was a separate history command,
  not part of validated parameter application. Stock remains the authoritative editor;
  its job-wide effect is stated. Missing material blocks preview/application and points
  to Stock. There is no new material transaction or implicit context substitution.
- `PresetBrowser.svelte` uses compact aligned rows for record/material/tool, mode,
  pack/status and compatibility. Search and all existing filters remain. Below 700px,
  secondary filters are disclosed under **More filters**, with an active-filter hint.
  Long labels truncate with complete titles/details. Arrow keys/Home/End move focus;
  Enter/Space explicitly select. Focus movement alone never selects. Filtered-out
  selection is retained with an explicit notice, rather than silently replaced.
- Compatibility is authoritative only for the selected record. Other eligible rows
  say **Check**; Draft/disabled/reference-only rows say **Inspect only**. Selected
  rows distinguish checking, compatible and incompatible. No client-side compatibility
  or numerical adaptation engine was introduced.
- `CuttingParameterComparison.svelte` has exactly three columns. Changed proposed
  values receive a restrained highlight. Profile stepover is N/A; absent optional
  proposals retain current values, marked with an explanatory footnote; unavailable
  values use a dash with a tooltip. A no-change summary explicitly explains the
  provenance/origin refresh. Identical numerical values do not incorrectly disable Apply.
- `cutting-comparison.ts` contains presentation/focus helpers, with seven new tests.
  English/German copy was added and obsolete keys removed; generated keys were
  regenerated. No new dependency or component-budget waiver.

### Preserved application boundaries

Both entry points still pin the original pack/record/profile identities and revisions,
use the same Rust preview and authoritative Apply request, verify source snapshots
before/after validation, and check operation/context freshness. Failures retain the
window and diagnostic. Manual replacement requires acknowledgment; full-width slotting
is never inferred. Undo/Redo, generation/export freshness and stored/private provenance
remain governed by the existing application command and tests. Removing the material
picker removes the temporary draft/pending-material state, not a required validity check.
The documented final-read/concurrent-write residual window remains unchanged.

### Visual and interaction evidence

Real local browser with `?api=wasm`, disposable SOFTWARE ONLY project, no CAM generation,
export or hardware action. Default viewport **1280×720**; narrow viewport **680×800**,
reset afterward. The scrolling list contains **11 existing synthetic/reference records**;
automated list coverage additionally exercises 180 long-label rows. These screenshots
are software review evidence, not human acceptance or a physical cutting trial.

| Scenario | Before | After |
|---|---|---|
| Recommended, desktop | [Before](preset3-selector-ux-2026-10-21-evidence/before-recommended.png) | [After](preset3-selector-ux-2026-10-21-evidence/recommended-after.png) |
| Browse, desktop | [Before](preset3-selector-ux-2026-10-21-evidence/before-browse.png) | [After](preset3-selector-ux-2026-10-21-evidence/browse-after.png) |
| Recommended, narrow | [Before](preset3-selector-ux-2026-10-21-evidence/before-recommended-narrow.png) | [After](preset3-selector-ux-2026-10-21-evidence/recommended-narrow-after.png) |
| Browse, narrow | [Before](preset3-selector-ux-2026-10-21-evidence/before-browse-narrow.png) | [After](preset3-selector-ux-2026-10-21-evidence/browse-narrow-after.png) |
| Compatible selection | [Before](preset3-selector-ux-2026-10-21-evidence/before-compatible.png) | [After](preset3-selector-ux-2026-10-21-evidence/compatible-after.png) |
| Incompatible / disabled Apply | [Before](preset3-selector-ux-2026-10-21-evidence/before-incompatible.png) | [After](preset3-selector-ux-2026-10-21-evidence/incompatible-after.png) |
| Manual feed 1000 → 1620 | [Before](preset3-selector-ux-2026-10-21-evidence/before-changed-manual.png) | [After](preset3-selector-ux-2026-10-21-evidence/changed-manual-after.png) |

Before desktop captures were taken before edits. Remaining before captures replayed
copies of the three original components in a temporary frontend snapshot, with only
removed baseline translation keys restored there; current repository sources were
not reverted. The temporary replay needed its own Vite filesystem allowance for the
existing linked WASM package; that setup correction did not modify repository policy.

Additional replayed paths: [library entry](preset3-selector-ux-2026-10-21-evidence/library-entry-after.png),
[empty filters with retained comparison](preset3-selector-ux-2026-10-21-evidence/filter-empty-after.png),
[full-width assumption refusal](preset3-selector-ux-2026-10-21-evidence/assumption-refused.png),
[missing material refusal](preset3-selector-ux-2026-10-21-evidence/missing-material-after.png),
[Undo restoring manual feed 1000](preset3-selector-ux-2026-10-21-evidence/undo-after.png),
[Redo restoring 1620](preset3-selector-ux-2026-10-21-evidence/redo-after.png),
[long list / End focus without selecting](preset3-selector-ux-2026-10-21-evidence/long-list-after.png),
and [scrolled narrow view with visible footer](preset3-selector-ux-2026-10-21-evidence/browse-narrow-scrolled-after.png).
The final two-mode/viewport replay after the reactive focus-reference correction
reported **no console errors or warnings**. Layout measurements are retained in
[metrics](preset3-selector-ux-2026-10-21-evidence/layout-metrics.json).

### Final automated qualification

| Command | Result |
|---|---|
| Node 24 `npm test` | **1505 PASS, 1 existing locale skip, 0 FAIL**, 171 files; **10 WASM package guards PASS** — [log](preset3-selector-ux-2026-10-21-evidence/frontend-tests.log) |
| `npm run check` | Svelte **0 errors / 0 warnings** and node TypeScript PASS — [log](preset3-selector-ux-2026-10-21-evidence/frontend-check.log) |
| `npm run lint` | ESLint and full Prettier PASS — [log](preset3-selector-ux-2026-10-21-evidence/frontend-lint.log) |
| Offline `npm run build` with existing Rust/WASM toolchain PATH | PASS; existing chunk-size, dynamic-import and plugin-timing notices retained — [log](preset3-selector-ux-2026-10-21-evidence/frontend-build.log) |
| Scoped diff / report references | `git diff --check` PASS; report local file targets checked |

New tests exercise numeric/retained/N/A/unavailable/zero presentation, keyboard
boundaries and 180-row long-label filtering without changing pinned identity.
Existing selector, source race, acknowledgment, history, serialization and
output-freshness tests remain in the full run. Interactive mode switching, comparison
synchronization, filter disclosure, focus/activation, material/full-width handling,
disabled Apply and both entry points were exercised in the actual browser; no new
DOM test harness or automated installed-native test is claimed. Rust and physical
qualification are the unchanged source-bound results above, not newly rerun here.

### Limitations, integration and hard stop

The compact list deliberately checks compatibility on selection rather than issuing
one Rust request per visible row. Very narrow screens may scroll the list horizontally.
Crossing the application's existing 1024px responsive boundary can unmount an
inspector-owned dialog; the library-owned selector was used for continuous narrow
resize review. Changing that shared responsive ownership is outside this focused pass.
Long Browse results and expanded provenance require scrolling, while footer actions
remain visible. Platform/font and human workshop acceptance remain open.

**No files staged; no commit, push, merge, rebase or release.** The source already
contains overlapping uncommitted PRESET-1/2/3 UI, command, test and native-storage
changes. A standalone refinement commit would depend on uncommitted components and
could inadvertently absorb their implementation. All were preserved. The
[incremental component patch](preset3-selector-ux-2026-10-21-evidence/refinement-existing-components.patch)
is relative to the actual dirty starting components, not independently applicable to HEAD.

Integration plan, after the relevant human approvals:

1. Review/commit the existing PRESET-1 library/navigation/layout and its tests/docs as
   `feat(presets): add preset library`; preserve the prior table/pane resizing work.
2. Review/commit PRESET-2 storage/copy workflow, dialogs, MaterialPicker draft-context
   copy and tests/docs as `feat(presets): duplicate records into inactive drafts`.
3. Review the remaining PRESET-3 selector, shared command correction, freshness tests,
   these presentation components/helpers/translations and evidence together; commit
   as `feat(presets): add unified cutting parameter selector`. Keep API commit
   `5ca5632` intact; do not cherry-pick incomplete UI fragments.
4. Review staged paths/hunks against the preserved baselines, rerun frontend/type/lint/
   build and relevant existing native/adapter checks on the assembled source, then
   record exact hashes. Commit sequencing itself remains to be executed separately.

**HARD STOP for human UX review.** No acceptance gate was closed automatically.
MAT-1 (material families, variants and search synonyms over existing identities)
remains the next proposed feature milestone, unstarted and requiring authorization.
