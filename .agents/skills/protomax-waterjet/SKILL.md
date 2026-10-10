---
name: protomax-waterjet
description: Analyze or implement future OMAX ProtoMAX/waterjet CAM semantics, OMX output, piercing, kerf, quality, leads, or waterjet integration.
---

# ProtoMAX / waterjet

Use this skill only when the task explicitly concerns waterjet, OMAX, ProtoMAX, OMX, or the future process architecture needed for them.

The current ShopBot M0–M4 track takes priority unless the user explicitly starts waterjet work.

## Principle

Reuse shared ivaCAM infrastructure while keeping waterjet semantics explicit. Do not model waterjet as milling with fake spindle, Z-depth, or tool-change values.

## First workflow

Before implementation:

1. inspect current ivaCAM laser/plasma/drag-knife process abstractions,
2. inspect how process-specific events reach simulation and post-processing,
3. document what can be reused,
4. identify the smallest missing waterjet semantics,
5. decide whether extension is additive or requires an architectural ADR.

Read `references/waterjet-concepts.md` for the conceptual model.

## Likely domains

Future work may include 2D cut path, traverse, pierce, kerf compensation, inside/outside/slit semantics, cut quality, lead-in/lead-out, etch, operation ordering, OMX generation, and handoff to OMAX MAKE.

Verify actual ProtoMAX/OMAX behavior and format documentation before implementing machine output.

## Boundary

The application should generate manufacturing intent/toolpaths and compatible output. Do not assume it should replace OMAX MAKE or direct machine control.
