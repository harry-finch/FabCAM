# Roadmap consolidation and FabSpeed integration planning

Documentation-only inspection **2026-10-24**, tracker `ivac-kwql5`.
[Active release plan](RELEASE_ROADMAP.md) ·
[STATUS](../shopbot-modernization/STATUS.md) ·
[Proposed ADR-0031](../shopbot-modernization/adr/ADR-0031-optional-one-way-fabspeed-catalogue.md).
No software qualification, implementation, release or acceptance is performed here.

## Source, ownership and preservation

Primary checkout `/Users/mugen/Documents/01_Projets/FabCAM/ivacam` was on
`FabCAM-v1.1` at **`6148b1059ab6020f751069d5c3a7d64b8047fb6b`**. It had ten
modified tracked files: STATUS/RELEASE_ROADMAP, two Rust pipeline files and six
frontend files, plus extensive untracked historical reports/evidence. Their content
was fingerprinted before edits; no primary tracked file, index or ref is modified by this
task. Documentation changes live in sibling **`FabCAM-roadmap-fabspeed`**, branch
**`codex/roadmap-fabspeed`**, from that exact committed source.

The managed-worktree API could not resolve the parent project directory as a Git
repository; the isolated worktree was created with repository-local Git instead.
No branch switch, reset, stash, rebase, push, merge or cleanup of another checkout.

QUAL-1A.4 owner is the active chat **“Astra élevé”**
(`01a159fe-836e-7fe1-b8ab-d5cad0c5aae3`), tracker **`ivac-awe23`**, isolated sibling
`FabCAM-qual-1a4` / `codex/qual-1a4-production`. At inspection it contained
`c74f5e9` (portable arithmetic/JSON rounding) and `61c00ba` (P1 guidance/saved
context), above `6148b10`. No final source-bound report or primary integration was
verified. Thus **IN PROGRESS**, irrespective of intermediate passing comparisons
or partial gate commentary. This task neither writes there nor messages/interrupts
its owner. It must not overwrite a later owner-authored STATUS/ROADMAP receipt.

Other worktrees were preserved: `codex/sec1-roadmap` at `7e1c24b`,
`codex/fs-reset-2-repairs` at `d6b08fe`, `codex/qual-1a-deterministic-vcarve` at
`7f55b2e`, `codex/qual-1a2-numeric-input` and `codex/qual-1a3-motion-presets` at
`aa846fc`, and `codex/v1-freeze-readiness` at `85a93c4`.
Frozen `main`/`origin/main` remain
**`ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`**. No remote state refresh or release
acceptance is inferred from inspecting these local refs.

The primary uncommitted documentation additions correct MAT-1 to implemented,
record manual inactive-recipe authorization under ADR-0029 and preserve its UX
review boundary. Their intent is retained in the ledger below; full inspected
working copies, including these additions, are preserved in new historical
[roadmap](history/RELEASE_ROADMAP_BEFORE_FABSPEED_CONSOLIDATION.md) and
[STATUS](../shopbot-modernization/history/STATUS_BEFORE_FABSPEED_CONSOLIDATION.md)
snapshots. Only relative links were shifted for the archive location. Original
user documents, historical reports, accepted ADR text and evidence were not edited.

## Milestone disposition

| Milestone / previous claim | Classification and active disposition |
|---|---|
| M0–M4 / frozen V1 RC1 | COMPLETED bounded foundation; HISTORICAL frozen source/evidence. Software/build success is not release/physical acceptance |
| FS0–FS3.9 | COMPLETED historical engineering; recommendation/source/ranking/adaptation ownership subsequently SUPERSEDED by ADR-0030/FS-RESET. Removed intentionally, not incomplete or scheduled to restart |
| PRESET-1 and PRESET-3 | COMPLETED/accepted historical library and selector work; old source-pack contracts SUPERSEDED. Current neutral Presets UX retained by FS-RESET |
| PRESET-2 and PRESET-4 / ADR-0028/29 | Delivered historical copy/recipe/trial engineering with recorded acceptance boundaries, then SUPERSEDED lifecycle. User's manual recipe authorization is preserved, not revived as a current feature requirement or falsely accepted UX |
| MAT-1 | COMPLETED presentation/search implementation; retained neutral family/variant/alias discovery. Prior human UX review boundary preserved; no renewed resolver/identity/schema work scheduled |
| FS-RESET-1 | COMPLETED audit/architecture; ADR-0030 Accepted. Old pending-architecture preambles are historical |
| FS-RESET-2 | COMPLETED implementation/repairs; accepted integrated source `d6b08fe` before CLI/removal. Pending installed/physical evidence remains distinct |
| CLI Option A / project v9 | COMPLETED explicit `generate --request` and bounded reviewed templates (`030e214`), current explicit role contracts; versions 1–8 refused, original files retained. No old-format migration |
| FS-RESET-3 | COMPLETED removal at `64d7584`; 16/20 historical mandatory gates, not unconditional acceptance. “Removal pending” retired from active plan |
| QUAL-1 | COMPLETED investigation/bounded polling correction at `558efc4`, 17/20 mandatory gates; final acceptance BLOCKED by evidence gaps. Not cooperative preflight cancellation |
| QUAL-1A / QUAL-1A.2 / QUAL-1A.3 | COMPLETED isolated numerical/motion/preset studies, HISTORICAL source-bound findings; not final production integration. Preserve earlier NO-GO/conditional GO and all limits |
| QUAL-1A.4 | IN PROGRESS, production integration/final qualification owned separately; no acceptance inferred |
| QUAL-1B / QUAL-1C / QUAL-2 / RC V1.1 | PLANNED ordered stabilization and release gates; physical ShopBot NOT QUALIFIED |
| Old scaling assistant / future integrated source ranking | SUPERSEDED FabCAM ownership. FabSpeed owns calculations; no local engine or recipe-promotion restart |
| INT-0 / INT-1 / INT-2 / INT-3 | PLANNED optional V1.2; product direction approved, technical contract OPEN. No sync implemented or V1.1 blocker |
| Later CAM, ProtoMAX, reporting, explicit registration assistant, STEP/MCP/smart workshop | DEFERRED from mandatory V1.1; retain existing contracts and independent approval/qualification rules |

Contradictions retired from active docs: MAT-1 simultaneously unstarted/implemented;
FS-RESET-3 simultaneously removed/pending; PRESET-1 repeatedly scheduled after
acceptance; old component-size failure presented as current; completed polling
confused with preflight cancellation; obsolete source-pack/Trial/scaling architecture
presented as future FabCAM behavior. Historical checkpoint prose remains untouched.

## Source-bound evidence

Reviewed committed references: [FS-RESET architecture](FS_RESET_ARCHITECTURE.md),
[FS-RESET-3 results](FS_RESET_3_RESULTS.md),
[CLI modernization](FS_RESET_3_CLI_MODERNIZATION.md),
[QUAL-1](QUAL_1_SOFTWARE_QUALIFICATION.md),
[ADR-0030](../shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md),
[V1 readiness](V1_READINESS.md), [V1 freeze](V1_FREEZE_READINESS_2026_10_20.md),
PRESET/MAT reports, navigation/index, repository instructions and current Git/tracker.

Three additional reports are **untracked preserved evidence in the primary
checkout**, not included in this commit and not portable relative links in the
active plan. Their exact paths under primary `docs/releases/` were inspected:

| Local report | What it actually establishes |
|---|---|
| `QUAL_1A_DETERMINISTIC_VCARVE.md` | Isolated portable-math comparison; dense JSON-input failure retained, NO-GO for complete integration; prototype fingerprints, not accepted production |
| `QUAL_1A_2_DETERMINISTIC_NUMERIC_INPUT.md` | Combined portable math + correctly rounded parser; successful program parity/refusal matrix, residual Profile path non-bit-equality and dense order/preset-signature implications; integration still gated |
| `QUAL_1A_3_MOTION_PRESET_ACCEPTANCE.md` | Conditional ideal-tool software motion assessment, different ordered history acknowledged; strict P1 recommended with context review. No force/contact/holder/fixture/physical claim; explicit subsequent integration decision |

The studies bind source/prototype file hashes, patches, dependencies and binaries;
diagnostic commit IDs alone are not the tested production candidate. QUAL-1A.4
must supply the actual integrated source receipt. Historical cancellation failure,
two watcher tests, advisory/network/headless limitations, actual file transfers and
installed platform gates remain visible. Two-sided publication is not pair-atomic;
CLI streaming is not full-program application-simulation qualification and stdout
is not transactional. Automatic dowel injection remains unsupported in v9.

RC1 identity/build results remain source-bound to `ef807cf`, with local report
`V1_RC1_CANDIDATES_RESULTS_2026_10_20.md` also untracked. No physical or publication
acceptance is inferred. Historical snapshots/readiness may reference other local
reports; their evidence portability is explicitly distinct from this docs commit.
No historical report or compact/raw evidence is silently added to Git for link checks.

## Approved future direction and OPEN decisions

V1.1 path: **QUAL-1A.4 → QUAL-1B → QUAL-1C → QUAL-2 → RC V1.1**. Preparation may
overlap; final qualification/acceptance must apply to consistent combined source.
V1.2 path: **INT-0 → INT-1 → INT-2 → INT-3**, from an appropriately qualified V1.1
baseline on a separate future branch. INT-0 documentation may run in parallel by
separate authorization; INT-1/2/3 do not block V1.1.

Approved principles are one-way FabSpeed → FabCAM business data, independent local
resources and stable local/shared identities, upstream-owned common fields plus
local CAM extensions, automatic sync with status/offline behavior, runtime autonomy,
explicit reviewed operation copies and truthful upstream provenance. Relevant safety
limit changes invalidate/requalify affected generated programs. No upstream writes,
CAM uploads, experiment submission, invented validation or automatic application.

INT-0 must inspect real data/API/storage, choose transport including FabSpeed-closed
behavior, define schemas/units/IDs/linking, field/limit ownership, conflicts,
revocation/deletion, transactions/cache, restart/offline and trust/rights. INT-3
remains an OPEN API/UX decision if identifier-based proposal reads are unavailable;
no reverse operation submission is authorized. ADR-0031 is Proposed, not an accepted
implementation contract or evidence of a working connection.

## Documentation checks and commit disposition

The change updates canonical STATUS/RELEASE_ROADMAP and track navigation, proposes
ADR-0031, indexes ADR-0030/31 and supersession scope, updates documentation entry
routing and stale V1.1 guidance in AGENTS/V1_READINESS, creates the two preservation
snapshots and this report. Frozen V1 register, accepted ADRs, qualification reports,
production code/tests, dependency/version/pipeline/release metadata remain unchanged.

Checks: scoped diff/whitespace review, Markdown fences/tables and relative links/
anchors, referenced-report existence, milestone/status/ownership consistency and
primary dirty-file preservation. Validation checked **271 local links/anchors**, balanced Markdown fences and
reviewed new tables; no unresolved target in the shared workspace. Exactly two
historical snapshot links require primary-only untracked reports:
`PRESET_BROWSER_UX_2026_10_21.md` and `V1_RC1_CANDIDATES_RESULTS_2026_10_20.md`.
Their absence from a standalone checkout is disclosed rather than hidden by staging
other people's evidence. Link validation distinguishes committed targets
from preserved primary-only historical evidence; no new active link relies on the
untracked QUAL-1A reports. No existing documentation-validator script was found.
No expensive Rust/frontend build or qualification suite runs for this docs change.
Exact local commit is reported in the delivery message; self-referential commit IDs
are intentionally not embedded here.

**Integration boundary:** this is a reviewable local documentation branch only.
Primary STATUS/ROADMAP remain dirty and QUAL-1A.4 may publish a later receipt; any
future application requires semantic reconciliation with both. Do not blindly
replace owner/user files or fast-forward this branch into production. If that
reconciliation cannot preserve both, keep this isolated proposal for human review.
No push, main merge, rebase, release, installer, hardware or next milestone.
**HARD STOP after documentation validation and the scoped local commit.**
