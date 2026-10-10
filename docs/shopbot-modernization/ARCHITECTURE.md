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
