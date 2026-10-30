# FabCAM — current status

**ALPHA-4 accepted as bounded software-qualified implementation; locally consolidated.**
The 1315-file qualified inventory is unchanged. Selective cross-context values,
advisory provenance, atomic history, V10 persistence and strict matching are accepted.
Stepover remains unsupported; source physical attestation never transfers.
[Report and commit receipt](../releases/ALPHA_4_REVIEWED_PRESET_REUSE.md). Earlier pending statements below are historical.
No merge, push, installer, TOOL-META/FabSpeed integration or physical acceptance.
**HARD STOP after the three authorized local commits.**

**ALPHA-4 — reviewed preset reuse software-qualified; human acceptance pending (2026-10-30).**
Isolated `feature/alpha-4-preset-reuse`, accepted ALPHA-3.2 base `38fc911`.
Separate selective numerical copy into V10 Main / already-enabled Finishing;
strict tested-preset matching stays unchanged. Advisory origin carries no target
physical attestation. 20/20 mandatory gates PASS; 1659 Rust / 1559 frontend tests.
[Source-bound report](../releases/ALPHA_4_REVIEWED_PRESET_REUSE.md).
Earlier ALPHA-4 “planned/not authorized” statements below describe preceding
checkpoints. No commit, integration, installer, release or physical acceptance.
**HARD STOP after this checkpoint for human review.**

**ALPHA-3.2 accepted and locally consolidated.** Software `610d35d6031cb94e4b93473ce1cff661b2b85421`
exactly matches all 1,308 qualified files. Earlier pending-review statements are
historical. Bounded software acceptance only; native/workshop/physical exclusions
remain. No integration or next milestone. **HARD STOP.**

**ALPHA-3.2 — bounded implementation qualified; human acceptance pending (2026-10-30).**
ADR-0033 is approved; historical executable preservation was waived. The isolated
worktree adds explicit Walls / Floor / Both scope, checked axial coverage, reachable
floor proof, curves/simple islands and revision-safe UI. Project/cutting v10 stays;
new `pocket_finishing_v2` and capability are strict. Conservative geometry, cutter,
entry and resource refusals remain. [Report and limitations](../releases/ALPHA_3_2_POCKET_FINISHING.md).
All changes are uncommitted on baseline `81e8d47d3ca6860303d534393bad37056ea0cfb2`.
20/20 mandatory gates PASS; 1,659 Rust / 1,549 frontend tests, 79 real transport
cases and three actual flipped pairs. **HARD STOP for human acceptance.**
No commit, push, merge, installer, release, ALPHA-4 or physical operation.

**ALPHA-3.1 accepted and locally consolidated — 2026-10-30.** Human acceptance
covers the bounded polygon implementation and documented limitations.
Core `59d8e233fa54facb9873b7203068ed45ad291011`; software/tests `56743de13a1f66005d7e24519fc298e66bf9c224`;
all 1,302 qualified software files match the final fingerprint exactly.
[Acceptance receipt](../releases/evidence/alpha31/consolidation.json).
Earlier pending-review/uncommitted statements are historical. ALPHA-3.2 proceeds
under accepted ADR-0033; its current human gate is documented above.
No merge, push, installer, release or physical operation.

**ALPHA-3.1 — bounded polygon finishing implemented and software-qualified;
human acceptance PENDING.** Isolated `feature/alpha-3-1-generalized-finishing`,
uncommitted over accepted ALPHA-3 HEAD `1a70e503b8661aeb59b8d3bff56e5fde4139836c`.
[Source-bound report](../releases/ALPHA_3_1_GENERALIZED_FINISHING.md): useful convex
and concave Profile Inside/Outside and single-boundary Pocket finishing, with
continuous planned/decoded coverage proofs. V10/preset v2, Main-only output,
rectangle output and ShopBot guards are preserved. **20/20 canonical gates**,
1,645 Rust PASS / 1 intentional ignored worker, 1,542 frontend PASS / 1 optional
skip; 86 runtime cases and 23 retained output fixtures. Final 1,302-file fingerprint:
`1673ddb0e22994cb716b04711c4943c4635d707dc1956c6fa9901c15a6c1224c`.
Source curves/islands/holding extensions remain unsupported; some Zigzag coverage
and Spiral/complexity cases refuse. New polygon streaming buffers before publishing
validated bytes. Installed native/physical tests NOT RUN. **CONDITIONAL GO for
human review; HARD STOP before commits/integration.** Other worktrees preserved;
no ALPHA-4, TOOL-META integration, installer or publication.

**ALPHA-3 human acceptance recorded:** bounded first implementation accepted;
three focused local commits authorized. Software/tests `42be450e080d63ef83b632ab3ec9389e4095e126`
match the qualified 1,295-file fingerprint exactly. Documentation/acceptance receipt
records the commit handoff. Earlier pending-review statements below are historical.
No integration, push, installer or next milestone. Finishing limits and native/physical
qualification exclusions remain unchanged. **HARD STOP.**

**ALPHA-3 — implemented and software-qualified; human review PENDING.**
Isolated `feature/alpha-3-main-finishing` retains HEAD
`96181814c6f97ac19390dade7a0d3110b2f4d681`; implementation is uncommitted.
Approved project/cutting v10 and staged preset v2 now drive actual Main/Finishing
Profile/Pocket motions, readers, UI, reports and simulation attribution.
[Final report](../releases/ALPHA_3_MAIN_CUT_OPTIONAL_FINISHING.md) and
[Accepted ADR-0032](adr/ADR-0032-explicit-main-and-optional-finishing.md).
**20/20 canonical gates**, Rust 1,628 PASS / 1 intentional ignored worker,
frontend 1,542 PASS / 1 optional skip; 15 exact native/WASM runtime comparisons,
23 controlled output fixtures and actual browser qualification.
Finishing is deliberately bounded to one exact axis-aligned rectangle, flat cutter,
direct entry, no holding/lead complications; positive allowances >=0.05 mm.
Pocket Finishing re-clears its reachable target; it is not optimized rest machining.
All unsupported proofs refuse. Main-only geometry and ShopBot guards are retained.
Installed native UI/physical tests NOT RUN. Other worktrees and private evidence
preserved. **HARD STOP before implementation commits; no integration, push or installer.**
The original RED preflight is archived, not rewritten as a passing historical result.

ALPHA-1 checkpoint accepted by the user; native/physical gates remain separate.
ALPHA-2 protocol separation is approved and implemented locally on isolated
`fix/alpha-2-geometry`, software commit `58ddde72295a01cbe43ce0270c98e1f79f2f1039` and the qualified source inventory in the
[qualification report](../releases/ALPHA_2_GEOMETRY_REPAIR_RESULTS.md).
Explicit diagnostic/snapshot v2 retains drawing/CAM v1 and guarded project v9.
Only proven isolated line-gap edits and unchanged single-owner placements are
accepted; overlapping/nested envelopes and all unproven cases refuse. The private
drawing is diagnosed, not repaired or CAM-qualified. **Bounded implementation accepted;
three local commits completed through `96181814`. No integration.** ALPHA-3 was
subsequently authorized separately, with its current version decision above.


ALPHA-1 is completed locally on isolated `fix/alpha-stock-diagnostics`, executable
source `b0c2f208bc0de1d38f733d378abbb0b85600785d`; primary V1.1 is not advanced.
[Stock/import/diagnostic report](../releases/ALPHA_1_STOCK_IMPORT_DIAGNOSTICS.md)
records source-bound software gates and browser observations separately from native
and human acceptance. No new installer, push or hardware test. ALPHA-2/3/4 and
TOOL-META integration remain separate; **HARD STOP for human review**.


Current focus: **FabCAM V1.1 — macOS Personal Alpha**. Latest integrated local
software/build source: **`1f353ef3b774382f63e67f93a9bb3a4652007de2`**. Drawing drag,
proportional dimensions, fixed manual stock, pocket diagnostics and header UI are
integrated, with **20/20 canonical gates**, 23 exact output references and a rebuilt
local arm64 app. [Current report and app receipt](../releases/V11_LOCAL_INTEGRATION_2026_10_24.md)
own this checkpoint. **Native human acceptance remains OPEN.** SEC-1's
[older source/report and publication receipt](../releases/SEC_1_REPOSITORY_CONSOLIDATION.md)
remain historical evidence; the new integration is committed locally, without push.
Public release, main merge, version promotion, FabSpeed integration and hardware
operation remain excluded.
[Release plan](../releases/RELEASE_ROADMAP.md) · [FabApps vision](../FABAPPS_VISION.md).

| Work | Current disposition |
|---|---|
| FS-RESET-3 | COMPLETED: explicit v9 actual-role parameters, neutral CAM safety and tested collections; legacy recommendation engine removed |
| QUAL-1A.4 | COMPLETED software integration: `8e2c9ce`, report `f2b0427`; [numerical/motion limits](../releases/QUAL_1A_4_PRODUCTION_INTEGRATION.md) retained |
| QUAL-1B | COMPLETED software cancellation: `fd8fed6`, report `aeae820`; [fixed campaigns](../releases/QUAL_1B_COOPERATIVE_CANCELLATION.md) meet 200 ms p95, without a universal GUI latency claim |
| QUAL-1C | BUILT / SOFTWARE TESTED: `700de73`, report `9520d98`, [19/20 gate](../releases/QUAL_1C_MACOS_PERSONAL_ALPHA.md); app not launched, native human acceptance pending |
| SEC-1 | QUALIFIED: `1350953`, 20/20 canonical gates, current audit PASS, 77 native/WASM cases exact, TEST1 256 PASS; rebuilt local arm64 app. Publication outcome/exact remote SHA in the report's post-push receipt |
| Local placement integration | INTEGRATED / SOFTWARE QUALIFIED: `1f353ef`, 20/20 gates, Rust 1,594 PASS / 0 FAIL / 1 intentional ignored worker; frontend 1,510 PASS / 1 optional skip; native build ready for human testing |
| Optional V1.2 | PLANNED one-way FabSpeed → FabCAM; [ADR-0031](adr/ADR-0031-optional-one-way-fabspeed-catalogue.md) Proposed, no implementation |

Tokio 1.52.2's RUSTSEC-2026-0386 is addressed by the minimal lockfile update;
1.54.0 is excluded because of the separate Interval race. The security commit
preserves every other resolved package. Focused tests: native Tauri 15, HTTP 21,
file export 11, CPS 55, 23 unchanged output references; full audit and strict
affected pedantic Clippy pass. SEC-1's original full qualification: Rust 1,592 PASS / 0 FAIL /
1 intentional ignored worker; frontend 1,498 PASS / 1 optional locale skip.
Native/WASM replay and the 23 fixed CLI references are exact.

The earlier watcher failures remain recorded as sandbox failures; the unchanged
five watcher tests passed on the Mac in QUAL-1C and SEC-1 focused qualification.
Actual native dialogs, file round-trips and cancellation recovery remain untested
in the installed app. Static ACL checks or mocks cannot close them. Two-sided
publication is not pair-atomic; CLI streaming does not acquire full-program
simulation qualification. Automatic dowel injection remains excluded.

MAT-1, tested collections and generic machine/tool libraries remain retained.
Drawing placement has automated and browser replay evidence; native human UX
acceptance is pending. The header translation failure is fixed without weakening
the guard. No combined-source pocket acceptance is added. All 8,471 inventoried
unrelated local files/evidence are preserved; the current report separates this
integration from SEC-1's earlier dirty-source preservation receipt.

Frozen `main`/V1 remain `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca` with their
original acceptance limits. Dated reports are unchanged; the earlier
[roadmap consolidation](../releases/ROADMAP_CONSOLIDATION.md) records planning
history rather than the current A4/B/C status.

Next product milestone: **Personal Alpha — Actual macOS Launch & Native Workflow
Acceptance**, using the separate macOS test account and the newly rebuilt
[current local bundle/procedure](../releases/V11_LOCAL_INTEGRATION_2026_10_24.md#preservation-and-next-human-checkpoint).
Older QUAL-1C/SEC-1 bundles remain preserved. No launch in the usual application-data
profile. Supervised QUAL-2 follows only by separate authorization.
**HARD STOP after local integration, native rebuild and report. No push.**
