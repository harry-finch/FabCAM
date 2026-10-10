---
name: cam-geometry
description: Implement or review 2D/2.5D CAM geometry, cutter compensation, pockets, contours, tabs, leads, ramps, or related toolpath algorithms.
---

# CAM geometry and toolpaths

Use this skill when a task changes geometric or toolpath correctness.

## Core rules

- Treat geometry as numerical software, not ordinary application code.
- Prefer existing ivaCAM geometry libraries and representations over custom geometry code.
- State units and tolerances explicitly.
- Keep topology epsilon, flattening tolerance, output precision, and physical machining tolerance distinct.
- Preserve exact lines/arcs when the existing pipeline supports them.
- Never judge correctness from a rendered preview alone.

## Workflow

1. Reduce the requested behavior to simple geometric invariants.
2. Inspect the current implementation and libraries.
3. Reproduce or add a minimal failing fixture before changing complex logic when practical.
4. Implement the smallest correct change.
5. Validate analytically on simple shapes before testing real-world drawings.
6. Exercise relevant topology edge cases.
7. Use `$cam-validation` for regression/golden/property testing.

Read `references/cam-invariants.md` for standard invariants.
Read `references/numerical-tolerances.md` before adding or changing floating-point tolerances.

## Important topology cases

Consider closed/open paths, holes and islands, nested contours, concavity, self-intersections, duplicate points, tiny segments, arcs, orientation, disconnected regions, and inaccessible narrow regions when relevant.

## Output

Explain geometric intent and validation, not just code structure. If correctness cannot be established, state what remains unproven.
