# SEC1 follow-up — V1 scope and deferred security

2026-10-20 · documentation reconciliation `ivac-pknlj`, inspected source **6043bfe**.
[SEC1 defensive report](SEC1_DEFENSIVE_REVIEW_2026_10_20.md), engineering `ivac-btkaq`
CLOSED, human review **`ivac-1ahyt` OPEN**. Closing the audit task does not close
its findings. The maintainer's bundled-only V1 policy below is approved in the
roadmap request; implementation and release acceptance remain separate.

[Release plan](RELEASE_ROADMAP.md) owns priorities;
[V1 readiness](V1_READINESS.md) owns acceptance gates. Existing
[NUMSAFE3 results](NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md) and human gate
`ivac-zulb3` are retained, not reopened or promoted to newer-source qualification.

## Approved V1 CPS policy

**FabCAM V1 supports bundled trusted CPS execution only.** Trust comes from the
application's bundled registry, not an imported script's name, claimed ID or
similarity to a bundled post. The policy must apply before any top-level
inspection/evaluation and before generation across enabled entry points.

Imported, embedded and path-based custom CPS scripts must not be evaluated under
V1. Preserve legacy project configuration and script/property data on load/save;
do not silently delete it, substitute a bundled post or generate with a different
post. Present a clear unsupported-custom-post diagnostic and let the user explicitly
select a supported bundled post. Opening an inspector must not execute the legacy
script. Existing bundled ShopBot support and its qualified subset/refusals remain;
bundled trust does not qualify every machine, operation or transport.

Custom CPS is deferred, not permanently removed. Future machine extensibility
continues through [ARCH-MACH](ARCH_MACH_PLAN.md), without making custom script
execution or the security track a prerequisite for native ProtoMAX OMX V1.5.
[ADR-0001](../shopbot-modernization/adr/ADR-0001-shopbot-cps.md) and
[ADR-0020](../shopbot-modernization/adr/ADR-0020-cooperative-cps-cancellation.md)
retain their accepted history; restricted host APIs and cooperative cancellation
are not aggregate resource confinement. No new sandbox design is selected here.

## SEC1 disposition matrix

Reconciled under explicitly approved V1-FREEZE Option A onto **eb747eb**.
SEC1-01/04/08 are implemented in 5e789e5/b348921/d7b3638, engineering
`ivac-dqlol` CLOSED; [qualification](SEC1_REMEDIATION_RESULTS_2026_10_20.md)
reports 20/20 local gates, 1662 Rust, 1460 frontend and 136 exact outputs.
Human remediation review **ivac-1mqti OPEN**; remote CI/promotion protection,
installed platforms and physical acceptance are not established. Other findings
remain open boundaries/follow-ups. Stable SEC1 IDs and existing tickets are reused.

| Finding / current state and risk | V1 classification and blocking condition | Proposed target / dependencies | Acceptance evidence required |
|---|---|---|---|
| **SEC1-01** High; custom CPS can consume native CPU/heap during inspection or execution; locally corrected; human review pending | **V1 required correction**: blocks while untrusted script evaluation remains reachable; approved bundled-only restriction is the bounded V1 remedy | Implemented SEC1 checkpoint `ivac-dqlol` CLOSED; review `ivac-1mqti` OPEN; registry identity, all enabled inspection/generation routes and legacy persistence; `ivac-1ahyt` | Negative tests for imported/embedded/path sources before evaluation, spoofed bundled IDs, legacy round-trip and explicit refusal; bundled ShopBot/output regressions and cancel/retry. Exact corrective commit and human review required |
| **SEC1-02** Medium; read/watch/write grants and custom IPC authorization differ; open | **Important nonblocking hardening** under reviewed local V1 assumptions; blocks affected support if unauthorized native access or ineffective required denies is demonstrated | Post-V1 desktop authority track, coordinated with SEC1-07; IPC inventory and approved workflow/grant contract | Unified read/watch/write authorization, custom-command ACLs, scoped grants and denies; dialogue/recents/reload/window-origin positive and negative tests on macOS/Windows/Ubuntu |
| **SEC1-03** Conditional Medium; parent identity can race; explicit NUMSAFE3 limitation | **Nonblocking within user-controlled directories**; blocks any hostile-directory guarantee until separately qualified | Longer-term filesystem architecture only if hostile-directory support is needed; retain NUMSAFE3 export contract and platform review | Document exclusion now; future handle-relative operations/identity checks considered by architecture review, with platform race/failure tests before expanding guarantees; more canonicalization alone is insufficient |
| **SEC1-04** Medium; cleanup error can misreport a completed publication; locally corrected; human review pending | **V1 required bounded correction before refreshed candidate**: inaccurate published/non-published diagnostics must not be accepted | Implemented SEC1 checkpoint `ivac-dqlol` CLOSED; review `ivac-1mqti` OPEN; native exporter/CLI and NUMSAFE3 per-file/new-only pair contract; `ivac-zulb3` remains independent | Controlled cleanup/publication failure tests distinguish not published, published with cleanup warning and per-face partial state; preserve old targets, no blind destructive rollback, no pair-atomic claim |
| **SEC1-05** Medium; successful byte streaming can be mistaken for complete CAM qualification; open | **V1 acceptance/claim boundary**: any supported route advertised as fully qualified must prove it or explicitly disclose/refuse that claim; no automatic ShopBot streaming expansion | Existing export/qualification contract (human review `ivac-3i00w` stays independent), NUMSAFE3 limits; fuller streaming equivalence is post-V1 and needs separate scope | User/API documentation and route tests distinguish complete bytes from full simulation/envelope/stock/provenance checks, and final errors from already-sent HTTP/stdout prefixes; full equivalence requires an independently approved validation strategy |
| **SEC1-06** Medium local, High before untrusted/shared clients; admission/disconnect cleanup incomplete | **Nonblocking hardening for bounded trusted local use** unless reproduced exhaustion prevents supported V1 work; **blocking before untrusted/network-shared support** | Post-V1 service resource track, shared transport cancellation where relevant; NUMSAFE3 budgets and `ivac-q1nz1` attribution are inputs, not substitutes | Admission limits, bounded concurrency, aggregate resource policy, disconnect cancellation and cleanup on error/panic; controlled load/cancel/retry tests. Network support additionally needs explicit authentication/ownership/Host/origin contract; loopback/CORS is not authentication |
| **SEC1-07** Conditional Medium; absent production CSP and broad desktop authority; no XSS chain demonstrated | **V1 platform acceptance and nonblocking hardening**: do not infer installed security from CI; an exploitable boundary violation blocks affected support | Post-V1 desktop authority/CSP track with SEC1-02; production resource/navigation inventory and existing clean-platform gates | Production CSP and minimum necessary filesystem permissions proposed after inventory; webview navigation policy; installed three-OS workers/WASM/preview/import/save/recents/reload qualification before claiming a hardened policy |
| **SEC1-08** Medium/P1 process; false-green or mismatched release gates; locally corrected; human review pending | **V1 required publication gate**: no promotion/publication without verifiable software evidence for exact candidate SHA; does not replace installed/physical gates | Implemented SEC1 checkpoint `ivac-dqlol` CLOSED; review `ivac-1mqti` OPEN; reuse `ivac-5yxyu` portable-size correction included in SEC1 (tracker disposition separate), schema/version/build tools and candidate receipt | Strict release checks or equivalent fail-closed evidence: missing prerequisites, codegen failures and invalid size measurements tested; correct portable sizes; CI/software receipt tied to exact release SHA and reviewed before promotion |
| **SEC1-09** Low; floating tools/actions and policy/documentation drift; open | **Important nonblocking maintenance**; missing identity/integrity evidence or a verified relevant vulnerability can block a candidate; no vulnerability inferred from age alone | Post-V1 reproducibility/maintenance track; existing signing/community distribution policy, dependency locks and candidate traceability | Prioritized dependency/toolchain upkeep, GitHub Actions revision pinning, actual build minima, signing/docs consistency and artifact/source/tool hashes; qualification of upgrades and fresh advisory evidence when making vulnerability claims |

V1 remains qualified ShopBot milling, V1.5 native ProtoMAX OMX, V2+ advanced CAM
and extensibility. Deferred security work has its own post-V1 track, **not an
implicit V1.5 dependency**. A newly confirmed supported-scope defect is triaged by
risk, not hidden by a deferred label. Existing human, clean-platform and physical
gates stay visible in V1 readiness; none is closed by this plan.

## Future custom CPS confinement milestone

**PLANNED / NOT STARTED — post-V1 security capability; EXTEND the existing CPS
boundary, DEFER custom execution until qualified.** Coordinate with existing
`ivac-q1nz1` non-preemptible-phase analysis; this is not a duplicate cancellation
ticket or a restart of accepted cooperative work.

Goal: safe inspection before generation and bounded execution of explicitly
untrusted custom posts, separate from bundled trusted code. Include interruptible
execution isolation; aggregate CPU/instruction or time budgets; heap/memory limits;
script-size, output and diagnostic budgets; recursion and cumulative-work limits;
cancellation, cleanup, failure attribution and recovery. Cover top-level inspection
and every callback, not only loop iterations or a timeout around a running worker.

Architecture review must select the confinement boundary and budget ownership
across transports, persistence, cache and packaging. Do not prescribe a sandbox,
VM replacement or subprocess design here. No geometry-engine redesign or new
ProtoMAX dependency follows from this milestone.

Dependencies: approved trust/threat model and resource ceilings, phase attribution,
compatibility inventory, platform/packaging review and separate implementation
authorization. Acceptance: bounded low-threshold adversarial tests (aggregate work,
recursion, allocation, emission/diagnostic growth), interrupted inspection/generation,
no partial success/cache pollution, recovery/retry, legacy preservation, legitimate
custom-post compatibility and unchanged qualified bundled output. Require exact
source receipts and supported-platform regression evidence, then human approval
before advertising custom CPS again. No universal latency or memory claim is made now.

## Independent UX and data follow-ups

These remain outside SEC1 and do not become V1 security blockers:

- Tool illustration: reuse the [existing planned stylized inspector refinement](RELEASE_ROADMAP.md#tool-inspector--stylized-illustration-and-dimensional-guidance)
  and documentation checkpoint `ivac-rhq07` (closed for documentation only); its
  brand-consistent families and bidirectional dimension highlights remain planned.
- **Planned post-V1.5:** different surface/core stock colors in the 3D preview,
  consistent with semantic colors and accessible contrast. Acceptance: recognizable
  original surface versus exposed core without implying simulated removal accuracy
  or changing stock/CAM semantics. No matching existing roadmap item was found.
- **Planned post-V1.5:** user-facing **Waterline → 3D Roughing** and **Relief → 3D
  Finishing**, with consistent localized labels/help and discoverable legacy terms.
  Acceptance: users can identify existing operations without changing serialized
  operation IDs, algorithms, capabilities or qualification. No matching existing
  terminology item was found; do not reopen implemented Waterline engineering.
- Source packs: the [user-oriented authoring/import guide](../feeds-speeds/CUSTOM_DATA_SOURCES.md)
  **already exists**; reuse human review **`ivac-7w1e1` OPEN**, not a new writing
  milestone. Review usability and the actual external-JSON/import workflow; do not
  promise an in-app generic authoring wizard or broaden data evidence claims.

## Evidence dependency and handoff

The SEC1 report was inspected unchanged in the primary checkout; SHA-256
`028d3a1fe86de186a552786f1918cce0685ebcdb55ceaa682f6ade24a701d42c`.
It and its evidence are local/untracked at this checkpoint and remain owned by
the audit workflow. Integrating that unchanged report/evidence is a prerequisite
to publishing these links; this documentation commit does not absorb another
checkpoint's uncommitted work. The audit snapshot remains unchanged; current implementation evidence is linked above.

Next human checkpoint: review the qualified SEC1-01/04/08 corrective commits/results,
review dispositions under `ivac-1ahyt`, then authorize matching candidates only
when the remaining V1 gates permit. Confinement architecture, desktop hardening
contract and any network/hostile-directory support require separate scope decisions.
**HARD STOP — documentation only; no implementation, merge, push or release.**
