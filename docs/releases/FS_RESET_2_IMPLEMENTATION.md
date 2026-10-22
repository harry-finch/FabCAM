# FS-RESET-2 — explicit operation values and tested collections

Branch: `FabCAM-v1.1`. Starting source: `184c2088beda0d7bb65a6ea97b7e19f6d28c658d`.
Architecture approved by the maintainer on 2026-10-22; ADR-0030 is Accepted.
Engineering tracker: `ivac-jcr1o`. This report is updated with each qualified slice.

## Implementation state

Slices 1–2 establish the contracts/storage and explicit operation workflow. Slices 3–4 remain pending;
this intermediate commit does not claim that the product UI or every driver has switched.
The legacy engine is retained temporarily; there is no new recommendation wrapper.
FS-RESET-3 removal, FabSpeed integration, publication and physical operation are unauthorized.

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

## Remaining boundaries

- Wire all creation/edit/reopen and generation clients to v9; remove active recommendation UI.
- Implement reviewed Load/Save history and simple tested collection workflows.
- Preserve neutral MAT-1 presentation without active manufacturer source catalogs.
- Complete specialized driver role dispatch and compare identical valid outputs.
- Require native new-contract payloads at public boundaries; qualify both sides and streaming.
- Before FS-RESET-3: review the completed four-slice report, operation matrix, output differences,
  UX evidence and remaining dependency inventory. Only a separate explicit approval authorizes
  mass removal. Legacy engines, endpoints, source formats and historical evidence stay retained
  until then; temporary retention must not become new product behavior.

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
