# FS-RESET-2 — explicit operation values and tested collections

Branch: `FabCAM-v1.1`. Starting source: `184c2088beda0d7bb65a6ea97b7e19f6d28c658d`.
Architecture approved by the maintainer on 2026-10-22; ADR-0030 is Accepted.
Engineering tracker: `ivac-jcr1o`. Human FS-RESET-2 review / FS-RESET-3 authorization: **`ivac-y25vq` OPEN**.

## Implementation state

All four bounded implementation slices are implemented. The active milling workflow owns
explicit values, uses v9 persistence and offers reviewed copies from tested collections.
Software qualification passes for the affected CAM, frontend, HTTP, WASM, CLI and storage
contracts, with **two unresolved native file-watcher test failures** detailed below. This is
not a claim of installed-platform, complete release or physical machining qualification.

**HARD STOP after FS-RESET-2.** Legacy code is retained for staged internal dependencies;
there is no new recommendation wrapper. FS-RESET-3 removal requires separate explicit human
approval. No FabSpeed integration, push, merge, rebase, version promotion, installer, release
or physical machine operation was performed.

| Slice | Focused local commit |
|---|---|
| 1 — core/contracts/storage | `ac9e38961be7b047a35a0f8612c670f437b22321` |
| 2 — direct editing/reviewed copy/v9 | `b6a41d6251c490a26620b9f0e376debff8de2b7a` |
| 3 — collection library/neutral MAT-1 | `f681737760e7a63903ec3927a36962cfee4985c5` |
| 4 — actual-role dispatch/neutral safety/qualification | This implementation commit; exact SHA recorded in the final handoff update. |

Earlier slice sections record what was qualified at that point. The final contracts and
qualification matrix below supersede their then-pending implementation notes.

## Slice 1 — core, contracts, current-document storage

- `ivac-core::cutting_parameters`: closed rough/finish/drill/spot/chamfer roles, explicit
  nullable integer RPM/feed/plunge, structural validation separate from generation readiness,
  process/geometry/known-limit checks. New values live at `Op.params.cutting` (within the
  existing parameter bag); depth schedules and engagement remain in their original fields.
- `Project.cutting_version = 9` identifies new native values. The new core pipeline path
  refuses missing/duplicate/misassigned roles and known limits. Old internal projects remain
  on their old path during staged consumer migration; external old-request refusal is not
  claimed by this slice. V9 generation disables old operation-cache reuse conservatively.
- Physical tool/machine IDs and tool maximum RPM have neutral native fields. Generic existing
  library records are not rewritten. Initialization is an explicit creation helper, not a
  generation fallback. V9 step resolution cannot inherit a live tool step.
- `ivac-core::presets`: bounded named current collections, attested preset contexts, explicit
  schedule/rate payloads, exact reviewed copy and portable loaded baseline without private notes.
  Project-local tool numbers may differ when the physical context matches; rebind only the
  copied rate references to already-selected tools. Core limits are always rechecked.
- Pure preset/parameter commands are exposed through Tauri, HTTP and JSON-text WASM; the
  latter preserves nulls exactly. Schema and generated TypeScript are derived from Rust.
- `project-v9.ts` rejects v1–8/newer envelopes before mutation, retains geometry guards and
  permits incomplete numeric fields. Snapshot/restore activation belongs to slice 2.
- Native current collections use `presets/collections/<safe-id>.json`, a directory write
  lock, SHA-256 optimistic token, same-directory temporary write/sync and atomic replacement.
  Browser collections use a dedicated IndexedDB database and one compare/write transaction.
  Limits: 2 MiB, 32 collections, 256 presets per collection, 96-character safe IDs,
  160-byte names, 4096-byte notes and 65536-byte context signatures.
- No existing source-pack database, workspace, project or application-data file is deleted.
  Native commands accept IDs/content, never an arbitrary destination path. Program export
  filesystem authorization and `ivac-file-export` are unchanged.

Attestation is an author's claim, not a validator result or proof of testing on an importing
machine. UI attestation renewal, import disclosure, target pinning and Undo/Redo are slice 2/3
responsibilities; their presence is not inferred from the core schema.

## Qualification record

Source SHA and exact receipt for each committed slice are recorded in the following slice
or final handoff. Logs are in `fs-reset-2-evidence/`. These are software checks, not machining
qualification. The full removal/release gate remains a separate FS-RESET-3 prerequisite.

Slice 1 qualified checks: core explicit parameters **6 PASS**, preset contracts **5 PASS**,
native atomic storage **3 PASS**, TypeScript v9 **5 PASS**, actual WASM JSON **1 PASS**,
retained legacy setup resolver **21 PASS**. Affected libraries pass strict pedantic Clippy
with the established gate allows; server/WASM compile, schema/client generation, targeted
ESLint, frontend typecheck and production build PASS. Fresh WASM built offline using cached
wasm-bindgen 0.2.120 and wasm-opt; build fingerprint verified. Existing bundle-size/dynamic
import advisories remain; build does not establish runtime or installed-platform qualification.
 Existing explicit Rust fixture
constructors receive `None` for new optional fields; their assertions and numerical inputs
are unchanged. Small pre-existing pedantic findings (must-use annotation, identical match
arms, path-extension comparison and command-owned arguments) are corrected without changing
runtime behavior or weakening CAM checks.

## Storage limitations

Storage limitations: atomic replacement is per collection, not a multi-document transaction
or a claim of crash durability. A process killed while holding the native lock can leave a
busy lock requiring deliberate recovery; original collections remain recoverable. No active
application-data reset or recovery was attempted.

## Slice 2 — explicit editing, reviewed copying, v9 activation

Source: `ac9e389` (qualified slice 1). The inspector now exposes each declared role's
RPM/feed/plunge directly, with editable missing values and a separate software-validation
result. New operations copy tool defaults once. Adding a secondary role produces missing
values; changing its assigned tool retains authored values and revalidates the context.
Operation templates copy explicit values; they do not confer tested-preset attestation.

Load previews rates and depth/engagement changes, pins the complete operation/setup during
async validation, and applies one Undo/Redo command. Save requires an unchecked, current-context
workshop-test attestation independent of validator success. Loaded baselines distinguish manual,
loaded and modified values; private notes do not enter the operation snapshot. Generic tool and
machine editors expose workshop identity, without recommendation compatibility controls.

Save/open uses v9 exclusively. Incomplete values round-trip. Old versions are rejected before
mutation. Machine/tool physical identity and maximum RPM persist; generating does not read
live defaults. HTTP/Tauri/WASM/worker clients require `cutting-parameters-v9` from the engine
before submitting v9 machining requests, preventing an older running engine from ignoring them.
Public server-side old-payload refusal and specialized actual-role dispatch remain slice 4.

Qualification: **150 frontend tests PASS** across 12 affected files, including actual rebuilt
WASM inspect/load JSON, snapshot/restore, stale target rejection, Undo/Redo, templates, client
capabilities and program qualification. Core explicit-value tests **7 PASS**, preset contracts
**5 PASS**. Strict pedantic affected-library Clippy, targeted ESLint (two keyed-loop findings
corrected), frontend typecheck **0 errors / 0 warnings**, fresh WASM and production build PASS.
The pocket-overlap fractional bound is corrected to 0–1; negative cases cover 0, 1, >1 and NaN.
Exact preset context now includes machine configuration while rechecking mutable ratings.

Visual browser QA is pending: port 5173 was unavailable; opening a local listener is denied by
the sandbox (EPERM). The maintainer has been asked to start the existing dev script. No user
application-data file was modified for UI testing. Source-pack settings and the old library
remain temporarily active until the immediately following slice 3.

## Slice 3 — tested collection library and neutral MAT-1

Source: `b6a41d6` (qualified slice 2). The active Presets tab now has named collection
create/rename/delete, preset inspect/edit/duplicate/delete, capture from an existing operation,
reviewed load, and collection import/export. Both load surfaces share a display-only row model;
only core exact-context validation authorizes a copy. Deletion names the reviewed item and
preserves copied operation values. Every update/delete uses its reviewed optimistic token.
Import validates before writing and presents explicit replace-versus-independent-copy choices.

Rates, context or test-date edits require a fresh unchecked attestation bound to those values.
Name/notes edits and exact duplication retain the original author claim. A context can be
replaced explicitly from the selected operation. Imported attestations are displayed as source
claims, never current-workshop qualification. No live source links are created.

The active source enablement panel and old recommendation library are disconnected; their
implementation files remain for FS-RESET-3 review. MAT-1 now consumes a neutral 19-entry catalog
and collection-context projections, retaining family/variant aliases, exact revision identities,
unknown/custom selection, deterministic aggregation, search and pagination. The Sienci-specific
presentation mapping is deliberately removed; tests now assert unknown/custom preservation.
Machine/tool asset storage and existing application data are not reset.

Qualification: **37 frontend tests PASS** across seven affected files, covering attestation
renewal, explicit import collisions, immutable filtering, MAT-1, v9, real WASM and selected-file
read/write/cancel/scope-error behavior. Targeted ESLint (one formatter-induced optional-index
finding corrected), typecheck **0 errors / 0 warnings**, fresh-package production build PASS.
No Rust/WASM schema changed in this slice. Existing native CAS/path/storage qualification
continues to apply; mocked file-plugin tests do not establish installed native-dialog behavior.

Browser QA on the maintainer-started server: empty collection creation and rename persisted
successfully; desktop and 640px layouts captured in `slice3-library-*.jpg`. One explicitly named
empty software-QA collection remains in the browser's new store. No physical testing was claimed.
The browser's automatic authorization review refused the collection download, so the real
browser export/import loop was **not performed** and was not retried by another route.
Populated inspector/role workflow checks continue in slice 4. Native export of a collection uses
the existing dialog-granted filesystem plugin scope; it is a backup write, not a program-export
qualification or multi-file atomicity claim. `ivac-file-export` and its program guards are unchanged.


## Slice 4 — actual roles, public boundaries and integrated safety

Starting source: `f681737760e7a63903ec3927a36962cfee4985c5`; exact modified-source and
log SHA-256 values: [slice4-receipt.json](fs-reset-2-evidence/slice4-receipt.json).
Public HTTP, Tauri, WASM (buffered, two-sided, streaming and drawing generation) and serialized
CLI requests require cutting contract 9 and reject legacy source context before emission.
Internal legacy fixtures retain their path for comparison; it is not a public v1–8 migration.

### Final operation and preset contracts

- `Op.params.cutting.rates` is the closed set of `{role, tool_id, rpm, feed_mm_min,
  plunge_mm_min}` records. Roles are `rough`, `finish`, `drill`, `spot`, `chamfer` as actually
  declared by the operation. The three rate fields are nullable positive `u32` values:
  null permits editing/persistence but blocks enabled-operation generation. Duplicate,
  missing, stray and misassigned roles fail. Tool and operation IDs must be positive/unique.
- `entry_angle_deg` is explicit for V-Carve, Halfpipe and Drill chamfering. Existing depth,
  step/finish-step/nonuniform depth schedule, lateral engagement, plunge strategy and cycle
  fields keep one executable owner in the operation. Tool defaults seed creation only;
  changing a library default later cannot change generation. New roles remain incomplete
  until authored. No tool/machine/material substitution occurs while generating or loading.
- Preset parameters copy the reviewed rates, depth schedule, supported engagement and entry
  angle. Typed `specialized` payloads additionally copy Drill cycle/spot depth/chamfer width,
  Relief scallop/stepover/sampling and Waterline Z-step/stepover. Required specialized payloads
  cannot disappear; a load cannot add/remove Drill roles or choose different tools.
  Thread pitch/form/depth and other operation geometry remain exact compatibility context,
  not a second independently executable preset geometry model.
- Exact context includes physical machine identity/configuration, exact material identity
  and revision, physical cutter identities/geometry, operation kind/strategy/role presence
  and plunge strategy. Mutable machine/tool ratings are revalidated at load, not frozen as
  compatibility proof. Local tool numbers may differ; only rate references are rebound to
  the tools already selected by the user. Incompatible records can be inspected/edited but
  cannot be silently adapted or applied.
- The versioned collection holds an explicit author attestation separately from software
  validation. Material parameter/context/test-date edits require a renewed unchecked claim;
  imported claims never prove testing on this workshop machine. The operation stores a
  compact loaded baseline; subsequent library edits/deletion never mutate copied values.
- Frontend build serialization no longer manufactures missing Drill cycles or missing
  Relief/Waterline numbers. V9 save/open preserves incomplete editable operations, values,
  baselines and generic assets. Versions 1–8 are refused before state replacement; original
  files remain untouched. Test-only fixture construction creates new v9 projects from old
  geometry for regression tests and is not shipped as a migration API.

### Safety ownership and preserved guards

| Safety property | Final owner / evidence |
|---|---|
| Positive integral rates, required actual roles and assigned tools | `cutting_parameters::{validate_structure,rates,validate_operation}`; native and real WASM positive/negative cases |
| Machine RPM min/max, common XY/Z feed ceiling, neutral tool max RPM | `validate_limits`, repeated for resolved secondary setups before old internal clamp helpers; violations reject without changing authored values |
| Unknown ratings | Visible role warnings; no fabricated machine/tool rating. Unknown values do not override a known bound. Halfpipe still requires known flute length. |
| Raw/effective diameter, wear, flute count, known usable flute length and axial/pass reach | Neutral operation validator and existing specialized geometry checks; secondary tools validated independently |
| Explicit depth/engagement, finite values, depth schedules, entry angle, peck/dwell | Neutral operation validator plus retained pass/numeric budgets and driver constraints; missing peck no longer inherits a tool default |
| Specialized mandatory process/tool restrictions | Shared checks for Planar/V-Carve/Halfpipe/Relief/Waterline/undercut/Thread, plus existing full driver geometry checks. V9 rejects former reach/radius clamping cases. |
| SAFE-Z, work-origin, fixtures, holder/tool changes, output budgets | Existing pipeline/resolver, CAM and numeric emit guards retained; full core suite with CPS passes |
| Tool provenance, internal spot/chamfer/finish, front/back | Existing provenance subsystem retained; new distinct/shared-tool and two-sided tests assert resolved tool coverage |
| Generation freshness and simulation/export qualification | Existing frontend context and qualification gates retained; stale/failed generation cannot export. The new operation values participate in the document state. V9 old operation-cache reuse is disabled. |
| Native path authorization / file publication | Tauri `app.fs_scope().is_allowed(path)` remains mandatory for program publication; `ivac-file-export` unchanged and its 11 tests pass |

These are CAM guards independently of attestation or preset presence. The legacy FS validator
is still called only by the retained internal old-contract path; no new v9 milling generation
requires a recommendation, source pack or FS binding. Final removal is deliberately pending.

### Operation-role qualification matrix

| Operation / path | Explicit roles and dispatch | Proof and limits |
|---|---|---|
| Profile | Rough and Finish retained; the current offset emitter consumes **Finish** for Profile contours | Byte-identical retained output; inspector defaults to Finish and explains Rough storage. No new roughing strategy claimed. |
| Pocket | Rough plus same/distinct-tool Finish; native no-contour mode Rough only | Byte equality for unchanged inputs; distinct RPM/feed/toolchange and provenance assertions; missing secondary values refuse |
| Drill | Drill, optional Spot and optional Chamfer, independently even with the same cutter | Same/distinct-tool programs contain each authored RPM/plunge/feed; missing peck, invalid reach/cycles refuse; preset cycle round-trip |
| Chamfer | Rough, plus Finish when enabled | Byte equality; unsupported cutter and excessive cone width refuse |
| Engrave / V-Carve | Rough; V-Carve also explicit entry angle | Engrave byte equality; TEST1 V-Carve executable golden equality; finite/tool/strategy/radius-cap restrictions retained |
| PlanarSurface | Rough | Byte/toolpath-count comparison; flat endmill/direct entry/stepover and known reach restrictions |
| Halfpipe Pocket | Rough only; explicit entry; no secondary finish | Byte/toolpath-count comparison; flute length, matching Ball/V geometry and step minimum mandatory; no silent depth clipping |
| Relief / Waterline | Rough plus operation-owned sampling/step fields | Byte/toolpath-count comparison; invalid sampling/Z/step/reach refuses; specialized preset parameters copied explicitly |
| Thread / TSlot / Dovetail | Rough, operation-specific geometry unchanged | Byte/toolpath-count comparisons; positive pitch/radial count and form-profile restrictions; existing thread compensation retained |
| Program-only / non-milling | No milling role set; process-specific guards remain | Retained core regressions; milling values rejected for non-milling context. This does not newly qualify laser/knife/waterjet. |
| Helix and other inherited variants | Single-role mapping where applicable | Retained core tests only; no new broad machining qualification or independent golden claim |
| Two-sided | Each side keeps its operation/tool roles | Unchanged front/back byte equality and resolved provenance; invalid back values refuse; restrictions below |
| Streaming | Same explicit values before write | Native streamed/buffered bytes agree; missing role rejected before writing; real CLI/WASM transport tests. No full-program simulation claim. |

### Output comparison and exclusion ledger

`pipeline/cutting_v9_tests.rs` adds 12 integrated tests, alongside 7 neutral contract tests
and 6 preset tests in the final core suite. Identical valid inputs produce byte-identical
programs in the retained native path and v9 for Profile, Pocket, Engrave, Chamfer, Planar,
Thread, TSlot, Dovetail, Halfpipe, Relief, Waterline and front/back examples. Toolpath entry
counts also agree where asserted; this is not an independent geometric proof for every shape.
TEST1 Profile and V-Carve match the established `.sbp` executable lines; historical comment
headers/build IDs are excluded from that golden comparison. Native legacy/v9 full strings
also agree for those requests. **No unexplained valid-output difference remains in this set.**

Intentional changes: authored secondary-role values now govern the actual block; missing
rates/cycles/entry values and known limit/reach violations refuse instead of falling back or
clamping. Profile's existing Finish consumption is disclosed rather than changing its path
algorithm. Existing tests of old document acceptance are replaced by v9 preservation tests
and explicit v1–8 rejection tests; historical fixture files and independent assertions stay.
Legacy inactive recommendation tests now supply their own old context instead of depending
on the new active builder. No safety threshold, invariant, golden file or test allowlist was
relaxed. One pre-existing language-coverage skip remains.

**Automatic two-sided dowel injection is refused in v9.** That implicit operation reads tool
library defaults and was already outside full-program qualification. The active Stock panel
no longer creates it; existing automatic-hole configuration is disclosed and requires explicit
disabling. Registration must be prepared using explicit Drill operations. Ordinary two-sided
jobs without injected holes retain output. Existing automatic-CPS, translated-XY and back-side
surfacing restrictions are not broadened. Review this product boundary at the human gate.

### Filesystem, two-sided publication and streaming boundaries

Native collection storage accepts safe IDs in its dedicated directory, with symlink/traversal,
size/count/schema and stale-write checks; it exposes no arbitrary native write command.
Collection import/export reuses the existing dialog-granted filesystem plugin scope. The
program-export destination authorization, helper executable boundary and capabilities were
not widened. Collection backup writes do not inherit program-export guarantees.

`ivac-file-export` guarantees per-file staging/publication, not transactional publication of
the two-sided pair. A second-side publication failure may leave the first complete file in
place; existing error reporting and the second-side-failure regression remain unchanged.
Desktop sides retain independent qualification. Nothing here converts publication success
into machining qualification or asserts crash durability for the pair.

CLI `stream-gcode` now refuses old serialized projects and missing explicit values before
publication. A real CLI regression preserves an existing destination on refusal, compares
valid output with buffered generation and checks staging cleanup. Streaming still lacks the
buffered full-program preview/simulation contract; stdout cannot roll back emitted bytes and
CPS/HPGL streaming restrictions remain. CLI geometry-to-Setup generation remains an internal
legacy consumer to assess during FS-RESET-3, not a project-v1–8 compatibility promise.

### Final qualification, commands and evidence

Commands ran offline with `RUSTC_WRAPPER=''` at the slice-4 source receipt; frontend commands
run from `frontend/`. Cached wasm-bindgen 0.2.120 / wasm-opt were used for a fresh package.
Counts below overlap (the full core suite includes focused cases); do not add them as unique tests.

| Command / check | Result | Evidence |
|---|---|---|
| `cargo test --offline -p ivac-core --features cps` | **1463 PASS**: 1255 library + 208 integration, no failures | `slice4-core-full.log` |
| `npm run test` with final fresh WASM | **185 files, 1580 PASS, 1 existing skip**; package guards **10 PASS** | `slice4-frontend-full.log` |
| `cargo test --offline -p ivac-server --bin ivac-server` | **19 PASS**, including old-payload refusal across generation routes | `slice4-http-tests.log` |
| `cargo test --offline -p ivac-cli --test cutting_v9` | **1 PASS** real executable integration | `slice4-cli-tests.log` |
| `cargo test --offline -p ivac-tauri --lib` | **19 PASS / 2 FAIL**, commands and collection storage pass | `slice4-native-final.log` |
| Native watcher-only recheck | **3 PASS / same 2 FAIL** | `slice4-watcher-recheck.log` |
| `cargo test --offline -p ivac-file-export -p ivac-tauri --lib` initial run | File export **11 PASS**; native initial failures followed by fixes/final result above | `slice4-native-tests.log` |
| Strict affected-library pedantic Clippy with existing gate allows | **PASS**, `-D warnings`; exact command in receipt | `slice4-clippy.log` |
| `npm run check`, targeted ESLint/Prettier; `cargo fmt --all --check` | **PASS**, typecheck 0 errors / 0 warnings | `slice4-frontend-check.log`, `slice4-eslint.log`, `slice4-format.log`, `slice4-rustfmt.log` |
| Schema/client/i18n code generation; fresh WASM + `npm run build`; native server build | **PASS**; existing bundle-size/dynamic-import advisories | `slice4-schema.log`, `slice4-i18n-codegen.log`, `slice4-build.log`, `slice4-server-build.log` |

The two native failures are `watcher::tests::single_path_emits_event` (2-second timeout) and
`watcher::tests::debounce_collapses_rapid_writes` (zero notifications). The watcher source is
unchanged from the starting revision. They repeat in isolation in this environment, but a
baseline comparison outside the sandbox has not established their cause. They are unresolved
qualification exceptions, not silently skipped tests or evidence of a fully green native suite.
The initial native run also exposed three stale test-fixture contracts; those were updated to
explicit v9 without weakening their collision/error/recovery assertions, and now pass.

Browser checks: collection create/rename, desktop and 640px layouts; example Profile direct
editing, missing Finish RPM refusal, stale program/export disabled, Undo restoration and
separate unchecked save attestation with a specific missing-material message. See
[slice4-incomplete-operation.jpg](fs-reset-2-evidence/slice4-incomplete-operation.jpg).
Tests used a new example and one clearly named empty QA collection; no machining attestation
was fabricated. The browser authorization review refused a collection download (no approved
local download authority in that action); no real browser export/import loop or alternative
route was attempted. Native dialogs have mocked boundary tests, not installed-app acceptance.

The maintainer restarted `scripts/dev-web.command`. The browser confirmed the native `/version`
response includes `cutting-parameters-v9`; listeners on 8766/5173 remain running. The local
shell cannot connect through this sandbox, so the browser response is the runtime evidence.
The ordinary test URL remains `http://127.0.0.1:5173/`; the agent's example tab uses `?api=wasm`.
The displayed beta.3/version strings were not promoted. Full all-features/pre-release/C2 and
installed three-platform gates are **not** claimed by this bounded FS-RESET-2 qualification.

### Remaining legacy dependencies and exact FS-RESET-3 prerequisites

| Retained dependency | Why still present / required review before removal |
|---|---|
| `ivac-feeds-speeds`, core `feeds_speeds` and old schema fields | Internal old-contract fixture/Setup consumers and output comparisons; v9 validation/rates use the neutral module. Identify every remaining consumer before deletion. |
| Old HTTP/Tauri/WASM recommendation/source APIs, capability flag and generated types | Temporarily retained transport/build compatibility inside the staged implementation. Active new product UI does not call them; remove coherently in FS-RESET-3. |
| Old source-pack stores, Sienci assets/catalogs, inactive recommendation/library/recipe UI and tests | Still on disk/in the repository; active Presets/MAT-1 path is replaced. Inventory code/assets separately from protected existing application data. |
| Generic asset legacy identity/reference metadata | Preserved machine/tool records may retain old fields; neutral physical IDs govern current presets. Retain useful generic inventory and avoid destructive asset migration. |
| Internal `Project` with absent cutting version and CLI Setup path | Deliberate staged implementation/test bridge. Public serialized projects require v9; remove fallback only after replacing its internal callers/proofs. |
| Historical reports, V1 evidence, old user projects and source data | Preserve permanently as historical/user data, regardless of later removal of executable compatibility code. |

Before FS-RESET-3: human review **`ivac-y25vq`** must explicitly approve this report, four
implementation commits, UX/attestation model, role/output matrix and the automatic-dowel
boundary. Resolve or explicitly disposition the two watcher failures with source-bound native
host evidence; complete or explicitly retain the pending installed-dialog and real browser
file round-trip qualification. Inventory remaining consumers and assign retained safety tests
before authorizing deletion. Refresh the exact combined-source baseline and plan the complete
architecture-report gate (all-features Rust, Clippy, frontend types/lint/tests, WASM/schema,
NUMSAFE, SAFE-Z, import/ShopBot/provenance, full-program/export, TEST1 including C2, pre-release).
Human approval is a prerequisite to starting removal; remaining release/physical/platform
acceptance is separate and cannot be inferred from that approval.

Preservation: no application-data reset, old-project migration/deletion, history rewrite or
worktree modification was performed. The two pre-existing local documentation edits remain
unstaged; task-only checkpoint edits are staged separately. Existing untracked historical
reports/evidence and the other worktrees remain in place. The final preservation receipt
records the inventory comparison. **Stop here; FS-RESET-3 has not begun.**
