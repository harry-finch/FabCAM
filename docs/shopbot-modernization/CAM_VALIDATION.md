# CAM validation register

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
