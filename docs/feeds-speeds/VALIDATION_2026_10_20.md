# Custom data-source guide — validation and handoff

2026-10-20 · work `ivac-sq3it` · human review **ivac-7w1e1 OPEN**.
[Guide](CUSTOM_DATA_SOURCES.md) · [Examples](examples/README.md).

## Snapshot and scope

Application source: `94cdd6bbdb5887ffce48adb86b8ea8a088402ac9`, beta.3/pipeline63.
Documentation work starts from `d63aa89` on `codex/docs-consolidation`, in the
isolated `FabCAM-docs-consolidation` worktree. Primary NUMSAFE `ivac-knphk` remains
paused/uncommitted and excluded. No production code, existing tests, schemas,
application behavior, registry, private data or evidence-retention policy changed.
No machine trial, source installation, application database write, push or merge.

## Implementation audit

Current English labels and conditions were checked in source, not inferred from
old documentation. No new live GUI or native-platform campaign was performed.
The existing software-only FS3.8 image was visually inspected and explicitly marked
historical in the guide; it is not a physical-use or current-shell screenshot.

| Capability / limitation | Actual implementation inspected |
|---|---|
| Save as preset… / Save cutting preset, Draft/Tested, stale-context guard | [FeedsSpeedsSection](../../frontend/src/lib/components/op_properties/FeedsSpeedsSection.svelte), [capture dialog](../../frontend/src/lib/components/LabPresetDialog.svelte), [lab state](../../frontend/src/lib/state/lab-presets.ts) |
| Stable machine/tool IDs, reference compatibility; custom material text becomes identity | [Compatibility picker](../../frontend/src/lib/components/FeedsSpeedsCompatibility.svelte), [MaterialPicker](../../frontend/src/lib/components/MaterialPicker.svelte) |
| Enable/import local packs, import adds enabled reference, no priority controls | [Source settings](../../frontend/src/lib/components/FeedsSpeedsSourcesSettings.svelte), [source selection](../../frontend/src/lib/state/feeds-speeds-sources.ts) |
| One enabled revision per generic source | `enabledPacks` in source selection; no automatic generic head substitution |
| Reserved Sorbonne authoring ID/policy, promotion/history, notes/disable/delete/restore | [Lab manager](../../frontend/src/lib/components/LabPresetManager.svelte), lab state and [Rust pack policy](../../crates/ivac-feeds-speeds/src/source_pack.rs) |
| Dedicated lab export; generic export service not exposed in Settings | [File service](../../frontend/src/lib/services/file_ops.ts), lab manager, source settings |
| Local immutable storage, limits and import validation | [Browser/native adapter](../../frontend/src/lib/state/feeds-speeds-pack-store.ts), [native storage](../../crates/ivac-tauri/src/source_packs.rs) |
| Schema1–3, identity limits, one origin/profile, physical-evidence constraints | [Actual loader](../../crates/ivac-feeds-speeds/src/source_pack.rs), [dataset validation](../../crates/ivac-feeds-speeds/src/dataset.rs), [model](../../crates/ivac-feeds-speeds/src/model.rs) |
| Only supported Profile/Pocket CAM adapter contexts; full-width Profile maps to full_slot | [CAM adapter](../../crates/ivac-core/src/feeds_speeds.rs) |

The guide corrects old-label assumptions: current capture button is **Save as preset…**,
compatibility section **Reference compatibility**, manager **Lab presets**, promotion
**Promote to Validated…**. No generic UI authoring/export/priority feature is invented.

## Automatic example validation

Command from repository root, with the documented Rust toolchain/cached dependencies:

```sh
python3 docs/feeds-speeds/examples/validate_examples.py
```

Result: **PASS for both complete source packs** using the actual
`ivac_feeds_speeds::load_source_pack_json_checked` and `recommend_packs_checked`.
The temporary Cargo harness is seeded with the source checkout's lockfile; target
artifacts are under the OS temporary directory, not either worktree's `target/`.

- Three exactly matching contexts per revision: **6 available results**.
- Unknown manifest field, inconsistent evidence promotion and missing origin are
  rejected for each pack: **6 negative controls**.
- Stable pack/catalog/record IDs, new immutable revision and unchanged other records
  across v1→v2: **PASS**.
- All profiles remain explicitly synthetic ReferenceData/Theoretical with no trial
  record. Neither pack is eligible for the public bundled registry.
- Passing structural validation cannot establish truth of a physical attestation.

| File | SHA-256 |
|---|---|
| `examples/workshop-demo-v1.json` | `5ff03916f0a8bb3e44891fb8e398aa7d5f41d3ef2b9fad6c6296504792dd253e` |
| `examples/workshop-demo-v2.json` | `10322439a6f358f4fa9bb6e13635496af2c01687a8a30b56920b08a68c36e61b` |

Focused existing engine regressions, with a temporary target directory:

```sh
RUSTC_WRAPPER='' cargo test --offline --locked -p ivac-feeds-speeds \
  --test source_packs --test lab_presets --test approximate \
  --target-dir /tmp/fabcam-fs-guide-target
```

**50 PASS / 0 failures**: source_packs21, lab_presets8, approximate21. These are
software/source-pack tests, not physical qualification, UI execution or full CAM
regression. No production tests were edited and no full release gate is claimed.

## Documentation and preservation checks

Relative links (**1757 targets**), used Markdown anchors (**19**), README parity
(**12 headings**) and `git diff --check` pass.
Quickstart, central documentation index and FS technical index link to the guide.
Example JSON is independent of `frontend/public` and the bundled registry.
Primary source/evidence hashes are compared against the existing 11009-file baseline;
NUMSAFE files and all 8211 local evidence files remain untouched. Only scoped guide,
examples, navigation and checkpoint Markdown/helper files are staged for this commit.

## Genuine UX limits — observations, no implementation authorized

- Captured local databases are hardcoded to Sorbonne Fablab and its reserved ID/policy;
  no generic named-workshop or multi-workshop authoring UI exists.
- Generic external sources require hand-authored JSON; no schema-aware wizard/editor.
- Settings exposes lab backup but no generic imported-pack export, edit or pruning UI.
- Import normally enables a generic revision; disable the old revision first to avoid
  the one-enabled-version conflict. There is no merge/rollback assistant.
- Other material… uses editable text as ID with revision job-1, making renaming and
  cross-workstation identity discipline less approachable than versioned catalogs.
- Source precedence is deterministic engine policy, not user-configurable ordering;
  this is a limitation to explain, not a reason to bypass compatibility/evidence.
- Pack/history/storage limits need explicit backup/migration planning; no automatic
  history compaction or cloud sync. No retention change is proposed or enacted here.

After NUMSAFE or future UI/schema changes, recheck labels/adapter/loader/storage
contracts before refreshing this guide. Existing consolidation review `ivac-yg3sm`
and all CAM/physical gates remain independent. **HARD STOP after documentation commit**;
no UX implementation, machine trial, push, merge or publication follows automatically.
