# FabCAM — current status

## Current implementation and source

The accepted software line is `feature/sec-harden-1`, documentation HEAD
`6bfecf3339524cc86e8c2bd83f168c63cbdd2860`, software commit
`32b5fbbb6fb2b6a170c0531549fc081e5d0c2c10`. It descends from accepted
SAFETY-FIX-1 and inherits INT-ALPHA-1, ALPHA-1–4 and compatible TOOL-META.
Canonical `FabCAM-v1.1` remains `16eb34fd07354627b3c3c2f48a227b30ba1cc3c2`;
the safety/security feature commits have **not** been merged there. The frozen V1
worktree remains on its separate historical line. Inspect Git before selecting a
build; a branch name or old installer is not evidence of feature freshness.

This reconciliation is isolated on `docs/doc-consolidation-1` from that accepted
security descendant. It changes documentation/guidance only; the maintainer accepted this
checkpoint and authorized three local documentation commits. [Verification and consolidation](../releases/DOC_CONSOLIDATION_1.md).

| Current contract | Implemented boundary |
| --- | --- |
| Persistence / native cutting | ProjectFile **10**, `project.cutting_version: 10`; no implicit V9 migration |
| Transports | Native CLI, loopback HTTP, Tauri IPC and WASM share core validation; `cutting-parameters-v10` and `pocket-finishing-v2` are explicit capabilities; CPS requires a CPS-enabled native build, absent from the default WASM package |
| Drawing / geometry | Persisted Drawing and GEOM1/generation protocol **1**; scoped Analyze/Preview/Apply and their geometry payload **2**; local edit acceptance is not CAM qualification |
| Profile / ordinary Pocket | Required explicit Main Cutting; optional authored Finishing. `main_finishing_v1` retains bounded polygon semantics; Pocket-only `pocket_finishing_v2` has Walls / Floor / Walls and floor scopes |
| Finishing coverage | Software-qualified polygons and bounded Pocket tangent curves/simple islands; direct-entry flat cutters, proof/resource and post restrictions apply; no universal finishing or repair |
| Presets | Collection **V2** for staged data, historical V1 inspection and limited unchanged-family strict matching; ALPHA-4 selectively reuses reviewed starting values without attestation or stepover transfer |
| Tools | TOOL-META substrate/coating are optional descriptive metadata, not cutting calculations |
| Calculation | FabCAM owns explicit RPM/feed/plunge/DOC and validation. FabSpeed is independent; optional one-way integration remains proposed |
| Safety | SAFETY-FIX-1 refuses unproven helical entry, legacy Profile overcut and affected two-sided holding/Back inputs |
| Security | SEC-HARDEN-1 restricts CSP/dynamic file scope, loopback Origin/admission/cancellation and parser expansion; installed-native workflows remain unqualified |
| Locales | Frontend English/German; no complete French UI catalog |

Details belong in [CUTTING_CONTRACT](../CUTTING_CONTRACT.md),
[TESTED_PRESETS](../TESTED_PRESETS.md) and [architecture](../ARCHITECTURE.md), not
another current-state banner. [HISTORY](../releases/HISTORY.md) owns chronology.

## Qualification and outstanding gates

Accepted SEC-HARDEN-1 qualified 1,330 logical software files under fingerprint
`498fe344f19035d8b6e9fc1d874790373c212aa5073f693774a4fa9ac289c6d0`:
20/20 canonical groups, 1,686 Rust/Tauri tests and 1,570 frontend tests passed;
109 fixed CNC comparisons remained exact. One inherited ignored Rust worker and
one skipped frontend case remained under the unchanged canonical suite policy.
[SEC-HARDEN-1 report/receipts](../releases/SEC_HARDEN_1_APPLICATION_SECURITY.md)
and [SAFETY-FIX-1](../releases/SAFETY_FIX_1_MOTION_GUARDS.md) define exact source,
new refusals and limitations. Documentation edits to inventoried Markdown do not
extend that qualification; executable/generated blob preservation is checked separately.

- Installed-native open/save/reload/watch/export acceptance remains unqualified;
  mock/command tests and browser CSP startup do not replace it.
- Authenticated remote HTTP access is unqualified; unauthenticated non-loopback
  binds refuse. Local header admission is not authentication.
- Physical machining, controller/machine acceptance and the wider release remain
  separate, unqualified gates. Historical M4 square evidence is not a blanket claim.
- Restricted Helix, unrestricted two-sided setups, legacy overcut, unsupported
  geometry/finishing combinations and stepover reuse are not requalified.
- No current-source installer/publication or canonical merge is authorized by
  software/documentation acceptance. Frozen V1 gates remain in [V1_READINESS](../releases/V1_READINESS.md).

This documentation checkpoint is accepted for local consolidation only. PERF-1
remains planned and unstarted in the technical roadmap; implementation needs separate
authorization. HARD STOP after consolidation. Future milestones update these sections
**in place**.
