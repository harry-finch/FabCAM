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
