# PRESET-1 — read-only cutting presets library

**Current disposition:** accepted and integrated locally on 2026-10-21.
[Commit/qualification receipt](PRESET_INTEGRATION_2026_10_21.md). The dated observations below describe
the original pre-integration snapshot, including then-open gates and historical failures.

2026-10-21. User authorized starting the V1.1 roadmap while staying on
`FabCAM-v1.1`. Inspected base `df6eb1f`; this implementation remains uncommitted.
Engineering `ivac-5x9a4`; prerequisite maintenance `ivac-fpx7n`;
**human UX review `ivac-0x2br` OPEN**. No PRESET-2 or persistent MAT-1 implementation.

## Delivered slice

Presets is a top-level desktop area immediately after Tools, also present in the
existing narrow-screen activity sequence. It works without a selected operation or
open project. The compact table browses the existing loaded reference/local packs,
including named and legacy Sorbonne packs. Fifty records per page bound DOM size.
Combined search/filter covers pack, material family, cutting-tool family, evidence
and source availability. Search includes names, context, diameter/flutes and metadata;
accent-insensitive text lookup does not establish machining equivalence.

The default view includes enabled sources when the active machine declares milling
support. All/disabled-source views explicitly expose disabled or process-incompatible
records without enabling them. This is a source/process filter, **not** a declaration
that a recipe matches the current machine, cutter, material or operation. Users can
browse other recorded contexts; only the existing engine can resolve applicability.
Draft/deleted/disabled statuses remain visible and do not gain recommendation status.
Empty collections, empty named packs and no-match states have explicit controls/text.

Rows show authored RPM, feed, plunge, DOC and stepover with units and unknown-value
markers. Chipload-only records retain an unknown feed; no calculation is performed.
Details show recorded machine/material/tool, source/record IDs and revisions, evidence,
license/attribution, trial notes, review and available history. Unknown IDs/revisions
remain readable. Existing normalized material IDs may supply their known labels and
nonpersistent top-level family labels; other family membership/taxonomy must be
explicitly recorded. No new aliases, persistent taxonomy, equivalence or migration.

All records are read-only in this library. Local editing remains in the existing
Settings → Feeds & Speeds → Lab presets workflow; references cannot be mutated here.
Browsing never calls recommendation/apply/generation or changes operation rates.
It reuses the existing source refresh/migration/head synchronization service rather
than introducing alternate storage, schema, matching or permission policies.

Row buttons support Up/Down/Home/End and visible focus; Tab reaches the details
scroll region. Details can be hidden for a wider table. Narrow layouts stack the
table/details and retain the shared activity navigation. Human legibility/focus and
installed-platform acceptance remain open, rather than inferred from browser checks.

## Prerequisite maintenance

Extracted the unchanged Group ops by tool control/styles into
`OperationToolGrouping.svelte`, and the compact preset identity diagnostic/styles
into `PresetIdentityHelp.svelte`. No cap increases or disabled tests. Counts under
the unchanged newline-split guard: OperationsList **982 / 1000**,
FeedsSpeedsSection **988 / 1000**, App **2189 / 2200**. The previous V1.1 failure
is corrected; the dated `6d4cff0` and roadmap reports retain their original results.

## Checks and scope

- `npm test`: WASM-package guard **10 PASS**; frontend **1482 PASS, 1 existing
  locale skip**, all **167** files pass, including component-size guard.
- Six projection/filter tests cover combined filters, accents, no mutation or
  invented feed, explicit families/revisions, disabled/non-milling sources, unknown
  IDs, optional fields and newest legacy head; navigation tests cover Presets order.
- Svelte diagnostics **0 errors / 0 warnings**, TypeScript node config, scoped
  ESLint/Prettier and production frontend/WASM-freshness build PASS. Existing build
  chunking/performance advisories are retained. No Rust/CAM source changed.
- Isolated browser `127.0.0.1:18813`: opened Presets without a project; displayed
  reference and local Draft records; combined Fablab X + MDF/diameter search;
  no-match/clear filters; hidden-details table; keyboard row selection; explicit
  disabled-source browsing (6 disabled records versus 2 enabled local records).
  Source enablement was restored after the test. No user desktop pack was edited.
- Maintenance browser smoke: temporary new project/Profile still shows
  **Choose a material first.**; duplicate Profile exposes Group ops by tool;
  toggle and Undo restore its prior state. No geometry generation or machining.

[Table](preset1-2026-10-21-evidence/library-table.png),
[local details](preset1-2026-10-21-evidence/local-details.png),
[maintenance help](preset1-2026-10-21-evidence/maintenance-help.png),
[tests](preset1-2026-10-21-evidence/frontend-tests.log),
[types](preset1-2026-10-21-evidence/typecheck.log),
[lint/format](preset1-2026-10-21-evidence/lint-format.log),
[build](preset1-2026-10-21-evidence/build.log).
[Source hashes](preset1-2026-10-21-evidence/source-sha256.json) identify this uncommitted slice; prior evidence remains untouched.
No installed desktop, Windows/Linux, whole-release or physical qualification claimed.
No commit, push, merge, release, version change or next milestone performed.

## Next human checkpoint

Review the read-only library, its table/focus/navigation, source distinctions and
record detail completeness (`ivac-0x2br`). PRESET-2 Duplicate & Adapt, advanced edit
entry points, persistent MAT-1 and scaling research remain separately scoped future
work. Existing V1 and V1.1 human/platform/physical gates remain independent.
**HARD STOP at this review checkpoint.**
