---
name: shopbot-opensbp
description: Implement or review ShopBot machine profiles, OpenSBP output, ShopBot post-processing, or ShopBot-specific CAM integration.
---

# ShopBot / OpenSBP

Use this skill only for ShopBot-specific work.

## Boundary

ShopBot-specific syntax and behavior belong in the machine profile, post-processor, or explicit adapter layers. Do not leak ShopBot commands into generic CAM algorithms.

## Source discipline

For machine syntax or controller behavior, use authoritative ShopBot/OpenSBP documentation when available, do not invent commands, record assumptions, and distinguish controller capability from ivaCAM capability.

If evaluating a third-party `.cps`, verify licensing before copying or adapting source.

## Post-processor workflow

1. Inspect ivaCAM's movement/event model and CPS/native post interfaces.
2. Confirm every required semantic event is available.
3. Choose CPS vs native Rust deliberately; write an ADR when the project reaches that decision.
4. Start with the smallest supported subset.
5. Fail explicitly on unsupported semantics.
6. Add deterministic output tests.
7. Validate generated files outside the physical machine before hardware use.

Read `references/shopbot-post-checklist.md` for post-processor review.

## Machine-profile workflow

Never invent real machine limits. Mark unknown values `TO VERIFY ON MACHINE`.

Track work envelope, safe Z policy, feed/plunge limits, spindle behavior, RPM range if applicable, tool-change behavior, park/home assumptions, controller/software assumptions, and arc support.

## Safety boundary

A successful parse or preview is not a physical-machine validation. Hardware testing remains a supervised human procedure.
