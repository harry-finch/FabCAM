# M2 emitted OpenSBP subset

Research checked 2026-10-10 against official historical documents, appropriate
for investigating a circa-2011 machine. Actual controller/software is **TO
VERIFY ON MACHINE**. No controller parser or Preview Mode was run here.

[OpenSBP / ShopBot Command Reference, 2007](https://www.opensbp.com/files/ComRef.pdf):

| Emitted form | Meaning | Printed page |
| --- | --- | --- |
| `VD,,1` | Set XYZ units to mm; first two settings omitted | 33–34 |
| `SA` | Absolute working coordinates | 24 |
| `JZ,z` | Z jog to a position | 17 |
| `J2,x,y` | XY jog to a position | 15 |
| `MS,xy,z` | Set move speeds in mm/s | 19 |
| `M3,x,y,z` | Interpolated XYZ cutting move | 17 |

The post serializes rapid lifts, XY travel and descents. Feed conversion is
mm/min ÷60; both move-speed fields receive that value. Jog speeds are retained
from the controller and must be verified. VD changes persistent settings and
converts existing system values. Neither calibration nor zeroing is emitted.
No command forces Move/Cut mode, disables limits, homes or parks.

[ShopBot Programming Handbook, 2010](https://www.opensbp.com/files/ProgHand.pdf):

| Emitted form | Meaning | Printed page |
| --- | --- | --- |
| `' text` | Full-line comment | 23 |
| `PAUSE` | Indefinite operator stop, preceding comment becomes prompt | 19–20 |
| `PAUSE seconds` | Timed dwell in seconds | 19 |
| `END` | End this part file | 12 |

Manual spindle start/stop and requested RPM appear in operator prompts.
No SO/TR/C-series macro, automatic tool change or spindle output is emitted.
Raw commands are rejected. Arcs become XYZ chords at 0.01 mm tolerance; no CG
syntax or controller arc mode is assumed. Simple/dwell/peck drilling becomes
explicit jog/cutting moves through the existing cycle expansion.

Numeric output: XYZ four decimal places (mm), speeds six (mm/s), dwell three
(seconds), period decimal separator, fixed deterministic text, no timestamp.
These formatting grids are separate from arc tolerance and machine accuracy.
Pure browser WASM has no CPS; the post is listed by CPS-enabled transports.
