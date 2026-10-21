# FabCAM V1.1 — roadmap and branch reconciliation

Snapshot **2026-10-21**. Documentation-only task `ivac-ikanz`; no new feature,
component extraction, release or human acceptance performed.
[Canonical milestones](RELEASE_ROADMAP.md#fabcam-11--presets-and-materials-ux) ·
[STATUS](../shopbot-modernization/STATUS.md) · [V1 readiness](V1_READINESS.md).

## Actual topology and commit disposition

Repository: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
Tracked tree/index clean before this task; contrary to the request's initial
assumption, all existing V1.1 work was already committed. Preserve these commits
without splitting tightly coupled implementation, rewriting or duplicating them:

| Commit | Disposition |
|---|---|
| `0075317` | Existing 72-file implementation: named packs, local/reference identities, library-tool workflows and UI; retained together |
| `b417d3c` | Existing macOS web-development launcher; retained separately |
| `6d4cff0e6fe0ccff6d11e5c88cd98b36e028cdbf` | Existing documentation/ADR-0027 and scoped verification evidence; inspected/tested source |

`FabCAM-v1.1` is three commits ahead of `main` at inspection. Actual merge base,
local `main`, `origin/main` and GitHub main all equal
**`ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`**. No V1 hotfix delta exists after
the fork at this snapshot. RC1 is **RC1-ef807cf-39492251757**, beta.3/pipeline63;
[CI 39487569970](https://github.com/harry-finch/FabCAM/actions/runs/39487569970)
and [candidates 39492251757](https://github.com/harry-finch/FabCAM/actions/runs/39492251757)
completed successfully. CI/build success is not installed or physical acceptance.
Since the older freeze baseline `85a93c4`, main added `ef807cf` (prepare verified
WASM before frontend tests); that fix is already the V1.1 base.

| Checkout / local ref | Source | Treatment |
|---|---|---|
| Primary `ivacam` / `FabCAM-v1.1` | `6d4cff0` | Only this checkout receives this documentation commit |
| Sibling `FabCAM-docs-consolidation` / `codex/sec1-roadmap` | `7e1c24b` | Preserved untouched |
| Sibling `FabCAM-v1-freeze` / `codex/v1-freeze-readiness` | `85a93c4` | Historical checkpoint, preserved untouched |
| `codex/docs-consolidation` | `744774d` | Already integrated ancestor; no duplicate integration |
| `feature/shopbot-mvp` | `c9d3f3b` | Historical ref retained |

8,245 untracked files were present at inventory, overwhelmingly local historical
evidence, including RC1 candidate/security reports. None are staged or disposed of.
Private packs outside the repository remain untouched. Existing implementation
consolidation `ivac-uvue4` is CLOSED; no second implementation commit is needed.
This task adds one focused documentation commit only; its Git commit contains this
report and the related canonical documentation updates, without historical archives.

## Implementation inventory

The [existing technical report](FABCAM_V1_1_UX_2026_10_21.md),
[ADR-0027](../shopbot-modernization/adr/ADR-0027-lab-and-reference-identities.md)
and [source authoring guide](../feeds-speeds/CUSTOM_DATA_SOURCES.md) retain detailed
contracts. Inspected implementation agrees with the following bounded inventory:

| Capability / source entry | Classification and qualification limits |
|---|---|
| Named packs: `feeds-speeds-pack-store.ts`, `LabPresetDialog.svelte`, `LabPresetManager.svelte`, native `source_packs.rs` | Implemented/tested; human UX acceptance pending. Name/create/select destination, independent immutable heads/history, import/export and legacy Sorbonne retained. Draft/inactive capture; conflict checks. Browser removal transactional; native removal can partially fail on I/O and report retry rather than promise filesystem atomicity |
| Physical/reference identities: `lab-presets.ts`, engine validation/model, ADR-0027 | Implemented/tested; human acceptance pending. Separate local physical IDs and reference compatibility, exact local Tested/Validated precedence and visible provenance. Legacy shared identity cannot be retrospectively split; frontend/native must be updated together |
| Library-tool assignment: `operation-tool.ts` and tests | Implemented/tested; human acceptance pending. Main/finish/spot tools, stocking, templates/palette, undo/redo; explicit values retained with original context/stale warning. Incompatible legacy assignments remain visible, not silently deleted |
| Operation catalog and inspector/navigation polish | Implemented; scoped automated/browser evidence, human UX acceptance pending. Explicit Add catalog, consistent labels, header spacing and compact separators/navigation. Browser smoke is synthetic, not installed-app or physical acceptance |
| Cross-source Presets library | Not implemented. Navigation currently has Tools but no Presets area; per-pack manager is not PRESET-1 |
| Library Duplicate & Adapt | Not implemented. Current-operation capture is not PRESET-2 |
| Material family/alias search | Not implemented. Current MaterialPicker uses a select over revision-keyed records and preserves unknown selections; no MAT-1 taxonomy |
| Overall V1.1 qualification | Partial: targeted regressions pass; full frontend has a structural gate failure. No full integrated-source, clean-install or machining acceptance |

No implemented capability is classified as released or physically qualified.
`ivac-tnpyz` remains the existing V1.1 UX acceptance checkpoint; closed pack/UI
implementation trackers are reused rather than recreated.

## Planned scope and ordering

The authoritative [release-plan section](RELEASE_ROADMAP.md#fabcam-11--presets-and-materials-ux)
contains scope, dependencies and acceptance for PRESET-1's table/details,
PRESET-2's new-Draft copy workflow and compatible/incompatible matrix, and MAT-1's
identity-preserving taxonomy. They are planned, distinct from named-pack capture.
Parameter Scaling Assistant remains separate future research reusing/auditing
FS3.9 bounded adaptation, with no automatic application or validation claim.

Recommended next maintenance: review/authorize `ivac-fpx7n` bounded component
extraction to regain a green unchanged size gate. Recommended next **feature**:
PRESET-1 read-only table/details, including source permissions/provenance and empty
states. Nonpersistent MAT-1 family mapping may support its filter after review;
PRESET-2 follows explicit compatibility/identity/rights decisions. No work begins
on these items under this planning authorization; no V1.5 dependency is imposed.

## Current-source checks and failure origin

All following checks ran on tracked source `6d4cff0`, before documentation edits;
no production code/tests changed. Local-only logs:
`dist-test/v1.1-roadmap-2026-10-21/` (excluded from this commit).

| Command | Actual result |
|---|---|
| `pnpm --dir frontend test` | Exit 1: Vitest 1474 PASS, 1 locale skip, 1 FAIL; 165 files pass / 1 fails |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-feeds-speeds --locked` | Exit 0: 113 integration PASS + 1 doctest PASS |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-core feeds_speeds::tests --locked` | Exit 0: 19 adapter PASS; other suites filtered |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-core project::tool::tests --locked` | Exit 0: 6 tool/process PASS; other suites filtered |
| `RUSTC_WRAPPER='' cargo test --offline -p ivac-tauri --lib source_packs --locked` | Exit 0: 7 native source-pack PASS, 10 filtered |

Frontend used host Node24/pnpm; targeted Rust used the installed toolchain and
cached offline dependencies. Earlier check/lint/build/browser receipts in the
technical report remain historical, not fresh results from this task.

The sole frontend failure is `component-size-guard.test.ts`, unchanged from RC1,
with a 1000-line budget. Counts use the guard's newline-split rule:

| Component | RC1 `ef807cf` | Implementation `0075317` | Tested `6d4cff0` |
|---|---:|---:|---:|
| `OperationsList.svelte` | 1000 | 1008 | 1008 |
| `op_properties/FeedsSpeedsSection.svelte` | 941 | 1010 | 1010 |

Thus the overages existed before the final named-pack checkpoint but originate in
V1.1 implementation relative to RC1; they are **not a pre-existing V1 defect**.
Tracked as **`ivac-fpx7n`**, open, separately authorized extraction without UI
redesign, cap increases or disabling the guard. No refactoring performed. Feature
branch commits remain incompletely qualified and **not merge-ready**.

## V1 / V1.1 policy and human decisions

V1 stays frozen for installation/runtime and supervised ShopBot qualification.
Only independently demonstrated acceptance-blocking defects (unsafe output,
geometry/units/provenance, critical refusal/export, corruption or installation)
belong on stabilization, each with a focused commit, regression and renewed
exact-candidate-SHA qualification. Features stay isolated on `FabCAM-v1.1`.

After V1 acceptance and release decisions, inspect the then-current V1 hotfix
delta, agree a reviewed forward integration strategy, preserve both sides' qualified
changes and run full software qualification on the combined source. Establish a
separate V1.1 candidate/acceptance matrix; V1 passing never accepts V1.1 implicitly.
No integration, rebase, push or release is performed here.

Remaining human decisions are: accept/revise existing V1.1 UX; authorize bounded
maintenance and then PRESET-1 scope/order relative to other tracks; review table
interaction/accessibility and material taxonomy coverage; define adaptation
compatibility, permissions and incomplete-Draft behavior; approve a separate
architecture/migration decision before persistent MAT-1 changes. Existing physical
identities remain authoritative; aliases do not establish machining equivalence.
V1 platform/workshop and release decisions remain independent and unclosed.

The local [RC1 receipt](V1_RC1_CANDIDATES_RESULTS_2026_10_20.md) and local logs are
available in this workspace but not shipped by this documentation commit. Their
portability/retention requires separate evidence handling; GitHub run links and
full source SHA above provide portable candidate identification. Historical reports,
accepted ADRs, qualification evidence and version/pipeline metadata are preserved.

**HARD STOP after this roadmap/report and local documentation commit.** No new
PRESET/MAT implementation, extraction, push, main merge, tag, release, installer,
version change or hardware operation. Next milestone requires separate authorization.
