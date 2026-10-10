---
name: ivacam-architecture
description: Review or change ivaCAM core architecture, project models, process boundaries, machine abstractions, or upstream integration.
---

# ivaCAM architecture

Use this skill for architectural work in the ivaCAM fork. Do not use it for a local algorithm change that does not affect boundaries or public models.

## Workflow

1. Inspect the relevant upstream code before proposing a new abstraction.
2. Identify whether the capability already exists and classify it as `KEEP`, `EXTEND`, `REPLACE`, or `DEFER`.
3. Preserve the separation between geometry/import, process-specific CAM logic, machine-independent path semantics, simulation, post-processing, and frontend/UI.
4. Keep ShopBot-specific logic out of generic CAM code.
5. Evaluate changes to generic process/toolpath abstractions against future non-milling processes, especially waterjet, without implementing speculative waterjet functionality.
6. Prefer the smallest reversible extension over parallel abstractions or rewrites.
7. For a significant architectural decision, use the project ADR workflow.

## Before changing a core type

Check serialization compatibility, generated schemas/API types, frontend consumers, tests/fixtures, simulation consumers, post-processors, and upstream merge impact.

Read `references/architecture-review-checklist.md` when the change touches a shared model or subsystem boundary.

Read `references/process-boundaries.md` when changing machine/process kinds, IR semantics, or process-specific behavior.

## Output

For an architecture review, report the existing implementation, proposed change, affected boundaries, compatibility risks, upstream delta, tests needed, and whether an ADR is warranted.

Do not redesign unrelated subsystems.
