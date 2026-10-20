# NUMSAFE3 — resource/export audit and decision gate

Date: 2026-10-20. Inspected HEAD: `274f543`; integrated engineering baseline:
`2c6e4e1` (NUMSAFE2), beta.3/pipeline63. Tracker: `ivac-fh2av`.

**The initial audit below is retained; the approved implementation follow-up is
recorded at the end of this report.** The initial Phase A stop was resolved by the
maintainer's approval of all five budgets and restriction to new two-sided output
names. Existing untracked qualification evidence was preserved. Supplied and
repository copies of the four requested skills are identical.

## Confirmed findings and actual exposure

| Path | Finding | Existing controls / relevance |
|---|---|---|
| `cam/waterline.rs::z_levels` | Eager, unbounded vector built by repeated subtraction. Finite tiny steps can require enormous counts; subtraction can cease progressing. | `waterline_rough_cancellable` rejects nonfinite inputs, but positive finite step has no count bound. Cancellation occurs before planning and per level, after this vector is built. These are genuine depth levels: the approved 1000-depth-pass policy is semantically appropriate. |
| `pipeline/op_drivers/waterline.rs::heightgrid_skin` | Two triangles per grid quad, unchecked capacity expression, no inner cancellation. Mesh is constructed before roughing. | The STL import cell cap does not bound this mesh to a modest allocation: a 4000×4000 grid produces 31,984,002 triangles, about 1.15 GB of triangle payload alone. Loaded heightgrids need their own validation. |
| `cam/waterline.rs::clear_level`, `cam/offsets/pocket_fill.rs` | Waterline delegates to shared pocket clearing. Its lateral scan loop is not a depth pass. A depth cap alone does not bound the work inside a slice. | No cancellation argument through `clear_level`; stitching, nesting and Boolean work remain separate concerns. Shared-pocket changes require regression coverage, not a Waterline-only shortcut. |
| `cam/thread.rs::helix_waypoints_with_density` | `ceil(max(abs(dz)/pitch,1) * density) as usize`, followed by `total_steps + 2`, can overflow before allocation. Ordinary large counts can exhaust memory. | Finite positive pitch is insufficient. The driver accumulates paths across `radial_passes: u32` and selected circles; it polls cancellation per object, not per helix point/pass. Helix samples and radial passes are not depth passes. |
| `cam/surface_mill.rs::tip_kernel_cancellable` | Signed i32 arithmetic in `(2*kr+1)^2` can overflow; smaller excessive kernels still cost memory and `cells × kernel` work. | There are cancellation checks per kernel row, field row, 64 cells and 1024 footprint visits. These are useful but do not bound allocation/sorting or total workload. |
| `cam/surface_mill.rs::axis_positions` | Repeated floating-point addition can cease progressing at large origins. Scanline vectors are eagerly allocated without cancellation inside the helper. | Along-step is at least quarter-cell and 0.001 mm; lateral step is at least half-cell. These reduce typical sampling, but are not an overall point/memory budget or a floating-point progress guarantee. |
| `pipeline/op_drivers/raster.rs` | Streaming raster routes intentionally have no total pixel cap; row/column buffers and processing can be large. | The 16M-pixel cap only applies to AlongY + Floyd–Steinberg whole-grid materialization. It is **not** a universal raster limit. Cancellation is checked when emitting a scanline, not throughout all preprocessing of that line. Laser/generic capability, outside qualified ShopBot milling. |

Waterline, Relief and Thread are present in the milling UI's special-operation
group (`OpKindPicker.svelte`). `inspector-policy.ts::operationCapability` does
not disable these three for ShopBot. Their CPS strategies are respectively
`contour`, `parallel`, `thread` (`pipeline/cps_ctx.rs`), all outside the bundled
ShopBot allowlist (`crates/ivac-cps/posts/shopbot.cps`). UI availability therefore
does **not** establish qualified ShopBot export, nor does a post refusal by itself
bound earlier planning. Generic built-in generation remains an affected path.
No new machine/post pairing is proposed.

Other existing bounds are retained: shared 1000 depth passes, planar 100,000
scanlines, V-Carve sweep/point budgets, STL 16M cells. These do not imply a global
operation memory/CPU limit. A complete review of every specialized loop is still
pending; this audit stopped when the required policy decision was established.

## Safe baseline reproductions

Run from the repository:

```sh
python3 scripts/numsafe3-audit-probes.py
```

Executed successfully on the inspected checkout (exit 0):

1. Actual Waterline helper accepts 1001 levels (small allocation).
2. Waterline arithmetic at top=-1e20, bottom=-2e20, step=1 satisfies the loop
   condition while subtraction leaves Z unchanged. Only one step is evaluated.
3. Actual Thread helper with depth=1 and pitch=1e-300 panics at `total_steps+2`
   with overflow checks enabled, **before allocation**. The harness catches the
   expected panic. This is not a claim about release-mode panic behavior.
4. Relief capacity arithmetic at radius/cell=30,000 exceeds i32 multiplication
   range (e.g. radius=3 mm, cell=0.0001 mm). No kernel is allocated.
5. Relief axis arithmetic at origin=1e16, span=10, step=0.5 does not advance.
   Only one step is evaluated.

The harness imports the actual Thread module with minimal type stubs and extracts
the actual Waterline helper. Relief/progress probes are arithmetic witnesses, not
full-pipeline regression tests. No billion-iteration or excessive-memory test ran.
Replace these baseline expectations with bounded-refusal regression tests after
approval. No geometric overcut or valid-output regression is inferred here.

## Proposed policy — not approved or implemented

Recommended bounded implementation:

* Extend the **existing 1000 depth-level limit to Waterline**, counting the final
  floor once. Check counts before mesh generation or entry moves. Preserve valid
  level values; reject non-progressing arithmetic rather than silently coarsening.
* Establish separate resource units for specialized planning. Proposed starting
  ceilings: **2M generated mesh triangles; 1M footprint entries; 100M conservative
  cell×footprint evaluations; 1M retained XYZ points per specialized operation**,
  aggregated across selected circles/passes/scanlines/levels as applicable.
  Count kernel bounding-square work as well as retained disc entries. Check
  products with checked arithmetic before allocation; do not rely on cancellation
  or an allocation failure as a refusal mechanism.
* These are **product limits requiring approval**, not measured universal memory
  guarantees. 1M XYZ tuples alone occupy roughly 24 MB; temporary structures,
  sorting, posts and simulation add more. The conservative Relief work budget
  may reject requests that pruning would process quickly. Test representative
  projects before treating these values as qualified; do not silently increase
  them to make a fixture pass.
* Keep raster's intentional streaming contract out of an automatic global cap.
  Record its residual risk; a separate laser/resource decision is needed to change
  that contract. A shared pocket scanline limit also needs explicit scope review
  because it affects ordinary pockets, not only Waterline.

Alternative: fix only arithmetic/progress and Waterline depth counts now, leaving
Thread/Relief workload sizes explicitly unbounded. **Not recommended** as completion
of NUMSAFE3, because it does not close the confirmed finite-input resource risks.

Architecture: **EXTEND** existing operation-specific preflight/error paths and
cancellation plumbing; **KEEP** geometrical sampling, valid output and post guards;
**DEFER** broad planner/Boolean redesign and unsupported capability qualification.
Use structured refusal, never truncated success or automatic resolution changes.

## Cancellation review

After budget approval, add bounded polling to mesh construction, Thread passes
and helix points, and Relief axis/path construction. Preserve deterministic point
order and propagate `Cancelled` distinctly from resource refusal. Test cancellation
after work starts, then clean regeneration using the same service.

Kernel sorting, stitching and Boolean calls are still noninterruptible spans.
Budgets alone do not prove the existing <=200ms p95 target. Keep `ivac-42ylx`
open; measure representative workloads. A broader redesign is not authorized by
this proposal. WASM worker termination/recovery remains a separate existing path.

## File export audit and proposed contract

Confirmed:

* CLI `cmd_stream_gcode` calls `File::create` on the final destination before
  generation; it truncates an existing target and can leave partial output.
* CLI two-sided generation completes in memory first, then `write_program`
  uses direct `std::fs::write`, front then back. A write failure can damage a
  target; the pair has no all-or-nothing publication contract.
* Desktop `file_ops.ts` checks current generation and independent safety before
  `writeTextFile`, including after the save dialog. Those checks are valuable,
  but the direct native write is not transactional on disk failure.
* Browser downloads use a completed Blob. FabCAM cannot claim atomic destination
  replacement by the browser. HTTP/stdout already-emitted bytes cannot roll back.

Proposed implementation after approval: a small native transactional writer
shared by CLI and desktop, with a uniquely created sibling temporary file,
complete generation/safety validation, explicit flush/close, then supported
atomic publication. On generation/cancel/write/publication error, clean up the
temporary file and preserve the prior target. Never fall back to truncate/copy
when atomic replacement fails. No crash-durability or universal-filesystem claim.

Preserve desktop dialog permission scope and existing export qualification;
a new unrestricted arbitrary-path Tauri command is not acceptable. Audit the
native dialog/FS scope integration before selecting the command API. Refuse
directory/symlink destinations rather than following or silently replacing links;
check parent/path handling and races using platform-supported primitives.
Permissions/metadata and Windows replacement behavior require explicit tests.

Recommended two-sided contract: stage **both** complete programs before publication,
then atomic replacement **per file**. Two renames do not form a transaction:
failure publishing the second can leave a new front and old back. Report that
precisely, never overall success. If pair-wide atomicity is required, use a
versioned directory/manifest publication model; that changes the CLI contract and
requires a separate architecture decision. **Approval requested for per-file
atomicity with this explicit two-sided limitation.**

## Qualification and remaining work

Only the bounded audit probes above ran. No production behavior changed.
NUMSAFE/NUMSAFE2, specialized regression suites, workspace all-features, strict
and pedantic Clippy, TEST1, SAFE-Z, ShopBot M1/M2, CPS/V-Carve, frontend/desktop
export tests and `scripts/pre-release.sh` were **not rerun for this interim audit**.
Historical integrated results remain historical; no new byte-identity, cancellation
latency, disk-failure, path/symlink or installed-platform qualification is claimed.

After approval: finish the loop inventory; implement resource refusals and
cancellation; implement file transactions; test boundaries, partial generation
failure, old-target preservation, cleanup, permissions, paths, cancellation/retry
and valid byte identity; run all requested gates. Make separate scoped resource
and export commits **only after successful qualification**, then HARD STOP.

Human decisions now: approve/adjust specialized resource ceilings and scope;
approve per-file atomicity for two-sided CLI exports. No commit at this gate:
the user's commit authorization is conditional on qualified bounded changes.
Software evidence never establishes physical machine safety.

## Approved implementation follow-up

The maintainer approved the five explicit ceilings above, with **no overwrite of
an existing two-sided pair**. Implementation follows
[the resource contract](NUMSAFE3_RESOURCE_BUDGETS.md). No geometry coarsening,
ShopBot allowlist expansion, project-format change or output-format change.
Application version remains beta.3 / pipeline63.

### Resource and cancellation implementation

* Waterline uses a checked depth schedule before mesh creation; repeated Z
  subtraction retains the old valid values, with a 1000-level bound and explicit
  non-progress refusal. Checked mesh dimensions precede triangle allocation.
  Mesh/grid traversal polls cancellation. Clearing supplies a shared remaining
  point budget through the existing pocket raster implementation.
* Thread checks pitch-derived counts before helix allocation, radial passes
  before their loop, and aggregate points across selected circles. The 17-point
  lead-in and optional whirling expansion consume the same operation budget.
  Whirling receives the **remaining** point allowance, not a fresh million for
  each pass. Helix, pass and whirling loops poll cancellation.
* Relief validates the footprint/evaluation work before cloning the source.
  Exact footprint entry counting bounds its allocation; checked cell×footprint
  products bound the conservative work estimate. Axis positions reject
  non-progressing increments and excessive counts. The scanline product is
  checked before drop-cutter work; axis creation and final point sampling poll
  cancellation, including inside long individual scanlines.
* Fallible planner results reach pipeline errors; no partial operation is
  returned as successful or cached as complete. Legacy convenience wrappers
  remain infallible signatures and may panic on refused inputs; production
  specialized drivers use checked interfaces. The Relief cancellable Rust helper
  now returns `Result<Option<_>, String>`; all repository callers were adapted.

Additional inventory: planar surfacing already bounds **total** scanlines
(passes×lines <=100,000), as well as 1000 depth levels. V-Carve retains its 4096
sweep/1M point policy. Halfpipe retains its existing depth checks and planner;
its input/medial geometry is not newly globally capped here. T-slot/dovetail use
the existing contour-offset route, not a new independent depth-step scheduler.
Laser raster's intentionally unbounded streaming routes remain outside this
ShopBot milling qualification. No universal resource-safety claim for every
inherited operation follows from the targeted changes.

### Transactional export implementation

`ivac-file-export` is a small native transport utility, shared by CLI and Tauri;
it has no CAM/post dependency. It stages a uniquely named sibling file, explicitly
flushes/synchronizes it, closes the staging file before publication, and publishes
the completed contents. It never falls back to truncating/copying over a target.
Ordinary replacement uses `tempfile`'s platform rename. New-only publication uses
atomic hard-link creation of the final name, then removes the staging name; a
filesystem without that capability fails closed.

Single-file CLI export preserves the target until successful generation and
publication. Desktop program export retains current-generation/independent
qualification checks, then invokes the native helper after the save dialog.
The native command resolves the destination and verifies the existing dialog/FS
scope before staging. Symlink, directory and read-only final destinations are
refused; existing regular-file permission bits are retained on replacement.
Parent resolution assumes a normal user-controlled destination directory, not
a hostile concurrent directory-tree mutation environment.

Two-sided CLI keeps `<base>.front.<ext>` / `<base>.back.<ext>`. Both names must be
new: an existing name causes refusal and the user must choose a fresh `--output`
base. Both programs are generated, staged and flushed before either publication.
Each final-name creation is no-clobber, including concurrent-file races.
Success summaries are printed only after both publications succeed.

**A second publication failure may leave the newly created complete front file.**
It is reported explicitly as partial publication, never overall success; existing
files are not overwritten and the user must choose another fresh base. This is
not pair atomicity. There is no rollback deleting a possibly concurrently
modified published file.

Machine-program exports are the scoped change. Project JSON, notes and STL
saves retain their existing workflows. Browser downloads use the already-complete
Blob; HTTP/stdout bytes cannot be rolled back. CLI streaming retains its existing
no-preview/no-toolpath-derived-warning contract; file transactions do not add
geometric qualification to that route. ShopBot/CPS guards are unchanged.

Ordinary error/unwind cleanup removes staging files. Forced process termination,
filesystem cleanup errors and power loss may leave a hidden staging file.
Directory entries are not fsynced: no crash-durability claim. Windows/Ubuntu
installed execution and network-filesystem replacement are not qualified here.

### Tests and evidence

Executed on macOS arm64, software only:

| Check | Result |
|---|---|
| Existing CAM unit subset | 273 PASS |
| NUMSAFE / NUMSAFE2 | 7 / 9 integration tests PASS, plus shared unit tests |
| NUMSAFE3 | 8 integration tests PASS; 2 resource counter unit tests PASS |
| Native file transactions | 9 unit tests PASS |
| SAFE-Z extended | 15 PASS |
| ShopBot M1 / M2 | 11 / 27 PASS |
| CPS / V-Carve | 5 / 10 PASS |
| Workspace all-features | 1654 PASS, 1 ignored; final frozen-source run recorded with the pre-release gate |
| Canonical strict/pedantic Clippy | PASS (workspace/all-targets, repository allow-list) |
| Frontend | 1458 PASS, 1 skipped; export qualification mocks now exercise the native command boundary |
| TEST1 release campaign | 256/256 PASS in 27.66s, including frontend lifecycle 32/32 |
| Existing output corpus | 136/136 byte-identical to the pinned NUMSAFE SHA-256 corpus |
| Real CLI export regression | PASS: prior target preserved after second-op refusal; temporary cleanup; file/stdout byte identity; old pair refusal; fresh pair success; failed back-side generation publishes neither |

The new tests include finite tiny pitch/step, stalled floating-point increments,
mesh/count boundaries without large allocation, independent Relief footprint,
work and retained-point refusals, a representative 256×256 relief with a 6mm tool,
remaining-budget whirling, refusal/cancel then regeneration, point-for-point
bounded-clearing parity, disk-write fault injection after a written prefix,
publication failure, read-only paths, symlinks, new-only races and cleanup.
Cancellation tests prove refusal/recovery semantics; they do not establish an
in-flight latency distribution or a universal 200ms deadline.

Reproduction commands:

```sh
RUSTC_WRAPPER='' cargo test --offline --workspace --all-features
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --all-features --test numsafe3
RUSTC_WRAPPER='' cargo test --offline -p ivac-file-export
RUSTC_WRAPPER='' cargo build --offline -p ivac-cli
python3 scripts/numsafe3-cli-check.py
python3 scripts/numsafe3-audit-probes.py # deliberately reproduces pinned 274f543
python3 scripts/test1/build.py         # prints executable for the next command
python3 scripts/test1/run.py --binary <printed-executable> --output <new-directory> --cases 32 --budget 120 --frontend
scripts/pre-release.sh
```

The pre-release environment uses Node24, pnpm10, the existing Chrome exec wrapper,
and the previously qualified macOS `stat -c %s` compatibility wrapper. Native
watcher tests require execution outside the sandbox. The first sandbox run had
two watcher timeouts; the unsandboxed workspace run passed. An initial pre-release
run failed only on formatting in the two changed frontend test mocks; fixed
before the successful rerun. An additional **all-features pedantic** experiment
reported two existing `must_use_candidate` warnings in unchanged `profiling.rs`;
it is not the canonical pre-release Clippy configuration and remains an explicitly
unpassed extra combination. No unrelated profiling edits were made.

Qualification logs and source/evidence receipts are retained locally under
`dist-test/numsafe3-2026-10-20/`, not committed as large generated evidence.
The final pre-release receipt and commit handoff are recorded below.

### Human gate and limits

No push, merge, installer packaging, publication or hardware operation.
The script's generic “ok to release” footer is **not** release authorization.
`ivac-42ylx` and installed-platform/physical gates remain open. New resource
ceilings are explicit initial product budgets; boolean/sort/stitch spans,
unsupported inherited modes and total process memory remain limited claims.
HARD STOP for review after the two authorized logical commits.


### Final qualification receipt

Frozen-source `scripts/pre-release.sh`: **exit 0, 16/16 PASS**. Its workspace
suite reports **1654 PASS / 1 ignored**. The final TEST1 release executable was
rebuilt after the last resource/cancellation changes: **256/256 in 27.66s**.
The CLI was also rebuilt and `scripts/numsafe3-cli-check.py` passed again.
Local [receipt](../../dist-test/numsafe3-2026-10-20/receipt.json) records source
and log SHA-256 fingerprints; raw logs remain local-only. Engineering task
`ivac-fh2av`; separate human review **`ivac-zulb3` OPEN**. No later checkpoint
is started by this result.


### Commit handoff

- `a480181` — specialized resource limits, cancellation and semantic regressions.
- Transactional export, native/CLI tests and this final report are in the following
  logical commit (the commit containing this section). Exact IDs are also recorded
  in tracking and the checkpoint response; neither commit is pushed.

Human review `ivac-zulb3` is the next step. No subsequent implementation milestone
or distribution action is authorized by these software results.
