# Upstream feature audit

Audited code: `9168104b4fc4e02e916740a54bd1e56b5019f662`. Paths below are relative
to repository root. **CONFIRMED** means a wired implementation exists in the
inspected source; it does not mean the Rust tests or machining were validated
here. **PARTIAL** denotes a material capability limitation, **ABSENT** no
implementation found, **UNCLEAR** insufficient evidence.

| Capability | Classification | Evidence |
| --- | --- | --- |
| SVG | CONFIRMED | `crates/ivac-core/src/input.rs`, `input/svg_in.rs` dispatch/parser |
| DXF | CONFIRMED | `input.rs`, `input/dxf_in.rs`, `input/nurbs.rs` |
| Profile | CONFIRMED | `project/op.rs` Profile + ToolOffset; `pipeline/offset_builder.rs`, `gcode.rs` |
| Pocket | CONFIRMED | `project/op.rs`, `cam/offsets/pocket_fill.rs`, `pipeline/op_drivers/dual_tool.rs` |
| Drill | CONFIRMED | `pipeline/op_drivers/drill.rs`; `gcode.rs` simple/peck/chip-break hooks |
| V-carve | CONFIRMED | `cam/vcarve.rs`, `cam/vcarve_emit.rs`, `pipeline/op_drivers/vcarve.rs` |
| Engrave | CONFIRMED | `project/op.rs`, `pipeline/op_drivers.rs` contour dispatch |
| Tabs | CONFIRMED | `cam/tabs.rs`, `pipeline/tabs.rs`, `gcode/tabs.rs`, `project/params.rs` |
| Lead-in/out | CONFIRMED | `gcode/leads.rs`, `gcode/emit.rs`, `project/params.rs` |
| Ramping | CONFIRMED | `gcode/entry.rs`, `gcode/z_schedule.rs`, plunge strategy params |
| Tool library | CONFIRMED | `project/tool.rs`, `frontend/src/lib/services/file_ops.ts` toolset export/import |
| Machine profiles | CONFIRMED | `project/machine.rs`, `frontend/src/lib/state/machine_profiles.ts`, MachineDialog |
| Fixtures | CONFIRMED | `project/fixture.rs`, `sim/fixture_check.rs`, Project.fixtures |
| Holder collision | CONFIRMED | `sim/holder.rs`, `sim/holder_check.rs`, ToolHolderEditor |
| Material-removal simulation | CONFIRMED | `sim/dexel.rs`, `heightmap.rs`, `sweep.rs`, `ivac-wasm/src/sim.rs` |
| CPS | CONFIRMED | `gcode/cps_recorder.rs`, `ivac-cps/src/{ir,engine,run,library}.rs`; feature-gated |
| Project save/load | CONFIRMED | Project Serde types; `frontend/src/lib/services/file_ops.ts`, `state/project-file-ops.ts` |
| Undo/redo | CONFIRMED | `frontend/src/lib/state/{commands,history}.ts` and passing logic tests |
| Laser | CONFIRMED | MachineMode::Laser; `gcode.rs` arm/pierce/off; tool power/pierce params |
| Plasma | CONFIRMED | MachineMode::Plasma; `gcode/emit.rs` pierce height/delay/cut height |
| Drag knife | CONFIRMED | MachineMode::Drag; OpKind::DragKnife; `gcode/walk.rs` trail/swivel compensation |
| ShopBot OpenSBP | ABSENT | Native registry is LinuxCNC/GRBL/HPGL; bundled CPS registry contains grbl only |
| ProtoMAX/OMX | ABSENT | No waterjet MachineMode or corresponding post/quality model found |

## Dispositions

| Existing subsystem | Decision | Reason / future delta |
| --- | --- | --- |
| Import + segment topology | KEEP | Existing shared geometry; qualify in M1 |
| Milling CAM / depth / entries | KEEP | Existing implementation and substantial tests; no rewrite justified by M0 |
| Pipeline / caches / simulation | KEEP | Preserve consumers and upstream mergeability; validate invariants |
| Native PostProcessor / CPS | EXTEND | Existing motion and program seams are candidates for ShopBot M2 |
| Machine profiles / tools | EXTEND | Lab profile belongs in M3 after real values are known |
| Frontend and transport architecture | KEEP | Existing multi-platform deployment and serialization seam |
| Waterjet semantics / OMX | DEFER | Requires process-specific additions; M0 analysis only |
| Major UI rewrite / new universal IR | DEFER | No justification before qualification and real-machine evidence |

No subsystem is selected for REPLACE. Fork delta is documentation plus supplied
`.agents/skills`; source and generated contracts remain unchanged.

## Licensing and agent context

Root LICENSE is GPLv3; workspace/frontend manifests declare GPL-3.0-or-later.
Dependency policy is in `deny.toml`; no fresh cargo-deny audit ran without Rust.
The authored bundled CPS post is GPL per `ivac-cps/src/library.rs`; upstream
Autodesk test-oracle files under optional refs are not bundled and cannot be
assumed redistributable. Inspect third-party licenses before reuse.

Read `AGENTS.md` and `CLAUDE.md`. Their main-branch/local-only/manual-push
instructions describe upstream context and conflict in places. The user's M0
authorization and fork policy control this work: feature branch, authorized M0
push, no merge or PR. `bd prime` was attempted but bd is unavailable. Upstream
Beads records were left untouched; their current issue state was not verified.
