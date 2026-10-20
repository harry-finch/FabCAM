# NUMSAFE — numerical and runtime safety hardening — 2026-10-20

Tracking **ivac-knphk**. Bounded checkpoint on the working tree based on
`94cdd6b` (local, not pushed; the two import commits `d30863d` and `94cdd6b`
were authorized and created in Phase 0). Software evidence only — no machine
was run. Nothing committed, pushed, packaged or published.
Evidence: [`numsafe-2026-10-20-evidence/`](numsafe-2026-10-20-evidence/).
**HARD STOP for human review.**

## 1. Audit findings — verdicts

| ID | Audit finding | Verdict |
| --- | --- | --- |
| NS-1 | Non-finite coordinates silently become zero in native G-code | **Confirmed** (reachable through the Rust API and derived values; JSON transports cannot carry NaN/∞) |
| NS-2 | Invalid or zero cutting feeds silently ignored | **Confirmed** for the LinuxCNC/GRBL dialect posts and the bundled `grbl.cps`; **refuted** for ShopBot (its post already refuses) |
| NS-3 | STL surface grid: excessive allocation / integer overflow | **Confirmed** (reproduced as a WASM trap) |
| NS-4 | WASM panic may leave the worker unusable | **Partially confirmed**: panics trap the instance and the worker kept serving from it; an actually unusable instance was not reproduced |
| NS-5 | ShopBot output/setup assumptions | Audit only — §9 |

Found while verifying (not in the audit list):

| ID | Finding | Verdict |
| --- | --- | --- |
| NS-6 | NaN depth makes the Z schedule loop forever (process killed by the OS) | Confirmed, fixed (refused) |
| NS-6b | Finite extreme depth/step (e.g. depth −1e9 mm, step −1 mm) gives an unbounded pass count | Confirmed by code reading; **deferred** (§8) |
| NS-7 | NaN tool diameter silently becomes 0.01 mm (`effective_diameter` floor), no warning | Confirmed, fixed (refused) |
| NS-8 | Bundled `grbl.cps` renders a zero feed as `F0` | Confirmed, fixed (refused on the CPS route) |

## 2. Root causes

- **NS-1** — `gcode::fmt_num_dp` deliberately snapped NaN/∞ to `"0"` (a unit
  test pinned it). Other leaves also lost the value: profile axis formats `%d`
  (`round() as i64` → 0) and HPGL coordinates (`as i64` → 0). The SAFE-Z
  preflight only checks clearance when a stock model is declared.
- **NS-2** — the dialect post's `feedrate(0)` returned without emitting, so the
  next cut inherited whatever `F` was modal. The resolver emits the critical
  warning `zero_rate_emitted`, but only the UI blocks export, and only while the
  *Block on critical warnings* preference is on. CLI, server and streaming
  export do not check it. Feeds are `u32` throughout, so negative or NaN feeds
  can only arrive as a cast, which saturates to 0 and is covered by the same rule.
- **NS-3** — `SurfaceField::from_stl_capped` took `max_dim: u32` from any
  transport without a ceiling. `ceil() as u32` saturated, and `cols * rows`
  wrapped on 32-bit WASM: the buffer got length 1, then an out-of-bounds index
  panicked. Natively the same request asks for ~1.8e19 cells. `parse_stl`
  accepted NaN/∞ vertices: an ∞ vertex returned a silent "no footprint", and a
  NaN vertex was silently ignored.
- **NS-4** — `wasm32-unknown-unknown` aborts on panic, so the `catch_unwind` in
  `ivac-wasm` cannot catch it. The trap surfaces as `RuntimeError: unreachable`.
  The worker's `try/catch` posted it as an ordinary error and kept the same
  instance. wasm-bindgen leaves instance state undefined after a trap: the shadow
  stack, borrowed cells and the thread-local diagnostic channels that drain per
  op can all be affected. The client recycled a worker only on `onerror` or
  cancellation.
- **NS-6 / NS-7** — `max(NaN)` ignores NaN: `build_z_schedule` never reaches
  `total_depth`, and `effective_diameter` floors NaN to 0.01 mm.

## 3. Before / after reproductions

Before: [`before-probe-head-94cdd6b.txt`](numsafe-2026-10-20-evidence/before-probe-head-94cdd6b.txt)
(HEAD 94cdd6b, [probe source](numsafe-2026-10-20-evidence/before_probe.rs.txt)).
After: `crates/ivac-core/tests/numsafe.rs` (7 tests).

| Case | Before (HEAD) | After |
| --- | --- | --- |
| NaN clearance, no stock, LinuxCNC/GRBL | Program generated: `G0 Z0`, `G0 X20 Y17 Z0` (traverse at the work surface); streamed too | `RefusedOutput(0, non-finite value …)`; the stream stops before the faulty line |
| Tool feed 0 | Cuts run at `F200` (the plunge feed) + critical warning | `RefusedOutput(1, zero feed rate …)`, buffered, streamed, cached retry |
| Tool plunge 0 | Plunges run at `F1200` (the cutting feed) + critical warning | Refused |
| Op feed override 0 | Same as tool feed 0 | Refused |
| `grbl.cps` feed 0 | `G2 … F0` + critical warning | Refused before the post runs |
| ShopBot feed 0 / NaN clearance | Refused by the post | Still refused (now by the core guard first) |
| NaN tool diameter | Offset by 0.005 mm, no warning | `NonFiniteParameter(1, tool diameter = NaN)` |
| NaN depth | Unbounded loop, process killed (exploratory probe) | `NonFiniteParameter(1, depth = NaN)` |
| STL `max_dim = u32::MAX`, WASM | `RuntimeError: unreachable` ([before](numsafe-2026-10-20-evidence/wasm-trap-before.txt)) | Structured `bad_input`: grid refused ([after](numsafe-2026-10-20-evidence/wasm-grid-after.txt)) |
| STL `max_dim = 4001` (native) | 16 008 001 cells allocated | `StlError::Grid` before allocation |
| STL ∞ / NaN vertex | Silent `None` / vertex ignored | `StlError::NonFinite(facet)` |
| Worker trap, then valid request | Same instance reused | Worker terminated; the next request runs on a fresh one ([tests before](numsafe-2026-10-20-evidence/worker-recovery-tests-before.txt): 3 failing) |

## 4. Corrections and affected files

- `crates/ivac-core/src/gcode/numeric_guard.rs` (new): a thread-local record of
  the first output fault (`NonFinite`, `ZeroFeed`, `MissingFeed`), in the same
  idiom as the existing offset diagnostics.
- `gcode.rs` (`fmt_num_dp`, `fmt_dwell`), `post_profile.rs`
  (`format_axis_value`, `<d>`), `hpgl.rs`: record the fault and never
  substitute a number. The text stays visibly invalid, so a leak could not
  become a zero move.
- `linuxcnc.rs` (GRBL delegates to it): `feedrate(0)` records `ZeroFeed`;
  `G1`/`G2`/`G3` and canned drill cycles record `MissingFeed` when no feed is
  established.
- `cps_recorder.rs`: the same rules on the CPS route (zero feed, unset feed,
  non-finite XYZ, arc centre, cycle depth).
- `sink.rs`: a streaming sink stops writing once a fault is recorded.
- `pipeline.rs`: `PipelineError::RefusedOutput` and
  `PipelineError::NonFiniteParameter` (structured, `op_id`, hint, "No program
  generated"). The record is reset before emission and checked after the
  header, after every op (fresh, cached or program-only) **before the op body
  can be cached**, and after the footer. `require_finite_setup` covers depth,
  start depth, step, through depth, finish step, depth list, clearance, and the
  raw and resolved tool diameter and wear.
- `cam/surface.rs`: `MAX_SURFACE_CELLS = 16 000 000`, `try_from_mesh` sizes the
  grid in f64 and validates the cell before allocating; `from_stl` and
  `from_stl_capped` return errors. `sim/stl.rs`: `StlError::{NonFinite, Grid}`;
  `parse_stl` refuses non-finite vertices.
- Frontend `api/wasm-fatal.ts` (new), `wasm.worker.ts`,
  `wasm-worker-protocol.ts` (`fatal` flag), `wasm-worker-client.ts`, `wasm.ts`.
- Tests: `tests/numsafe.rs` (new); unit tests in `numeric_guard`, `gcode`,
  `hpgl`; updated `zero_feed_rate_is_refused` and a streaming byte-identity test
  that drove `G1` without a feed; `wasm-worker-client.test.ts` (+4),
  `wasm-fatal.test.ts` (new, 2).

No schema, project format, CAM operation, post capability or OpenSBP text
change. `zero_rate_emitted` is still emitted (for example for spindle 0).

## 5. Machine-output impact

Valid programs are unchanged. 136 programs were generated with HEAD and with
the NUMSAFE tree and compared byte for byte, all identical (SHA-256
[manifest](numsafe-2026-10-20-evidence/output-identity-sha256.txt),
[harness](numsafe-2026-10-20-evidence/output_identity_probe.rs.txt)). The
corpus is TEST1 profile and V-Carve requests plus the 16 ShopBot M1 DXF/SVG
fixtures × profile and pocket, × LinuxCNC, GRBL, HPGL and ShopBot (34 programs,
82–91 k lines per post). Existing golden, M1/M2, SAFE-Z, V-Carve and TEST1
suites also pass unchanged.

Programs that are now refused (previously generated):

1. A zero cutting feed, plunge feed or feed override on LinuxCNC, GRBL or any
   `.cps` post.
2. Any non-finite coordinate, arc, dwell or axis word (all posts).
3. A cutting move with no feed established in the program (never seen in the
   valid corpus).
4. Non-finite motion parameters or tool diameter.

## 6. Resource and recovery policies

- **STL surface grid:** at most 16 000 000 cells, the same ceiling as the raster
  op's materialized grid (`MAX_RASTER_PIXELS`). That allows `max_dim` up to
  4000 for a square mesh; the UI uses 256. Over budget is refused with a
  message, never silently reduced. The cell must be finite and above zero, and
  vertices must be finite.
- **Output faults:** the first fault wins and the whole program is refused. A
  refused op is never cached, and a stale record from an aborted run is cleared
  at the next start.
- **WASM worker:**
  - A trap (`WebAssembly.RuntimeError`) or a `RangeError` (stack overflow) is
    fatal. The worker then answers every queued request with a fatal error and
    never calls into wasm again.
  - The client terminates the worker and rejects all in-flight work with
    `WasmFatalError`. The next request starts a fresh worker. There is never an
    automatic retry, and no interrupted run is resolved as a result.
  - After 3 consecutive fatal failures without a success, calls are refused
    with "reload the page".
  - Recoverable API errors keep the worker. Cancellation, request ids and
    stale-message handling are unchanged.
  - The main-thread fallback cannot re-instantiate the module, so after a trap
    it refuses every later call explicitly.

## 7. Regression results

| Gate | Result |
| --- | --- |
| `tests/numsafe.rs` (`--features cps`) | 7/7 |
| SAFE-Z: `safe_z_extended`, `hatch_safe_z` | 15/15, 14/14 |
| ShopBot M1 / M2: `shopbot_m1_qualification`, `shopbot_m2_post` | 11/11, 27/27 |
| CPS: `cps_posts` | 5/5 |
| V-Carve: `vcarve_qualification` | 10/10 |
| `tool_provenance`, `comment_safety`, `import_correctness`, `golden_corpus` | 12/12, 1/1, 5/5, 1/1 |
| TEST1 (`scripts/test1/run.py --cases 32 --budget 120 --frontend`, release) | 256/256 in 27 s; frontend lifecycle 32/32 |
| `cargo test --workspace --all-features` (CI) | 1624 passed, 0 failed, 1 ignored |
| `cargo clippy --workspace --all-targets --all-features -D warnings` | OK |
| Output identity (§5) | 136/136 byte-identical |
| WASM Node reproduction (rebuilt pkg) | Trap replaced by a structured `bad_input` error |
| vitest (incl. 6 new worker/WASM tests) | 163 files, 1458 passed, 1 skipped |

`scripts/pre-release.sh` (Node 24, pnpm 10.34.7, cargo-deny 0.20.2), final
tree: fmt, clippy with pedantic, workspace tests (1624/0/1), schema-check,
README parity, pnpm install, codegen drift, i18n drift, lint, check, test,
build, wasm-pack, wasm size guard and cargo-deny all ✓. The UI smoke step
failed inside that run only because my `google-chrome` shim was a symlink,
which Chrome refuses to start from (`Abort trap: 6`). Rerun on the same tree
through a wrapper script on `PATH`, `scripts/ui-smoke.sh` passes, as it does
with `CHROME=<app path>`. An earlier full run failed one pedantic
`doc_markdown` lint in the new test file; it was fixed and the final run is
green.

**Not run:** Tauri desktop and Android UI, a browser run forcing a real wasm
trap (no remaining panic path is known), physical machine or controller
preview. Tauri crate tests are part of the workspace run.

## 8. Remaining safety limitations

- **NS-6b (deferred):** finite extreme depth/step combinations can still request
  a huge number of passes (CPU/memory exhaustion, no wrong motion). A pass-count
  ceiling is a new limit for a human to choose.
- A NaN `step` keeps the existing policy: any non-finite or zero step is
  "unspecified", giving one full-depth pass plus the `step_unspecified` warning.
- The guard checks parameters that feed the shared motion core. Other
  per-operation parameters (V-Carve angles, thread, waterline, …) rely on the
  output guard, which fails closed but cannot prevent a hang inside their own
  loops. Not reproduced.
- Streaming export: a fault now stops writing before the faulty line, but lines
  already written stay. The CLI `--output` file is left truncated and the
  command fails; the server aborts the stream. This is the pre-existing
  mid-emit contract.
- HPGL clamps a zero feed to `VS1` (slowest pen speed); left for the VINYL track.
- WASM recovery is covered by unit tests with a scripted worker and a Node
  reproduction of the trap. No browser run forced a trap, because the known
  panic path is now closed.

## 9. ShopBot observations (audit only, no output change)

| Observation | Classification | Notes |
| --- | --- | --- |
| Manual pauses at every section: stop spindle, load tool *N*, set work Z zero, start spindle, stop at section end | Existing qualified behaviour (M2: "Pauses cannot be disabled"); **usability improvement** | Repeated even when consecutive sections use the same tool. Asking for a Z re-zero each time adds operator steps and chances of zeroing error. Candidate UX item; needs operator review. |
| Rapid clearance vs stock top Z | Existing qualified behaviour + **controller/setup uncertainty** | Core SAFE-Z refuses clearance ≤ declared stock top (tests incl. ShopBot path). The post also requires clearance > 0 and refuses lateral rapids with Z ≤ 0, which assumes stock top at Z0; with a non-zero stock top, intermediate retract planes are covered only by the core checks. Recommended follow-up test: ShopBot + stock top ≠ 0 + drill retract. |
| `VD,,1` | Documented, **controller-specific uncertainty** | Repo sources (OPEN_SBP_COMMANDS.md, M2_QUALIFICATION.md): sets XYZ units to mm and persists converted controller settings, no automatic restoration. Actual controller **TO VERIFY ON MACHINE**. |
| Operator instructions | Existing qualified behaviour; usability | Prompts cover spindle stop/start, RPM, tool, Z zero, clearance and jog speeds. They do not say when the tool is unchanged. |
| Zero/non-finite feed and XYZ | **Existing qualified fail-closed behaviour** | `shopbot.cps` already refuses them; NUMSAFE now refuses earlier in the core, and valid ShopBot output is byte-identical. |

No ShopBot safety defect was confirmed. SAFE-Z, M1/M2 and the post's
restrictions are unchanged.

## 10. Human acceptance decisions

1. Accept refusing zero cutting or plunge feeds on every post and route, instead
   of a critical warning plus an inherited modal feed.
2. Accept the "feed must be established before a cutting move" rule.
3. Accept the 16 M-cell STL surface limit (refuses `max_dim` > 4000 for square
   meshes) and the refusal of non-finite STL vertices.
4. Accept the WASM recovery policy (recycle on trap, 3 consecutive failures,
   then reload).
5. Decide on NS-6b (pass-count ceiling) and the ShopBot usability items
   (repeated pauses, same-tool Z re-zero).
6. Authorize (or not) committing NUMSAFE; nothing is committed or pushed.
