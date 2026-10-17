# AUDIT3 — ivaCAM → FabCAM: Git History & Defect Provenance Retrospective

2026-10-17 · **PLANNED — NOT STARTED**.
Placement: **POST-V1 APPROVED RELEASE checkpoint**, not a V1 release blocker.
This plan does not establish that 1.0 is approved/released or authorize Git archaeology.
Existing release/audit identifiers, qualification evidence and open gates are retained.

## Trigger, dependencies and bounded scheduling

FabCAM 1.0 validated and explicitly accepted by the human → AUDIT3 → main FabCAM
1.5 ProtoMAX implementation campaign. The frozen accepted V1 commit/tag, release
scope and qualification records are the historical endpoint; record exact immutable
identities at launch, without creating a tag or assuming today's beta is that endpoint.
1.5 remains the next product-development priority. Previously authorized early
ProtoMAX feasibility research is not silently revoked or converted into implementation.

Dependencies: human V1 acceptance record; frozen V1 reference and retained evidence;
available upstream/fork history; Argon/Astra audit reports, correction commits and
regression records. Verify repository identities/divergence during AUDIT3 rather than
promote a documented working baseline into proven ancestry now.

Proposed time box: **up to three engineering days**, duration to confirm at launch.
Prioritize confirmed high-severity findings and representative areas first. At the cap,
deliver evidence/results plus explicit gaps; do not silently extend or delay ProtoMAX.
Any extension requires a human scheduling decision. Newly found critical regressions
are escalated immediately for separate authorized tracking/triage; never silently reopen
V1 scope, alter its evidence or mix corrective implementation into this retrospective.

## Future Git investigation and evidence contract

Identify exact original ivaCAM repository, relevant fork baseline and ancestry/divergence;
inspect commits/diffs/file history with git log/blame/diff and historical source reads.
Correlate original findings, confirmed causes, actual fixes and independent regressions.
When necessary/feasible, reproduce on isolated historical revisions with compatible
toolchains/dependencies, recording build failures and behavior limits. A missing toolchain
or failed historical build is not proof that a bug was absent. No current checkout
history rewrite, automatic upstream sync or modification of frozen evidence.

Function existence, blame attribution or authorship alone is insufficient. Separate
root-cause origin, enabling context/new caller, first demonstrable affected revision,
first observation and correction. State contract/applicability differences explicitly.

## Mutually exclusive confirmed-defect classification

Use one primary category per deduplicated confirmed causal defect. Apply rules in
this order, with supporting historical behavior/contract evidence:

1. **Inherited:** the accepted ivaCAM baseline demonstrably violates its applicable
   behavior/contract on a valid baseline workload; later integrations may reveal it,
   but discovery timing does not change origin.
2. **Introduced by FabCAM:** absent under the comparable baseline contract, and a
   post-fork causal change creates the defect, including a new caller violating an
   inherited API precondition. Merely leaving an old function unchanged is no defense.
3. **Exposed by FabCAM:** pre-existing latent behavior becomes problematic in a new
   valid use/integration/requirement, without evidence of a baseline contract defect
   or a defect-creating post-fork change under rule 2. Name both old behavior and new
   applicability; do not use this as a catch-all for introduced defects.
4. **Undetermined:** comparable historical behavior, contract or causal evidence is
   insufficient, conflicting or not reproducible within the time box.

For inherited code plus new callers: baseline already faulty → Inherited; caller
misuses defined contract → Introduced; legitimate new applicability exposes latent
limitation → Exposed; unresolved distinction → Undetermined. Separate independently
testable causes where justified, not every report/caller symptom into another bug.
Record origin of underlying behavior separately from first observable failure even
when primary category is Exposed. Document High/Medium/Low confidence with reasons;
low-confidence historical attribution should remain Undetermined if proof is missing.

## Inventory and quantitative report

Consolidate available Gemini Argon and Codex/Astra reports; preserve original wording
and verification outcomes with aliases/backlinks. One row per distinct confirmed issue:
unique ID, area, original finding, severity/rationale, confirmed root cause, affected
versions/evidence limits, correction commit(s) or Uncorrected, regression reference(s),
primary provenance, historical/source/test evidence and confidence. Keep duplicate
aliases and disputed/unresolved/refuted findings visible in separate tables. Refactoring,
performance suggestions and unverified findings do not count as confirmed defects.

Report distinct confirmed total; provenance counts/percentages over that explicit total;
severity/area distributions with stated population; confirmed/corrected/uncorrected
and unresolved findings; regression additions; substantial safety/correctness improvements;
performance improvements only with comparable benchmark evidence and known limits.
Separate unconfirmed findings from confirmed-but-uncorrected issues. Explain deduplication,
multi-area/severity policy, multi-test/multi-defect links, missing severity and how tests
are counted; do not count one regression repeatedly across issue aliases. State cutoff
and coverage, and ensure four provenance categories sum to the confirmed population.

No invented figures or historical attribution. These selective audits are not a random
sample: distribution is **not the overall defect rate or relative reliability** of either
project. Historical performance results retain their hardware/build/workload scope.

## Engineering retrospective and deliverables

Trace new capabilities, architectural decisions, geometry/CAM correctness, output safety,
FS, UX/desktop delivery, tests/qualification and maintainability to reviewed evidence.
Distinguish inherited facilities, FabCAM extensions and new work; preserve ivaCAM author/
source attribution and GPL obligations. Do not present partial/unverified behavior as
new qualified capability or software tests as physical validation.

Deliver detailed traceable technical report, concise executive summary, reconciled
provenance/severity/area tables and charts, engineering-evolution retrospective and
short factual public-facing draft. Charts use the same deduplicated data/denominators;
show Undetermined and confidence limitations. Public draft remains unpublished pending
separate authorization; no promotional reliability claims or implied upstream blame.

## Acceptance and hard stop

Required: release trigger evidenced; immutable historical references; reproducible
method/links or explicit gaps; mutually exclusive classifications/deduplicated inventory;
reconciled counts/denominators; qualification/benchmark limits and attribution preserved;
all five deliverables; time-box outcome and separate critical-regression escalation.
End at human report review. Audit completion does not itself authorize ProtoMAX coding,
publication, corrective work or changes to V1 release records.

Current task is documentation only: no Git history investigation, historical test run,
production change, commit, tag, issue, release, push or public publication.

**HARD STOP — awaiting human approval of the updated roadmap.**
