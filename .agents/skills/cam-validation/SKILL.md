---
name: cam-validation
description: Design or run CAM correctness tests, regression fixtures, golden tests, invariants, simulation checks, or physical validation protocols.
---

# CAM validation

Use this skill when proving CAM behavior, preventing regressions, or defining acceptance criteria.

## Validation hierarchy

Prefer the lowest-cost proof that establishes the required property:

1. analytical/unit test,
2. deterministic geometry fixture,
3. invariant/property test,
4. golden/semantic output comparison,
5. simulation/visual inspection,
6. trusted CAM comparison,
7. controller preview,
8. supervised physical validation.

Do not replace analytical validation with screenshots.

## Workflow

1. State the behavior to prove.
2. Choose the simplest geometry that exposes it.
3. Define expected invariants before inspecting generated output.
4. Add regression coverage for bugs.
5. Separate pre-existing failures from failures caused by the change.
6. Record exact commands and results when working under a project milestone.
7. Never claim physical validation unless a human actually performed it and supplied the observations.

Read `references/test-pyramid.md` for test selection.
Read `references/physical-validation.md` for hardware-validation boundaries.

## Golden tests

Use byte-for-byte golden files only when formatting itself is part of the contract. Otherwise prefer semantic comparison.

A CAM change is not complete merely because tests pass if the tests do not establish geometric intent.
