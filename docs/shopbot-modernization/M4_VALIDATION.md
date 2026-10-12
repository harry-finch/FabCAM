# M4 — human qualification protocol and record

M4 authorized by the user on 2026-10-10; **in progress, not achieved**.
M3 approved and pushed at `b8a073a`. Local macOS build and offline test kit are
preparation; real-machine evidence is still absent.

## Software handoff first

[Offline test instructions](../../tests/fixtures/shopbot-m4/README.md) and
[self-contained project](../../tests/fixtures/shopbot-m4/offline-square.ivac-project.json)
exercise file loading, CAM, owned ShopBot CPS, 2D/3D inspection and export.
The generated [reference SBP](../../tests/fixtures/shopbot-m4/offline-square.sbp)
uses synthetic M2 test parameters, explicitly labelled OFFLINE DEMO ONLY.
No additional installation or compilation is required on the current Apple
Silicon Mac. Other OS builds are not supplied or validated in this handoff.

Source geometry: M1 `square_100.svg`, square (10,10)–(110,110) mm. The nominal
retained square is 100 × 100 mm, external compensation for Ø6 puts centreline
extrema at 7 and 113 mm. Virtual stock 140 × 140 × 10 mm; depth passes −2/−4/−5,
clearance +5, synthetic feed 1200 mm/min, plunge 200 mm/min, 18000 RPM CW.
These are software fixture parameters, **not verified lab cutting settings**.
The output requires mandatory manual spindle/tool/zero handling; do not treat
it as an automatic spindle-control program.

## Software preparation evidence — 2026-10-10

Built release frontend and CPS-enabled Tauri macOS arm64 bundle, version 0.5.0,
with Rust 1.88.0, Node 24.21.0, pnpm 10.34.6 and Tauri CLI 2.12.1. Delivery is
`/Users/mugen/Documents/01_Projets/FabCAM/ivaCAM-M4`; BUILD.json identifies the
M3 source base plus uncommitted M4 preparation and artifact hashes. Local ad-hoc
signing repaired the initial bundle signature; strict verification passes.
No notarization, installer or Windows/Linux build produced.

Native UI check: launch, file load/reopen, correct machine/CPS settings,
generation, warning panel, `.sbp` label and native Save picker, actual file write.
The native export equals the independently generated HTTP/reference SBP
byte-for-byte (164 lines plus the final newline). Only the expected manual-spindle
post warning remains. The operation explicitly selects `svg`; selecting ALL
would also machine the visible synthetic stock outline. Human software test is
still requested; these agent UI checks are not controller/hardware evidence.

Delivery fixes: generated extension now drives toolbar labels; optional boolean
checkbox fields are normalized before the machine draft baseline so mounting
does not falsely mark it dirty and block project refresh; frontend version falls
back to the Tauri release version when package.json has no version. No CAM/post
change. Full frontend suite 1,046 passes/one optional skip; Svelte/TypeScript clean,
targeted ESLint and formatting pass. Offline adapter regression covers actual
stock-augmented geometry and explicit layer selection; native response remains
identical to the reference. Rust broad baseline is unchanged from M2.

Reproduction: `tauri build --bundles app -- --locked` in crates/ivac-tauri with
Node/pnpm/Rust on PATH and RUSTC_WRAPPER empty; `pnpm exec vitest run` and
`pnpm check` in frontend. The demo test's optional IVAC_M4_REQUEST_OUT writes
the real frontend request for `/generate` verification; normal tests write no
fixtures. CPS is buffered: CLI stream-gcode deliberately does not support it.

## Machine programme preparation

Complete [SORBONNE_MACHINE](shopbot/SORBONNE_MACHINE.md) with a human operator.
Record actual travel and jog settings, machine/controller/software identity,
spindle/RPM, appropriate tool/material feeds and plunge, holding/obstacles,
tool-change and zeroing procedure. Choose an authored clearance within verified
travel and above all relevant obstacles. Nominal reported XY alone is not a
measurement or a complete operating envelope.

Make a separate job/profile copy with those reviewed values and a distinct name.
Regenerate SBP; identify the project, machine profile, app build and file SHA-256.
No direct machine transfer or execution is performed by the coding agent.

## Ordered human gates

The user replied **“je valide !”** on 2026-10-10 after the compiled-app/offline
test handoff. This records positive software acceptance. No individual test
observations, actual-machine text review, Preview or physical results were
provided. The gates below retain their pending status.

| Gate | Human action and observations to record | Current evidence |
| --- | --- | --- |
| M4-TEXT | Read the exact final machine file. Check metric/absolute setup, bounds, Z sign, exterior compensation, depth passes, retract-before-XY, manual pause sequence, RPM requests and END. Identify reviewer/date and file digest. | Awaiting human review |
| M4-PREVIEW | Load that exact file in reported SB3.8 Preview Mode; record full build/controller, parser messages, bounds and Z interpretation, screenshots and pause/end behavior. Any discrepancy returns to software review. | Not performed |
| M4-AIR | Supervised air cut only after the prior gates. Operator establishes a separate documented air-cut origin/clearance so the entire swept tool remains above stock/fixtures throughout negative programmed depths; reconcile all tool-zero prompts with that convention. Verify motion order, direction, stops, jog settings, clearance and end position. | Not performed |
| M4-PART | Restore and independently verify the machining work origin/tool Z after the air-cut setup. Use reviewed sacrificial stock/tool, verified clearance and cutting parameters. Record operator, workholding, observations and any unexpected movement. | Not performed |
| M4-MEASURE | Measure nominal square XY and 5 mm shoulder depth, compensation intent and corner behavior; set acceptable dimensional/depth tolerances with the operator **before cutting**, not after observing results. Record instrument, measured values and deviations. | Not performed |
| M4-VCARVE | Prepare equivalent VCarve 8 square job with the same tool, origin, stock, outside compensation, depths and feeds. Compare machining intent, depth, dimensions and ordering; paths need not be byte-identical. | Not performed |

Proceed in order under human supervision. Record any unexpected motion or pause,
stop the trial and retain the exact file/settings for diagnosis. Passing an
offline preview or producing one part does not establish general production
readiness. M4 completes only when the measured reference part has correct XY,
depth, compensation and ordering with no unexpected movement.

## Results record

| Field | Operator-supplied evidence |
| --- | --- |
| Operator/reviewer/date | Pending |
| Machine serial/model and measured envelope | Pending |
| SB3.8 full build / controller firmware | Pending |
| App identity, project/profile, final SBP SHA-256 | Pending |
| Tool, measured diameter, stock/material, workholding | Pending |
| Work zero, Z touch-off and clearance | Pending |
| Configured jog/feed/plunge/RPM and actual procedure | Pending |
| Pre-agreed XY/depth tolerances, measuring instrument | Pending |
| Text/Preview observations and messages | Pending |
| Air-cut origin and observed motion/stops | Pending |
| Machining observations and anomalies | Pending |
| Measured XY/depth/corners; equivalent VCarve evidence | Pending |
| Decision and remaining limits | Pending |

Beads tracks the preparation work separately from the pending physical evidence.
Do not close M4 or begin post-M4 product tracks while these results are absent.

## Human evidence update — 2026-10-12, r1 superseded for spindle workflow

User supplied ShopBot Preview photographs showing Completed: Yes, min Z -5,
max Z +15, no offset; reported operator pauses and two passes observed, then
confirmed supervised air cut behaved as expected. Preview material was corrected
to 500x500x12 at XY(0,0), surface Z zero. Y max 387.83 includes the pre-run
position; programmed XY stays 7..113. Photo jog fields: XY 152.4 mm/s and
Z 76.2 mm/s (9144/4572 mm/min), replacing previously reported 10000.

Material trial then exposed a blocker: spindle cannot be started through SB3
while paused; generic manual-setup prompts are unusable. No cut/measurement
result was reported. r2 is described in M4_SPINDLE_R2.md and requires renewed
text/Preview and spindle-enabled non-contact qualification. The older table
above is the original handoff record, not a claim that user evidence is absent
for r1. Do not carry r1 motion acceptance into r2 spindle validation.

## Human evidence update — 2026-10-12, r2

User supplied r2 Preview photograph PXL_20261012_093159935.jpg: Completed Yes,
X/Y bounds 7..113 mm, Z -5..+15 mm, Offset no. Automatic Preview material
106 × approximately 106 × 5 mm represents the toolpath bounds, not the physical
500 × 500 × 12 mm MDF; manual stock setup was requested again.
Following the supervised spindle-enabled air-test handoff, the user replied
“c’est un sans faute !”. Record positive overall r2 air-test acceptance, including
the requested automatic spindle start/stop and two-pass behavior. No separate
RPM measurement, air-origin coordinates or detailed checklist was supplied; do
not invent those values. No material machining or dimensional result reported.

Current r2 SBP SHA-256: 10d0420055ef4e4d259357b62a7a21dc9deaf195fa291253d21307f2f4945c4a.
Next gates: restore/verify surface Z, agree tolerances before cutting, MDF trial,
measurements and equivalent VCarve comparison. M4 remains open.

## Human material-trial acceptance — 2026-10-12, r2

After the handoff explicitly requesting the real MDF trial with the same r2 SBP,
the user replied “c’est un deuxième sans faute, tout fonctionne parfaitement !”.
Record successful operator-reported material machining with no anomaly reported.
No measured dimensions, shoulder depth, instrument, pre-cut tolerance agreement
or equivalent VCarve comparison were supplied. M4-PART accepted by operator;
M4-MEASURE and M4-VCARVE remain pending. The application and SBP are unchanged;
no new implementation, milestone completion, commit or push follows this report.

## Operator target-conformity confirmation — 2026-10-12, r2

User states “tout est parfait, dimensions, profondeur cible, dégagement à 15mm
à la fin. l’usinage est propre. c’est nickel.” Record operator-confirmed conformity
to the nominal retained square 100 × 100 mm and 5 mm depth targets, clean machining
and final +15 mm clearance. No exact dimensional readings, measuring instrument
or quantified tolerance agreement were supplied; do not invent them or claim
a metrology certificate. M4-MEASURE has qualitative operator acceptance.
Equivalent VCarve comparison remains unreported and is the outstanding M4 gate.
No change to the tested application or SBP; no milestone commit/push or new track.

## VCarve reference received — 2026-10-12

Compared supplied M4-r2-carre-MDF_vcarve.sbp with tested r2. Nominal 100 mm square,
6 mm exterior compensation, final depth 5 mm and 18000 RPM agree. Reference is
translated (+1.246161,+0.409699 mm), uses -2.5/-5 passes, 4980/1794 mm/min feed/
plunge and safe/home Z 6/20 mm. Traversal opposite; CG versus linearized corners.
See M4_VCARVE_COMPARISON.md for hashes and evidence. Parameter alignment pending;
no defect inferred, no new cut or application change requested. M4 remains open.

## M4 qualification decision — 2026-10-12

Second VCarve export aligns square location/exterior D6 compensation and -3/-5
passes. Actual feed/plunge 5976/1992 differ by -0.4%; imposed safe/final Z 6/20,
opposite traversal, arc representation and XY parking documented and accepted
for semantic comparison. See M4_VCARVE_COMPARISON.md for source hashes/results.

Operator confirmed r2 Preview, spindle-enabled air cut, real MDF cut, target
dimensions/depth, clean finish and +15 final retract. M4 qualification completed
at this reported evidence level; no exact metrology readings/instrument or measured
RPM invented. ivac-o4oy closes. Explicit milestone commit/push approval still
required; no code/app/SBP changes or post-M4 governance/FS implementation performed.
