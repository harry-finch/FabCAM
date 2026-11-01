# FabCAM Architecture

FabCAM derives from [ivaCAM](https://github.com/aalarchiv/ivacam); inherited `ivac-*`
names and upstream attribution remain. [STATUS](shopbot-modernization/STATUS.md)
owns the current source/qualification identity. [CUTTING_CONTRACT](CUTTING_CONTRACT.md)
owns V10 and staged milling; [fork architecture](shopbot-modernization/ARCHITECTURE.md)
owns process/security boundaries; [HISTORY](releases/HISTORY.md) owns chronology.

## V1.1 operation and safety ownership

ProjectFile 10 and native cutting version 10 are current. Profile/ordinary Pocket
use explicit Main Cutting and optional Finishing: `main_finishing_v1`, or scoped
Pocket `pocket_finishing_v2`. The latter supports Walls / Floor / Walls and floor
with bounded polygon/curve/island proof; this is not rectangle-only or universal
finishing. Other families retain their actual role sets. Readers refuse old project
versions, malformed nested contracts and retired calculation fields before mutation.

`cutting_parameters`, `cutting_stages` and the setup resolver validate authored
values, actual tools and known limits. Geometry drivers, numeric budgets, posts,
provenance and full-program simulation are independent safety owners. Tool defaults
seed creation once; no runtime calculator or silent rate fallback participates.
Material identities are exact id/revision pairs. TOOL-META substrate/coating is
optional descriptive metadata, not rate calculation or compatibility evidence.

Collection V2 and strict tested matching are distinct from [ALPHA-4 reviewed
starting-value reuse](TESTED_PRESETS.md#reviewed-starting-value-reuse--alpha-4).
Copied advisory provenance never transfers physical testing. No stepover reuse.
FabSpeed owns independent calculations/experiments; optional V1.2 exchange remains
proposed. The removed integrated recommendation crate/source-pack APIs are historical.

`ivac-file-export` stages and publishes individual files. Tauri raw plugin access
has no ambient home/temp scope; native dialogs grant individual paths. Custom
source/export/watch authorization is separate and canonical-target checked. HTTP
is loopback-only with explicit Origin admission, two heavy-job slots and cooperative
cancellation. Parser expansion is bounded. See the [security boundary](shopbot-modernization/ARCHITECTURE.md#security-and-untrusted-data).
Per-file transactional publication is not pair-atomic; software/mock/CSP evidence
is not installed-native, authenticated remote or physical acceptance.

### Actual Cargo workspace

| Member | Responsibility |
| --- | --- |
| `ivac-core` | Source models/readers, CAM, validation, simulation, output semantics and preset contracts |
| `ivac-cps` | Budgeted CPS runtime/recorder; no geometry ownership or external calculation |
| `ivac-file-export` | Validated transactional individual-file publication |
| `ivac-cli` | Headless explicit-request commands over core |
| `ivac-server` | Loopback HTTP adapters/admission |
| `ivac-tauri` | Native IPC, dialogs/scopes, persistence and watchers |
| `ivac-wasm` | Browser bindings/worker-facing core adapter; default build has no CPS |
| `xtask` | Schema/version developer checks |

The frontend is Svelte/TypeScript/Three.js, with EN/DE catalogs. Generated OpenAPI
and TypeScript derive from Rust types; do not hand-edit them. Existing drawing
storage/GEOM1 generation is V1; scoped geometry Analyze/Preview/Apply is V2, independent
of the project/cutting versions. Local edit acceptance does not certify machining.

## Big picture

FabCAM turns a 2-D drawing (DXF or SVG) into G-code for a
CNC mill, router, plasma, or laser. The same CAM engine runs in three
deployments: a desktop app (Tauri), a self-hosted server (axum HTTP),
and a browser tab (WASM). All three share one Rust core and one Svelte
UI; the only difference is the **transport** wiring them together.

```
┌────────── frontend/ (Svelte 5 + Vite + TypeScript) ──────────┐
│  components/  UI                                             │
│  state/       reactive state (slices + command-bus undo)     │
│  api/         transport-agnostic client interface            │
│       ├── http.ts     ─→ ivac-server (axum)                  │
│       ├── tauri.ts    ─→ ivac-tauri (native invoke)          │
│       └── wasm.ts     ─→ ivac-wasm (in-page WASM)            │
└──────────────────────────────────────────────────────────────┘
                            ▲ ▼  JSON request / response
┌──────────── transports ─────────────────────────────────────┐
│  ivac-cli      headless converter (no transport, direct lib) │
│  ivac-server   axum HTTP wrapper                             │
│  ivac-tauri    Tauri 2 desktop shell                         │
│  ivac-wasm     wasm-bindgen browser bindings                 │
└──────────────────────────────────────────────────────────────┘
                            ▲ ▼  Rust function calls
┌────────── ivac-core (the only thing with CAM logic) ─────────┐
│  input/       DXF / SVG / text → Vec<Segment>                │
│  cam/         geometry, offsets, V-carve, halfpipe, …        │
│  pipeline/    orchestrator: project → ordered toolpath ops   │
│  gcode/       polyline / drill / V-carve → dialect-specific  │
│  sim/         heightfield voxel simulation                   │
│  project.rs   wire types (Project, Op, ToolEntry, …)         │
│  schema.rs    JsonSchema → openapi.yaml (single source)      │
└──────────────────────────────────────────────────────────────┘
                            ▲ ▼
                    schema/openapi.yaml
                    (re-codegen'd into frontend/src/lib/api/generated.ts)
```

**Ownership:** geometry and machining semantics live in `ivac-core`; budgeted CPS
execution lives in `ivac-cps`, individual-file publication in `ivac-file-export`.
Transports own their authorization/admission and wire adaptation. The UI owns
authoring/history, presentation and evidence review, not an alternate CAM planner.

## Data flow: one user click

This is the single most important picture. Memorize it.

```
1. User clicks "Generate" in the UI
       │
       ▼  components/GenerateBar.svelte calls
2. state/project.svelte.ts.generate()
       │  (mutates `gen.generating = true`, kicks off async)
       ▼  passes the current Project to
3. api/build-project.ts (assembles the wire request)
       │
       ▼  via the WiacClient interface
4. api/{http,tauri,wasm}.ts (whichever transport is active)
       │  ──── JSON across process boundary ────
       ▼
5. ivac-core::pipeline::run_pipeline()
       │  (resolves ops, builds offsets, dispatches per kind,
       │   emits G-code via the chosen PostProcessor)
       ▼  PipelineResponse (gcode + warnings + stats + toolpath)
6. transport sends it back
       │
       ▼  resolved at the awaiting promise in
7. state/project.svelte.ts updates `gen.generated`, `gen.toolpathCumLen`, …
       │
       ▼  Svelte 5's reactivity scheduler re-renders
8. components/GenerateBar / Scene3D / EntityCanvas2D paint the new state
```

The arrows go **one way per phase**. UI mutates state; state calls the
client; the client speaks the wire contract; the core does the math.
Never short-circuit (UI calling `ivac-core` types directly, transport
shipping a raw Svelte proxy, the core reaching back into the UI).

## Layers in detail

### `ivac-core` (the math)

Pure Rust, no UI, no transport. **If you can write your change here, do.**

```
ivac-core/src/
  cam/              geometry primitives, offsets, V-carve, halfpipe, threads
  gcode/            per-dialect emitters (linuxcnc, grbl, hpgl)
  gcode.rs          PostProcessor trait + per-block-kind emit shells
  geometry.rs       Point2, Segment, BBox — shared with the schema
  input/            DXF, SVG, text-to-segments parsers
  pipeline/         orchestrator submodules (offset_builder, op_drivers, …)
  pipeline.rs       run_pipeline() entry point + integration tests
  pipeline_cache.rs per-op result cache (keyed by hashed inputs)
  project.rs        wire types: Project, Op, ToolEntry, MachineConfig
  sim/              destructive material sim — multi-span Z-dexel
                    (undercuts, two-sided); see docs/material-model.md
  schema.rs         emits openapi.yaml from JsonSchema derives
```

**The pipeline orchestrates; it doesn't compute.** `run_per_op` owns the
per-op envelope that every kind shares — boundary tool-change (M6) state,
the result cache (lookup / replay / store), pause handling, progress and
event emission — and delegates the actual geometry to a driver. The
kind→driver switch is a single `match` in the `run_op_driver` helper; each
driver is a single file in `pipeline/op_drivers/` (`drill.rs`, `thread.rs`,
`vcarve.rs`, etc.) that owns its emission strategy and its own tests.
**Add an op kind by adding a driver and one `run_op_driver` arm — not by
growing `run_per_op`'s shared envelope.**

### Transports (`ivac-cli` / `-server` / `-tauri` / `-wasm`)

Each is a thin adapter over `ivac-core`. They share `pipeline::PipelineRequest` / `PipelineResponse`, using the common
project/tool/machine models, over a
different channel:

- **ivac-cli** — no channel; calls `run_pipeline` directly.
- **ivac-server** — axum HTTP, JSON request body, SSE for streaming.
- **ivac-tauri** — Tauri 2 `invoke`, native dialogs, window state, file
  watching. Commands live in `crates/ivac-tauri/src/commands.rs` and
  must keep the same JSON shape as the HTTP endpoints.
- **ivac-wasm** — wasm-bindgen functions exported under
  `crates/ivac-wasm/src/lib.rs`; the browser calls them like local fns.

**Keep CAM rules in core.** Native filesystem grants, HTTP Origin/admission and
transport cancellation are adapter-specific defensive controls; do not move them
into geometry or bypass them through a second transport path.

### `frontend/`

Svelte 5 with runes (`$state`, `$derived`, `$effect`). TypeScript
strict. Vite for dev/build.

```
frontend/src/lib/
  api/        transport-agnostic WiacClient + generated wire types
  cam/        front-end-side CAM helpers (preview math, no canvas)
  canvas/     pure 2D rendering helpers (spatial-index, fixture-hit, …)
  components/ Svelte UI (one component per .svelte file)
  scene3d/    Three.js setup, lifecycle, voxel mesh
  sim/        front-end side toolpath simulation (animation runner)
  state/      reactive state — the interesting bit; see below
```

### `frontend/src/lib/state/` (the slice layer)

`ProjectState` (in `project.svelte.ts`) is the root of frontend state.
It's split into focused **slices**, each a `*.svelte.ts` class that
`ProjectState` composes and exposes directly (`project.data.…`,
`project.gen.…`, `project.sel.…`):

- `generated.svelte.ts` — pipeline output (gcode, toolpath, sim diagnostics, version, progress)
- `selection.svelte.ts` — selectedObjects / hover / selectedOpId / …
- `project-data.svelte.ts` — imported geometry + ops + tools + machine (extracted from the root slice in `n5v5`)
- `workspace.svelte.ts` — view / camera / panel layout state
- `text_preview.svelte.ts` — live text-entity preview
- `warning-focus.svelte.ts` — which pipeline warning is focused
- `confirm.svelte.ts` — the shared styled-confirm prompt store
- `project.svelte.ts` — the root composer: slice wiring, the command
  bus (`history` + `target()`), cross-slice deriveds
  (`transformedImport`, `geometryView`, `hasUnsavedWork`), and the
  command-wrapping mutation methods
- `import-ops.ts` / `project-file-ops.ts` — the import-lifecycle and
  save/load/workspace domains, as functions over the live `ProjectState`

Call sites address slices directly: `project.gen.generated`,
`project.sel.selectedOpId`, `project.data.operations` (the old root
proxy accessors were dropped in `361x`). The command-target fields on
`project.data` are read-only from outside the state layer — undoable
edits must go through the command-wrapping methods (`mukh`). **When you
add a new field, decide which slice owns it; don't pile it onto
`ProjectState` directly.**

### `schema/` (the contract seam)

`schema/openapi.yaml` is **regenerated** from the `#[derive(JsonSchema)]`
types in `ivac-core`. `frontend/src/lib/api/generated.ts` is regenerated
from the YAML. Both files are checked in so downstream builds don't
need the toolchain.

After touching any `pub` JsonSchema-deriving type in `ivac-core`:

```bash
cargo xtask schema && (cd frontend && pnpm run codegen)
# or: just regen-schema
```

The pre-commit hooks `xtask-schema-check` and `frontend-codegen-check`
refuse drift. **Don't hand-edit `generated.ts`.**

## Key patterns

These show up over and over. Know the names; reach for them when the
shape matches.

### 1. State slice (frontend)

Multiple `$state` fields that move together → extract a class in its
own `*.svelte.ts` file. Methods become the only way to mutate. Old
call sites keep working because `ProjectState` exposes proxy
getters/setters that forward to the slice. See `selection.svelte.ts`
for the cleanest example.

### 2. Op-driver dispatch (Rust)

One file per op kind under `pipeline/op_drivers/`. Each exports a single
`run_xxx_op` fn with `pub(in crate::pipeline)` visibility; `op_drivers.rs`
re-exports them. The `run_op_driver` helper in `pipeline.rs` is a single
`match` on `op.kind` — adding a kind means adding a driver file and one
arm. The surrounding `run_per_op` loop is the shared orchestrator (tool
changes, cache, pause); resist the urge to add kind-specific logic there.

### 3. Post-processor trait (Rust)

`PostProcessor` in `gcode.rs` is the seam. Per-dialect impls live in
`gcode/linuxcnc.rs`, `gcode/grbl.rs`, `gcode/hpgl.rs`. The per-block
emit shells (`emit_polylines`, `emit_drill`, `emit_vcarve_block`) accept
`&mut impl PostProcessor` and call methods on it — never branch on
the dialect inline.

### 4. Command-bus undo (frontend)

`state/commands.ts` is the **only** path to mutate undoable state.
A command captures its inverse on apply; undo replays the inverse.
Ephemeral state (hover, drag-in-progress, modal-open) is **not**
undoable and skips the bus. Decide which category your mutation
belongs to before writing it.

### 5. Schema seam (Rust ↔ TypeScript)

Type that crosses the wire? Derive `JsonSchema` in `ivac-core::project`
(or `ivac-core::lib.rs` for top-level request/response). Regen, commit
both files, done. **Don't** define a TypeScript wire type by hand —
Rust types are the source of truth; generated schema/API artifacts must remain
in sync, and an independent manual wire type will drift.

### 6. Test co-location

Tests live next to the code they exercise. Pipeline op-driver tests are
in the driver's own `#[cfg(test)] mod tests` block; shared fixtures
live in `pipeline/test_helpers.rs` with `pub(in crate::pipeline)`
visibility. **Don't** put tests for `gcode/grbl.rs` in `pipeline.rs`.

### 7. Reactive collections — build fresh, assign

Plain `Map` and `Set` are fine for `$state<Set>(…)` / `$state<Map>(…)`
fields **as long as you replace, never mutate in place**:

```ts
// ✓ reactive — assigning the $state field triggers $effects
const next = new Set(this.selected);
next.add(id);
this.selected = next;

// ✗ NOT reactive with plain Set — silently no-op
this.selected.add(id);
```

The selection slice, layer-visibility, fixture map, and every other
reactive-collection field in the frontend follows this convention. If
you need in-place mutation (e.g. a hot path), reach for
`SvelteSet`/`SvelteMap` from `svelte/reactivity` instead — but the
build-fresh pattern is the default and the
`svelte/prefer-svelte-reactivity` ESLint rule is intentionally off for
that reason (tvjy review found all 39 baseline sites were already
correct under this convention).

## Anti-patterns (don't do these)

Each of these caused a real bug. Don't reintroduce them.

- **Self-scheduling `$effect`** (caught by `dh1n-regression.test.ts`):
  inside an `$effect` body, writing to a `$state` field and then
  reading the same field via a function call (incl. `JSON.stringify`,
  `snapshotKey()`) creates a dependency cycle that kills the Svelte
  scheduler. Build pristine snapshots from **local** vars before
  assigning `$state`. See `MachineDialog.svelte`'s `$effect` for the
  fix shape.
- **`window.confirm` in Tauri** (issue C10): WebKitGTK's native
  `confirm` blocks the renderer and never returns. Use the shared
  styled prompt — `confirmStore.ask({…})` rendered by
  `ConfirmPrompt.svelte` (see `confirmDiscardIfDirty` in
  `lib/services/file_ops.ts` for the discard-before-load shape).
- **`JSON.parse(JSON.stringify(proxy))` for clone** of Svelte 5
  `$state` objects: the proxy survives in the result and writes leak
  back. Use `structuredClone` instead.
- **Hand-editing `frontend/src/lib/api/generated.ts`**: it gets
  overwritten on every codegen. Modify the Rust type, regen.
- **Branching on post-processor dialect inline** in an emit shell:
  defeats the trait abstraction. Push the difference into a new
  trait method instead.
- **Adding op-kind logic to `run_per_op`**: the dispatcher should
  only dispatch. Put the logic in a new file under `pipeline/op_drivers/`.

## "How do I add X?"

CONTRIBUTING.md walks the two big ones (new op kind, new
post-processor) end-to-end. The smaller recipes:

### A new state slice

1. New file `frontend/src/lib/state/<name>.svelte.ts` exporting a
   class with `$state` fields + mutating methods. Mirror
   `selection.svelte.ts`.
2. In `project.svelte.ts`: instantiate `pub readonly <name> = new
   <Name>State()` and add proxy getter/setter for every field that
   should be accessible as `project.<field>`.
3. Update commands.ts if the new fields are undoable.
4. No test for the slice file directly — vitest can't load
   `*.svelte.ts` modules (the Svelte plugin is off). Cover lifecycle
   via the integration tests in `commands.test.ts` etc.

### A new dialog

1. Component under `frontend/src/lib/components/<Name>Dialog.svelte`.
   Mirror `MachineDialog.svelte`'s `$effect` shape (pristine from
   local vars **before** writing `$state`).
2. Inline two-step "discard changes?" confirm — never `window.confirm`.
3. Triggered from `App.svelte`'s menu; the pure menu/shortcut decision
   logic lives in `lib/state/app-menu.ts`.
4. Route display text through the current typed i18n catalog. Frontend locales
   are English/German; preserve keys/placeholders and do not hardcode UI strings.
   The removed third-party i18n package is not the current catalog boundary.

### A new wire type

1. Define the struct in `ivac-core` with `#[derive(JsonSchema,
   Serialize, Deserialize)]`. Decide whether it's a `pub` lib export
   (top of `lib.rs`) or lives in `project.rs`.
2. `cargo xtask schema && (cd frontend && pnpm run codegen)`.
3. Use the codegen'd TS type in the frontend; don't hand-roll.

## Where the audit issues live

The architecture above describes the **current** state, not an aspiration:
`project.rs` is split into the `project/` module, the `ProjectState` god
class is decomposed into slices, and `gcode.rs` is the `PostProcessor`
trait plus per-block emit shells (large, but factored — not a god
function). Where the code still has known rough edges, they're tracked in
bd rather than left as silent debt:

```bash
bd ready                # see what's available
bd list --status=in_progress   # current refactor wave
```

The biggest files by line count (`offset_builder.rs`, `cam/offsets.rs`,
`gcode.rs`) are mostly co-located tests — check the production half (the
lines above the first `#[cfg(test)]`) before assuming a file is bloated.

When you finish one, follow the patterns above. Don't invent a new one
unless you can justify why the existing one didn't fit.
