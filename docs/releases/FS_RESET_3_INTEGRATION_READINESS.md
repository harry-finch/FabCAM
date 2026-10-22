# FS-RESET-3 integration readiness — decision checkpoint

Date: **2026-10-22, Europe/Paris**. Engineering `ivac-804tb`.
**Integrated source: `d6b08fe82011c54d0166a6559b329804c2e56cee`**, branch `FabCAM-v1.1`.
**CONDITIONAL GO for planning; HARD STOP before CLI changes or FS-RESET-3 removal.**
The two approved blocking fixes are integrated and software-qualified. The public CLI
contract below still requires approval. Human gate `ivac-y25vq`, installed-platform,
controller and physical acceptance remain separate and open.

References: [architecture](FS_RESET_ARCHITECTURE.md), [implementation](FS_RESET_2_IMPLEMENTATION.md),
[ADR-0030](../shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md),
[original acceptance plus repair receipt](FS_RESET_2_FINAL_ACCEPTANCE.md),
[Sol UX receipt](FS_RESET_2_UX_REFINEMENT.md).
ADR-0030 supersedes the previous integrated recommendation policy. This checkpoint
changes no production behavior, API, tests, translations or machine output.

## 1. Git integration and ownership

Before integration, `FabCAM-v1.1` was `76cf19e7cc799e8002e62cdab5dc144785515fe7`.
The repair branch was `d6b08fe82011c54d0166a6559b329804c2e56cee`; ancestry verified that
it already contains Sol's `76cf19e` UX commit and all four implementation slices.
A local `git merge --ff-only codex/fs-reset-2-repairs` preserved the exact commits:

| Commit | Scope |
|---|---|
| `81382100b56b1540833532bd451546bd3c493288` | Actual-role v9 machining report and tests |
| `496c46f0e60472b36c529768d0d3974987615fd0` | Minimal native stat permission, ACL check and import failure test |
| `d6b08fe82011c54d0166a6559b329804c2e56cee` | Repair qualification report and preserved evidence |

No cherry-pick, rebase, history rewrite or new merge commit. Only this readiness report
and new evidence are committed afterward; the report's documentation commit hash is in
its handoff. The tested source remains the SHA above, not an earlier successful build.

Initial tracked changes were limited to STATUS and RELEASE_ROADMAP. Their entire pending
patch was byte-compared before/after fast-forward and remains untouched and unstaged.
Existing untracked evidence was inventoried and preserved; no evidence or application data
was deleted. Sol's chat was idle; no other active editing task was identified in the
inspected app inventory. UX-owned files were not edited.

Local `main`, remote-tracking `origin/main`, and the live remote `refs/heads/main` (read via
`git ls-remote`) all remain `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca`.
Remote identity: `https://github.com/harry-finch/FabCAM.git`; upstream unchanged.
Other worktrees remain: docs consolidation `7e1c24b`, V1 freeze `85a93c4`, repairs `d6b08fe`.
No push, main merge, installer, version change, promotion, publication or hardware use.
[Integration/hash receipt](fs-reset-3-readiness-evidence/integration-receipt.json).

## 2. Repairs and Sol UX reconciliation

Report formatter matches exact role **and tool ID**, never live legacy defaults. Profile
uses Finish; Pocket Rough/Finish retains distinct tools; Halfpipe has only Rough;
Drill/Spot/Chamfer use their assigned roles/tools. Missing, invalid, duplicated or wrongly
bound slots display dashes. Screen and Markdown use the same projection. The five report
regressions rerun here cover edited values and the real v9 snapshot/restore path as well
as export serialization. The previous actual browser Save Markdown comparison and screenshot
remain [accepted repair evidence](fs-reset-repairs-evidence/exported-report.md); they are
not relabeled as a new browser download at this integration checkpoint.

`fs:allow-stat` remains the only capability delta from the UX baseline. No path glob,
dialog bypass, read-size bound or storage code changes. `cargo check -p ivac-tauri`
regenerated manifests in the integrated checkout; the generated capability contains stat.
The ACL script resolves actual dependency permission sets, proves the old set lacks stat,
and proves the added permission grants only stat with no configured scope. The prior
HOME read scope is unchanged, not newly granted. Manifest hashes are in the receipt.

The following files are byte-identical to Sol's reviewed `76cf19e`:
`TestedPresetsLibrary.svelte`, `TestedPresetEditor.svelte`, `CuttingParameters.svelte`,
`cutting-ui.ts`, `cutting-workflow.ts`. App routes the library to TestedPresetsLibrary;
OpPropertiesPanel and PlanarSurfaceSection route to CuttingParameters. Source inspection
confirms compact empty/populated states, management target, exact role editing, grouped
unknown-limit warnings, Load/Save review, attestation, pinned target, and loaded/modified
state are preserved. `cutting-ui`, `cutting-workflow`, `preset-library-v9`, project-v9 and
real-WASM tests pass in the integrated full suite. No new UX redesign or visual acceptance
is claimed; Sol's visual evidence remains valid for the unchanged component bytes.

Active legacy dependencies still exist outside these new components: project-operation-ops
imports `preserveManualCutting`; project-data imports source settings, and project/file-load
normalization retains FsContext/reference identity handling. Old component trees also remain
tracked. ReportDialog's old rate-helper import is now gone. These are FS-RESET-3 migration
items, not justification to declare every old UI module unreachable or delete it blindly.
[Active UI edge scan](fs-reset-3-readiness-evidence/active-ui-edges.txt).

## 3. CLI decision — public command, not just an internal test path

### Observed contract

`crates/ivac-cli/src/main.rs::cmd_generate` exposes **`ivac generate <geometry>`** through
CLI help, `docs/BUILDING.md:447`, and `docs/CPS_POSTS.md`. It accepts importer-supported
geometry, post selection, diameter/depth/step/offset/overcut and CPS selection/properties.
It emits JSON with `gcode`, interpreted/recorded `toolpath`, and geometry stats to stdout.
This public, documented surface corrects the earlier inventory's shorthand “internal CLI”.
No evidence supports declaring it unused; SEC1's real CLI probe also exercises it.

For LinuxCNC/GRBL/HPGL, cmd_generate → build_offsets → `Setup::default()` → emit_polylines
bypasses the project-v9 neutral validator and setup resolver entirely. ToolConfig defaults
include 18000 RPM, horizontal 800 and vertical 100 mm/min. A real integrated CLI invocation
with no supplied rates emitted S18000/F800/F100; [counterexample](fs-reset-3-readiness-evidence/cli-default-counterexample.json).
This is direct legacy default semantics, **not a call to the recommendation calculator**.
Deleting the FS crate alone would not repair this path. Setup itself remains a legitimate
internal resolved CAM representation and must not be deleted with the CLI convenience path.

For CPS, generate_cps builds a one-Profile Project v9 and calls run_pipeline, but initializes
cutting via `cutting_parameters::seed` from ToolEntry::default: the user still supplies no
rates. External CPS refusal remains in force. The old --post-file help/examples do not
constitute authorization to execute external scripts; revise current documentation during
the approved future CLI change, retaining historical security receipts.

`stream-gcode <project.json>` already reads a serialized **PipelineRequest**, requires v9,
and invokes current core generation. It is not the frontend ProjectFile envelope. It retains
bounded native streaming, staged file publication, two-sided handling and refusal rules.
This is an existing useful explicit-input route, but not an exact replacement for buffered
JSON preview or geometry convenience. `run_pipeline` already supports the four post kinds.
The current real CLI test compares streamed native output with buffered output and verifies
old/missing roles preserve an existing destination. It passed again here.

### Options and recommendation

| Option | Scope / cost | Risk / effect on removal |
|---|---|---|
| **A — modernize (recommended)** | Moderate: parser/request assembly, one buffered pipeline path, current docs and dedicated CLI regressions. Reuse PipelineRequest, require_current, validate_project and run_pipeline. No new calculator or rate model. | Preserves useful import/generation/JSON workflows; removes implicit rates and duplicated offset/emission path. Deliberate CLI input-contract change; legacy direct-emitter output may differ and must be compared/classified. |
| B — retire geometry generation explicitly | Smaller code change, but remove documented `generate <geometry>` and its CPS shortcut/help/examples; preserve import, posts and stream-gcode. | Loses buffered JSON preview and one-shot geometry convenience, including supported documented use. No evidence this loss is necessary; not recommended merely to simplify deletion. |

**Proposed A contract for approval, not implemented:**

- Canonical buffered entry `ivac generate --request <PipelineRequest.json>` consumes explicit
  v9 operations/tools/machine and returns buffered pipeline output with warnings, preserving
  the existing gcode/toolpath/stats fields (document any additive fields/stat semantics).
- Retain geometry convenience as `ivac generate <geometry> --request <template.json>` in a
  bounded first slice: require empty input segments and exactly one enabled Profile with
  explicit assigned tool, required roles, depth/schedule and machine context. Import segments
  into that reviewed request; reject pre-existing geometry/source selections that cannot
  resolve to the imported geometry. No auto-created operation or rate seeding.
- No-request geometry generation fails with an actionable message pointing to the request
  form. Reject conflicting old geometry flags instead of silently overriding the request.
  Post/property selection must have one documented authority; prefer request fields, reject
  duplicate CLI/request declarations rather than infer precedence.
- All paths call require_current and the authoritative neutral validator before output.
  Missing/nonpositive/fractional/overflow rates, mismatched/duplicate roles/tools, wrong
  version, known-limit violations and unsupported tool/process/post cases refuse; no clamps.
  Unknown ratings remain explicitly disclosed. Preserve CPS bundled-only enforcement.
- Do not widen HPGL/CPS/ShopBot or two-sided support to retain old behavior. Compare the
  supported post cases independently; explicit differences/refusals belong in an exclusion
  ledger, not patched geometry or relaxed tests.
- Leave import, posts, stream-gcode, transactional publication and streaming limitations
  intact. No implicit conversion of saved ProjectFile documents, old requests or source packs.

Required tests: successful explicit Profile with distinct stored Rough/effective Finish;
canonical vs geometry-template pipeline equivalence; old/no-request refusal; malformed input;
all numeric/role/tool/known-limit negatives; no output before failed validation; bundled CPS
and malicious/exact-copy external CPS refusal; GRBL/LinuxCNC/HPGL supported-case comparisons;
input JSON/stdout/stderr contract; unchanged streaming and per-file/two-sided failure behavior.
Update `scripts/sec1-cli-check.py` so it still reaches the external-script guard with otherwise
valid v9 input, rather than accidentally passing on an earlier missing-request refusal.
Keep old request/output evidence; do not weaken assertions to accommodate removal.

## 4. Refreshed dependency inventory

[Reproducible inventory](fs-reset-3-readiness-evidence/inventory.py) scans committed integrated
source only. **3282 tracked files; 254 operational files / 4385 reference lines; 320 protected
historical files.** [Per-file hashes/dispositions](fs-reset-3-readiness-evidence/legacy-files.csv),
[exact references](fs-reset-3-readiness-evidence/legacy-references.csv),
[receipt](fs-reset-3-readiness-evidence/inventory-receipt.json).
This is conservative lexical/module/gate closure, not arbitrary runtime reachability or a
bulk-delete manifest. The prior source-bound inventory is preserved unchanged.

| Remaining family | Required disposition before removal |
|---|---|
| ivac-feeds-speeds and workspace/dependency lock entries | Remove recommendation/ranking/adaptation after all imports detached. Extract neutral material/identity vocabulary first. |
| core feeds_speeds; cutting_parameters fallback | Port absent-version internal callers/tests; then remove delegates in validate_project, saved_rates and effective_machine. Keep neutral limits/roles. |
| setup_resolver / project structs | Remove old applied-rate origins/context warnings only; keep resolved Setup, generic geometry, tools, safe Z and specialized guards. Creation-only explicit seed policy is distinct from live generation fallback. |
| CLI | Approve/implement A above first; do not silently delete generate or retain hidden defaults. |
| source-pack storage / shipped Sienci assets | Detach APIs, startup/settings and data consumers; remove distributed obsolete assets only. Never remove private source-pack directories or migrate user data. |
| HTTP/Tauri/WASM recommendation endpoints | Remove registrations and transport handlers together with obsolete models. Keep generation/preset/current-contract checks, native path authorization and CPS guards. |
| schema / generated TS / xtask | Regenerate after model removal; no manual type erasure. Preserve current project/preset contracts and strict serde validation. |
| frontend old UI/translations | Replace active project/tool/settings helper dependencies before deleting unused component closures. Preserve material picker, creation/edit/history and exact physical identities. |
| historical tests / benches | Port neutral safety assertions to explicit v9 fixtures. Only calculation/rank/source lifecycle/v1–8 compatibility assertions are intentionally obsolete. |
| NUMSAFE3 / TEST1 / CI / pre-release / WASM fingerprints | Update request producers and source fingerprints, not expected safety results. Preserve TEST1/C2, corpus comparison, limits, release checks and exact-source qualification. |

## 5. Safety invariant ownership and replacement coverage

Existing coverage below is a migration destination, not permission to delete every legacy
assertion. FS-RESET-3 must map each removed test to retained assertions and ensure fixtures
reach the intended guard rather than failing earlier at the new request boundary.

| Invariant | Neutral/current owner | Coverage / required preservation |
|---|---|---|
| Explicit per-role rates, no live fallback | cutting_parameters::rates/validate_operation/validate_project; v9 transports | 7 cutting tests, cutting_v9_tests, frontend cutting-contract/project-v9/real-WASM. Missing values persist for editing but generation refuses. |
| Distinct finish, spot, chamfer | role selection, op_drivers/dual_tool and drill, setup_resolver, provenance | v9 standard/shared-distinct Drill/output tests; report-cutting tests. Preserve exact IDs, spindle transitions and tool attribution. |
| Known machine/tool RPM/feed limits | validate_limits, effective_machine, secondary setup validation | positive/negative no-mutation tests; repeat secondary-tool limits, do not clamp. |
| Unknown limits | CuttingCheck warnings, cutting-ui grouping | known_limits_reject_without_mutation_unknowns_disclosed + cutting-ui tests. Never infer a rating from silence. |
| Geometry/flutes/depth/engagement | neutral validate_operation + specialized driver validation | cutting numeric tests, specialized v9 equivalence/refusal, Halfpipe/V-Carve/Relief/Waterline suites; port old experimental safety assertions. |
| SAFE-Z/work origin/fixtures | safe_z, setup_resolver, pipeline guards | safe_z_extended.rs, hatch_safe_z.rs, work-offset/export tests. Keep authored vs program coordinates distinct. |
| Tool provenance | pipeline/provenance; simulation tool-provenance | tool_provenance.rs, frontend provenance tests, v9 dual/shared tool comparisons. |
| Simulation/full-program qualification | simulation and program-qualification/worker | qualification, qualification-wasm, qualification-export tests. Stream output is not full-program qualification. |
| Stale export refusal | generation-context/run, program-qualification, export eligibility | frontend generation-context/run/program-qualification, qualification-export; retain edit/undo/load invalidation. |
| Resource budgets | core geometry/pipeline bounded algorithms | numsafe{,2,3}.rs and CLI probes. Keep checked arithmetic, refusal before expansion, cancellation and retained geometry budgets. |
| Native file authorization | capability manifests, plugin dialog/fs scopes, publish_program_file | generated ACL check, import file tests, existing SEC1 scope guards. Installed native chooser pending. |
| Preset storage/collisions/failure preservation | preset_collections native; preset-collections/library-v9 frontend | native 3-test suite; frontend collisions/attestation/transfer tests. Atomic current-document and CAS preserved. |
| Two-sided publication | ivac-file-export, CLI/native structured outcomes | 11 export tests, side-publication tests. Do not claim pair atomicity; retain partial-result reporting and unique new filenames. |
| CLI streaming | require_current + stream_gcode_to_writer + staged publication | actual CLI cutting_v9 test, streaming/NUMSAFE3 refusal and equality tests. Preserve unsupported CPS/HPGL and stdout partial-stream limitations. |
| ShopBot scope / CPS execution | bundled CPS loader, shopbot post preflight, provenance checks | v9 ShopBot comparisons, CPS/security/adversarial tests. No global Engrave/V-bit allowance or expanded controller capability. |

## 6. Fresh integrated qualification

All commands below ran on integrated source `d6b08fe`, Darwin arm64, Rust 1.88.0,
Node 24.21.0; Rust offline with RUSTC_WRAPPER empty. Previous repair/UX logs are distinct.

| Command | Fresh result |
|---|---|
| `npm test` in frontend | **1589 PASS, 1 existing locale skip**, 187 files; 10 Node WASM package guards PASS |
| `npm run check` | **0 errors / 0 warnings** |
| `npm run lint` | PASS |
| `npm run build` with cached WASM tools/offline Cargo | PASS, WASM source freshness verified; existing chunk-size notices |
| `cargo check --offline -p ivac-tauri` | PASS; integrated capability manifests generated |
| `node scripts/check-preset-import-acl.mjs` | PASS |
| `cargo test --offline -p ivac-tauri --lib preset_collections` | **3 PASS** |
| `cargo test --offline -p ivac-core --features cps --lib cutting_v9_tests` | **12 PASS** |
| `cargo test --offline -p ivac-core --features cps --lib cutting_parameters::tests` | **7 PASS** |
| `cargo test --offline -p ivac-cli --features cps --test cutting_v9` | **1 PASS** |
| `cargo test --offline -p ivac-file-export` | **11 PASS**, 0 doc tests |
| Schema/client affected-file comparison | No schema/type changes between UX baseline and integrated source; no regeneration required |
| Scoped whitespace, pending patch and component hash checks | PASS |

[Logs and SHA-256 inventory](fs-reset-3-readiness-evidence/SHA256SUMS.json).
No new complete workspace/all-features/strict-Clippy/pre-release/TEST1-C2/corpus run is
claimed. Their complete final-source execution is mandatory after FS-RESET-3 changes.
No new installed application or physical machine qualification.

## 7. Remaining human gates and proposed execution sequence

Preserve the source-bound pre-FS2 watcher comparison: two tests fail in equivalent host
conditions before and after FS2. No disabled test, timeout increase or acceptance waiver.
Native-host watcher testing remains pending.

Native selected-file import remains pending despite correct generated ACLs and storage
tests. Browser collection download/import round-trip remains pending under the previously
recorded authorization limitation; the successful *machining report* download does not
qualify *collection* transfer. Follow the existing manual protocol for valid/cancelled,
oversized/malformed, collision/new-ID/replacement and preservation-on-failure cases.
Do not bypass denied transfer authorization. None of these pending cases is marked PASS.

Automatic dowel injection remains unsupported in v9. Explicit Drill operations are the
accepted registration preparation. No new two-sided/controller capability is proposed.

After explicit human authorization only:
1. Implement and independently qualify the approved CLI explicit-request contract; port
   internal test/benchmark request construction to current explicit roles. Retain old
   fallback until retained callers and safety assertions are migrated.
2. Detach active frontend settings/state/identity/manual-edit helpers from legacy sources;
   keep Sol UX and current preset/material/history behavior. Remove dead UI/translations
   only after import and coverage checks.
3. Remove obsolete transport/storage registrations and shipped assets, preserving user files.
   Move neutral identity/material types, regenerate schema/client/WASM exports together.
4. Remove old setup metadata/delegates and the FS crate once retained imports are zero.
   Keep generic Setup, drivers, safety/provenance/export code and independent assertions.
5. Run full final-source qualification: workspace/all-features, strict Clippy, frontend,
   codegen, fresh WASM/build, NUMSAFE/SAFE-Z/security/provenance/import/export, TEST1 including
   C2 and corpus/pre-release gates. Investigate output differences; do not loosen checks.
6. Present exact commits, exclusion ledger, remaining native/browser/physical gates and
   final UX review. HARD STOP before any release/push/installer/hardware action.

**Human decision now:** approve the integrated FS-RESET-2 checkpoint and Option A's explicit
CLI contract (including intentional refusal of no-request/ambiguous legacy flags), then
separately authorize the bounded FS-RESET-3 execution sequence. This report is a recommendation,
not that authorization. There is no unconditional GO for legacy deletion or release.
