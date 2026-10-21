# PRESET-2 — strict Duplicate & Adapt checkpoint

**Current disposition:** accepted and integrated locally on 2026-10-21.
[Commit/qualification receipt](PRESET_INTEGRATION_2026_10_21.md). The dated observations below describe
the original pre-integration snapshot, including then-open gates and historical failures.

Date: 2026-10-21. Branch **FabCAM-v1.1**, HEAD **df6eb1f** plus preserved
uncommitted PRESET-1 work and this intervention. Engineering `ivac-h5nln`;
**human review `ivac-1oe9l` OPEN**. No commit, push, merge or publication.

The user accepted PRESET-1, then explicitly approved the strict framework in
[ADR-0028](../shopbot-modernization/adr/ADR-0028-preset-duplicate-adapt.md) before
implementation. Architecture approval is separate from acceptance of this software
checkpoint. MAT-1, scaling research and subsequent milestones remain unstarted.

## Delivered behavior

Presets → select source → **Duplicate & adapt…** → choose a private local pack,
existing physical cutter and material → review differences and copied values →
**Save new Draft**. An explicitly created empty destination pack can remain after
cancel; cancelling the copy creates no preset. The current physical machine is
read-only in this dialog. Associations are fixed through existing editors.

The source and operations remain unchanged. The material choice is local to the
copy form; it does not select job material. Selecting a tool does not stock it on
any machine. No recommendation engine, CAM or post-processing call is involved.
Original RPM/feed or chipload/plunge/DOC/stepover data are copied unchanged.
Source reduction rules are retained as provenance only, never recalculated or
applied. A chipload source does not acquire an invented feed.

| Boundary | Implemented rule |
|---|---|
| Identity/process | Complete physical machine/tool and material identities; milling capability; ambiguous legacy identities blocked |
| Machine | Local recipe must explicitly cover the same physical machine; references require a matching reference/declared membership |
| Material | Exact ID/revision or an explicit existing membership/taxonomy relationship; no label-based equivalence |
| Cutter | Same supported known cutting family; finite positive effective diameter and integer flute count; known flute length must cover copied DOC |
| Diameter/flutes | Differences shown and require review; no scaling, calculation or appropriateness/physical qualification claim |
| Rights | Reference requires manifest transformation permission; authored private local recipe without that grant requires explicit ownership/copy authorization; private storage alone grants nothing |
| Result | Independent UUID and exact target context; Draft/theoretical/disabled; test date, operator/job claim, review, physical booleans, evidence record and history cleared |
| Provenance | Source pack/profile/record IDs and revisions, original manifest/restrictions, classification/evidence label, source reference/author/date/license/attribution/notes, original scope/data/reductions, target references, differences and local authorization retained in existing source notes |
| Concurrent changes | Source/destination revisions pinned; freshness checked before save; stale context refuses without silently rebasing or partial preset writes |

Changing any form value invalidates the review checkbox. Missing context or rights
keeps save disabled with explanations. Long provenance notes are collapsed behind
an explicit disclosure rather than dominating the details pane. EN/DE strings are
provided; the existing experimental French locale is not expanded here.

## Storage and implementation

- `preset-adaptation.ts`: pure eligibility/difference/copy functions; JSON clones
  accept Svelte proxies and preserve source data. Existing `revisionPack` and
  `addCatalog` are reused, including rejection of inconsistent catalog properties
  under a reused physical identity. Existing recorded tool RPM limits are retained.
- `DuplicatePresetDialog.svelte`: pinned source/destination/machine/tool snapshots,
  explicit local rights and value review, local material selection, independent save.
- Existing native/IndexedDB private pack store remains authoritative. The new native
  `save_adapted_lab_pack` command takes source and destination write locks in sorted,
  deduplicated order through immutable compare-and-append. Same-pack copy uses one
  lock. Browser source/destination checks and add share one readwrite transaction.
  Existing size/count limits and immutable history remain unchanged.
- The separate native command deliberately fails closed with an older installed
  backend. A native application rebuild is required to use this new command; this
  report does not claim an installed Tauri GUI replay.
- No production CAM/FS mathematics, project format/schema, permission/CSP policy,
  machine output, source reference catalog or release artifact was changed.

## Validation and evidence

| Check | Result / receipt |
|---|---|
| Full frontend | **1488 PASS, 1 existing locale skip, 0 FAIL**, 168 test files; plus **10 WASM freshness guards PASS** — [log](preset2-2026-10-21-evidence/frontend-tests.log) |
| New pure copy tests | Six cases cover provenance/reset/no mutation, chipload/dimension changes, explicit material relations, copy rights, incompatible/incomplete contexts and catalog consistency |
| Svelte/TypeScript | **0 errors / 0 warnings**, node TypeScript also passes — [log](preset2-2026-10-21-evidence/frontend-check.log) |
| Native storage | **8 PASS**, including source/destination CAS, source lock contention and same-pack copy — [log](preset2-2026-10-21-evidence/native-storage-tests.log) |
| Native Clippy | `RUSTC_WRAPPER='' cargo clippy --offline -p ivac-tauri --lib -- -D warnings` PASS — [log](preset2-2026-10-21-evidence/native-clippy.log) |
| Scoped ESLint/Prettier | PASS — [lint](preset2-2026-10-21-evidence/scoped-eslint.log), [format](preset2-2026-10-21-evidence/scoped-prettier.log) |
| Production frontend build | PASS with existing bundle-size/dynamic-import notices and timing diagnostics — [log](preset2-2026-10-21-evidence/frontend-build.log) |

Isolated browser origin `http://127.0.0.1:18813/`, using agent-created synthetic
packs and current WASM validation, without touching the user's 5173 application:

1. Missing tool/material and absent machine compatibility block save with reasons.
2. Explicit reference association permits the recorded material relationship.
   Source Ø6.35/one-flute → target Ø3/two-flute differences are visible; values
   17000 rpm / 1620 feed / 810 plunge / DOC2.8 remain unchanged. These are copied
   **software test data, not recommended or physically qualified rates**.
   [Reference review](preset2-2026-10-21-evidence/reference-review.png).
3. Save passes the actual WASM pack validator; the independent record appears as
   inactive Draft. It cannot provide recommendations.
4. Local copy requires rights attestation; changing observations clears prior review.
5. A second window saves a new revision. The first window's pinned form refuses
   with “Source, destination or context changed”; no extra record is created.
   [Refusal](preset2-2026-10-21-evidence/stale-copy-refused.png).
6. Final implementation replay copies the local derivative into Fablab Z with the
   complete retained provenance, passes the real validator and persists across
   development reloads. [Final review](preset2-2026-10-21-evidence/final-review.png),
   [saved inactive Draft](preset2-2026-10-21-evidence/final-saved-inactive-draft.png).

UI replay is browser software evidence. Native storage was compiled/tested, but
installed native application behavior, Windows/Linux acceptance, physical cutting,
rate appropriateness and ShopBot qualification are **not** established here.

## Preservation and checkpoint

The pre-intervention hash inventory covered **11,162 tracked/untracked files**;
none disappeared. Only the scoped source, ADR/index/status/roadmap/log files changed;
previous qualification reports/evidence and other PRESET-1 changes were retained.
[Preservation receipt](preset2-2026-10-21-evidence/preservation.json).
No historical report was rewritten to claim new qualification. Existing V1/SEC1,
platform and physical workshop gates remain independent.

Human review should inspect copy semantics, strict compatibility explanations,
rights wording, differences and retained provenance under ADR-0028. Broader geometry,
permissive incomplete drafts or cross-machine transfer require a new reviewed scope.
**HARD STOP:** no next milestone, commit or publication. If accepted, the next planned
milestone is [MAT-1](RELEASE_ROADMAP.md#mat-1--material-families-variants-and-synonyms):
clearer navigation of material families, variants and synonyms while preserving
saved identities and avoiding inferred machining equivalence.
