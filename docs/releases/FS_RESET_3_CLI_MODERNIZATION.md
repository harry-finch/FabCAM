# FS-RESET-3 — CLI Option A and removal preflight

Date: 2026-10-22. Branch: `FabCAM-v1.1`. Engineering: `ivac-89fca`.
Approved integrated source: `d6b08fe82011c54d0166a6559b329804c2e56cee`.
Starting HEAD: `10b04fce6eafb75eca65fe8683532bdef9d840fa` (two subsequent documentation commits).
**Qualified implementation commit: `030e2149f5bcb7f356711face54caafcf6155843`.**
The separate documentation commit records this receipt and refreshed inventory; it changes
no production source. User approval authorizes CLI Option A and preflight only.

**CLI phase complete; HARD STOP before mass removal.** The public default-rate bypass is
removed. The remaining legacy engine is still present, with useful dependencies and tests
to migrate. This is readiness for the next human decision, not completed FS-RESET-3 removal,
release acceptance or authorization to continue deleting code.

References: [approved Option A](FS_RESET_3_INTEGRATION_READINESS.md),
[ADR-0030](../shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md),
[CLI contract](../CLI_GENERATE.md), [earlier acceptance](FS_RESET_2_FINAL_ACCEPTANCE.md).

## Implemented contract and boundaries

`ivac generate --request request.json` reads a serialized v9 PipelineRequest, requires the
current contract and calls the existing buffered pipeline. That pipeline owns the neutral
role/limit validator, geometry setup, resource budgets, SAFE-Z, origin, provenance and
simulation qualification. No new rate model or calculator, no seeding during generation.

`ivac generate geometry.dxf --request template.json` is bounded convenience: no existing
segments/text/relief and exactly one enabled Profile with source All. Explicit machine,
tool ID, source, depth, step and SAFE-Z fields are required, as are the actual-role rates.
The reviewed request supplies all machining intent; import only supplies segments. Object
and layer selections are refused rather than guessed. Other supported operations use the
canonical request form. Generic resolved `Setup` is retained in core.

All old geometry/post flags, duplicate request flags, unknown top-level request fields,
duplicate serialized post fields and non-CPS/cps_post conflicts refuse. Missing/malformed
v9 requests, role mismatch/duplication, nonpositive/fractional/overflow values and known
machine/tool limits fail before stdout. Profile Finish values determine contour output;
Rough stays stored independently. Unknown ratings remain warnings, never invented limits.
The shared request's omitted-post LinuxCNC default is retained and documented; it does not
provide cutting values. CPS source/properties have only one authority, the JSON request.

Successful stdout is the buffered PipelineResponse. Existing `gcode`, `toolpath`, `stats`
remain, with provenance, warnings, gcode index, timing and diagnostic fields added. Counts
are now pipeline counts. The former always-zero distance placeholders are removed; preview
is the common pipeline preview. Tracing/diagnostics go to stderr. Serialization completes
before stdout writing, but shell redirection/output I/O is not transactional publication.

`stream-gcode`, native staged publishing and two-sided partial-publication handling are
unchanged. `generate` adds no two-sided switch. Automatic dowel injection remains unsupported;
explicit Drill operations remain the registration preparation. No ShopBot command or supported
process/post combination was added. External CPS inspection/execution, including exact bundled
copies, still fails before evaluation. SEC1 now reaches that guard with valid explicit input.
NUMSAFE3 now uses the same v9 fixture and retains every previous assertion.

No shared Rust model, OpenAPI schema, generated frontend type or WASM ABI changed. Frontend,
MAT-1, tested collections, v9 persistence, tools/machines, Tauri ACL and core validators are
byte-unchanged from the approved integrated source. Full frontend/schema/native rebuilding
was not repeated for a CLI-only change; real native acceptance is not inferred from this.

## Qualification on the implementation source

Offline Rust, `RUSTC_WRAPPER=''`; commands run from repository root. CLI was rebuilt with
default CPS features after testing the no-CPS configuration.

| Command | Result / proof |
|---|---|
| `cargo test --offline -p ivac-cli` | 13 PASS: 6 existing locale, 1 existing transactional streaming, 6 new CLI process tests |
| `cargo test --offline -p ivac-cli --no-default-features` | 12 PASS; external-CPS execution test correctly feature-gated, native posts still exercised |
| `cargo clippy --offline -p ivac-cli --all-targets -- -D warnings` | PASS |
| `cargo fmt -p ivac-cli -- --check` | PASS |
| `cargo test --offline -p ivac-core --features cps cutting_` | 25 PASS across filtered suites; role/limit, dual Pocket, shared/distinct Drill, two-sided bytes/provenance/refusal and established ShopBot comparisons |
| `cargo test --offline -p ivac-file-export` | 11 PASS; includes first-side preservation when second publication fails |
| `cargo build --offline -p ivac-cli` | PASS |
| `python3 scripts/sec1-cli-check.py` | PASS; malicious/renamed/exact-copy external inspection and generation reach External CPS guard with empty stdout; bundled listing succeeds |
| `python3 scripts/numsafe3-cli-check.py` | PASS; intended 1000-pass budget refusal, target preservation, cleanup, byte identity, new-only pairs and both-side staging |
| Scoped source whitespace, preserved patch and retained-tree checks | PASS |

[Logs and hashes](fs-reset-3-cli-evidence/SHA256SUMS.json).
The new process tests compare native LinuxCNC/GRBL/HPGL and bundled ShopBot output exactly
against the authoritative buffered pipeline, including provenance; compare template/canonical
responses; exercise actual distinct Profile rates, invalid inputs, missing fields and JSON
purity; and verify external CPS never executes. Test aggregate counts are not a claim that
all possible operation combinations were exercised through the CLI.

Initial development checks exposed a temporary-directory name collision in the new parallel
tests (fixed with a per-process atomic counter), a Rust MSRV lint (replaced the newer helper
with the supported equivalent), and the historical golden's two missing provenance comments
(described below). No existing test was weakened, skipped or disabled. Final logs are the
qualified runs. No full release gate or complete TEST1/C2 corpus is claimed at this checkpoint.

## Output comparisons and intentional differences

1. For unchanged explicit requests, CLI output equals `run_pipeline` bytes and provenance
   for LinuxCNC, GRBL, HPGL and bundled ShopBot. Template import equals a canonical request
   containing those same imported segments. The implementation does not alter core output.
2. Established `tests/fixtures/test1/profile.sbp`: exact equality after accounting for exactly
   one REQUEST and one ACTIVE `FABTOOL/1` comment. Those two comments predate this CLI work;
   the test checks their exact text/count and every other byte. Existing core Profile/VCarve
   comparisons also pass. No executable-command difference is waived in that comparison.
3. Old public geometry CLI vs new explicit template: original CLI source at the approved SHA
   was compiled as a disposable diagnostic crate against unchanged core/CPS/export source.
   This is not a second complete workspace baseline build. Shared lockfile versions were
   reused. [Per-post before/after JSON, diffs and receipt](fs-reset-3-cli-evidence/comparison/receipt.json)
   retain all outputs for the rectangle fixture with explicitly matched diameter/rates/depth.

| Post / old geometry route | Investigated difference |
|---|---|
| Native LinuxCNC/GRBL | New common setup adds tool provenance, tool/spindle transitions and explicit clearance in the approach. Profile uses Finish and the common conventional outside-contour winding (`offset_builder` → `apply_cut_direction`); old direct offset emission skipped this winding step. G3 CCW becomes G2 CW with the same offset contour/depth/rates. These are deliberate pipeline adoption, not an attempt to preserve the bypass. |
| HPGL | Same exact multiset of emitted commands, now ordered for the conventional contour traversal; comparison asserts that multiset equality. No new HPGL capability inferred. |
| Bundled GRBL CPS | Both use the pipeline; the new reviewed template names the operation and authors SAFE-Z 5 rather than the old auto-created operation's 0. Approach/retract and explicit surface entry reflect that input difference. Machining depth/rates and contour remain matched. |
| Bundled ShopBot, old implicit convenience | Old CLI fails `positive authored clearance Z required`. This is retained as an old-path exclusion, not relaxed. The current explicit ShopBot fixture supplies SAFE-Z and matches the established reference as above. |

Before/after native programs are intentionally not byte-identical. The authority for future
output is the reviewed v9 request and common pipeline, not the removed direct-emitter path.
The software fixtures are not workshop-tested configurations and must never be imported as
physical attestation. No output was sent to hardware.

## Refreshed dependency inventory and test ownership

[Exact lines](fs-reset-3-cli-evidence/legacy-references.csv),
[files/dispositions/hashes](fs-reset-3-cli-evidence/legacy-files.csv),
[receipt/counts](fs-reset-3-cli-evidence/inventory-receipt.json),
[reproduction script](fs-reset-3-cli-evidence/inventory.py), and
[protected historical files](fs-reset-3-cli-evidence/historical-references.csv)
are bound to implementation SHA `030e2149f5bcb7f356711face54caafcf6155843`.
The refreshed scan covers 3,311 tracked files: 258 operational files / 4,385 reference
rows, plus 329 protected historical files. No unresolved CLI HUMAN DECISION rows remain.
The inventory remains conservative lexical/module/indirect-gate closure, not a bulk deletion
manifest or proof of arbitrary dynamic reachability. Prior inventories remain unchanged.

The CLI Option A HUMAN DECISION is resolved: CLI source, its explicit fixtures and migrated
SEC1/NUMSAFE3 gates are now KEEP. Report rate consumers were already repaired in the approved
baseline; their old blocker is not carried forward as unresolved. Remaining boundaries:

| Family | Required migration / replacement coverage before deletion |
|---|---|
| Core `cutting_parameters` absent-version delegates; `feeds_speeds` saved rates/effective machine/validation | Port retained internal tests/benches to explicit roles; retain positive/negative bounds, no defaults/clamping, unknown-limit disclosure and specialized mandatory restrictions |
| Project material identity, tool/machine metadata and setup resolver legacy slots/origins | Extract neutral identity vocabulary; preserve exact physical IDs, actual secondary tools, generic Setup, geometry, SAFE-Z, fixtures and origin; remove only legacy metadata/delegates |
| Active frontend project/settings/tool/identity/manual-edit helper imports | Migrate retained generic behavior before deleting old helper/component closures; preserve Sol UX, MAT-1, loaded/modified state, review/copy, Undo/Redo and stale generation |
| HTTP/Tauri/WASM endpoints, source-pack storage registration, generated bindings | Detach obsolete APIs coherently; retain v9/preset models and current transport checks; regenerate schema/client exports, no manual type erasure |
| Source data/Sienci assets, old libraries/trials/ranking UI and translations | Remove shipped obsolete behavior only after zero retained consumers; keep reused material labels, licenses/history and all user-owned files |
| `ivac-feeds-speeds` and workspace dependencies | Remove last, after neutral Identity/material vocabulary and useful tests migrate; no replacement calculator |
| Other TEST1/perf/NUMSAFE/CI/build fingerprints | Audit every request producer for v9 and intended-guard reachability; preserve final full gate, TEST1 including C2, corpus, cancellation/resource/serialization assertions and WASM freshness |

The [accepted readiness safety matrix](FS_RESET_3_INTEGRATION_READINESS.md#5-safety-invariant-ownership-and-replacement-coverage)
and [test-family migration ledger](FS_RESET_2_FINAL_ACCEPTANCE.md#test-migration-and-intentional-exclusions)
remain prerequisites. This phase adds the CLI process/explicit-input replacement coverage;
it does not authorize deletion of old tests that also exercise a generic safety invariant.
Only recommendation math/ranking/adaptation, obsolete source/trial lifecycle and v1–8
compatibility expectations are candidates for intentional exclusion after that ledger is
resolved. Geometry, output, safety, provenance, budgets and export assertions remain.

## Final proposed removal sequence — not executed

1. Refresh inventory against the next approved source. Migrate remaining internal old-contract
   test/benchmark request construction and extract neutral identity/validation vocabulary.
   Keep old modules until each retained invariant has explicit replacement assertions.
2. Detach active frontend settings, identity and manual-edit dependencies; retain accepted
   inspectors, tested collections, generic libraries, MAT-1, persistence/history and stale
   generation. Verify imports and coverage before removing dead UI/translations.
3. Remove obsolete transport/storage registrations and distributed assets only after callers
   detach. Regenerate OpenAPI/TS/WASM exports together. Never erase user application data.
4. Remove old setup metadata/fallback delegates and FS crate once useful references are zero;
   retain Setup, drivers, all safety/provenance/export logic and independently useful tests.
5. Qualify the final source: workspace/all-features, strict Clippy, frontend/typecheck/lint,
   generated models, fresh WASM/build, NUMSAFE/SAFE-Z/security/provenance/import/export and
   complete TEST1 including C2/output corpus. Investigate every unexplained output difference.
6. Deliver source-bound FS-RESET-3 removal report and stop for final human acceptance.

Each step requires focused qualified local commits and preservation of prior evidence.
No push, main merge, rebase, release, installer, FabSpeed integration or hardware operation.

## Remaining gates and exact next action

The watcher exception remains: the same two failures occurred before/after FS2 under equivalent
sandbox conditions; installed/native-host event delivery is still unqualified. No watcher
change, timeout increase or disabled test here. Static ACL and mocked file tests remain
insufficient for installed-native collection transfer acceptance. Browser/native collection
round-trip still follows its documented manual protocol; no new transfer success is claimed.
Automatic double-sided dowel injection remains excluded, with explicit Drill preparation.

**Next action is human review of this CLI contract, output-difference ledger and dependency
plan, followed by explicit authorization for the bounded removal sequence.** The earlier
integrated-baseline/CLI approval is recorded but does not close the mass-removal gate.
The complete final-source gate and installed/physical acceptance remain separate later work.

[Preservation receipt](fs-reset-3-cli-evidence/preservation.json): pending STATUS/ROADMAP edits
remain outside the commits; other worktrees, main, retained application code and historical
reports were preserved. No application-data reset or private pack access occurred.
