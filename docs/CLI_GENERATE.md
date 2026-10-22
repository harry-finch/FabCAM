# Explicit buffered CLI generation — V1.1

```sh
ivac generate --request reviewed-request.json > response.json
ivac generate part.dxf --request reviewed-template.json > response.json
```

The JSON is a `PipelineRequest`: `project` with `cutting_version: 9`, geometry,
machine, tools and operations; optional `post_processor` and `cps_post`. This is
not a saved frontend ProjectFile envelope. Old versions and missing v9 contracts
are refused. No migration, recommendation calculation or rate seeding occurs.

Each enabled milling operation supplies `params.cutting.rates` with the exact
role/tool assignments: positive integer `rpm`, `feed_mm_min`, `plunge_mm_min`.
Profile stores Rough and Finish and emits its contour with Finish. Pocket, Drill,
Spot and Chamfer preserve their actual assignments through the same validator as
the application. Missing/invalid values, duplicate/mismatched slots and known
machine/tool limit violations fail; unknown ratings remain warnings, never invented
limits. Other operation schedule/geometry and specialized mandatory constraints
remain those of the neutral CAM pipeline.

The request alone selects post and properties. Omitted `post_processor` retains
the shared PipelineRequest LinuxCNC default; this does not supply cutting values.
`cps_post` with a non-CPS post is rejected. CPS requires an explicit bundled selection
and typed JSON properties; external paths/source/exact copies never execute.
All old geometry/post flags (including diameter/depth/step/offset/overcut) and
unknown/duplicate flags are rejected. Modify and review the request instead.

Geometry convenience is bounded to a template containing no segments, text layers
or relief sources and exactly one enabled Profile with `source: {"kind":"all"}`.
Machine, assigned tool ID, source, depth, step and SAFE-Z (`fast_move_z`) must be
written explicitly, along with valid actual-role cutting values. Object/layer
selections and pre-existing geometry are rejected rather than remapped. Import
supplies only segments, never an operation, tool, cutting values or machine.
The canonical request form supports the pipeline's broader supported operations.

[Request fixture](../tests/fixtures/cli-v9/profile.request.json) and
[geometry template](../tests/fixtures/cli-v9/profile.template.json) are **software-only
examples**, not physically tested presets or recommended feeds/speeds. Review machine,
tool, geometry, depth, fixture, stock, origin and all role values before real use.

On success stdout is one JSON `PipelineResponse`: `gcode`, `toolpath`, `stats`,
plus authoritative provenance, warnings, gcode index, timing and supported diagnostics.
Stats now come from the pipeline. The old always-zero `cut_distance` and
`travel_distance` placeholders are removed; counts retain their names but describe
pipeline planning. Preview now uses the pipeline's recorded/interpreted path instead
of the old CLI-specific interpretation. No JSON/program is written before successful
validation and generation. Diagnostics/tracing go to stderr. Output I/O errors can
still interrupt writing; shell redirection itself is not transactional publication.

`stream-gcode request.json --output file` retains its existing transactional per-file
publication, resource/refusal rules and incomplete full-program simulation qualification.
`--two-sided` stays on stream-gcode; it requires an output base and can report partial
publication if the second side fails. Buffered generate does not add a two-sided CLI
mode. Automatic double-sided dowel injection remains excluded. No controller support,
installed-native acceptance or physical qualification is added by this CLI change.

FS-RESET-3 removes the legacy transport fields `fs_context`, `applied_cutting` and
`fs_full_width`; JSON requests containing them are refused. Current v9 examples remain
explicit software fixtures. [Final contract and qualification](CUTTING_CONTRACT.md)
and [output comparisons](releases/FS_RESET_3_RESULTS.md) record the fixed corpus and
remaining native/WASM parity limits. The old direct-emitter G2/G3 winding difference
remains the deliberate pipeline-adoption change documented in the CLI modernization
receipt; the removed direct path is not restored.
