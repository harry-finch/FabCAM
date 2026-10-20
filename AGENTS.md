# Agent Instructions

## Fork governance after M4

This section takes precedence over the upstream Git/session assumptions below.
Read `docs/shopbot-modernization/STATUS.md` and `ROADMAP.md` before new work.
M0–M4 are the completed ShopBot foundation; M4 baseline is
`e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`. Preserve their names and evidence limits.
Use named future tracks: FS (Feeds & Speeds), UX (milling workflow), CAD
(geometry preparation), WJ (ProtoMAX/waterjet), VINYL (drag knife, lower priority).
Priority: FS foundation → priority milling UX/CAM improvements → WJ → VINYL.
Every milestone ends at human approval; recording a roadmap does not authorize
starting FS0 or implementing any later milestone.

The future FS assistant is milling-specific, independent of UI and toolpath
geometry generation, and independently testable. An adapter supplies CAM
context; accepted or manually edited parameters are explicitly persisted.
Post-processors must not call the engine dynamically. Manual overrides remain
authoritative; aggressiveness and evidence are separate. Waterjet/drag knife
must not depend on FS. See `docs/shopbot-modernization/feeds-speeds/README.md`
and ADR-0002. Use the architecture/documentation skills; use feeds-speeds only
if it exists, and do not create that skill without an explicit request.

The published baseline is now `main`; `feature/shopbot-mvp` is retained historically.
Origin is `harry-finch/FabCAM`, upstream is
`aalarchiv/ivacam`. Commits and pushes require explicit user authorization;
M4 publication approval does not authorize publication of subsequent work.
No automatic merge, PR, upstream sync or force-push. A reviewable documentation
handoff may remain uncommitted pending approval.

## Navigation and authority

Start with the [documentation index](docs/README.md), [current STATUS](docs/shopbot-modernization/STATUS.md)
and [V1 readiness](docs/releases/V1_READINESS.md). The single release plan is
[RELEASE_ROADMAP](docs/releases/RELEASE_ROADMAP.md); the track ROADMAP links historical
milestone contracts. Dated reports preserve their tested source and limits.
Accepted ADRs and qualification evidence must not be silently rewritten.

Use relevant versioned skills in `.agents/skills/`; read their `SKILL.md` and
referenced contracts. A host-provided skill catalogue can point outside this repo:
inspect the supplied path and note any divergence instead of overwriting either copy.
Use [CONTRIBUTING](docs/CONTRIBUTING.md) for extension recipes and [BUILDING](docs/BUILDING.md)
for toolchain/setup. Significant architecture changes follow the ADR workflow.

## Git and parallel-work boundaries

`origin` is the FabCAM fork, `upstream` is ivaCAM; inspect `git remote -v` rather
than assuming a remote is absent. Do not change remotes during ordinary tasks.
Commits require explicit task authorization; push, merge, rebase, reset, clean,
history rewriting and publication require their own authorization. Never infer
publication permission from an accepted milestone or a passing test suite.

When another agent has unfinished changes, prefer an isolated branch/worktree
from the agreed committed revision. Preserve the primary checkout and local-only
evidence; do not stage, stash, clean, restore or commit another agent's work.
A clean isolated branch does not imply that all qualification evidence is in Git.
Record the inspected revision and what must be resynchronized after parallel work.

## Tracking and handoff

Use `bd` for task tracking; run `bd prime` for the installed workflow. Typical
commands: `bd ready`, `bd show <id>`, `bd update <id> --claim`, `bd close <id>`.
Do not create Markdown task lists as a substitute for issue tracking. Preserve
other active tasks and do not close human-review gates merely because code is done.

Before handoff: review your scoped diff; run checks appropriate to the change;
commit only reviewed authorized files when commits are requested; report commit IDs,
remaining changes, evidence limits and the next human decision. Leave unrelated
uncommitted work untouched. Do not push automatically. Every milestone ends at the
requested human checkpoint / HARD STOP; do not start a subsequent milestone.
