# TEST1 — bounded property campaign, 2026-10-18

Status: bounded implementation ready for human review; **TEST1 overall remains open**,
particularly full D2/SAFE-Z coverage. This is not release qualification.
The user accepted HATCH/SAFE-Z (ivac-th8y2) and authorized this checkpoint.
Pipeline59, version0.9.0-beta.2, dirty tree based on
`2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b`.
No commit, push, package, remote workflow or publication.

## Implemented mechanism

- Seven native families, 32 cases each, plus32 frontend lifecycle schedules.
- SplitMix64-v1: root `0x5445535431000001`; case seed is
  `splitmix64(root XOR (property_id << 32) XOR index)`;
  A1/A2/B1/B2/C1/D1/D2 IDs1/2/3/4/5/7/8. Default scale0.25, frozen after
  resource-domain calibration (failed larger cases retained below). Frontend C2 enumerates32
  deterministic branch combinations, rather than using floating-point random seeds.
- One native test process per case,10-second case watchdog and120-second native
  combined native/frontend stage cap, build excluded. Timeout/unfinished stages fail; no silent truncation.
- Every request, full successful response, seed, outcome and process output is
  saved with SHA256SUMS. Output directories must not already exist.
- Replay uses the exact case JSON. Reduction preserves the error category and
  only decreases source multiplicity/width variation within the generator domain;
  <=128 trials/60seconds, retaining original and reduced inputs. It is a bounded
  reduction, not a globally minimal counterexample guarantee. No production
  geometric failure was found; reducer reproduced the real resource-budget
  failure and retained it when smaller geometry no longer failed.
- CI job configured on Ubuntu24.04 with Rust1.88/Node24/pnpm10. It has **not run
  remotely**. The120-second policy is measured locally, not certified on that runner.

## Independent oracles and actual coverage

A1 compares provenance identity multisets across permuted/reversed coincident
sources and refuses a deleted binding. A2 uses analytic cardinal arc extrema and
XY frame translation. These do not replace ambiguous-binding and nonfinite-import
regression tests.

B1 checks inside/outside rectangle/circle profiles and cascade/raster pockets with
0–2 rectangular islands. Generated cutter diameter is2mm (within the approved
1–12mm domain; this campaign does not exhaust that range). Straight segments use
analytic minima of squared distance; circular output and rounded joins use the
separate0.01mm chord budget. Output rounding is separate from exact geometry
comparison. B1 does not certify complete area coverage.

B2 separates Perimeter/medial-axis,60°/90° V-bits, rectangles, islands and30–150°
wedges. The continuous oracle splits projection regimes at edge endpoints and
checks quadratic extrema, including variable radius and surface-crossing entries.
Convex outer half-planes and island interior checks prevent wrong-side acceptance.
Radial budget:0.0002 +0.0000317 +sqrt(2)*0.00005 +tan(half-angle)*0.00005 +1e-8mm;
squared residual1e-9mm². Target-depth witnesses detect a missing final depth;
they do not establish full pocket or every medial branch coverage.

C1 compares entire serialized responses on cold/hit/mutation/restoration, with
an actual scoped CPS-hit witness. No machining-time/output fields are excluded.
Per-case process isolation avoids the known global-cache test race(ivac-rt80e).

C2 exercises GenerationRuns and the actual export freshness predicate extracted
without changing its behavior. It covers edit/undo/redo context restoration,
failed/cancelled completion, late responses, replacement results and post-dialog
freshness. It does not execute UI commands or a real filesystem save dialog.

D1 independently decodes OpenSBP movement, feed state, operation markers and line
mapping, with unknown initial axes retained; compares to preview. Initial unobserved
movement is not fabricated. Profile/Pocket/V-Carve/Drill and sequential operation
cases are included; multi-tool manual transitions still rely on existing ShopBot
regressions rather than this generated subset.

D2 varies stock top, positive/equal-top clearance and known/unknown work envelope,
checking continuous rapid stock crossings or explicit refusal. HATCH/SAFE-Z
regressions separately cover nonfinite/negative/stock-edit/streaming guards.
**Full Z06 fixture/holder and all Z07 transitions remain ivac-j09eh, unqualified.**
TEST1 passing cannot close that release gate or substitute for platform/physical tests.

Two frozen shared Profile/V-Carve requests replay through the pure request boundary
used by the WASM entry point. Native bridge replay does not exercise JsValue
serialization, browser execution or package freshness.

## Reproduce

With Node24 on PATH, from repository root, build outside the timed campaign:

```sh
python3 scripts/test1/build.py > /tmp/test1-binary
python3 scripts/test1/run.py --binary "$(cat /tmp/test1-binary)" --output /tmp/test1-new-run --frontend
pnpm --dir frontend exec vitest run src/lib/state/test1-lifecycle.test.ts src/lib/state/generation-run.test.ts
```

Replay: add `--replay /absolute/path/to/case.json` and use a fresh output directory.
Extended mode: `--cases 512 --budget 600` (no claim it completes within budget).
Reduction:

```sh
python3 scripts/test1/reduce.py --binary "$(cat /tmp/test1-binary)" --case /path/case.json --output /tmp/test1-reduction
```

## Validation record

Final bounded campaign: **256/256PASS in26.379s**, no skips/timeouts.
Native oracle self-checks4PASS; worker ignored by ordinary test discovery and
executed explicitly by the watchdog runner. Core1204PASS; Halfpipe16PASS;
native WASM23PASS including fixed bridge replay. Clippy/fmt/diff-check PASS.
Human review **ivac-930yx OPEN**. TEST1 tracking **ivac-4z6lh remains OPEN**.

[Evidence directory](test1-2026-10-18-evidence/): archived complete bounded,
expanded failed and intermediate failed campaigns, reduction, logs, source hashes
and SHA256 manifests. Use a fresh directory to extract/replay; generated programs
are software-test artifacts, not instructions for a machine.
First native campaign before oracle strengthening:224/224PASS,41.43s.
Frontend:1360PASS,1existing skip; Node package tests10PASS.
Typecheck: zero errors/warnings. Targeted HATCH/SAFE-Z14PASS; V-Carve10PASS.

**HARD STOP at checkpoint handoff. Human review must retain the bounded domain
and the unqualified SAFE-Z/CI/browser/installation/physical gates.**

## Resource-domain calibration (not hidden passing cases)

The initial small-pocket campaign passed224 native cases in41.43s. Expanding
pocket geometry to40×30mm exposed18 case failures at the approved20000-move cap:
4 B1,6 C1,8 D1. The representative cascade pocket has one4×4mm island, D2,
two passes,18 offsets and20167 emitted moves. Total movement2482.69mm;
11540 moves are shorter than0.01mm. This is a resource-domain observation,
**not demonstrated overcut or a production geometry defect**.

The reducer tried6 variants in6.25s, preserving the original failure. The
reduced input metadata retains scale1 because smaller geometry removes the
budget failure. It does not establish a globally smallest failing geometry.
Scale0.5 passed single-operation cases but four two-operation D1 cases still
emitted20448 moves. The final fixed generator scale0.25 uses pocket bounds
(10,10)–(39.5,34), retaining0–2 islands, positive margins and both fill strategies.
All original failures remain archived and replayable; none is relabelled PASS.
The20000 cap and all geometric oracles are unchanged. Larger geometry remains
outside this bounded campaign; ivac-53ky9 records the observation for review.

No general geometry/planner/postprocessor fix was made. The only production
changes are behavior-preserving extraction of export freshness and the native
request boundary used by WASM. Generated-output safety guards remain unchanged.
