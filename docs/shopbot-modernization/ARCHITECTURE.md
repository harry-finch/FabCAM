# FabCAM fork contracts and process boundaries

This is the active fork-boundary reference, not an M0 snapshot. Current source and
qualification provenance live in [STATUS](STATUS.md); stable layer/data-flow patterns
are in [general architecture](../ARCHITECTURE.md). [HISTORY](../releases/HISTORY.md)
and accepted [ADRs](adr/README.md) preserve the original decisions and milestone evidence.

## Source and contract authority

Production source/generated contracts precede accepted versioned ADRs, source-bound
qualification reports and tests, then historical prose. Package/binary names retain
ivaCAM attribution and compatibility; they are not an assertion of old contracts.
The accepted security feature lineage and canonical V1.1/frozen V1 remain separate.
Do not infer integration, a fresh installer or physical acceptance from a report title.

ProjectFile 10 and `project.cutting_version: 10` are current. Collection V2 is a
separate format. Profile/Pocket stages and reader refusals belong to
[CUTTING_CONTRACT](../CUTTING_CONTRACT.md); do not map old V9 Rough/Finish records
or source/recipe contexts automatically into the current model. ADR-0030 removed the
integrated calculator. FabSpeed is independent; ADR-0031 is Proposed for optional
exchange, not an implemented runtime dependency.

## Geometry, placement and authoring

Imports preserve original coordinates/unit choices and stable source/layer/object
identities. Drawing placement, stock placement and work origin are separate intent;
physical machine zero is established outside FabCAM. Manual stock stays fixed across
supported imports/edits; explicit drawing transforms include mouse translation and
proportional dimension controls. Do not silently rebase source coordinates or stock.

The persisted Drawing contract and GEOM1 evaluate/generate requests are **V1**.
Scoped Analyze/Preview/Apply and their geometry payload are **V2**. Native commands,
HTTP `/drawing`, `/drawing-v2`, `/generate-drawing` and WASM bindings retain these
separate boundaries. Capability negotiation explicitly advertises cutting V10 and
Pocket finishing V2; do not invent a drawing-v2 capability string.

Bounded repair preserves originals and checks continuity/topology, revision/staleness
and mutation through command/history. `local_edit_accepted` is an editing result,
not a CAM or physical safety certificate. Unsupported topology requires explicit
review; an unchecked topology warning does not mean other repairs can safely apply.
No universal repair, automatic numeric cleanup or silent partial imported drawing.

## Milling execution and attribution

Main Cutting is mandatory on Profile/Pocket; optional Finishing is authored and OFF
by default. ADR-0032 separates retained-for-later stock from material Main leaves
for this operation's Finishing. ADR-0033 adds an explicit Pocket-only nested
`pocket_finishing_v2` contract/scope while retaining the V10 outer envelope.
ALPHA-3.1 proves bounded polygons beyond rectangles; ALPHA-3.2 adds bounded tangent
curves/simple islands and complete axial/surface proof. See [cutting limits](../CUTTING_CONTRACT.md#qualified-geometry-and-explicit-refusals).
No generic finishing, implicit scope upgrade, strategy fallback or optimized rest machining.

CAM owns machine-neutral movement intent. Posts own dialect spelling and budgeted
CPS recording; ShopBot-specific constraints stay out of generic geometry algorithms.
Stage/tool provenance must describe actual emitted intervals/rates where resolved;
planned-only attribution cannot masquerade as decoded execution. Full-program
simulation/export qualification is independent of preview playback.

[SAFETY-FIX-1](../releases/SAFETY_FIX_1_MOTION_GUARDS.md) contains unproven Helix
entry and legacy Profile overcut, plus affected Front holding/Back input combinations.
Thread's intrinsic cutting helix is distinct. Through Front cuts require the bounded
resolved MaterialV1 bridge proof; empty manual flags alone are insufficient. Back
text/approach/pattern/raster and edited two-sided cases can refuse. Historical M4
output/physical observations do not waive these current guard conditions.

## Cutting values, presets and tools

Explicit RPM/feed/plunge/DOC and actual stage/role tools are operation-owned.
Known machine/cutter limits refuse; unknown limits remain disclosed, with mandatory
specialized constraints preserved. Native output generation never calls FabSpeed.

[TESTED_PRESETS](../TESTED_PRESETS.md) owns collection V2, historical V1 inspection,
strict context matching and ALPHA-4's separate selective starting-value copy. Advisory
reuse metadata is not CAM input, a live source link or transferred target attestation.
No stepover reuse. TOOL-META substrate/coating remain descriptive optional strings;
unknown is different from explicitly uncoated. No automatic rate/capability derivation.

## Security and untrusted data

[SEC-HARDEN-1](../releases/SEC_HARDEN_1_APPLICATION_SECURITY.md) owns the detailed
source-bound proof. Production CSP restricts scripts/assets/IPC, allows WASM-specific
compilation and current inline styles, and disables objects/framing/forms/base changes.
Development alone adds loopback Vite/HMR endpoints. There is no general JS unsafe-eval.
Raw filesystem operations have no static home/documents/temp grants. Dynamic native
selection authorizes an individual path; custom import/watch/export commands perform
separate canonical-target checks. Recent/reload paths reacquire missing selection;
saved embedded source paths are not automatic grants. At most 128 source watches.
Preset storage keeps fixed app-data paths, safe IDs, size/CAS/symlink controls.

HTTP defaults to `127.0.0.1:8766`; unauthenticated non-loopback binds refuse.
Allowed local development/own origins and explicit configured origins are enforced
before request bodies. Foreign/null/duplicate Origins refuse; missing Origin supports
CLI, with cross-site Fetch Metadata refusal. Headers are not authentication.
Two heavy jobs, no queue and 429 admission protect generation/import/CPS inspection;
health/version/cancel remain accessible after Origin checks. A 120-second pre-response
deadline signals cancellation; workers retain permits until exit. Body/disconnect
cancellation is cooperative, not universal CPU preemption or a slow-stream timeout.

Imports are capped at 64 MiB. SVG XML/decompression is 16 MiB with 50,000 nodes,
64 nesting levels and 200,000 admitted work/emitted primitive bounds; nested/ambiguous
use and unbudgeted marker expansion refuse. DXF group-pair scan, depth 32, 200,000
expanded entities/primitives and 16 MiB expanded text are bounded; exceeding recursion
fails the entire import. Font prospective subdivisions (65,536/arc) and cumulative
point/token budgets refuse before unbounded iteration. STL retains exact binary
length checks plus 64 MiB / 1,000,000 triangle caps. Conservative refusal does not
imply complete parser memory/latency control or permission against hostile same-user
filesystem races. Installed-native file workflows and remote authentication remain
unqualified; mocked/command/browser tests do not replace them.

## Process and future boundaries

Share source geometry, persistence, history, view transforms and output infrastructure
only where semantics agree. Milling spindle/Z/chip engagement is not waterjet kerf,
pierce/quality or vinyl drag behavior. Do not add dummy milling values or speculative
universal IR for ProtoMAX/WJ/VINYL. Existing process/post restrictions remain.
Future job/machine separation, grouped/nesting workflows, STEP, FabSBP and optional
FabSpeed exchange need their own approved scope/ADRs/qualification. See the
[technical roadmap](ROADMAP.md) and [release gates](../releases/RELEASE_ROADMAP.md).
