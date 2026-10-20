# NUMSAFE2 — bounded pass scheduling and depth-step safety — 2026-10-20

Tracking **ivac-o9gwj** (NS-6b), follow-up of NUMSAFE (`e1a0c95`, unchanged).
Software evidence only; no machine was run. Pass budget chosen by the
maintainer during this checkpoint: **1000**.
Evidence: [`numsafe2-2026-10-20-evidence/`](numsafe2-2026-10-20-evidence/).

## 1. Findings

| ID | Finding | Verdict |
| --- | --- | --- |
| P-1 | Finite extreme depth/step makes the shared Z schedule generate an unbounded number of passes (CPU, memory, output size) | **Confirmed**: 200 000 passes → 2.2 M lines in 2.7 s; linear, so depth −1e9 mm at 1 mm would need ~11 billion lines |
| P-2 | Peck drilling: unbounded pecks per hole (GRBL default expansion and `.cps` posts expand each peck) | **Confirmed**: 100 000 pecks → 420 k lines |
| P-3 | Half-pipe: `ratchet_emit` allocates one level per depth-per-pass down to the axis floor | **Confirmed by code** (depth-per-pass floored at 0.05 mm, depth bounded only by tool reach) |
| P-4 | V-Carve depth levels | **Partially bounded already** (plan: 4096 sweeps, 1 M points). 1112 levels were still generated (167 k lines) |
| P-5 | Planar surfacing | **Refuted**: already limited to 1000 depth passes and 100 000 scanlines |
| S-1 | An explicitly supplied step of 0, positive, NaN, ±∞ or below 1e-9 mm silently became a single full-depth pass (`step_unspecified` warning only, not export-blocking) | **Confirmed**: 10 mm in one pass |
| S-2 | The same invalid values made V-Carve and half-pipe fall back to a 1 mm depth per pass | **Confirmed** (V-Carve NaN/0 → one pass at −1 mm) |
| S-3 | An invalid tool default step (op inheriting) behaved like S-1 | **Confirmed** |

Overflow and infinite loops: f64 arithmetic does not overflow to a panic. The
only unbounded loop was the NaN depth case, already fixed by NUMSAFE. The
cost was purely proportional to the pass count. Cancellation (`CancelToken`)
is checked between ops and inside some drivers, not inside one op's pass
loop, so it could not interrupt a huge single op. Existing per-feature limits:
planar surfacing (1000 passes / 100 k lines), V-Carve plan (4096 sweeps / 1 M
points), raster (16 M pixels), STL grid (16 M cells).

Before reproductions:
[`before-probe-e1a0c95.txt`](numsafe2-2026-10-20-evidence/before-probe-e1a0c95.txt)
([source](numsafe2-2026-10-20-evidence/before_probe.rs.txt)).

## 2. Depth-step contract

| Value at the boundary | Before | After |
| --- | --- | --- |
| Op `step` omitted (`null`), tool `default_step` valid | Tool default | Unchanged |
| Op and tool both omitted | Documented default: one pass at final depth + `step_unspecified` | Unchanged |
| Op `step` supplied: finite, ≤ −1e-9 mm | Used | Unchanged |
| Op `step` supplied: 0, positive, NaN, ±∞, > −1e-9 | Same as omitted (S-1/S-2) | **Refused**: `InvalidStep` |
| Op omitted, tool `default_step` supplied but invalid | Same as omitted | **Refused**: `InvalidStep` ("tool default step") |
| Op valid, tool default invalid | Op step used | Unchanged (tool value unused) |
| Drill, thread, T-slot, dovetail | Step unused | Unchanged (exempt, as the editor hides the field) |

The API/model already separates "absent" (`None`/`null`) from "supplied"
(`Some(v)`), so no schema change was needed. JSON cannot carry NaN or ∞; the
UI only stores finite negative values (`storedPassDepth`). All 1937 stored
steps in the repository's projects and requests are valid negatives, and the
largest existing schedule there has 12 passes.

## 3. Pass-count policy

`cam::MAX_DEPTH_PASSES = 1000` per operation, applied to:

- the shared Z schedule (all multi-pass contour, pocket, engrave and similar
  ops), counting the finish pre-pass and explicit depth-list entries;
- drill pecks per hole, `ceil((R − Z) / Q)`;
- half-pipe and V-Carve depth levels, `ceil(depth / depth-per-pass)`;
- planar surfacing (its literal 1000 now uses the same constant; its message
  and the 100 k scanline limit are unchanged).

Justification: it is the same ceiling as the existing qualified planar
surfacing contract. It covers 100 mm at 0.1 mm or 50 mm at 0.05 mm. At the
limit, a square profile is 11 k lines in about 0.02 s. The repository corpus
needs at most 12 passes. Single-pass modes (plot, drag knife, plasma, laser)
are not affected.

Over the budget, the operation is refused with
`PipelineError::PassLimit` (structured, `op_id`, `limit`, "No program
generated", hint: increase depth per pass or reduce depth). The step is never
enlarged, passes are never truncated, and the depth is never changed.

## 4. Corrections

- `cam.rs`: `MAX_DEPTH_PASSES`.
- `gcode/z_schedule.rs`: `build_z_schedule` takes `max_passes` and stops after
  `max_passes + 1` iterations (`Err(count)`). It uses the same arithmetic as
  before, so the boundary is exact and in-budget schedules are identical.
- `gcode/emit.rs`: `depth_plan` is shared by `multi_pass` and a pre-check at
  the start of `emit_offset`, so an over-budget offset emits nothing (not even
  its entry rapid or plunge). The refusal goes through the NUMSAFE output-fault
  channel (`OutputFaultKind::PassLimit`): checked per op before caching, and
  the streaming sink stops writing.
- `gcode.rs` (`emit_drill_block`): pecks-per-hole check before any hole.
- `pipeline/op_drivers/vcarve.rs`: `check_depth_levels`, shared with
  `halfpipe.rs`, runs before planning or allocation. `planar_surface.rs` uses
  the constant.
- `pipeline.rs`: `PipelineError::{InvalidStep, PassLimit}`, their structured
  mapping, and `require_valid_explicit_step` (run after setup synthesis,
  before any driver).
- `docs/BUILDING.md`: the macOS Chrome instruction now describes the `exec`
  wrapper that the release gate uses. A symlink makes Chrome abort, as
  observed during NUMSAFE.

## 5. Machine-output invariance

The NUMSAFE identity corpus was regenerated: TEST1 profile and V-Carve plus
the 16 ShopBot M1 fixtures × profile/pocket × LinuxCNC, GRBL, HPGL and
ShopBot. All 136 programs are **byte-identical** to `94cdd6b`/`e1a0c95`.
Refused scenarios previously produced the S-1/S-2 single passes and the
P-1…P-4 oversized programs shown in §1.

## 6. Tests and benchmarks

New: `crates/ivac-core/tests/numsafe2.rs` (9 tests), plus unit tests
`bounded_schedule_is_exact_and_identical_below_the_limit` and
`depth_levels_budget_boundary_and_non_finite`. They cover:

- shallow and deep schedules and exact landing (3 mm at 0.7 mm gives 0.7, 1.4,
  2.1, 2.8, 3.0);
- a step larger than the depth;
- 999, 1000 and 1001 passes, with the finish pre-pass, and a 1001-entry depth
  list;
- 1 mm at 0.001 mm (1000 passes, accepted) and 1 mm at 0.0009 mm (refused);
- depth −1e9 and −1e300, and step −1e-6, each refused in under 5 s with
  nothing emitted;
- 1000 passes on a circle (accepted);
- NaN, ±∞, 0, −0, +1 and −1e-12 refused, and omitted steps kept;
- a refused op twice, then the corrected op byte-identical to a fresh run;
- a streamed refusal with no `G1`;
- pocket, drill pecks (1000 accepted, 1001 refused, micro-peck refused, odd
  step on a drill ignored) and V-Carve (1112 levels refused, NaN step
  refused).

| Gate | Result |
| --- | --- |
| `tests/numsafe2.rs` + 2 unit tests | 9/9, 2/2 |
| NUMSAFE `tests/numsafe.rs` (`--features cps`) | 7/7 |
| SAFE-Z: `safe_z_extended`, `hatch_safe_z` | 15/15, 14/14 |
| ShopBot M1 / M2 | 11/11, 27/27 |
| CPS `cps_posts`, V-Carve `vcarve_qualification` | 5/5, 10/10 |
| `tool_provenance`, `comment_safety`, `import_correctness`, `golden_corpus` | 12/12, 1/1, 5/5, 1/1 |
| TEST1 (`run.py --cases 32 --budget 120 --frontend`, release) | 256/256 in 28 s; frontend lifecycle 32/32 |
| `cargo test --workspace --all-features` | 1635 passed, 0 failed, 1 ignored |
| Clippy strict (all features) and pedantic gate | OK |
| Output identity | 136/136 byte-identical |
| vitest (frontend/WASM lifecycle, unchanged by this checkpoint) | 1458 passed, 1 skipped |
| `scripts/pre-release.sh` (Node 24, pnpm 10.34.7, cargo-deny 0.20.2, `~/.local/chrome-shim` exec wrapper) | **all 16 gates green**, including UI smoke |

The first full `pre-release.sh` run failed only on pedantic cast lints in the
new test file. A justified module-level allow was added, as in
`tests/common`, and the second full run is green. **Not run:** Tauri and
Android UI, physical machine.

## 7. Remaining resource limitations

- The budget limits passes, not geometry: 1000 passes over a very dense
  contour (or many objects) can still produce a large program. Total
  output-size budgeting is out of scope.
- V-Carve keeps its own qualified plan budgets (4096 sweeps, 1 M points) under
  the 1000-level check.
- Waterline, relief and thread use their own step parameters and were not
  reviewed for pass counts in this checkpoint. Waterline warns and roughs
  nothing for a z-step ≤ 0, but a tiny positive z-step is not bounded here.
- A single op's pass loop is still not cancellable mid-op; the budget bounds
  its duration.
- The streaming export atomicity limitation from NUMSAFE is unchanged (tracked
  separately).

## 8. Human decisions

1. Pass budget = 1000: **decided by the maintainer during this checkpoint.**
2. Accept refusing explicitly supplied invalid steps (op, or inherited tool
   default) instead of a single full-depth pass or a 1 mm fallback. Omitted
   steps keep the documented default.
3. Keep the 1000 budget out of waterline, relief and thread for now (§7), or
   schedule it.
