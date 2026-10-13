# FS3.9 — Post-milestone review and optimization proposal

Status: sensitivity audit approved by the user and completed. Production smoothing remains a proposal, not implemented.

## Assessment

The software resolves the reported metric-diameter gap without inventing source
rows: normalized derivation, immutable originals, bounded applicability and visible
quality are now separate from evidence. Six exact source cases remain unchanged;
explicit Apply/manual save/reopen and private-data boundaries are tested.

Coverage remains narrow: compatible 1F upcut flat tools in softwood/MDF, with an
explicit machine-class transfer. More permissive matching does not supply missing
2F, compression, plastics, finishing or surfacing knowledge. ReferenceData remains
reference evidence; physical suitability is unmeasured while FS4 is deferred.

The separate ShopBot RPM-transition defect (ivac-2zll) is a practical export issue
and should be prioritized on the milling/post track. It must not be hidden by
changing finishing values in the recommendation layer.

## Recommended optimization: characterize diameter sensitivity

Before indexing or adding more records, add a reproducible developer-only sweep of
requested diameter around 10%, 25%, 50% relative distance and candidate/interpolation
switches. Current code deliberately changes the small-tool factor from q to q²
above10%. At that boundary q≈0.909, so the factor changes from≈0.909 to≈0.826:
a roughly9.1% decrease before clamps/rounding. This is an analytic policy consequence,
not evidence of an unsafe cut or an observed final-output discontinuity in every case.
Actual impact depends on the winning candidate, interpolation and hard limits.

Proposed deliverable: a deterministic table/plot with source selection, quality,
RPM/feed/plunge/DOC and every transition; a small boundary regression suite using
approved references and clearly synthetic test fixtures. No production UI needed.

Acceptance criteria:
- repeatable results independent of candidate order;
- exact existing recommendations remain unchanged;
- hard constraints, evidence and saved overrides remain intact;
- every recommendation/refusal or method transition is explained;
- observed output jumps are quantified rather than assumed;
- only after review, consider a continuous conservative attenuation policy if the
  measured transitions justify it, with a new policy version and unchanged saved values.

This improves predictability and reviewability at low scope. Performance indexing
is premature for six bundled profiles. Additional manufacturer imports and physical
Sorbonne trials remain separate decisions; FS4 stays deferred/nonblocking.

## Approved audit results — 2026-10-13

[Full measurements](FS3_9_SENSITIVITY.md), [all 11997 samples](FS3_9-sensitivity.csv).
Three modes, DOC1/DOC10 and synthetic hard-cap contexts; 1333 diameters per context.
Two independent executions produce byte-identical CSV. Four boundary/sweep tests
pass, including record-order reversal throughout all modes, hard caps and evidence.
The complete FS suite now passes98 tests +1 doctest; Clippy all-targets and fmt pass.
Existing exact-reference regressions remain included. No production resolver, source
pack, UI, persistence or post code changed in this audit.

Prudent, DOC10, no supplied limits, observed adjacent samples:
- Around2.886364mm: feed867→954 (+10.03%); DOC also≈+10%, from q²→q.
- Around4.233333mm: feed787→1239 (+57.43%), RPM16500→19500 (+18.18%):
  a second eligible anchor activates interpolation.
- Around5.772728mm: feed1516→1472 (-2.90%); DOC≈3.504→2.545 (-27.36%):
  close-reference priority replaces interpolation.
These are mathematical software transitions, not measured machining outcomes.
DOC1 and hard caps can hide them; they do not establish physical suitability.

Recommendation after measurement: address selection/interpolation continuity before
only smoothing q/q². A separate policy revision should define a conservative,
continuous transition envelope at anchor eligibility and near-match priority, then
compare all modes against this baseline. Preserve exact outputs and old saved values;
version the new policy and review any increases. No formula is approved by this
proposal alone. Indexing remains unnecessary for this dataset.

Reproduce (set CARGO_TARGET_DIR to the external build cache where appropriate):
```sh
cargo run -p ivac-feeds-speeds --example sensitivity > /tmp/fs39-sensitivity.csv
python3 crates/ivac-feeds-speeds/examples/sensitivity_report.py /tmp/fs39-sensitivity.csv
cargo test -p ivac-feeds-speeds
cargo clippy -p ivac-feeds-speeds --all-targets -- -D warnings
```

The sweep characterizes sampled boundaries, not all possible datasets or real tool
limits. New sources need their own sensitivity checks. FS4 remains deferred and
ShopBot RPM-transition bug ivac-2zll remains independent.
