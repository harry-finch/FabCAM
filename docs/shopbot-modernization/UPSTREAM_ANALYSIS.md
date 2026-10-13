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


## M1 fork delta

Imports and milling remain KEEP with targeted corrections, not replacements.
The fork now changes `geometry.rs` (arc extrema bounds),
`cam/offsets/parallel.rs` (conservative island join clearance),
`cam/offsets/pocket_fill.rs` (cascade/spiral ring stride cap), and the existing
island test in `cam/offsets/mod.rs`. New qualification integration tests and
16 SVG/DXF fixtures document regressions. No model/schema/dependency change.
These four upstream files may conflict during future synchronization: preserve
or rerun the independent M1 invariants when incorporating upstream fixes.
See M1_QUALIFICATION for altered stepover behavior and performance limits.

Beads 1.3.1 imported the original 1,165 records; upstream statuses preserved.
Only two new M1 issue records are appended, both closed after software validation.
No upstream sync, commit/push, PR or merge was performed in M1.


## M2 fork delta

EXTEND `ivac-cps/src/library.rs` with an authored ShopBot script. KEEP the existing
recorder, CPS engine, native posts, picker and extension-aware export. Generic
optional metadata is added in `gcode/cps_recorder.rs` and `pipeline/cps_ctx.rs`;
no model/schema/dependency change. Add owned-post guard tests, six pipeline tests
and five full SBP goldens. Future sync must preserve preflight for omitted IR
records and machine-policy context; run both M1 and M2 suites after integration.
ADR-0001 explains browser limits/private IR coupling/manual spindle policy.
Two existing infrastructure weaknesses surfaced, tracked separately rather than
rewriting caches or build scripts in this milestone: ivac-htc8 and ivac-5g51.
M1 publication authorized and completed; M2 remains uncommitted pending approval.

## Post-M4 FS planning disposition

**DEFER implementation**, with an eventual additive engine/adapter extension to
be qualified in FS0. Preserve current Tool defaults, operation overrides, machine
constraints and CAM/post consumers until their ownership/compatibility audit.
No recommendation capability is claimed by this planning update. No source,
public model, schema or dependency changes; no upstream synchronization.
See ADR-0002 and feeds-speeds/README.md for the requested milling-only boundary.


## FS0 disposition — 2026-10-12

KEEP upstream models/defaults for legacy projects, geometry/simulation/post seams
and non-milling behavior. EXTEND through a future isolated FS crate and core
adapter, optional material identity and operation applied-rate authority. REPLACE
independent CPS section rate selection with shared resolved rates during FS3;
DEFER advanced cutting/process semantics. The audited tool/operation/machine
models and setup resolver match local upstream baseline 9168104; no claim about
latest remote upstream. No fetch/sync or source/dependency/schema delta in FS0.
See FS0 qualification for compatibility seams and ADR-0003 for design proposals.


## FS1 fork delta — 2026-10-12

EXTEND workspace with isolated `crates/ivac-feeds-speeds`, using only existing
Serde/serde_json lockfile versions; Cargo.lock adds the local package only.
KEEP all upstream core models, geometry, pipeline, native/CPS posts and frontend.
No integration dependency points from those packages to FS. Existing workspace
CI automatically includes the new member; schema drift guard still passes.
Future upstream synchronization has a small manifest/lockfile conflict surface;
retain the crate and rerun FS semantic tests. No upstream fetch/sync performed.


## FS2 fork delta — 2026-10-12

EXTEND isolated FS crate with pure JSON loader/catalog validation; serde_json
moves from test-only to normal dependency at existing lockfile version. Add
separate CC BY-SA 4.0 source-derived data/attribution/license. KEEP upstream CAM,
public models, schema generation, posts and frontend. Loader owns no filesystem
or network. No upstream source edits or integration dependencies introduced.

## FS3 integration delta

Fork-owned core adapter, optional material/identity context and applied rate
slots; additive schema/HTTP/Tauri/WASM calls; guarded portable FS project format;
milling panel explicit Calculate/Apply and manual values. Shared setup/CPS
consumers now use saved resolved rates. Legacy inheritance remains covered by
workspace golden tests. Source data stays independently licensed and separate.
Existing ignored WASM imports were corrected for release browser execution.
See FS3 qualification for scope and evidence; no upstream sync or PR.

## FS3.5 fork delta

EXTEND existing AppliedCutting with optional proposal for manual operation rates;
KEEP tool defaults and native geometry fields. EXTEND guarded portable cutting
format v3, frontend job material label and explicit reference compatibility.
REORGANIZE operation/source/library/machine UI; KEEP inventory/profile IDs and
non-milling paths. Source Settings reuse per-installation persistence. Generated
schema/types must follow optional proposal. No engine coefficients or ShopBot
post delta. See ADR-0004 and FS3.5 qualification before future upstream merging.


## FS3.7 fork delta

EXTEND ivac-feeds-speeds with source-pack envelope, application taxonomy and
conservative per-record resolver; KEEP existing numerical Engine and post paths.
EXTEND applied metadata and guarded portable v4 for private-safe identity summaries.
REPLACE production Sienci toggle/import-body architecture with generic reviewed
registry and installation-local storage; retain legacy migration inputs. EXTEND
native IPC and local WASM validation without a private HTTP data route. Schema/types
must be regenerated together. New public source licensing is independently reviewed;
no automatic upstream sync, source-data relicensing or process-model change.

FS3.8 adds fork-local SourcePack authoring metadata (schema 2, schema 1 compatible),
read-only core/WASM capture and private native/browser revision writes plus compact
Settings UI. No upstream sync, mathematical change or ShopBot post modification.
See ADR-0006 for identity, validation and compatibility decisions.

## UX1 fork delta

KEEP generic Profile/cascade Pocket planners, cutting values and existing CPS arc
linearization. EXTEND generic transient section metadata and bundled ShopBot
preflight for scoped Ramp and explicit unqualified-entry/WCS refusal. EXTEND existing
inspector with a small contextual paragraph and export with runtime input identity;
no schema, FS mathematics or upstream synchronization change. Generic numeric WCS
translation remains follow-up ivac-vj11, guarded for ShopBot in UX1.

UX1 continuation: EXTEND bundled ShopBot's post/preview adapter, using the owned
emitted movement subset for final simulation rather than generic IR. KEEP other
CPS previews and generic geometry. EXTEND generic emission only with an explicit
Helix geometry-fallback comment, not a ShopBot branch or path rewrite. Existing
warning params expose derived requested/resolved representation; no API/model
migration. See ADR-0007 for upstream impact and alternatives.

CAM2/UX2 fork delta: generic once-before-CAM XY work-frame normalization, invalid
origin/fixture guards and resolved-path fixture warnings; compact stock-corner/WCS
and fixture authoring UI with existing persistence/history. No schema migration,
FS engine change or machine-specific geometry semantics. See ADR-0008.
