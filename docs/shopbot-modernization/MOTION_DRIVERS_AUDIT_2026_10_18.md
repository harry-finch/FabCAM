# Motion and operation-driver audits — lots 09–10 — 2026-10-18

Engineering: `ivac-d1joh`. Human review: **`ivac-7y9ft` OPEN**.
The independent reports were checked against source, not treated as authority to
accept all proposed changes. Baseline: `2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b`,
0.9.0-beta.2 / pipeline54. These local changes advance the cache version to 55;
no project format, application identity or public version changes.
Incoming candidate-campaign documentation is preserved.

## Audit 09 dispositions

| Finding | Assessment and correction |
| --- | --- |
| Initial vertical descent uses XY feed | Confirmed. Shared XYZ emitter reaches the complete first-point Z at the plunge feed, then traverses subsequent points at cutting feed. No repeated first-point plunge at XY feed. Existing surface-start V-Carve/Halfpipe geometry is retained. |
| Waterline height-grid displacement | Confirmed. Reconstructed mesh vertices now use cell centres, matching ReliefMill. The centre-to-centre sampled domain is retained; no unsupported half-cell edge extrapolation is invented. |
| Internal thread entry touches bore wall | Confirmed entry at nominal tangency; the report's claimed collision threshold at half the bore radius is not itself a mathematical collision proof. Internal entries now begin on the bore axis before approaching tangency. Existing helix exit already returns to the centre. A cutter unable to fit the bore is rejected. This does not qualify real holder/shaft clearance. |
| Chamfer nominal diameter / Circle-only / fixed lead angle | Confirmed. Use effective diameter, the shared analytic-circle recognizer and the cutter's configured lead angle (existing 10° default, 0.5–45° range). The recognizer validates connected, single-turn common-circle geometry, including bulge-derived Arc centres; it does not infer circles from arbitrary facets. Small-circle drilling already received this recognizer in lot 8. Thread preflight/emission now share it too. |
| Thread feed influenced by unselected holes | Confirmed. Accumulate the smallest actual emitted pass radius only from selected qualified bores; the same compensation remains confined to internal threads. |
| Cancellation ignored | Confirmed. Drivers pass the token into cancellable geometry APIs. Poll the drop-cutter kernel/rows/cells/footprint scans and waterline levels/triangle scans; cancellation returns no successful partial result. Legacy non-cancellable API signatures remain wrappers. Latency qualification remains open, below. |

## Audit 10 dispositions

| Finding | Assessment and correction |
| --- | --- |
| Open multipass tabs and exit use wrong direction | Confirmed. Track the actual endpoint through passes and ramp cleanup, walk tabs on the current directed chain, calculate exit from that endpoint and retain the actual exit position. Straight first-pass ramp/fallback handling already existed; the report overstates that part. |
| Two-arc circles treated as open / wrong lead side | Confirmed. Closure includes connectivity and nonzero arc-aware area. Lead side uses the shared exact arc-aware area. Helix candidate placement samples curves, then checks clearance against source segments; two-arc circles no longer reverse on alternate passes. |
| CPS tool change retains old feeds | Confirmed. Pass resolved horizontal/plunge rates in structured tool context, update all four section feed parameters and plunge classification. A comment-only placeholder section is replaced by the spot tool while preserving comments; sections containing machine events are not discarded. No ShopBot capability is broadened. |
| Arc fitting rejects coarse circular facets / fits three points | Proposed policy change declined. The input is a polyline: co-circular vertices do not authorize changing its edges beyond tolerance. A two-segment fit within the full tolerance is not inherently a defect. A separate real error was found: independently checking vertex error and nominal sag can undercount their combined deviation. Now evaluate endpoints and the closest point to the centre on every finite chord. The tolerance is unchanged; angular-order checks remain. |
| Grid nearest-neighbour diagonal bound | Not a defect, as the report itself concludes. The distant-start empty-ring cost is confirmed. Use a bounded exact linear scan outside the starts' box, with the existing comparator/ties, and relative grid coordinates inside. Tests compare the full order with the linear oracle for distant clusters and endpoints, including large positive/negative translations. |
| LinuxCNC inch probe feed / stale modal feed | Confirmed. Probe feed goes through the normal unit/profile formatter, then invalidates the feed cache. The next feed is emitted explicitly. This targets `probe_toward_z`; it is not a redesign of generic raw Probe operations. |

## Numerical and qualification limits

- Arc-fit error is measured over complete input chords, not just their vertices.
  No tolerance increase or speculative reconstruction of an original CAD circle.
- Helix candidate polygon spacing is at most PI/32 radians. Exact distance to
  source lines/arcs additionally constrains the candidate's clearance. This
  sampling is candidate search, not a new material-removal or machine proof.
- Cancellation is cooperative. Pre-cancel tests and propagation are verified;
  **the ≤200 ms p95 end-to-end target is NOT QUALIFIED**. Large allocations,
  sorting, mesh stitching and Boolean clearing can remain uninterruptible.
  Follow-up `ivac-42ylx` records the required workload measurements and any
  further bounded polling. No latency claim is inferred from fast unit tests.
- No additional Engrave, tool kind or output strategy is enabled in ShopBot.
  No post-specific workaround was added to geometric algorithms.

## Validation

Evidence is retained in [the manifest](motion-drivers-audit-2026-10-18-evidence/checks.json)
and its raw logs. Four baseline counterexamples failed before the corrections
(initial Z feed, internal-thread axis entry, circular closure and inch probing).
Seventeen targeted tests now cover these and the other changes. The open-path
matrix spans 1/2/3 passes × tabs on/off × ramp on/off (12 combinations), with
straight entry/exit and decoded-motion checks for unintended diagonal cuts.

The GRBL snapshot changes only the second circular pass from G3 to G2. A test
restores that one line and matches the entire old digest, in addition to checking
the corrected circle's direction and the new digest. Geometry parity remains a
separate test. Four probe expectations now permit the correctly formatted modal
F line before G38.2. The stricter continuous arc-fit bound changes circle tessellation in the two
ShopBot circle/multiple fixtures. Both decoded-geometry checks pass before their
references are refreshed. Independent review retains depths -2/-4/-5 mm and
20/3.333333 mm/s feeds; circle moves increase 370→373. Maximum endpoint radial
error is 0.007044 mm and minimum chord radius is 27.990673 mm for the 28 mm
compensated radius, within existing unchanged budgets. The preceding drill
program in the multiple fixture is byte-identical. See `golden-review.json`.

That tessellation change exposed a separate existing rectangular-tab defect:
an arc could plunge to the floor at a seam still inside a tab. Preserve the
required tab height at that seam. The existing ShopBot curved-tab test now
passes for Rectangle and Ramp; a new two-arc seam regression protects the fix.
No tolerance or holding assertion was loosened.

| Check | Result |
| --- | --- |
| Core, CPS enabled, offline, serial | 1,174 PASS (17 new regressions) |
| Seven integration suites | 87 reported PASS; 86 executed, optional FANUC absent |
| Clippy core/CPS with warnings denied | PASS |
| Rust formatting / diff whitespace | PASS |

The serial run avoids the separately tracked pre-existing cache-test isolation
issue `ivac-rt80e`. Existing Halfpipe and V-Carve checks are included. No frontend,
installer, physical machining or hardware-latency qualification was performed.

## Handoff and release boundary

The existing desktop candidates from run39053047404 remain pipeline54 artifacts;
these source changes are not present in them. Existing installation/runtime and
physical evidence is not promoted to pipeline55. Frontend unchanged: no new UX
or browser qualification claimed. No new installers produced or transmitted.
Clean macOS install and Windows/Ubuntu tests remain separate release gates.

**HARD STOP for human review. No commit, push, merge, tag, release or publication.**
