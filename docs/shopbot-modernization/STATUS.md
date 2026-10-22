# FabCAM — current status

## Current checkpoint — FS-RESET-1, 2026-10-22

The maintainer has authorized separating FabSpeed calculation from FabCAM CAM/presets.
[Architecture audit](../releases/FS_RESET_ARCHITECTURE.md) and
[proposed ADR-0030](adr/ADR-0030-fabcam-fabspeed-separation.md) are ready for human review.
Audited code: `FabCAM-v1.1` at `e48241c89dbd911841eaf93dc72553f2763aa45f`.
Documentation only; baseline checks: core adapter 30, setup resolver 21, frontend 59 PASS.
No production change, source-pack reset, physical operation or V1 modification.
**HARD STOP for architecture approval; FS-RESET-2/3 have not started.**

The snapshot below records the preceding PRESET/MAT-1 checkpoint and retained independent
gates; it does not authorize continued recommendation-system development.


Snapshot **2026-10-21**, recorded on `FabCAM-v1.1` only.
[Release plan](../releases/RELEASE_ROADMAP.md) ·
[V1 readiness](../releases/V1_READINESS.md) ·
[V1.1 inventory, qualification and policy](../releases/FABCAM_V1_1_ROADMAP_AND_BRANCH_2026_10_21.md).

| Line | Source and state | Next decision |
|---|---|---|
| V1 stabilization / `main` | `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`; RC1-ef807cf-39492251757, beta.3/pipeline63; CI and candidate jobs succeeded | Clean-host installation/runtime and supervised ShopBot evidence, remaining human gates, explicit V1 acceptance |
| `FabCAM-v1.1` | PRESET API `5ca5632`, guarded native copy `bed3a92`, integrated frontend `b8deaef` | PRESET-1/2/3 software/UX accepted; local commits only, no push/merge/release |
| Next feature | MAT-1 material families, variants and synonyms remains planned | Requires a separate start instruction; no identity remapping or machining equivalence inferred |

[PRESET-4 final handoff](../releases/FABCAM_PRESET4_EXPERIMENTAL_RECIPES_2026_10_21.md):
WASM Profile null serialization fixed (`b0920ca`); approved recipe authoring and trial UI
consolidated (`be9720a`). Actual v8 save/reopen, modified status, Undo/Redo, Browse and
480px keyboard/scroll replay PASS. Frontend 1556 PASS / 1 existing skip; core adapter 30
and WASM crate 24 PASS; type/lint/build/Clippy PASS. **HARD STOP for final human UX
review `ivac-jwwom`**. No physical qualification, push, release or next milestone.

[PRESET integration receipt](../releases/PRESET_INTEGRATION_2026_10_21.md): frontend **1505 PASS / 1 existing skip**,
WASM package guards **10 PASS**, source storage **8 PASS**, Feeds & Speeds
**119 + 1 doctest PASS**, core adapter **20 PASS**; type/lint/build, native Clippy,
Rust formatting and schema/client synchronization PASS. User validation closes
PRESET-2 `ivac-1oe9l` and PRESET-3 `ivac-ms6rz` within their bounded software/UX scope.
The earlier branch-wide identity workflow checkpoint `ivac-tnpyz` and independent
V1/platform/physical gates are not closed by this integration.

RC1 [CI](https://github.com/harry-finch/FabCAM/actions/runs/39487569970) and
[candidates](https://github.com/harry-finch/FabCAM/actions/runs/39492251757) belong to
`ef807cf`, not the V1.1 implementation. The detailed
[local RC1 receipt](../releases/V1_RC1_CANDIDATES_RESULTS_2026_10_20.md) is retained
untracked; its availability in another checkout requires separate evidence transfer.
macOS ad-hoc/non-notarized, Windows unsigned and Ubuntu unsigned-package limits
remain; successful packaging does not establish installed or physical acceptance.

Historical V1.1 verification at `6d4cff0`: frontend **1474 PASS / 1 skip / 1 FAIL**;
unchanged size guard rejects OperationsList 1008 and FeedsSpeedsSection 1010 lines.
Both were within budget at RC1; the overages originated in V1.1 `0075317`.
The current PRESET-1 maintenance extraction restores the unchanged guard;
[latest checks and source limits](../releases/PRESET_1_READONLY_2026_10_21.md).
Targeted Rust: Feeds & Speeds 113 integration + 1 doc PASS, adapter 19 PASS,
tool/process 6 PASS, native source packs 7 PASS. No full integrated V1.1 or
installed-platform qualification is claimed. See the report for exact commands.

Named packs, physical/reference identity separation, library-tool assignment and
compact UI changes are implemented with bounded tests; PRESET acceptance is recorded above.
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

**Current checkpoint:** PRESET-1/2/3 accepted and committed locally; see the
[integration receipt](../releases/PRESET_INTEGRATION_2026_10_21.md). No push, merge, tag, release,
packaging or hardware operation. MAT-1 was subsequently authorized; see the current checkpoint below.

## MAT-1 implementation checkpoint — 2026-10-21

Architecture approved; implementation `13b8a95` adds shared presentation/search only.
[Report and screenshots](../releases/FABCAM_MAT1_MATERIAL_TAXONOMY_2026_10_21.md).
FS120/frontend1512 PASS (one existing locale skip); types/lint/WASM/build PASS.
Unknown private catalogs retain source/name/ID browsing. No resolver/schema/V1 changes.
**HARD STOP for final human UX acceptance.** Optional local classification and scaling
remain separate future work. No push, release or hardware action.
