# Project state reconciliation — 2026-10-20

Engineering task **ivac-7sm9v**. User authorized ordinary synchronization of four
approved engineering commits, integration of completed documentation, state reconciliation
and software validation. No next functional milestone, native packaging, release or
hardware operation. No evidence deletion, relocation or retention-policy modification.

## Initial Git inventory

- Primary worktree `ivacam/`: `main` at **2c6e4e11ac3d8bc9e8d3b959ffa76c6e50c6669a**;
  fetched `origin/main` at `73b66bb`, zero commits behind and exactly four ahead.
- `origin`: `https://github.com/harry-finch/FabCAM.git`; `upstream`:
  `https://github.com/aalarchiv/ivacam.git`. Authenticated account `harry-finch`.
- Documentation worktree `../FabCAM-docs-consolidation/`:
  `codex/docs-consolidation` at `744774d`, based on `94cdd6b`; clean.
- Other local branch `feature/shopbot-mvp` at `c9d3f3b`, historical; left untouched.
- Both worktrees had no tracked modifications. Primary had **8211 untracked local
  qualification files / 586210786 bytes**. Hash baseline recorded before integration.
- Tracker showed NUMSAFE `ivac-knphk` and NUMSAFE2 `ivac-o9gwj` CLOSED with explicit
  acceptance/commit records. No unfinished NUMSAFE source was present in the index
  or working tree. Other IN_PROGRESS records are preserved below; they do not imply
  permission to modify another agent's work. No task was messaged or interrupted.

The first network check failed under sandbox DNS restrictions. An approved external
fetch/authentication check succeeded and confirmed **0 behind / 4 ahead**. No remote
was changed; no upstream integration was attempted.

## Engineering synchronization

| Commit | Inspected intended scope |
|---|---|
| `d30863d82f2e933e4aa994dadad8ed69d4de8654` | DXF OCS / BLOCK base / INSERT correctness, explicit unsupported cases, SVG/DXF ambiguity warnings, analytical fixtures and audit evidence |
| `94cdd6bbdb5887ffce48adb86b8ea8a088402ac9` | ADR-0026 explicit import units/options and visible warnings, transport/schema/UI integration, persistence/reload tests and browser evidence |
| `e1a0c95ced5e46b34f0832d9562483150932752d` | NUMSAFE output/parameter refusals, bounded STL allocation and WASM trap recovery; regression and output-identity evidence |
| `2c6e4e11ac3d8bc9e8d3b959ffa76c6e50c6669a` | NUMSAFE2 1000-pass refusal and explicitly invalid depth-step refusal; tests/reports plus Chrome wrapper setup clarification |

Commit scopes, changed paths, reports and tracker closure were checked against the
requested checkpoints. No unrelated staged or unfinished work was included. Ordinary
push **73b66bb..2c6e4e1 → origin/main** succeeded. This synchronizes approved source,
not release acceptance. No force, rebase, reset, clean, tag or Release.

## Documentation integration and conflict resolution

Original commits retained as merge ancestry:

| Commit | Completed documentation batch |
|---|---|
| `a90cb64` | Compact STATUS/readiness, central index, roadmap responsibilities and preserved historical navigation |
| `d63aa89` | Agent/developer onboarding, complete ADR index, portable references and inventory/retention proposal |
| `744774d` | Finished custom cutting-data guide, two synthetic source-pack revisions, reproducible actual-loader validation and navigation |

The guide was **completed**, not ongoing authoring: clean committed worktree,
engineering `ivac-sq3it` CLOSED, source/loader validation recorded. It is included as
completed documentation; human review `ivac-7w1e1` stays OPEN. Review of the branch
found no production/architecture/policy change: retention options explicitly remain
unapproved proposals. All original engineering/documentation commits remain traceable.

`git merge --no-ff --no-commit codex/docs-consolidation` produced one textual conflict:
**STATUS**. Resolution keeps the compact structure and carries forward newer NUMSAFE
and NUMSAFE2 commits/limits, replacing stale paused/uncommitted wording. DEVLOG merged
automatically; its newer NUMSAFE entries are preserved. CLAUDE/index/readiness were
semantically resynchronized. Historical reports and all accepted ADR files are unchanged.

## Documentation structure and authority

Before: long STATUS (747 lines at the audited docs base), track roadmap (728 lines),
release preambles and dated remaining-gates file mixed current state with old OPEN
notices. After: concise current documents with links to preserved original records.

| Canonical entry | Responsibility |
|---|---|
| [docs index](../README.md) | Onboarding and routes to setup, architecture, sources, qualification and plans |
| [STATUS](../shopbot-modernization/STATUS.md) | Current source, implementation/qualification boundaries, blockers and next decision |
| [ROADMAP](../shopbot-modernization/ROADMAP.md) | Actual remaining tracks/dependencies and links to historical milestone contracts |
| [RELEASE_ROADMAP](RELEASE_ROADMAP.md) | Approved V1 / V1.5 / V2+ product scope; historical maturity explicitly labelled |
| [V1_READINESS](V1_READINESS.md) | Current gate dispositions, evidence and acceptance criteria |
| [DEVLOG](../shopbot-modernization/DEVLOG.md) | Append-oriented engineering history; concise integration checkpoint |
| [HISTORY](HISTORY.md) | Report succession and original status/roadmap snapshots, without relocating evidence |
| [ADR index](../shopbot-modernization/adr/README.md) | Accepted/proposed decision navigation; original decisions and upstream attribution retained |

Consolidation repairs include six repository-external skill hyperlinks replaced by
versioned repository references, two unavailable external UX input hyperlinks made
explicitly unavailable text references, and the local capture gallery linked through
its tracked delivery inventory. README language structure remains paired. The earlier
[documentation checkpoint](DOCUMENTATION_CHECKPOINT_2026_10_20.md) retains the detailed
before/after inventory; its paused-NUMSAFE wording is historical, superseded here.

## Feeds & Speeds milestone matrix

All eight milestones were planned and then delivered in bounded scope; none is an
unstarted FS0 replacement. “Software qualified” below means the tests/source and
limits named in the linked report, with the integrated rerun separately recorded.
No row claims measured cutting suitability, FS4 evidence, or V1 release acceptance.

| Milestone | Implementation and code/contract evidence | Recorded software qualification | Human disposition verified in tracker | Physical / release status |
|---|---|---|---|---|
| FS0 | Domain/ownership architecture, [ADR-0003](../shopbot-modernization/adr/ADR-0003-feeds-speeds-data-ownership.md), subsequently realized by the independent crate | [FS0 audit](../shopbot-modernization/feeds-speeds/FS0_QUALIFICATION.md); 19 existing resolver controls; FS0 itself documentation only | `ivac-9ma3` CLOSED: explicit approval 2026-10-12 | Physical n/a to design; no release acceptance inferred |
| FS1 | `ivac-feeds-speeds/src/{model,engine,matching,rank,validation}.rs`; independent recommendation library | [FS1](../shopbot-modernization/feeds-speeds/FS1_QUALIFICATION.md): 38 synthetic engine tests + doctest | `ivac-406n` CLOSED: explicit approval and FS2 authorization | Software only; no physical or V1 acceptance |
| FS2 | `ivac-feeds-speeds/src/dataset.rs`, versioned references and source attribution | [FS2](../shopbot-modernization/feeds-speeds/FS2_QUALIFICATION.md): 38 engine +11 dataset +1 doctest | `ivac-otng` CLOSED: explicit approval before FS3 | Reference data is not ShopBot/lab cutting qualification |
| FS3 | Core `feeds_speeds.rs`, saved `AppliedCutting`/rate slots, frontend explicit Apply/manual/persistence and resolved post consumption | [FS3](../shopbot-modernization/feeds-speeds/FS3_QUALIFICATION.md): 1218 non-Tauri workspace tests, 1056 frontend +1 skip, software browser/save/reopen checks | `ivac-vbaa` CLOSED as **superseded**, not standalone usability approval; successor accepted FS3.5 | No physical evidence; saved manual values remain authoritative |
| FS3.5 | Job material, operation-owned cutting values, assistant and source/tool disclosure in core/frontend state | [FS3.5](../shopbot-modernization/feeds-speeds/FS3_5_QUALIFICATION.md): 10 adapter,1065 frontend, ShopBot8, compatibility/build checks | `ivac-eh1g` CLOSED: user found functionality satisfactory and authorized FS3.6 | No FS4 trial or blanket release qualification |
| FS3.7 | `source_pack.rs`, checked loader/resolver, source storage and private-safe persistence; normalized cutting knowledge | [FS3.7](../shopbot-modernization/feeds-speeds/FS3_7_QUALIFICATION.md): 69 engine/data/pack +1 doctest,13 adapter,2 native storage tests | `ivac-tm7u` CLOSED: accepted bounded delivery 2026-10-18 | No wider tool/material or physical evidence; attribution/privacy boundaries retained |
| FS3.8 | `LabPreset`, `LabReview`, core capture and frontend `lab-presets*`; Draft/Tested/Validated authoring workflow | [FS3.8](../shopbot-modernization/feeds-speeds/FS3_8_QUALIFICATION.md):77 +1 doctest,16 adapter,3 storage,1087 frontend +1 skip | `ivac-7yii` CLOSED: accepted bounded delivery 2026-10-18 | Ability to record trials is not proof a trial occurred; generic authoring still JSON-based |
| FS3.9 | `adaptation.rs`, bounded matching/transfer/interpolation, explanation trace and saved provenance | [FS3.9](../shopbot-modernization/feeds-speeds/FS3_9_QUALIFICATION.md):94 +1 doctest,17 adapter; [sensitivity](../shopbot-modernization/feeds-speeds/FS3_9_REVIEW.md):98 +1,11997 samples | `ivac-t0fk` CLOSED: accepted bounded delivery 2026-10-18 | Narrow reference coverage; physical adaptation untested; smoothing still proposed |

The [2026-10-18 acceptance](ACCEPTANCE_CANDIDATES_2026_10_18.md) also records FS3.6,
FS3.9.1 and related review IDs (`ivac-anp0`, `ivac-vgu4`); historical report OPEN headers
do not reopen those decisions. **FS4 remains DEFERRED / NONBLOCKING**, not authorized
here. A source/data revision never recalculates saved applied/manual values silently.
Aggressiveness and evidence remain separate; original source attribution/licensing,
missing-data refusals and unsupported operation/tool/process coverage remain intact.

## Current readiness and human gates

- NUMSAFE `ivac-knphk`, NUMSAFE2 `ivac-o9gwj`: CLOSED/accepted engineering source;
  no longer unfinished. NUMSAFE2 supersedes the earlier deferred NS-6b/invalid-step
  policy in the immutable NUMSAFE report. 1000 is maintainer-selected, not silently
  resampled. Both reports retain their actual historical test statements.
- ADR-0024 architecture approved and implemented; human checkpoint `ivac-3i00w` OPEN.
  Engineering defects `ivac-3xqtb` / `ivac-52tpd` closed; no stale unresolved claim.
- ADR-0025 architecture approved, implemented and corrected; `ivac-b0js3` /
  `ivac-b0js3.1` IN_PROGRESS. Back-face fixtures/dowel registration remains unqualified.
- ADR-0026 authorized implementation present in `94cdd6b`; `ivac-feder` IN_PROGRESS,
  human UX/architecture acceptance pending despite source synchronization authority.
- UX A/B/C accepted; D `ivac-he4lt` and consolidation `ivac-amibd` OPEN, with later
  ADR-0024 corrections referenced instead of reopening their fixed defects.
- Documentation engineering `ivac-lgudk` and guide `ivac-sq3it` CLOSED; reviews
  `ivac-yg3sm` / `ivac-7w1e1` OPEN. No gate automatically closed by this intervention.
- `ivac-4ql18` remains IN_PROGRESS/human despite already committed/pushed comment
  safety; `ivac-2p53v` and `ivac-2p53v.4` retain platform/macOS distribution work.
  Other active records `ivac-58nl.1.1`, `ivac-e4ic` are untouched, not folded into V1.
- Open milling risk/reserves: `ivac-2zll` ShopBot RPM transition; `ivac-53ky9` large
  pocket budget; `ivac-42ylx` cancellation latency; `ivac-jtkj` two-sided CPS/shifted
  registration. Scope/triage decisions belong to the maintainer, not this docs task.
  `ivac-6lehl` waterline/relief/thread pass-count review and `ivac-q425p` repeated
  same-tool pauses remain follow-ups; streaming export may leave a failed partial file.
- Existing beta.3 installers are source **db5bde5 / pipeline61**, not current pipeline63
  source. Matching packaging, clean-host macOS/Windows11/Ubuntu runtime and supervised
  ShopBot/controller/material records remain separate unperformed acceptance gates.
  Historical M4 qualitative evidence is not release-wide physical qualification.
- V1.5 native ProtoMAX/MAKE retains accepted scope, lawful OMX1 and ARCH-MACH dependencies;
  AUDIT3 follows accepted V1 before main V1.5 implementation. V2+ advanced CAM/STEP/smart
  workflows/MCP remain planned. No FS0/FS4/new milestone is launched.

## Evidence inventory and retention proposal

Before integration, SHA-256 captured all **8211** untracked files (586210786 bytes):

| Local evidence group | Untracked files |
|---|---:|
| `tool-provenance-implementation-evidence` | 4169 |
| `ux-consolidation-2026-10-19-evidence` | 2501 |
| `ux-safety-load-2026-10-19-evidence` | 1541 |

See [local evidence register](LOCAL_EVIDENCE_2026_10_19.md) and the [full earlier inventory](DOCUMENTATION_CHECKPOINT_2026_10_20.md).
That earlier audit measured docs at 9834 files /715577083 bytes, of which tracked
1621 /129341713 bytes; it is a dated inventory, not the post-integration total.
External root validation kits and native capture galleries remain in place. Build
caches and ignored outputs are not cleaned; no untracked file is staged or removed.

**Proposal only, no policy adopted:** keep canonical contracts/ADRs/attribution and
unresolved findings in Git; preserve reports, failure/intermediate cases, source/tool
identity and hashes; maintain a lightweight index for large per-case evidence.
Evaluate verified redundant copies and immutable external archives only in a separately
approved batch, with restore checks and durable references before any deletion or
relocation. Byte duplication alone does not prove evidence is disposable. Approval
is still required for retention changes, evidence moves/deletion or external archives.

## Integrated validation

Pending execution on the merged source. This section will record the exact commit,
full script exit status, tools, every gate and skips/failures before final push.
No green claim is made until the entire script exits 0.

## Final Git state and checkpoint

The final merge/report commit IDs and remote alignment will be recorded after
validation. Original commits are preserved; documentation worktree/branch stays
available. No new functional work, package, tag, Release or hardware operation.
**HARD STOP after this checkpoint.**
