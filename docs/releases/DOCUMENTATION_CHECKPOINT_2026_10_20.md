# Documentation consolidation checkpoint — 2026-10-20

Work `ivac-lgudk`; human review **ivac-yg3sm OPEN**. Documentation-only branch `codex/docs-consolidation`, base
**94cdd6bbdb5887ffce48adb86b8ea8a088402ac9**, beta.3/pipeline63.
No production/test/schema/output/security change, candidate, push, merge or release.
**HARD STOP after documentation. Evidence-retention proposals below are NOT policy.**

## Isolation and NUMSAFE resynchronization

Primary checkout: sibling `ivacam/`. Documentation worktree: sibling
`FabCAM-docs-consolidation/`. NUMSAFE `ivac-knphk` is paused according to the user;
at inventory it has 14 tracked modified files and 4 untracked source/test files.
No uncommitted primary source or local qualification evidence was copied into this
branch, staged or cleaned. The primary source/evidence baseline was SHA-256 captured
outside the repository for final comparison. Worktree Git registration and separate
branch commits are intentional; primary HEAD/index/source are not changed by them.

After Claude finishes, inspect its actual commit/diff, version/pipeline, report,
regression outcomes, pending risks and human acceptance. Then update STATUS/readiness
and assess combined-source candidate freshness. Historical test figures here do not
qualify NUMSAFE; no current CAM tests were run for documentation-only edits.

## Before / after document responsibilities

Before: STATUS 747 lines and roadmap 728 lines accumulated historical checkpoints;
release roadmap 466 lines also repeated status; dated V1 remaining-gates entry 35 lines
contained contradictory corrected/unresolved and old beta.2 statements. DEVLOG 2558
lines is history and remains append-oriented. No central `docs/README.md` existed.

After: [central index](../README.md), short [STATUS](../shopbot-modernization/STATUS.md),
track [ROADMAP](../shopbot-modernization/ROADMAP.md), one [release plan](RELEASE_ROADMAP.md),
[V1 readiness](V1_READINESS.md), [report succession catalogue](HISTORY.md).
Original STATUS/roadmap/readiness wording and release checkpoint preamble are copied
into explicitly historical navigation snapshots, with relative links rebased. All
accepted scope, exclusions, original reports, ADRs and evidence remain available.
No report is retroactively made more favorable, nor does an index confer approval.

## Evidence inventory — inspected primary checkout

Logical bytes, measured 2026-10-20. The isolated worktree contains tracked files only.
Numbers describe a snapshot, not a continuously synchronized inventory.

| Location/status | Files | Bytes | Meaning |
|---|---:|---:|---|
| All `docs/` | 9834 | 715577083 | 682.4MiB logical; approximately706MiB allocated |
| Tracked documentation/evidence | 1621 | 129341713 | Same content as inspected commit at inventory |
| Untracked evidence | 8211 | 586210786 | 559.1MiB, deliberately local TEST1 artifacts; NOT disposable |
| Ignored files in docs | 2 | 24584 | OS metadata |
| `docs/releases/` | 8759 | 657799983 | Reports plus most qualification artifacts |
| `docs/shopbot-modernization/` | 1057 | 57602175 | Fork contracts, history, fixtures and earlier evidence |

| File category | Files | Bytes |
|---|---:|---:|
| JSON | 6292 | 626829693 |
| Images | 363 | 33565251 |
| Logs/text | 2928 | 6231297 |
| Markdown | 233 | 2128330 |
| Archives/other | 18 | 46822512 |

Largest evidence groups: `tool-provenance-implementation-evidence/`313538625bytes,
`ux-consolidation-2026-10-19-evidence/`159515309bytes,
`ux-safety-load-2026-10-19-evidence/`117844689bytes. See the unchanged
[local register](LOCAL_EVIDENCE_2026_10_19.md) for the 6 per-case locations.
Largest tracked files include TEST1 `expanded-failed.tar.gz`13.38MB,
`intermediate-failed.tar.gz`11.60MB, `bounded.tar.gz`9.17MB and SAFE-Z
`test1-pipeline60.tar.gz`9.17MB. Failed runs are valuable traceability, not waste.

Exact file hashing found 1062 nonempty duplicate groups (7013 files), with 485271996 bytes
of repeated content as a theoretical upper bound, not a deletion recommendation.
Cold/warm/restored responses can be identical precisely to prove cache invariants;
case paths and campaign provenance must remain reconstructible. No exact duplicate
Markdown files were found; prose repetition is semantic rather than byte-identical.

Outside Git, beside the primary checkout:

| Local delivery folder | Files | Logical bytes |
|---|---:|---:|
| `Captures-FabCAM-beta3-2026-10-18` | 68 | 38851043 |
| `Validation-V1-2a3fe56` | 112 | 65685164 |
| `Validation-V1-beta3-2026-10-18` | 115 | 67071112 |

They are not available by cloning this worktree. [Delivery inventory](beta3-delivery-2026-10-18/README.md)
retains their provenance; their presence is not new platform/physical acceptance.
Ignored `dist-test/` contains 45 files / 88687310 bytes. Primary `target/` is about 44.6 GiB
allocated and belongs to ongoing compilation, outside this documentation intervention.
No build output or local file was removed.

## Retention proposal — approval required, no change enacted

The existing local register and `.gitignore` are unchanged. No evidence deleted,
moved, compressed, externally archived or uploaded. No Git history rewritten.

| Class | Proposed treatment | Preservation requirement |
|---|---|---|
| Accepted ADR/contract, report, scope, light regression fixture | KEEP tracked | Preserve acceptance history, licensing, stable IDs and test consumers |
| Current V1 / unresolved-defect evidence | KEEP complete and backed up | Exact source/configuration, failures, limits, manifest and retrieval |
| Superseded but valuable qualification | REFERENCE successor; retain original | Do not change old observations or discard failed/reduced cases |
| Large reproducible output | Consider archive only after separate approval | Complete original case paths/bytes, full SHA-256, source/dirty state, tools/build identity, commands/seeds, date/status, restore test |
| Duplicate/temporary or unknown importance | REVIEW before any removal | Byte identity alone is insufficient; preserve campaign meaning and licensing |

Preferred option to assess: backed-up local artifact storage plus a lightweight
tracked catalogue. Git retains convenient access but permanent history weight;
LFS adds server/quota dependency; durable artifact storage needs access/backup policy;
CI artifacts expire and are unsuitable as the sole archive; GitHub Releases would
be separate publication requiring explicit authorization. None is selected here.

The local register's abbreviated aggregate hashes are identification hints, not
complete durable verification keys. A future approved catalogue should retain full
hashes, stable storage IDs, retrieval instructions and appropriate access/licensing.
Publicly readable manufacturer references or local presets are not automatically
redistributable: preserve the [data rights matrix](../shopbot-modernization/feeds-speeds/DATA_SOURCES.md).
Replaying seeds on newer source does not replace original qualification evidence.

## Proposed cleanup manifest — not executed

| Path/group | Current role/size | Git | Proposal | Preservation/risk |
|---|---|---|---|---|
| Primary 8211 per-case files | 559.1MiB TEST1 evidence | No | KEEP; assess ARCHIVE later | Backup + full manifest + restore; high risk until approved |
| TEST1/SAFE-Z tracked archives | Historical successes and failures, ~44MiB together | Yes | KEEP / REVIEW | Never rewrite history; check report/test consumers |
| Three root `V1_*GATE*.md` reports | ~19.2KB release-tooling history | Yes | REFERENCE in history; optional later relocation | No move needed for discoverability; links must survive |
| Local validation/capture folders | ~171.6MB delivery records | Outside repo | KEEP LOCAL; catalogue | Preserve installer/source checksums and actual human records |
| Future generated campaign outputs | New runs only | To decide | Consider a dedicated ignored output root | No blanket ignore over existing untracked evidence |
| Agent skill copies outside/inside repo | Onboarding contexts | Mixed | Review source priority, no automatic deletion/sync | `cam-ux-ui` differs; repo has 10 skills, parent 8; do not overwrite independent instructions |

Safe navigation consolidation produces little disk saving: historical snapshots
are deliberately retained. If later approved and safely relocated, all untracked
per-case artifacts represent a maximum 559.1 MiB reduction of local `docs/`, not a
reduction in total retained data or Git history. No storage reduction is claimed now.

## Remaining approvals and next boundary

Human review of this documentation checkpoint is required. Existing UX, import,
provenance, platform and physical gates remain in [readiness](V1_READINESS.md).
Only after a separate decision may a retention/backup/retrieval pilot be implemented.
No evidence deletion, movement, external publication, broad ignore rule or irreversible
structural cleanup is authorized by the present documentation commits.

## Documentation validation

Checks performed on 2026-10-20 before the final documentation commit:

| Check | Result |
|---|---|
| Inline Markdown relative links, outside fenced code | 1711 checked, zero missing targets; no links escape the repository |
| Markdown heading / explicit-ID anchors | 19 checked, zero unresolved; old FabCAM/FabSBP roadmap anchor preserved |
| README language parity | `bash scripts/check-readme-parity.sh` PASS, 12 headings shared |
| ADR inventory | 26/26 indexed; every ADR document byte-identical to the source baseline |
| Historical navigation copies | Original text reconstructed after reversing link rebasing; original snapshots match baseline hashes |
| Git whitespace / scope | `git diff --check` PASS; only Markdown changed; no deleted paths |
| Primary preservation | 11009 tracked/nonignored file SHA-256 values unchanged, including NUMSAFE and all local evidence; primary HEAD unchanged and index unstaged |
| Retention / licensing boundaries | `.gitignore`, `.gitattributes`, LICENSE, local evidence register and qualification artifacts unchanged |

Link validation covers relative target existence and the Markdown anchors used here;
it is not an HTTP availability, full HTML, privacy or renderer certification. No
production tests, build, installer or release gate was rerun for documentation.

Corrections: two absent external audit-input hyperlinks now explicitly identify
unavailable source documents without fabricating contents; a local-only gallery
link points to the tracked delivery inventory and preserves its retrieval path;
six host-dependent skill links point to their versioned repository counterparts.
No external skill copy or accepted ADR was edited. ADR-0023/0025 index states now
quote their approved architecture documents rather than obsolete proposed wording.

Before/after on the isolated tracked documentation tree: 1621 files /129341713 bytes
before, 1629 files /approximately129.4MB after. The small growth preserves old
navigation text; no disk saving is claimed. Primary evidence counts remain unchanged.
STATUS is 50 lines, track ROADMAP 28 lines, readiness 51 lines; DEVLOG remains history.

Commit batches:

- `a90cb64` — `docs: consolidate current status and release navigation`.
  Relevant new links passed; two pre-existing missing external inputs remained for
  the next batch. Original snapshots were verified before this commit.
- `docs: clarify onboarding and validate documentation references` — second batch,
  containing final validation and the human-review gate. Obtain its exact identity
  with `git log --oneline 94cdd6b..codex/docs-consolidation`; hashes are also reported
  in the user handoff, avoiding a self-referential commit hash inside this file.

The branch is for review only. No push, merge, rebase, reset, clean or history rewrite.
**HARD STOP** at human review `ivac-yg3sm`; NUMSAFE and retention changes are separate.
