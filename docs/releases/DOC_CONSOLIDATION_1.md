# DOC-CONSOLIDATION-1 — documentation and agent reconciliation

2026-11-01 · Documentation-only checkpoint · **Accepted; locally consolidated. HARD STOP.**

## Starting source and authority

Isolated `docs/doc-consolidation-1` starts at accepted SEC-HARDEN-1 documentation
commit `6bfecf3339524cc86e8c2bd83f168c63cbdd2860`, a descendant of committed
SAFETY-FIX-1 and the accepted INT-ALPHA-1/ALPHA-1–4/TOOL-META lineage. There is no
unexpected divergence. Canonical V1.1 remains at its historical HEAD, identified in
[STATUS](../shopbot-modernization/STATUS.md); no canonical integration is assumed.
Five pre-existing worktrees and their working states remain preserved.

Source/generated contracts, accepted ADRs, source-bound receipts and current tests
were inspected before historical prose. The original `audit-fabcam.md` was unavailable
in targeted root/worktree/canonical/security and tracked/untracked documentation
inventories; this report does not claim to reproduce its unpublished contents.
Two concise read-only agent reviews covered technical contracts/skills and public
navigation/examples. The primary agent made all edits. No architecture ambiguity
required Astra or any production change.

## Updated inventory and responsibilities

[Exact path inventory and proposed commit groups](evidence/doc-consolidation1/updated-files.json).

| Group | Reconciliation |
| --- | --- |
| STATUS / technical ROADMAP / RELEASE_ROADMAP | One current section per file; source/qualification/blockers, workstreams and release gates have distinct owners. No banner stack |
| HISTORY / V1_READINESS | Chronology indexes ALPHA → integration → safety → security; frozen V1 remains explicitly historical and routes to current STATUS |
| Architecture / CUTTING_CONTRACT / TESTED_PRESETS | V10, discriminators/scopes, bounded finishing, metadata, strict/advisory reuse and current security boundaries |
| README EN/DE / Quickstart / index | Existing authored-value workflows, accurate platform/localization limits and current navigation; historical screenshot labelled |
| CLI / BUILDING / CONTRIBUTING / CPS | Existing V10 fixtures and executable commands; source/version ownership, actual workspace crates, loopback limits and no obsolete current external-post workflow |
| ADR index | Existing ADR-0032/0033 links retained; later accepted/committed disposition clarified without rewriting accepted ADR text |
| AGENTS / CLAUDE / skills | Concise precedence/onboarding; current cutting/preset guidance; removed engine no longer instructed; historical formulas/references retained with scope notes |
| Historical FS guide/navigation | Explicitly historical; removed-source links labelled unavailable instead of fabricated replacements |

Future milestones update current sections **in place**. Original dated qualification
reports are not duplicated, rewritten or globally link-repaired. HISTORY preserves
chronology; Git retains the previous active-document text. No evidence was deleted,
relocated or externally archived and no retention policy changed.

## Authoritative current contract summary

- ProjectFile **10**, native `cutting_version: 10`; current capabilities include
  `cutting-parameters-v10` and `pocket-finishing-v2`. Default WASM has no CPS;
  native CPS depends on the enabled build. No invented drawing capability string.
- Drawing storage/GEOM1/generation is **V1**, scoped Analyze/Preview/Apply and its
  geometry payload **V2**. Local edit acceptance is not CAM certification.
- Profile/ordinary Pocket require Main and nullable optional Finishing.
  `main_finishing_v1` remains supported; Pocket-only `pocket_finishing_v2` adds
  strict `walls_only`, `floor_only`, `walls_and_floor`. Old versions and malformed,
  missing/mixed nested contracts refuse, without implicit migration.
- ALPHA-3.1 accepts bounded polygons beyond rectangles; ALPHA-3.2 adds bounded
  Pocket tangent curves/simple islands with explicit direct-entry/cutter/topology,
  axial/coverage/error budgets. Unsupported Spiral/proof cases refuse; no universal
  finishing, rest machining or complete geometry repair claim.
- Collection **V2** supports stages; V1 remains historical/inspectable with limited
  unchanged-family strict matching. V1 Profile/Pocket cannot migrate into V10 stages.
  ALPHA-4 separately copies reviewed RPM/feed/plunge/DOC as starting values; no
  stepover or source attestation transfers. Strict tested matching is unchanged.
- TOOL-META substrate/coating is descriptive; unknown differs from explicitly
  uncoated. FabCAM authors/validates values; FabSpeed is independent and exchange
  remains Proposed. No active manufacturer recommendation or chipload calculator.
- SAFETY-FIX-1 motion refusals and SEC-HARDEN-1 CSP/file authorization, loopback
  Origin/two-job admission/cancellation and parser budgets remain authoritative.
  Installed-native file workflows, authenticated remote HTTP and physical machining
  remain **unqualified**. No release/installer or functional CAM extension is implied.

Detailed semantics belong in [CUTTING_CONTRACT](../CUTTING_CONTRACT.md),
[TESTED_PRESETS](../TESTED_PRESETS.md) and [fork architecture](../shopbot-modernization/ARCHITECTURE.md).

## Qualification before commit approval

The following receipts preserve the pre-consolidation qualification checkpoint,
including their original no-commit/not-staged fields. They were not rewritten as
newly executed qualification. Final consolidation checks are recorded below.

[Verification receipt](evidence/doc-consolidation1/verification.json),
[CLI outcomes](evidence/doc-consolidation1/cli.json),
[source preservation](evidence/doc-consolidation1/source-preservation.json).

- Documented shell blocks executed with an SHA-256-verified copy of the immutable
  qualified CLI in a disposable local directory. Existing V10 scoped Pocket and
  bundled-CPS Profile succeed. A V10 template derived explicitly from the real V10
  fixture imports a small physical-unit SVG successfully. V9 refusal returns no
  program/stdout; no tracked fixture is renamed, regenerated or modified.
- **516 local targets, including 21 anchors**, all ten skill frontmatters and their local
  referenced paths pass the recorded checks. Existing README parity script passes
  with the same twelve-heading skeleton and reciprocal language links.
- Active-version/obsolete-term review distinguishes explicit historical/removal
  references from active functionality. Three links into the removed FS crate are
  replaced by clearly unavailable historical paths, and an obsolete release-plan
  anchor points to current release lines. Source/guidance headers no longer claim
  active V9, rectangle-only finishing or unfinished ALPHA-4/TOOL-META integration.
- Frontend source supports EN/DE only. README content was reconciled in both
  languages; no runtime catalogs, generated keys or UI strings changed.
- `git diff --check` passes. The real index remains empty and no commits are created.
  The 20-group CNC gate is not rerun for documentation-only work.

## Software and evidence preservation

Before edits, all **1,330** files matched accepted inventory fingerprint
`498fe344f19035d8b6e9fc1d874790373c212aa5073f693774a4fa9ac289c6d0`.
That inventory includes Markdown instructions: **14 documentary exceptions** now
change, so claiming the entire old fingerprint remains identical would be incorrect.
All **1,271 non-Markdown inventoried files** remain byte-identical, and Git changes
outside documentary Markdown/new evidence receipts are absent. This includes
production source, dependencies/configuration, generated contracts and fixtures.
All **3,503 tracked non-Markdown files** also retain their Git blob equivalence;
no executable behavior or schema changed. The preservation receipt records exact
per-file documentary exceptions and non-Markdown fingerprint.

The five prior worktree HEAD/status hashes are unchanged, including dirty canonical
and audit states. Original safety/security inventories, raw evidence and qualified
binaries remain at their original locations. New raw logs, generated CLI responses
and the copied executable remain ignored/local; sanitized receipts contain only
relative paths, hashes, counts and verification results.

## Remaining documentation gaps and approved commit sequence

- Original Argon audit unavailable; reinspect it if supplied, without assuming it
  contains only the summarized findings.
- CLI production diagnostics still say “v9” in two old parse/usage messages although
  validation requires V10. Documented explicitly; correcting runtime wording would
  be a separately authorized code change, not smuggled into this task.
- A host catalogue may load a separate, older `.agents/skills` copy outside this
  worktree. It was not overwritten. AGENTS directs readers to versioned guidance;
  any host-copy synchronization needs its own scope.
- Historical reports deliberately retain source-bound V9/FS/old pending wording;
  their links/artifacts are not globally repaired or invented. Native/remote/physical
  qualification gaps are unchanged.

The maintainer accepted these three local documentation groups:

1. `docs(contracts): reconcile current V10 architecture and acceptance boundaries`
   — technical contracts, status/workstream/release responsibilities, chronology,
   readiness routing and ADR navigation.
2. `docs(onboarding): align public guides and verified V10 examples`
   — README EN/DE, Quickstart, build/contribution/CPS guidance, indexes and labelled
   historical source-pack references.
3. `docs(agents): reconcile versioned skills and document source preservation`
   — AGENTS/CLAUDE, skill scope/frontmatter/reference notes, this report and sanitized
   verification receipts. Exact inventory is linked above.

## Local consolidation receipt

Accepted starting source is unchanged. The reviewed inventory contains 39 documentary
paths in groups of 10 / 12 / 17. Exact staging is checked against the preserved path
inventory before each commit; generated APIs, dependencies, fixtures, binaries,
private data, caches and raw logs are excluded.

- Contracts/architecture/status/roadmap: `5ddc60d57d8ab5e911847aa49e334728d2d0d454`.
- Public guides/navigation: `0e96c30346edf80cdfe1fb2571aef2141928e495` (depends on the first commit).
- Agent guidance/evidence/report: the commit containing this receipt, depending on
  both preceding commits. Its exact SHA and all three commit inventories are recorded
  in the final ignored local receipt and human handoff, avoiding a self-referential SHA.

The technical roadmap lacked PERF-1, so one concise **Planned, not started** row was
added for measured finishing-proof, DXF-chaining and frontend/WASM optimization
against recorded baselines. It preserves proof/numeric/security guards and requires
separate implementation approval. No PERF-1 implementation was begun.

Focused Markdown/anchor/skill checks, README parity and diff checks pass after that
roadmap adjustment. All 1,271 qualified non-Markdown file hashes and 3,503 existing
tracked non-Markdown Git blobs remain unchanged. Existing qualification receipts,
historical reports/evidence and five prior worktrees/refs are preserved. No full CNC
gate rerun was needed. Current acceptance was updated in place, without new banners.

No canonical merge, push, publication, installer, CAM/security change, FabSpeed
integration or physical operation was performed. Installed-native file workflows,
authenticated remote HTTP and physical machining remain unqualified.
**HARD STOP after the three local documentation commits.**
