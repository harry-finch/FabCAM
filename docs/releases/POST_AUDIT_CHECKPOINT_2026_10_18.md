# Post-audit consolidation — pipeline58

> 2026-10-18 follow-up: contracts approved; [targeted HATCH/known-stock SAFE-Z fixes](HATCH_SAFE_Z_RESULTS_2026_10_18.md) implemented locally on pipeline59, human gate ivac-th8y2. Full SAFE-Z fixture/transition qualification remains ivac-j09eh; TEST1 not started. Earlier observations below describe pipeline58.

2026-10-18 · **ENGINEERING CHECKPOINT COMPLETE — HUMAN REVIEW PENDING**.
Task ivac-9v68y; human gate **ivac-af5v1 OPEN**. No production algorithm change, commit, push, package, tag or release.
Added a diagnostic example/fixture and documentation; TEST1 implementation remains pending.

## Outcome

Combined regressions pass, but **two reproduced correctness/safety blockers remain**:

1. **ivac-aoany — HATCH in DXF BLOCKS.** A10x10 boundary inserted at(100,200)
   imports at(0,0)..(10,10), not(100,200)..(110,210). Generic recovery warning only.
   The raw recovery scanner has no BLOCK/INSERT expansion context. Recommend a bounded
   fail-closed import diagnostic for block-contained HATCH before V1; top-level HATCH
   recovery remains supported. Do not silently drop the misplaced boundary and call
   the drawing complete. Full transformed block-HATCH support is a separate option,
   not necessary for V1. Detection should include unused definitions/nested inserts
   conservatively until a supported expansion policy is qualified.
2. **ivac-jnom4 — SAFE-Z.** Known stock top18/start18/depth17 with rapid5 generates
   OpenSBP that jogs inside declared stock. Unknown machine work-area yields only the
   generic CPS warning; default envelope warning on positive cuts does not cure this.
   Recommend shared-frame preflight refusal and independent decoded-path checks,
   retaining existing origin/post guards. [Exact contract and cases](SAFE_Z_CONTRACTS.md).

These diagnostic probes expose gaps beyond the green regression corpus. Their successful
execution means reproduction succeeded, **not safety PASS**. No machine was operated.

## Consolidated verification

All runs use local uncommitted pipeline58 on HEAD
`2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b`, version0.9.0-beta.2.
[Manifest, commands and logs](consolidation-2026-10-18-evidence/README.md).

| Layer | Fresh result | Qualification limit |
| --- | --- | --- |
| Core library with CPS | 1,202 PASS, serial | Global cache test isolation unresolved; no parallel-suite claim. |
| Eleven integration suites | 101 reported PASS,100 executed | One optional FANUC oracle absent; includes CAD1, V-Carve, GEOM1, drawing, source UX, tabs, volume, waterline, stabilization, CPS/OpenSBP. |
| Feeds/speeds | 110 integration tests +1 doctest PASS | Software contracts, not validated machining recipes. |
| CPS | 46 reported PASS,40 executed; independent Node/V8 39 PASS | Six optional FANUC oracle early returns are not executed qualification. |
| HTTP / native WASM bridge | 15 /22 PASS | WASM tests run natively, not browser WASM runtime. |
| Native Tauri | 12 PASS, including filesystem watcher | Outside sandbox for FSEvents; not packaged desktop installation. |
| Frontend | 1,328 PASS,1 skip +10 package-state Node tests | Current state/policy regression evidence, not new visual human acceptance. |
| Static checks | Typecheck, ESLint/Prettier, core and adapter Clippy, cargo fmt, diff whitespace PASS | No packages built or redistributed. |

Existing CI39053047404 installers are pipeline54 and do not contain audits09–16.
Native tests cannot transfer qualification to those packages or to Windows/Ubuntu.
No clean-host, Gatekeeper, physical ShopBot or fresh UI visual acceptance is claimed.

## Ticket / roadmap reconciliation

| Item | Current disposition |
| --- | --- |
| Audit review gates | User accepted all audit lots; previous closure records retained. Separate GEOM1/FS/UX gates are not implicitly closed. |
| ivac-13go | Closed as already fixed: current two-sided CPS test asserts explicit refusal and passes. ivac-jtkj remains unsupported future capability. |
| ivac-htc8 / ivac-rt80e | Duplicate consolidated under ivac-rt80e; serial baseline only. TEST1 requires process/scoped cache isolation before cache assertions. |
| ivac-dli7 / ivac-j2po | Already CLOSED: bounded nested Pocket parity / XOR regressions re-pass. No blanket arbitrary-topology qualification. |
| ivac-vt6e / ivac-hs9i / ivac-5g51 | Already CLOSED: text reopen, project-tool identity, complete WASM asset guard corrected previously. Frontend regressions re-pass; actual new-candidate runtime still pending. |
| ivac-57kp | Local curved tab splitting/versioned material width fixed by audits09–12. Extended entry/open/concave/dense topology and physical holding remain, not the obsolete whole-arc-only allegation. |
| ivac-2zll | Mixed rough/finish RPM still explicitly refused; not unsafe successful output. Preserve saved values and declare scope. No new spindleSpeed capability here. |
| ivac-u1ko / ivac-jtkj / ivac-wgovg | Numeric Z translation, two-sided CPS and VINYL deferred; safeguards remain. SAFE-Z does not enable them. |
| ivac-snqn | REL1-A public FabCAM rename already accepted; documented beta-test transition supersedes mandatory migration infrastructure. Cross-platform preservation checks remain. |
| Cancellation ivac-42ylx / ivac-nh6q / ivac-q1nz1 | Cancellation propagation/refusal tests pass; large-workload <=200ms p95 remains unqualified. No response-time guarantee inferred. |

## Approval requested / next bounded checkpoint

Approve [TEST1 contracts](TEST1_CONTRACTS.md) (8properties/256CI cases, independent
oracles, explicit budgets, deterministic test-only infrastructure), and [SAFE-Z](SAFE_Z_CONTRACTS.md).
Recommended order: **bounded HATCH refusal + SAFE-Z correction with red/green regression
evidence → TEST1 implementation/qualification → new candidates with separate authorization**.
Each implementation package ends at human review. A broad GEOM-CORE refactor is not
justified by this checkpoint. If the bounded guards require incompatible source changes
or expanded Z/geometry support, stop and submit that decision.

Remaining V1 gates: separate software/UX reviews; fresh macOS/Windows11/Ubuntu
candidate integrity and clean-host/runtime tests; authorized physical ShopBot A01–A07;
release notes/limits/checksums and explicit RC/release decision. macOS community
ad-hoc/no-notarization policy remains; no Apple credentials or publication.

**HARD STOP — await human validation; no next implementation launched.**
