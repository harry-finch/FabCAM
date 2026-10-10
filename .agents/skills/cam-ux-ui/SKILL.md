---
name: cam-ux-ui
description: Design, implement, or review UX/UI for CAM workflows, geometry selection, operation setup, simulation, warnings, or machine/job configuration.
---

# CAM UX/UI

Use this skill for user-facing workflow or interface changes. Optimize for safe, comprehensible manufacturing intent rather than decorative UI.

## Product context

The application should feel simpler and more modern than legacy VCarve while preserving its strongest workflow:

```text
select geometry → choose operation → set parameters → preview → validate → export
```

The user may be an experienced fabmanager or an occasional fablab user.

## Core UX principles

1. Make manufacturing intent visible.
2. Separate geometry, operations, machine/job setup, and validation conceptually.
3. Prefer progressive disclosure over exposing every advanced CAM parameter at once.
4. Use domain language with concise explanations where useful.
5. Show consequences visually: cut side, depth, tabs, leads, stock origin, fixtures.
6. Treat warnings and safety states as workflow information, not modal noise.
7. Do not hide dangerous ambiguity behind smart defaults.
8. Keep interactions efficient for repeated professional use.

Read `references/cam-ux-principles.md` for detailed patterns.
Read `references/accessibility-and-safety.md` when implementing warnings, colors, validation states, or destructive actions.

## Before implementing

Identify the user goal, current workflow, highest-risk misunderstanding, minimum information required, advanced parameters that can be deferred, persistent vs operation-local state, and undo/redo expectations.

Reuse the existing Svelte/Tauri design system and components where sensible before introducing new UI frameworks.

## Review output

Distinguish usability issues, safety/comprehension issues, information-architecture issues, and visual-polish issues. Prioritize workflow and correctness before aesthetics.
