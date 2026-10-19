# ARCH-MACH — Machine & Process Extensibility Contract

2026-10-18 · **PLANNED — NOT IMPLEMENTED / NOT PRACTICALLY VALIDATED**.
V1.5 architectural preparation and first practical validation, not a generic plugin
program or V1 blocker unless an actual supported ShopBot defect requires triage.
Documentation checkpoint ivac-bens7; no new machine support or framework now.

## Placement and success condition

ARCH-MACH boundary/workflow planning → OMX1 verified native output contract and
human architecture approval → WJ1–3 process/adapter implementation/qualification
→ MAKE/WJ4 physical and ShopBot non-regression → V1.5 approval. Initial guide evolves
with actual ProtoMAX findings, not a speculative universal framework before delivery.
OMX1 research can inform the contract iteratively after authorization; production
writer architecture must not precede sufficient output evidence. Main campaign retains
accepted V1/time-boxed AUDIT3 prerequisites and current review gates.

Same supported process, new machine: aim for capability/profile + adapter + final-output
validation + fixtures/qualification + machine docs, normally without core geometry
changes. New process: may need operations, physical state semantics and contextual UI.
This is a design target, not a claim every protocol fits the existing motion API.

## Source-backed extension inventory — preliminary, audit before implementation

| Existing seam (repository-relative) | Reuse / limitation |
| --- | --- |
| `crates/ivac-core/src/geometry.rs`, `input/`, `drawing/`, frontend effective geometry | KEEP import/edit/identity/topology/transform boundary; no machine-name conditions in geometry |
| `project/machine.rs`, `project/op.rs`, `cam/setup.rs`, `pipeline/op_drivers/` | KEEP explicit modes, operation variants and setup. Mode/capability fields are coarse eligibility, not proof of adapter support |
| `frontend/src/lib/state/machine_profiles.ts`, `workspace.ts` | KEEP named local machine/tools bundles and embedded project copies; not a universal cross-process or shared-device plugin schema |
| `gcode.rs::PostProcessor`, `gcode/emit.rs`, native LinuxCNC/GRBL/HPGL paths | KEEP existing adapters where semantic fit is demonstrated; planning/emission currently interact, not a fully universal machine-independent process IR |
| `gcode/cps_recorder.rs`, `crates/ivac-cps/src/ir.rs`, runtime/posts | KEEP feature-gated versioned CPS recorder/runtime contract and units; existing Autodesk/milling-oriented fields do not establish waterjet pierce/quality or arbitrary binary OMX semantics |
| `pipeline/cps_ctx.rs`, `pipeline/shopbot_resolution.rs`, bundled `shopbot.cps` | KEEP explicit capabilities/refusals and final emitted-output decoding/source mapping as an output-validation precedent, not a universal decoder |
| `sim/`, frontend preview/sim paths | KEEP shared rendering/motion where applicable; cutter/depth material simulation is not qualified jet/knife simulation |
| Core Serde/JsonSchema, schema/codegen, HTTP/Tauri/WASM clients | KEEP compatible contracts and transport feature checks; new process/version changes need explicit audit, not frontend-only support |
| Machine/tool/operation inspectors, tool-family/applicability helpers | KEEP contextual fields/unsupported values and current UI ownership; new process may require new semantics/UI |
| ShopBot guard/qualification and independent decoded-output tests | KEEP positive/negative/independent evidence approach, not inherited universal physical certification |

Known friction: coarse capability booleans versus concrete post subset, motion/CPS
semantics tied partly to milling, persisted tool/operation fields, unequal backend
features, caches/diagnostics/source mapping and process-specific simulation. Identify
small additive changes only after real contract evidence; DEFER universal IR/plugin
registry, runtime plugin discovery and broad unrelated migration. No Rust/API schema
selected in this planning document. Preserve prior ADRs and explicit refusals.

## Six-part contract

Shared geometry → process operations/physical meaning → machine capabilities →
output adapter → actual-output validator → qualification package. Each layer names
owner, supported subset, inputs/units/frame, unsupported behavior and evidence.
Capabilities describe operations/axes/units/conventions/limits/features, but **profile
settings cannot authorize output the adapter cannot correctly represent**. Use strict
intersection of qualified process/machine/adapter/backend support; unknown combinations
fail explicitly, never silently convert intent. No invented limits or protocol fields.

## Developer/agent deliverables

Initial [ADDING_A_MACHINE guide](../development/ADDING_A_MACHINE.md) and
[integration specification template](../development/MACHINE_INTEGRATION_TEMPLATE.md)
link existing architecture/qualification instead of duplicate them. Include standard
completion/HARD STOP reports, source inspection, reference analysis, positive/negative
tests and human gates. This documentation is delivered; contract adoption and actual
integration are still planned, not certified by creating a guide.

Proposed future skill: `ivacam/.agents/skills/machine-integration/SKILL.md`, alongside
existing local domain skills. A concise entry links guide/template and relevant process/
post skills; requires evidence ledger, missing references, scoped architecture proposal,
compatibility/regression report and stop on unestablished semantics. **Not created or
installed now**; no new runtime/skill authority or duplicated architecture text.

## Practical validation and future case studies

ProtoMAX is first practical validation: direct native OMX without LAYOUT preparation,
installed MAKE execution, official user-provided references and OMX1 ownership gate.
Waterjet quality/kerf/pierce/bridges/order are not milling RPM/Z/tool placeholders.
Record guide improvements and actual extension effort/changes during WJ implementation;
architectural success criterion “ProtoMAX validates the approach” remains unmet now.

Post-V1.5 conceptual stress tests, **no vendor research/support claim or implementation**:

- WAZER: could reuse qualified waterjet process, but machine/output constraints, vendor
  permissions, transfer/acceptance and physical safety require independent evidence.
  Do not assume direct external program generation is allowed or equivalent to OMX.
- Graphtec/Silhouette: knife process needs qualified blade offset, up/down, force,
  corner compensation/material handling and verified protocol. Existing Drag/HPGL is
  partial precedent, not supported GP-GL/vendor compatibility. First knife process may
  need semantics; second compatible machine should ideally reuse it. Preserve deferred
  VINYL work; neither case justifies implementing a protocol now.

## Acceptance and remaining human decisions

Require documented geometry/process/machine/output boundaries and real extension points;
repeatable evidence-driven workflow; reference methodology; qualified capability subset
and fail-closed output; independent verification with limits; usable developer/agent guide;
standard software/physical acceptance; normally no core-geometry rewrite for another
same-process machine; actual ProtoMAX implementation/qualification evidence later.
Do not mark last criteria complete on documentary inspection alone.

Open: minimal additional waterjet process records, adapter/validator boundaries versus
CPS/native APIs, backend capability exposure, persistence version/preview effects,
artifact/licensing policy, optional future skill adoption and practical guide validation.
Approve initial ARCH-MACH scope/workflow, then OMX1 dossier and writer architecture;
actual coding/physical/publication steps each require their existing authorization.
No current review, release ID, TEST1/AUDIT3/platform or physical gate is closed here.

**HARD STOP — awaiting human approval of ARCH-MACH before implementation.**

## Optional later MCP consumer

[MCP1](MCP1_PLAN.md), planned POST-V1.5/V2+, may discover and expose qualified process/
machine/adapter capabilities through shared authoritative services. It must not hardcode
machine semantics or duplicate CAM when a new machine is added. ARCH-MACH/ProtoMAX and
FabCAM do not depend on MCP or an AI service; capability discovery grants no mutation,
export or hardware authority. No integration implemented by this cross-reference.

## MULTI1 linked product objective and STEP source independence

MULTI1 is [shared-process product experience](SMART_WORKSHOP_PLAN.md), not another
architecture milestone: reuse accepted geometry/cleanup across machines while preserving
process-specific operations. Switching machine never silently converts milling to jet;
qualified semantic mapping needs explicit preview/approval, otherwise preserve original
intent and offer process-specific replacement. V1.5 foundation remains native OMX/WJ;
advanced assistance stays V2. [STEP1](STEP_PLAN.md) adds a process-independent source
adapter later, not ShopBot/ProtoMAX-specific import or a requirement for V1.5.
