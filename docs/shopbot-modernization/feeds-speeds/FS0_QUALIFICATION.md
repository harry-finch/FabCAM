# FS0 — Architecture and data-model qualification

Date: 2026-10-12. Status: human-approved on 2026-10-12; FS1 subsequently authorized.
Audited source: `f6d17dd89be9229c243709d7c2df06dc49df45fa`, on
`feature/shopbot-mvp`; M4 evidence baseline remains `e7aeebd`.
Audit tracking: `ivac-c943`; human approval gate: `ivac-9ma3`.
Primary workflow: repository-local feeds-speeds skill,
especially `references/fs0-checklist.md`, with architecture, validation and
documentation skills. The FS0 deliverable changed documentation only. The types
and policies below record the approved design at FS0 completion. See
[FS1 qualification](FS1_QUALIFICATION.md) for the subsequent pure library implementation;
CAM integration and production datasets remain future work.

## 1. Source-of-truth and consumer audit

Paths below are repository-relative. The persistent frontend project is a
different representation from the Rust pipeline project: `ProjectFile` uses
camelCase and `{kind: 'ivac-project', version: 1}`; `build-project.ts` constructs
the snake_case Rust request. Both seams must be covered during integration.

| Quantity | Source / serialized location | Editor | Runtime and post consumer / fallback |
| --- | --- | --- | --- |
| RPM | `ToolEntry.speed`, `speed_finish`, `speed_drill` in `project/tool.rs`; portable file `tools[].speed/ speedFinish/ speedDrill` | `ToolLibraryDialog`, `ToolRowSummary`, `ToolRowExpandedEditor` | `resolve_tool_rates` selects Rough/Finish/Drill; unset variants inherit general speed. `pipeline/setup_resolver.rs` clamps to machine bounds into `ToolConfig.speed/ speed_finish`; emit shells call spindle hooks. No per-operation RPM field exists. Struct default speed is 18,000, a legacy default, not material evidence. |
| XY feed | Tool `feed_rate` and finish/drill variants; `Op.params.feed_rate_override`; frontend `operations[].feedRateOverride` | Tool rows and `OpPropertiesPanel` | Operation override wins over both rough and finish tool rates; shared machine ceiling then clamps into `rate_h/ rate_h_finish`. Post receives feed hooks/motions; legacy tool default 800 mm/min. |
| Plunge | Tool `plunge_rate` and variants; `Op.params.plunge_rate_override`; frontend `plungeRateOverride` | Same editors | Same precedence and shared ceiling into `rate_v/ rate_v_finish`. Entry strategies consume these; post cycle/motion hooks receive them. Legacy tool default 100 mm/min. Not derivable from chipload alone. |
| DOC / stepdown | `Op.params.step`, `finish_step`, `depth_list`; tool `default_step`; frontend `step/ finishStep/ depthList` | Operation panel; tool default in summary row | `pipeline.rs::effective_step`: operation then tool, negative only; missing/invalid emits `step_unspecified`, setup substitutes zero. Legacy serialized zero maps to unset. Explicit depth list overrides loop scheduling; total depth/start depth are separate. `MillConfig` and `gcode/z_schedule.rs` turn schedule into Z motions; no post recommendation. |
| Stepover | `PocketParams.xy_overlap`, tool `default_xy_overlap`; frontend `xyOverlap/ defaultXyOverlap` | Operation panel/tool editor | `pipeline/offset_builder.rs`: positive op overlap → tool overlap → 0.5; clamp overlap to 0.05..0.95. Nominal `xy_step = effective_diameter * (1 - overlap)`. Profile cascades also use the fallback. Actual pocket stride can be constrained by offset algorithms; post only sees generated XY paths. |
| Other lateral/depth schedules | ReliefMill `stepover_mm`, WaterlineRough `stepover_mm/ z_step_mm`, tool whirl fields | Kind-specific operation/tool controls | Specialized drivers; `build-project.ts` also supplies relief/waterline defaults. These are not the pocket overlap contract and are outside initial FS1 support. |
| Cutter geometry/type | Tool ID, kind, nominal/effective diameter, tip angle, corner radius, wear, flute/stickout/holder geometry | Tool library/calibration editors | Setup uses `effective_diameter()`; CAM and simulation use geometry. CPS tool metadata uses nominal diameter. `ToolKind::family()` is a geometry family, not a cutting-data family. Preserve it. |
| Flute count | Tool `flutes` (`u8`), frontend `flutes`; default 2 | Tool summary row | CPS tool metadata; inspected rate resolver does not calculate feeds from flute count. Must be validated as positive by FS, never infer it from geometry. |
| Machine limits | `MachineConfig.spindle_rpm_min/max`, `max_feed_mm_min`; frontend `spindleRpmMin/Max`, `maxFeedMmMin` | `MachineDialog`; saved machine/profile | Resolver warns and clamps RPM and both XY/Z feeds. All bounds default unknown. No independent maximum plunge or tool maximum RPM is modeled. Rapid speed/acceleration are timing inputs, not cutting-feed limits. |
| Stock / material | `StockConfig` origin, width, height, thickness, top Z, flip; frontend stock dimensions | `StockPanel` | Envelope, stock checks and removal simulation. No cutting material identity/catalog exists in these models. `docs/material-model.md` describes dexel removal representation, not cutting properties. |
| Operation intent | `OpKind` tagged variants, contour/pocket parameters, tool refs, entry strategy | `OpKindPicker`, `OpPropertiesPanel` | Driver dispatch, offsets, finish ring selection. Profile/Pocket/Engrave are distinguishable; no reliable full-slot classification or instantaneous engagement field exists. |

Audit entry points: `crates/ivac-core/src/project.rs`, `project/{tool,params,op,machine,config}.rs`,
`pipeline/{setup_resolver,offset_builder,cps_ctx}.rs`, `pipeline.rs`,
`gcode.rs`, `gcode/{z_schedule,entry,walk,cps_recorder,preview}.rs`,
`frontend/src/lib/api/build-project.ts`, `state/{project-types,project-file-ops,commands,tool-migration,machine_profiles}.ts`,
`services/file_ops.ts`, and the editors named above.

### Post, simulation and persistence observations

Native posts consume resolved `Setup` via `PostProcessor` spindle, feed, motion
and cycle methods. CPS records those calls into IR; ShopBot consumes IR movement
feed in mm/min and writes `MS` in mm/s (`crates/ivac-cps/posts/shopbot.cps`).
DOC and stepover are already encoded in path coordinates, not post inputs.
Neither path must call FS. Preview/timing/removal consume emitted motion and
tool geometry; they cannot establish cutting efficacy or evidence quality.

`pipeline/cps_ctx.rs::build_section_ctx` independently selects raw tool rates,
clamps RPM silently, and advertises `operation:tool_feedCutting/Plunge/Entry/Exit`
without operation feed overrides or feed clamps. `gcode/cps_recorder.rs` also
uses this advertised plunge rate for movement classification. This is a real
consumer discrepancy, not evidence that ShopBot motion feeds are incorrect.
Future integration must route section metadata and classification through the
same resolved rates as motion; add a semantic regression before correcting it.

`snapshotProject` embeds tools, machine and operations; `restoreProject` uses
embedded machine/tools rather than automatically applying a changed workspace
profile. Toolset/machine side files use JSON `kind/format_version/payload`
envelopes. Existing holder presets (`tool_presets.ts`) are geometry patches,
not cutting presets. Tool naming/renumbering and library import make numeric
tool ID or display name insufficient as global preset identity.

### Conflicts and disposition

| Existing capability | Disposition | FS0 decision |
| --- | --- | --- |
| Tool defaults and finish/drill variants | KEEP | Preserve old projects and manual workflows; treat as legacy user defaults, never as reference/validated material data. |
| Feed/plunge overrides | EXTEND integration | Preserve absent-FS behavior; for FS-applied operations, use one complete pass-rate authority rather than two editable rate sets. |
| Negative step / overlap | KEEP and adapt | Positive DOC and radial ratio in FS; negative step and complementary overlap at CAM boundary. Persist explicit native geometry knobs on Apply. |
| Machine warning/clamp system | KEEP and EXTEND | Adapter maps domain diagnostics into `PipelineWarning` codes/parameters; known tool limits and separate plunge ceiling need later additive model fields. |
| CPS independent metadata resolution | REPLACE during FS3 | Reuse resolved runtime values; no fix/code change in FS0. |
| Material identity | EXTEND during FS3 | Optional first-class process-neutral project identity, separate from stock geometry and milling data. |
| Recommendation capability | New isolated crate | No existing engine to extend. Proposed `ivac-feeds-speeds`; no crate created here. |
| Exact engagement, advanced tools/processes | DEFER | No geometry inference, force model, universal process abstraction, WJ/VINYL coupling or final UI. |

## 2. Boundary and minimum contracts

Choose a dedicated pure Rust `ivac-feeds-speeds` crate over a module inside
geometry-heavy `ivac-core`. Dependencies initially limited to standard library,
Serde and error support; no `ivac-core`, CPS, geometry or UI dependencies.
Core owns a thin adapter and wire-facing DTOs at integration time. The engine
uses copied domain snapshots and stable external IDs/revisions, not references
to mutable project/tool libraries. Integration derives `JsonSchema` wrappers
and uses existing generation, never handwritten parallel transport contracts.

| Proposed type | Minimum semantics |
| --- | --- |
| `FeedsSpeedsRequest` | Machine snapshot with optional min/max RPM and XY/plunge ceilings; material ID/revision and optional family; tool cutting family, effective diameter mm, positive flute count, optional immutable manufacturer/tool identity and max RPM; operation context; aggressiveness; dataset identity/revision. |
| `OperationContext` | Profile/Pocket/FullSlot/Finish/Surfacing/Engraving discriminator; positive optional DOC mm; radial engagement `KnownFullWidth`, `Nominal { ratio }`, or `Unknown`; entry Direct/Ramp/Helix/Unknown; finish intent and allowance when known. Geometry and total depth stay in CAM. |
| `FeedsSpeedsRecommendation` | Tagged `Available` or `Unavailable { reasons }`. Available contains RPM, XY feed mm/min, plunge mm/min, positive DOC mm and optional stepover ratio where applicable; derived effective chipload mm/tooth; diagnostics; aggressiveness; source evidence and result applicability; full provenance and normalized request snapshot. |
| `ResolvedCuttingParameters` | Conceptual applied contract: complete rates for every actually used tool/pass role; explicit native step/finish-step/depth-list and overlap fields where relevant; per-field origin Manual/AcceptedRecommendation/LegacySnapshot; last proposal metadata/request snapshot. This is not a second stored copy of native geometry fields. |

Rates calculate in finite floating point; transport/application uses existing
integer RPM/feed precision with a named, deterministic rounding rule and reports
effective chipload after rounding/clamps. FS1 must specify and test that rule.
Ratios are fractions of diameter, never percentages on the wire. Reject zero,
negative, NaN/infinite required quantities, incoherent bounds, or contradictory
full-slot/low-engagement context. Optional means unknown, never unlimited.
Do not promise optional tool limit checks when the adapter has no such field.

### Operation mapping and initial support

Profile → Profile with unknown engagement unless explicit trustworthy full-slot
context is supplied. Centerline cutting alone does not establish that material
occupies the full cutter width. Pocket → Pocket with nominal `1 - resolved_overlap`,
not guaranteed maximum instantaneous engagement: first cuts, corners and islands
can be more demanding. Finish rings/finish allowance → separate Finish request
only when identified by CAM; never reclassify an entire pocket as finishing.
Distinct finish tool → distinct request and independently applied pass rates.

FS1 supports ordinary flat end mills with explicit supported cutting-family
data, Profile/Pocket/explicit FullSlot and known Finish context. It must expose
unknown engagement and only use profiles permitting it (conservative full-width
envelope where supplied by data). Surfacing/Engraving are represented for an
explicit unsupported outcome initially; ReliefMill, WaterlineRough, V-carve,
drilling, thread, form/whirl and other specialized operations are deferred.
Geometry-family equality does not establish cutting-family equivalence; in
particular compression cutters need an explicit data family/profile.
Reject non-milling requests; program-only operations produce no FS request.

## 3. Material, data and deterministic matching

A first-class `MaterialRef` is needed because stock dimensions cannot identify
cutting behavior. Propose optional `Project.material` (frontend counterpart in
`ProjectFile`), containing stable namespaced ID, revision and display label;
grade/condition is part of identity or explicit qualifiers. Default is unknown.
One project material is adequate for today's single-stock model; multi-stock
and mixed materials are deferred. Generic identity belongs to the project;
milling-specific profiles live only in FS data. No non-milling process depends
on the crate or its properties. A missing external material catalog does not
prevent reopening or exporting saved applied values.

Choose versioned JSON envelopes for future datasets, consistent with existing
portable snapshots, Serde and schema tooling. Human notes are explicit fields
rather than JSON comments. Proposed envelope: `kind`, `schema_version`, stable
`dataset_id`, immutable `revision`, material identities, cutting tool-family
definitions and profiles/presets. No material cutting table is populated in FS0.
Separate schema version (shape) from dataset/profile revision (data change).
Reject unsupported versions, duplicate identities, invalid ranges/nonfinite
values, impossible evidence claims and unresolved family references at load.

Each profile/preset carries stable ID/revision, source/author, date, origin or
license/permission, source reference, notes, evidence, explicit applicability
scope and data payload. Payload separates chipload/RPM/DOC/radial/plunge
constraints or targets from empirical correction rules; each rule has ID/revision
and applicability. Exact presets can contain complete reviewed cutting values.
Missing cutting fields do not borrow silently from tool defaults or another
preset. Profile inheritance is flattened and validated during dataset loading,
with field origins retained; no implicit family chain at recommendation time.

Aggressiveness Prudent/Normal/Rapid is independent of Theoretical/ReferenceData/
Tested/Validated evidence. A mode absent from the profile is unsupported, not an
invented multiplier. Source evidence remains recorded when a clamp/adjustment
occurs; the resulting settings are labelled outside the tested/validated scope
unless the recorded scope explicitly covers that transformation. No numeric
confidence score. Promotion to Tested requires an identified actual trial;
Validated requires FS4-approved criteria, complete context and qualifying
observations. M4's successful MDF cut is historical evidence, not a qualified FS
preset; incomplete metrology/tool/limit records forbid automatic promotion.

### Matching policy

1. Validate request and dataset first. Scope dimensions are ANDed. Missing
   request facts never satisfy a constrained selector. Family matching requires
   explicit membership; no matching by display name or substring.
2. Scope includes machine identity/family, material identity/grade or family,
   tool identity/cutting family, diameter, flutes, operation/pass, DOC, radial
   engagement, entry and mode, plus thickness where declared. Numeric ranges
   are inclusive with declared units; endpoints tested. Unknown engagement
   requires an explicitly unknown-safe profile and never matches a low-radial
   validated range. Exact local presets require machine/material/tool identity,
   operation and all recorded physical qualifiers; unknown qualifiers cannot
   be treated as wildcards to elevate evidence.
3. Rank eligible candidates by: exact Validated preset → exact Tested preset →
   machine/material/tool-family profile → general ReferenceData profile →
   explicitly authored theoretical profile. Broad tested/validated records may
   only carry that label when their whole declared range is actually qualified;
   otherwise they are reference profiles with linked trial records.
4. Within a tier, strict scope containment determines specificity: a candidate
   constrained to a subset of another's applicability wins. Equal or incomparable
   maximal scopes produce `Unavailable: AmbiguousProfiles`, with sorted IDs.
   No lexicographic winner, file order, recency or opaque score. Do not fall
   through an ambiguous high tier to a lower tier.
5. No eligible complete source → `Unavailable: MissingCuttingData`, preserve
   manual CAM values and explain missing context. The chipload equation alone
   cannot supply physical cutting data. Legacy tool settings are available to
   the manual workflow, not a manufactured recommendation or evidence label.

Provenance contains engine version, dataset/schema/profile IDs and revisions,
source/license, source evidence, request snapshot, selected tier, inherited-field
origins, assumptions, ordered rule IDs/revisions, before/after clamps/rounding
and result applicability. Evidence never outranks a hard constraint. Invalid
limit intersections (e.g. tool max below spindle min) yield Unavailable.

## 4. Application, persistence and compatibility

Proposal is ephemeral until explicit Apply. Recalculate generates a new proposal;
it does not apply it. Apply is one undoable command and validates all accepted
fields against current context before committing the operation update.

Propose optional operation `milling_cutting` containing complete applied
rough/finish rate triplets keyed by actual tool/pass role and metadata.
Only add slots used by the operation; never make a finish pass inherit later
edits to library rates. DOC/stepover Apply writes existing `params.step` and
kind-specific overlap fields explicitly; the applied contract references these
fields rather than duplicating them in `milling_cutting`. Custom `depth_list`
and finish schedules remain authoritative: applying DOC must explicitly retain
or deliberately replace them in the reviewed proposal, never silently discard.

When this block is present it is the rate authority for that operation; legacy
feed/plunge overrides must be absent, and conflicting imported shapes are
rejected rather than assigned another hidden precedence. Manual edits to applied
operations update this block/native geometry fields and mark edited fields
Manual. Legacy operations retain current override → pass tool defaults behavior.
Converting a legacy operation snapshots every used rate slot and effective native
geometry setting, preserving existing manual overrides and their origins. Apply
does not mutate the tool library. A partial manual acceptance fills other required
slots from reviewed current values; no unspecified rate slot can resolve later
from a changed library. An explicit detach returns to legacy inheritance only
after reviewing that consequence.

Changing machine/material/tool/operation/engagement marks the proposal metadata
stale; applied values remain. A new tool/pass role absent from the saved block
requires explicit resolution before generation, never reuses a different cutter's
applied rates. Staleness uses semantic snapshots, not display-name equality or an
unstable runtime hash. After save/reopen, dataset deletion/revision change cannot
alter applied values; missing provenance links remain inspectable via snapshots.
Manual edits survive recalculation, save/load and undo/redo until deliberate Apply.

Runtime machine constraints remain an independent final safeguard. A newly
changed limit may clamp emitted values and warn without mutating saved values;
therefore reproducibility means unchanged applied parameters and identical output
under unchanged CAM/post/tool geometry/machine context, not byte-identical output
across arbitrary software or machine changes. Compare requested/applied/effective
rates explicitly and recompute effective chipload after constraints.

Integration must introduce a guarded `ProjectFile` version 2 for FS-bearing files
and validate versions in the actual load service. Current `restoreProject` checks
kind but not version: a version bump alone does not stop old readers from ignoring
new authority. Old binaries cannot enforce a new guard; FS files must be clearly
identified as requiring the new reader and must not be opened/exported by old
apps. No compatibility claim for old-app reading of v2. New reader accepts v1,
leaves missing material unknown and preserves absent-FS legacy behavior; it must
not auto-apply recommendations or silently migrate all old operations. Raw Rust
pipeline requests need analogous default/unsupported-field validation, not just
the portable frontend-file guard. Do not save an FS-applied project as v1 by
dropping its rate authority; any future downgrade/export requires explicit values
materialized into compatible fields and a separate approved contract.

Affected integration seams: Rust Serde/JsonSchema and exports, OpenAPI/codegen,
frontend ProjectFile/OpEntry/state/commands, file service and tool/machine imports,
`build-project.ts`, HTTP/CLI/Tauri/WASM request handling, setup/header/dual-tool
resolution, CPS section/tool context, cache keys, native/CPS post and simulation
rates/timing. Additive model/adapter work must preserve all non-milling paths.
Current FS0 changes none of these files or serialized representations.

## 5. Validation contract and evidence

Synthetic arithmetic inputs below establish software properties only, never
recommended plywood cutting settings. Physical cutting values stay unspecified
until legitimately sourced in FS2. Use in-memory synthetic profiles for FS1.

| ID | Deterministic case | Required proof / milestone |
| --- | --- | --- |
| FS-01 | Fixture material labelled plywood, Ø6 mm 2F profile | With synthetic RPM/feed/chipload, prove `feed = rpm * flutes * chipload`, units and normalized context; unknown engagement visible. FS1. |
| FS-02 | Same combination, explicit full slot | Reject low-radial-only profile; use only declared full-slot data/rules; disclose correction and never invent factor. FS1. |
| FS-03 | Pocket, nominal stepover known | Ratio conversion `overlap = 1 - ratio`, e.g. 0.4 → 0.6, and estimated engagement not instantaneous maximum. Engine FS1; CAM adapter FS3. |
| FS-04 | RPM upper/lower clamp and tool bound intersection | Known bounds respected; before/after values and effective chipload correct; contradictory bounds unavailable. FS1. |
| FS-05 | Feed/plunge ceilings | Independently bound rates; shared CAM cap maps to both; unknown cap stays unknown. Recompute effective chipload after feed clamp/rounding. FS1/FS3. |
| FS-06 | Unknown material / missing profile | Unavailable with missing reason; no built-in material constants or substitution of tool defaults. FS1. |
| FS-07 | Unsupported tool/process/operation | Explicit unsupported result; no conventional endmill equation for V tool, WJ, drag knife or program-only operation. FS1/FS3. |
| FS-08 | Exact synthetic validated preset | Exact selector wins; altered machine/grade/flutes/tool/pass or outside range cannot broaden validation; synthetic label confers no physical evidence. FS1. |
| FS-09 | Equal/incomparable scopes and shuffled dataset | Ambiguity unavailable; output/diagnostic order identical; inclusive range endpoints and explicit family matching. FS1. |
| FS-10 | Apply → edit → save/reopen → dataset revision/deletion | All applied slots and origins persist exactly; manual edit survives Recalculate; explicit Apply only mutation; undo/redo atomic. FS3. |
| FS-11 | Legacy project/toolset/machine fixtures | v1 unchanged absent FS; missing step/default, finish/drill variants and override precedence preserved; v2 version/conflict handling covered. FS3. |
| FS-12 | Native/CPS/header/dual-tool post and sim | One resolved source in motion/section metadata/plunge classification; per-operation RPM, changed finish tool, clamps, timing and mm/min → SBP mm/s semantics. FS3. |
| FS-13 | Serialization/schema/data validation | Round-trip request/result/provenance; unknown versions, duplicate IDs, bad ranges and unsupported claims rejected; schema and generated TS synchronized. FS1/FS2/FS3 by seam. |
| FS-14 | Context edit after Apply | Values retained, proposal stale; changed tool role refuses unresolved output; engine update alone cannot mutate the project. FS3. |
| FS-15 | Invalid/precision inputs | Zero flutes, nonfinite/negative required inputs, ratio bounds and unrepresentable rounded rates rejected; deterministic repeated requests. FS1. |
| FS-16 | Evidence and aggressiveness | Independent axes; adjustments cannot silently retain validated-result claim; mode unavailable without data; provenance complete. FS1/FS2. |

FS4 defines and approves physical acceptance criteria before evidence promotion;
record exact machine, material grade/thickness, tool, entry, DOC/WOC, rates,
workholding, chips, heat, vibration, finish, dimensions/tolerances and operator.
FS0 makes no new machine-validation or preset-validation claim.

Executed on 2026-10-12:

- `git diff --stat 9168104 -- crates/ivac-core/src/project crates/ivac-core/src/project.rs crates/ivac-core/src/pipeline/setup_resolver.rs`: empty; audited model/resolver files match local upstream baseline. `upstream/main` does not exist locally; no fetch/sync performed or latest-upstream claim.
- `cargo test -p ivac-core --lib pipeline::setup_resolver::tests --locked -- --test-threads=1`: initial run stopped before compilation because sandboxed sccache returned Operation not permitted.
- `RUSTC_WRAPPER='' cargo test -p ivac-core --lib pipeline::setup_resolver::tests --locked -- --test-threads=1`: approved execution passed **19 tests**, zero failures. Confirms existing inheritance, step, RPM/feed clamp and warning behavior only; does not validate the proposed FS types/engine.
- `git diff --check`: pass. A Python relative-Markdown-link check across all nine changed/new documents found zero missing targets; trailing-whitespace/newline and changed-path checks include the two untracked documents. Changed paths are documentation only. No build/schema regeneration needed because no implementation changed.

## 6. Handoff and gate

FS0 decisions were approved by the user on 2026-10-12 and are recorded in [ADR-0003](../adr/ADR-0003-feeds-speeds-data-ownership.md).
No blocking source-model unknown remains for this design handoff. Actual machine
plunge/feed/RPM limits, legitimate cutting sources and scoped local physical
evidence remain unknown, explicitly required before corresponding claims.

The user subsequently approved FS0 and authorized FS1. Its implementation scope is the
pure crate/domain validation, deterministic selection/arithmetic and synthetic
tests FS-01/02/04–09/13/15/16. FS1 specifies transparent RPM selection, rounding
and correction semantics backed by supplied profile fields; it cannot invent
material constants or usable production data. FS2 supplies reviewed datasets;
FS3 supplies integration/migration/consumer fixes; FS4 supplies calibration.
FS0 ended here. At that handoff, no engine, schema artifact, final UX or FS1 work
had started; subsequent FS1 progress is documented separately.
