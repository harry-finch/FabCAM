# ADR-0030: FabCAM operation parameters and independent FabSpeed calculation

Status: **Accepted — explicit maintainer approval, 2026-10-22**
Date: 2026-10-22
Branch: `FabCAM-v1.1`
Audit: [FS_RESET_ARCHITECTURE](../../releases/FS_RESET_ARCHITECTURE.md), `ivac-j47dy`.
Human architecture gate: **`ivac-909v6` CLOSED by explicit approval**.

## Context

The maintainer explicitly chooses to stop calculating/recommending feeds and speeds in
FabCAM. FabSpeed is an independent calculator/experimentation application with an API.
FabCAM has no external distributed-user compatibility obligation; breaking old project and
source-pack contracts is accepted. This does not authorize deleting user data or changing V1.

The current implementation combines an independent FS crate, a core adapter, source catalogs,
ranking/adaptation, local immutable revisions, reference/physical identities and PRESET-1–4
workflows. Some ordinary CAM requirements are now owned by FS-named modules: authoritative
saved rate slots, tool maximum RPM application and generation-time validation. Those must
survive replacement independently of recommendation and preset presence.

PRESET-1 library organization and MAT-1 material discovery remain useful. PRESET-2/4 inactive
recipes, evidence promotion and source registry complexity are superseded for the new product.
Old reports and ADRs remain truthful historical records, not implementation requirements.

## Accepted decision

1. FabCAM core owns **explicit operation cutting parameters per actual tool/pass role**.
   Preserve rough/finish, distinct Pocket finish tool, Drill/spot/chamfer and specialized
   operation semantics. Tool defaults may seed creation visibly once; generation never
   substitutes them for missing new-format milling values. Incomplete editable operations
   can be saved but cannot generate. Geometry/depth/engagement keep one executable owner.
2. Introduce neutral core cutting-parameter and preset-validation modules. Move relevant
   FS numeric, geometry, role and machine/tool-limit checks there before deleting FS.
   Known out-of-bound milling values are rejected, not clamped; unknown ratings remain
   explicitly unknown with existing specialized mandatory requirements intact. This policy
   intentionally replaces the old general clamps and the removed trial-only completeness gate.
3. Presets are current **workshop-tested configurations** in named private collections,
   with stable ID/name, exact machine/tool/material/operation context, relevant geometry,
   explicit values and a human tested attestation. Date/description/notes are optional.
   No recommendation rank, Draft/Tested/Validated promotion or immutable evidence archive.
4. Load means preview and explicitly copy validated values through the existing atomic
   operation/history boundary. It never changes Stock, selected tools or machine and never
   creates a live link. Exact context is required for tested-preset loading; similar records
   remain inspect/edit/duplicate choices, without inferred equivalence or numerical scaling.
5. Save a compact loaded-preset baseline separately from executable operation values.
   Changed values/context display **Modified from preset**, including after reopen.
   Library edits/deletion cannot modify saved operations. New or materially edited preset
   saves require renewed human attestation; import/FabSpeed/software success is not that act.
6. Keep top-level Presets and a compact direct Cutting parameters inspector. Keep MAT-1
   family/variant/alias/unknown-material presentation, decoupled from source packs. Taxonomy
   and search never change material identity or establish cutting compatibility.
7. Use one proposed portable project envelope, `ivac-project` **version 9**, retaining all
   unrelated geometry/CAM features and rejecting versions 1–8 before state mutation.
   No old-project, source-pack, evidence or experimental-recipe migration/compatibility API.
   New native requests must also require explicit new contracts, without legacy fallbacks.
8. Store current collections as bounded versioned JSON, atomically replaced with optimistic
   concurrency tokens, in dedicated app-data/IndexedDB namespaces. No source revision archive.
   Keep generic workspace/tool/machine functionality and existing physical identity values;
   a bounded explicit generic-asset import may be needed if workspace fields change.
   Preserve old data in place; cleanup is optional and separately confirmed after inventory.
9. Remove `ivac-feeds-speeds`, recommendation/adaptation/ranking/source infrastructure,
   old transport APIs, Sienci assets, v8 trial persistence and obsolete tests once all retained
   consumers use the new contracts. Regenerate schemas/clients/WASM and retain safety gates.
10. A later optional FabSpeed adapter may exchange versioned explicit values and minimal
    metadata, require review, and reuse core validation/application. No integration, direct
    calculator-crate dependency, automatic apply or algorithm duplication is part of FS-RESET.

## Supersession boundaries

On architecture approval, this decision supersedes recommendation/source/evidence ownership
for **V1.1 FS-RESET** in ADR-0002/0003/0004/0005/0006/0011 and the reference-pack or inactive
recipe machinery of ADR-0027/0028/0029. It preserves their valuable constraints on explicit
operation authority, physical identities, truthful provenance and independent CAM safety.
Their recorded states and V1 qualification are not rewritten retroactively. PRESET-4 v8
trial lifecycle is replaced rather than migrated. This proposal does not accept prior UX,
security, platform or physical gates and does not authorize implementation before review.

## Alternatives considered

- **Hide the old engine behind a new UI:** rejected by the product decision; retains dead
  contracts, ranking and sources and fails to establish simple ownership.
- **Single global operation/tool rate:** rejected; loses finish/spot/drill and internal
  toolchange semantics and risks false simulation/provenance.
- **Permanent live link to tool defaults or presets:** rejected; library edits would silently
  change saved jobs and defeat reproducibility.
- **Preserve/migrate all old formats:** rejected; no external compatibility need and extensive
  engineering without future product value. Preserve original files and Git history instead.
- **Delete FS modules before moving guards:** rejected; generation currently depends on them.
- **Embed FabSpeed core or require its API:** rejected; contradicts independent applications
  and offline ordinary CAM operation.
- **Unrestricted import-and-apply of similar presets:** rejected; inspection/manual editing
  remains possible without presenting transferred values as the tested combination.

## Consequences

The product loses automatic recommendations, manufacturer source catalogs and evidence ranking
by design. It gains direct operation ownership and a smaller library/storage/application flow.
The change is a substantial fork delta affecting persistence, public APIs and generated clients;
it must stay on V1.1. Valid in-bounds explicit parameters should preserve comparable toolpath
and output; old clamps, unsupported formats and removed recommendation workflows deliberately
do not. Those differences require a written exclusion ledger rather than weakened tests.

Imported old theoretical wiki packs are not automatically tested presets. No data file is
silently deleted. New physical tests are not authorized. Collection attestation records a user
statement; it is never proof that arbitrary future toolpaths or machine state are safe.

## Validation and approval

FS-RESET-1 is documentation/audit only; source-bound baseline checks and preservation evidence
are linked from the report. **HARD STOP for human architecture approval.**

After approval, FS-RESET-2 uses qualified local commits for core contracts/storage, operation
editing, library and integrated role behavior. FS-RESET-3 removes legacy code and runs the
complete relevant gate: Rust all-features, strict/pedantic Clippy, frontend types/lint/tests,
actual WASM/JSON round trips, schema/codegen, NUMSAFE, SAFE-Z, ShopBot/provenance/import,
full-program checks, transactional export, TEST1 including C2 and pre-release.

Qualify unchanged valid program output against the established corpus; investigate differences.
Keep native scope checks, truthful two-sided partial publication and CLI streaming limitations.
Human desktop/narrow UX acceptance follows software qualification; merge/release and installed
platform/physical acceptance remain separate. Revisit the design if it cannot represent an
existing supported role without hidden defaults or duplicate executable values.

## Implementation status — FS-RESET-2

All four approved slices are implemented: neutral explicit role/limit validation, current
collection storage, v9 save/open and public generation boundaries, direct operation editing,
reviewed copy/attestation, tested collection library, neutral MAT-1 and specialized actual-role
dispatch. [Implementation report](../../releases/FS_RESET_2_IMPLEMENTATION.md) records contracts,
commits, comparisons and the qualification matrix. The final affected core/frontend/HTTP/CLI
checks pass; two unchanged native watcher tests fail by missing events and remain unresolved.
Native installed dialogs and a real browser file round-trip are not qualified.

The current Profile emitter consumes Finish values for contours; Rough values remain explicit
and stored. Automatic two-sided dowel injection is refused in v9 because it has no explicit
operation-owned cutting values; explicit Drill operations are the supported preparation path.
Unknown ratings remain disclosed, with specialized mandatory restrictions unchanged. No
FabSpeed integration, data migration/deletion, version promotion, publication or physical
operation was implemented.

Legacy crate/modules/APIs/source assets and internal old-contract test/Setup consumers remain
for separately reviewed FS-RESET-3 removal. New product behavior does not wrap the recommendation
engine. **HARD STOP: human review `ivac-y25vq` OPEN**. Engineering completion does not authorize
mass removal, close platform/physical gates or grant release acceptance.
