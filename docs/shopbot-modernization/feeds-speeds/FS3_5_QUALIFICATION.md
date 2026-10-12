# FS3.5 — UX and domain model qualification

Date: 2026-10-12. Engineering complete; **human acceptance pending** (ivac-eh1g).
This checkpoint stops before FS4. No physical machining, new calibrated preset,
recommendation mathematics change or ShopBot post change is included.

## Problems found and audit

The default FS3 panel mixed source loading, job material, recommendation internals
and applied rough/finish roles. Physical tool editing presented context cutting
defaults as primary properties. Machine tooling led with inventory bookkeeping.
A mandatory recommendation in AppliedCutting prevented manual-only operation
values without inventing provenance. [Pre-implementation audit](FS3_5_AUDIT.md)
traces storage, serialization, UI and CAM consumers. The smallest persistent
extension is documented in [ADR-0004](../adr/ADR-0004-manual-operation-cutting-and-source-settings.md).

## Changes and ownership

**Tool != CuttingParameters != Recommendation.** Physical cutter geometry remains
ToolEntry. Existing tool RPM/feed/plunge/finish/drill/step/overlap values remain
unchanged fallback defaults for historical projects. Resolved operation rates
use existing AppliedCutting slots; native step/depth_list/xy_overlap remain the
only geometry authority. Recommendation/provenance is now optional: manually
entered values can exist without a dataset, evidence claim or engine call.

Stock remains dimensions/placement. Existing job material identity is edited
once in Stock and inherited by every operation. An optional portable display
label preserves readable material names when sources are disabled; native
matching receives only id/revision. Unknown material does not acquire cutting
properties. Machine/tool reference compatibility is explicit secondary setup,
never inferred from names, dimensions or a supposed ShopBot/LongMill equivalence.

Default milling Profile/Pocket panels show current spindle, feed, plunge,
pass depth and pocket stepover. Modified fields use text labels. Profile displays
its actual contour/finish role; Pocket's rough cut is primary and its native
finishing role is collapsed. Depth schedules, entry/direction controls, corner
feed behaviour and detailed provenance are advanced disclosures. Physical tool
selection, tabs, leads and native geometry controls remain available.

Suggest/Review opens a focused known-context assistant. Calculate and Apply are
explicit. Suggested values are labeled **not yet applied**; aggressiveness is
independent of source evidence. Context changes show “Recommendation out of
date” and preserve cutting values. A stale preview cannot be applied. Changing to a cutter without saved rates
shows a blocking explanation and requests explicit manual entry or Apply; no
new cutter defaults are silently inherited. Save validates complete positive
rate blocks so an unfinished manual entry cannot produce an unreopenable file. No
calculation occurs at load, context edit, settings edit or CAM generation.

Settings → Feeds & Speeds configures Sienci and imported sources once per
installation through existing ivac.settings persistence. Disabled/unavailable
sources give an assistant state, not per-operation dataset loaders. Multiple
available sources remain explicit choices; no new cross-source precedence.
Tool Library defaults to physical columns and moves cutting defaults behind
“Fallback cutting defaults”. Machine Tooling leads with “Tools available on
this machine” and “Add tool from library”; inventory/profile/id safeguards stay.

## Data model and backward compatibility

- Native AppliedCutting.proposal becomes optional; existing recommended JSON
  still deserializes. Manual-only blocks have no recommendation stale warning.
- New portable saves containing cutting/material/reference context use
  **ivac-project-cutting/version 3**. v1 and FS3 v2 remain readable unchanged.
  Distinct kind protects older FS3 applications that require proposal.
- No file is rewritten at open; no old tool defaults are removed or reinterpreted.
  Machine/tool reference metadata falls back to legacy fsContext only when
  undefined; explicit null clears the binding. Project material display labels
  are UI metadata, not matching rules or source data.
- The core adapter, output consumers, geometry algorithms and rates math retain
  FS3 semantics. Non-milling panels and inheritance remain on their legacy path.

## Qualification evidence

| Check | Result |
|---|---|
| Frontend `npm test` | 110 files; 1,065 pass, one optional skip |
| `npm run check` | Svelte/TypeScript: zero errors/warnings |
| Core `cargo test -p ivac-core feeds_speeds --all-features --locked --offline -- --test-threads=1` | 10 pass |
| ShopBot `cargo test -p ivac-core --test shopbot_m2_post --all-features --locked --offline -- --test-threads=1` | 8 pass, including real CAM/manual spindle regression |
| `cargo clippy -p ivac-core -p ivac-feeds-speeds --all-features --all-targets --locked --offline -- -D warnings` | pass |
| `cargo check -p ivac-server -p ivac-tauri --all-features --locked --offline` | pass |
| `cargo xtask schema-check`; `cargo fmt --all --check` | pass |
| Targeted ESLint/Prettier; Git whitespace | pass |
| `wasm-pack build crates/ivac-wasm --target web --release`; frontend `npm run build` | pass |

New cases cover manual-only operation save/reopen without fabricated provenance,
unchanged values after changing library defaults, accepted-proposal preservation,
explicit reference clearing, cutter changes without silent inheritance, actual Settings storage persistence, disabled-source
no I/O, configured source loading, legacy Settings normalization, malformed
import, and material display metadata excluded from native matching. Existing
Apply/stale/rate-role/legacy process and tool availability coverage stays green.
The new native manual-only case serializes/reloads and compares generated output
byte-for-byte with the equivalent accepted-rate project, after changing tool
fallback defaults and removing source/material/recommendation context.

FS3's broader serial workspace baseline was 1,218 passes; it was not rerun for
this small model/UI change. FS1 engine arithmetic and the six licensed source
profiles were not edited. No native release app bundle was created for FS3.5;
software browser qualification uses rebuilt release WASM/frontend.

An initial sandbox-blocked WASM build left bindings without package metadata.
Rebuilding the complete wasm-pack package succeeded; the subsequent frontend
production build passed. This was a build artifact repair, not a code/data change.

## Real browser workflow and screenshots

On the local release app, opened the software fixture, selected job material,
opened Profile and selected its available physical cutter. Suggest initially
left current values at 10,000 / 1,000 / 500. Explicit full-slot confirmation and
Calculate produced 17,000 rpm / 1,620 feed / 810 plunge, DOC 2.8, chipload 0.0953,
Prudent and Reference data/Sienci in separate fields. Apply wrote those values.
Manual feed 1,500 was labeled Modified; changing DOC to 2 marked the recommendation
out of date without changing RPM/feed/plunge. Actual browser Save downloaded
v3 JSON with finish feed 1,500/manual and step -2. The retained
[actual saved project](FS3_5-software-manual-saved.ivac-project.json) reopened
with the same values even after disabling Sienci. Settings disable persisted
through page reload; Recalculate displayed no enabled sources and disabled
Calculate. Sienci was restored afterward. No automatic recalculation was observed.

- [Recommendation before Apply](screenshots/FS3_5-recommendation.jpg)
- [Manual field and stale recommendation](screenshots/FS3_5-manual-stale.jpg)
- [Global source settings](screenshots/FS3_5-source-settings.jpg)
- [Physical tool library](screenshots/FS3_5-tool-library.jpg)
- [Machine tools available first](screenshots/FS3_5-machine-tooling.jpg)

[Human software review](FS3_5_TEST_LOGICIEL.md) supplies the same short workflow.
Screenshots and automated actions establish software behavior; they do not
replace a normal fablab user's usability acceptance.

## Files modified in this checkpoint

Core: feeds_speeds.rs, pipeline/setup_resolver.rs; schema/openapi.yaml and
frontend/api/generated.ts. Frontend: build-project.ts and its tests; operation
FeedsSpeedsSection/OpPropertiesPanel/PocketSection; StockPanel/JobMaterial;
SettingsDialog/FeedsSpeedsSourcesSettings; MachineWorkspace/MachineTooling;
ToolLibraryDialog/ToolRowSummary/ToolRowExpandedEditor/FeedsSpeedsCompatibility;
state feeds-speeds helpers/tests/persistence/source manager/source settings tests,
commands/project-data/project-types/project-file-ops; services/file_ops save validation; English/German i18n keys
and hardcoded-unit allowlist. Documentation: audit, ADR-0004, this qualification,
human protocol, software fixtures/screenshots, STATUS/ROADMAP/DEVLOG,
ARCHITECTURE/CAM_VALIDATION/UPSTREAM_ANALYSIS/FS README. Existing uncommitted
FS0–FS3 changes remain in the shared tree; no checkpoint commit/push is authorized.

## Remaining debt and FS4 readiness

Generate G-code still both computes toolpaths and produces a selected dialect.
Tracked follow-up ivac-xz59: a process-neutral Generate toolpaths with machine-aware export is preferable in
a later scoped UX change; FS3.5 leaves behavior/wording intact. Physical tool
manufacturer/material/coating/hard-limit schema is not expanded in this cleanup;
existing geometry and known machine/reference limits remain authoritative.
Source import remains full JSON in advanced Settings; no source editor, cloud
catalog or silent general-material fallback is provided. Tracked follow-up ivac-xf9w: current source failures
can make the configured assistant unavailable; manual rates and generation remain
usable. FS-bearing cache bypass and partial build-freshness guard remain existing
debt. English/German are the repository's existing locales.

**The workflow is clear enough for human usability review and, after that
approval, to prepare supervised physical fablab validation.** This is an
engineering judgment, not human acceptance or machine clearance. The only
shipped cutting profiles are narrow Sienci LongMill full-slot references: there
is still no ShopBot preset, arbitrary plywood equivalence, Pocket transfer or
local Tested/Validated evidence. FS4 must establish actual lab context, known
constraints, protocol and applicability before physical work or evidence promotion.

**STOP — waiting for human FS3.5 approval before starting FS4.**
