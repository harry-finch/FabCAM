# FabCAM — current status

Snapshot **2026-10-21**, recorded on `FabCAM-v1.1` only.
[Release plan](../releases/RELEASE_ROADMAP.md) ·
[V1 readiness](../releases/V1_READINESS.md) ·
[V1.1 inventory, qualification and policy](../releases/FABCAM_V1_1_ROADMAP_AND_BRANCH_2026_10_21.md).

| Line | Source and state | Next decision |
|---|---|---|
| V1 stabilization / `main` | `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`; RC1-ef807cf-39492251757, beta.3/pipeline63; CI and candidate jobs succeeded | Clean-host installation/runtime and supervised ShopBot evidence, remaining human gates, explicit V1 acceptance |
| `FabCAM-v1.1` | Base/merge-base `ef807cf`; implementation `0075317`, launcher `b417d3c`, documentation `6d4cff0`; existing changes already committed locally | Human UX acceptance `ivac-tnpyz`; frontend size failure `ivac-fpx7n`; not merge-ready |
| New V1.1 work | PRESET-1 library, PRESET-2 Duplicate & Adapt, MAT-1 taxonomy are planned only | Review maintenance first, then authorize PRESET-1 read-only slice separately; scaling assistant remains future research |

RC1 [CI](https://github.com/harry-finch/FabCAM/actions/runs/39487569970) and
[candidates](https://github.com/harry-finch/FabCAM/actions/runs/39492251757) belong to
`ef807cf`, not the V1.1 implementation. The detailed
[local RC1 receipt](../releases/V1_RC1_CANDIDATES_RESULTS_2026_10_20.md) is retained
untracked; its availability in another checkout requires separate evidence transfer.
macOS ad-hoc/non-notarized, Windows unsigned and Ubuntu unsigned-package limits
remain; successful packaging does not establish installed or physical acceptance.

V1.1 verification at `6d4cff0`: frontend **1474 PASS / 1 skip / 1 FAIL**;
unchanged size guard rejects OperationsList 1008 and FeedsSpeedsSection 1010 lines.
Both were within budget at RC1; the overages originate in V1.1 `0075317`.
Targeted Rust: Feeds & Speeds 113 integration + 1 doc PASS, adapter 19 PASS,
tool/process 6 PASS, native source packs 7 PASS. No full integrated V1.1 or
installed-platform qualification is claimed. See the report for exact commands.

Named packs, physical/reference identity separation, library-tool assignment and
compact UI changes are implemented with bounded tests, awaiting human acceptance.
[Existing implementation report](../releases/FABCAM_V1_1_UX_2026_10_21.md) and
[ADR-0027](adr/ADR-0027-lab-and-reference-identities.md) retain their limitations,
including partial native removal on I/O failure and legacy identity ambiguity.

V1 qualification remains source-bound. The [readiness register](../releases/V1_READINESS.md)
retains human review, supported/refused Pocket RPM, resource/cancellation,
provenance, platform and workshop gates; this task closes none of them.
[SEC1 follow-ups](../releases/SEC1_FOLLOWUP_ROADMAP.md) stay independently scoped.
FS0–FS3.9 and M0–M4 remain delivered within recorded limits; FS4 is deferred for
software development but physical evidence is required before real recommendation
claims. [Historical status](history/STATUS_2026_10_20.md) and dated reports retain
previous checkpoint facts; older no-remote-CI/candidate statements are historical.

Only demonstrated V1 acceptance-blocking defects belong on stabilization, with
focused regression tests and renewed exact-SHA qualification. Features stay on
`FabCAM-v1.1`; after V1 acceptance/release decisions, review any V1 hotfix delta
before integration and qualify the combined V1.1 source independently.

**HARD STOP after planning and local documentation commits.** No PRESET/MAT
implementation, extraction, push, merge, tag, release, packaging or hardware action.
