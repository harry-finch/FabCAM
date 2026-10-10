# M2 — minimal deterministic OpenSBP export

2026-10-10. User approved publication of M1 and starting M2. M1 is committed
and pushed at `7737a2a5c8e3766b03869b8e6618e3e32c7e1f0f`. M2 is reviewable and
uncommitted on that base; no authorization to publish M2 or begin M3 yet.

## Architecture and supported contract

[ADR-0001](adr/ADR-0001-shopbot-cps.md) chooses the existing CPS seam. An authored
GPL post `crates/ivac-cps/posts/shopbot.cps`, registry id `shopbot`, returns
extension `sbp` through the existing pipeline. CPS-enabled CLI/server/Tauri
list it automatically. The existing frontend picker/export honors registry
metadata and extension; no UI rewrite. Browser WASM has no CPS and remains
explicitly unavailable for this post; use a CPS-enabled transport.

Metric XYZ, one operator-established work zero, flat end mills and drills,
manual tool changes and **manual spindle control including RPM**. Every section
retracts to authored clearance and pauses for stationary-spindle tool/zero
verification, then pauses for CW spindle at requested RPM. At section end it
retracts and requires manual stop. Pauses cannot be disabled by post properties.
These are operator responsibilities, not electronic interlocks. Controller jog
speeds and actual clearance suitability remain M3/M4 checks.

Generic optional CPS metadata exposes MachineConfig JSON, clearance Z and
whether any project tool carries a static Z shift. No schema/IR shape change.
The post preflights the entire IR before writing. It refuses ATC/Ignore/M6 modes,
post-change Z automation, controller tool offsets, static tool shifts (including
unused project tools), station/parking/custom scaffold, coolant, CCW, unsupported
tools/strategies, kinematics/work planes, additional work offsets, machine-frame
rapids, raw commands, mid-section spindle changes, advanced cycles and invalid
coordinates/feeds/dwells. Machine-specific knowledge stays in the post.

Planar XY arcs are explicitly linearized at 0.01 mm through existing CPS code.
Simple/dwell drilling and full-retract peck cycles expand to explicit moves.
Rapid lifts precede XY travel; descents follow. Unsupported lateral rapids wholly
below stock fail. The post does not insert cutter compensation or invent limits,
home positions, relay mappings, macros, probing or automatic RPM control.

All emitted syntax and source pages: [command map](shopbot/OPEN_SBP_COMMANDS.md).
VD sets metric units and persists converted controller settings; there is no
automatic restoration. No command forces Move/Cut mode or disables limits.

## Evidence

`crates/ivac-core/tests/shopbot_m2_post.rs`: six integration tests, including
five actual import→CAM→CPS→SBP cases in both SVG and DXF and eight unsupported
machine-policy cases. SVG generation repeats exactly and matches required full
text goldens in `tests/fixtures/shopbot-m2/`; no silent missing-file skip.

An independent numeric parser accepts only the emitted command subset and
checks finite values, positive speeds, mm/min÷60 conversion, positive-Z XY jogs
and final Z5 clearance. Analytic checks cover square outside bounds 7..113 mm,
circular profile radius 28 mm, pocket island clearance and floor coverage,
peck tip depth −5−3/tan(59°), at most 2 mm new-material pecks, authored Z1.5
retracts, dwells and two ordered operations with tool 1→2 and stop/load/start
pauses. Pocket checks use exported cutting segments, not just the internal
preview: boundary/island samples and a 2 mm lattice covered by Ø6 sweeps.

Circle tolerance budget: upstream compensated geometry 0.02 mm + post chord
sagitta 0.01 mm + rounding allowance 0.0002 mm. Chord midpoints stay within
0.0302 mm of the analytic radius; endpoints within 0.0202 mm. These are software
comparison budgets, not machine tolerances. Feed grid is 0.000001 mm/s, XYZ grid
0.0001 mm and dwell grid 0.001 s. Timed controller pauses may have coarser timing.

`crates/ivac-cps/tests/shopbot_guards.rs`: three tests looping over unsupported
records, missing scaffold, tools/planes/work offsets, kinematics and comment
injection. Hostile multiline comments remain single ASCII comment lines.
Errors return no successful program response; no partial SBP is offered for use.

Full all-features non-Tauri workspace: **1,158 pass, zero failed/ignored** with
serial test execution. Existing M1 and CPS/native regressions pass, Clippy and
formatting pass, schema/version unchanged. Frontend: **1,044 pass, one optional
skip**, Svelte clean. See TESTING_STRATEGY for builds and exact commands.

## Remaining issues and limits

- A pre-existing global-cache test race surfaced in a parallel run: the island
  cache test expected two entries but saw three from another pipeline test.
  Its test mutex does not isolate production callers. Serial execution is the
  qualification workaround; open Beads issue `ivac-htc8`, no cache change in M2.
- The WASM freshness guard accepted partial wasm-pack output after sandbox
  interruption. Full rebuild restored packaging; open Beads issue `ivac-5g51`.
  No guard/dependency/config change was made in M2.
- `pocket.sbp` is 31,733 lines with this fine island corpus. Controller buffering,
  execution time and very short segments require Preview/performance assessment.
  Determinism and sampled coverage do not establish controller execution quality.
- Existing ivaCAM preview/simulation uses recorded CAM motion; it does not parse
  this SBP's manual scaffold or model indefinite operator waits. M2 text tests
  independently inspect output, but no ShopBot parser/Preview Mode was available.
- Hardware/software version, spindle wiring, motion limits, jog settings, safe
  positions, clamp clearance and emergency behavior are unverified. No Windows/
  Linux execution, Tauri bundle, VCarve comparison or physical cut performed.
- Helical/rotated/advanced cycles and other tools/strategies intentionally fail.
  Only the minimal corpus is qualified. Manual spindle management is mandatory;
  automatic spindle support awaits verified machine information and approval.

No generic CAM algorithm changed; ProtoMAX remains deferred, assessment B.
This milling post adds no fake waterjet semantics.

## Handoff

Review M2 and authorize its commit/push. Next milestone M3 needs explicit
approval: gather actual machine facts and create the Sorbonne ShopBot profile,
marking unknowns TO VERIFY ON MACHINE. M4 remains the physical validation gate.

STOP — waiting for human approval before starting the next milestone.
