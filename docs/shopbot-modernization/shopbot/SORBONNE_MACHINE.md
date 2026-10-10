# Sorbonne ShopBot PRS machine record — M3

Profile: [native machine snapshot](../../../machines/sorbonne-shopbot-prs.ivac-machine.json).
Status: loadable configuration for qualification; **physical qualification pending**.

## Sources and facts

The original project brief identifies a Sorbonne lab ShopBot PRS Gantry,
approximately 2011. On 2026-10-10 the user supplied: **“Gantry 120x60cm from
2011, Sb3.8. le reste est à vérifier.”** This is the source for the nominal
1200 × 600 mm XY size, year 2011 and reported software SB3.8. No machine plate,
controller settings, measured travel, spindle plate or physical observations
have been inspected. Do not substitute generic PRS catalogue specifications.

Software policies below come from the owned M2 post at
`047a75872ca7ea7d28077555c9859f92fa9a7411`, not measured controller capability.
Command references are recorded in [OPEN_SBP_COMMANDS](OPEN_SBP_COMMANDS.md).

| Item | Recorded value / source | Qualification still required |
| --- | --- | --- |
| Identity | ShopBot PRS Gantry; 2011, user report | Exact model/serial, controller generation: **TO VERIFY ON MACHINE** |
| Work envelope XY | Nominal 1200 × 600 mm, converted from user's 120 × 60 cm | Actual usable travel, fixture margins and relation to work zero: **TO VERIFY ON MACHINE** |
| Work envelope Z | **TO VERIFY ON MACHINE** | Snapshot `workArea.z = 0` is an unknown sentinel, not physical travel |
| Practical rapid limit | **TO VERIFY ON MACHINE** | Record actual configured XY/Z jog speeds and tested practical limits; post emits J2/JZ and does not set JS |
| Cutting-feed limit | **TO VERIFY ON MACHINE** | No profile ceiling supplied; confirm controller, axes, cutter, material and workholding constraints |
| Plunge-feed limit | **TO VERIFY ON MACHINE** | No independent machine plunge ceiling exists in the current model; use reviewed tool plunge rates; shared maxFeedMmMin can cap both feeds |
| Spindle | **TO VERIFY ON MACHINE** | Type, power, VFD/relay wiring, CW direction, manual start/stop and interlocks |
| RPM range | **TO VERIFY ON MACHINE** | No min/max supplied; post pauses to request manual RPM from the reviewed tool; no automatic spindle command |
| Tool changes | M2 software policy: manual, mandatory PAUSE, operator sets Z for each section/tool | Actual lab procedure and spindle fully stopped: **TO VERIFY ON MACHINE**; no ATC or controller tool-length offsets |
| Safe Z | **TO VERIFY ON MACHINE** | `fastMoveZ = 0` deliberately blocks post export; replace only after choosing clearance above stock, clamps and obstacles within measured Z travel |
| Park/home | Software policy: lift to authored clearance at final XY; no XY parking/home motion | Homing procedure, machine origin and acceptable final location: **TO VERIFY ON MACHINE** |
| Work zero | M2 software policy: one operator-established work origin, stock top Z=0, positive Z up; absolute metric XYZ | Actual XY location, Z convention, zeroing/touch-off method and origin retention after pauses: **TO VERIFY ON MACHINE** |
| Arcs | Profile disables arc fitting; M2 linearizes XY arcs, rejects helical/non-XY arcs | Native controller arc capability **TO VERIFY ON MACHINE**; it is not needed for this subset |
| Software/controller | User reports SB3.8; emitted subset uses metric VD,,1 and SA | Full version/build, controller/firmware, command acceptance, Preview behavior, pause/jog/END behavior: **TO VERIFY ON MACHINE** |

## Profile representation and limits

The native snapshot selects `cps` / bundled `shopbot`, milling, metric output,
comments, mandatory manual tool pauses and no parking. These are deliberate
software restrictions; they do not assert installed hardware features. There
are no relay/VFD assumptions, tool library, RPM/feed recommendations, or sample
machining parameters in this profile.

The current numeric machine schema has no explicit “unknown” type. Required
clearance is therefore zero, as is the unknown Z component of the XY envelope.
Both sentinels are explained here and the machine name marks the entire profile
unverified. The checked-in file preserves that name and both zero values through
load/save and workspace-profile application. The existing ShopBot guard rejects
nonpositive program clearance before returning an exportable SBP response.

Other unknown physical values are omitted. This does **not** mean their limits
have been verified or that an unbounded feed/RPM is permissible. Existing ivaCAM
simulation/UI fallbacks (including rapid speed 5000 mm/min, acceleration
250 mm/s² per axis and estimated tool-change time 5 s) are software defaults,
**not ShopBot measurements**. Opening/applying the existing Machine dialog may
materialize acceleration defaults. Review those fields against the machine
record before any physical qualification. The envelope is used for stock sizing;
it is not an enforced machine travel boundary. Simulation time remains unqualified.

Load replaces machine settings while retaining project tools, geometry, stock,
operations and work offset. Inspect these separately. No complete project or
machine command is sent to hardware. Standalone browser WASM cannot execute CPS;
use an existing CPS-enabled transport for software generation.

## Completing the record at the machine

Before requesting M4 physical validation, record exact model/serial, software
build and controller; measured usable XYZ and reference positions; actual jog
settings and practical limits; feed/plunge limits and reviewed tool/material
parameters; spindle type, RPM range, direction and manual operating procedure;
zeroing/tool-change procedure; clearance and fixture heights; and end-location
policy. Identify the operator, date and evidence source for every confirmed
value. Preserve nominal user-reported facts separately from measurements.

Update a copy of the profile with the confirmed fields and an authored positive
clearance. Feed/plunge and RPM remain tool/job choices inside verified machine
limits. No machine probing, homing script, preview, air cut or machining has been
performed in M3. M4 remains a separate explicitly authorized milestone.
