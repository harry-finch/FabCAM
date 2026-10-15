> 2026-10-14 continuation: user explicitly requested UX4; ivac-k4z3 accepted.
> UX3 evidence limits remain. Current next action is UX4 human review ivac-51gg.

# UX3 — Planar spoilboard surfacing

2026-10-14 · engineering ivac-gepq · human gate **ivac-k4z3**.
Implemented locally following the user's request to launch the next milestone.
CAD1 continuation accepted under ivac-7r7r; its geometry/evidence limits remain.
No commit, publication, native installer or physical surfacing qualification.

## Scope and source audit

| Existing seam | Disposition |
| --- | --- |
| `cam/surface_mill.rs`, ReliefMill | KEEP for ball/bull-nose 3D relief; no fake image source |
| `gcode/face_mill_overlay.rs` | KEEP existing decorative path overlay; not a surfacing zone |
| `project/op.rs`, specialty dispatcher | EXTEND with explicit PlanarSurface and isolated driver |
| `gcode::emit_vcarve_block` | KEEP shared XYZ emission, saved rates and safe-Z transitions |
| Work origin, final output preview, fixtures | EXTEND rectangle translation and scoped diagnostics; preserve final-program parity |
| `computeFootprint`, build-project, history, project lifecycle | EXTEND standalone authored sizing, explicit stock snapshot, guarded save/reopen |
| FS engine/data/packs | KEEP frozen; existing manual-rate snapshot only, no surfacing recommendation |

[Architecture decision](adr/ADR-0014-explicit-planar-surfacing.md).
The zone is operation-owned drawing XY in **mm**, independent of current drawing
selection and later stock edits. Copy current stock zone is an explicit undoable
snapshot. Physical tool size uses effective diameter (including valid wear).

Margin enlarges the target on all sides. Extra overtravel adds to the compulsory
cutter-radius extension at the along-scan endpoints. The transverse first/last
centers lie on the target edges; the cutter footprint extends beyond them too.
The selected 2D operation displays the target and dashed full cutter envelope;
Edges & travel reports separate center/cutter bounds. Generated 3D shows the
actual final path and stock. This is not fixture avoidance or automatic zeroing.

## Qualified V1 policy

- Flat endmill or compression cutter; no Whirl overlay. Manual cutting values
  can be edited and persisted; no inherited Pocket recommendation evidence.
- X or Y scan; distributed spacing at most the explicit stepover; driver
  stepover at most 95% effective diameter, retaining overlap.
- Z=0 at the material surface, start depth 0; removal/stepdown negative on the
  wire and displayed as positive mm. Retract to configured positive safe Z
  before every XY scanline transfer. Final pass reaches exact requested depth.
- Positive finite configured machine XYZ work area; resolved center travel,
  negative depth and positive safe Z checked against those declared extents.
  These checks do not establish actual machine travel or physical zero.
- Declared flute reach respected; removal must remain above the declared stock
  bottom. Single-sided/front only. No nonzero stock top, numeric Z translation,
  non-direct entry, secondary cutter, explicit depth list or through extension.
  HPGL plotting and plot-mode Z are refused; an XYZ milling post is required.
- At most 1000 depth passes and 100000 total scanlines. Invalid or excessive
  parameters refuse generation before cache replay/output.

Planned planar XY edge travel is a visible warning, rather than generic critical
`out_of_stock`. Z remains checked. Surfacing-only jobs use explicit travel checks
instead of the imported-contour work-zero/bbox heuristic; mixed contour jobs
retain that heuristic. Fixture collision remains critical. No general warning
severity, collision math or export override policy changed.

V6 (`ivac-project-surfacing`) prevents old applications silently losing these
operations. Legacy versions 1–5 retain their contracts; V6 can preserve CAD1 and
cutting provenance. No new tool database or workspace format.

## Software evidence

- 4 analytical planner tests: dense independent nearest-segment coverage,
  corners/edges, both directions, radius/margins/extra, alternating rows,
  tiny zones, malformed/nonfinite/overflow/excessive inputs.
- 8 `ux3_qualification` integration tests: decoded ShopBot coverage for both
  directions with fractional dimensions and XY offsets; final-output preview
  parity; depths −0.5/−1/−1.3 mm; all lateral rapids at safe Z; 17 refusal
  contexts; manual RPM/feed/plunge save/roundtrip and output; fixture on extended
  edge path; stock-bottom guard; generic output/disabled invalid op; independent
  contour work-origin warning retained.
- Existing CAD1 integration matrix passes (18 contexts in one test); existing
  27 ShopBot M2/UX1/UX2/CAM3/CAM4 regressions pass.
- Full core suite: **1074 passed**. Frontend: **1161 passed, 1 optional skip**. Targeted lifecycle/zone/CAD1 tests
  passed; manual values, no synthetic source contour, atomic Undo/Redo,
  guarded legacy rejection and actual v6 save/reopen are exercised.
- Svelte/TypeScript check and frontend ESLint/Prettier pass. Optimized WASM and
  production frontend build pass. CLI/WASM/Tauri transport compilation passes.
  Core/server all-targets Clippy and schema-check pass; 14 server tests pass.
  All 6 dedicated ShopBot post guard tests pass, including flat-only surfacing and retained
  3D-parallel rejection.

Commands use `--offline`, `--features cps`, and `--test-threads=1` for Rust.
Native target: `/private/tmp/fabcam-cps-runtime`; WASM build target:
`/private/tmp/fabcam-ux1-target`. `RUSTC_WRAPPER=''`, debug profiles 0 and
`CARGO_INCREMENTAL=0`. Node: `/opt/homebrew/opt/node@24/bin`.

```sh
cargo test -p ivac-core --features cps --offline --lib -- --test-threads=1
cargo test -p ivac-core --features cps --offline --test ux3_qualification --test shopbot_m2_post --test cad1_qualification -- --test-threads=1
npm --prefix frontend run test
npm --prefix frontend run check
npm --prefix frontend run lint
npm --prefix frontend run build
```

## Human review

Open [the software-only fixture](UX3-software-surfacing.ivac-project.json) in
the local [CPS-enabled review session](http://127.0.0.1:18767/?api=http%3A%2F%2F127.0.0.1%3A18772).
Its cutter/rates/machine values are test inputs, not physical cutting advice.

1. Select Planar surfacing. Inspect zone, flat cutter, removal, pass depth and
   manual rates. Expand Edges & travel and compare center/cutter extents.
2. Generate; inspect 2D/3D rows, edge travel and safe-Z lifts. Review the amber
   surfacing warning and normal manual-spindle ShopBot warning.
3. Change X/Y direction, zone, margin or overtravel; confirm stale output,
   Undo/Redo, regeneration and saved v6 reopen. Copy stock is a snapshot.
4. Enter an overlarge stepover or a zone whose extended center travel leaves
   the machine area; confirm explicit refusal. Declare an edge clamp and confirm
   critical collision/export safeguards remain.
5. Review the `.sbp` program offline. Controller preview and physical surfacing
   require a separate supervised qualification with actual setup/tool/machine
   evidence. No measured flatness, tram, spoilboard condition or workholding claim.

**HARD STOP before UX4.** UX3 software handoff awaits ivac-k4z3; independent
FS/UX/native/physical reviews remain unchanged.

### Browser evidence

Actual local browser session at 1280×900 verified standalone open/generate and
`.sbp` download with two noncritical warnings (planned edge travel and manual
spindle policy). X→Y plus feed1500 edits mark output stale and disable export.
Actual Save/download produces kind ivac-project-surfacing/version6; reopening
preserves Y and the saved manual feed. Stepover100 is explicitly rejected.
Browser Undo restores the invalid stepover to 2.4 mm and the saved document state;
Undo/Redo invariants are also established by the command/history tests.

- [Generated preview](ux/UX3-surfacing-preview.png)
- [2D target and cutter envelope](ux/UX3-zone-and-edges.png)
- [Stale after manual edit](ux/UX3-stale-edit.png)
- [Invalid stepover refusal](ux/UX3-invalid-stepover.png)
- [Downloaded software-only OpenSBP example](UX3-software-surfacing.sbp)

Browser review uses the CPS-enabled HTTP engine. Optimized standalone WASM has
no CPS runtime; native desktop packaging/controller/physical review is not claimed.
