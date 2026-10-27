# ALPHA-1 — Stock, import and diagnostic stabilization

Date: 2026-10-27. Tracking: `ivac-yi7u3` (bounded completion), parent
`ivac-4sylp` (broader alpha feedback, still open). Local software only.

## Source and preservation

Started in `FabCAM-alpha-feedback`, branch `fix/alpha-stock-diagnostics`, at
`34f41692ef1d90eddf1b3ff3b564d2b8600879d7`. Fifteen existing modified/untracked
files were inventoried and copied before editing, with a full tracked patch.
The original partial report and screenshot remain unchanged and untracked.
No reset, stash, cleanup, cherry-pick, push or installer creation occurred.

Source commits:

- `f43d5285e1bfc9acd64b2fdd142ca623881e93b6` — stock/import state, opening a
  drawing, persistence/history and stale-export regression tests.
- `5c6e26e89cdd5559a0f45f63d48ff213cde20076` — cutter validation, cleanup
  states/errors, frontend identity and their shared EN/DE messages.
- `b0c2f208bc0de1d38f733d378abbb0b85600785d` — watched-source reload uses
  the same fixed-stock command and atomic Undo/Redo.

The final executable source is **`b0c2f208bc0de1d38f733d378abbb0b85600785d`**.
A later documentation-only commit records this source without changing executable
files. No fast-forward of the primary checkout was performed. Keep this qualified
branch isolated until review; subsequent integration must recheck ancestry and
ownership of the primary worktree's untracked files.

Local evidence: `dist-test/alpha1/` (ignored, not distributed). It includes the
initial inventory, patch/copies, preservation receipt, gate logs and synthetic
browser screenshots. Source drawings and private screenshots are not committed.

Preservation verification compared SHA-256 for **8,479** pre-existing primary
checkout files and the original two alpha evidence files: no byte changes. Primary
HEAD remains `34f41692ef1d90eddf1b3ff3b564d2b8600879d7`; TOOL-META remains
`efe98dc2e7e46f4d12783dd8dbc9c0b09d2f773d`. No changes to frozen main/V1.
The read-only private DXF digest remains
`36ecdf0b3498b3df40fdf9e6c39bdf82fb300bf10dae96cb865578ca3bbf4a1a`.
Its current CLI import succeeds; parser output is local-only evidence. Ambiguous
units still require the existing explicit review. No claim that near-closed
contours were welded or that V-Carve continuity was repaired.

## Completed behavior

### Manual stock and work zero

Entering Manual or editing Length/Width anchors the lower-left world-space corner
to the **current** work zero. At zero (15,25), 320 × 200 gives (15,25)–(335,225).
The dimension and necessary XY offset compensation are one history command.
Changing zero alone does not move stock: the next dimension edit anchors to the
new zero. Explicit minimum-corner fields remain deliberate placement controls.
Z, original geometry and machine-coordinate conventions are unchanged.

Storage still uses the existing bbox-relative stock offsets. Compensation preserves
the effective footprint; it does not reinterpret saved offsets or introduce a new
project contract. Real v9 snapshot/restore regressions compare the effective
footprint and unchanged source coordinates. No schema/version change.

### Imports and automatic stock

First, replacement and additional imports keep parser coordinates and an identity
file transform; no automatic bed placement or WCS inference. Explicit DXF unit
review remains in place. Opening a drawing still clears the previous document's
operations/history, but preserves its configured stock and work zero through the
old-geometry clear and new-source installation. This document boundary is not an
undoable append; Add is undoable, including the first drawing in an active project.

Adding/removing drawings, deleting layers, adding segments and watched-source
reload now preserve manual bounds. Reload snapshots source data for Undo/Redo;
its regression checks structural equality rather than JavaScript object identity.
Native file watching itself still requires installed-app acceptance.

Auto remains drawing envelope plus the existing margin and placement offsets.
It follows geometry changes; it never moves WCS implicitly. Switching from Manual
to Auto retains existing offsets intentionally. Auto is not silently reset to a
new placement policy. Geometry outside a manual blank is retained; CAM stock,
work-area, SAFE-Z and simulation refusals remain authoritative and unchanged.

### V-bit validation

The draft requires a finite outer diameter and a finite tip satisfying
`0 <= tip < outer diameter`. A blank tip keeps the existing zero-tip convention.
Equal, negative, NaN and infinite tips are invalid; negative typed values are no
longer silently converted to blank. The Tip diameter field exposes an inline error,
`aria-invalid` and its associated explanation. Review/apply remains disabled until
valid. Other cutter-kind applicability and Rust generation guards are unchanged.

### Cleanup diagnostics

Disabled-action reasons use busy → stale → placement conflict → no selection →
missing/failed preview precedence. A failed preview can be retried but cannot be
applied. Reanalysis clears old results before requesting new ones. Changing the
selected repair resets the previous preview's failure state.

Placement protection remains global and conservative: the message names operation
and ID, explains that placements cannot yet be safely remapped, and states none
were removed. No automatic tab/approach deletion or remapping. Structured failures
show message/recovery hint; unknown JSON is kept out of the primary message and
retained in optional technical details. No topology predicates or attribution were
changed: a global crossing can still appear under “contour 1” (ALPHA-2).

### Build identity

A small navigation badge opens About, including keyboard activation. Package
version, frontend Git describe and dirty-source status remain available in About;
its explanation explicitly does **not** certify the native executable or WASM
assets. Narrow layout uses the existing Screen → About route. No header redesign,
version promotion or native source-certification claim.

## Qualification

Final exact-source run: `scripts/pre-release.sh`, **20/20 PASS**, on
`b0c2f208bc0de1d38f733d378abbb0b85600785d`. Only this report/STATUS/roadmap
were being edited; no executable-source changes during that final run.

- Rust workspace/all-features: **1,594 passed, 0 failed, 1 intentional ignored
  watchdog worker**. Formatting and strict all-target pedantic Clippy PASS.
- Frontend: **1,539 passed, 1 optional locale skip**, 176 files. Typecheck
  **0 errors, 0 warnings**; lint and production build PASS.
- Schema/version checks, schema/i18n generation drift, fresh WASM build and CPS
  size guard PASS. Dependency audit, headless smoke, CLI trust/transactional export,
  SEC1 workflow boundaries and candidate identity regressions PASS.
- Immutable CLI corpus: **23/23 exact**. Single-program cases compare gcode,
  toolpath, provenance, gcode index and statistics; two-sided case compares both
  emitted programs. No Rust/CAM/post/schema/dependency source changed.
- Stock setup and WCS edits invalidate generation keys and export qualification;
  existing work-area, stock-boundary, SAFE-Z, invalid-tool and ShopBot tests pass.
  No executable-motion difference was introduced for unchanged setup/operations.

The initial sandboxed gate failed because dependency download/DNS, advisory access
and local smoke-server permissions were unavailable. Logs are retained. After
fetching locked dependencies and using authorized execution, the full gate passed;
the final rerun above is the authoritative evidence. Earlier in-progress test
failures (a test-only optional offset type, snapshot reference equality, and a lint
escape) were corrected, not suppressed. Large frontend chunk warnings remain
nonblocking build observations. Historical partial counts are not reused.

### Actual browser walkthrough

Real app using WASM, isolated localhost origin; no native launch into normal app data.

| Scenario | Observation |
| --- | --- |
| First nonzero DXF | Synthetic bounds (80,60)–(100,70), scale 1 retained; Auto 30 × 20 with margin 5, zero remains (0,0) |
| Manual and nonzero zero | 300 × 200 at zero; set zero (15,25), resize to 320 × 200: min (15,25); Z stays 0 |
| Atomic history | Undo restores 300 × 200 and previous min (0,0); Redo restores min (15,25), 320 × 200 |
| Additional distant DXF | Bounds expand to (80,60)–(520,610); manual stock does not follow |
| Remove/Undo/Redo | Correct geometry count/bounds, stock min (15,25) and dimensions retained |
| Invalid V-bit | Tip = outer = 3 gives inline error and disabled Review; -1 remains visible and invalid |
| Corrected V-bit | Tip 0.2 allows explicit review/apply; save/reopen retains 0.2; subsequent invalid draft Revert restores 0.2 without applying |
| Cleanup selection/preview | 0.05 mm synthetic open gap: explicit selection, preview and Apply succeed; source preserved, Undo/Redo restores 4/5 effective segments |
| Stale cleanup | Changing tolerance after preview disables Preview/Apply with Analyze-again explanation; reanalysis recovers |
| Failed cleanup | Crossing synthetic case refuses preview with readable topology message; Apply disabled, failure explanation visible, Preview retry available |
| Placement protection | Auto tabs in Profile (#1) cause named global refusal; no placement removed |
| Identity | Badge Enter opens About; frontend/dirty explanation readable in dark/light 1440 px and About at 900 px |
| Save/reopen | Production frontend: downloaded v9 file inspected and reopened; stock 320 × 200, min (15,25), WCS (15,25,0), original bbox and identity transform retained; second save/reopen includes V-bit tip 0.2 |

The first dev-browser file chooser/download attempts failed and the disposable
page reset while development/build activity was ongoing. These failed attempts are
retained as limitations of that run, not counted as PASS. A separate compiled
frontend at localhost:18847 eliminated live development reloads: download capture,
byte inspection and reopening the actual saved v9 file then succeeded twice.
The compiled label is `b0c2f20-dirty` because tracked documentation was being
written; executable-source diff against that SHA was empty. Native binary identity
is not implied. Automated v9 round trips and import cancellation/failure tests are
additional, separate evidence.

The initial automation's fill-only numeric changes did not dispatch real change
commits; the walkthrough was repeated with keyboard input/blur and verified against
both stock summary and effective minimum corner. No application fix was needed for
that automation artifact.

Screenshots: `tool-invalid-desktop.png`, `cleanup-stale.png`, `cleanup-failed.png`,
`cleanup-placement.png`, `about-desktop.png`, `about-narrow.png`, `about-light.png`, `stock-reopened.png`, `tool-invalid-narrow.png`.
The invalid-tip field and footer actions were also inspected at 900 × 800: no
clipped message or overlapping critical controls; Revert remains reachable.
Earlier screenshots honestly show the starting SHA plus dirty; the reopened-stock
screenshot shows the final executable SHA plus documentation dirty state. They are
behavior evidence, not clean native-build identity receipts.

## Human/native acceptance and limits

**Native GUI NOT RUN.** The separate macOS account decision is preserved; no launch
in the ordinary profile, no installer and no physical ShopBot operation. Frontend
browser tests cannot qualify native dialogs, source watchers or filesystem ACLs.

On a separately authorized matching native test build/account:

1. Record frontend build in About and the independently supplied native build SHA.
2. Set Manual 300 × 200 at zero, move zero to (15,25), resize to 320 × 200;
   confirm stock min (15,25), unchanged Z and atomic Undo/Redo.
3. Open/Add distant synthetic DXFs, cancel another import, remove one and Undo;
   check original coordinates, fixed manual stock and explicit Auto behavior.
4. Save, quit/reopen the v9 project; compare stock, WCS, transforms and V-bit tip.
   Edit a watched source externally and verify fixed stock plus Undo/Redo.
5. Generate a valid disposable operation, then resize stock/change zero; old output
   must not remain exportable. Re-generate and review existing safety diagnostics.
6. Confirm invalid-tip inline feedback, cleanup stale/failed/placement explanations
   and About access in the actual window size. Never send QA output to a machine.

ALPHA-2 remains endpoint continuity/topology attribution/repair localization and
placement-remapping investigation. ALPHA-3 remains explicit Main/Finish semantics;
ALPHA-4 remains preset reuse. None was implemented. TOOL-META stays independent.
No loose V-Carve tolerance, geometry welding, postprocessor change or guard bypass.

## Recommendation

**CONDITIONAL GO for personal macOS software acceptance preparation**, not hardware
or distribution. Bounded implementation, full software gate and browser scenarios
above pass; the actual native walkthrough and final human UX acceptance remain
OPEN. A matching native rebuild is a separate next approval, not produced here.
ALPHA-2 can begin independently after explicit authorization, using this source as
its reviewed base; it has not been started. **HARD STOP for human review.**
