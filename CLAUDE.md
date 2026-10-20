# FabCAM instructions for Claude and other coding agents

[AGENTS.md](AGENTS.md) is the shared repository instruction source. Read it first,
then [the documentation index](docs/README.md), [STATUS](docs/shopbot-modernization/STATUS.md),
[V1 readiness](docs/releases/V1_READINESS.md) and the relevant skill/contracts.
It owns M0–M4 governance, FS/process boundaries, Git authorization, parallel-work
isolation, task tracking and human HARD STOP requirements. Do not duplicate or
weaken those rules in agent-specific notes.

## Current source and parallel work

NUMSAFE and NUMSAFE2 are committed in `e1a0c95` and `2c6e4e1`; they are no longer
paused/uncommitted. Use [STATUS](docs/shopbot-modernization/STATUS.md) and the
[reconciliation report](docs/releases/PROJECT_STATE_RECONCILIATION_2026_10_20.md)
for the integrated baseline, actual checks and still-open human gates.
Inspect Git/worktrees/tracker before new work and preserve all local-only evidence.

## Build, tests and compatibility

- [BUILDING](docs/BUILDING.md) and [Windows setup](docs/BUILDING_WINDOWS.md) own setup
  and transport-specific commands. Rust is pinned by `rust-toolchain.toml`, Node
  by `.nvmrc`; use the committed frontend lockfile. Do not edit toolchain policy
  merely to make a historical command pass.
- [CONTRIBUTING](docs/CONTRIBUTING.md) owns CAM/post/machine extension recipes,
  generated-schema workflow, validation and commit conventions.
- [General architecture](docs/ARCHITECTURE.md), [fork contracts](docs/shopbot-modernization/ARCHITECTURE.md)
  and [ADRs](docs/shopbot-modernization/adr/README.md) own layer/process/compatibility
  boundaries. State mutations use the command/history path; posts own dialect
  differences; transports remain thin.
- Never hand-edit `frontend/src/lib/api/generated.ts`; update Rust contract types
  and use the documented schema/codegen checks. No implicit project migration.
- Workspace package version is authoritative; use [the bump procedure](docs/BUILDING.md#versioning)
  and [VERSIONING](docs/releases/VERSIONING.md), not independent version edits.
  Preserve the documented Android generated-properties refresh requirement.
- Run `scripts/pre-release.sh` only for an authorized release/candidate gate; its
  software result is not installation, platform or physical ShopBot acceptance.

## Handoff

[AGENTS](AGENTS.md) defines the shared completion procedure. Record exact source,
actual checks (including skips/failures), pending human gates and remaining changes.
Do not push, merge or start the next milestone without explicit authorization.
