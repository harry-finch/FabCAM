# CPS, HTTP and desktop contracts — audits 15–16

2026-10-18 · Engineering ivac-q1hjs · Human review **ivac-2aow8 OPEN**.
Incoming: `2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b` plus uncommitted audits
09–14 (pipeline57). Existing changes and evidence retained. Internal pipeline
version **58**, public version **0.9.0-beta.2**. No commit, push or publication.

## Findings and disposition

| Item | Verified result |
| --- | --- |
| 15.1 HTTP CPS path access | Confirmed on SSE. Both streaming routes now reject Path selections before worker/channel creation. Tests exercise all five generation routes (ordinary, two-sided, SSE, raw G-code and edited drawing), requiring the explicit CLI-only error. Raw G-code already rejected CPS downstream; this makes the boundary uniform before HTTP 200. No claim of HTTP SSRF: the defect was server-local path access. |
| 15.1 Async worker blocking | Confirmed on ordinary and two-sided generation. Both now use spawn_blocking; join failures become structured internal errors. SSE and drawing already used blocking workers. |
| 15.1 Execution budgets | Inspection is intentionally capped at 20M loop iterations / 256 recursion; generation already has finite 100M / 512 bounds plus the existing SSE cancellation path. Preserve this generation policy rather than silently lowering limits for previously supported large jobs. These are per-loop/recursion bounds, not a wall-clock deadline or admission-control guarantee. |
| 15.2 Import temp files | Confirmed leak. Import now uses import_bytes inside a blocking task, with a DXF/SVG allowlist for format hints. No temporary file or generated filesystem path. Original upload filename retained in the response. STL was not supported by this route before; it retains its separate raster endpoint. The report's arbitrary path traversal depends on intermediate directories existing; removing disk access avoids the mechanism entirely. |
| 15.3 CPS footer capture | Confirmed. Bounded footer locations map to the user's final source line. A leading semicolon separates the generated footer so a dangling `var x =` cannot absorb the export assignment and appear valid. Unclosed braces/comments and incomplete expressions are regressions. |
| 15.4 Error source mapping | Confirmed. Recognize `at line N` positions, not arbitrary `line` substrings within `polyline` or user prose. Remap all position occurrences in one pass; attribution uses the last compiler position. Prelude attribution preserved. |
| 15.5 Emit fragments | Contiguous text buffer plus integer chunk boundaries replaces retained per-fragment strings. Engine.output still reconstructs exact raw chunks, including partial lines; sink concat/clear semantics retained for differential tests. JS-to-Rust conversion still creates a temporary string; no unmeasured “half the memory” claim. |
| 16.1 Sorbonne revision order | **Not reproduced on valid input.** Source-pack validation already requires exactly 16 decimal digits for this specific private pack. Thus lexicographic order and filename order agree. Tests reject `2`, `10`, `v1`, `v1-a`, `2026.10`, and accept the padded 9→10 transition. Natural ordering was investigated and not retained. No migration, renaming or deletion of existing packs. |
| 16.2 Desktop drawing errors | Confirmed. Evaluate errors now serialize bad_input; generation retains project context, code/params/recovery/auto-fix, catches panic and maps worker join failures. Test proves missing-tool retains AssignTool suggestion 7 for operation 3. |
| 16.2 Desktop post inspection | Confirmed synchronous inspection. Listing and inspection now run on blocking workers with 20M / 256 inspection limits. No credential or post capability changes. |
| 16.3 Source watcher | Confirmed. Deduplicate targets and parent roots; retain missing-file targets when their parent exists (delete/recreate saves), skip missing parents, avoid watching directories as file targets. Failed watch setup retains the old watcher; explicit empty set still clears it. |
| 16.4 WASM changed-tool warning cache | Already addressed in lot13–14 by core fingerprints. Added bridge regression: reset, change cutter diameter, advance the same segment index; warnings are recomputed. No redundant invalidation layer, and pure backward scrubbing retains cache reuse. |
| 16.5 Linux helper identity | Confirmed permissive substring match. Only argv[0]'s exact basename can identify WebKitWebProcess, WebKitNetworkProcess or WebKitGPUProcess. Existing parent-PID filter remains. Pure predicate tested on macOS; actual Linux exit/process behaviour remains a platform test. |

## Architecture, compatibility and UX

Changes stay in transport adapters and the generic CPS runtime. No machine-specific
patch in geometry, no global Engrave/V-bit permission, no ShopBot guard weakened.
Project schema, app identity/data paths and source-pack validation are unchanged.
Async command names and JSON success responses remain stable. Drawing failures
now use the existing frontend structured-error contract; no new UI layout.
SSE path refusal is HTTP 400 before a cancellation token or event stream exists.

The CPS sink's internal concrete type changes from Vec<String> to EmitBuffer;
repository consumers use preserved output()/concat()/clear() access. This is a
Rust implementation change, not an IPC or persisted schema change. Pipeline58
invalidates older caches after the CPS bundling change; the stable-key fixture
was updated for that explicit version bump only.

## Validation

Raw logs, exact commands and source fingerprints:
[checks.json](transport-desktop-audit-2026-10-18-evidence/checks.json).
Before correction: two CPS attribution/EOF tests and the HTTP path-route test
failed. These failures are retained as baseline evidence.

- Core: **1,202 tests PASS**, serial (existing parallel-cache isolation issue unchanged).
- CPS: **46 reported PASS, 40 executed**; six optional FANUC-oracle cases return
  early because the external oracle is absent. Node/V8 differential harness:
  **39 PASS**, actually executed.
- HTTP: **15 PASS**, including multipart format hints and all path-refusal routes.
- WASM bridge: **22 native Rust tests PASS**, including reset/changed-tool caching.
- ShopBot/CPS/V-Carve integration: **41 reported PASS, 40 executed** (one optional
  FANUC case absent). Existing output goldens unchanged by this lot.
- Tauri: **12 PASS**. Clippy (all targets for CPS/server/Tauri), formatting and
  diff checks PASS. Native watcher event
  tests require execution outside the restricted sandbox; sandbox timeouts are
  retained and distinguished from the successful native runs.

No fresh packaged app, clean installation, browser UI review, Windows/Ubuntu
runtime or physical machining qualification is claimed.

## Audit16 cross-cutting summary versus current source

The report's Top10 mixes historical findings with current work. Tabs use the
user-approved versioned width/conversion from lot11–12; the legacy path is
intentionally retained. Entry feeds, open-path exits, curved areas/entries,
waterline bounds/registration, drawing topology, DXF/NURBS and dexel arcs/STL
are covered by their earlier reports and tests. The ShopBot MS recommendation
was already reviewed in lot7; it does not authorize expanding the audited
post subset here. Streaming two-sided/fixture guards were fixed in lot14;
HTTP source rejection and disk-free imports are fixed in this lot. This audit
summary does not itself validate the physical or platform gates.

## HARD STOP / handoff

Human review **ivac-2aow8 OPEN**. Prior reviews ivac-7y9ft, ivac-80ewz and
ivac-97zve remain open; receipt of later audits does not imply acceptance.
VINYL ivac-wgovg remains explicitly deferred. Desktop candidates from
run39053047404 remain pipeline54 and do not contain these local changes.
Clean macOS installation, Windows/Ubuntu and physical qualification remain
open. No tag, package, commit, push or release was produced.
