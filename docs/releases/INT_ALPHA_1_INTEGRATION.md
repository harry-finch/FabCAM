# INT-ALPHA-1 — Controlled integration

**Accepted bounded software-qualified integration candidate; locally consolidated (2026-11-01).**
Branch `integration/alpha-v10-tool-meta`, worktree `FabCAM-alpha-integration`.
Human approval authorizes only the three local consolidation commits.
No merge into V1.1/main, push, installer, release or physical operation.

## Actual source and scope

Accepted ALPHA-4 HEAD: `78be45928f8c63ccb284787f90d9d5c065765717`.
Implementation `68744a5`, complete software `7fddc77`, documentation `78be459`.
All 1315 source files matched accepted fingerprint
`6d544fa193e5eb07f42b632d6cf985517d2891e77cfdc3c7f2f46a99a09c245a`
before changes. Tracked source was clean; local untracked qualification evidence
was preserved and was not copied into the new worktree.

| Milestone | Actual inherited HEAD | Integration action |
|---|---|---|
| ALPHA-1 | `4d0bf277c4023acc980f39accf8c8a40b77dbd13` | Already ancestor; no merge |
| ALPHA-2 | `96181814c6f97ac19390dade7a0d3110b2f4d681` | Already ancestor; no merge |
| ALPHA-3 | `1a70e503b8661aeb59b8d3bff56e5fde4139836c` | Already ancestor; no merge |
| ALPHA-3.1 | `81e8d47d3ca6860303d534393bad37056ea0cfb2` | Already ancestor; no merge |
| ALPHA-3.2 | `38fc911fab5ac7058329e1edbad7e4489134ac91` | Already ancestor; no merge |
| ALPHA-4 | `78be45928f8c63ccb284787f90d9d5c065765717` | Authoritative integration base |

TOOL-META remains at `efe98dc2e7e46f4d12783dd8dbc9c0b09d2f773d` in its original
clean worktree. Model `ca61767` and UI `3145f4d` have recorded 20/20 software gates,
with original human/native/physical acceptance still open. Common ancestor is
`34f41692ef1d90eddf1b3ff3b564d2b8600879d7`, a V9 source. This intervention ports
its compatible metadata feature; it does not claim acceptance of the original
branch's unrelated language or future-catalogue changes.
[Preflight](evidence/int-alpha1/preflight.json) and
[exact port inventory](evidence/int-alpha1/port-inventory.json).

## TOOL-META semantic integration

Optional `substrate` and `coating` strings are carried by the current Rust tool
macro, generated schema/API, frontend types, build-project transport, V10 project
validation, toolset drafts, library editing, review and stocked copies. Absence/null
is unspecified; explicit `uncoated` remains distinct. Custom strings and untouched
empty strings round-trip without case folding or equivalence inference. Non-string
values reject before project/toolset state mutation. Missing values serialize by
omission, keeping old tools usable without new defaults.

Neither field changes identity, cutter family, geometry, authored rates, stage
assignments, physical-test status or machine output. The existing strict preset
signature includes supplied technical tool properties, so changing specifications
invalidates the exact tested context instead of fabricating compatibility. It does
not rewrite source history. Generation identity changes through existing tool-list
freshness; Undo/Redo restores the corresponding tool copies and identity.

Semantic ports and narrow resolutions:

- `ToolDetailBasics.svelte`: add metadata fields/help while retaining ALPHA's current
  geometry validation and dimensional UI; do not replace it with the V9 component.
- `file_ops.ts`: insert metadata validation before returning a toolset draft while
  retaining current export/notification safeguards.
- Map the old `project-v9.ts` validation addition onto `project-v10.ts`; no old reader,
  operation model, migration or V9 schema is restored.
- Regenerate the schema and API from the current Rust source. Project/cutting V10,
  `main_finishing_v1`, `pocket_finishing_v2`, collection V2 and historical V1 handling
  retain their existing contracts. Descriptive optional fields add no machining
  meaning that an older reader could silently reinterpret. Metadata preservation
  claims apply to this candidate; older readers may discard unknown descriptions.
- Adapt the metadata test's Profile/Pocket inputs to current V10 fixtures. Other
  families keep their established role semantics with the V10 transport version.
- Tool-library impact review now includes a cutter used only by Finishing, including
  the actual stage assignment. A focused regression covers the integration seam.
- Add current EN/DE labels/help; do not import TOOL-META's partial French runtime,
  AppNavigation changes, FabSpeed mapping/ADR changes or catalogue synchronization.

No CAM algorithm, proof/resource budget, postprocessor command, security policy,
project version or qualified golden was changed. Edits in the other existing Rust modules are `None` initializers in exhaustive
test scaffolding; the new metadata regression is separate. No new
reader discriminator or endpoint is introduced. No unresolved contract conflict
required model escalation.

## Targeted regressions and fixed CNC comparison

Final focused frontend run: **81 PASS, one existing i18n skip**, 12 files. Lint and
Svelte/TypeScript: **0 errors / 0 warnings**. Native metadata pipeline integration:
**1 PASS** across Profile, scoped Pocket, Drill and V-Carve; unit filtering also
passes metadata serialization and exact-preset context checks. The final canonical
suite below supersedes preliminary counts.

Coverage includes old absent/null/empty/custom metadata, invalid imports before
mutation, V10 snapshot/restore, tool identity and Main/Finishing references, source
collection immutability, strict applicability, reviewed copying without metadata
mutation, advisory extension persistence, modified disclosure, atomic history,
stock/import placement, ALPHA-2 real WASM protocol and stale generation/export.
Existing V9 refusal and both V10 stage-contract tests remain active.

[109 fixed corpus comparisons](evidence/int-alpha1/cnc-comparison.json):
79 hash-bound ALPHA-3.2 cases, 23 retained V10 corpus cases and seven ALPHA-4 cases.
The immutable accepted ALPHA-4 CLI snapshot is checked against its qualified binary
hash. For each unchanged request the candidate must match the **entire** native
response exactly, including program, decoded toolpath, tools/stages, line mapping,
provenance, warnings, statistics and timing. For the metadata variant only the two
explicitly asserted descriptive tool-snapshot properties are removed before the
same full equality check. No golden update or motion tolerance is used.

**96 successes and 13 preserved refusals**, including unsupported proof budgets,
same-tool ShopBot spindle-speed refusal and qualified distinct-tool behavior.
Both streamed faces of the retained two-sided case are byte-identical too.
The canonical Cargo commands later rebuild the CLI with a different feature-set
artifact identity. The fixed corpus is replayed on that final binary without source
changes or another release-gate run; its result supersedes the pre-canonical binary
receipt, retained separately as `cnc-before-canonical.json`. The replay uses a fresh
disposable output directory; the first replay correctly refused to overwrite old
two-sided files. Only that output-directory literal is redirected in the local
harness; the qualified repository script and all assertions remain unchanged.
Four malformed metadata-variant JSON requests move the parser's physical column
because their input length changed. Only the terminal parser line/column suffix
is excluded for these edited-input refusal comparisons; the refusal reason must
remain exact. Unchanged-input refusal strings remain byte-identical. This is an
explained diagnostic offset, not a motion/output difference or relaxed guard.

The corpus is replayed by `scripts/int-alpha1-cnc-check.py` using existing synthetic
qualification evidence and verifies its request hashes against committed receipts.
Detailed requests/responses remain in local `dist-test/int-alpha1/cnc`.
[Four exact full native/HTTP responses](evidence/int-alpha1/http-comparison.json)
add V10 metadata checks for Walls/Floor/Both and reviewed-value output.
[Eight direct ALPHA-4/candidate WASM comparisons](evidence/int-alpha1/wasm-comparison.json)
are also exact over the full response, with **zero coordinate tolerance**. They
include Profile Main, Pocket Main, each scope, curves/islands and distinct cutters;
the metadata variant changes only the asserted descriptive snapshot properties. Real WASM
strict context inspection/load tests and the production walkthrough use the freshly
built current package, not a copied older schema/runtime.

## Actual production frontend/WASM walkthrough

Disposable loopback origin `127.0.0.1:19005`, production bundle with matching WASM.
No normal application profile or private drawing was accessed.
[Structured browser receipt](evidence/int-alpha1/browser-acceptance.json).

Created a new V10 project, chose Manual stock before importing the committed
`shopbot-m1/concave_L_shape.svg`, and verified the original (10,10)–(110,110) mm
coordinates, fixed stock and WCS (0,0). Explicitly resized stock to 150×150×5 mm.
The library starts with unknown specifications; entered `carbide` and `uncoated`,
reviewed/applied the normal draft, and set the synthetic cutter's known flute length
12 mm. Profile Main generated 79 moves with Finishing OFF.

Added Pocket with explicit Finishing and explicitly copied Main cutting values.
The initial 100 mm L-shape correctly exceeded the 5000-segment Finishing proof
budget: actionable refusal, stale export disabled, no qualified output. Explicit
file scale 0.3 about its center gives bbox (45,45)–(75,75) mm. Stock/WCS stay fixed;
no proof limit was changed to accept the larger case.

| Current browser scope | Total moves, including Profile | Pocket Main | Pocket Finishing |
|---|---:|---:|---:|
| Walls only | 315 | 220 | 16 |
| Floor only | 698 | 310 | 309 |
| Walls and floor | 639 | 250 | 310 |

Each scope change uses its explicit review/apply step. Floor reserve 0.1 mm and
Both reserves 0.1/0.1 mm are authored explicitly. Reports show actual stage/tool
attribution and emitted line ranges, not inferred finish intent. Simulation completes
and ordinary export becomes eligible after successful generation. Unknown machine
limits and cell-budget coarsening remain visible warnings.

Saved and reopened the actual project: V10, one drawing, two operations, stock,
transform, tool ID, metadata and scoped finishing survive. Imported the synthetic
ALPHA-4 Profile preset into an empty disposable library. Exact tested application
remains blocked for the nonmatching context; separate reuse copies **feed only**
800→600 into Pocket Main, leaving Finishing at 800 and keeping tool metadata.
Undo returns 800 and removes advisory origin; Redo restores 600 and origin. Manual
edit to 601 displays modified origin and disables stale export. Undo/Redo of that
edit behaves correctly. Regeneration completes and the report explicitly says
“No target-context test attestation”; the copied original feed remains recorded.
The actual persistence regression separately round-trips the advisory root extension
alongside new tool metadata and verifies no private source notes are embedded.

Standard 1280×720 and narrow desktop 1120×800 are exercised. A 1000 px transition
uses the existing compact layout, not a new desktop navigation. Tab moves from
Substrate to Coating; EN/DE descriptions remain visible and editable. The temporary
viewport override was reset. [Narrow metadata](evidence/int-alpha1/browser-narrow-metadata.png),
[German](evidence/int-alpha1/browser-narrow-de.png),
[generated view](evidence/int-alpha1/browser-final.png),
[stale export](evidence/int-alpha1/browser-stale.png),
[budget refusal](evidence/int-alpha1/browser-budget-refusal.png).
Detailed synthetic report text remains local evidence in ignored
`dist-test/int-alpha1/local-consolidation/`; original unsanitized receipts are
preserved there under `original-evidence/`. Raw logs and synthetic saved projects
remain local and are excluded from commits. Screenshots above contain only the
disposable synthetic scenario. The budget screenshot shows generation failure
and disabled export; the exact refusal text was observed before its toast closed.

**Installed-native GUI acceptance NOT RUN.** Relevant Rust/Tauri command boundaries
are included in the canonical workspace tests; the installed application/normal
profile was not launched, no native installer was built or installed. No controller
or physical acceptance is inferred. `socle.dxf` was not accessed, repaired or declared
machining-ready; its known real crossings remain a separate geometry issue.

## Final canonical qualification and source freeze

**20/20 mandatory gates PASS**, exit 0, in one canonical run on the final source.
Rust/Tauri: **1662 PASS**, one existing ignored worker. Frontend: **1565 PASS**,
one existing i18n skip. Strict fmt/Clippy, schema/codegen drift, readme parity,
lint/typecheck, production build, WASM/CPS size, dependency audit, headless smoke,
transactional CLI/export, SEC1 and candidate identity gates all pass.
[Qualification, binary identities and local log hashes](evidence/int-alpha1/qualification.json).
The canonical `scripts/pre-release.sh` run starts only after stable targeted tests
and the real walkthrough. It uses a private temporary Git
index for reviewed generated API/i18n files, with drift guards active; the real
index stays unstaged. No gate is waived. Earlier targeted failures were limited to
a CPS fixture in a non-CPS metadata unit test (select the explicit LinuxCNC fixture
path), a sandbox sccache launch (run with empty RUSTC_WRAPPER), and the edited-input
parser column described above. None was bypassed by weakening a CAM guard.

Final software inventory: **1318 files**,
`974de18341dd476e16699b0dcf9bf76bd5fdc6eaf5b4b70ba11aa5a01be5377d`.
[Exact inventory](evidence/int-alpha1/software-files.json). Same sorted path + NUL +
file SHA256 + LF convention as accepted ALPHA; documentation/evidence are excluded.
Source bytes are rechecked after the gate. Detailed logs stay local with paths/hashes
in the final qualification receipt.

## Preservation, accepted commits and decision

[Preservation receipt](evidence/int-alpha1/preservation.json): all **16 pre-existing
worktrees** retain their HEAD and Git-status hash, including primary V1.1, frozen
main/V1, ALPHA branches and TOOL-META. This is a HEAD/status reinspection, not a new
byte inventory of every untracked private file. No operation here wrote to those
working trees. Accepted ALPHA-4 and its binary source fingerprint are preserved.

Authorized dependent commits (each builds on the preceding commit):

1. `feat(tools): port descriptive cutter specifications onto V10` — optional model,
   regenerated schema/API, compatible persistence/transport, editor/review, EN/DE.
2. `test(integration): verify ALPHA and TOOL-META seams and exact CNC output` —
   cross-feature regressions, Finishing-only impact coverage and corpus replay.
3. `docs(integration): record INT-ALPHA-1 source-bound qualification` — this report,
   minimal canonical status/roadmap pointers and sanitized receipts.

**Human-accepted bounded integration software checkpoint**, with TOOL-META included.
This is not installed-platform, physical or release acceptance. Human-review gate
`ivac-ol10d` is fulfilled by the explicit 2026-11-01 approval. A subsequent controlled
macOS Personal Alpha acceptance remains a separate authorized step. No new CAM feature,
FabSpeed synchronization, primary merge, push, publication, installer or machining.
**FINAL HARD STOP.**

## Consolidation receipt — 2026-11-01

- Implementation: `f1be4919cc61e0970fd8b07eb35199788dcdc19e` (25 exact reviewed paths).
- Complete software: `51f0ccce6d6ced3f061c8dfc4d93c13d3a93c054` (10 further reviewed regression/harness paths).
- Documentation: the containing commit of this receipt; its full SHA is recorded
  in the final handoff, avoiding a self-referential commit hash.
- [Exact staging and dependency receipt](evidence/int-alpha1/consolidation.json).
- All 1,318 committed logical source files match the approved fingerprint exactly,
  including target content and link text for 11 inherited symlinks.
- No executable-source changes during consolidation; no release-gate rerun.
- Qualification receipts describe their original precommit checkpoint; this
  consolidation receipt supersedes their pending-approval/zero-commit state.
- The 16 other worktrees retain their original HEAD/status hashes. This does not
  claim a new byte inventory of every private untracked file. Local-only evidence
  remains preserved; the integration Git worktree is clean after consolidation.

**HARD STOP after the three local commits.** No primary V1.1 integration, push,
public release, installer, FabSpeed synchronization or physical machining.
