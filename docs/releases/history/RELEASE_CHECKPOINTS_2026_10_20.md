> Historical snapshot, captured 2026-10-20 from source `94cdd6bbdb5887ffce48adb86b8ea8a088402ac9`.
> Preserved checkpoint wording, including superseded OPEN/current claims; relative links adjusted only.
> For current state use [release roadmap](../RELEASE_ROADMAP.md); this snapshot grants no approval.

# FabCAM release roadmap — approved direction

> 2026-10-19 — [P0/P1 export/load checkpoint](../UX_SAFETY_LOAD_RESULTS_2026_10_19.md): software fixes and integrated evidence complete; human gate **ivac-3i00w OPEN**, explicit qualification coverage limits **ivac-b0js3 OPEN**. No new candidates/commit/publication authorized. UX D and consolidation gates remain distinct. **HARD STOP.**

> 2026-10-19 — **Consolidation UX A–D terminée, candidats BLOQUÉS** (beta.3/pipeline62). [Bilan et preuves](../UX_V1_CONSOLIDATION_2026_10_19.md). Core1204, frontend1387/1 SKIP +10 Node, intégration59 et TEST1 optimisé256/256 PASS. Un correctif clavier local ; défaut sécurité lecture/export **ivac-3xqtb** et réserve chargement/format **ivac-52tpd** soumis sans correction. Revues **ivac-amibd / ivac-he4lt OPEN**. Candidats existants beta.3/pipeline61 inchangés. Aucun commit/push/publication ni nouveau candidat. **HARD STOP pour validation.** Les notices antérieures sont historiques.

> 2026-10-19 — **Lot D UX livré localement**, beta.3/pipeline62 : vues stables, diagnostics lisibles, réglages consolidés et fenêtres recalées. [Résultats et limites](../UX_V1_LOT_D_2026_10_19.md). A/B/C acceptés ; revue D **ivac-he4lt OPEN**. Frontend1387 PASS/1 SKIP +10 Node, check/lint/build PASS. Aucun commit/push/publication. **HARD STOP** ; consolidation et candidats ultérieurs non lancés.

> 2026-10-19 — **Lot C UX livré localement**, beta.3/pipeline62 : origine XY, Frame réversible, matériau en brouillon, compatibilité texte et fiche atelier fidèle aux résultats courants. [Résultats et limites](../UX_V1_LOT_C_2026_10_19.md). Lot B accepté (`ivac-ex4v8` fermé). Revue C **ivac-9pljy OPEN**. Tests frontend 1383 PASS/1 SKIP +10 Node ; check/lint/build PASS. Aucun commit/push/publication. **HARD STOP avant lot D**, non commencé. Les notices antérieures sont historiques.

> 2026-10-19 — **Lot B UX livré localement**, beta.3/pipeline62 : copies projet/locales explicites, comparaison type/diamètre avant application, accès contextuel et schéma dimensionnel. [Résultats et limites](../UX_V1_LOT_B_2026_10_19.md). Lot A accepté (`ivac-lec7h` fermé). Revue B **ivac-ex4v8 OPEN** ; réserve fenêtre redimensionnée `ivac-ntg7i`. Aucun commit/push/publication. **HARD STOP avant lot C**.

> 2026-10-19 — [V1 UX audit Phase0](../UX_V1_POLISH_PHASE0_2026_10_19.md) complete: full audit01–62 consolidated against beta.3/pipeline61 source. Four batches proposed, not started; consumer/capability corrections and separate CPS/envelope/fonts decisions documented. Existing beta.3 candidate evidence retained. Human approval required before implementation. No production change, commit/push/publication. **HARD STOP**.

> Current checkpoint — 2026-10-18: [Final SAFE-Z / TEST1 consolidation](../FINAL_CONSOLIDATION_2026_10_18.md), pipeline61. Fresh29 focused checks PASS; preserved20,167-move budget failure reproduced. Source/evidence integrity checked; no production change. Human review **ivac-l6ouh OPEN**; ADR UX review ivac-8371y separate. Resource/cancellation reserves conditional; platform/physical gates unchanged. Next after approval: matching candidates with separate commit/push/workflow authorization. No commit/push/build/publication. **HARD STOP**.

Earlier checkpoint notices below are historical snapshots; their OPEN/not-started wording is superseded by the current report and tracker.

> Current checkpoint — 2026-10-18: [ADR-0023 publication guard](../ADR0023_PUBLICATION_GUARD_RESULTS_2026_10_18.md), pipeline61. Mandatory collision refusal with inspection-only preview, pre-byte fixture-stream refusal and explicit incomplete-body diagnostics implemented. Human review **ivac-8371y OPEN**. Previous ivac-6cske accepted. Full SAFE-Z/TEST1 and platform/physical gates remain open. No commit/push/publication. **HARD STOP**.

> 2026-10-18 latest: [SAFE-Z extended checkpoint](../SAFE_Z_EXTENDED_RESULTS_2026_10_18.md), pipeline60 targeted fixes and passing bounded tests. Full qualification remains open; proposed ADR-0023 publication policy requires human approval (ivac-6cske). No commit/publication. HARD STOP.

> 2026-10-18: [TEST1 bounded harness](../TEST1_RESULTS_2026_10_18.md) passes256 cases locally; human review pending. This does not close full TEST1/SAFE-Z, installation or physical gates. No release authorization.

> Source publication — 2026-10-17: user authorized commit/push of all pending
> changes. Current source version is **0.9.0-beta.2**; existing beta.1 installers
> and validation records retain their original versions. Earlier publication
> restrictions are superseded for this source checkpoint only; human/physical
> review gates remain open. No workflow dispatch, tag or Release.

> Current checkpoint — 2026-10-16: [REL1-B1 local packaging preparation](../v1-stabilization-campaign/REL1_B1_RESULTS.md) complete: arm64 community DMG, strict ad-hoc/identity/integrity and SHA-256 PASS; workflows linted, no dispatch. Clean-download/install/runtime on the other Mac NOT TESTED and still V1-blocking. Developer ID/notarization remain optional POST-V1. Windows/Ubuntu Monday plans unchanged. HARD STOP for review; no commit/push/publication or transmission.

> Historical checkpoint — 2026-10-16: Phase 0 and simple beta-installation transition approved. [REL1-A implemented locally](../v1-stabilization-campaign/REL1_A.md): FabCAM public package names, stable data identity, strict ad-hoc signature and native reopen PASS on macOS. No publication. Human review gate before UX7, then UX6-C; distribution remains UNQUALIFIED.

> Historical Phase 0 audit — 2026-10-16: UX7 generation timing, UX6-C bounded UI consistency, REL1-A public packaging identity, REL1-B macOS distribution blocker. [Phase 0 audit](../v1-stabilization-campaign/PHASE0_AUDIT.md); implementation awaits explicit installer-identity transition decision. No RC/release qualification.

> Historical 2026-10-16 checkpoint: [desktop candidate preparation](../DESKTOP_CANDIDATES.md).
> Stabilization accepted. macOS DMG built; Windows/Ubuntu candidates not yet built.
> Host availability expected Monday 19 October; installation/runtime gates remain open.

Current development version: **0.9.0-beta.1**, V1 stabilization beta.
[Version decision and progression](../VERSIONING.md); 1.0 release gates remain open.

> Historical 2026-10-15 checkpoint: [bounded stabilization corrections](../V1_STABILIZATION_RESULTS.md),
> following explicit user approval. Windows 11 x64 and Ubuntu 26.10 amd64 remain
> required alongside macOS; actual desktop installation/runtime gates remain open.
> GEOM1 human review is still pending. HARD STOP before the next checkpoint.

2026-10-15 · Documentation only · Local tracking ivac-7m4al.
**ROADMAP APPROVED with amendment: V1 requires macOS, Windows and Linux.**
Human approval recorded 2026-10-15 (ivac-zgmds). No release date, feature launch,
platform qualification or publication is implied; independent gates remain open.

> End of the preserved release-checkpoint preamble.
