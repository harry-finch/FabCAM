# CAM validation toolbox

## Analytical fixtures

Prefer square, rectangle, circle, L-shape, hole, island, open polyline, and nested-contour fixtures with obvious expectations.

## Unit tests

Best for coordinate transforms, pass-depth generation, parameter validation, orientation/topology helpers, and post formatting helpers.

## Invariant tests

Best for cutter compensation, depth bounds, pocket containment, island preservation, machine envelope, and maximum stepdown.

## Golden tests

Best for stable serialized IR, post-processor output, and deterministic warning sets.

Avoid brittle goldens where multiple path orderings are equally correct unless ordering is part of the contract.

## Comparative tests

VCarve or another trusted CAM can be a reference for intent, but do not require identical toolpaths. Compare dimensions, compensation side, depths, preserved regions, and other invariants.
