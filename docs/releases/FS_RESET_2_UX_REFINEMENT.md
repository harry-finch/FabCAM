# FS-RESET-2 — Cutting parameters and presets UX refinement

Date: **2026-10-22**. Branch: **FabCAM-v1.1**. Inspected parent:
`3d00dea` (including all four FS-RESET-2 implementation commits).
Engineering: `ivac-6ne31`; human UX acceptance: **`ivac-cj71l` OPEN**.
**Software-qualified UX checkpoint; HARD STOP for human review.**

## Scope and audit

Read [FS-RESET architecture](FS_RESET_ARCHITECTURE.md),
[implementation](FS_RESET_2_IMPLEMENTATION.md),
[ADR-0030](../shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md)
and the workshop UI/CAM/architecture/validation/documentation skills.
The current App already routes Presets to `TestedPresetsLibrary`; legacy components
remain inactive pending separately authorized FS-RESET-3. No parallel tracker task
claimed these components at inspection. Other worktrees, active tasks, untracked
qualification evidence and the pre-existing STATUS/RELEASE_ROADMAP edits were preserved.

The original library exposed management and browsing collection selectors together,
all filters over empty data, a header-only empty table, and page 1/1 for zero records.
The inspector used broad numeric columns, a long Profile explanation and duplicated
role-prefixed unknown-limit diagnostics. Related preset actions sat at opposite ends.

The audited role assignment is intentional: Profile contour emission and CPS section
metadata consume **Finish** (`pipeline/cps_ctx.rs::op_pass_kind` and existing v9
dispatch/output tests). Its stored Rough role remains explicit and required by the
current contract. Pocket keeps Rough/Finish, including a distinct finish tool;
Drill keeps Drill/Spot/Chamfer. This UX pass does not correct the separately reported
actual-role project-report defect (`ivac-tnu7d`).

## Changes

- **TestedPresetsLibrary.svelte:** concise Presets header/subtitle, two primary
  entry actions, one browsing collection selector including All collections, and
  contextual Manage disclosure explicitly naming its target collection. New/imported
  collections become the browsing scope. Selecting a row still identifies its
  collection for management; this never authorizes a preset load.
- Three distinct empty states: no collections, empty collection, no filter matches.
  Irrelevant headers/filters/pagination disappear. Clear filters retains collection
  scope. The save-operation action appears only for a selected milling cutting
  operation; authoritative identity/value checks remain unchanged.
- Populated browser retains search and machine/tool/material/operation filters,
  50-row paging and keyboard navigation. Rows measure **33 px**; long cells truncate
  with full titles and detailed metadata still available. Single-page results show
  a count without disabled paging controls. Filtering out a selected row hides its
  details rather than presenting an unrelated selection.
- **CuttingParameters.svelte:** compact role row and short Profile hint; stored main
  values are named explicitly. Numeric fields align label/value/unit with 82 px value
  inputs and existing shared 28 px controls. Wider containers use two columns; normal
  narrow sidebars retain aligned rows. Depth/entry conditions and all edit callbacks
  are unchanged. Specialized engagement/stepover controls remain in their existing
  operation sections; no duplicate or generic stepover value is introduced.
- Unknown-limit detail is collapsed initially and groups identical backend messages
  by their actual roles. The generic message is preserved: it does **not** establish
  which individual rating is missing. Missing rates, invalid values and known-limit
  errors remain red, immediately visible lists outside the disclosure.
- Load/Save actions sit together and use shared secondary/primary button utilities.
  Existing loaded/modified/manual provenance, replacement preview, unchecked save
  attestation, context pinning and history command remain authoritative. A disabled
  library preview explains that an operation must first be selected.
- **TestedPresetEditor.svelte / library:** replay exposed a pre-existing
  `DataCloneError` when collection deletion tried to clone a Svelte proxy. Snapshot
  reactive UI data before existing clone/edit/import boundaries. The same issue
  affected editing/saving; metadata save and delete-confirmation/cancel were replayed
  successfully after the fix. No CAS token, format, authorization, attestation or
  storage behavior was changed.
- **cutting-ui.ts / .test.ts:** presentation projections for empty states, role hints
  and exact-message grouping; three new tests. English/German copy and generated
  translation keys updated; obsolete unused copy removed without weakening guards.

## Rendered evidence and interactions

Actual Svelte components and actual WASM were used on disposable origins
`127.0.0.1:18815` and `localhost:18815`. No private workshop presets were touched.
The test collection contains **220 synthetic records** explicitly labeled SOFTWARE
ONLY. Its `tested` flag exercises the required format; no physical test is claimed.
The [fixture generator](fs-reset-2-ux-evidence/fixture-generator.ts.txt) validates it
through WASM. Temporary fixture controls used existing collection storage and
project snapshot/restore; they were removed from the source tree before final checks.

Before empty/Profile/limits were captured in the original full application. Additional
before details/Pocket/Load/Save images replay the `3d00dea` components in an isolated
component harness with the original removed translation strings restored in that
harness only. This is labeled baseline evidence, not a second application release.

| View | Before | After |
|---|---|---|
| No collections | [Original](fs-reset-2-ux-evidence/before-empty.jpg) | [Purposeful empty state](fs-reset-2-ux-evidence/after-empty.jpg) |
| Populated/details | [Baseline component](fs-reset-2-ux-evidence/before-details.jpg) | [220 records](fs-reset-2-ux-evidence/after-populated.jpg), [selected details](fs-reset-2-ux-evidence/after-details.jpg) |
| Profile | [Original](fs-reset-2-ux-evidence/before-profile.jpg) | [Compact fields](fs-reset-2-ux-evidence/after-profile.jpg) |
| Pocket | [Baseline component](fs-reset-2-ux-evidence/before-pocket.jpg) | [Rough T1](fs-reset-2-ux-evidence/after-pocket-rough.jpg), [Finish T2](fs-reset-2-ux-evidence/after-pocket-finish.jpg) |
| Unknown limits | [Original](fs-reset-2-ux-evidence/before-limits.jpg) | [Grouped detail](fs-reset-2-ux-evidence/after-limits.jpg) |
| Load | [Baseline component](fs-reset-2-ux-evidence/before-load.jpg) | [Existing load workflow](fs-reset-2-ux-evidence/after-load.jpg) |
| Save | [Baseline component](fs-reset-2-ux-evidence/before-save.jpg) | [Unchecked attestation](fs-reset-2-ux-evidence/after-save-unchecked.jpg) |

Additional evidence: [empty collection](fs-reset-2-ux-evidence/after-empty-collection.jpg),
[no matches](fs-reset-2-ux-evidence/after-no-matches.jpg),
[delete confirmation](fs-reset-2-ux-evidence/after-delete-confirmation.jpg),
[known-limit error](fs-reset-2-ux-evidence/after-limit-block.jpg),
[missing rate](fs-reset-2-ux-evidence/after-missing-rate.jpg),
[Spot](fs-reset-2-ux-evidence/after-drill-spot.jpg),
[Chamfer](fs-reset-2-ux-evidence/after-drill-chamfer.jpg),
[modified provenance](fs-reset-2-ux-evidence/after-modified.jpg).

Measured Profile block at the same **295 px width**: **321 → 288.86 px**, about
**10% less height**, no horizontal overflow. Wide-container fields use two columns;
the actual 760 px compact application viewport expands its operation drawer to a
695 px inspector ([capture](fs-reset-2-ux-evidence/after-narrow-inspector.jpg)).
The [760 px library](fs-reset-2-ux-evidence/after-narrow-library.jpg) measures
clientWidth = scrollWidth = 760; details move below the independently scrollable table.
Browser viewport override did not take effect, so responsive checks used a real
760 px iframe viewport, not a scaled screenshot. Original override was reset.

Replayed search for sample 220 (one result, no paging), 50-row paging to sample 051,
ArrowDown selection, long-name details, rename, delete confirmation/cancel, metadata
edit/save, actual load/replace, modified-from-preset, Undo/Redo, changed role/tool,
missing rate and known feed limit. Save remained disabled with unchecked attestation.
Snapshot/restore retained modified provenance. The final fresh application console
had no errors. [Recorded observations](fs-reset-2-ux-evidence/observations.json).

## Final qualification

All commands ran against this scoped UX working tree; no tests/budgets were weakened.

| Check | Result / evidence |
|---|---|
| `npm test` (Node 24) | **1583 PASS / 1 existing locale skip**, 186 files; WASM package guards **10 PASS**; component-size guards **3 PASS**. [Log](fs-reset-2-ux-evidence/tests.log) |
| `npm run check` | **0 errors / 0 warnings**, Svelte/TypeScript. [Log](fs-reset-2-ux-evidence/check.log) |
| `npm run lint` | ESLint/Prettier PASS. [Log](fs-reset-2-ux-evidence/lint.log) |
| `npm run build`, offline Rust tooling | WASM package fingerprint verified; production build PASS. Existing chunk-size/plugin-timing advisories remain. [Log](fs-reset-2-ux-evidence/build.log) |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --lib cutting_v9_tests` | **12 PASS**, 1243 unrelated tests filtered. Existing standard/specialized/two-sided/ShopBot output comparisons retained. [Log](fs-reset-2-ux-evidence/core.log) |

## Limits, Git and human checkpoint

No Rust production code, cutting structures, v9 serialization, collection format,
role dispatch, matching, numerical limits, SAFE-Z, simulation/export qualification,
ShopBot output, native permissions or release workflows were changed. No V1 baseline,
installer, hardware operation, push, merge, rebase or version promotion.

Browser import chooser replay failed at `fileChooser.setFiles` because the detached
input's backend node could not be found. Export control reached “Collection export
requested”; no downloaded file contents were verified. These do not qualify native
dialogs or an OS file round-trip. Collection validation/import unit coverage remains
passing. The successful project round-trip above is in-memory snapshot/restore.
The two pre-existing native watcher failures and separate native-import `stat` /
project-report acceptance defects remain independent and were not fixed or waived.

One focused local UX commit contains this report, the reviewed source/translation
changes and only this evidence directory. Its exact hash is recorded in the final
handoff and tracker receipt; recover it with
`git log -1 --format=%H -- docs/releases/FS_RESET_2_UX_REFINEMENT.md`.
The pre-existing STATUS/RELEASE_ROADMAP modifications and unrelated local evidence
remain outside the UX commit.

Human review: density/readability, role wording, empty states, long-name details,
warning discoverability and responsive behavior. This checkpoint does not close
FS-RESET-2 acceptance gate `ivac-y25vq` or authorize FS-RESET-3. **HARD STOP.**
