# ADR-0014: Explicit planar surfacing operation

Status: Implemented locally; human UX3 review pending (ivac-k4z3)
Date: 2026-10-14

## Context

The user authorized the next milestone after CAD1: UX3 planar spoilboard
surfacing. Existing `ReliefMill` uses a sampled 3D source and round-tipped tool;
`face_mill_overlay` decorates another path. Neither defines a bounded flat-cutter
rectangle with explicit edge travel. Pocketing a synthetic rectangle would mix
source selection, inside compensation and ordinary out-of-stock semantics.

## Decision

Add milling-only `OpKind::PlanarSurface { surface: PlanarSurfaceParams }`.
An operation owns a persisted rectangle in drawing-frame mm, margin, extra
overtravel, absolute stepover and X/Y scan direction. Copy stock takes a snapshot;
subsequent stock/import edits do not silently resize it. No imported contour,
relief source, derived drawing or new machine-control event is required.

`cam/planar_surface.rs` is a pure XY planner. Target margin expands all four
edges. Cutter centers follow both transverse target edges; along-scan endpoints
extend by effective radius plus extra overtravel. Scanline spacing is distributed
to land on both edges. The driver retains at least 5% overlap, bounds allocations,
uses negative depth/stepdown from the common operation model and retracts to the
existing safe Z between every line. The common XYZ emitter plunges at the saved
plunge feed to each pass depth before XY cutting.

Work-origin normalization translates this rectangle alongside other authored
objects. Validation precedes output/cache replay: milling, flat endmill or
compression cutter, finite positive physical/effective diameter and work area,
single front face, stock-top/start Z=0, safe Z>0, attainable flute/depth and
center travel. Invalid settings return a structured generation error. No Whirl,
non-direct entry, second cutter, explicit depth list or through extension.

The bundled ShopBot CPS receives a distinct `planar_surface` strategy and accepts
flat tooling explicitly. Existing `parallel`/3D relief refusals remain. Final
program moves continue to supply preview, simulation and fixture checks.

Two scoped diagnostic distinctions are necessary. Validated planar XY excursions
are intentional and receive `surfacing_edge_overtravel`; stock Z remains checked.
For a job whose cutting operations are exclusively planar surfacing, the old
heuristic requiring work zero inside the source bbox does not apply: the explicit
zone and machine travel are validated directly. Mixed contour jobs retain it.
Neither distinction exempts fixture collisions, under-stock depth or work limits.

The existing rate snapshot supports manual values for this operation. Dedicated
recommendations remain unsupported; proposals/reference-pack evidence cannot be
reused as surfacing recommendations. No FS mathematics, data or resolver change.

## Alternatives considered

- Pocket over a synthetic source: rejected because its compensation and source
  semantics do not express the intended full-edge raster.
- Flat image passed to ReliefMill: rejected because the source/tool contract and
  drop-cutter engine solve a different problem.
- Infer the zone dynamically from stock on every Generate: rejected because
  later drawing/stock edits would silently change an accepted operation.
- New movement IR or ShopBot-only path synthesis: unnecessary; the existing
  specialty driver and XYZ event boundary are sufficient.

## Consequences

Projects containing any surfacing operation, even disabled, use guarded
`ivac-project-surfacing` version 6. Old readers reject it rather than omit the new
operation. Versions 1–5 remain readable. V6 can also retain CAD1 modifiers and
existing cutting provenance. Removing all surfacing operations selects the
appropriate older guarded format again; no migration of tool/machine storage.

Frontend sizing uses the authored bbox without inventing source chains. The
selected 2D operation shows target and full cutter-envelope bounds; the inspector
distinguishes center travel from cutter footprint. History owns edits, duplication,
deletion and stock copying. This adds enum/model consumers and one isolated
planner/driver, not a parallel CAM architecture. Other manufacturing processes
do not acquire a surfacing or FS dependency.

## Validation

Analytical coverage and independent decoded ShopBot coverage include corners,
both directions, fractional dimensions and XY work offsets. Qualification checks
multiple depths, safe rapids, manual rates, fixture collisions, explicit refusals,
legacy contour warnings, save/reopen and Undo/Redo. See [UX3 qualification](../UX3_QUALIFICATION.md).
Native packaging, controller preview, physical surfacing and measured flatness
remain unqualified. Arbitrary third-party CPS numeric precision is not certified
by the bundled ShopBot tests. Human review can revise defaults or presentation
without expanding this geometric/process boundary.
