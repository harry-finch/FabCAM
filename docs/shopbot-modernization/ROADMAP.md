# FabCAM technical workstreams

## Current state

The accepted security descendant inherits ALPHA-1–4 and TOOL-META; canonical V1.1
is a separate, older checkout. [STATUS](STATUS.md) owns source identity, software
qualification and blockers. [RELEASE_ROADMAP](../releases/RELEASE_ROADMAP.md) owns
release gates; [HISTORY](../releases/HISTORY.md) owns chronology. This document
classifies technical work, not release permission. Update this section in place.

| Workstream | Implemented / accepted software scope | Remaining or excluded scope |
| --- | --- | --- |
| M0–M4 ShopBot foundation | Existing CPS runtime, restricted OpenSBP output and source-bound historical evidence | No general controller/physical qualification; preserve attribution and operator boundaries |
| FS-RESET / ADR-0030 | Removed integrated recommendation engine; explicit authored cutting and tested collections | FabSpeed is independent; no automatic rate calculation or active source-pack engine |
| ALPHA-1 | Manual stock/import placement and diagnostic refinements | Imported artwork is not automatically safe manufacturing geometry |
| ALPHA-2 / drawing | Bounded Analyze/Preview/Apply, geometry protocol V2 and fail-closed topology checks | No universal repair, silent cleanup or local-edit-to-CAM certificate |
| ALPHA-3 / ADR-0032 | Project/cutting V10, Main Cutting and optional Finishing, collection V2 | No implicit V9 or V1 staged-preset migration |
| ALPHA-3.1 | Bounded polygon finishing beyond rectangles | Not arbitrary geometry; preserve topology/cutter/output proof limits |
| ALPHA-3.2 / ADR-0033 | Pocket Walls / Floor / Walls and floor, bounded curves/simple islands | No unsupported strategy fallback, non-tangent/complex topology or unrestricted entries |
| ALPHA-4 | Explicit selective starting-value reuse, advisory provenance and atomic history | Strict tested matching unchanged; no stepover/attestation transfer |
| TOOL-META / INT-ALPHA-1 | Descriptive substrate/coating ported onto V10; cross-feature integration accepted | No live rate calculation or catalogue synchronization |
| SAFETY-FIX-1 | Motion containment refusals accepted | Helix, legacy overcut and unrestricted two-sided functional requalification remain separate |
| SEC-HARDEN-1 | CSP/file authorization, loopback admission/cancellation and import bounds accepted | Native installed workflows, authenticated remote access and physical machining unqualified |
| Documentation / agent guidance | Reconciliation accepted; active prose/skills aligned with source | Local documentation consolidation only; no implementation change |
| PERF-1 | **Planned, not started**: measured finishing-proof, DXF-chaining and frontend/WASM optimization against recorded baselines | Separate implementation approval; preserve CNC output, proof/numeric budgets and security guards; no work authorized by this consolidation |
| Optional V1.2 / ADR-0031 | Proposed one-way FabSpeed catalogue direction | INT-0 contract review before implementation; no dependency of CAM generation/export |
| Future CAM / architecture | Existing deferred plans retain their own contracts | Rest machining, expanded entries/registration, STEP/ARCH-MACH/FabSBP require separate approval |
| WJ / VINYL | ProtoMAX/waterjet and drag-knife concepts remain separate | No inferred implementation or qualification from milling infrastructure |

## Acceptance criteria for future changes

Use [CONTRIBUTING](../CONTRIBUTING.md), [current cutting contract](../CUTTING_CONTRACT.md)
and [ADRs](adr/README.md). Identify the actual accepted source, preserve existing
refusals and source-bound evidence, implement only the approved scope, validate
appropriate transports and stop for human review. Architecture approval, software
qualification, native/platform acceptance and physical operation are separate.
Do not restart historical FS0–FS3.9 or treat old pending banners as current work.
