# FabCAM / FabApps — macOS Personal Alpha roadmap update

Documentation/planning snapshot **2026-10-24**, tracker `ivac-98bic`.
[Current release plan](RELEASE_ROADMAP.md) ·
[STATUS](../shopbot-modernization/STATUS.md) · [FabApps vision](../FABAPPS_VISION.md).
No implementation, build, installer, release, hardware or acceptance performed.

## Strategy and maturity boundaries

Previous planning aimed at a general V1.1 RC after broad three-platform software,
installed-runtime and physical qualification. The maintainer now approves
**FabCAM V1.1 — macOS Personal Alpha**, installed on their own Mac for controlled
personal evaluation, with progressive supervised ShopBot pilot qualification.
This is a deliberate product-scope decision, not a failure declaration or a waiver
of safety gates. Fast development is not product readiness.

Distinguish implementation, software qualification, personal operational evaluation,
supervised physical qualification, controlled workshop availability and public
release. Personal Alpha may remain software preparation/simulation only until
physical cases pass. Machine movement/air cuts require supervised QUAL-2 approval.
After meaningful use, Workshop Beta is a separate controlled-fablab human decision.
Public Release is deferred with no announced date or implied launch commitment.

Public packaging/signing/notarization, Windows/Linux installers and broad platform
acceptance are deferred, not abandoned; they are not immediate alpha blockers unless
required for the actual selected local install method. No unnecessary distribution
pipeline work. Actual macOS installation/native filesystem/persistence and CNC safety
cannot be replaced by a successful build or software mocks.

## Revised qualification and acceptance policy

Path: **QUAL-1A.4 → QUAL-1B → QUAL-1C macOS → QUAL-2 supervised pilot → Personal
Alpha scope acceptance**. Non-conflicting protocols can be prepared in parallel.
Bounded software-only personal evaluation need not wait for exhaustive hardware or
cross-platform coverage; affected evidence must still bind the actual source.

- QUAL-1A.4: complete the owner's portable arithmetic/JSON/P1/motion evidence and
  exact-source handoff; do not infer integration from prototype passes.
- QUAL-1B: preserve preflight checks, cancellation propagation, stale/partial-result
  refusal and unchanged non-cancelled output. Keep **200 ms p95** as the engineering
  target; historical **200.353 ms** remains failed. A characterized non-critical
  latency exception needs independent correctness/publication evidence, a named
  workflow and explicit human disposition. No threshold change or exception here.
- QUAL-1C: target the actual maintainer Mac. Verify startup, project lifecycle,
  import, local libraries/collections and transfers, explicit rates, generation/
  simulation, actual-value reports, export freshness/refusals and native permissions/
  dialogs. Preserve canonical CAM/ShopBot/SAFE-Z/NUMSAFE/provenance/output/security
  and automated gates. Observed development host is **arm64 / macOS 26.5.2** via
  `uname -m` and `sw_vers -productVersion`; confirm the intended installation target,
  actual installed configuration and local install/build method separately.
- QUAL-2: choose a small representative fixture set for WCS/setup/clearance and
  inspected non-cutting movement, Profile/Pocket, rough/finish tools, Drill/Spot/
  Chamfer, V-Carve and later supported cases. Two-sided work requires explicit
  registration preparation; no automatic dowel injection. Retain exact program,
  source SHA, configuration/rates, simulation/controller preview, observed physical
  outcome/anomalies and human decision per case. Untried families remain excluded.

Block unexplained safety-critical motion, wrong generation-affecting machine/tool
values, broken SAFE-Z/known-limit refusal, unsafe/stale publication, corrupted
persistence, critical target-Mac failures and unsupported physical claims. For
unresolved safety risk, narrow scope or keep a blocker. Minor cosmetics,
characterized non-critical latency, excluded families/platforms and optional
integration limits can be considered only through explicit scoped acceptance.

For each gap classify **product defect / known baseline failure / environment
limitation / unexecuted acceptance / explicitly accepted alpha restriction**.
Record actual outcome, evidence/risk, consequences, workflow restrictions and human
decision. An environment explanation is not proof of no product defect. Watcher,
cargo-deny and Chromium/CDP failures remain failed/blocked; no disabled test,
unexecuted PASS or silently accepted exception. Appropriate unrestricted-host
execution remains necessary where risk cannot otherwise be assessed.

## Current-source evidence and concurrency

Previous consolidation **`f9316ebadbc335bdfcefb47710c822acac3a0c0b`** remains only on
`codex/roadmap-fabspeed`, sibling `FabCAM-roadmap-fabspeed`; it is not integrated into
primary `FabCAM-v1.1`. This task extends that clean isolated documentation branch.
Primary was inspected at **`6148b1059ab6020f751069d5c3a7d64b8047fb6b`**, with the
same ten tracked user modifications and untracked historical evidence; dirty
STATUS/RELEASE_ROADMAP notes remain preserved by the prior snapshot/ledger.
No source/worktree/index/user-data cleanup or integration is performed.

QUAL-1A.4 is owned by chat **“Astra élevé”**, tracker `ivac-awe23`, isolated
`FabCAM-qual-1a4` / `codex/qual-1a4-production`. At inspection HEAD was
**`8e2c9ce427007f9c5a3787b34dd28647b8cdcf95`**, following numerical `c74f5e9` and
P1 `61c00ba`. A newly available **uncommitted provisional** report,
`docs/releases/QUAL_1A_4_PRODUCTION_INTEGRATION.md` in that sibling, records
conditional local integration readiness and **17/20** mandatory gates, not
unconditional release, installed-platform or machining acceptance. It names two
watcher failures, blocked advisory retrieval and Chromium/CDP failure. It also
records TEST1/C2 256/256 and retained safety/streaming/two-sided limitations.
These are the owner's reported results, not tests rerun by this task. Owner issue
remains IN PROGRESS and primary is still `6148b10`; no final committed receipt or
primary integration was verified. Do not close the milestone based on this draft.
The owner may complete it concurrently; future reconciliation must use its final
receipt, not overwrite it with this planning snapshot.

Reviewed references include current STATUS/roadmap and
[prior consolidation](ROADMAP_CONSOLIDATION.md),
[FS-RESET-3](FS_RESET_3_RESULTS.md), [QUAL-1](QUAL_1_SOFTWARE_QUALIFICATION.md),
[Accepted ADR-0030](../shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md),
[Proposed ADR-0031](../shopbot-modernization/adr/ADR-0031-optional-one-way-fabspeed-catalogue.md)
and [frozen V1 readiness](V1_READINESS.md). The primary-only QUAL-1A,
QUAL-1A.2 and QUAL-1A.3 reports retain their exact experimental limits described in
[the evidence inventory](ROADMAP_CONSOLIDATION.md#source-bound-evidence).
No report, accepted ADR or historical qualification is rewritten. Frozen
`main`/`origin/main` remain **`ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`**;
V1/RC1's historical three-platform requirements are not changed retroactively.

## FabApps portfolio and future integration

The dedicated [vision](../FABAPPS_VISION.md) keeps independent product roadmaps:
FabCAM blue CAM, FabSpeed magenta calculations/experiments/catalogues, FabVents
lime/green parametric perforations, FabParts amber/yellow **Electronic component
manager** (formerly KiPart Manager), FabWeave parametric 3D-print extrusion/G-code.
No suite-wide maturity or mandatory release dependency follows shared visual identity.

Evidence inspected outside this repository, read-only: `KiPartManager/docs/FABPARTS_RESTYLE.md`
and its owner handoff (restyling/rename delivered; off-screen Qt validation, native
review still limited); `FabWeave/README.md`, `STATUS.md` and `branding/README.md`
(v0.1 vase/CORE One pilot recorded, v0.2 in progress, blue/interlaced-wave identity
recorded as retained). FabWeave remains development/exploration, not a completed
or publicly released product; no new identity color is invented. FabSpeed and
FabVents README responsibilities were also inspected without inferring releases.
These independent repositories remain authorities for their own progress.

Recommend one primary stabilization effort—FabCAM—with at most one bounded
secondary effort as capacity permits. Future FabApps Design System starts with
principles/tokens/density/accessibility/icons/navigation/examples, not an automatic
shared code package. A future portfolio website presents actual use cases/maturity
and available docs/demos/releases; no website or shared-library work starts here.

INT-0/1/2/3 stay separate, likely V1.2 intent, not immediate publication or alpha
requirements. Preserve approved one-way FabSpeed → FabCAM business data, stable
shared/local IDs, persistent offline copies, CAM extensions, reviewed independent
operation values and no implicit attestation transfer/upstream CAM writes.
ADR-0031 needs no policy change: it already preserves these principles and OPEN
transport/storage/API decisions. No synchronization or suite coupling is implemented.

## Changed documents and handoff checks

Updated canonical STATUS/RELEASE_ROADMAP, track navigation, documentation entry,
V1-readiness routing and AGENTS release guidance; added FABAPPS_VISION and this
report. Prior consolidation/report/history and ADRs remain unchanged. Local commit
identity is supplied in the delivery message rather than self-embedded.

Checks: **143 relative links/anchors across eight changed documents**, no missing
target/anchor; balanced Markdown fences and reviewed tables; `git diff --check`;
milestone/product/scope consistency; all ten initial primary modified tracked-file
fingerprints unchanged. Historical local-only evidence dependencies from the
previous consolidation remain disclosed; no unrelated evidence is staged. No
extensive Rust/frontend tests or build runs for this documentation-only change.

Remaining decisions: final A4 source/human disposition, separate QUAL-1B authority,
target installation configuration/method and build permission, focused macOS runtime
acceptance, representative physical fixture scope/operator/preconditions, any
explicitly justified non-critical alpha exception, and final limited alpha acceptance.
Workshop Beta/public scope, INT contracts, shared design implementation and website
all require later independent decisions. Future application of this isolated commit
must semantically preserve primary user edits and A4's final handoff.

**HARD STOP after this planning/documentation commit.** No QUAL-1B, A4 production
edit, synchronization, shared UI library, website, version change, installer,
push/rebase/main merge/tag/release/publication or hardware operation.
