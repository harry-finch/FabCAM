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

## UX5 future application-command access layer

**PLANNED — NOT IMPLEMENTED.** [UX5 plan and preliminary inventory](ux/UX5_PLAN.md)
proposes a thin CommandRegistry for local command/tool search and eventual shared
menu/shortcut dispatch. Current `app-menu.ts` resolves shortcuts while App owns
shell effects; `commands.ts`/history owns undoable mutations. These are distinct
responsibilities, not an existing unified application registry. Audit component-local
actions before freezing descriptor/context/execution contracts or moving handlers.

Registry availability reads authoritative document/selection/tool/machine/generated
state; execution rechecks existing handlers/guards. No independent palette-ready
state, CAM/FS logic or command execution state machine. Search reads workspace shop
inventory plus relevant stocked/project tools and preserves existing identity,
compatibility and explicit assignment. Derived search indexes remain read-only and
transient. UX4 template handlers are a future dependency for template commands only.
No project serialization, tool storage, post or process boundary changes now;
no ADR selecting a concrete registry schema is warranted at this planning checkpoint.

## UX3 — planar surfacing boundary

`PlanarSurface` owns its drawing-frame rectangle and explicit raster parameters;
copying stock snapshots dimensions rather than binding to future stock edits.
`cam/planar_surface.rs` plans XY only. Its specialty driver validates resolved
milling/tool/zone/depth/travel before output/cache replay and uses common XYZ
emission with saved manual rates. Work-origin normalization translates the zone;
final-program movement supplies preview/simulation/fixture checking. ShopBot
receives a distinct strategy, without broadening 3D or controller semantics.

Frontend authored sizing does not introduce source chains. V6 guards persistence
of the new operation and can retain CAD1 modifiers/cutting provenance. Scoped
stock XY and surfacing-only bbox diagnostics distinguish intended overtravel while
retaining Z/collision/mixed-contour guards. [ADR-0014](adr/ADR-0014-explicit-planar-surfacing.md)
and [qualification](UX3_QUALIFICATION.md) define the bounded V1 and evidence limits.

## UX4 local operation templates

`state/operation-templates.ts` defines strict version-1 portable operation recipes.
The existing workspace owns the local library; explicit destination binding creates
an ordinary independent OpEntry via command history. Manual role snapshots preserve
captured values without evidence promotion or dynamic FS evaluation. Source IDs,
zone bounds and source-bound placements are excluded. Per-store writes serialize
and transport errors propagate. Job files contain only instantiated operations,
using existing guarded formats. [ADR-0015](adr/ADR-0015-reusable-operation-templates.md).

## UX4.1 presentation and sidebar context

Template search/parameter rows and validation feedback are pure presentation
helpers over the unchanged UX4 model. Secondary save/rename/confirmed-delete
views retain workspace persistence; create uses the existing command and reveals
the inspector. The dialog is outside the clickable header. Main sidebar active
hosts own scrollports; intermediate wrappers remain visible in this layout.
Shared section-bounded sticky headers and inline title/count identity preserve
context, using existing design tokens. Counts derive directly from project data.
[Audit and evidence](UX4_1_QUALIFICATION.md); no new core/model/post boundary.

## UX5 proposal — owner-bound palette dispatch

The live action audit found component-owned export policies not fully represented
by the raw file service, and tool identities/rate slots that require origin/role
context. Proposed localized search descriptors bind to existing authoritative
owners with execution-time target revalidation; mutation history and stores remain
unchanged. No palette source code exists at this checkpoint.
[Audit](ux/UX5_AUDIT.md), [Proposed ADR-0016](adr/ADR-0016-command-palette-owner-dispatch.md).

## UX5 palette ownership (accepted ADR-0016)

The frontend palette is a transient search and dispatch surface. Pure descriptors,
prepared whitelisted metadata, numeric/ranking, keyboard and export-policy helpers
are separate from disposable shell/GenerateBar/OperationTemplates owner bindings.
History and existing file services remain authoritative. Execution rechecks captured
owner/context/result identity; tool Inspect is read-only and Assign reuses existing
operation updates after origin, stocked-definition, kind/mode and saved-role checks.
Template results only preselect UX4.1's binder. GenerateBar retains generation,
cancellation and complete export guards; file_ops retains late native-picker and
fixture/freshness checks. No CAM/FS/post/schema/store migration. See
[ADR-0016](adr/ADR-0016-command-palette-owner-dispatch.md) and
[software evidence/limits](UX5_QUALIFICATION.md).

## Future two-sided registration and V-Carve inlay boundaries

**PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** for the new complete capabilities.
[Two-sided inventory and plan](TWO_SIDED_PLAN.md) distinguishes inherited stock flip,
Front/Back fields, reflection helpers, native wrapper and approximate conflict checks
from qualified ShopBot output. Open ivac-jtkj covers CPS/translated registration;
existing rejection remains. Future drawing/stock/A/B/machine/registration frames need
explicit transform composition, side Z/clearance ownership and independently qualified
pin preparation. Keep generic geometry separate from machine syntax/physical setup.

[Inlay inventory and plan](VCARVE_INLAYS_PLAN.md) reuses V-Carve only within its
qualified scope. Inherited source inset and duplicate-as-plug do not establish
female/male mating geometry, assembly clearance or physical fit. Pair depth/angle/tip,
start depth, engagement/glue/fit and topology require separate proof and output identity.
No second V-Carve engine or concrete paired-project schema selected in this plan.

Both retain capability resolution, history/save compatibility, fixture and stale-output
protections and final output/preview parity. FabCAM declares setup intent; operator
or future FabSBP establishes physical references. Standalone instructions do not depend
on FabSBP metadata. Any future model/contract change needs architecture approval;
no new ADR selecting those unfrozen decisions or serialization change now. Active
V-Carve qualification proceeds within its own scope and human gate.

## V-Carve planning boundary

[ADR-0017](adr/ADR-0017-vcarve-continuous-planner.md): source-region/medial geometry → continuous V-Carve target validation → V-Carve-only pass scheduler → shared XYZ emitter → CPS strategy `vcarve` → existing resolved preview. Halfpipe retains its legacy emitter. ShopBot changes authorize only the qualified strategy/tool pair, never repair geometry. No serialized project migration.

## Planned geometry preparation and editing (GEOM1/2/3)

**PLANNED — NOT IMPLEMENTED.** [Shared source-backed plan](GEOMETRY_PREPARATION_PLAN.md)
separates external sources, editable drawing entities/layers, CAM selections/regions,
generated paths and process-specific output. Current SVG normalized/transformed paths
and DXF layers/entities/units differ; do not promise lossless source editing. Import
normalization may already discard tiny geometry; later diagnostics cannot recover it.

Reuse Segment/import metadata, per-import transforms, common transformedImport,
CAD1's preserved-source/accepted-result precedent and history/generation identity.
CAM source_combine booleans are not persistent shape merging. Multi-import object-ID
namespacing and source/relief/tab/approach bindings require explicit stable identity
or remapping on topology changes; no silent operation rebinding. GEOM1/2 must share
one authoritative effective geometry for ordinary consumers, with transient proposals
and explicit atomic Apply/history. No concrete new schema, editable model or ADR frozen
now. Purpose-specific mm tolerances stay separate from CAM/physical tolerances.

Geometry diagnostics and operation-specific incompatibility differ: points/open
paths may be valid. Generic infrastructure serves future WJ without kerf/pierce or
milling-engine coupling. [GEOM1](GEOM1_PLAN.md), [GEOM2](GEOM2_PLAN.md) and
[GEOM3](GEOM3_PLAN.md) define audit-first scope, exclusions, tests and acceptance.
Active V-Carve work and current project/post semantics stay intact.

## Future grouped transforms and sheet instances (GEOM2-A/B, NEST1/2)

**PLANNED — NOT IMPLEMENTED.** [Shared architecture inventory](NESTING_PLAN.md)
refines GEOM2 into selected-geometry transformations and basic authoring. Current
per-import TS transforms and Rust helpers are partial reuse; audit their parity
instead of extending two independent implementations. Nonuniform circular-curve
scale needs qualified exact representation, bounded conversion or explicit refusal.

Logical source parts and placed instances must have distinct ownership and stable
identity. Source → part/entity → instance transform → accepted positioned geometry
→ normal CAM source resolution, compensation/WCS/capability/output. Do not translate
finished code. Clarify sheet/current-stock linkage, operation-to-instance binding and
source/instance edit invalidation; no duplicate stock or hidden coordinate conversion.
No concrete schema/matrix/baking contract selected before GEOM2-A/NEST1 architecture
review. NEST2 starts with algorithm/license/performance audit after NEST1; no optimizer
library adopted here. Placement constraints and process/path clearance remain separate.
Common-line cutting changes topology/toolpaths and needs a separate future milestone.

## Planned PERF1 measurement and optimization boundary

[PERF1 plan](PERF1_PLAN.md) consolidates historical core DXF/Criterion and CPS/GRBL
throughput harnesses with scoped V-Carve evidence; none currently proves reported
small-text end-to-end latency. Audit actual stage ownership and build/transport
capability before instrumentation; separate CPU/wall, inclusive/exclusive spans,
serialization/IPC, post/decoder, preview preparation/rendering and user interaction.
Equivalent native/WASM comparisons must disclose unsupported CPS paths, startup and
cache conditions. Existing progress/cooperative cancel and worker termination need
latency/resource/state coverage, not a second generation owner.

PERF1-A is PLANNED report-first; PERF1-B awaits its human scope gate. Indexes may
conservatively prune candidates but cannot replace continuous clearance predicates.
Cache identity/version/tolerance and concurrent correctness, deterministic order,
source mapping, current-input export checks and final ShopBot parity are retained.
Numerical approximation changes need independent bounds/comparison and approval.
No new cache/index/threading ADR, dependency, schema or architecture rewrite chosen.
Open ivac-htc8 interference must be recorded rather than hidden by benchmarking.
Generic measurement conventions may serve GEOM/NEST/WJ without coupling their
functional milestones to a V-Carve algorithm or changing FS semantics.

## UX6 planned machine/tool management presentation boundary

[UX6 plan](ux/UX6_PLAN.md) records preliminary source inventory, not completed Phase 0.
Workspace profiles are named machine+tool bundles; project owns embedded machine/tools
and optional profile ID. Applying copies payload/reference through existing history;
project edits mirror to referenced workspace profile. Current MachineDialog picker
immediately applies, rather than independently browsing. AppNavigation reads project
machine.name; generated profile display names may differ. Audit exact naming/draft/
assignment transitions before treating a mismatch as a bug or changing behavior.

Workspace shop tool inventory and same-ID stocked/project copies are distinct. Tool
Apply may sync stocked copies and mirrored profile; legacy/project-local tools and
operation tool IDs need explicit dependency handling. Workspace editing and project
Undo are not one assumed global transaction. KEEP existing formats/snapshots/mirror,
FS applied-value/fallback precedence, capability resolution and generation preflight.
EXTEND presentation after field-consumer/action matrix and UX approval; no new schema,
association model, autosave policy or domain ADR selected. Future browser/inspector
patterns must not create parallel authoritative machine/tool state.

[UX6-A](ux/UX6_A_PLAN.md) and [UX6-B](ux/UX6_B_PLAN.md) each need separate tests,
actual screenshots and human gates. FabCAM program-level output settings stay distinct
from future FabSBP live control; draft/software/physical evidence labels are not invented.
No active PERF/CAM algorithm/post behavior changes in this planning checkpoint.

## GEOM1 — authoritative edited drawings (ADR-0021)

`ivac-core::drawing` owns source-preserving local snapshots, stable import/entity
identity, provenance/lineage, bounded repair proposals and evaluated membership.
The frontend adapts that evaluated geometry for drawing/picking; it does not perform
repair math. Dedicated revision-1 transports carry edits to the same generic CAM
pipeline. Stable operation sources are resolved inside Rust, with explicit refusal
of ambiguous/deleted bindings and unsupported synthetic/modifier combinations.
The source-to-work adapter and all machine post boundaries remain downstream.
Guarded project v7 prevents an older reader from machining the unedited original.
See [contract](GEOM1_CONTRACT.md), [serialization](GEOM1_SERIALIZATION.md) and
[software evidence and limits](GEOM1_RESULTS.md). No general parametric model,
GEOM2 editing, new toolpath strategy or additional ShopBot capability was introduced.

## Planned GEOM2-A1 imported-file dimensions

[GEOM2-A1](GEOM2_A1_PLAN.md) is a small planned view/command extension over existing
FileTransform.scale, not independent persisted dimensions. Whole-import effective
local bounds after GEOM1 edits supply W0/H0; displayed dimensions are scaled before
rotation. Selection bbox and combined world/project bounds cannot serve that role.
Audit authoritative per-import local bounds and curves/extrema; current legacy TS
endpoint helpers and edited Rust bounds need explicit agreement, not inverse-AABB math.

ADR-0021 original pivot and local-edit→file-transform→applicable-relief order remain.
Patch only Scale; keep translation/angle/mirrors, work zero and stock semantics. A repaired
bbox is measurement, not a new pivot. Preserve lineage/bindings/async guards, command
history/serialization and CAM preflight; no new format, geometric evaluator or FS
recalculation. File, future selected-group and nesting-instance transforms share math
but distinct ownership. Existing status dimensions are read-only measurement; A1 is
editable whole-file sizing. Degenerate/invalid input and early-delivery scope need audit.

## ARCH-MACH planned machine/process integration boundary

[ARCH-MACH source inventory](../releases/ARCH_MACH_PLAN.md) identifies reusable geometry/
drawing/identity, modes/operation drivers, profile snapshots, native PostProcessor and
feature-gated CPS IR, actual ShopBot output decoder, preview/transports and tests.
Existing planning/emission interaction, coarse modes, milling/CPS assumptions and backend
feature differences mean this is not already a universal process/plugin architecture.

Contract layers: shared geometry, process semantics, concrete machine capabilities,
output adapter, actual-output validation and qualification package. Capability fields
never authorize an adapter-unsupported feature. Same-process additions normally reuse
geometry; new process may require explicit operations/UI. Fail closed, preserve source/
project compatibility and existing machine support; no chosen new Rust interfaces.

[Guide](../development/ADDING_A_MACHINE.md) and [template](../development/MACHINE_INTEGRATION_TEMPLATE.md)
standardize lawful official-file evidence, independent/differential negative tests,
manufacturer software and physical gates. ProtoMAX native OMX validates them in V1.5,
not during planning. Proposed machine-integration skill links authoritative docs only.
No plugin discovery, universal IR replacement or speculative WAZER/knife implementation.

### Milling publication guard (ADR-0023, pipeline61)

After assembling the supported execution path, the common pipeline checks declared
fixture envelopes before publishing a successful response. A collision returns a
structured error with `diagnostic_preview` (path/warnings, no executable text).
Transports preserve that failure; the frontend displays it with export blocked.
Unknown body dimensions are a separate diagnostic, not evidence of clearance.
Write-through executable streams with fixtures are refused before bytes/status;
progress streaming still uses asynchronous buffered validation. Arbitrary CPS IR
intent is not substituted for a decoded execution path. See
[ADR-0023](adr/ADR-0023-milling-program-publication-guard.md).
