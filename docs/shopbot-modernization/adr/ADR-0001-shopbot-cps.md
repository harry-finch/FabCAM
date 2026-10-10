# ADR-0001: ShopBot MVP through the existing CPS recorder/runtime

Status: Accepted for the approved M2 implementation; milestone review pending
Date: 2026-10-10

## Context

M1 validated simple SVG/DXF milling. Native PostProcessor and CPS both receive
motion, feed, spindle, cycles and sections. CPS owns scaffold, and its library,
property inspection, selection, output extension and errors already work in
CLI/server/Tauri. WASM does not enable CPS; pure browser ShopBot export is
therefore unavailable and must keep reporting CpsUnavailable.

## Decision

EXTEND CPS with an ivaCAM-authored GPL-3.0-or-later bundled `shopbot` post.
No Autodesk/third-party source copied. Reuse production CAM and recorder and
the current prelude's analytic arc linearization and drill expansion.
Metric XYZ, flat mills/drills, one work zero, manual tool and spindle control.
Reject non-XY/helical arcs, advanced cycles, raw commands, coolant, CCW,
ATC/probing/tool tables/static Z shifts, station/park settings and unsupported IR records.
Operator pauses establish each tool's work Z and requested CW RPM; automatic
relay/VFD commands require verified machine behavior and remain deferred.

The existing general CPS driver omits machine-frame rapids and unknown records,
and lacks machine scaffold policy in its header. Add generic optional header
parameters `ivacam:machine-config` (existing MachineConfig JSON string) and
`ivacam:clearance-z` (mm), plus a section flag
`ivacam:uses-static-tool-z-shift`; no IR/schema shape change. ShopBot checks the entire
version-1 IR before output, using ivaCAM-private __ivacRun.program. This local
coupling is deliberate: only this owned post relies on it; third-party posts
keep their current entry-point API. Other posts can ignore these parameters.
Require this context rather than assume machine policies when absent.

## Alternatives considered

- Native Rust: available in WASM and strongly typed, but trait defaults and
  scaffold helpers are G-code-oriented and need more dialect-specific overrides.
  No present requirement warrants duplication of the existing CPS path.
- CPS callbacks alone: cannot reject records the driver silently omits. Rejected
  because machine-frame/tool/probe semantics must not disappear.
- New universal IR or process layer: unnecessary for this milling subset.

## Consequences

CAM remains ShopBot-independent. Fork delta touches the bundled registry and
recorder/section builder's generic metadata only; no new machine enum or generated client type.
CPS-enabled transports list the post automatically. Frontend export already uses
output_extension. Browser users require a CPS-enabled server or desktop build.
Post knowledge of MachineConfig serialization and IR v1 is a maintenance seam;
reject unknown IR versions and regression-test policies. Manual spindle pauses
are mandatory; output does not imply automatic interlock or hardware readiness.
Finer arc chords increase file size; controller buffering/performance needs M4.
Future ProtoMAX still requires distinct process events, as assessed in M0.

## Validation

Inspect bundled metadata; run actual CAM→CPS→SBP for square/circle/pocket/drill/
multiple operations; exact goldens plus independent numeric/sequence checks.
Test rejection of machine policies, records, commands, units assumptions,
kinematics and invalid numbers. Run existing CPS/native regressions and frontend
checks. Revisit native output if browser-only CPS becomes a requirement or the
owned post's private IR coupling grows beyond this bounded scope.
