> Historical snapshot, captured 2026-10-20 from source `94cdd6bbdb5887ffce48adb86b8ea8a088402ac9`.
> Preserved checkpoint wording, including superseded OPEN/current claims; relative links adjusted only.
> For current state use [legacy readiness entry](../V1_REMAINING_2026_10_18.md); this snapshot grants no approval.

# Remaining V1 gates — consolidated 2026-10-18

> 2026-10-19 — [P0/P1 corrections qualified in software](../UX_SAFETY_LOAD_RESULTS_2026_10_19.md). Playback no longer controls export qualification; optional post metadata no longer leaks between full project loads. Review **ivac-3i00w** and coverage limits **ivac-b0js3** before candidate preparation. UX D/consolidation and platform gates remain separate. Existing desktop artifacts unchanged; no new release authorization.

> Latest follow-up — 2026-10-19: [Stage A export/load investigation](../UX_SAFETY_LOAD_STAGE_A_2026_10_19.md) confirms both defects with executable red evidence. P0/P1 remain unresolved. Dedicated background simulation qualification is proposed in ADR-0024 and awaits the requested architectural approval. No new candidate preparation; B–D and final qualification still pending.

> 2026-10-19 — **Consolidation UX A–D terminée, candidats BLOQUÉS** (beta.3/pipeline62). [Bilan et preuves](../UX_V1_CONSOLIDATION_2026_10_19.md). Core1204, frontend1387/1 SKIP +10 Node, intégration59 et TEST1 optimisé256/256 PASS. Un correctif clavier local ; défaut sécurité lecture/export **ivac-3xqtb** et réserve chargement/format **ivac-52tpd** soumis sans correction. Revues **ivac-amibd / ivac-he4lt OPEN**. Candidats existants beta.3/pipeline61 inchangés. Aucun commit/push/publication ni nouveau candidat. **HARD STOP pour validation.** Les notices antérieures sont historiques.

Current source: beta.2/pipeline61, uncommitted; existing candidates: pipeline54.
[Consolidation and exact evidence](../FINAL_CONSOLIDATION_2026_10_18.md) supersedes the
pipeline58/59 triage state. No commit, push, build or publication authorized.

| Gate | Current disposition | Next evidence / approval |
|---|---|---|
| Post-audit HATCH/known-stock/holder/publication defects | Targeted fixes implemented and software regressions passing | Preserve current guards; no broad rewrite required. |
| TEST1 / SAFE-Z | Bounded tests passing; Z01–Z08 mapped to fixed/generated evidence with explicit limits | Human review ivac-l6ouh; overall ivac-4z6lh/ivac-j09eh remain open pending bounded disposition. |
| ADR-0023 UX | Local HTTP inspection and code tests recorded | Human review ivac-8371y; do not invent manual test observations. |
| Large pocket / cancellation | Open conditional reserves ivac-53ky9/ivac-42ylx, not demonstrated new safety defects | Representative packaged-runtime measurement; freezes/OOM/wrong output/partial success block release. No universal200ms or size guarantee. |
| Candidate freshness / CI | Old installers omit latest fixes; local source fingerprint recorded | Separate commit/push/workflow authorization, matching macOS arm64/Windows11 x64/Ubuntu amd64 builds, CI and integrity checks. |
| Installation / runtime | Current-source platforms NOT TESTED | Clean browser-download/install/runtime on M3/macOS14.6 and Windows/Ubuntu hosts; preserve data and record actual OS versions. |
| ShopBot workshop | Current release NOT physically qualified | Existing A01–A07 operator protocol, controller preview, supervised air/material tests and measurements. |
| Remaining UX / RC | Separate GEOM1/FS reviews must be reconciled against actual observations | Supported/refused matrix, user docs, exact artifacts and explicit V1 go/no-go. |
| Publication | NOT AUTHORIZED | Separate user approval; no automatic tag or release. |

Recommended next checkpoint after human acceptance: prepare matching three-platform
candidates, then run installation/runtime and representative performance checks.
Do not require an unlimited fuzz campaign or a whole-machine collision engine to
accept the explicitly bounded scope; do not label unknown conditions safe either.

Developer ID/notarization, AUDIT3, FS4, PERF2, TIME2, broader GEOM-CORE, ProtoMAX/WJ,
VINYL and unsupported advanced/two-sided variants remain deferred/conditional under
the existing scope. UX8 remains a bounded recommendation. Existing refusals stand.

[Platform/workshop protocol](../v1-validation-2026-10-17/PROTOCOLE-V1.md) ·
[V1 scope](../FABCAM_V1_SCOPE.md) · [Release roadmap](../RELEASE_ROADMAP.md).
