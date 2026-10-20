# Status — 2026-10-18

> 2026-10-20 — **NUMSAFE2 pass limits (ivac-o9gwj)**: [results](../releases/NUMSAFE2_PASS_LIMITS_RESULTS_2026_10_20.md). 1000 depth passes per operation (maintainer decision; same as planar surfacing) for the shared Z schedule, drill pecks per hole, half-pipe and V-Carve levels — refused (`PassLimit`) before any motion, never resampled. Explicitly supplied invalid steps (0, positive, non-finite, < 1e-9 mm) are refused (`InvalidStep`) instead of becoming a full-depth pass; omitted steps keep the documented default. 136 valid programs byte-identical. Local commit only, not pushed. **HARD STOP.**

> 2026-10-20 — **NUMSAFE numerical/runtime safety (ivac-knphk), human review pending**: [results](../releases/NUMERICAL_RUNTIME_SAFETY_RESULTS_2026_10_20.md). Non-finite output values, zero/missing cutting feeds (LinuxCNC, GRBL, `.cps` route) and non-finite motion parameters now refuse the whole program instead of emitting `Z0` or inheriting a modal F; STL surface grid capped at 16 M cells; WASM worker recycled after a trap (3-strike bound). 136 valid programs byte-identical to `94cdd6b`; ShopBot audit-only, no OpenSBP change. Import checkpoints committed locally as `d30863d` and `94cdd6b`; NUMSAFE validated and committed locally as `e1a0c95` (none pushed).

> 2026-10-20 — **Import units & warnings UX (ADR-0026, ivac-feder), human acceptance pending**: [results](../releases/IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md). Import summary before commit for assumed units or importer warnings: grouped warnings, detected units, live size in mm, explicit choice (1 mm, 96/72 dpi, inch, custom) persisted as `unit_status = explicit` through save/reopen/reload; physical/declared units never reconverted; legacy sources read `unknown`. Sidebar badge reopens the summary. vitest 1452/0, browser smoke with screenshots. No commit/push. **HARD STOP.**

> 2026-10-20 — **Import correctness (ivac-feder), human review pending**: [results](../releases/IMPORT_CORRECTNESS_RESULTS_2026_10_20.md). DXF OCS ±Z, BLOCK base points and INSERT extrusion now follow DXF semantics; tilted normals and MINSERT refused; unit-ambiguity warnings added. 38/38 analytic cases (13/38 before), 53 real files geometry-identical, workspace 1604/0. Units policy and visible import warnings proposed in [ADR-0026](adr/ADR-0026-explicit-import-units.md). No commit/push. **HARD STOP.**

> 2026-10-19 — **Software release-gate script green (maintenance, uncommitted)**: [closure report](../../V1_RELEASE_GATE_CLOSURE_2026_10_19.md). `scripts/pre-release.sh` exits 0 with all 16 gates (Node 24, pnpm 10.34.7, Rust 1.88, cargo-deny 0.20.2, headless Chrome; Rust 1609/0/1 ignored, frontend 1438 + 1 skip). Four documented *unmaintained* advisory exceptions (no vulnerabilities); UI smoke rewritten for the Atelier shell; README parity restored. This is a software gate only: native installers, platform acceptance, human reviews (e.g. ivac-3i00w, ivac-b0js3) and physical ShopBot qualification remain open, and nothing is authorized for publication. Supersedes the earlier 'README parity red; cargo-deny unavailable' remark. No CAM/post/output/schema change.

> 2026-10-19 — **G-code comment safety and release gate (ivac-4ql18)**: [results](../releases/GCODE_COMMENT_SAFETY_RELEASE_GATE_2026_10_19.md). User text in native comments can no longer close/nest a comment, add a line or form a LinuxCNC active comment; `comments: false` wording clarified; pedantic pre-release gate restored without behaviour change. Workspace 1601/0, CI all-features 1596/0, TEST1 256/256 and 1824/1824, frontend green. README parity (pre-existing) red; cargo-deny unavailable. Committed work does not close ivac-3i00w, ivac-b0js3, ivac-b0js3.1 or other open human gates.

> 2026-10-19 — **ADR-0025 follow-up (ivac-b0js3.1), human review pending**: [results](../releases/ADR0025_FOLLOWUP_RESULTS_2026_10_19.md). Confirmed and corrected two provenance false negatives (ShopBot imperial load diameters; tool names read as G-code words), explicit planned-only reasons, ADR verification scope clarified. Seven red tests triaged (metadata-only, outdated ADR-0023 expectation, parallel cache interference, example lint); `cargo test --workspace` green in parallel. No output-byte, guard, export-policy or schema change; no commit/push/publication. **HARD STOP before the next correction campaign.**

> 2026-10-19 — **ADR-0025 implemented locally, human review pending**: [movement-tool provenance results](../releases/MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md). Pipeline63; immutable per-move tool bindings, independent native intent manifest/cache replay, bounded ShopBot emitted-intent validation, shared playback/full-program resolver. Existing post guards, manual assumptions and advanced two-sided exclusions retained. No performance optimization, commit, push or publication. **HARD STOP before any next checkpoint.**


> 2026-10-19 — **ivac-b0js3 Phase A complete, architecture PROPOSED**: [independent multi-tool provenance review](../releases/MULTI_TOOL_PROVENANCE_PHASE_A_2026_10_19.md) and [ADR-0025](adr/ADR-0025-movement-tool-provenance.md). Confirmed identity/lookup limits; cache-only-index and invented-dowel-tool claims refuted. 37 frontend +5 Rust controls PASS; read-only counterexamples recorded. Documentation/evidence only; production, schemas, output and qualification fixtures unchanged. **HARD STOP for architecture approval**, no implementation/commit/publication. Earlier review gates remain separate.

> 2026-10-19 — [ADR-0024 P0/P1 results](../releases/UX_SAFETY_LOAD_RESULTS_2026_10_19.md): background full-program verification and actual export guards implemented; full-project restore no longer inherits previous optional machine data. Frontend1429 PASS/1 existing SKIP +10 Node, check/lint/build PASS; SAFE-Z/post targeted regressions and TEST1 256/256 PASS; browser Home/report/clean export/CPS roundtrip verified. Human review **ivac-3i00w OPEN**, coverage limits **ivac-b0js3 OPEN**. UX D/consolidation gates separate; pipeline61 installers unchanged. No commit/push/publication. **HARD STOP before candidates.**

> Follow-up — 2026-10-19: [Export/load Stage A](../releases/UX_SAFETY_LOAD_STAGE_A_2026_10_19.md) reproduces P0 ivac-3xqtb and P1 ivac-52tpd (3 red cases, 2 controls; 47 existing focused tests PASS). No production correction yet. [ADR-0024](adr/ADR-0024-full-program-simulation-qualification.md) PROPOSED: independent background full-program verifier versus manual completed-3D qualification. HARD STOP at requested architecture gate; B–D await decision. Candidates blocked; no commit/push/build/publication. UX D and consolidation gates remain distinct.

> 2026-10-19 — **Consolidation UX A–D terminée, candidats BLOQUÉS** (beta.3/pipeline62). [Bilan et preuves](../releases/UX_V1_CONSOLIDATION_2026_10_19.md). Core1204, frontend1387/1 SKIP +10 Node, intégration59 et TEST1 optimisé256/256 PASS. Un correctif clavier local ; défaut sécurité lecture/export **ivac-3xqtb** et réserve chargement/format **ivac-52tpd** soumis sans correction. Revues **ivac-amibd / ivac-he4lt OPEN**. Candidats existants beta.3/pipeline61 inchangés. Aucun commit/push/publication ni nouveau candidat. **HARD STOP pour validation.** Les notices antérieures sont historiques.

> 2026-10-19 — **Lot D UX livré localement**, beta.3/pipeline62 : vues stables, diagnostics lisibles, réglages consolidés et fenêtres recalées. [Résultats et limites](../releases/UX_V1_LOT_D_2026_10_19.md). A/B/C acceptés ; revue D **ivac-he4lt OPEN**. Frontend1387 PASS/1 SKIP +10 Node, check/lint/build PASS. Aucun commit/push/publication. **HARD STOP** ; consolidation et candidats ultérieurs non lancés.

> 2026-10-19 — **Lot C UX livré localement**, beta.3/pipeline62 : origine XY, Frame réversible, matériau en brouillon, compatibilité texte et fiche atelier fidèle aux résultats courants. [Résultats et limites](../releases/UX_V1_LOT_C_2026_10_19.md). Lot B accepté (`ivac-ex4v8` fermé). Revue C **ivac-9pljy OPEN**. Tests frontend 1383 PASS/1 SKIP +10 Node ; check/lint/build PASS. Aucun commit/push/publication. **HARD STOP avant lot D**, non commencé. Les notices antérieures sont historiques.

> 2026-10-19 — **Lot B UX livré localement**, beta.3/pipeline62 : copies projet/locales explicites, comparaison type/diamètre avant application, accès contextuel et schéma dimensionnel. [Résultats et limites](../releases/UX_V1_LOT_B_2026_10_19.md). Lot A accepté (`ivac-lec7h` fermé). Revue B **ivac-ex4v8 OPEN** ; réserve fenêtre redimensionnée `ivac-ntg7i`. Aucun commit/push/publication. **HARD STOP avant lot C**.

> 2026-10-19 — **Lot A UX livré localement**, beta.3/pipeline62 : états de génération fiables, refus CPS Homing/Probe, inspecteurs par consommateurs, menu/contextes et copie sécurité. [Résultats et preuves](../releases/UX_V1_LOT_A_2026_10_19.md). Plan Phase0 accepté (`ivac-2tndr` fermé) ; revue du lot A **ivac-lec7h OPEN**. Aucun commit/push/publication ; candidats existants inchangés. **HARD STOP avant lot B**.

> 2026-10-19 — [V1 UX audit Phase0](../releases/UX_V1_POLISH_PHASE0_2026_10_19.md) complete: full audit01–62 consolidated against beta.3/pipeline61 source. Four batches proposed, not started; consumer/capability corrections and separate CPS/envelope/fonts decisions documented. Existing beta.3 candidate evidence retained. Human approval required before implementation. No production change, commit/push/publication. **HARD STOP**.

> User approval and candidate campaign — 2026-10-18: all delivered pending milestones accepted explicitly; [decision and delivery record](../releases/ACCEPTANCE_CANDIDATES_2026_10_18.md). Source advances to **0.9.0-beta.3**, pipeline61, with commit/push and fresh macOS/Windows/Ubuntu builds authorized. Refreshed root validation kit and native app screen captures requested. Prior review OPEN headers are historical; unperformed platform/physical measurements are not fabricated. No tag/Release. [Delivery complete](../releases/beta3-delivery-2026-10-18/README.md): three builds PASS at db5bde5, verified installers/app and refreshed root kit,62native screenshots with gallery. macOS open/generate/preview/export smoke PASS; manual platform/physical records remain actual tests to execute.

> Current checkpoint — 2026-10-18: [Final SAFE-Z / TEST1 consolidation](../releases/FINAL_CONSOLIDATION_2026_10_18.md), pipeline61. Fresh29 focused checks PASS; preserved20,167-move budget failure reproduced. Source/evidence integrity checked; no production change. Human review **ivac-l6ouh OPEN**; ADR UX review ivac-8371y separate. Resource/cancellation reserves conditional; platform/physical gates unchanged. Next after approval: matching candidates with separate commit/push/workflow authorization. No commit/push/build/publication. **HARD STOP**.

Earlier checkpoint notices below are historical snapshots; their OPEN/not-started wording is superseded by the current report and tracker.

> Current checkpoint — 2026-10-18: [ADR-0023 publication guard](../releases/ADR0023_PUBLICATION_GUARD_RESULTS_2026_10_18.md), pipeline61. Mandatory collision refusal with inspection-only preview, pre-byte fixture-stream refusal and explicit incomplete-body diagnostics implemented. Human review **ivac-8371y OPEN**. Previous ivac-6cske accepted. Full SAFE-Z/TEST1 and platform/physical gates remain open. No commit/push/publication. **HARD STOP**.

> Previous checkpoint — 2026-10-18: [SAFE-Z extended findings and targeted fixes](../releases/SAFE_Z_EXTENDED_RESULTS_2026_10_18.md), pipeline60. Sloped holder collision detection and unsupported ShopBot home parking corrected. TEST1 bounded256/256 and regressions PASS. Full SAFE-Z/TEST1 remains OPEN: collision publication parity and incomplete-body status require [ADR-0023 approval](adr/ADR-0023-milling-program-publication-guard.md). Human review **ivac-6cske OPEN**; TEST1 bounded review ivac-930yx accepted. No commit/push/publication. **HARD STOP**.

> Previous checkpoint — 2026-10-18: [TEST1 bounded harness](../releases/TEST1_RESULTS_2026_10_18.md) implemented; **256/256 bounded cases PASS in26.38s**, human review **ivac-930yx OPEN**. Larger-domain move-budget failures preserved (ivac-53ky9); no threshold relaxation. Core1204, frontend1360+10, HATCH/SAFE-Z14, V-Carve10, Halfpipe16, native WASM23 PASS. TEST1 overall remains OPEN for full D2/SAFE-Z (ivac-j09eh); remote CI/browser/platform/physical gates unchanged. Previous HATCH/SAFE-Z review ivac-th8y2 accepted. No commit/push/publication. **HARD STOP**.

> Previous checkpoint — 2026-10-18: [Targeted HATCH + known-stock SAFE-Z corrections](../releases/HATCH_SAFE_Z_RESULTS_2026_10_18.md), pipeline59, local software review **ivac-th8y2 OPEN**. Block HATCH imports now refuse explicitly; invalid stock clearance refuses before cached/streamed output, with structured HTTP error. Source/save compatibility and existing post guards retained. Full fixture/transition SAFE-Z qualification remains ivac-j09eh. TEST1 not started. Existing installers pipeline54; no commit/push/publication. **HARD STOP**.

> Previous checkpoint — 2026-10-18: [Post-audit consolidation and TEST1/SAFE-Z contracts](../releases/POST_AUDIT_CHECKPOINT_2026_10_18.md) complete, human review **ivac-af5v1 OPEN**. Pipeline58 combined tests PASS with documented optional FANUC skips. Two reproduced blockers: HATCH BLOCK/INSERT placement (ivac-aoany) and stock-top/rapid clearance (ivac-jnom4). Diagnostic example only; no production algorithm changes. Approve bounded corrections and contracts before implementation. Existing packages pipeline54; platform/physical and separate UX gates unchanged. No commit/push/publication. **HARD STOP**.

> Audit acceptance — 2026-10-18: user explicitly accepted audit lots 15–16 and preceding lots. Audit human-review gates ivac-2aow8, ivac-97zve, ivac-80ewz, ivac-7y9ft, ivac-rnf0l and ivac-v103z are CLOSED. Historical checkpoint notes below retain their original status. This accepts bounded software corrections, not platform/runtime, physical, separate GEOM1/FS UX or V1 release qualification. Source pipeline58; existing installers pipeline54. Next: reconcile remaining correctness risks and establish the combined baseline, then approve bounded TEST1 contracts and SAFE-Z assessment. No new implementation, commit, push or publication authorized by this acceptance. [Remaining V1 gates](../releases/V1_REMAINING_2026_10_18.md).

> Previous checkpoint — 2026-10-18: [Transport, CPS and desktop audits 15–16](TRANSPORT_DESKTOP_AUDIT_2026_10_18.md) implemented locally. HTTP path guards, disk-free imports, blocking-worker dispatch, CPS EOF/source mapping/output buffer, structured drawing errors and resilient source watching corrected. Sorbonne revision allegation not reproduced (16-digit validation already enforced); WASM cache covered by lot13–14. Pipeline58; 1,202 core, 15 HTTP, 12 native Tauri, 22 WASM tests PASS; CPS 40 executed + 6 optional FANUC absent, Node/V8 39 PASS; integration 40 executed + 1 optional FANUC absent; Clippy/fmt PASS. Human review **ivac-2aow8 OPEN**. Existing installers pipeline54; physical/platform gates unchanged. No commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-18: [Simulation and pipeline cache audits 13–14](SIMULATION_CACHE_AUDIT_2026_10_18.md) implemented locally. Finite form-tool arc removal, arc-aware collision checks, STL sample registration, effective cutter radius, diagnostic/operation cache context, streaming guards and timing corrected. Pipeline57; **1,202 serial core tests PASS**, 15 focused profiling tests PASS; **101 integration cases reported PASS (100 executed; optional FANUC absent)**; Clippy/fmt/WASM compilation PASS. Human review **ivac-97zve OPEN**. Existing installers remain pipeline54; physical/platform gates unchanged. No commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-18: [Holding, preview and contract audits 11–12](HOLDING_PREVIEW_CONTRACT_AUDIT_2026_10_18.md) implemented locally. Versioned material-width tabs with explicit undoable conversion; legacy projects preserved. Arc splitting, preview/token parsing, drill-angle fallback, strict strategies and schema conflicts corrected. Pipeline56; 1,188 serial core tests PASS; 92 integration cases reported PASS (91 executed; optional FANUC absent); 1,328 frontend tests PASS, 1 skipped; typecheck/lint/Clippy/fmt PASS. Browser conversion/Undo reviewed. Human review **ivac-80ewz OPEN**; drag knife explicitly deferred to VINYL **ivac-wgovg OPEN**. Existing installers remain pipeline54; physical/platform gates unchanged. No commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-18: [Motion and driver audits 09–10](MOTION_DRIVERS_AUDIT_2026_10_18.md) implemented locally: entry/plunge feeds, cell-centred waterline skin, selected-bore thread entries/feeds, analytic chamfers, directed open passes/tabs/exits, arc-aware entry geometry, CPS tool feeds, continuous chord fit error, distant-start ordering and inch probing. Integration also exposed and fixed rectangular-tab punctures at arc seams. Pipeline55; 1,174 serial core tests PASS; 87 integration cases reported PASS (86 executed; optional FANUC absent); Clippy/fmt PASS. Cancellation propagation added, ≤200 ms p95 qualification OPEN (`ivac-42ylx`). Human review **ivac-7y9ft OPEN**. Existing desktop candidates remain pipeline54. Software only; no commit/push/package/publication. HARD STOP.

> Candidate validation campaign — 2026-10-17: user requested three builds and a root test kit. [Run39053047404](https://github.com/harry-finch/FabCAM/actions/runs/39053047404) launched at source `2a3fe56` (beta.2, pipeline54), macOS arm64 / Windows x64 / Ubuntu amd64. [Exact protocol and kit](../releases/v1-validation-2026-10-17/README.md): 35 SVG/DXF imports verified, 25 software checks per platform, 4 human reviews and 7 physical-campaign records. All three builds PASS; DMG/NSIS/DEB collected with matching clean source SHA and verified checksums, plus verified FabCAM.app at kit root. Manual result records remain NOT TESTED; no runtime or physical acceptance claimed. No tag/Release.

> Source publication authorization — 2026-10-17: user explicitly authorized committing and pushing all pending work, including audit lots 4–8, their evidence, the AUDIT3 roadmap plan and cleanup references. This supersedes publication holds for this source checkpoint only. Public version remains **0.9.0-beta.2**, pipeline 54. Human/physical/platform gates remain open; no tag, Release or workflow dispatch.

> Previous checkpoint — 2026-10-17: [Pocket/CPS audit, lot 8](POCKET_CPS_AUDIT_2026_10_17.md) implemented locally: qualified Boolean holding tabs with explicit refusal for changed contours, effective CPS tool geometry/frame preview, analytic Arc-chain drills and corner-aware uniform auto-tab phase. Open-stroke island loss not reproduced; finish RPM already reaches an explicit unsupported-record refusal. Pipeline 54. 1,157 serial core tests PASS; 87 integration cases reported PASS (86 executed; optional FANUC absent); frontend 1,324 PASS/1 skip plus 10 guard tests, check/lint/Clippy PASS. Software only; human review **ivac-v103z OPEN**. No package, commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-17: [Pipeline guards audit, lot 7](PIPELINE_GUARDS_AUDIT_2026_10_17.md) implemented locally: conservative fixture travel including ID-zero ambiguity, explicit unchanged-feed RPM diagnostics, arc/surface two-sided bounds and restricted native dowels/heightfields, exact compact remaining-material keys. Bundled ShopBot MS subset confirmed rather than widened. Pipeline 53. 1,145 serial core tests PASS; 87 integration cases reported PASS (86 executed; optional FANUC absent); frontend 1,324 PASS/1 skip, check/lint/Clippy PASS. Software only; human review **ivac-rnf0l OPEN**. No package, commit/push/publication. HARD STOP.

> Historical source publication — 2026-10-17: user authorized commit/push of all pending
> changes. Current source version is **0.9.0-beta.2**; existing beta.1 installers
> and validation records retain their original versions. Earlier publication
> restrictions are superseded for this source checkpoint only; human/physical
> review gates remain open. No workflow dispatch, tag or Release.

> Previous checkpoint — 2026-10-17: [Input audit, sixth lot](INPUT_AUDIT_2026_10_17.md) implemented locally: HATCH metadata/layers, bounded invalid NURBS degree handling, affine DXF curves, 0.01 mm SVG flattening, implicit/relative/closed SVG font paths and bounded HATCH parsing. Pipeline 52. 1,138 core tests PASS serial; 84 integration tests reported PASS (optional FANUC absent). Separate block-HATCH follow-up `ivac-aoany` remains open. Software only; human review pending. No commit/push/package/publication. HARD STOP.

> Previous checkpoint — 2026-10-17: [Drawing audit, fifth lot](DRAWING_AUDIT_2026_10_17.md) implemented locally: line/arc topology, per-step simplification containment including drill points, exact closure diagnostics, snap-collapse refusal and indexed layer selection. Transform order confirmed unchanged. Pipeline 51. 1,123 core tests PASS serial; 84 integration tests reported PASS (optional FANUC absent); 11 frontend transform tests PASS. Software only; human review pending. No commit/push/package/publication. HARD STOP.

> Previous checkpoint — 2026-10-17: [CAM offsets audit, fourth lot](CAM_OFFSETS_AUDIT_2026_10_17.md) implemented locally: geometric raster erosion, arc-aware winding, island context, interval-tested bridges, precise rounded offsets, checked waterline topology and projected entries. Pipeline 50. 1,123 core tests PASS in serial; 71 integration tests reported PASS (optional FANUC oracle absent). Human review pending; software only. No commit/push/package/publication. HARD STOP.

> Previous checkpoint — 2026-10-17: [CAM clearance audit](CAM_CLEARANCE_AUDIT_2026_10_17.md) implemented locally: checked V-Carve inputs, stable flat-arc preparation, exact redundant-knot removal, bounded spatial proof, operation-scoped trochoidal diagnostics, cached tabs and drop-cutter pruning. Pipeline version 49. 1,108 core tests PASS in serial; 71 integration tests PASS. Safety budgets retained; human review pending. No commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-17: [CAM follow-up](CAM_FOLLOWUP_2026_10_17.md) implemented locally: bulge-based arc probes, isolated drill points, linear prepend accumulation, Halfpipe witness fix, exact geometry-cache keys, cached helix fitting and arc-aware frames. Pipeline version 48. 1,098 core tests PASS in serial; 71 integration tests PASS. Human review pending; existing release gates unchanged. No commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-17: [CAM audit corrections](CAM_AUDIT_2026_10_17.md) implemented locally: honest plunge limit, exact imported provenance, arc-aware bounds, area centroid, bounded angle normalization, relief optimization and milling-only Stufenfase. 1,091 core tests PASS in serial and 71 integration tests PASS. Parallel cache-test isolation tracked separately (`ivac-rt80e`). Obtuse T-bones retained with analytical evidence. Human review pending; no commit/push/publication. HARD STOP.

> Previous checkpoint — 2026-10-16: [FS audit improvements](feeds-speeds/FS_AUDIT_IMPROVEMENTS.md) implemented locally: RPM/feed coupling, diagnostics, continuous diameter adaptation and deterministic matching/validation. Follow-up taxonomy/plunge/classification/RPM-floor fixes included: 111 engine + 17 CAM tests and frontend typecheck PASS. Engine policy `fs1-policy-2`; saved values preserved. Software tests/build PASS; human review pending. REL1-B1 clean-host gate and Windows/Ubuntu plans unchanged. No commit/push/publication. HARD STOP.

> Release checkpoint — 2026-10-16: [REL1-B1 local packaging preparation](../releases/v1-stabilization-campaign/REL1_B1_RESULTS.md) complete: arm64 community DMG, strict ad-hoc/identity/integrity and SHA-256 PASS; workflows linted, no dispatch. Clean-download/install/runtime on the other Mac NOT TESTED and still V1-blocking. Developer ID/notarization remain optional POST-V1. Windows/Ubuntu Monday plans unchanged. HARD STOP for review; no commit/push/publication or transmission.

> Historical checkpoint — 2026-10-16: Phase 0 and simple beta-installation transition approved. [REL1-A implemented locally](../releases/v1-stabilization-campaign/REL1_A.md): FabCAM public package names, stable data identity, strict ad-hoc signature and native reopen PASS on macOS. No publication. Human review gate before UX7, then UX6-C; distribution remains UNQUALIFIED.

AUDIT3 Git history/defect-provenance retrospective is **PLANNED — NOT STARTED**
([plan](../releases/AUDIT3_PLAN.md)): after explicit acceptance of validated/frozen
V1, before main V1.5 ProtoMAX implementation. Not a V1 blocker; current audit,
installer and human/physical gates remain unchanged. No archaeology or issue/tag/
release/publication action authorized by this documentation-only request.

Post-audit consolidation is **IMPLEMENTED, awaiting bounded human acceptance** (ivac-l6ouh): [TEST1 bounded V1 requirement](../releases/TEST1_PLAN.md)
and [release-bounded backlog](../releases/POST_AUDIT_CONSOLIDATION.md). TEST1 bounded evidence and final Z01–Z08 matrix are available; pre-V1 GEOM-CORE only for
confirmed defects/demonstrated risks. Broader diagnostics/types/parallelism/calibration
remain later. Existing platform/physical gates and AUDIT3→ProtoMAX sequence retained;
no review closed or implementation launched by this roadmap update.

Release refinement is **PLANNED**, no implementation: [ShopBot SAFE-Z/UX8/TAB-SAFE/OP-ORDER](../releases/SHOPBOT_WORKSHOP_ADDITIONS.md),
[OMX1](../releases/OMX1_PLAN.md), [WJ1–WJ4](../releases/FABCAM_V1_5_SCOPE.md),
[REST1/HELIX1/VCARVE2](../releases/ADVANCED_CAM_PLAN.md). V1.5 now requires native
OMX to MAKE without LAYOUT preparation; format/compatibility remain unverified.
Current audit reviews/TEST1 and platform/physical gates, package identities and
AUDIT3 placement unchanged. No writer/research, machine test or next work starts now.

ARCH-MACH is **PLANNED**, V1.5 architectural preparation before production native
OMX writer design: [contract/initial inventory](../releases/ARCH_MACH_PLAN.md),
[developer workflow](../development/ADDING_A_MACHINE.md). No V1 gate added absent
real supported-ShopBot risk; current audit reviews and TEST1/release priorities stay.
Initial docs do not validate machine support; ProtoMAX practical qualification remains
future. No new adapter, skill installation or machine investigation started here.

MCP1 AI/MCP integration is **PLANNED — POST-V1.5/V2+**, optional and not started:
[plan](../releases/MCP1_PLAN.md), [developer entry](../development/MCP_INTEGRATION.md).
Read-only MVP before any separately approved writes/generation/export; local candidate
transport and desktop/offline ownership/security need audit. Current accepted beta.3
candidate campaign/manual platform/physical work and TEST1/release priorities unchanged;
no server/listener/commands/schema/AI dependency or machine control introduced now.

STEP1-A/B, optional STEP2 and Smart Workshop SMART1/2, FIT1-A/B and refined REST1
are **PLANNED — NOT IMPLEMENTED**, V2+; [unified priorities](../releases/V2_WORKSHOP_PRIORITIES.md).
MULTI1 is linked ARCH-MACH/product scope, not another architecture project. No V1/V1.5
expansion except separately demonstrated safety/correctness need; current V1 candidate
blockers/reviews and native OMX priorities preserved. No kernel/dependency/feature launch.

## Historical Phase 0 audit — identity decision subsequently approved

[Audit and independent plan](../releases/v1-stabilization-campaign/PHASE0_AUDIT.md): UX7 / UX6-C / REL1-A / REL1-B. 31 baseline lifecycle tests pass. Progress overcount confirmed; no code changed. Global productName affects installer identity; approval required before rename. macOS signature failure reproduced, no valid local signing identity, no stapled ticket. HARD STOP at Stage A; no commit/push/notarization/release.

## macOS native candidate — 2026-10-16

Installed CI candidate 5ac4023 in Applications; native launch, square generation/3D/OpenSBP golden parity, V-Carve text and save/reopen passed on the existing profile. [Report](../releases/desktop-preparation/MACOS_RUNTIME_RESULTS.md). Signature assessment fails; clean-install/Gatekeeper/upgrade and the wider runtime matrix remain open. Windows/Ubuntu host checks Monday 19 October. No release; HARD STOP.

## Desktop candidates — preparation complete, platform qualification open

2026-10-16 · ivac-2p53v.1 / parent ivac-2p53v.
User accepted stabilization and authorized preparation for macOS, Windows 11 and Ubuntu.
[Preparation results](../releases/desktop-preparation/RESULTS.md),
[build workflow and Monday protocol](../releases/DESKTOP_CANDIDATES.md).
Local 0.9.0-beta.1 arm64 app/DMG built; disk image checksum and metadata verified.
Four collector tests, actionlint and version check pass. No clean install/upgrade or
runtime qualification claimed. Windows NSIS / Ubuntu DEB are not yet produced;
manual workflow is ready. User explicitly authorized commit, push and manual dispatch on
2026-10-16, without a tag or Release. Preparation published as `5ac4023`; [manual CI run](https://github.com/harry-finch/FabCAM/actions/runs/38863177268)
is building all three candidates. Final artifact results remain unverified.
User's Windows/Ubuntu hosts available again Monday 19 October. No reminder scheduled.
GEOM1 and independent human/physical gates remain open. HARD STOP before release.

## Historical version checkpoint — 0.9.0-beta.1

User authorized version assessment, documentation/UI alignment and publication of
all pending work on 2026-10-15. [Version policy](../releases/VERSIONING.md): bounded
V1 software scope is implemented and stabilizing; installer, human and physical
gates remain open. This publication supersedes earlier no-publish statements for
this checkpoint, without closing reviews or declaring 1.0 ready.

## V1 bounded stabilization — software corrections delivered

2026-10-15 · ivac-chhz0 · Lot explicitly approved by the user.
[Results, regressions and actual UX evidence](../releases/V1_STABILIZATION_RESULTS.md).
XOR parity and deep Pocket selected-nesting semantics corrected; operation-tool gear
now edits the project copy; text-only reopen framing/diagnostics corrected; production
WASM guard checks complete assets and content fingerprints with locked rebuild.
Native/WASM program parity and independent OpenSBP/preview decoding are qualified.
Known parallel cache-test race ivac-htc8 remains; serialized core qualification and
all final check counts are recorded in the linked report.

**Not release-ready:** current macOS/Windows 11/Ubuntu packages and actual install,
upgrade/runtime qualification remain pending; Windows host/VM still needed.
GEOM1 user review remains explicitly open; no physical validation, commit or publication.
**HARD STOP before the next checkpoint.** Earlier entries below are historical.

## V1 Scope Freeze & Release-Blocker Triage — handoff

2026-10-15 · ivac-oznar. [Results, target matrix and stabilization proposal](../releases/V1_SCOPE_FREEZE_TRIAGE.md).
User confirms Windows/Linux Intel x64; Windows 11 selected after Windows 7 compatibility
review. Latest Ubuntu pinned to 26.10; macOS reference remains 26.5.2 arm64. Proposed
packages: macOS DMG, Windows NSIS, Ubuntu DEB. Actual installs remain unqualified;
a Windows 11 test host/VM is still needed.

7 core geometry characterization, 22 frontend logic and 7 CPS guard tests pass.
XOR defect and partial-WASM freshness acceptance reproduced; Pocket nesting/tool
shortcut risks source-confirmed, full UI/output repros remain for bounded fixes.
GEOM1 explicitly **not yet reviewed** by user; ivac-e2jnb stays open. Independent
human/physical gates unchanged. No production fixes/build/release performed.
**HARD STOP before proposed V1 stabilization lot implementation.**

## Release roadmap approved — V1 macOS, Windows and Linux

2026-10-15. User approved the [release roadmap](../releases/RELEASE_ROADMAP.md)
with an explicit amendment: **Windows and Linux are V1 REQUIRED alongside macOS**.
[V1 matrix](../releases/FABCAM_V1_SCOPE.md) now requires packages and actual clean
install/upgrade/runtime checks on all three platforms. Exact OS versions, CPU
architectures, Linux distributions and package formats remain to be pinned.
V1.5 ProtoMAX and 2.0+ direction accepted; no waterjet implementation authorized.

Documentation update only (ivac-zgmds); no new code, build, platform qualification,
physical test, commit/push or release. GEOM1 ivac-e2jnb and independent FS/UX/
cancellation gates remain open; M4 physical evidence retains its narrow scope.
Next unstarted checkpoint: **V1 Scope Freeze & Release-Blocker Triage** — complete
GEOM1 human review, pin target variants and operation claims, then prioritize defects.

**HARD STOP — roadmap approved; no next milestone started by this update.**

## GEOM1 — engineering complete; human software/UX review pending

2026-10-15. ADR-0021 and four refinements approved; bounded phases A–F implemented.
[Results, screenshots and limits](GEOM1_RESULTS.md), [normative contract](GEOM1_CONTRACT.md),
[serialization](GEOM1_SERIALIZATION.md), [checks](geom1/checks.json).
Stable source identity/lineage, authoritative Rust drawing evaluation, explicit line
repairs, undoable geometry/bindings, guarded v7 and safe old-backend refusal are present.
Automatic curves/complex healing, source rebasing, synthetic stock/modifier combinations
and new machine capabilities remain excluded/refused as documented.

Validation: 1198 full core tests, 23 final focused references (overlapping), 44 CPS,
1312 frontend tests (1 existing skip); check/lint/clippy/schema/build pass. Real
WASM/native parity and UI save/reopen/Undo/Redo/stale-export reviewed. Reset/import-ID
reuse and unsafe old-format binding downgrade counterexamples have explicit guards.
Engineering `ivac-8rura`; human gate **ivac-e2jnb OPEN**. Software only; no physical or
packaged-app qualification. **HARD STOP before any next milestone.** No commit/push/
merge/release performed. Independent prior gates remain unchanged.

GEOM2-A1 — Precise Imported-File Dimensions is **PLANNED — NOT IMPLEMENTED**;
[specification](GEOM2_A1_PLAN.md). Early GEOM2-A subtask: local whole-file Width/
Height over existing uniform Scale, preserving ADR-0021 pivot and edited-source
lineage. Current GEOM1 review ivac-e2jnb remains next; no new implementation or
publication authorization is inferred from this documentation update.

## Source publication — authorized

User explicitly requested committing and pushing all pending PERF1 and UX work
on 2026-10-15. This supersedes earlier publication restrictions for this source
checkpoint only. Pending human review gates, the CPS cancellation latency target,
native/physical qualification limits and future milestone approvals remain unchanged.

## Preview navigation — focused follow-up delivered

ivac-tgxsn: zoom-adaptive decimal2D grid and middle-button3D pan; updated gesture
help. [Results and actual zoom captures](ux/PREVIEW_NAVIGATION_POLISH.md).42 targeted
tests pass; Svelte0 diagnostics, lint/build pass. Actual2D zoom/fit and3D/help reviewed;
central drag mapping source-verified, physical gesture not automated. No CAM/snap/
project-format changes. UX6-B accepted; GEOM1 audit is now authorized; see current checkpoint above.

## UX6-B — accepted

User accepted UX6-A (ivac-osa4z closed) and authorized B. [Results and actual
captures](ux/UX6_B_RESULTS.md): searchable compact list/contextual editor, whole-library
draft, same-ID consequence review, staged import, copy availability and retained unknown
fields. Existing manual rates/history/guards preserved. 1284 frontend passes/1 existing
skip; check/lint/build pass; actual desktop/keyboard/1024px review. No physical claim.

Availability review polish: compact inspector typography/section headings, lists and
messages now match Geometry/Mounting; [actual after](ux/ux6-b/availability-after-polish.png).
Svelte/check/lint/build pass; 32 targeted tests pass; 1024px and keyboard focus reviewed.
Association/draft behavior unchanged (ivac-b54jh).

**ivac-y2aln closed: user accepted UX6-B and Availability polish.** Engineering
ivac-f8w26 and polish ivac-b54jh complete. The subsequent user request authorizes the GEOM1 architecture/tolerance audit;
see the current checkpoint above. No UX7 is defined. Independent PERF/hierarchy/native/physical gates unchanged.

## UX6-A — accepted

[Results and actual desktop captures](ux/UX6_A_RESULTS.md): independent profile
inspection, explicit assignment review, scoped drafts/conflict protection, staged
creation/import/export and contextual compact editor. Existing project commands,
formats, CAM/post behavior and manual cutting values preserved. 1277 frontend
passes/1 existing skip; check/lint/production build pass. Browser desktop reviewed;
no packaged-native or physical qualification claim.

Review polish delivered: persistent active-section accent and spaced icon/text
library-add button; [actual capture](ux/ux6-a/tools-polish.png). Same checks pass.

**ivac-osa4z closed: user accepted A and authorized B.** Engineering ivac-jhfns complete.
Independent PERF/hierarchy gates unchanged; no commit/publication authorized.

## UX6 Phase 0 — architecture and UX proposal accepted

User accepted Drawings/Layers; ivac-ena7m closed. UX6 Phase 0 audit ivac-4df39
complete: [78-field ownership/effects matrix, action contracts and qualification plan](ux/UX6_PHASE0_AUDIT.md),
[representative layouts](ux/ux6-phase0/proposed-layouts.png). Existing current UI and
dirty-draft-after-Undo behavior reviewed; 140 targeted tests pass. No production
source, machine/tool files, CAM/FS/post behavior or persistence formats changed.

**Phase 0 accepted; ivac-tjgt1 closed when user authorized UX6-A.**
Proposed A introduces independent profile inspection, explicit project assignment,
contextual grouped editing and honest limits/time estimates. UX6-B tool library
follows only after A's separate acceptance. Independent ivac-1drzo, ivac-nh6q and
UX2.2 gates remain open. No publication authorized.

## Drawings and layers clarification — accepted

Focused continuation ivac-yjcv2 delivered and human review ivac-ena7m accepted
2026-10-15. [Audit, qualification and actual before/after](ux/DRAWINGS_LAYERS_CLARIFICATION.md).
Consistent Drawings/import count, separate Layers count, visibility controls and
read-only ACI swatches; no CAM/import/history/serialization changes. 1270 frontend
passes/1 existing skip; check/lint/build pass. Portable view-state limits retained.

## Visual hierarchy polish — human visual review pending

Focused UX2.2 continuation ivac-28e92: slightly distinct navigation surface, quiet
machine/tool context cues, compact section-heading contrast and less prominent
dark minor grid. Four source files, CSS only; all CAM/state/export/persistence
logic unchanged relative to the incoming working tree. [Qualification and actual
before/after captures](ux/VISUAL_HIERARCHY_REFINEMENT.md). Header heights preserved;
1024px names truncate without overflow or hiding SOFTWARE ONLY.1263 frontend
passes/1skip; check0 diagnostics; lint/production build pass.

**Next action: human visual review ivac-1drzo.** No new major milestone or functional
roadmap edit. Existing PERF cancellation gate ivac-nh6q remains independent/open;
no acceptance, next implementation or publication is inferred from this polish.

## CPS cancellation — cooperative improvement, latency target still unmet

B2 accepted by user; ivac-s5xy closed. [Results](PERF1_CPS_CANCELLATION_RESULTS.md)
and [ADR-0020](adr/ADR-0020-cooperative-cps-cancellation.md): connect existing CPS
probe to native token, retain cancellation/cache/output guards. Native Test median
750→213ms, dense contour1181→193ms; HTTP743→207ms. Proposed p95≤200ms target
**not met** (Test216ms/native,218ms/HTTP). All18 output/refusal outcomes unchanged.
44 CPS and1086 core unit tests pass; actual UI cancel/stale export/retry reviewed.
Engineering scope ivac-sxmw hands off the remaining latency objective to
ivac-q1nz1 (phase attribution before stronger boundary). **ivac-nh6q OPEN — HARD
STOP for human review.** No next work/publication/physical qualification.
Older pending-gate wording below is historical and superseded by acceptance above.

## PERF1-B2 — engineering complete, human results review pending

[Results/evidence](PERF1_B2_RESULTS.md): qualified planar-kernel reuse,4entries/
8MiB, live eligibility/stock/provenance guards. Repeated Pocket islands stage
337.845→0.222ms; total338.370→0.625ms. Cold409.988→413.449ms, no cold speedup.
All18 native outcomes unchanged; WASM remaining/program parity;1085 core and38
integration tests pass. UI stock edit/Undo, unsupported reasons and overlay checked.
Small CAM4 UI fixture shows no material latency gain (separate from native benchmark).
ivac-dlrm complete; **ivac-s5xy OPEN — HARD STOP** for human results review.
No physical qualification/publication. B1 human gate ivac-sf7y accepted by user.

## PERF1-B1 — engineering complete, human results review pending

[Results and evidence](PERF1_B_RESULTS.md): bounded bundled ShopBot post reuse,
fail-closed benchmark evidence, finer simulation attribution. Native Test repeat
832→15ms; HTTP first 3D frame899→141ms, rendered relief2259→1516ms. First/new
calculation unchanged within small-sample variation. All18 A outcomes preserved;
1081 core,10 V-Carve,7 ShopBot guard and1263 frontend tests pass (1optional skip).
Actual UI edit/Undo freshness and byte-identical SBP checked. ivac-rnsg and
ivac-lf1e closed; **human gate ivac-sf7y OPEN — HARD STOP**. No B2, physical
qualification or publication. [ADR-0018](adr/ADR-0018-bounded-shopbot-post-reuse.md).
Older A pending wording below is historical; ivac-xzys closed by B authorization.

## PERF1-A — engineering complete, human report review pending

User explicitly accepted delivered software/UX on 2026-10-15 and authorized the
next milestone. Geometry ivac-xy3w, V-Carve ivac-1b75 and UX5 ivac-txve human gates
are closed as software acceptance, with their documented numerical/native/physical
limits retained. Published snapshot: main 2032848 (270 files), pushed 2026-10-15.
Earlier pending/STOP wording below is historical and superseded by this record.

PERF1-A engineering ivac-sgnd is complete: [results and proposed B scope](PERF1_A_RESULTS.md),
[raw summary](perf1/SUMMARY.md). Saved Test is a **representative surrogate**, not
the reported 5–10-second project. Native Test CPS warm median904ms; profiled post
share98.4%. Actual HTTP first frame median935ms; simulation replay2310–4051ms
including debounce. Pocket bottleneck is remaining-material analysis. Output
checks,1262 frontend/1075 core tests and qualification/check/lint/build gates pass.

**Next action: human report and PERF1-B scope review ivac-xzys.** Proposed B1 is
audited unchanged bundled ShopBot whole-post reuse (ivac-rnsg); no optimization
implemented or authorized. Cancellation latency ivac-sxmw and finer simulation
attribution ivac-lf1e are deferred. Older independent machine/FS gates remain
separate; no new physical observations or publication authorization.


## Geometry source UX refinement — human review pending

Requested shared-inspector refinement ivac-uebs: Geometry / Use geometry from /
Contour interpretation, actual SVG/DXF/file provenance, preserved saved multi-layer
bindings, explicit pending-source state, one-command source edits and consumer-specific
help. [Architecture, semantics, qualification and actual captures](GEOMETRY_SOURCE_UX.md).
No geometry/import/boolean/toolpath/post changes: 120 protected production files
unchanged; actual SBP output identical for both V-Carve strategies.

1260 frontend passes/1skip; check0 diagnostics; lint/build pass. 7 new core audit
fixtures and existing combine9/selection9/offset54/import41 suites pass (2 optional
DXF corpus tests return early; new self-contained DXF coverage runs). V-Carve10,
Halfpipe15, ShopBot7 pass. Actual source Undo/Redo/Save-reopen/duplication, missing
layer diagnostics/export refusal and desktop/narrow layouts verified.

**Next action: human visual review ivac-xy3w**, including native popup/tooltip and
arrow-key checks. Separate follow-ups: source highlight ivac-9oty, deep Pocket Auto
preview/emission nesting audit ivac-dli7, XOR early-empty audit ivac-j2po.
**V-Carve final acceptance ivac-1b75 remains independent and pending.** No new
milestone or publication; roadmap scheduling remains subject to human approval.


## V-Carve strategy UX clarification — human review pending

The user-requested presentation follow-up (ivac-klww) replaces algorithm labels
with **Outline groove / Variable-depth engraving**, a full-width native selector
and contextual explanations. Final depth becomes Maximum carving depth only for
the variable strategy; the same field, sign,0-auto semantics and pass values remain.
Existing boolean/default/history/persistence/templates/diagnostics/export guards
are preserved. [Strategy UX evidence](VCARVE_STRATEGY_UX.md).

1249 frontend passes/1skip; check0 diagnostics; lint/build pass.10 V-Carve,
15 Halfpipe and7 ShopBot guard tests pass. Nine protected sources remain byte-
identical; actual OpenSBP programs match exactly for both strategies. Browser
verifies Undo/Redo/Save-reopen, selected descriptions, advanced controls, warnings
and approximately280/360/550px sidebars. Native popup keyboard navigation remains
an explicit manual acceptance check; no new physical/native qualification.
**Next action: existing final human gate ivac-1b75. STOP before another milestone
or publication.** Future Two-Sided plan remains planned, with separate launch approval.
This section supersedes earlier V-Carve next-action presentation wording below.


Final UX/diagnostics refinement is implemented; see [final UX audit and screenshots](VCARVE_FINAL_UX.md). Unsupported saved settings remain visible/rejected and explicitly correctable; inlay action unavailable; warning severities/export guards preserved. **Final human acceptance ivac-1b75 remains pending.**

## V-Carve qualification — software implementation ready for review

User approved the targeted V-Carve-only planner. A1/A2/A3 red references now pass;
A4 independently verifies generic paths → OpenSBP → resolved preview. Halfpipe
sources unchanged; 15 named tests pass. Perimeter stays a contour groove; Full
medial stays sampled spines, **partial area coverage**, not a complete pocket.
Strict clearance refuses the square-island Perimeter counterexample.
[Qualification, limits, fixtures and screenshots](VCARVE_QUALIFICATION.md),
[ADR-0017](adr/ADR-0017-vcarve-continuous-planner.md). 10 dedicated, 1,074 core,
7 post guard, 51 existing CAM/post regression tests pass; frontend 1,234/1skip,
check/lint/build and Clippy pass. No schema migration or FS changes in this work.
**HARD STOP: human software review ivac-1b75.** No physical qualification,
commit/push or next milestone authorized. ivac-2eh7 implementation completed;
independent pre-existing review gates remain unchanged.

Future **Two-Sided Machining & Registration** and **V-Carve Inlays** are recorded
as **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** in their promised complete scope.
[Two-sided plan](TWO_SIDED_PLAN.md), [inlay plan](VCARVE_INLAYS_PLAN.md).
Active V-Carve priority/decision/HARD STOP and independent gates stay intact;
no implementation starts here. Open ivac-jtkj still tracks rejected CPS/translated
registration; closed inherited inlay work is not qualified fit evidence.

GEOM1, GEOM2-A/B, GEOM3 and NEST1/2 are **PLANNED — NOT IMPLEMENTED**:
[geometry plan](GEOMETRY_PREPARATION_PLAN.md), [transform/nesting architecture](NESTING_PLAN.md).
GEOM2 is refined into transformations A and drawing B. NEST1 depends on qualified
A/reliable sources, not all B; NEST2 follows manual layout. Current V-Carve review
ivac-1b75 and independent priorities/gates remain unchanged; no new code now.

PERF1-A profiling and PERF1-B targeted optimization are **PLANNED**, not started;
[measurement/architecture plan](PERF1_PLAN.md). Reported 5–10 s text generation
is informal and awaits reproduction. A is proposed near-term after current V-Carve
acceptance; its report/HARD STOP precedes B. Current ivac-1b75 and other priorities
remain unchanged; no instrumentation or optimization in this checkpoint.

## UX5 — discoverability refinement ready, human review pending

UX5 implementation is satisfactory in the human review; its requested small
polish is implemented locally (ivac-snsy). Welcome returns to the approved three
primary actions and branding with no palette hint/replacement. Existing File
menu retains **Command Palette…** with platform shortcut; Help concisely documents
keyboard navigation/execution/dismissal. The authoritative shortcut/opening
handler, palette registry/search/ranking/availability/history/export guards are
unchanged. [Current qualification and five captures](UX5_QUALIFICATION.md).

1232 frontend passes/1skip;0 check diagnostics; lint/build/diff check pass. Actual
macOS browser verifies welcome/empty/loaded/geometry/operation contexts, menu,
focus/Esc/Tab and modal/input guards; before/after Save files are byte-identical.
Windows/Linux mapping/display are unit-tested, without new native-runtime claims.
Earlier UX5/native/IME/picker/physical limits remain. No new major milestone.
**Next action: UX5 human software gate ivac-txve. STOP before later milling work
or publication.** Roadmap and next V-carve qualification scope remain unchanged.
No commit/push requested. This section supersedes earlier UX5/UX4.1 next actions.

## UX4.1 — templates and sidebar refinement, human review pending

User-requested UX refinement is implemented locally (ivac-01ux): reuse-first
searchable templates; separate save/contextual rename/confirmed delete; stored
parameter preview and diameter review. Existing v1 templates/manual values and
v3/v6 projects retain their semantics. Shared section-bounded sticky headers and
adjacent total counters preserve orientation during long inspector edits.
[Qualification, before/after captures and fixtures](UX4_1_QUALIFICATION.md).

1178 frontend passes/1skip; zero check diagnostics, lint and production build pass.
Actual browser confirms history/counts/save-reopen/ShopBot/stale guards, all four
sections, 32-operation fixture, sidebar resize and 1100×660 desktop. No new core,
FS/post semantics or physical/native qualification. Initial review ivac-51gg is
superseded by **ivac-si5w**, not retrospectively approved.
**Next action: UX4.1 human review. HARD STOP before UX5.** No publication requested.
This section supersedes earlier UX4 review-next-action wording below.

## UX4 — operation templates software handoff, human review pending

User explicitly authorized UX4 on 2026-10-14; UX3 continuation gate ivac-k4z3
accepted without new physical/native claims. UX4 is implemented locally
(ivac-epg2): named local Profile/Pocket/planar-surface templates, explicit current
geometry and rough/finish tool binding, frozen manual values, independent job
operations and atomic Undo/Redo. Source-specific/private evidence is excluded.
Library supports rename/remove/restore and persists in the existing workspace.
[Qualification and review fixtures](UX4_QUALIFICATION.md),
[ADR-0015](adr/ADR-0015-reusable-operation-templates.md).

1172 frontend tests pass (1 optional skip), check/lint and production build pass.
Browser verifies library reload, explicit choices, Undo/Redo, v3/v6 save/reopen,
different-tool retained rates, ShopBot generation/export and stale blocking.
No UX4 Rust/CAM/post changes or new native bundle/physical qualification.
**Next action: UX4 human gate ivac-51gg. HARD STOP before UX5.**
No commit/push requested. This section supersedes older UX3 pending and
UX4-unstarted wording below; independent FS/UX/native/physical gates remain.

## UX3 — planar surfacing software handoff, human review pending

User authorized the next milestone on 2026-10-14, accepting CAD1 continuation
(ivac-7r7r closed; no wider geometry or physical evidence claimed).
**UX3 — Planar spoilboard surfacing** is implemented locally (ivac-gepq):
explicit persisted rectangle/stock snapshot, flat cutter, X/Y raster, margins and
edge overtravel, depth passes, manual rates, Undo/Redo and guarded v6 save/reopen.
Shared XYZ emission, resolved work frame, final ShopBot preview and fixture/export
checks are retained. Invalid depth/tool/travel/context refuses generation.
[Qualification, fixture and review](UX3_QUALIFICATION.md),
[ADR-0014](adr/ADR-0014-explicit-planar-surfacing.md).

1074 core tests, 1161 frontend tests (1 optional skip), 8 UX3 integration tests,
27 existing ShopBot regressions and the CAD1 integration matrix pass.
Svelte/TypeScript, lint, core/server Clippy, schemas, production WASM/frontend
build and CLI/WASM/Tauri compilation pass; 14 server tests pass.
Browser verification: standalone load/generate/.sbp export; X→Y/manual feed edit
marks output stale and disables export; actual v6 Save/reopen preserves Y and
1500 mm/min; excessive stepover refuses generation. Review captures are linked
in the qualification. No new desktop bundle or physical flatness qualification.

**Next action: UX3 human gate ivac-k4z3. HARD STOP before UX4.**
Local CPS review runs on 18767/18772; independent FS/UX/native/physical gates remain.
No commit/push requested. This section supersedes older CAD1-next-action and
unstarted-UX3 wording below.

## CAD1 — software qualified, human review pending

The user approved the architecture checkpoint and bounded CAM4 A/B prerequisite
on 2026-10-14 (ivac-alch / ivac-tvox closed). CAD1 now implements exact-radius
Dogbone and both T-bone orientations, explicit corner/batch selection and preview,
a persistent source-preserving modifier, atomic Undo/Redo and stale-generation guards.
[Qualification and review steps](CAD1_QUALIFICATION.md),
[accepted architecture](adr/ADR-0013-persistent-corner-relief.md).
1070 core tests and 1151 frontend tests pass (1 frontend skip); 18 CAD1 integration
cases verify decoded ShopBot coverage and preview parity. Acute corners requiring
isolated plunges/access enlargement and other unsupported contexts are refused.
Curved residual diagnostics remain outside CAM4 A/B qualification.
Human CAD1 gate **ivac-7r7r** remains open. No physical machining qualification.
**HARD STOP: UX3/UX4 are unstarted.** Independent FS/UX gates remain unchanged.
This section supersedes older next-action wording below.

UX5 — Command Palette & Quick Actions is **PLANNED — NOT IMPLEMENTED**;
[architecture/UX plan](ux/UX5_PLAN.md), human planning gate **ivac-2zh9**.
Future order CAD1 → UX3 → UX4 → UX5 retains current labels and completion status.
CAD1's human qualification remains the next action; no UX5 development or
publication is authorized. Existing FS/CAM/UX reviews remain independent.

## FabCAM branding and identity — publication approved

User approved the A1 branding, public identity and documentation on 2026-10-14
and explicitly requested commit/push to `main`. Product/About/documentation review
**ivac-7pul** is accepted. The micro logo candidate, installer migration and
independent CAM/FS/UX qualification gates remain outside this approval.

## FabCAM product identity — accepted

Public UI/browser/window/About/report identity is FabCAM (ivac-9td7), with
verified ivaCAM origins/author and unchanged GPL license. English/German README
and active guides updated; [audit, compatibility policy and captures](ux/PRODUCT_IDENTITY.md).
Version remains 0.5.0. Mac display names use FabCAM; packaging productName,
`org.opcenter.ivacam`, executable/storage/schema identifiers remain legacy.
[ADR-0012](adr/ADR-0012-fabcam-product-identity.md); proposed installer qualification
ivac-snqn. Frontend1145 pass/1skip, check/lint, Rust29 project tests pass; local
macOS bundle/runtime verified, old M4 project/settings/recents accessible.
Human product/About/documentation gate: **ivac-7pul**, accepted.
Windows/Linux native packaging/upgrade untested. No migration/relicense/functional
milestone or release; existing milling/FS qualification gates are unchanged.

## UX2.2 A1 branding integration — human review next

User-authorized production branding is integrated locally (ivac-7cdj): canonical
header/About symbol, outlined Welcome lockup, SVG/PNG favicons and existing Tauri
desktop icon resources. Micro candidate excluded. Frontend 1145 pass/1skip,
check/lint/production build pass; macOS .app built, launched and checked in Finder.
Name/identifier/config/security unchanged. Windows/Linux assets structurally
verified, native packages/runtime untested. [Handoff and genuine captures](ux/A1_BRANDING_INTEGRATION.md).
Broader UX2.2 qualification remains **ivac-ein1**; branding publication is approved.
This is a UX2.2 branding continuation, not a new CAM milestone.

## FabCAM A1 branding asset handoff — preparation checkpoint

Approved A1 source SVGs preserved exactly in `assets/branding/fabcam/source`.
Asset-only finalization (ivac-79bq) includes an Inter outlined lockup, four
monochromes, a separate 16–20 px micro candidate, 22 PNGs, ICO/ICNS and review
sheets. [Specification and limitations](../../assets/branding/fabcam/README.md).
Canonical integration approval closes **ivac-8k3o**; micro remains excluded.
Native macOS extraction alters 16/32 px edge RGB; encoded PNG entries
are lossless and the report records both. The subsequent integration is recorded
above. Milling qualification gates below remain unchanged.

## Repository transition — 2026-10-14

User authorized integrating `feature/shopbot-mvp` into the default branch `main`
and renaming the GitHub fork to `harry-finch/FabCAM`. The integration is a
fast-forward to c9d3f3b; the feature branch and upstream relationship are preserved.
Origin now uses `https://github.com/harry-finch/FabCAM.git`. Application/package
names and the local checkout directory remain unchanged. Existing qualification
gates and the CPS regression follow-up ivac-13go retain their recorded status.

## Publication authorization — 2026-10-13

User explicitly authorized committing and pushing all current repository changes
on `feature/shopbot-mvp`. This publication checkpoint includes the CAM/UX/FS
software and qualification records below. Pending human/native/physical review
gates retain their recorded status; publication does not qualify those gates.

## Current direction and next action

**UX2.2 — Application Shell & Welcome Experience**: Atelier implemented locally
(ivac-j6ba), following explicit user approval (ivac-dwdd closed). Welcome,
New/Close, shared Open/Import, project name/save state, active machine and
selected-operation tool are integrated. Lifecycle tests and browser review passed;
[qualification and screenshots](ux/UX2_2_QUALIFICATION.md). Next: human review and
native desktop qualification **ivac-ein1**. No native bundle rebuilt.
Final polish continuation **ivac-8u1p** complete: literal machine limitations
stay visible, Layers owns the primary import, generation messaging is contextual.
Frontend 1145 pass/1skip; [seven review captures](ux/UX2_2_POLISH.md), including
critical export rejection and edit → Save → stale → reopen. Await human approval
under ivac-ein1. Existing CAM4/FS gates remain independent; CAD1 remains separate.

**FS3.9.1 — Recommendation UX Simplification** engineering qualified (ivac-hcea);
human visual review **ivac-anp0** is next. [Handoff and screenshots](ux/FS3_9_1_QUALIFICATION.md).
Inspector/advanced-entry continuation also qualified (ivac-m0qf): no default provenance,
shared actions/origin indicator and consistent disclosure rows; blocking entry warnings
remain visible while closed. Frontend1121 passes/1skip and27 ShopBot regressions pass.
Compact existing Modal, automatic unapplied mode preview, guarded asynchronous results,
explicit Apply, local capture separated and Draft by default. Frontend1117 pass/1skip,
engine98 +1doctest; check/lint/build pass. No FS mathematics, source priority, storage
or ShopBot post changes. FS4 remains deferred/nonblocking; CAM ordering unchanged.


**FS3.9 — Approximate Matching & Coverage Expansion** is explicitly authorized
as a targeted exception to the FS freeze (ivac-02in). Software qualification
is complete; human gate **ivac-t0fk** is next. See [handoff](feeds-speeds/FS3_9_QUALIFICATION.md), [policy](feeds-speeds/FS3_9_MATCHING.md) and
[coverage](feeds-speeds/FS3_9_COVERAGE.md). FS4 remains deferred/nonblocking.
Independent known limitation: ShopBot CPS rejects a rough/finish RPM transition
(ivac-2zll); see the FS3.9 handoff for the conditional export check.
[Approved sensitivity audit](feeds-speeds/FS3_9_REVIEW.md) complete (ivac-v6pf):11997 samples,98 FS tests +1 doctest. Selection/interpolation jumps quantified; production smoothing requires a separate policy decision.

Milling software remains active. CAM3 continuation approved (ivac-wh2m closed).
**CAM4 A/B — Remaining Material Analysis & 2D Preview** (ivac-qk8y) is software
qualified within the explicit per-pocket/depth scope in
[the handoff](CAM4_QUALIFICATION.md). Human gate **ivac-tvox** is next.
Amber residual rings, area/depth metrics, toggle and detail use final linear output;
stale/unsupported contexts are explicit. CAD1/Phase C corner relief remains
separate scope/approval; no Rest Machining or physical validation.
User review follow-up fixes WASM checkpoint BigInt scrubbing, warning-chip alignment
and adds transient 3D Toolpaths visibility. Existing Spiral choice is clarified;
FS reference coverage unchanged in that earlier follow-up; targeted FS3.9 now adds derived coverage. [Evidence](ux/CAM4_REVIEW_FOLLOWUP.md).

Stock/WCS/imported-geometry dragging and snapping remain deferred (ivac-t68s).
UX2 numeric Z and two-sided CPS limits remain (ivac-u1ko / ivac-jtkj).

FS4 — Physical Feeds & Speeds Validation
- **Status: DEFERRED** (ivac-mwry).
- **Reason: requires physical lab / ShopBot access**, tools and materials currently
  unavailable.
- **Blocking further software development: NO**. Neither failure nor blocker.
- Resume later with existing FS3.8 capture/storage/resolver/history/validation
  workflow; no Feeds & Speeds architecture redesign required.

FS architecture through FS3.8 is sufficient for continued CAM work. Freeze FS
expansion: no new manufacturer imports, Sorbonne tuning, mathematics, physical
trials or FS UX redesign unless a separately approved feature needs integration.
External values remain ReferenceData; Sorbonne Tested/Validated require real cuts.
WJ/ProtoMAX and VINYL remain deferred. Historical FS human reviews remain recorded,
not automatically closed by track redirection or dependencies of milling software.

Future FabCAM/FabSBP ownership and unnumbered planning groups are recorded in
[the workflow plan](FABCAM_FABSBP_WORKFLOW.md) and ADR-0009. Documentation only:
FabCAM declares requirements; future FabSBP establishes physical state. No implementation in that documentation checkpoint; no FabSBP development authorization.

UX2.2 is now the selected next milestone; its first source/design checkpoint is
complete. [Specification](ux/UX2_2_PLAN.md), [audit](ux/UX2_2_AUDIT.md), human
design/scope gate **ivac-dwdd**. This selection does not rerun CAM3 or silently
close the pending CAM4/FS reviews. Production implementation awaits the concrete
design/lifecycle decision; no persistence/post/FS behavior changed.

## Current browser review session

User test URL `http://127.0.0.1:18767/?api=wasm` explicitly forces the standalone
WASM engine, which has no CPS support in the current package. For ShopBot review,
use `http://127.0.0.1:18767/?api=http%3A%2F%2F127.0.0.1%3A18772` with the current
CPS-enabled native server running on 127.0.0.1:18772. CORS allows the 18767 origin.
The server is deliberately left running for human testing (ivac-t79u); binary is
in /private/tmp/fabcam-cps-runtime, outside root target/debug. This is a local review
session, not a service installation or new native desktop bundle. Save/reopen your
own project when switching page URLs; no user project file was rewritten.

## Verified source and delivery

- Checkout: /Users/mugen/Documents/01_Projets/FabCAM/ivacam.
- Branch: main; origin harry-finch/FabCAM, upstream aalarchiv/ivacam.
- Previous committed/pushed software: **96e41ec4e8a3b593579079cbb6c051e070cd03ea**
  (FS3.7/FS3.8), explicitly published by user authorization on 2026-10-12.
- M4 baseline: e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1; M0–M4 names retained.
- Current historical native delivery: ../archives/M4/ivaCAM-M4-r2/ivaCAM.app. No FS3.8 native
  bundle or physical trial is claimed. target/debug cleaned by prior user request;
  release/WASM packages retained.
- M4 operator-reported Preview/air-cut/MDF exterior-square and VCarve comparison
  evidence remains in [M4 validation](M4_VALIDATION.md) and
  [comparison](M4_VCARVE_COMPARISON.md). Exact instrument/readings/tolerance and
  measured RPM remain unreported; this is not blanket milling qualification.

## Current evidence and limits

FS3.9: 94 FS tests +1 doctest,17 core FS adapter tests,1112 frontend passes/1 optional
skip; check/lint/fmt/Clippy/schema-check and optimized WASM/frontend build pass.
Original six public cutting rows numerically unchanged; no physical evidence
promotion, additional manufacturer data or private Sorbonne population.
[Qualification](feeds-speeds/FS3_9_QUALIFICATION.md).


CAM4 A/B: core1,062; M1 11; ShopBot27; snapshots4 pass. Frontend1,112 passes /
1 optional skip in120 files; check/lint/fmt/Clippy/schema-check and complete optimized
WASM/frontend build pass. Buffered response/type addition is compatible; project
serialization and machining output unchanged. Actual browser four-corner overlay,
strategy stale suppression even after Save, unsupported reason and Save/reopen
checked. New generation resets visibility. No native desktop bundle/physical trial.

Earlier CAM3 holding-tab and UX2.1 fixture/history evidence remains in their records.
A/B covers metric Cascade Pocket, one Endmill, straight closed targets, linear
returned execution moves and qualified Z0 stock depth. Tabs/allowance, curved or
analytic-arc targets/paths and broader contexts are Not analyzed, with reasons.
No cumulative job/physical stock-removal verdict.

Helix, Ramp+tabs/leads, short-path vertical fallback and ShopBot G55–G59 retain
explicit rejection. Numeric XY origin is fixed (ivac-vj11); Z refuses rather than
being ignored. Genuine two-sided CPS/translated registration refuses. Fixtures
warn when holder geometry is missing; no physical clearance/automatic avoidance
claim. FS4 does not block this software work.

[Milling audit](MILLING_CAPABILITY_AUDIT.md) remains the dated feature inventory.
Dogbones/T-bones, dedicated spoilboard surfacing and named operation templates are
future gaps; fixture creation is now available under Stock.
Known ivac-htc8 global pocket-cache parallel-test race retains serial testing;
ivac-5g51 partial WASM freshness guard limits retain complete package verification.

Redirection, UX1, CAM2/UX2, UX2.1, future workflow documentation, CAM3, CAM4 A/B and FS3.9 changes are uncommitted; publication requires authorization.
