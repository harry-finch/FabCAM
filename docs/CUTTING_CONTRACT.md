# Current V10 cutting, preset and safety contract

Source identity and qualification provenance live in [STATUS](shopbot-modernization/STATUS.md).
Production owners are `ivac-core::project`, `cutting_parameters`, `cutting_stages`,
`presets`, `pipeline::op_drivers::staged`, `stage_polygon`, `stage_provenance` and
`setup_resolver`. Rust types generate [OpenAPI](../schema/openapi.yaml) and
[TypeScript](../frontend/src/lib/api/generated.ts). Accepted
[ADR-0030](shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md),
[ADR-0032](shopbot-modernization/adr/ADR-0032-explicit-main-and-optional-finishing.md)
and [ADR-0033](shopbot-modernization/adr/ADR-0033-explicit-pocket-finishing-scope.md)
own the versioned decisions. No recommendation engine or live FabSpeed client is active.

## Persistence and generation

Frontend ProjectFile **version 10** is distinct from a native `PipelineRequest` with
**`project.cutting_version: 10`**. Adapters negotiate `cutting-parameters-v10`;
scoped Pocket additionally requires `pocket-finishing-v2`. Portable versions 1–9 and
future versions refuse before project/history mutation. Historical V9 requests,
files and fixtures remain evidence, not current examples or automatic migrations.

Profile and ordinary Pocket author `params.stages`. Both discriminators require
`main` and a present nullable `finishing`; Finishing OFF is `null`, not an inferred
wall-path tag. Each stage owns actual `tool_id` and required nullable `rpm`,
`feed_mm_min`, `plunge_mm_min`, `depth_per_pass_mm`. Null values preserve incomplete
editing; generation requires positive integral rates and positive constant DOC.
Redundant operation tool assignments must agree. Legacy rate/depth fields cannot
coexist with stages. Other families retain `params.cutting.rates` and their exact
actual-role/tool set within the V10 envelope; no synthetic Main/Finishing conversion.

| Nested contract | Supported operation / Finishing fields |
| --- | --- |
| `main_finishing_v1` | Profile and existing ordinary Pocket. Explicit Main and optional Finishing with stage, lateral/floor allowances; **no scope field** |
| `pocket_finishing_v2` | Ordinary Pocket only. Finishing requires `scope`: `walls_only`, `floor_only` or `walls_and_floor`, alongside its stage/allowances |

Mixed/missing/unknown discriminators, missing keys, wrong family, a scope added to
v1 or contradictory v2 allowances refuse. V10 is retained for both discriminators;
new nested semantics are not smuggled into v1. No automatic v1-to-v2 normalization.
Tool defaults seed creation once; generation never fills missing authored rates.
Known machine/tool violations refuse, unknown ratings remain disclosed; specialized
families may require known ratings to prove their own constraints.

Rust generation JSON structs reject unknown fields. Frontend file loading explicitly
rejects retired source/recipe fields, including null placeholders, while retaining
its existing unrelated extension-field preservation. WASM guards retired keys and
required nested stage shapes before serde conversion for ordinary, paired, streaming
and drawing generation. Do not claim every unrelated unknown JS property is rejected.

## Main, Finishing and retained stock

Main Cutting always exists on Profile/Pocket; Finishing is OFF on new operations.
Enabling it requires authored tool/rates/DOC and relevant allowances. Copy Main is
an explicit editor action, not a calculation. Positive finishing allowances must be
at least **0.05 mm**; zero means an explicit repeat. Retained-for-later stock `R`
is separate: Main targets cutter radius + R + lateral allowance and final Z + floor
allowance; Finishing targets its own radius + R and final Z where its scope applies.
When Finishing is OFF its allowances cannot influence Main.

| Pocket v2 scope | Main target / finishing obligation |
| --- | --- |
| Walls | Main leaves lateral finishing stock, reaches final floor; Finishing covers outer/island walls at every authored axial level; no finishing interior clearing fill |
| Floor | Main reaches final walls and leaves floor stock; Finishing clears the reachable final floor |
| Walls and floor | Main leaves both applicable reserves; Finishing proves both wall and reachable floor coverage |

Scope changes use explicit Apply, Cancel and atomic Undo/Redo. Irrelevant allowances
must be zero; changing a label alone cannot silently reinterpret a contract. Combined
Pocket finishing re-clears the reachable target; it is not optimized rest machining.

## Qualified geometry and explicit refusals

[ALPHA-3.1](releases/ALPHA_3_1_GENERALIZED_FINISHING.md) extends the initial rectangle
scope: v1 accepts one exactly closed straight polygon with 3–256 edges, no islands.
Profile Inside/Outside lateral finishing requires equal effective cutter diameters
and lateral allowance no greater than cutter radius; Profile On/floor finishing
refuses. Ordinary Pocket remains bounded to its proven clearing/coverage domain.

[ALPHA-3.2](releases/ALPHA_3_2_POCKET_FINISHING.md) qualifies Pocket v2 polygons and
bounded tangent line/arc joins or circles: one connected region, at most eight
simple disjoint single-level islands and 2–256 source primitives per boundary.
Non-tangent curved joins, self-intersections/touching/ambiguous containment, nested
islands, disconnected regions, unsupported trimming and collapsed offsets refuse.
Flat endmills with known usable flute length, direct entry and ordinary Cascade,
Zigzag or bounded Spiral are required. Tabs, authored approach/leads, ramp/helix,
Trochoidal/whirling, generated frame and Boolean-source proof are excluded.
Single-surface scopes require equal effective cutter diameters; wall lateral reserve
cannot exceed cutter radius. Combined scopes can accept different cutters only
when the entire Main/Finishing proof succeeds. Some concave Spiral cases correctly
refuse the decoded proof budget; no strategy fallback or relaxed tolerance occurs.

Proof budgets include 1,000 axial levels/stage, 5,000 paths/chords per proof set,
eight distinct planar geometries/stage and 5,000,000 checked work units/invocation.
Planned/decoded planar bounds remain 0.003/0.02 mm, with chord/enclosure/arc-rounding
costs charged. Continuous intervals/cells and complete axial coverage are checked;
finite sample points are not represented as a global proof. These are software
representation/proof budgets, not machine accuracy or universal latency guarantees.

## Other actual milling roles

Drill retains Drill plus enabled Spot/Chamfer roles; Chamfer retains Rough and
Finish where its own finishing pass exists. Halfpipe, PlanarSurface, V-Carve,
Engrave, Relief, Waterline, Thread, TSlot and Dovetail retain their actual Rough
assignment and specialized geometry/entry rules. Program controls and non-milling
processes do not acquire invented milling slots. Front/back uses authored roles;
automatic dowel injection is excluded. “Rough” in an unchanged family is not the
new Profile/Pocket Main Cutting model.

## Presets, metadata and provenance

[Collection V2](TESTED_PRESETS.md) stores staged records; historical V1 Profile/Pocket
records remain inspectable but cannot apply Rough/Finish to V10 stages. Unchanged
families can retain exact compatible V1 matching. Both formats keep 2 MiB / 256
record limits, exact signatures/identities and explicit author attestation. No
implicit conversion, live link or attestation renewal.

ALPHA-4 is a separate explicit selective copy of RPM/feed/plunge/DOC from V2 staged
Profile/Pocket into Main or already-enabled Finishing. Cross-context starting values
can be reviewed without claiming compatibility/testing. `fabcamPresetValueReuse`
is advisory file metadata, not CAM input or target attestation. Stepover reuse is
unsupported. Strict tested-preset matching remains unchanged.

TOOL-META substrate/coating are optional descriptive strings; absent/empty is unknown,
not an assertion of uncoated tooling. They do not calculate or override cutting rates.
Output stage bindings/provenance describe actual tool/rates and supported emitted
intervals; planned-only CPS attribution cannot be called decoded Finishing. Same-tool
ShopBot RPM transitions can refuse. Full-program simulation, not playback position,
owns qualification; distinct-tool remaining-material diagnostics remain bounded.

## Safety, publication and security owners

[SAFETY-FIX-1](releases/SAFETY_FIX_1_MOTION_GUARDS.md) refuses unproven Helix entry,
implicit Trochoidal Pocket helix and legacy Profile overcut. Thread's intrinsic
machining helix is distinct. Front through-cut holding needs the narrowly resolved
Profile/MaterialV1 bridge proof; empty/ineffective tabs and affected combined Front
or untransformed Back text/approach/pattern/raster configurations refuse. GEOM1 edited
two-sided generation remains refused. Effective start and final depths count.

[SEC-HARDEN-1](releases/SEC_HARDEN_1_APPLICATION_SECURITY.md) removes broad ambient
raw FS grants and adds restrictive CSP. Custom import/watch commands authorize
canonical dynamic dialog-granted paths; trusted OS-open grants are internal.
Source reload/recent open reacquire missing grants through a native picker. Custom
export retains its separate canonical destination/symlink/dynamic-scope checks and
per-file transactional publication; paired export is not pair-atomic.

HTTP defaults to loopback, enforces explicit Origin admission and two heavy jobs,
returns 429 without queueing excess work, and signals cooperative cancellation on
disconnect/deadline. No universal preemption or authenticated remote mode is claimed.
Import XML/reference/block/primitive/font/STL budgets refuse complete unsafe inputs
rather than publish a successful partial drawing. See architecture for limits.

Regression owners include `tests/alpha3_v10.rs`, `tests/alpha32`, `presets.rs`,
`security_cancellation.rs`, `import_security.rs`, current V10 frontend lifecycle/reuse
tests and source-bound CNC comparisons. Historical test filenames containing `v9`
do not define the current wire version. Software, installed-native and physical
acceptance remain separate; [STATUS](shopbot-modernization/STATUS.md) owns the gates.
