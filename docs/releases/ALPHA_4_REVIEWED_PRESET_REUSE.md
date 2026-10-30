# ALPHA-4 — Reviewed Preset Reuse

**Accepted bounded software checkpoint — local commits authorized (2026-10-30).**
This acceptance supersedes earlier pending-human-review statements in the original
qualification narrative below. [Consolidation receipt](evidence/alpha4/consolidation.json)
records the completed local commit checkpoint; no wider release or physical acceptance.
Implementation: `68744a56dc572d92aab5dc955c71c4ae5c5a2160`; complete software: `7fddc775f82bac1decf2c3ef9982ff07a87a48a9`.
The third, documentation-only SHA is the commit containing this receipt and is
recorded explicitly in the final handoff. All 1315 committed software files match
the accepted fingerprint (including resolved committed symlinks); no gate was rerun.

Original pre-acceptance checkpoint on 2026-10-30: human acceptance and commits
were pending then. The accepted local consolidation receipt above supersedes that
checkpoint; merge, installer, release and physical acceptance remain excluded.

## Audit and implementation plan — 2026-10-30

Accepted baseline `38fc911fab5ac7058329e1edbad7e4489134ac91`; all 1,308 software
files match ALPHA-3.2 fingerprint `62265b79d933d4681d70592e273e33cca9bbbafcb162909a6a3994e9b0650c42`.
Isolated `feature/alpha-4-preset-reuse` in `FabCAM-alpha-4`. Other worktrees and
local data are preserved; baseline receipt in `evidence/alpha4/baseline-summary.json`.

Extend existing picker with a separate “Reuse as starting values” review. V2 typed
Profile/Pocket stage sources only; explicit source/destination stage, numeric
checkboxes and current/proposed comparison. No automatic Finishing enable, cutter
assignment, allowances, geometry, strategy, total depth, entries or attestation.
RPM/feed/plunge require positive representable integers; DOC preserves finite
positive f64 values where stage semantics match. Stepover is deferred.

Domain service validates the existing typed collection, binds review to source
content/storage revision and target/document revision, and uses the existing
operation/history command for one atomic copy. Source is re-read before confirmation.
One versioned root advisory extension preserves sanitized source context, exact
selected fields and values; original trial notes/full records are excluded.
An existing loaded tested-preset snapshot is cleared atomically. The numerical
values remain authored CAM inputs; provenance never authorizes machining.

A requested Astra Medium read-only review found this compatible with existing V10
portable root extensions. Older V10 readers preserve them but display ordinary
manual values. Rust/native/HTTP/WASM generation deliberately receives only existing
cutting fields. No new wire endpoint, schema, nested discriminator or version.
Recognized records are validated; unknown metadata versions remain opaque and
unmodified. Modification status means current divergence, not a permanent edit log.

Tests precede implementation: selective/cross-stage copies, strict behavior,
revisions, history, persistence, source changes/deletion, provenance and unchanged
scope. Extend existing report and stage editor with advisory disclosure. Then
exercise actual WASM/transport/CNC boundaries and one production-browser synthetic
workflow. Run canonical pre-release once after targeted tests pass. Deliver one
source-bound report and proposed commits; no automatic commit, push, installer,
publication or physical operation. Stop for human review.

## Implemented contract

The existing picker now offers **Reuse as starting values** independently of
**Apply matching tested preset**. Only typed V2 Profile/Pocket stages can supply
this action. Historical V1 records remain readable through the existing UI and
are not converted into stage data. Supported destinations are Profile/Pocket
Main and already-enabled Finishing. The destination starts unselected.

The user selects RPM, cutting feed, plunge feed and/or depth per pass, previews
current/proposed values, compares contexts, and confirms once. Integer fields
must be positive uint32-representable integers; DOC must be positive finite f64.
No rounding, unit conversion, inferred defaults or calculation occurs. Both
supported stage contracts express DOC as millimetres per axial pass. Stepover is
excluded because strategy-specific representations do not establish equivalence.

Only checked fields on the chosen stage change. Cutter assignment, Main/Finishing
scope, enable state, allowances, retained stock, strategy, geometry, entry,
absolute/final depth, stock, work origin, fixtures and post configuration remain
unchanged. Reuse never generates output automatically. Existing target validators,
generation freshness, simulation and export qualification remain authoritative.
Unmatched contexts are allowed for this advisory copy, not for strict application.

### Identity, revision and history

Preview captures document epoch, target cutting/context key and complete current
source collection revision/content hash. Confirmation re-reads the existing local
collection service and rejects changed, missing or replaced sources/targets. One
history command changes selected values and provenance atomically. Undo restores
both, including any previous loaded-preset snapshot. Redo uses reviewed copied
values independently of subsequent source deletion. Cancellation does not mutate.

The target's existing `stagePreset` is cleared atomically: retaining that snapshot
would incorrectly describe edited stages as a loaded tested preset. This does not
change strict application rules or mark a new source as tested.

### Portable advisory extension and review boundary

`fabcamPresetValueReuse`, version 1, is a frontend-owned portable project-root
extension using the existing V10 unknown-root-field preservation path. Its source
record contains collection/preset names and IDs, storage revision, SHA256, source
operation/stage/context, target operation anchor/stage/context, and copied fields
and values. No original trial notes, complete preset record, tested Boolean or
physical validity certificate is embedded. Source content is hashed; only the
review object temporarily retains its complete JSON in memory.

A requested Astra Medium read-only review returned **bounded GO** for this route:
supported older V10 frontend readers preserve the extension opaquely and retain
ordinary authored values; they do not interpret its advisory meaning. The Rust
project model remains strict and receives only existing CAM fields through
`buildGenerationProject`. No wire contract, project version, nested discriminator,
permission or CNC schema changed. Rust-generated reports do not claim to preserve
this frontend-only provenance; the project report explicitly derives its disclosure
from the frontend document.

Recognized metadata is validated and bounded (256 records, 262144 JSON characters,
limited scalar context). Unknown versions/malformed envelopes remain untouched
and cannot be silently overwritten by reuse. Operation deletion removes recognized
associations with Undo support. Duplicate IDs/kind/anchor mismatches cannot inherit
an association; disabled Finishing has no active disclosure. An applied strict
preset supersedes the advisory explanation. Current value/context divergence
shows “modified”; returning to the copied state shows “matches recorded copy”.
This comparison is not a permanent edit history or physical test record.

## Focused qualification

- 28 targeted tests pass (one existing i18n skip) across reviewed reuse, real snapshot/restore persistence,
  cutting workflow and V10 boundaries. The reuse domain includes independent
  field selections for both target kinds, enabled/disabled stages, cross-stage
  copying, invalid/overflow/fractional fields, unsupported V1 sources, stale source
  and target, atomic history, deletion, duplication, manual changes and opaque data.
- The snapshot/restore test uses actual frontend project-file services, preserves
  high-precision DOC and unknown root extensions, and proves advisory metadata is
  absent from CAM transport and does not itself change the generation key.
- [Runtime receipt](evidence/alpha4/runtime-summary.json): five successful actual frontend
  service → WASM/native CLI/HTTP cases (baseline and four individual fields), with
  exact equality of G-code, toolpath, stage attribution and warnings. Strict matching
  succeeds on the exact source and refuses the cross-context target. No Rust/CAM/
  post source changed from the qualified baseline.
- RPM/feed/plunge copies retain 7294 motion segments; DOC changes the axial schedule
  to 3660 segments. Every successful case retains minimum Z = -0.5 mm. The five
  independent [SAFE-Z checks](evidence/alpha4/safe-z.json) retain all nine lateral
  rapids at or above authored SAFE-Z 5 mm. No source identity, geometry or allowance
  enters the target through the copy.
- Two expected actual CLI/HTTP refusals remain: feed 600 exceeding target machine
  maximum 500, and ShopBot intra-section spindle-speed change. The ordinary machine
  limit refusal is also checked in WASM. The browser WASM build intentionally lacks
  the CPS runtime; ShopBot is therefore checked through its native/HTTP consumers.

## Production-browser acceptance

One synthetic local workflow used the built production bundle at loopback 18985
with WASM, without launching a native application or touching user presets.
Imported the synthetic V2 Profile collection using the actual file picker and
opened the committed ALPHA-3.2 Pocket fixture. A missing target material still
blocks strict tested application while the separate advisory action remains usable.

Verified explicit destination, feed-only copy 811 → 600, unchanged RPM/plunge/DOC
and Finishing values, one Undo/Redo, keyboard-edited feed 601 and modified disclosure,
V10 download/reopen retaining values/provenance, and German disclosure. Disabling
Finishing removes it from the destination list; keyboard cancellation leaves the
operation unchanged. The final fresh bundle also copied all four fields, generated
14270 moves with Main/Finishing attribution, completed whole-program simulation,
and enabled normal export after qualification. The [downloaded project report](evidence/alpha4/browser-project-report.md)
contains source identity/hash, copied values and “No target-context test attestation”.

The source-role inspection found and corrected `rough` vs V2 `main` when reading
source cutter context; a regression assertion now checks source family/diameter.
The disclosure is derived reactively from current values. One older open tab
encountered missing lazy chunks while production assets were rebuilt; reloading
and reopening the saved synthetic file resolved this, and fresh generation/report
passed. No runtime policy was weakened to bypass the obsolete bundle.

`browser-review.png` (retained locally; excluded from commits),
`browser-copied-edited.png` (retained locally; excluded from commits),
`browser-reopened-de.png` (retained locally; excluded from commits),
[final generated view](evidence/alpha4/browser-final.png),
`browser-report.png` (retained locally; excluded from commits),
`browser-saved-project.json` (retained locally; excluded from commits).
Screenshots supplement assertions and report text; they do not establish machine
or controller qualification. No physical material was cut.

## Canonical source-bound qualification

Final software fingerprint (1315 files):
`6d544fa193e5eb07f42b632d6cf985517d2891e77cfdc3c7f2f46a99a09c245a`.
[Exact software inventory](evidence/alpha4/software-files.json). The algorithm is
sorted UTF-8 path + NUL + per-file SHA256 + LF, matching the ALPHA-3.2 inventory
convention. Documentation/evidence is outside the executable-source fingerprint.

Canonical result: **20/20 mandatory groups PASS**, exit 0: 1659 Rust tests
(one existing ignored worker), 1559 frontend tests (one existing i18n skip), lint,
typecheck, generated drift guards, WASM/CPS size, dependency audit, production UI
smoke, CLI transactional export and release/candidate checks.
[Qualification receipt and binary hashes](evidence/alpha4/qualification.json),
`pre-release-qualified.log` (retained locally; excluded from commits),
`targeted-tests.log` (retained locally; excluded from commits). A first obsolete pass was interrupted after
source cutter-context correction was identified; its diagnostic log is retained.
The next full pass found one dead i18n key after the strict action was relabeled;
the obsolete English catalog key was removed, codegen and all 28 focused tests
then passed. A new complete run qualifies the final bytes.
The complete final pass starts after corrected targeted tests and production UI
acceptance. It uses the normal `scripts/pre-release.sh`, all mandatory groups and
a private temporary Git index containing reviewed generated files, exactly like
the accepted ALPHA-3.2 procedure. The actual index remains unstaged. Drift guards
are active and generated bytes must match. No source changes follow this pass.

## Preservation and reviewed commit groups

[Baseline receipt](evidence/alpha4/baseline-summary.json) records accepted HEAD, all 1308
baseline files and the HEAD/status hashes of other worktrees before edits.
The primary checkout, TOOL-META, frozen V1 and local qualification evidence are
outside this implementation. The [final preservation receipt](evidence/alpha4/preservation-summary.json) records
15 other worktrees with unchanged HEAD/status hashes and no staged paths in the
actual index. This is a HEAD/status reinspection, not a new byte inventory of all
untracked external evidence; no operation in this task wrote to those trees.
All 1315 qualified software-file hashes are rechecked unchanged after the gate.

The three authorized groups are listed exactly in [commit-plan.json](evidence/alpha4/commit-plan.json).
Group 1 supplies the domain/history services; group 2 depends on it and completes
all qualified software, including the runtime diagnostic script. That script moves
from the originally proposed third group so the third commit is documentation-only.
No test or production bytes change during consolidation.

The user accepts this as a **bounded software-qualified implementation**, including
selective cross-context copy, advisory origin, atomic history, V10 persistence and
unchanged strict matching. Stepover remains unsupported and source-context physical
attestation never transfers. Native/platform and physical acceptance remain separate.
Raw logs, raw error traces, portable projects and unselected screenshots remain local
and untracked; no existing evidence is deleted. The sanitized committed runtime
summary is derived from the original structured receipt without changing outcomes.

[Local consolidation receipt](evidence/alpha4/consolidation.json) records the
implementation and complete software SHAs and exact index review. The documentation
SHA is the commit containing that receipt and is reported in the final handoff.
No merge, push, publication, installer, TOOL-META/FabSpeed integration or physical
machining is authorized or performed. **HARD STOP after the three local commits.**
