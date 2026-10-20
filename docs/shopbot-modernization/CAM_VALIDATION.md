# CAM validation register

> 2026-10-19 — **ADR-0025 implemented locally, human review pending**: [movement-tool provenance results](../releases/MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md). Pipeline63; immutable per-move tool bindings, independent native intent manifest/cache replay, bounded ShopBot emitted-intent validation, shared playback/full-program resolver. Existing post guards, manual assumptions and advanced two-sided exclusions retained. No performance optimization, commit, push or publication. **HARD STOP before any next checkpoint.**


M1 software evidence obtained 2026-10-10 on the uncommitted working tree based
on `7752ef281350faca18f737b3cb13986e6abd0474`. Exact tolerances and reproduction
notes: [M1_QUALIFICATION](M1_QUALIFICATION.md). M0 environment limitations are
historical; Rust is now available.

| ID | Evidence required | Current status |
| --- | --- | --- |
| BASE-UI | Frontend logic/type checks | 1,044 pass, 1 skipped; Svelte clean; production build passes |
| BASE-CPS-JS | Prelude/runtime behavior | M0: 39 Node tests pass; M1: CPS Boa/V8 differential suite passes |
| BASE-RUST | Rust tests and feature matrices | 1,149 pass all-features workspace excluding Tauri; 1,060 core without CPS; optional absent oracles unqualified |
| M1-IMPORT | SVG/DXF units, bounds, topology | Pass: eight cases in each format, closure and containment |
| M1-PROFILE | Ø6 outside/inside/on-line, depth passes | Pass: analytic square/rectangle bounds, circle radii, -2/-4/-5 mm, open and disconnected paths |
| M1-POCKET | Concavity/islands, floor, stepover, stepdown, allowance | Pass: independent polygon checks and heightmap; overlaps 25/50/75%; 1 mm rough allowance and final finishing ring |
| M1-ISLAND | Rounded chord cutter clearance | Pass: radii 0.1/3/10 mm and bounded padding |
| M1-DRILL | One/multiple holes, depth and peck/retract | Pass: Ø6 118° tip compensation, 2 mm pecks, authored 1.5 mm clearance plane |
| M1-DETERMINISM | Repeated generation | Pass: square profile and island pocket exact G-code/toolpath equality |
| M2-SBP | OpenSBP output and unsupported cases | Software pass: nine tests, five required goldens; actual output coordinate/sequence checks; see M2_QUALIFICATION |
| M3-PROFILE | Native machine load/save, workspace profile, post selection and unverified clearance rejection | Software pass: frontend integration and Rust CAM/post guard; see M3_QUALIFICATION; no physical measurements |
| M4-SOFTWARE | Native macOS launch/load/generate/export and offline demo | Pass: 0.5.0 arm64 app, real native export equals reference; user test pending |
| M4-TEXT | Human review of exact program | Not performed |
| M4-PREVIEW | Identified ShopBot controller/software Preview Mode | Not performed |
| M4-AIR | Supervised air cut | Not performed |
| M4-PART | Measured sacrificial part and VCarve comparison | Not performed |

Future records must identify fixture, parameters, code SHA, machine/controller,
units/origins/tool/material, invariant, tolerance, observation, reviewer/operator,
date and evidence. Record the M1 SHA after commit authorization. Software checks
do not establish controller or hardware qualification.


M1 subsequently approved and published at `7737a2a5c8e3766b03869b8e6618e3e32c7e1f0f`.
M2 subsequently approved and published at `047a75872ca7ea7d28077555c9859f92fa9a7411`.
Its evidence was collected on the working tree before that commit. Rust workspace 1,158
pass (serial), core without CPS 1,060. Optional external oracles still absent.
The independent M2 parser checks this emitted subset; it is not ShopBot Control
Software and does not establish controller acceptance.


## FS0 — 2026-10-12

[FS0 qualification](feeds-speeds/FS0_QUALIFICATION.md) defines FS-01–FS-16:
arithmetic/units, slot/pocket context, limits, missing/unsupported data,
preset matching, persistence/manual edits, legacy compatibility, consumer parity,
serialization, staleness, precision and evidence. These are **planned tests**,
not implemented or passed FS features. Synthetic fixtures are not cutting data.

Existing baseline only: the following command passed 19 tests:

```sh
RUSTC_WRAPPER='' cargo test -p ivac-core --lib pipeline::setup_resolver::tests --locked -- --test-threads=1
```

Initial sccache invocation was sandbox-blocked before compilation; approved
wrapper-free rerun passed. No new geometry, software-build or physical claim.


## FS1 — 2026-10-12

[FS1 qualification](feeds-speeds/FS1_QUALIFICATION.md) records final **38 synthetic
qualification tests + 1 API doctest** passing, Clippy, WASM compilation, workspace
format/schema checks and serial all-features Rust workspace pass excluding Tauri.
FS-01/02, engine FS-03, FS-04–09, engine FS-13, FS-15/16 are covered. CAM integration
FS-10/11/12/14 and overlap/schema/file/manual persistence proof remain deferred
FS3; real cutting profiles are FS2 and physical calibration is FS4. No production
or physical evidence is inferred from synthetic labels/values.


## FS2 — 2026-10-12

[FS2 qualification](feeds-speeds/FS2_QUALIFICATION.md): 11 dataset tests extend
FS-13 loading/version/provenance proof and FS-02/06/07/08 scope/evidence checks.
The 38 synthetic engine tests and API doctest still pass. Real source rows are
checked against independent expected values; malformed data is refused atomically.
External ReferenceData and synthetic local trial authoring remain separate.
No machine trial or calibrated lab preset is claimed. FS3 persistence/CAM tests
and FS4 physical qualification remain deferred.

## FS3 — 2026-10-12

[FS3 qualification](feeds-speeds/FS3_QUALIFICATION.md) supplies FS-10/11/12/14
integration evidence: native and portable round-trips, manual saved authority,
stale context, overlap complement, invalid saved role refusal, resolved native
and CPS rates. Serial Rust workspace and frontend suites pass. The real browser
scenario uses a software-only source-context fixture. Human software approval
remains pending; physical cutting and lab calibration remain FS4.

## FS3.5 software qualification — 2026-10-12

FS35-01: manual-only operation rates serialize/reload and generate identical
output after removing recommendation/data context and changing tool fallbacks.
FS35-02: Apply → manual feed → changed DOC → stale → actual v3 save/reopen
retains 17,000/1,500/810 and step -2 in the real browser.
FS35-03: Settings source disable survives browser reload; assistant has no
enabled sources while saved rates remain intact.
FS35-04: existing selection/availability/non-milling and eight ShopBot golden
regressions pass. Frontend 1,065 pass/one optional skip; core FS ten pass.
[Qualification](feeds-speeds/FS3_5_QUALIFICATION.md) records commands, screenshots
and limits. Human usability gate ivac-eh1g remains open; FS4 unstarted.


## FS3.7 — software architecture qualification, 2026-10-12

[Qualification](feeds-speeds/FS3_7_QUALIFICATION.md) covers source-pack validation,
license-bound public enumeration, taxonomy/specificity/fallback, derived quantities,
conservative reconciliation, deterministic ordering and local storage. Private
Apply/save carries resolved values and identity metadata without the source body.
Actual browser reopen on a separate origin with the pack absent preserves rates
and generates 34 moves; native adapter and post regressions pass. Synthetic altered
references are software fixtures only, never cutting advice. Human gate ivac-tm7u
is open; no FS4 physical evidence or local calibration is established.

## FS3.8 software qualification and future physical policy

See [FS3.8 qualification](feeds-speeds/FS3_8_QUALIFICATION.md) for capture, overrides,
loader/resolver, revision/promotion/storage and browser backup tests.
[Sorbonne policy](feeds-speeds/LOCAL_VALIDATION_POLICY.md) defines dated Tested
and later operator-reviewed Validated. All current test attestations are synthetic,
SOFTWARE ONLY. Human approval precedes FS4 real-machine calibration.

## Milling redirection audit — 2026-10-13

[Milling audit](MILLING_CAPABILITY_AUDIT.md) classifies implementation/evidence and
defines proposed UX1-01..08 analytical/semantic acceptance. Fresh frontend audit
checks: 124 passes / 9 files; Rust assertions inspected, not freshly rerun.
No new machine validation. FS4 physical FS calibration is DEFERRED for lab access
and does not block milling software; M4 evidence remains limited to its record.

## UX1 — scoped milling entry software qualification, 2026-10-13

[UX1 qualification](ux/UX1_QUALIFICATION.md) maps UX1-01..08 to analytical/native
output and real inspector evidence. Curved Profile/cascade Pocket Ramp without
tabs/leads exports XYZ; unsupported combinations and numeric WCS refuse explicitly.
Depth, direction, island clearance, tab retention, shifted geometry and saved stale
export tested. Native 1,054 + 11 + 14 + 4 pass; final strengthened variants 6 pass;
frontend 1,092 / 1 optional skip. Human gate ivac-cb64 pending; physical entry
qualification unperformed. FS4 remains DEFERRED/nonblocking.

UX1 continuation: final bundled ShopBot preview/simulation now uses emitted SBP
XYZ, ordered rapid axes, feed/dwell and source lines (ADR-0007); independent decoder
compares Direct/Ramp Profile/Pocket output exactly. Generic local Helix depth/winding
and preserved requested intent tested separately from geometry fallback and post
refusal; island/open requests reject. Final suite 1,056/11/17/4; frontend 1,092/1skip.
No clamp/fixture authoring acceptance in UX1; it belongs to CAM2/UX2.

## CAM2 / UX2 setup qualification

[UX2 qualification](ux/UX2_QUALIFICATION.md) maps stock/XY work-zero, coupled
approach/polar inputs, resolved path/region frame mapping, fixture persistence and
actual collision/export recovery. Native 1,056 core, 11 M1, 22 ShopBot cases
across full/targeted runs and 4 snapshots; frontend 1,096 pass / 1 optional skip.
Check/lint/fmt/Clippy and complete optimized WASM/frontend build pass. UX1 gate
ivac-cb64 approved; CAM2 human gate ivac-97fb pending. No physical clearance or
new native bundle; Z/two-sided limitations remain explicit. Historical UX1 numeric
origin restrictions above are superseded only by the bounded XY scope of ADR-0008.

## UX2.1 interaction follow-up

[UX2.1 qualification](ux/UX2_1_QUALIFICATION.md) adds UX21-01..10 for real structured
notifications, red fixture feedback, first 3D→2D focus, model-backed shape dragging,
one history entry, actual Save/reopen and stale/collision/export recovery. Existing
UX2 collision mathematics and native evidence are retained. Frontend 1,106 pass /
1 optional skip in 119 files; zero check warnings and lint/build pass. Human gate
ivac-svnf pending before CAM3; no physical clearance or new native bundle claim.

## CAM3 — holding tabs and cutting transitions

[CAM3 qualification](CAM3_QUALIFICATION.md) defines CAM3-T01..T04: width and
absolute override-height preservation, 48-case output/preview/direction matrix,
curved placement/ramp compensation and entry-tab preservation. Reproduced defects
corrected with independent emitted-SBP geometry assertions. Conservative arc
rectangle residuals and unqualified topology combinations remain explicit.
Software handoff only; human gate ivac-wh2m before CAM4, no physical qualification.

## CAM4 A/B — planar remaining material

[Qualification](CAM4_QUALIFICATION.md) records CAM4-A01..A07 analytical sweep,
sharp corners, concavity, depth, islands, cutoff and emitted/origin proof plus
CAM4-B01..B05 drawing-frame, stale, unsupported, toggle and browser persistence
checks. Optional diagnostics do not change machining output. Explicit unsupported
contexts are not classified as fully machined. Human gate ivac-tvox before broader
scope; no CAM4/CAD1 Phase C or physical qualification claim.

## V-Carve software checkpoint

[Current qualification](VCARVE_QUALIFICATION.md): A1 pass scheduling/radius, A2 interpolated envelope, A3 complete short-chain depth, A4 independent OpenSBP/preview parity PASS. Full medial area coverage PARTIAL; Perimeter square-island offset explicitly refused. Halfpipe unchanged/regressions pass. Synthetic settings, no physical test; human gate ivac-1b75.

## 2026-10-19 — Full-program export and load regression checkpoint

[ADR-0024 results](../releases/UX_SAFETY_LOAD_RESULTS_2026_10_19.md) records direct
export, worker lifecycle, real WASM and browser Home/report invariants; project
replacement/roundtrip/Undo evidence; targeted SAFE-Z/post regressions and TEST1
256/256. Software scope only. Human gate ivac-3i00w, coverage limits ivac-b0js3.
No installer or physical qualification inferred.

## 2026-10-19 — ADR-0025 follow-up (ivac-b0js3.1)

[Results](../releases/ADR0025_FOLLOWUP_RESULTS_2026_10_19.md). Software scope only; human review pending.

| ID | Evidence required | Current status |
| --- | --- | --- |
| PROV-A | Bundled ShopBot provenance for imperial and metric load diameters | Pass after fix: 1/64–1/2 in matrix and 6 mm + 3/32 in rough/finish resolve; malformed/out-of-bound diameters and number mismatches refuse |
| PROV-B | Native tool names never read as machine words; real `T`/`M6`/`M06`/`M61` still refuse | Pass after fix: LinuxCNC/GRBL names with M6, M06, T2, Unicode, parentheses, `;`; adversarial envelopes |
| PROV-C | Unmodelled native features stay planned-only with a reason | Pass: Homing (G28), custom profile, optional stop; single plain tool resolves |
| PROV-WS | Full workspace regression | `cargo test --workspace` parallel green; CI `--all-features` variant recorded in results |

## 2026-10-19 — G-code comment safety and release gate (ivac-4ql18)

[Results](../releases/GCODE_COMMENT_SAFETY_RELEASE_GATE_2026_10_19.md). Software scope only.

| ID | Evidence required | Current status |
| --- | --- | --- |
| GC-COMMENT | Hostile tool/op/group/pause/label text never changes executable LinuxCNC/GRBL lines | Pass: `tests/comment_safety.rs`, unit and GRBL rewriter tests; provenance resolved |
| REL-PEDANTIC | `pre-release.sh` pedantic Clippy gate | Pass under the documented `[workspace.lints]` policy |

## 2026-10-20 — Import correctness (ivac-feder)

[Results](../releases/IMPORT_CORRECTNESS_RESULTS_2026_10_20.md). Software scope only; no third-party CAD export available.

| ID | Evidence required | Current status |
| --- | --- | --- |
| IMP-OCS | DXF OCS ±Z for ARC/CIRCLE/LWPOLYLINE/POLYLINE/ELLIPSE/TEXT/INSERT; tilted refused | Pass: analytic fixtures, `tests/import_correctness.rs` |
| IMP-BLOCK | Base point, rotation, scale, mirror, nested, repeated, combined OCS; MINSERT refused | Pass: analytic fixtures |
| IMP-UNITS | SVG mm/cm/in/pt exact; px/unitless/viewBox and unitless DXF warned | Pass: core + import summary UI ([ADR-0026 results](../releases/IMPORT_UNITS_WARNINGS_RESULTS_2026_10_20.md)); human acceptance pending |

