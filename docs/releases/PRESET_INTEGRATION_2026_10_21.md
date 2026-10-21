# Accepted PRESET-1/2/3 — local integration checkpoint

2026-10-21 · branch **FabCAM-v1.1** · task `ivac-0szm9`.
The maintainer explicitly accepted the refined selector and authorized integrating
and committing PRESET-1/2/3 without pushing. This records bounded software/UX
acceptance; it does not accept V1, a release, installed platforms or physical cutting.

## Commits and scope

Starting HEAD: `5ca56321bd7201eeda6dd5b777966c3e82882e8b`, the existing approved
record-selection API commit. It is retained unchanged as an ancestor.

| Commit | Reviewed scope |
|---|---|
| `bed3a92` — `feat(presets): guard native draft copies against stale sources` | Two Tauri files: registered guarded copy command, source/destination locks and compare-and-append tests |
| `b8deaef` — `feat(presets): integrate library, draft copying and parameter selector` | 39 frontend files: PRESET-1 library/navigation/maintenance extraction; PRESET-2 strict inactive Draft copying and IndexedDB/native dispatch; PRESET-3 shared selector/application/freshness/Undo guards; typography, table/pane resizing, translations and regressions |
| Documentation commit containing this receipt — `docs(presets): record accepted integration and qualification` | Current status/index/roadmap, accepted ADR-0028, source-bound reports, selected existing evidence and this integration receipt |

The frontend slices share components, translations and regression dependencies.
Committing them together avoids incomplete intermediate UI/catalog states and
retrospectively fabricating old implementations. The native storage guard is a
separate coherent commit. No commit was amended, reordered or rewritten.

The 41 source files match both the reviewed starting worktree and their committed
blobs, verified by [SHA-256 fingerprints](preset-integration-2026-10-21-evidence/source-fingerprints.json).
This integration adds no new production behavior beyond the accepted changes.
The existing API, source-pack formats, private provenance limitations, machine
output and security policies remain unchanged.

## Validation on the assembled source

Commands run from the repository root unless the frontend directory is specified.
Node 24 and the existing offline Rust/WASM toolchain were used. The final frontend
production build was repeated after the implementation commit to stamp `b8deaef`;
dirty status at build time reflects pending documentation, not uncommitted source.

| Check | Result |
|---|---|
| Frontend `npm test` | **1505 PASS / 1 pre-existing locale skip / 0 FAIL**, 171 files; **10 WASM package guards PASS** — [log](preset-integration-2026-10-21-evidence/frontend-tests.log) |
| Frontend `npm run check` | Svelte **0 errors / 0 warnings**, node TypeScript PASS — [log](preset-integration-2026-10-21-evidence/frontend-check.log) |
| Frontend `npm run lint` | ESLint and full Prettier PASS — [log](preset-integration-2026-10-21-evidence/frontend-lint.log) |
| Frontend offline `npm run build` | PASS; existing chunk-size/dynamic-import/plugin-timing notices retained — [log](preset-integration-2026-10-21-evidence/frontend-build.log) |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-tauri --lib source_packs::tests` | **8 PASS** — [log](preset-integration-2026-10-21-evidence/native-storage-tests.log) |
| `RUSTC_WRAPPER='' cargo clippy --offline -p ivac-tauri --lib -- -D warnings` | PASS — [log](preset-integration-2026-10-21-evidence/native-clippy.log) |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-feeds-speeds` | **119 tests + 1 doctest PASS** — [log](preset-integration-2026-10-21-evidence/feeds-speeds-tests.log) |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --lib feeds_speeds` | **20 PASS** — [log](preset-integration-2026-10-21-evidence/adapter-tests.log) |
| `RUSTC_WRAPPER='' cargo fmt --all -- --check` | PASS — [log](preset-integration-2026-10-21-evidence/rustfmt.log) |
| `RUSTC_WRAPPER='' cargo xtask schema-check` | PASS — [log](preset-integration-2026-10-21-evidence/schema-check.log) |
| Frontend OpenAPI TypeScript generation to a temporary file, byte comparison with committed client | PASS — [log](preset-integration-2026-10-21-evidence/frontend-codegen.log) |
| Staged source/document whitespace and documentation references | PASS for source, Markdown and JSON; **358 local file targets** checked. Raw logs/patch preserve their original whitespace |

Raw historical and current logs retain trailing tool-output whitespace and terminal
blank lines. The preserved incremental patch also has literal blank context lines.
The unrestricted `git diff --cached --check` reports these evidence-only lines;
source, Markdown and JSON checks pass. No raw evidence was normalized or rewritten
just to silence whitespace diagnostics.

The frontend run includes real WASM selected/legacy request equivalence and refusal,
source/context races, manual acknowledgment, strict copy eligibility/rights,
projection/filtering, numeric presentation, Undo/Redo, serialization and output
freshness guards. No test budget/threshold was relaxed. No new browser replay was
needed for this commit-only operation: source bytes are identical to the accepted
[desktop/narrow selector review](FABCAM_PRESET3_AUDIT_AND_IMPLEMENTATION.md#unified-selector-ux-refinement--2026-10-21).
The prior broader core tests remain historical, not a new full-core/workspace run.

## Documentation, acceptance and preservation

Current STATUS, release/track roadmaps and the documentation index now identify
implemented/accepted PRESET functionality rather than the obsolete audit-only state.
The original [PRESET-1](PRESET_1_READONLY_2026_10_21.md),
[PRESET-2](PRESET_2_DUPLICATE_ADAPT_2026_10_21.md),
[layout](PRESETS_LAYOUT_POLISH_2026_10_21.md) and
[PRESET-3](FABCAM_PRESET3_AUDIT_AND_IMPLEMENTATION.md) reports retain their tested
source, historical failures, then-open gates and limits. Current acceptance pointers
supersede their old uncommitted/pending labels. ADR-0028 remains accepted unchanged.

The explicit human validation closes PRESET-2 `ivac-1oe9l`, PRESET-3 `ivac-ms6rz`
and completed engineering `ivac-pio0v`. PRESET-1 `ivac-0x2br` was already closed.
The older branch-wide identity-workflow gate `ivac-tnpyz` and V1/platform/physical
acceptance gates are independent and were left alone.

The initial inventory contained **8365 dirty tracked/untracked files**, including
local-only qualification evidence. None disappeared and no file outside reviewed
documentation updates changed; source bytes were preserved exactly.
[Preservation receipt](preset-integration-2026-10-21-evidence/preservation.json).
Selected PRESET reports, logs and screenshots are included in the documentation
commit. Large unrelated TEST1/qualification trees, prior RC/security reports and
ignored `dist-test` artifacts remain local and untouched; no retention policy was
changed and no evidence was deleted, moved or archived. Untracked unrelated files
may remain after the scoped commits; that is not an uncommitted PRESET implementation.

## Remaining boundaries and next action

- Native command/storage tests and compilation do not establish installed Tauri
  runtime or Windows/Linux acceptance. No new installation, package, controller,
  export execution or physical cutting was performed.
- Private applied KnowledgeSummary does not gain complete offline selected-profile
  revision identity; stored values remain snapshots. Cross-backend source rereads
  retain the previously documented concurrent-write residual window.
- Compatibility is checked on selection; Draft/ineligible records stay inspect-only.
  The existing 1024px application breakpoint can unmount an inspector-owned dialog;
  the library-owned selector supports continuous narrow-layout review.
- No push, merge, rebase, tag, version, release or subsequent milestone. Existing
  V1 qualification remains tied to its own source and independent human gates.

**HARD STOP after the local documentation commit.** The next proposed feature is
[MAT-1](RELEASE_ROADMAP.md#mat-1--material-families-variants-and-synonyms): material
families, variants and search synonyms over existing identities. It remains
unstarted; this acceptance/integration request does not authorize implementing it.
