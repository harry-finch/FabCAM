# TEST1 — bounded property qualification plan

Current state — 2026-10-18: implemented bounded software evidence consolidated on
pipeline61; [current matrix and limits](FINAL_CONSOLIDATION_2026_10_18.md). Human review ivac-l6ouh OPEN;
TEST1/SAFE-Z overall remain open pending scoped acceptance. No commit/publication.
The contracts below retain their original numerical budgets and scope. Historical
proposal/reproduction text is context, not an assertion that implementation is absent.

## Entry and scope freeze

Finish/review current audit-correction checkpoints, triage unresolved safety/correctness,
and establish integrated regression evidence on the combined final source before TEST1.
Freeze the exact tested revision/dirty-state manifest, supported operation/input contracts
and budgets. Existing pipeline54 desktop candidates do not contain later pipeline56
corrections; do not transfer their checks to the newer source. Packages are rebuilt/
requalified after required source corrections and bounded TEST1, then existing platform/
ShopBot physical/RC/human V1 acceptance gates apply.

Before coding, deliver written property contracts, generated families, independent
oracles and CI policy for human scope approval. Reuse current tests and dependency
policy; `proptest` is a candidate, not selected/added by this document. Fuzz only if a
specific parser/geometry boundary justifies it. Extended coverage/fuzzing is POST-V1.

## Initial V1 property families

| Phase | Bounded generated domain / promised property | Independent evidence and limits |
| --- | --- | --- |
| TEST1-A — geometry and source identity | Bounded line/arc chains, valid/degenerate/nonfinite cases, multiple sources: membership/ordinals preserved through chaining; analytic arc extrema contained in bounds; qualified transforms/frame round trips; ambiguous topology never silently rebinds operations | Independent sampled/analytic extrema and explicit identity/provenance oracle, not positional equality or duplicate production helper; rejection/canonical handling according to actual contract |
| TEST1-B — cutter clearance and machining intent | Small operation-specific Profile/Pocket/V-Carve inputs in supported topology/tool/entry domains, plus deliberate unsupported cases; protect independently defined regions and preserve refusal | Pocket islands/protected material, Profile chosen side/allowance and qualified V-Carve local envelope use different predicates; no universal cutter-distance rule or full-area V-Carve claim |
| TEST1-C — cache equivalence | Same qualified requests cold/disabled versus enabled/hit; one-at-a-time mutation of relevant source/transform/tool/depth/operation/machine/post inputs and repeat/restore | Compare complete qualified results and refusals with source mapping; exact output where promised, explicitly bounded geometric equivalence only where contract permits; no unrelated-input hits |
| TEST1-D — machine-output consistency | Supported feed/plunge and coordinate constraints, qualified operation requests and rejected/cancelled states; decoded OpenSBP agrees with intended movement/frame/rate | Existing independent decoder/reference, actual supported post/tool subset; relevant frontend lifecycle checks establish stale/rejected results cannot export, not Rust output tests alone |

First deliverable selects a small named property set from these four families, not
all geometries/operations/settings. Existing Tab and applied-cutting/cache regressions
remain integrated evidence; broad generated tab/FS/Boolean matrices are future expansion
unless a confirmed V1 risk justifies a specific bounded property. A newly found defect
needs reproduction, triage and an approved bounded fix or honest unsupported scope;
never change code just to satisfy a false or overbroad property.

## Contract requirements for every property

Specify valid/adversarial domain, preconditions, input/segment/iteration budgets,
expected result or refusal, oracle independence, exact versus numerical comparison,
units and numerical error budget. Keep numerical/approximation/output-rounding and
physical machining tolerance distinct. Do not infer bitwise identity through
discretization or from equivalent topology. In particular, exact associativity of
DexelField is **not adopted**: audit dexel/grid/sweep/ordering semantics first; only
justified bounded/monotone or other proven properties could enter later scope.

Record deterministic seeds, generator/framework versions and fixture hashes. Shrink
failures, verify the minimal counterexample independently, retain raw seed/input and
turn the confirmed failure into a permanent regression. Preserve source IDs, saved
manual cutting values, tool/holder distinctions and current capability/refusal rules.
Separate flaky harness/global-cache interference from machining failures; do not hide
either by reducing assertions or treating a skipped case as PASS.

## Proposed bounded execution policy — approve before implementation

CI: fixed seeds, initially 32 generated cases per approved property and bounded
geometry size, with a proposed total property-stage wall cap of two minutes on the
reference runner. Freeze exact cases/runner limits after measuring the test harness;
an overrun is a reported policy failure/review, not silently truncated successful proof.
Keep release integrated/regression tests in addition to this generated layer.

Local extended mode: same named property families with explicit seed/case-count
configuration and proposed ten-minute cap (initially up to 512 cases/property), plus
replay of saved counterexamples. Publish one reproducible command/recipe at implementation
handoff; none is claimed to exist now. Longer fuzzing/extra domains remain optional
POST-V1, separately scheduled. Native/WASM applicability and job isolation must be
stated; no promise of exhaustive proof from a random sample.

## Acceptance and release evidence

1. Human-approved bounded contracts/infrastructure/families/budgets and CI policy.
2. Implemented deterministic CI subset covers A/B/C/D with independent oracles,
   meaningful refusals and reproducible local extended/replay instructions.
3. Failures minimized/classified; confirmed release-critical failures corrected and
   requalified, or affected unsupported scope explicitly approved/refused.
4. Evidence records revision/toolchain/platform/backend/seeds/counts/durations,
   outcomes/skips, known gaps and exact/approximate equivalence policy.
5. Combined-tree integrated regressions, relevant OpenSBP/preview/stale-export and
   cache isolation checks pass with no numerical/output/serialization weakening.
6. Human software review accepts the **bounded subset**, not universal correctness
   or physical qualification. Package/platform/workshop gates remain separate.

States stay distinct: PLANNED → IN PROGRESS → SOFTWARE QUALIFIED → HUMAN REVIEW
PENDING/accepted for scope; physical qualification and RELEASE ACCEPTED need their
own evidence. No test run/qualification or new dependency in this planning task.

**HARD STOP — awaiting human approval of contracts/scope before TEST1 implementation.**
