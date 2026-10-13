# Architecture at M0

Audit reference: upstream `9168104b4fc4e02e916740a54bd1e56b5019f662`, version
0.5.0. Cross-check with upstream [architecture](../ARCHITECTURE.md) and
[contributing recipes](../CONTRIBUTING.md).

```text
Svelte 5 / TypeScript frontend
  state + command history → api/build-project.ts → generated wire contract
  HTTP / Tauri IPC / WASM adapters
                         ↓
ivac-core: Project + SVG/DXF/text import → Segment geometry
  chaining / containment → offsets / operation drivers
  pipeline setup, ordering, caches, warnings
                         ↓
PostProcessor calls ──→ native LinuxCNC / GRBL / HPGL text
                 └──→ CpsRecorder → Program IR → ivac-cps / Boa → text
                         ↓
Toolpath preview + material/fixture/holder simulation → frontend
```

The current engine is not a completely separate universal IR subsystem. Path
planning, motion emission and post calls interact in the core pipeline; the CPS
recorder creates a structured IR from those calls. Avoid inventing a parallel
engine before testing these seams.

## Workspace map

| Area | Role / source |
| --- | --- |
| `crates/ivac-core` | Import, geometry, CAM, pipeline, project models, simulation and native posts |
| `crates/ivac-cps` | Optional Boa JS runtime, CPS metadata, versioned IR, prelude, bundled posts |
| `crates/ivac-cli` | Direct headless pipeline caller |
| `crates/ivac-server` | Axum HTTP and streaming transport |
| `crates/ivac-tauri` | Tauri 2 commands, native files/dialogs/watcher, platform shell |
| `crates/ivac-wasm` | wasm-bindgen pipeline/simulation adapter |
| `xtask` | Build/CI/schema/version tooling; alias in `.cargo/config.toml` |
| `frontend/src/lib/api` | Client interface and HTTP/Tauri/WASM adapters; generated.ts |
| `frontend/src/lib/state` | Svelte state slices, commands/history, project/machine/tool helpers |
| `frontend/src/lib/components` | Operation, stock, tools, machine and simulation UI |
| `schema/openapi.yaml` | Checked-in schema generated from Rust JsonSchema types |
| `scripts`, `.github/workflows` | Build/release/quality automation |

## Core boundaries

- `input.rs`, `input/svg_in.rs`, `input/dxf_in.rs`: import to shared segments.
  Dependencies: usvg and dxf; geometry offsets use cavalier_contours and
  clipper2-rust; V-carve uses voronator (`ivac-core/Cargo.toml`).
- `project.rs` and `project/{op,params,machine,tool,fixture,config}.rs`: Serde and
  JsonSchema wire types. Project embeds segments, tools, machine, operations,
  fixtures and stock-related setup. Compatibility defaults/aliases matter.
- `pipeline.rs` and `pipeline/`: resolve setup/source selection, dispatch drivers,
  cache results, coordinate events, emit warnings and assemble the result.
- `cam/offsets/`, `cam/chaining.rs`, `cam/setup.rs`: topology, compensation,
  pocket fill and resolved configuration; specialized drivers use additional
  CAM modules. Profile/Pocket/Engrave share the contour path.
- `gcode.rs`: `PostProcessor`, motion/arc/cycle hooks and cut-block shells;
  `gcode/entry.rs`, `leads.rs`, `tabs.rs`, `z_schedule.rs`, `walk.rs`: emission.
- `gcode/preview.rs` and `sim/`: toolpath segments, dexel/heightmap material
  removal, sweep, holder and fixture checking, rapid checks and timing. These
  are software models; they are not proof of safe real-machine operation.
- `frontend/src/lib/services/file_ops.ts`: project, toolset and machine file
  save/load. `state/machine_profiles.ts`: named machine+tools profile bundles.
  `state/commands.ts`, `history.ts`: undo/redo.

## Schema, CI and portability

Public wire changes require Rust schema regeneration (`cargo xtask schema`),
frontend codegen and compatibility tests. Never edit generated.ts manually.
CI tests/lints Rust on Linux/macOS/Windows while excluding Tauri from the fast
logic job; desktop release workflows build the shell. CI also covers WASM,
frontend, schema/version drift, README parity and dependency/license checks.
Ordinary feature pushes do not trigger this CI; PRs, v* tags and manual dispatch
do (`.github/workflows/ci.yml`). No new workflow was added in M0.

## Architecture impact

M0 changes documentation and installs the supplied local skills only. No core
types, schema, algorithm, post or UI behavior changed. ShopBot should extend
existing post seams; ProtoMAX needs explicit process semantics later, assessed
separately in FUTURE_PROCESSES. No architecture ADR selecting a new subsystem
is warranted yet.


M1 keeps these boundaries. Qualification adds static fixture inputs and analytic
integration tests using existing import/pipeline/preview/simulation seams.
Minimal corrections stay in `geometry.rs` and existing pocket offset modules.
No serialized model, schema, cache-key contract or process interface changed;
no new subsystem/ADR required. ShopBot remains deferred to M2; ProtoMAX assessment
B is unaffected. Algorithmic behavior and limits are in M1_QUALIFICATION.


M2 extends the CPS library with `shopbot`; [ADR-0001](adr/ADR-0001-shopbot-cps.md)
records native/CPS tradeoffs. Recorder and section-builder optional metadata
expose existing machine scaffold policy/clearance/static-shift presence without
changing serialized types or IR version. The owned post checks IR v1 before
using existing dispatch. Generic CAM, simulation and public API are retained.
Pure WASM has no CPS; CPS-enabled transport is required. ProtoMAX process-event
gaps remain deferred; the ShopBot post introduces no universal process layer.

## Planned post-M4 milling assistant

The future [Feeds & Speeds subsystem](feeds-speeds/README.md) follows
[ADR-0002](adr/ADR-0002-feeds-speeds-boundary.md): CAM context → adapter → pure
milling recommendation engine → user acceptance/edit → explicit persisted
parameters → existing CAM/post flow. The engine is independent of UI and geometry
generation; post-processors never invoke it. WJ/VINYL remain independent of it.
Aggressiveness and evidence/provenance are separate; manual overrides are protected.

[FS0 qualification](feeds-speeds/FS0_QUALIFICATION.md) and accepted
[ADR-0003](adr/ADR-0003-feeds-speeds-data-ownership.md) specify domain snapshots,
material identity, scoped JSON data and explicit ambiguity/fallback behavior.
FS1 now implements the independent `ivac-feeds-speeds` crate, with Serde and serde_json
as normal dependencies. It owns no project state and is not called by CAM/posts.
See [FS1 qualification](feeds-speeds/FS1_QUALIFICATION.md) for concrete policy/API.

FS3 makes saved operation rates authoritative and retains native step/overlap
as the sole geometry fields. The core adapter is the only project-to-engine
boundary; HTTP/Tauri/WASM explicit endpoints call it. CAM generation reads saved
rate slots and never calls the engine. CPS metadata uses resolved active-pass
rates, including tool changes and plunge classification. Native schema additions
are optional for legacy projects. Portable FS files use a separate kind/version;
old applications reject them. Context changes warn without automatic mutation.
FS-bearing projects bypass cache until the key captures all new context inputs.
See [FS3 qualification](feeds-speeds/FS3_QUALIFICATION.md) for ownership,
secondary-pass snapshots, persistence, limits and compatibility details.

## FS3.5 ownership and UX boundary

Physical ToolEntry geometry is distinct from operation AppliedCutting and from
optional recommendation/provenance. Legacy tool defaults stay fallback values.
Manual-only slots need no fabricated recommendation; native step/depth_list/overlap
remain geometry authority. Portable cutting/context saves use guarded v3 while
v1/v2 remain readable. Material identity belongs to the job and is edited in
Stock; optional portable display labels do not enter native matching. Sources
are per-installation Settings, loaded without automatic calculation. Explicit
reference compatibility never implies machine equivalence. See
[ADR-0004](adr/ADR-0004-manual-operation-cutting-and-source-settings.md) and
[FS3.5 qualification](feeds-speeds/FS3_5_QUALIFICATION.md).

### Transient refusal explanations (FS3.6 review revision)

FsCamResponse may include scope_rejections for missing cutting data, derived from
the same pure native scope checks used for matching. Requested/required values
are display metadata; closest rejected scopes never become recommendations.
The field defaults empty for old responses and is omitted when empty. It is not
stored in AppliedCutting or project files. Math/precedence/data and post-time
consumption remain unchanged. The frontend formats reasons without matching data.


### Generic cutting source packs (FS3.7)

[ADR-0005](adr/ADR-0005-source-packs-and-cutting-knowledge.md) extends the pure
engine with DATA-only schema-1 packs and a deterministic knowledge resolver;
[contract](feeds-speeds/SOURCE_PACKS.md) defines taxonomy, quantities and ranking.
The CAM adapter accepts legacy datasets or enabled-pack collections. Numerical
calculations still call the same Engine; generation reads only saved rates.
Settings read an explicitly reviewed public registry and installation-local packs.
Tauri path APIs own native storage; browser storage is origin-local IndexedDB.
Pack JSON never uses the HTTP transport; native IPC/local WASM handle calculations.

AppliedCutting optionally carries a safe identity/evidence/policy summary. Private
Apply omits the source profile snapshot; authoritative slots and native geometry
remain persisted. Public snapshots remain compatible. Portable version 4 guards
new knowledge metadata, while versions 1/2/3 remain readable. Local pack absence
is informative and does not trigger recommendations during generation. This is
milling-specific infrastructure; no waterjet/knife coupling is introduced.

## FS3.8 local authoring boundary

[ADR-0006](adr/ADR-0006-local-preset-authoring-and-validation.md) extends SourcePack
schema 2 and existing app-data/IndexedDB storage, with read-only core capture,
immutable revisions/head comparison and retained authoring history. Draft is inactive;
Tested/Validated use existing resolver categories. Existing project identity fields
support explicit local UUID setup. No parallel database, math or post change.

## UX1 entry/post and generated-output boundary

Generic CAM remains authoritative. Section context carries transient entry strategy,
start Z, tab/lead/path-helix flags and numeric WCS metadata for consumer qualification;
no persisted Tool/CuttingParameters/Project schema change. Bundled ShopBot accepts
scoped varying-Z XY Ramp arcs and uses existing CPS XYZ linearization; unsupported
composition and numeric origin offsets reject before output. Generic geometry does
not learn ShopBot semantics. Runtime generation input identity prevents exporting
a program after setup edits, even after saving; the shared wire builder supplies
both generation and identity. See [UX1 qualification](ux/UX1_QUALIFICATION.md).

UX1 continues with [ADR-0007](adr/ADR-0007-shopbot-resolved-preview.md): the final
bundled ShopBot preview decodes emitted supported movements, so simulation uses
straight, rounded SBP XYZ chords and ordered jogs rather than analytic generic arcs.
Existing per-op diagnostics carry requested/resolved representation; no new schema.
Generic Helix fallback comments make geometric failure inspectable; ShopBot refuses
fallback before output, separately from unqualified true Helix. Other posts retain
existing preview paths. Stock/fixture workflows remain CAM2/UX2.

## CAM2 coordinate and obstacle boundary

[ADR-0008](adr/ADR-0008-work-origin-frames.md) defines single-sided milling XY
normalization on an owned request before generic CAM. Program/returned geometry
use work coordinates; desktop display maps resolved moves/regions back to drawing
coordinates using captured origin. No persistence migration, post text rewriting
or ShopBot-specific geometry semantics. Fixture candidates use resolved paths and
existing warning/export controls; missing holder geometry is explicit. Z and
unqualified two-sided output reject before generation.

## Future job-intent / machine-setup boundary

**DESIGN DIRECTION; implementation deferred.** FabCAM describes job requirements;
future FabSBP establishes physical machine state and executes. Declared work zero
is not physical zero establishment. Future XY origin and Z reference are distinct;
current qualified XY normalization and Z/two-sided guards remain unchanged.
Origin declaration alone must not cause zero-setting commands such as Z2/ZZ.

Reusable future profiles separate capabilities, workshop setup and execution
workflow. FabCAM shares the CAM-relevant subset; connection, home/probe/I/O and
execution procedures remain FabSBP responsibilities. Both applications remain
standalone; explicit lightweight requirements metadata may connect them after a
separate contract-design checkpoint. No format/schema or persistence change now.
See [ADR-0009](adr/ADR-0009-job-intent-machine-setup.md) and the
[future workflow plan](FABCAM_FABSBP_WORKFLOW.md) for ownership, current seam
dispositions, guided policy, structured pre/post actions, UX and dependencies.

## CAM3 holding-material emission

Generic tab attachment projects onto swept arcs rather than their chords. Generic
emission honors configured width and resolves individual tab tops against the
final floor before each pass. Direct contour entry and lead-out preserve holding
tabs too. Curved ramps use a continuous height field across bounded chords, not
independent repeated chord ramps. No post-specific syntax, new schema or FS calls.
ShopBot final preview still follows emitted XYZ. Conservative whole-arc lifts for
rectangular curved tabs remain explicit partial behavior. See
[CAM3 qualification and limits](CAM3_QUALIFICATION.md).

## CAM4 A/B depth-scoped diagnostics

[ADR-0010](adr/ADR-0010-depth-scoped-remaining-material.md) extends only buffered
pipeline output with optional per-operation remaining_material diagnostics. Reuse
Pocket intended regions, hole-preserving Clipper conversion and final returned
linear moves; depth clipping precedes flat-cutter sweep/difference. Native text
interpretation/bundled ShopBot resolved output is distinguished from arbitrary
CPS IR at the adapter boundary. No machine syntax in removal geometry.

The additive response/schema has no project/Tool/preset migration. Drawing-frame
adaptation translates residual rings while leaving area/depth/program intact.
Transient UI state and generation identity control fresh overlays; Save cannot
validate old results. Unsupported scope is explicit, not empty-area success.
[Qualification](CAM4_QUALIFICATION.md) records bounded metric Cascade scope,
intentional-material exclusions, numerical cutoffs and per-operation limits.

## FS3.9 derived reference layer

The pure pack resolver adds bounded normalized adaptation before the existing
Engine calculator, restoring original source profiles afterward. Optional public
derivation provenance and private-safe match quality extend FS metadata; no
Tool/CuttingParameters, geometry, post, application or save-value ownership changes.
See [ADR-0011](adr/ADR-0011-bounded-cutting-reference-adaptation.md) and
[policy](feeds-speeds/FS3_9_MATCHING.md). FS4 remains deferred/nonblocking.
