# M4 VCarve comparison — 2026-10-12

Operator-supplied file: `/Volumes/NO NAME/M4-r2-carre-MDF_vcarve.sbp`.
Read only; no machine execution, source copying or modification of either programme.

- VCarve SHA-256: `5f66228fdb2af11b62fe4446878949152eb816fc0ae77abf22b2e899bdf1bc03`.
- Tested ivaCAM r2 SHA-256: `10d0420055ef4e4d259357b62a7a21dc9deaf195fa291253d21307f2f4945c4a`.

## Semantic findings

| Property | Tested ivaCAM r2 | Supplied VCarve | Assessment |
| --- | --- | --- | --- |
| Stock setup | 500 × 500 × 12 mm, lower-left XY, surface Z | Same in header | Matches declared setup |
| Retained square | 100 × 100 mm at (10,10)..(110,110) | 100 × 100 mm inferred at approximately (11.246161,10.409699)..(111.246162,110.409698) | Same dimensions; translated by approximately +1.246161 X / +0.409699 Y mm |
| Compensation | Outside, 3 mm radius | Straight edges 3 mm outside inferred square, radius-3 corner commands | Matches exterior intent for 6 mm tool |
| Tool/RPM | Tool 1, 6 mm, 18000 | Tool 1; header 6 mm/2 flutes; TR18000 | Matches declared diameter/flutes/RPM; exact tool reference absent |
| Depth passes | -3 then -5 mm | -2.5 then -5 mm | Same final depth/two passes, different first depth |
| Cutting feed | MS100 mm/s = 6000 mm/min | MS83 mm/s = 4980 mm/min | Different |
| Plunge feed | MS33.333333 mm/s ≈ 2000 mm/min | MS Z29.9 mm/s = 1794 mm/min | Different |
| Clearance | +15 mm throughout rapid/retract | +6 mm near contour; +20 mm initial/final | Different |
| Final XY | At contour entry (10,7) | Returns (0,0) | Different intentional parking |
| Direction | Clockwise around square | Counterclockwise around square | Opposite traversal |
| Corners | Linearized exterior arcs | CG circular commands | Different representation, compatible exterior-radius intent |
| Spindle/setup | Bounded TR/SO4/SO1, no C9 | CN90, C9, TR, C6, 2s wait, C7 | Expected lab-macro workflow; C9 may alter setup, per previously inspected macros |

Depths/bounds taken from actual movement lines, not stock header extrema.
VCarve arc endpoint extrema also occur in M3 endpoints for this square, so these
bounds are not limited by hidden arc bulges. Small coordinate rounding is present.
Both programmes lift before final spindle shutdown; VCarve additionally homes XY.

## Decision

This is useful independent reference evidence for nominal square dimensions,
exterior cutter compensation, 5 mm final depth, metric surface-zero setup and
spindle RPM/start/stop intent. It is **not an identical-parameter reference job**.
The observed parameter/placement differences do not identify an ivaCAM defect.
No new physical run is requested for this textual comparison. Request a VCarve
re-export with square position (10,10), pass depth 3 mm, feed/plunge 6000/2000
mm/min and safe/final Z 15 mm to finish the planned controlled comparison.
Opposite traversal and CG versus linear segments must remain explicitly noted;
byte-identical output is not required. Tested ivaCAM software/SBP stay unchanged.
M4 remains open pending the aligned reference or explicit acceptance of deviations.

## Second reference and final assessment — 2026-10-12

Source: `/Volumes/NO NAME/M4-r2-carre-MDF_vcarve2.sbp`.
SHA-256: `68018b8c85d7c570c216d289f9b9f27977f7d66ea8ddee66be10ca048218e750`. Operator reports no control over safe/final Z
in this export workflow. Original reference and tested ivaCAM programme unchanged.

- Actual cutting XY bounds now 7..113 on both axes; inferred retained square
  (10,10)..(110,110), 100 × 100 mm, exterior radius-3 compensation matches.
- Cutting depths -3 then -5 mm match the 3 + 2 mm pass schedule.
- Tool 1, declared 6 mm / 2 flutes, TR18000 and stock/surface-zero conventions match.
- Actual MS99.6,33.2 means 5976/1992 mm/min, versus ivaCAM 6000/2000: -0.4%
  on both. Do not label the feeds exactly identical or assume the cause.
- VCarve retains local safe Z +6 and initial/final +20, return XY(0,0), opposite
  traversal and CG corners. ivaCAM retains +15, finishes at contour entry and
  uses linearized exterior arcs. These differences are explicitly documented.
- Both retain a 2s spindle startup delay and final retract before spindle shutdown,
  through different bounded direct-output versus lab-macro setup policies.

Comparison passes for intended retained geometry, exterior compensation, pass
schedule, final depth and command ordering, with the above residual differences.
Identical byte output, traversal or post-imposed parking is not an acceptance
requirement. No third export or reference physical cut is needed for this scope.
This is textual reference comparison; it does not claim this VCarve file was run.

Together with operator-confirmed r2 Preview, spindle-enabled air cut, real MDF
cut, target conformity, clean finish and +15 final retract, M4 qualification
criteria are satisfied at the reported evidence level. Exact measurement values,
instrument, quantitative tolerances and RPM sensor measurement remain unreported.
M4 is ready for explicit milestone commit/push approval. No production-wide
qualification, FS implementation or governance update is implied.
