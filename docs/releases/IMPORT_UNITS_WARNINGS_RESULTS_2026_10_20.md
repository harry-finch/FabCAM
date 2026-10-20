# Import units and warnings UX — ADR-0026 — 2026-10-20

Tracking **ivac-feder** (follow-up of the import correctness checkpoint).
Bounded implementation on the uncommitted working tree based on `73b66bb`;
software and browser evidence only. No commit, push, package or publication.
Decision record: [ADR-0026](../shopbot-modernization/adr/ADR-0026-explicit-import-units.md).
Screenshots: [`import-units-warnings-2026-10-20-evidence/`](import-units-warnings-2026-10-20-evidence/).
**HARD STOP for human UX/architecture acceptance.**

## 1. Architecture review (phase A)

| Stage | Before | After |
| --- | --- | --- |
| Core importer (`ivac-core::input`, `dxf_in`, `svg_in`) | `ImportOutput { unit_scale, warnings, … }`; `ImportOptions.scale` existed but no transport exposed it | Adds `unit_status` (`unknown` / `declared` / `physical` / `assumed` / `explicit`) and `unit_source` (the raw declaration). `checked_import_scale` validates any explicit scale |
| Transports | server multipart, Tauri `import_path(path)`, WASM `importBytes(name, bytes)` | Each accepts an optional `scale`, validated by the core; schema/TS client regenerated |
| Frontend open/add flows (`file_ops.ts`) | Commit immediately | `reviewImport` gate before commit for assumed units or any warning |
| Persistence | `source` (the ImportResponse) stored in the project; `warnings` stored but **never displayed** | Same record, now with status and source; no new field elsewhere |
| Source reload (Tauri) | Re-import without options | Re-import with `unit_scale` only when status is `explicit` |

The one authoritative scale is the stored importer result's `unit_scale`. The
file transform (`fileTransform.scale`) is untouched and stays a separate,
user-visible placement edit; it is not used to express source units.

## 2. Warning propagation (phase B)

Importer warnings travel unchanged from the core through every transport into
`source.warnings`. The frontend (`state/import-units.ts → groupImportWarnings`)
deduplicates identical messages with a count, classifies them (units, missing
block, unsupported entity + layer, geometry approximations, other,
informational `_CAMCFG` note) and orders them by importance. Nothing is
dropped: unknown messages are shown verbatim, and each group expands to the
raw importer text. Fatal importer errors still reject the whole import and are
shown as an error notification; no partial geometry is committed. The importer
strings the classifier relies on are pinned by a Rust test
(`warning_texts_match_the_frontend_classifier`).

## 3. Unit contract (phase C)

| Source | Status | Behaviour |
| --- | --- | --- |
| DXF with `$INSUNITS` set | `declared` | Converted exactly; no unit choice offered |
| DXF without units | `assumed` | 1 unit = 1 mm by default, summary opens; choices 1 mm, 1 inch, custom |
| SVG with physical width/height (mm, cm, in, pt, pc) | `physical` | Converted exactly by usvg; an explicit scale is **refused** by the core (would convert twice) |
| SVG in px / unitless / viewBox only | `assumed` | 1 unit = 1 mm by default, summary opens; choices 1 mm, 96 dpi, 72 dpi, 1 inch, custom |
| User-confirmed choice | `explicit` | Re-import with `ImportOptions.scale` (even for 1:1) |
| Project saved before this change | `unknown` | Unchanged geometry and scale; no reinterpretation offered |

72 dpi is offered as one choice and never inferred (the Illustrator fixture is
a 72-dpi A4 written in px; it stays 595.28 × 841.89 mm until the user chooses
72 dpi). Custom scale: finite, 10⁻⁶ to 10⁶ mm per unit, comma accepted; the
core enforces the same range on every transport.

## 4. User interface (phases B and D)

Import summary dialog (`components/ImportReviewDialog.svelte`, mounted once in
`App.svelte`): file name, type, units declared in the file with a status chip,
scale, drawing size in mm (updates live with the choice), geometry count
(segments only in HTTP mode, where object metadata is not returned), unit
choices when adjustable, grouped warnings, then **Cancel import** /
**Import with these units** (or **Import** when units are not adjustable).
In the drawings sidebar, an imported file with warnings or assumed units shows
a ⚠ badge (with count) that reopens the summary read-only.

Browser smoke test (production build + release `ivac-server`, HTTP transport):

| Step | Result | Evidence |
| --- | --- | --- |
| Illustrator px SVG opens summary, default 1:1 | 595,28 × 841,89 mm, chip “assumed 1 unit = 1 mm” | [01](import-units-warnings-2026-10-20-evidence/01-svg-illustrator-assumed-1to1.jpg) |
| Choose 72 dpi | Size updates live to 210 × 297 mm | [02](import-units-warnings-2026-10-20-evidence/02-svg-illustrator-72dpi-choice.jpg) |
| Confirm | Committed bbox 210 × 297, `unit_scale = 0.35277…`, no badge | [03](import-units-warnings-2026-10-20-evidence/03-svg-illustrator-committed-210x297.jpg) |
| Unitless DXF with SOLID ×2 and missing block GHOST | 4 warnings in 3 groups, SOLID “×2” | [04](import-units-warnings-2026-10-20-evidence/04-dxf-grouped-warnings.jpg) |
| Expand a group | Raw importer message shown | [05](import-units-warnings-2026-10-20-evidence/05-dxf-warning-raw-details.jpg) |
| Escape | Dialog closes, nothing committed | DOM check |
| Badge after confirm | ⚠ 3 (units warning gone, status “your choice”); click reopens read-only | [06](import-units-warnings-2026-10-20-evidence/06-sidebar-badge-view-mode.jpg), [08](import-units-warnings-2026-10-20-evidence/08-sidebar-warning-badge.jpg) |
| Custom scale `0` | Field flagged invalid, size “—”, confirm disabled | [07](import-units-warnings-2026-10-20-evidence/07-custom-scale-invalid.jpg) |
| Physical mm SVG | No dialog, imported directly | [09](import-units-warnings-2026-10-20-evidence/09-physical-svg-no-dialog-and-fatal-error.jpg) |
| Tilted-OCS DXF | Fatal error notification (HTTP 400), nothing committed | [09](import-units-warnings-2026-10-20-evidence/09-physical-svg-no-dialog-and-fatal-error.jpg) |
| 375 px viewport | Dialog fits (359 px), no horizontal scroll | [10](import-units-warnings-2026-10-20-evidence/10-mobile-375px.jpg) |
| Keyboard | Focus inside the dialog moves summary → Cancel; Escape cancels | DOM check |

The smoke test found and fixed one layout defect: the badge made the remove
button wrap to a second line in the sidebar row (grid had three tracks); the
row now has four tracks.

## 5. Persistence and compatibility (phase E)

- Save/reopen: the stored ImportResponse carries `unit_status = explicit` and
  the chosen `unit_scale`; a lifecycle round-trip test checks both survive.
- Reload/reimport (Tauri source watch): `reimportFromPath` passes the stored
  scale only for `explicit` sources (`import-reload.test.ts`), so a declared or
  physical source is never rescaled and an assumed one stays assumed.
- Toolpath regeneration reads the committed geometry; there is no second scale
  to drift.
- Legacy projects: both new fields are `#[serde(default)]`; a source saved
  before this change reads as `unknown` with its geometry and scale untouched
  (`legacy_saved_source_reads_as_unknown_units`, frontend `unitStatus`).
- No post-processor, CAM or G-code change.

## 6. Before / after

| Situation | Before | After |
| --- | --- | --- |
| Importer warnings | Stored, never shown | Summary dialog plus sidebar badge |
| px / unitless SVG (Illustrator 72 dpi A4) | Silently 595 × 842 mm | Summary with live size; 72 dpi gives 210 × 297 mm, persisted |
| Unitless DXF in inches | Silently 1 mm per unit | 1 inch choice, persisted |
| Wrong-units fix | Manual file-transform scale | Explicit source units; file transform stays free |
| Cancel on doubt | Not possible (already committed) | Cancel import commits nothing |

## 7. Test results (phase F)

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | OK |
| `cargo clippy --workspace --all-targets --all-features -D warnings` | OK |
| `cargo test --workspace --tests` | 1606 passed, 0 failed (final tree) |
| `cargo test -p ivac-core --test import_correctness` | 5/0 (3 new: unit status/explicit scale contract, legacy unknown, warning texts) |
| Pedantic clippy gate (documented allow-list) | OK (after a `float_cmp` fix in the new test) |
| `cargo xtask schema-check`, `version-check` | OK |
| `wasm-pack` build + size guard | OK |
| vitest | 162 files, 1452 passed, 1 skipped, 0 failed |
| New frontend tests | `import-units.test.ts` (6), `import-review-flow.test.ts` (5), `import-reload.test.ts` (2), lifecycle round-trip (+1) |
| `svelte-check` | 0 errors, 0 warnings |
| `tsc -p tsconfig.node.json` | OK |
| eslint, prettier | OK |
| Component size guard | OK (LayerList at its 1000-line budget; budget not raised) |
| i18n coverage (no dead keys, en/de) | OK |
| Vite production build | OK |
| Browser smoke test | §4 |

**`scripts/pre-release.sh`** (Node 24, pnpm 10.34.7 from `~/.local/pnpm-10`,
cargo-deny 0.20.2; corrected 2026-10-20 — an earlier draft wrongly reported
pnpm and cargo-deny as missing): fmt, clippy with pedantic, workspace tests,
schema-check, README parity, lint, check, test, build, wasm-pack, wasm size
guard and cargo-deny all pass. The codegen and i18n drift guards report a diff
only because they compare against the git index while `generated.ts` and
`keys.ts` are still uncommitted; regenerating both is byte-identical to the
working tree. The script skips UI smoke when no `chromium`/`google-chrome` is on
`PATH`; `CHROME=… scripts/ui-smoke.sh` run separately with the installed Google
Chrome passes.

**Not exercised:** Tauri desktop UI and Android; the Tauri path is covered by
`import-review-flow.test.ts` (scale passed to `import_path`) and the Tauri
crate tests.

## 8. Limitations

- The units of an already committed source cannot be changed in place; re-open
  or re-add the file. (In-place change would need an undoable source-replace
  command; not in scope.)
- A browser session cannot re-read a saved source file, so browser projects
  keep the committed geometry; only Tauri reloads re-import.
- HTTP mode shows segment counts only (`ivac-server` does not return object
  metadata; pre-existing).
- The DXF unitless raw message still suggests the file-transform scale as a
  workaround; the summary's own text points to the unit choice. The raw text is
  kept unchanged because the earlier import checkpoint pinned it.
- Summary text is English and German only (the existing locales).
- Unchanged and out of scope: 3D DXF, MINSERT (refused), tilted OCS (refused),
  HATCH redesign, STEP, automatic geometric correction, post-processors.

## 9. Files

Core: `crates/ivac-core/src/input.rs`, `input/dxf_in.rs`, `input/svg_in.rs`,
`tests/import_correctness.rs`. Transports: `ivac-server/src/main.rs`,
`ivac-tauri/src/commands.rs`, `ivac-wasm/src/lib.rs`, `schema/openapi.yaml`.
Frontend: `api/*` (scale pass-through, regenerated `generated.ts`),
`state/import-units.ts`, `state/import-review.ts`,
`components/ImportReviewDialog.svelte`, `components/LayerList.svelte`,
`services/file_ops.ts`, `state/import-ops.ts`, `App.svelte`, i18n en/de, tests,
synthetic ImportResponse literals. Docs: ADR-0026, `QUICKSTART.md`, this report.

## 10. Human acceptance decisions

1. Accept the summary gate opening only for assumed units or warnings (clean
   physical/declared imports go straight in).
2. Accept that confirming 1 unit = 1 mm records an explicit choice (the
   summary will not reopen on reload for that source).
3. Accept the limitation “change units = re-open the file”, or schedule an
   in-place units change as a follow-up.
4. Accept the warning wording and grouping, including the read-only sidebar
   badge.
5. Approve committing this work together with the uncommitted import
   correctness checkpoint (ivac-feder); nothing is committed or pushed.
