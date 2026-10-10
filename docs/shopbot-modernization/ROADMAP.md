# Roadmap and approval gates

Every milestone ends with a human approval gate. Never proceed automatically.

| Milestone | Included work / acceptance | Status |
| --- | --- | --- |
| M0 | Fork/remotes/feature branch; code and feature audit; known build/test baseline; post and future-process assessment; committed/pushed documentation | Complete with recorded environment limitations; see STATUS |
| M1 | Deterministic SVG/DXF corpus; analytical profile outside/inside/on-line, pocket and drill checks; regressions and unresolved issues recorded | Approved, committed/pushed at 7737a2a; see M1_QUALIFICATION |
| M2 | CPS vs native ADR; authoritative OpenSBP research; minimal deterministic post; unsupported semantics explicit; output tests | Approved, committed/pushed at 047a758; see M2_QUALIFICATION |
| M3 | Actual lab machine profile; known values sourced; unknown values marked TO VERIFY ON MACHINE | Loadable unverified profile complete; review and commit/push approval pending; see M3_QUALIFICATION |
| M4 | Human textual review, Preview Mode, supervised air cut, sacrificial machining, measurement, VCarve comparison; no unexpected motion | Not authorized |

M1 corpus minimum: `square_100`, `rectangle_100x50`, `circle_d50`,
`concave_L_shape`, `rectangle_with_hole`, `rectangle_with_island`,
`multiple_closed_contours`, `open_polyline`. Use a known Ø6 mm cutter and test
depth passes, compensation, pocket islands/concavity/stepover/stepdown and
available allowance; drill depth and peck semantics. Compare intent/invariants,
not byte-identical VCarve paths.

M2 may deliberately restrict output to metric, one spindle and manual changes,
without probing/ATC/advanced cycles. Every emitted command needs verified syntax.
Do not copy third-party CPS source without a compatible license.

M3 records envelope, rapid/feed/plunge limits, spindle/RPM, tool changes, safe Z,
parking, work zero, arcs and controller/software assumptions. Invent no values.

M0–M4 exclude waterjet implementation, nesting, multiple sheets, full CAD, PDF
import, inlays, advanced 3D/adaptive/rest machining, plugin architecture, major
frontend rewrite and direct machine control unless explicitly authorized.

After M4, milling product tracks can address setup/warnings/tabs/ramps and UX,
then dogbones/presets/templates/CAD/job sheets, then advanced workflows.
Future ProtoMAX milestones remain deferred: M5 architecture, M6 semantics,
M7 OMX post, M8 corpus, M9 MAKE validation, M10 physical validation.

## Git policy

Preserve upstream history. `origin` is the user fork; `upstream` is
`aalarchiv/ivacam`. Work on `feature/shopbot-mvp`. No force-push, automatic
default-branch merge or PR creation. The user's M0 instruction authorizes fork,
remotes, branch, baseline documentation, commits and push. After M0, commits and
pushes require explicit authorization again. These instructions override the
upstream agent files' local-only/main-branch assumptions.
