# CAM validation register

M0 does not qualify machining correctness. No new CAM corpus was implemented.
Existing Rust geometry/volume/snapshot tests could not run without cargo.

| ID | Evidence required | Current status |
| --- | --- | --- |
| BASE-UI | Existing frontend logic/type checks | 1,044 tests pass, 1 skipped; Svelte clean; see TESTING_STRATEGY |
| BASE-CPS-JS | Existing prelude behavior | 39 Node tests pass; Boa execution not tested |
| BASE-RUST | Existing Rust unit/integration tests, feature matrices | Not executed; environment blocked |
| M1-IMPORT | Required deterministic SVG/DXF geometry, units/transforms/open paths | Awaiting M1 approval |
| M1-PROFILE | Ø6 compensation, inside/outside/on-line, Z schedule, tolerances | Awaiting M1 approval |
| M1-POCKET | Islands/concavity/coverage, stepover/stepdown, allowance | Awaiting M1 approval |
| M1-DRILL | Hole locations, depth, peck/retract semantics | Awaiting M1 approval |
| M2-SBP | Deterministic square/circle/pocket/drill/multiple-op output and unsupported cases | Not implemented |
| M4-TEXT | Human review of exact exported program | Not performed |
| M4-PREVIEW | ShopBot Preview Mode on identified software/controller | Not performed |
| M4-AIR | Supervised air cut with documented setup | Not performed |
| M4-PART | Sacrificial reference part, dimensions/depth/order, equivalent VCarve job | Not performed |

Future records should identify source fixture, parameters, code SHA, machine and
controller/software version, units/origins/tool/material, expected invariant,
tolerance, observed result, reviewer/operator, date and supporting evidence.
Do not report physical validation unless a human operator supplies results.
One successful part does not establish production readiness.
