# V-Carve qualification — Phase A audit

2026-10-14 · Historical Phase A audit. **Targeted strategy subsequently approved.**
Current implementation/evidence: [software qualification](VCARVE_QUALIFICATION.md).
Tracking: ivac-2eh7. This is an audit, not completion of the qualification milestone.
No production code changed by this audit; existing uncommitted CAD1/UX3/UX4/UX5
work is retained. The user's explicit V-Carve request supersedes the older next-action
wording without inventing acceptance of independent human gates.

## Actual algorithm and semantics

`pipeline/op_drivers/vcarve.rs` has two modes:

1. **Perimeter (default)**: closed regions are offset inward by a fixed radius R.
   The resulting rings are traversed at constant Z. The interior plateau remains.
   This is a compensated V-bit perimeter groove, not arbitrary variable-width
   region carving and not the separate fixed-depth `Engrave` operation.
2. **Full medial axis (opt-in)**: flatten boundaries, sample at 0.1 mm, triangulate
   samples with Delaunay/Voronoi, retain interior chains and prune short branches.
   Radius at each node is recomputed against actual flattened boundary segments
   (an existing, valuable correction; not the circumradius of discrete samples).
   Convert radii to depths and pass chains through a progressive-deepening emitter.

`cam/vcarve.rs` provides the axis and depth conversion; `cam/vcarve_emit.rs` plans
ramps/ratchet passes and is also used by Halfpipe. `gcode::emit_vcarve_block` emits
linear XYZ segments and inter-chain retracts. There is no native variable-Z arc
from this driver: source curves have already become polylines.

Keep region combination, tool model, generic toolpath/IR and post/preview boundaries.
Do not replace this with a claim of complete sign/letter V-carving or general
flat-bottom clearing. Comments comparing behavior to other commercial products
are not qualification evidence.

## Capability classification

READY requires proof of the promised scope, not merely an implementation.

| Area | Classification | Existing evidence / missing proof |
| --- | --- | --- |
| Operation model and selection | PARTIAL | `VCarveParams`, stable normal operation/source pipeline; selected closed regions supported; mixed open/closed sources filtered without a dedicated partial-selection report |
| Open paths / straight line V-carving | UNSUPPORTED BY DESIGN | Closed-region driver warns/no-ops; fixed-depth `Engrave` is a separate intention, not qualified here |
| Polygons / nested regions / holes | NEEDS QUALIFICATION | Existing source-combine topology and medial-axis hole checks; continuous cutter clearance and full topology matrix not proven |
| Curves | PARTIAL | `source_combine::TESS_INTERPOLATE=6`, then 0.1 mm boundary sampling; not an independently qualified global chord-error budget |
| V-bit geometry | PARTIAL | Included angle, tip diameter, effective maximum diameter, optional flute length/holder exist; driver clamps invalid values and does not enforce flute length locally |
| Ideal cone depth formula | PARTIAL | `h=(r-d0/2)/tan(theta/2)` and negative cutting Z implemented; independent 60°/90°/flat-tip matrix still needed |
| Width/depth cap | PARTIAL | `carve_max_width_mm` actually caps radius; UI label says width and mode help says width/2; perimeter XY offset does not shrink when depth cap shrinks engagement |
| Variable-depth path | PARTIAL | Axis-derived XYZ exists; local ramp-depth violation reproduced below |
| Multipass | PARTIAL | Medial ratchet exists; default perimeter ignores computed pass step; short chains can stop shallow without reporting achieved-depth metadata |
| Refinement toggle | MISSING | Persisted `multi_pass_refine`; UI explicitly disabled as not implemented; driver does not implement it |
| Entry / travel / clearance | PARTIAL | Shared emitter retracts before inter-chain XY; perimeter's first depth descent is emitted after selecting horizontal feed; nonzero start-depth semantics need proof |
| Source inset / inlay | NEEDS QUALIFICATION | Existing inset path assumes first ring outer and remaining rings holes; split regions and collapsed inset need explicit handling; no inlay-fit claim |
| Toolpath determinism | NEEDS QUALIFICATION | Indexed graph traversal, HashSet used for membership; no new permutation/repeat test run in this audit |
| ShopBot V-Carve output | UNSUPPORTED BY DESIGN (current post) | Reproduced refusal of `engrave` strategy; V-bit tool type also excluded by post whitelist |
| ShopBot resolved preview | NEEDS QUALIFICATION for V-Carve | ADR-0007 decoder architecture exists, but current post emits no V-Carve output to qualify |
| 2D / 3D / material removal | PARTIAL | Path infrastructure and conical/truncated-cone heightmap profile exist; grid simulation is not exact swept-volume proof; invalid angle fallback differs from driver clamp |
| Persistence / stale output / Undo | NEEDS QUALIFICATION | Backend VCarve serde test passes, frontend wiring exists; end-to-end tool-edit/save/reopen/history/export matrix not yet run |
| Performance | NEEDS QUALIFICATION | One small rectangle already produces 3591 preview segments in medial mode; not a performance benchmark or proof of a hang |
| Diagnostics / inspector | PARTIAL | Two modes exposed, no-region/tip/depth warnings exist; width wording contradicts implementation; no achieved-depth diagnostic from ratchet |

## Reproduced findings, not hypotheses

Run the retained read-only probe:

```sh
cargo run -p ivac-core --features cps --offline --example vcarve_audit
```

[Captured output](vcarve/AUDIT_PROBE.txt). Synthetic values are software references,
not cutting recommendations. Probe uses normal pipeline and bundled ShopBot post
where stated; the last two cases deliberately isolate the shared planning kernel.

**A1 — default pass limit ignored / incorrect depth-limited offset.** Rectangle
20×10 mm, sharp 90° bit D6, max depth −1 mm, step −0.2 mm. Actual perimeter has
10 preview segments and directly descends from 0 to −1 at fixed XY. Its XY ring
remains inset 3 mm. At h=1 a sharp 90° cone engages radius 1 mm, leaving the intended
source wall 2 mm away. `vcarve_depth_limited` is emitted, but does not repair the
mismatch or the ignored pass step. Source confirms `dpp` is used only in medial mode.

**A2 — variable ramp overcuts its target envelope.** Kernel input target points
`(x,z)=(0,-1),(1,-0.01),(10,-0.01)`, all y=0, step 0.5 mm. The ramp inserts
`(x,z)=(2.8356409098,-0.5)` inside a segment whose entire target is −0.01.
This is 0.49 mm deeper than the target. It is a kernel counterexample, not a claim
that every imported shape produces this chain. It invalidates the emitter's
promised local depth bound and needs a regression independent of screenshots.

**A3 — short-chain target not reached.** Kernel chain length 1 mm, constant target
−1 mm, step 0.5 mm, default 10° entry. Output bottoms at −0.1763269807 mm and returns.
The API returns only polylines, with no undercut/achieved-depth result. Driver warning
flags come from radius/Z caps, not this ramp limitation; a comment asserting warning
propagation does not implement it.

**A4 — ShopBot fails closed today.** Both rectangle modes fail with
`ShopBot MVP: unsupported strategy engrave`. This is the existing protective policy,
not a decoder error. Removing only that guard would still encounter the allowed-tool
check. Do not globally allow all engrave operations or all V-bit uses.

## Existing tests and baseline

`cargo test -p ivac-core --features cps --offline --lib vcarve`:
**26 passed, 0 failed**, 1048 filtered (current working tree, not a clean release).
Tests cover triangle radius, rectangle centerline, plus shape radius, nearest-edge
oracle, hole chord sampling, pruning, ramp basics, deepening termination, lead angle,
open-region refusal, default perimeter behavior, diameter reach, flat-tip no-cut,
inset and serde. They do not cover A1–A4 sufficiently; passing them is not a V-Carve
qualification PASS. `sim/heightmap.rs` and `sim/sweep.rs` contain additional V-bit
profile tests, identified but not rerun as part of this initial filtered baseline.

Missing: independent width/depth and continuous clearance oracle, acute/concave/
curved/topology/degenerate matrix, pass material increment and final-depth proof,
feed/plunge semantics, final ShopBot decoding/preview/WCS parity, tool physical
limits, frontend lifecycle tests, saved fixtures, actual UI captures, determinism
and performance benchmarks. No full regression/build/visual/hardware PASS is claimed.

## Proposed minimal implementation, pending the decision below

- **KEEP** existing source/model/normal pipeline/ShopBot decoder, plus the sampled
  medial-axis builder as a candidate path generator. It is not a coverage proof.
- **EXTEND** V-bit validation (finite cone, included angle, flat tip, usable diameter,
  authored flute limit), consistent explicit refusals and an error budget. Preserve
  stored parameters; never reinterpret saved radius values as diameters.
- **REPLACE only V-Carve's pass/entry scheduling** with a generic scheduler that
  takes the geometric depth envelope, guarantees it at inserted/interpolated points,
  honors per-pass increments, reports achieved vs requested depth, and refuses
  chains with no qualified entry. Reuse existing representations and emission.
  Leave Halfpipe on its existing behavior in this milestone; do not silently alter it.
- Preserve the legacy perimeter mode with honest wording, geometry-consistent depth
  caps and real bounded passes. Keep medial mode separate; do not silently change
  the default or turn a saved perimeter operation into a different strategy.
- Qualify supported medial paths with independent boundary/cutter checks; refuse
  unsupported topology and entries. Do not advertise complete arbitrary-region
  removal, inlay fit, or new flat-bottom clearing.
- **EXTEND** only the V-Carve-specific post capability path after geometric tests,
  ideally with an explicit V-Carve strategy discriminator rather than enabling
  generic `engrave`. Preserve all other tool/strategy/Helix guards and lab spindle
  constraints. Use existing M3 XYZ and final decoder; add no OpenSBP syntax.
- Target UI wording/diagnostics, then complete geometry → output → preview,
  persistence, unrelated-operation regressions, fixtures, screenshots and docs.

## Decision gate

The current variable-depth scheduler violates its own envelope and is shared with
another operation. Correct multipass and entry planning requires a new qualified
V-Carve planning stage, not just an extra ShopBot capability flag or UI label.

**Recommended:** approve the bounded V-Carve-only scheduler replacement described
above, retaining both saved modes and existing axis builder where qualified. This
implements meaningful multipass correctness without promising a new arbitrary-region
V-carving algorithm. A later complete-region/flat-bottom strategy remains separate.

**Smaller alternative:** constrain this milestone to explicitly validated single-pass
perimeter engraving, reject variable-depth/multipass V-Carve for export and record
those capabilities as deferred. This would leave the primary variable-depth goal
PARTIAL and is not recommended.

This pause follows the user's Phase A §5 and multipass §13 approval requirement,
not a request to approve routine edits. Production changes wait for this choice.
Hardware validation remains unauthorized. No commit, push or next milestone.

## Future capability boundary — V-Carve Inlays (planning only)

[V-Carve Inlays plan](VCARVE_INLAYS_PLAN.md) records a separate later paired capability,
**PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** in its complete scope. Inherited
source-inset and duplicate-as-plug helpers and closed ivac-rt1.7 do not qualify
arbitrary-region removal or physical fit. This audit/current milestone must first
establish the promised V-bit, variable-depth/pass, topology, machine-output and
preview foundation. Later male/female geometry, start depth, overlap/clearance/glue
and trimming need independent tests and physical fit evidence. No inlay implementation
or scope expansion is authorized here; the active milestone's HARD STOP is preserved.
Two-sided registration is separate [future general CAM work](TWO_SIDED_PLAN.md).
