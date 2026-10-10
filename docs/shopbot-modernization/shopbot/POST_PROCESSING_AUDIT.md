# M0 post-processing audit

No OpenSBP command syntax was researched or emitted in M0. No machine profile
was invented. Machine/software specifics remain **TO VERIFY ON MACHINE**.

## Existing representation

`crates/ivac-core/src/gcode.rs` defines the stateful PostProcessor trait: unit,
feed, spindle CW/CCW/off, tool, coolant, rapid/linear/XY arcs, machine-frame moves,
drill cycles, program flow and optional structured section hooks. Native posts
are `gcode/{linuxcnc,grbl,hpgl}.rs`; configurable native templates are in
`gcode/post_profile.rs`. The templates still sit in G-code/plotter-oriented
emitters and do not by themselves establish a safe OpenSBP dialect.

`gcode/preview.rs` represents motion separately for simulation. It is not a
complete machine event program. `pipeline/cps_ctx.rs` supplies program, section,
tool and scaffold ownership context. CPS recording replaces native scaffolding,
so the future post need not inherit a LinuxCNC prologue or tool-change envelope.

| Semantic | Evidence / consequence |
| --- | --- |
| Lines/rapids | Optional axes in trait; CPS recorder resolves absolute XYZ |
| Arcs | XY CW/CCW hooks; recorder resolves center into Program Circular, feed, end and normal; arc fitting in `gcode/arc_fit.rs` |
| Feeds | Core uses mm/min; recorder stamps modal feed onto motion; native trait feed API is u32 |
| Spindle | CW/CCW/speed/off and dwell; Section has initial RPM/direction, records handle changes |
| Tools | ToolEntry model, section tool specification; section boundaries and mid-section changes; native manual/ATC strategies exist |
| Cycles | Simple/peck/chip-break hooks; native defaults can expand to moves; CPS retains Cycle/CycleEnd records |
| Units | Machine UnitSystem mm/inch; CPS IR invariants are mm, mm/min, radians; JS driver applies output-unit scaling |
| Precision | Native formatting and machine separator/arc tolerance; CPS `createFormat`/output variables control output grid; prelude tests verify rounding/suppression |
| Raw commands | PassThrough/raw and GcodeInclude exist; unsupported foreign commands must be rejected or explicitly handled by a future post |
| Coordinate frames | Machine rapid distinct from WCS in IR; ShopBot implementation must preserve the distinction explicitly |

## CPS extension seam

`ivac-cps/src/ir.rs`: versioned Program/Header/Sections and tagged Records.
`engine.rs` / `run.rs`: Boa execution, bundled prelude and diagnostics.
`prelude/15_driver.js`: post callback dispatch; `12_cycles.js`: cycle support.
`library.rs`: authored bundled registry currently containing grbl only.
Project post selection and source/config handling are in `pipeline.rs` and
`frontend/src/lib/components/CpsPostConfig.svelte`.

CPS is optional: CLI/server/Tauri enable it by default; WASM does not. A ShopBot
CPS post therefore needs capability-aware availability and an explicit decision
on browser support. The existing register/selection/runtime paths can be extended
without changing compensation or milling path generation, subject to M1/M2
validation. Native PostProcessor is an alternative; some default cycle/tool
helpers contain G-code text and must be overridden/audited for OpenSBP.

## M0 conclusion and M2 decision boundary

The inspected representation appears sufficient for the minimal 3-axis milling
corpus. No generic CAM change is currently justified just to add ShopBot output.
This is a feasibility inference, not an end-to-end proven result. M2 must decide
CPS vs native in an ADR using maintainability, testability, licensing, available
events and capability distribution. CPS remains the user's preference if it
meets requirements cleanly.

M2 must verify every emitted command using authoritative ShopBot documentation,
test arcs/units/feed conversion/precision/cycle expansion/tool changes/safe
motion/program start and end, and reject unsupported semantics rather than
silently alter intent. Actual PRS controller/software, arc behavior, zeroing,
spindle wiring, tool handling and safe positions are not established by M0.
The ProtoMAX implication is different: its process event gaps are recorded in
FUTURE_PROCESSES and cannot be solved by merely choosing a different dialect.
