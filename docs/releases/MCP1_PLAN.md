# MCP1 — AI Integration & MCP Server

2026-10-18 · **PLANNED — NOT IMPLEMENTED · POST-V1.5 / FabCAM 2.0+**.
Documentation checkpoint ivac-c2q72. Optional integration, not a dependency of FabCAM,
ARCH-MACH, audit closure/TEST1, ShopBot V1 qualification/distribution or native OMX V1.5.
No server, network endpoint, dependency, command, schema or output change now.

## Product objective and candidate deployment

Compatible AI clients inspect authorized projects/geometry/tools, request cutting
suggestions and propose operations for user review. FabCAM remains fully usable without
AI subscription/vendor/API, MCP client, cloud or embedded chatbot. MCP is a thin protocol
adapter to authoritative services, not a second CAM/FS business-logic implementation.

Consider a dedicated local `fabcam-mcp` executable using stdio initially, subject to
MCP1-A architecture/security audit. No executable exists or transport/SDK/version is
selected here. Remote service/new listener needs a separate security review and explicit
authorization. Current HTTP availability does not make all endpoints safe MCP tools.

## A — Architecture and API audit

Audit source/availability/consumer/ownership/validation for each capability; classify
KEEP/EXTEND/DEFER and identify smallest service extraction before selecting interfaces.
Preliminary source inventory, **not a completed headless API audit**:

| Existing seam (repository-relative) | Reuse and limits |
| --- | --- |
| `crates/ivac-core/src/{project.rs,input.rs,drawing/,pipeline.rs}` | Rust models/import/accepted-geometry/generation can run without Svelte; standalone ownership, serialized validation and exposure limits still require audit |
| `crates/ivac-cli/src/main.rs` | Existing headless import/generate/stream entry points; some CLI paths directly orchestrate lower-level APIs. Do not assume identical semantics to desktop/full pipeline |
| `crates/ivac-server/src/main.rs`, `ivac-tauri`, `ivac-wasm` | Existing generate/drawing/FS/diagnostic transports and feature differences; reuse services/guards, not a new HTTP listener or assumed cross-backend parity |
| `frontend/src/lib/services/file_ops.ts`, `state/project-file-ops.ts`, `project-types.ts` | Portable project load/save, compatibility/migration and authoritative user file workflows; Rust Project wire data alone is not the complete portable desktop document |
| `state/workspace.ts`, `machine_profiles.ts`, `tool_inventory.ts` | Installation-local profile/inventory plus project embedded snapshots; not automatically available to a separate executable or authorized for disclosure |
| `state/project-operation-ops.ts`, `commands.ts`, `history.ts`, `palette-dispatch.ts` | Existing operation mutations, Undo and shell-owned commands; live actions may require service extraction, not direct file edits or invoking UI callbacks from an unrelated process |
| `crates/ivac-feeds-speeds`, core FS adapter, applied-cutting models | Pure suggestions separate from saved values; local pack access/provenance/privacy remain caller-controlled |
| `generation-context.ts`, export preflight, core capability/post checks | Freshness/resource/cancellation/final-output authority must remain; no MCP-only ready/safe flag |

Audit process capabilities dynamically against qualified support and backend availability,
project lifecycle, pending edits/generation, error envelope and data sensitivity. No universal
application-service contract or project revision token is claimed implemented by this plan.
Human architecture/security/tool-scope gate before server implementation.

## B — Useful read-only MVP

Candidate tools, **illustrative names, not frozen/implemented schemas**:
`inspect_project`, `list_geometry`, `analyze_geometry`, `list_machines`, `list_tools`,
`inspect_operations`, `suggest_cutting_parameters`, `inspect_generation_diagnostics`.
Expose only actual available qualified services and authorized project/data scope.
Structured validated input/output with source identity/lineage, units/frame, provenance,
scope/qualification and stale/current status. Analysis/proposal never applies repairs,
rates or operations; recommendations are not AppliedCutting or physical validation.

MVP is useful without any writes or generation capability. Authorize computation/read
of private packs separately; return only permitted derived provenance, not raw private
source profiles or arbitrary local files. List tools/machines only from authorized scope,
not automatic workspace enumeration. Diagnostics disclose no hidden file/script content.
Bound queries/results/geometry analysis and reject unknown IDs/invalid input honestly.

## C — Later controlled editing and distinct state owners

Candidate approved actions: create/patch operation, assign tool, propose repairs, reorder
and apply explicitly chosen cutting values. Workflow:
**Inspect → Propose → Preview diff → Validate → Human approval → Apply**.

Bind proposal to project/session identity, exact revision/context and precise change set;
approval cannot authorize a different later mutation. Revalidate live state/capabilities
on Apply; reject conflicts/stale proposals, no implicit last-writer-wins. Preserve source
lineage/bindings, existing validation/command history, cutting-value authority, atomicity
and generated-output invalidation. Reordering is explicit intent, not agent optimization
that silently changes machining order. Proposal/search/preview do not mutate.

Evaluate separately:

- **Offline independent document:** explicit load/copy scope, snapshot/revision owner,
  transaction/Undo semantics to design; save-as/export only with permission and conflict/
  overwrite checks. Do not overwrite a desktop-open original behind the application's back.
- **Live desktop session:** desktop owns pending/unsaved data, history and generation.
  Authorized session bridge must dispatch existing authoritative actions with revision
  checks and user-visible approval. A file timestamp or a separate server copy cannot
  represent unsaved state; no shadow project store or bypass of desktop Undo.

Human preview/approval should reuse compact existing FabCAM review/diff patterns under
[$professional-workshop-ui](../../.agents/skills/professional-workshop-ui/SKILL.md),
with clear units, before/after impact and keyboard-accessible consequential actions;
no embedded chatbot or new permanent dashboard required.

Commit/rollback/recovery after client disconnect or validation failure require explicit
audit. No write-capable interface implemented in this roadmap task. Phase C has separate
human security/state/UX approval after useful read-only qualification.

## D — Authorized generation and output analysis

Later client requests may generate under explicit permission and inspect bounded results.
Use actual source bindings/effective geometry, saved rates, post/capability checks,
resources/cancellation, structured refusal/blocking warnings and resolved-output parity.
Capture immutable request identity; reject late/stale/partial/failed results as valid output.
Preserve publication/preflight gates, not merely generation success. Inspection of returned
program data and writing/exporting a generated file are distinct authorized scopes.
Explicit approval is required for file export; client presence is insufficient.

No execution, jogging, spindle/hardware control. Future FabSBP needs its own separate
hardware authority/human-safety contract, not a widened FabCAM MCP permission.

## Security, privacy and auditability — audit before implementation

Default local-only; stdio is a candidate, not automatic trust. Authorize project/session
and filesystem scopes explicitly, validate paths/handles/identities and prevent traversal,
unauthorized symlink access and ambient file enumeration. Protect imported/script/post
content, local source packs and secrets; minimal results and redact sensitive diagnostic
details. Protocol schemas are validation, not authorization.

Separate inspect/analyze/recommend, edit, generate and export permissions. Limit request
size, geometry/iterations/time/memory/concurrency/output and safe cancellation; no arbitrary
shell/script/path execution exposed as a tool. Preserve corrected HTTP/CPS/import/desktop
guards, including HTTP CPS-path refusal; do not copy transports without their security
contracts. Authentication/consent identity, IPC trust and remote threat model remain open.

AI instructions and project/text/metadata/tool results are untrusted data, never permission
to broaden scope, apply changes or export. Audit proposals/approval/revisions/applied actions
with privacy-aware retention; no silent telemetry or cloud dependency. Error/disconnect/
retry/idempotence policy must prevent duplicate mutation or successful partial output.

## ARCH-MACH and developer documentation

[ARCH-MACH](ARCH_MACH_PLAN.md) defines qualified process/machine/adapter extension;
MCP discovers/exposes its supported capability contract without hardcoded ShopBot/OMX/
knife assumptions. A new qualified machine should normally not require MCP business-logic
rewrites. ARCH-MACH/FabCAM never depend on MCP; future adapters remain independent.

[Developer planning guide](../development/MCP_INTEGRATION.md) indexes architecture,
setup/transport/schema policy, example workflows, lifecycle/errors/security/tests/limits.
Future small `.agents/skills/mcp-integration/SKILL.md` may link these authoritative docs
after architecture approval; **not created now**, no duplicated domain authority.

## Acceptance, evidence and phase gates

All future requirements, no PASS claimed: protocol/version/cross-client conformance;
validated schemas/errors; deterministic scoped reads; offline/local operation; access/
private-pack restrictions; read/write separation; human approvals/revision conflicts;
desktop unsaved edits/history/persistence; stable geometry identities; stale/refused
output rejection; unchanged qualified CAM safety; resource/cancel/disconnect behavior;
negative/adversarial/path/untrusted-content tests and no unsupported capability claims.

A delivers source-backed service/state/security design and approved transport/subset.
B delivers qualified read-only MVP and cross-client evidence; C transactional edits
require separate gate; D guarded generation/export likewise. Each ends with human review,
not automatic continuation. No phase delays V1/V1.5/current audit/TEST1 work.

Open: headless portable loading, scoped workspace/private-data access, minimum service
extraction, stdio/client identity, actual protocol/SDK compatibility to verify at future
implementation, offline revision/history and desktop bridge ownership, approval UX,
export boundary, audit retention and supported first tool set. No schema frozen now.

**HARD STOP — awaiting human approval of the MCP1 roadmap addition.**
