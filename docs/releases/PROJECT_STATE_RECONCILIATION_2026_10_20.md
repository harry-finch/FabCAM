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
here. This is a software-development disposition, not removal of the approved release
plan's minimal selected trials before recommending real ShopBot cutting settings.
A source/data revision never recalculates saved applied/manual values silently.
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

The compared engineering tree is **2c6e4e11ac3d8bc9e8d3b959ffa76c6e50c6669a**.
Both full runs started with HEAD **f814c2697ee9c1faabe210820a7ea6cdd9bed14c**,
plus documentation-only follow-ups described here. No runtime/test/schema/script
file differs; the final documentation receipt does not change the tested software.

### Pre-existing macOS gate defect — ivac-5yxyu

The first full script exited **0**, but `wasm-size-guard.sh` called unsupported
macOS `stat -c %s` four times, then treated empty measurements as a zero-byte delta
and reported PASS. This **is not accepted as complete gate evidence**, despite the
script's exit status. Original raw log: [first run](project-state-reconciliation-2026-10-20-evidence/pre-release.log).
The problem exists unchanged at `2c6e4e1`; no production/script correction is made here.
New issue **ivac-5yxyu OPEN** records the portable/fail-closed maintenance follow-up.

For the qualified rerun only, a scoped external PATH entry supplies the exact GNU
`stat -c %s <file>` invocation via native `/usr/bin/stat -f %z`. The
[wrapper source](project-state-reconciliation-2026-10-20-evidence/macos-stat-wrapper.sh.txt)
is retained as evidence, not installed into the repository scripts or user tools.
Its output was independently checked against Python byte lengths for zero/one/1024/
1048577-byte files with spaces and all generated WASM/gzip artifacts:
[controls](project-state-reconciliation-2026-10-20-evidence/stat-compatibility-validation.json).
Real measured gzip sizes are 1940417 and 3565228 bytes, delta1624811 bytes (~1587 KiB),
within the existing 1700 KiB budget. The full script is rerun, not merely the size step.
A default macOS run without this environment qualification retains the known defect.

### Qualified full-run result

**Second complete run: exit 0, 16/16 gates PASS, no gate skipped.**
[Raw qualified log](project-state-reconciliation-2026-10-20-evidence/pre-release-qualified.log) ·
[Machine-readable receipt](project-state-reconciliation-2026-10-20-evidence/qualification.json).

| Gate group | Observed result |
|---|---|
| Rust formatting / pedantic Clippy | PASS, warnings denied |
| `cargo test --workspace --all-features` | **1635 passed, 0 failed, 1 ignored**; 53 result blocks including doctests |
| xtask schema-check / README parity | PASS; both README files share 12 headings |
| pnpm frozen install / API codegen / i18n drift | PASS; generated files unchanged |
| Frontend lint / check | PASS; Svelte 0 errors / 0 warnings |
| Frontend tests | **1458 passed, 1 skipped**,163 files |
| Frontend build / wasm-pack | PASS; existing chunk-size/dynamic-import warnings retained |
| WASM CPS size guard | PASS with qualified `stat` wrapper:1587 KiB gzip delta,1700 KiB budget |
| cargo-deny | PASS under existing committed advisory policy; no policy/dependency edits |
| Headless Chrome UI smoke | PASS, tabs and120-tool search/sort/paging/selection; no page exceptions |

Tools: Node 24.21.0,pnpm 10.34.7,Rust 1.88.0,wasm-pack 0.15.0,cargo-deny 0.20.2;
Chrome exec wrapper already documented in BUILDING plus the byte-length-qualified
`stat` shim described above. PATH includes the pinned Node/pnpm/Cargo tools and
cached wasm-bindgen 0.2.120/wasm-opt binaries; `RUSTC_WRAPPER=''`.
Recreate the scoped `stat` wrapper from its retained source before reproducing the
qualified run on macOS. No user/global tool or production script was modified.

Additional checks: `cargo run --quiet -p xtask -- version-check` exits0 (beta.3
aligned); relative Markdown paths/anchors checked with no missing/escaping targets;
26 ADRs indexed and byte-unchanged versus 2c6e4e1; four historical copies preserve their
original text after reversing relocation links and ignoring EOF newline counts
([comparison](project-state-reconciliation-2026-10-20-evidence/history-preservation.json)).
Guide helper `python3 docs/feeds-speeds/examples/validate_examples.py` passes both
actual-loader packs,6 matching contexts,6 negative controls and the revision pair.

Documentation/source whitespace checks excluding the two verbatim terminal logs
pass. The default cached whitespace check flags renderer trailing spaces in those
raw logs; their original bytes are deliberately preserved, not presented as a clean
raw-log formatting check. No whitespace policy is changed. The initial engineering range
`73b66bb..2c6e4e1` already contains an extra blank EOF in ADR-0026; it was recorded,
not silently changed in an immutable decision. All engineering source, tests,
schemas, scripts, deny policy, ignore policy and original local evidence register
remain identical to 2c6e4e1. No installed-platform or physical result is inferred;
no new native packaging, manual desktop campaign or controller run was performed.

## Final Git state and checkpoint

- Engineering synchronized: the four commits above, through **2c6e4e1**.
- Documentation merge: **f814c2697ee9c1faabe210820a7ea6cdd9bed14c**,
  parents 2c6e4e1 and 744774d; a90cb64,d63aa89,744774d remain original ancestors.
- Final reviewed documentation/qualification receipt is the commit titled
  `docs: record integrated qualification and reconciliation checkpoint`.
  Its exact hash is available with
  `git log -1 --format=%H --grep='^docs: record integrated qualification and reconciliation checkpoint$'`;
  the handoff records that hash and actual push result. This receipt changes
  documentation/evidence only; tested engineering tree remains 2c6e4e1.
- Target **origin/main**, `https://github.com/harry-finch/FabCAM.git`.
  Pre-push fetch confirmed origin still 2c6e4e1,0 remote-only commits; synchronization
  uses an ordinary fast-forward push. After success, `main` and `origin/main`
  identify the receipt commit. If push/alignment fails, retain local commits and
  HARD STOP; never force or rewrite history.
- Documentation worktree/branch remains clean at 744774d as original reviewed source;
  historical feature/shopbot-mvp remains c9d3f3b. No worktree is reset or removed.
- **8211 original untracked evidence files unchanged by SHA-256**,586210786bytes;
  [inventory/digest](project-state-reconciliation-2026-10-20-evidence/local-evidence-inventory.json).
  One additional local-only redundant copy of the first raw run is retained at
  `project-state-reconciliation-2026-10-20-evidence/pre-release-first-run.log`;
  identical authoritative bytes are committed as `pre-release.log`. Thus remaining
  untracked qualification files total 8212; no source changes remain uncommitted.
- Tracker synchronization notes appended without changing old history or closing
  human gates. `ivac-5yxyu` is the newly confirmed gate-portability follow-up; its
  implementation is not started here. Retention proposal still requires approval.

**HARD STOP after this checkpoint.** Remaining decisions are the listed human reviews,
release-gate portability maintenance, separately authorized matching candidates and
actual platform/workshop qualification. No FS0,FS4 or other functional milestone,
package,tag,Release or hardware operation is launched.
