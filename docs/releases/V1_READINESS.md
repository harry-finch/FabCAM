# FabCAM V1 readiness

## Frozen V1 and current feature-branch routing

V1 stabilization/RC1 remains `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`,
RC1-ef807cf-39492251757 (beta.3/pipeline63). Its
[CI](https://github.com/harry-finch/FabCAM/actions/runs/39487569970) and
[candidate workflow](https://github.com/harry-finch/FabCAM/actions/runs/39492251757)
completed successfully; the retained local RC1 receipt `V1_RC1_CANDIDATES_RESULTS_2026_10_20.md`
is untracked evidence, not included by this documentation commit. Signing/platform
and supervised workshop limits remain; no final V1 acceptance is recorded here.

The separate `FabCAM-v1.1` branch now uses explicit role parameters, project v9,
local tested collections and independent CAM validators after FS-RESET.
[Current STATUS](../shopbot-modernization/STATUS.md) and [qualification roadmap](RELEASE_ROADMAP.md)
own its current gates. QUAL-1A.4/1B software work is complete; QUAL-1C built the
macOS app with native user acceptance still pending. SEC-1 owns the current
Tokio correction/consolidation and final-source qualification.
Historical FS/PRESET implementation and size-guard snapshots remain in their dated
reports. Its current target is a limited **macOS Personal Alpha**, followed by
supervised ShopBot cases; general public/fablab distribution and Windows/Linux
qualification are deferred for V1.1, without changing this frozen V1 register.
Optional V1.2 FabSpeed integration is not a V1/V1.1 blocker. No V1 scope,
freeze, platform/physical acceptance or release state changes in this cleanup.

## Historical V1-FREEZE register — retained evidence and gates

The following **2026-10-20 pre-RC snapshot** preserves its original source-bound
facts and pending decisions. Its no-remote-CI, db5bde5 freshness and pre-build
instructions are superseded for RC1 by the exact-source receipts above; gate states
are historical snapshots and are not closed or re-certified by this planning task.

Snapshot **2026-10-20**, V1-FREEZE review checkpoint, beta.3/pipeline63.
[Exact final source and aggregate qualification receipt](V1_FREEZE_READINESS_2026_10_20.md)
controls freeze readiness; human freeze review `ivac-4k4qx` remains OPEN.
[SEC1 frozen-source results](SEC1_REMEDIATION_RESULTS_2026_10_20.md): 20/20 local gates;
CPS external execution refused; promotion disabled, remote workflows not run.
[Current implementation receipt](NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md#final-qualification-receipt);
previous integrated engineering source **`2c6e4e1`** is retained below.
[Reconciliation](PROJECT_STATE_RECONCILIATION_2026_10_20.md) records the tested integration revision.
[STATUS](../shopbot-modernization/STATUS.md) · [Release plan](RELEASE_ROADMAP.md) ·
[Accepted V1 scope/matrix](FABCAM_V1_SCOPE.md) · [Report history](HISTORY.md).

**Not a release acceptance.** Each result belongs to the source/configuration named
in its report. No current-source regression, installed-platform or physical result
is inferred from historical passing tests or from a closed engineering issue.

## Current acceptance and blocking register

Tracker states below were read on 2026-10-20; they are a snapshot, not a live mirror.
The task's human decision must be recorded separately from implementation completion.

| Gate | Current disposition | Required next evidence / decision |
|---|---|---|
| V1-FREEZE Option A `ivac-zdgjf` | Documentation reconciliation completed in isolated freeze branch; [supported/refused Pocket matrix](V1_POCKET_RPM_QUALIFICATION_2026_10_20.md) qualified with deterministic controls | [Final qualification/source receipt](V1_FREEZE_READINESS_2026_10_20.md); `ivac-2zll` scope and other human reviews remain pending; no freeze/build approval inferred |
| SEC1 `ivac-dqlol` / review `ivac-1mqti` | SEC1-01/04/08 implemented and locally qualified; source-bound workflow verification tested offline | [Results](SEC1_REMEDIATION_RESULTS_2026_10_20.md); remote CI, operational human protection and any promotion activation remain separate |
| NUMSAFE3 `ivac-fh2av` / review `ivac-zulb3` | Approved budgets implemented; frozen-source 16/16 gates, 1654 Rust tests, 256 TEST1 and 136 byte-identical outputs | [Report](NUMSAFE3_RESOURCE_EXPORT_RESULTS_2026_10_20.md); human review OPEN; no pair-atomic, universal latency, installed-platform or physical claim |
| NUMSAFE / NUMSAFE2 `ivac-knphk` / `ivac-o9gwj` | CLOSED; accepted and committed as `e1a0c95` / `2c6e4e1` | [NUMSAFE](NUMERICAL_RUNTIME_SAFETY_RESULTS_2026_10_20.md), [NUMSAFE2](NUMSAFE2_PASS_LIMITS_RESULTS_2026_10_20.md); retain streaming partial-file, specialized-loop and mid-op cancellation limits |
| Import / ADR-0026 `ivac-feder` | In progress; implemented in inspected source, [correctness](IMPORT_CORRECTNESS_RESULTS_2026_10_20.md) / [UX](IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md) evidence recorded | Human UX/architecture acceptance; retain unit/legacy/refusal boundaries |
| Export/load ADR-0024 `ivac-3i00w` | Human review OPEN; two engineering defects closed | Review [results](UX_SAFETY_LOAD_RESULTS_2026_10_19.md), full-program authority and limitations; no return to playback-dependent qualification |
| Provenance ADR-0025 `ivac-b0js3` / `ivac-b0js3.1` | In progress; [implementation](MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md) and [follow-up](ADR0025_FOLLOWUP_RESULTS_2026_10_19.md) recorded | Human checkpoint review; back-face fixture/dowel registration remains explicitly unqualified |
| UX D `ivac-he4lt` / consolidation `ivac-amibd` | Separate human reviews OPEN | Review [D](UX_V1_LOT_D_2026_10_19.md) / [consolidation](UX_V1_CONSOLIDATION_2026_10_19.md) with the later ADR-0024 corrections, not as two still-unfixed defects |
| Large pocket `ivac-53ky9` / cancellation `ivac-42ylx` | Conditional reserves OPEN | Representative packaged-runtime measurements; no unlimited-size or universal200ms guarantee; freezes/OOM/wrong output/partial success remain blocking observations |
| Combined-source software gate | [Integrated validation](PROJECT_STATE_RECONCILIATION_2026_10_20.md#integrated-validation) owns exact commit, exit status and gate results | Passing software tests does not accept native installers or physical machining |
| Release-gate portability `ivac-5yxyu` | SEC1 portable byte counts, checked gzip/measurement failures and invalid-budget refusal qualified without wrapper | [SEC1 results](SEC1_REMEDIATION_RESULTS_2026_10_20.md); historical receipts retain their original environment |
| Candidate freshness / CI | Existing [beta.3 artifacts](beta3-delivery-2026-10-18/README.md): db5bde5/pipeline61 | Separately authorized matching builds and integrity records after source/human gates; no new candidates created here |
| Clean installation / runtime | Current-source macOS/Windows11/Ubuntu qualification not established here | Real clean-download/install/upgrade/runtime records, actual OS versions and data preservation; CI build is not desktop acceptance |
| ShopBot workshop / physical | No current release-level physical qualification inferred | [Operator protocol](beta3-delivery-2026-10-18/PROTOCOLE-V1.md), controller preview, supervised air/material tests and measurements; retain bounded M4 evidence |
| Documentation `ivac-yg3sm` / guide `ivac-7w1e1` | Completed commits integrated; reviews remain OPEN | Integration authorization does not close review gates or approve retention changes |
| Feeds & Speeds | FS0–FS3.9 delivered; bounded acceptance recorded in [FS matrix](PROJECT_STATE_RECONCILIATION_2026_10_20.md#feeds--speeds-milestone-matrix) | FS4 deferred for software development; approved minimal trials before real ShopBot recommendation claims remain required; ShopBot RPM transition `ivac-2zll` remains OPEN |
| RC / V1 / publication | No release or publication authorization | Exact supported matrix, reviewed evidence/limits, explicit RC/V1 decision and separate publication approval |

## Closed earlier checkpoints, retained limits

Tracker `ivac-l6ouh`, `ivac-4z6lh`, `ivac-j09eh` and `ivac-8371y` are CLOSED in this
snapshot. Their [final consolidation](FINAL_CONSOLIDATION_2026_10_18.md),
[TEST1](TEST1_RESULTS_2026_10_18.md), [SAFE-Z](SAFE_Z_EXTENDED_RESULTS_2026_10_18.md)
and [publication guard](ADR0023_PUBLICATION_GUARD_RESULTS_2026_10_18.md) retain their
bounded domains, failed/resource cases and qualification limits. Do not reopen them
merely by copying an old OPEN banner, or extend them to later source automatically.
The [2026-10-18 acceptance record](ACCEPTANCE_CANDIDATES_2026_10_18.md) is preserved.

## Deferred and conditional scope

[SEC1-01–09 disposition matrix](SEC1_FOLLOWUP_ROADMAP.md) retains conditional
security boundaries and post-V1 hardening; existing human gates remain independent.

Developer ID/notarization, AUDIT3, FS4, PERF2, TIME2, broader GEOM-CORE, ProtoMAX/WJ,
VINYL and unsupported advanced/two-sided variants retain their existing status in
[the release plan](RELEASE_ROADMAP.md). UX8 is bounded/conditional. GEOM1/FS acceptance is recorded in the [2026-10-18 decision](ACCEPTANCE_CANDIDATES_2026_10_18.md);
historical OPEN report headers are not current tracker status.

After this checkpoint, review the remaining gates and seek separate matching-candidate
authorization. **HARD STOP after integration/reconciliation**. Evidence retention/relocation needs its own approval.
