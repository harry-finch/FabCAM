# FS-RESET-3 bounded engineering receipts

Branch `FabCAM-v1.1`; engineering tracker `ivac-wjymz`. Full removal was authorized by
the maintainer on 2026-10-22. These are incremental receipts, not final acceptance.
Starting HEAD: `951b12f4477f64fd5e9c229b693748f23e794a0e`.
CLI Option A `030e2149f5bcb7f356711face54caafcf6155843` is an ancestor.

## A — source inventory and fixed comparisons

The starting scan covers 3,348 tracked files, 258 operational files and 4,385 exact
reference lines. `start-consumers.csv` uses REMOVE/MIGRATE/KEEP/TEST-ONLY dispositions;
`start-historical-references.csv` protects 337 historical files. The scanner also
includes whole-module candidates and indirect gate scripts; lexical matches are not
wholesale deletion authority. Mixed files require symbol-level review.

All existing untracked evidence, two pre-existing tracked documentation edits, other
branches/worktrees and application-data namespaces are preserved. Complete initial
status, patch, worktrees/branch refs and tracked-content hashes are in the local
`dist-test/fs-reset3/start-*` files. No production data was read or erased.

Commit `a5907ac4ae284feb44c9984d1c3cc17ade524c88` freezes 23 explicit v9 cases generated
at starting HEAD. `tests/fixtures/fs-reset3-output` covers native LinuxCNC/GRBL,
ShopBot Profile/V-Carve, all supported milling role families and both sides. The new
integration test compares exact program bytes, every toolpath field, provenance,
line bindings and statistics. Both expected and actual JSON pass through the same
serializer/parser representation; this avoids a one-ULP serde Value round-trip
artifact. There is no numerical tolerance or executable-program normalization.

## B — neutral generation dependency

All pipeline requests now require explicit v9 values, including internal consumers.
`cutting_parameters` no longer delegates absent-version validation, saved rates or
machine limits to `feeds_speeds`. Setup resolution no longer reads recommendation
metadata; PlanarSurface no longer requires a recommendation origin. Machine limits
and neutral `ToolEntry.max_rpm` remain authoritative. General setup representation,
SAFE-Z, work origin, numeric budgets, provenance and post security remain in place.

There is no live milling-rate, contour-step or drill-peck default at generation.
Creation-time seeding is unchanged. Existing non-milling process setup stays separate.
Unknown limits remain disclosed; specialized mandatory restrictions remain mandatory.
A pocket without a finish wall ring does not resolve its unused secondary tool.
Automatic dowel injection stays refused for v9; explicit Drill roles remain supported.

The test-only `tests/support/explicit.rs` authors old geometric fixture constructors
into v9 before the pipeline call. It does not participate in any production adapter,
never changes already-authored v9 or invalid-version requests and never modifies
fixed output references. Test/probe serialization records the authored request.

`test-replacements.json` records each retired/replaced family. Invalid inputs that
previously warned or clamped now assert explicit refusal. The independent simulation
volume/stock-floor thresholds and NUMSAFE pass budgets are retained. Analytic volume
fixtures author the same formerly effective geometry (4 mm chamfer width, identical
V-bit cone, explicit height-grid range). The already-approved v9 per-op-cache bypass
is asserted directly; exact tee cap/overflow/recovery, CPS cache invalidation and
cancellation remain independently covered.

Three obsolete applied-recommendation generation tests are retired; immutable output
comparisons plus v9 saved-role/provenance regressions own their useful responsibilities.
The old engine, transports and disconnected frontend remain temporarily for phases
C–E. No claim of dependency closure or final release qualification is made here.

### B qualification

- Core with all features, all normal test binaries: **1,467 PASS, 0 FAIL, 1 ignored**
  (`core-b-all-features.log`). The ignored TEST1 worker must run through its watchdog
  in final qualification, including C2; it is not waived or counted as qualified here.
- Subsequent header-only/default cleanup: **1,255 unit PASS**, immutable 23-case corpus
  **1 integration PASS**, NUMSAFE2 **9 PASS** (`core-b-header2.log`). The additional
  regression replacing live tool defaults with zero/NaN while retaining explicit
  operation rates and depth list passes (`no-live-defaults-b.log`).
- `cargo test --offline -p ivac-cli`: **13 PASS**. Actual SEC1 and NUMSAFE3 CLI probes
  pass, retaining external CPS refusal, pass-budget refusal and publication assertions.
- Core strict pedantic Clippy with the release gate's documented allows and rustfmt
  pass. Small inherited test-style lint findings were fixed; no lint was disabled.
- No workspace-wide, frontend, WASM, installed-native, physical or final-release
  acceptance is claimed by phase B. Later source qualification remains mandatory.

Every fixed pre-removal program, toolpath and provenance comparison passes unchanged.
Header-only milling setup carries zero (absent) rate metadata when there is no cutting
operation; it no longer reads a library's unrelated rates or spindle minimum. No
machining operation is seeded or clamped by that path. The retained actual-cut corpus
is byte-identical. Legacy fixtures requiring invalid clamping were rewritten as
refusals; positive volume fixtures explicitly author the same effective geometry.
