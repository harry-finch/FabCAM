# FabCAM agent entry point

## Authority and onboarding

Current production source and accepted versioned contracts override historical
reports, old examples and superseded skill instructions. Check the actual branch,
HEAD, worktrees and dirty state before interpreting a document as current.

Start with [STATUS](docs/shopbot-modernization/STATUS.md), the
[documentation index](docs/README.md), [cutting contract](docs/CUTTING_CONTRACT.md),
[architecture](docs/ARCHITECTURE.md) and [accepted ADRs](docs/shopbot-modernization/adr/README.md).
STATUS owns the current accepted source/provenance and blockers; the
[technical roadmap](docs/shopbot-modernization/ROADMAP.md) owns workstreams;
[RELEASE_ROADMAP](docs/releases/RELEASE_ROADMAP.md) owns release/acceptance gates.
[Frozen V1 readiness](docs/releases/V1_READINESS.md) is a separate historical line.
[HISTORY](docs/releases/HISTORY.md) and dated reports retain source-bound evidence.
Update current sections in place; never prepend another status-banner stack.

## Current contracts

ProjectFile version 10 and native `project.cutting_version: 10` are current.
Profile/ordinary Pocket use explicit Main Cutting and optional Finishing under
ADR-0032/0033; other families retain their validated actual-role assignments.
Preset collection V2, strict tested matching and separate ALPHA-4 reviewed starting
values are described in [TESTED_PRESETS](docs/TESTED_PRESETS.md). Do not migrate V9,
map Rough/Finish to Main/Finishing, transfer physical attestation or invent defaults.

ADR-0030 separates FabCAM from FabSpeed. The integrated recommendation/source-pack
engine is removed. FabCAM owns explicit cutting values, CAM validation and presets;
FabSpeed calculation/experiments and optional V1.2 exchange remain independent/planned.
Postprocessors never call a live calculator. FS0–FS3.9 reports/formulas are historical
references, not instructions to recreate the deleted engine.

Preserve SAFETY-FIX-1 motion refusals and SEC-HARDEN-1 authorization/resource limits.
Software qualification, installed-native acceptance, release approval and physical
ShopBot qualification are separate gates. M0–M4 evidence keeps its original limits;
waterjet/ProtoMAX and vinyl remain deferred. Never weaken a guard to satisfy an example.

## Skills and implementation

Use the relevant **versioned** `.agents/skills/` and its referenced contracts.
A host catalogue may point at a separate copy: note divergence and consult the
versioned skill; do not overwrite another checkout or treat an obsolete skill as authority.
See [CONTRIBUTING](docs/CONTRIBUTING.md) and [BUILDING](docs/BUILDING.md).
Keep transports thin, CAM neutral and dialect differences in posts. Use command/history
mutations. Never hand-edit generated schemas/API types. Significant changes require
the ADR workflow and appropriate source-bound validation.

## Git, evidence and human gates

`origin` is the FabCAM fork, `upstream` is ivaCAM; inspect actual remotes. Preserve
upstream attribution. Do not change remotes or synchronize upstream incidentally.
Use an isolated authorized worktree when work is concurrent. Preserve unrelated
changes, untracked user data and ignored qualification evidence; never stash, clean,
reset or stage another agent's files. A clean worktree is not a complete evidence archive.
Commits need explicit task authorization. Merge, push, rebase, destructive cleanup,
publication and installers each require their own scope/approval; no automatic PR.

Use `bd prime` and Beads for tracking; preserve other tasks and leave human gates
open until accepted. Before handoff, review exact paths, validate relevant checks,
record source/limits and remaining changes, and stop at the requested human checkpoint.
Do not start the next milestone merely because it appears in a roadmap.
