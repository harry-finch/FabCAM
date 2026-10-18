# Simulation and pipeline cache — audits 13–14

2026-10-18 · Engineering ivac-3mbco · Human review **ivac-97zve OPEN**.
Software-only checkpoint. Incoming source: `2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b`
plus uncommitted audits 09–12 (pipeline56). Those changes and evidence are preserved.
This lot advances the internal pipeline version to **57**; public version stays
**0.9.0-beta.2**. No project-format or installer identity change.

## Verified findings and disposition

| Audit item | Disposition |
| --- | --- |
| 13.1 Form-tool arcs remove overhangs | Confirmed. Arc carving now dispatches through CarveTarget: dexel targets remove the finite `eval_interval(r)` band, heightmaps retain top-down semantics. Full and split partial advancement tested. |
| 13.2 Arc collision checks use chords | Confirmed for fixtures and holder walls. Shared analytic XY arc footprint used for holder/rapid grid walks; fixture arcs enclosed by sagitta-bounded chord capsules. Ordinary emitted G0 is linear, but the rapid API also respects an arc descriptor if supplied. |
| 13.3 Smooth STL sampling offset | Confirmed. Shell height samples use cell centres, matching dexel samples. No change to the solid voxel exporter. |
| 13.4 Nominal/effective radius mismatch | Confirmed for wear compensation. Cutting exclusion uses effective diameter; physical shank fallback retains nominal diameter. Positive and negative wear tested. This does not establish a complete collision proof for every ball/V/form-tool body. |
| 13.5 Warning cache context | WASM already clears on new toolpath/fixtures and reconstructs on cell changes. Core cache now additionally verifies tool profile, holder, fixtures, grid layout and segment content before reuse. Reusing an index with a changed tool is tested. |
| 13.6 Rapid scan cost | Skip walks entirely above stock; use shared footprint walk and row pruning for linear shank motion instead of scanning the entire bounding box. No new performance SLA claimed. |
| 14.1 Operation cache dependencies | Confirmed for the native G-code operation cache. Include incoming XY, captured post state, spot-tool entry and a once-per-request hash of all classified objects (including topology). Serialization failure bypasses the cache. CPS/ShopBot does not use this operation-body cache; its separate whole-program protections remain unchanged. |
| 14.2 Streaming guards | Confirmed. Two-sided guard runs before any streaming output; streaming-with-preview now reports fixture warnings like buffered generation. |
| 14.3 Timing units/rates/dwell | Confirmed. G20 feeds normalized to mm/min, decimal commas accepted, modal drilling dwell counted, and rate caps derived from resolved overrides/applied cutting/finish/spot settings rather than library defaults. |
| 14.4 Tool count/warmup | Grouping and spot round trips accounted for; finish warmup uses the same eligibility predicate as internal tool swaps. Do not add envelope waits again: emitted G4/IR dwell already includes them. Pipeline warmup is reclassified out of dwell rather than double-counted. |
| 14.5 Included G-code classification | Compact words recognized; unsupported decimal/modal codes cannot be masked by a following supported G-code. Preview no longer turns G43.x/G92.x state changes or probing into fictitious cutting motion. |
| 14.6 Profiling reset after error | Not reproduced. RAII releases spans on Err and unwind; feature-enabled regression confirms reset works afterwards. Reset during a live span is a contract violation; forcing depth to zero would cause a later Drop underflow. Assertions retained. |

## Numerical and architecture limits

Fixture arc enclosure uses **0.001 mm maximum sagitta** and inflates each chord
capsule by that amount. This is a conservative collision test: near contacts
within the enclosure band may warn. It is not a CAM output tolerance or a
physical machine clearance. The stable asin formula avoids cancellation at
large radii; an empty fixture set returns immediately.

Grid collision and carving resolution is still the requested cell size. Tests
prove planar arc footprint behaviour, not a new continuous 3D collision solver
for arbitrary helical/form-tool/holder combinations. A heightmap does not model
undercut cavities; dexel finite intervals do. Smooth STL remains a sampled shell;
this change does not certify watertightness or exact outer-boundary coverage.

Warning reuse still assumes the same deterministic material-removal prefix.
Callers must clear it after replacing the field or changing earlier motions;
the context check is not a hash of every stock cell. Existing simulator lifecycle
resets are retained. Operation topology hashing deliberately invalidates more
than the selected objects when any classified geometry changes. This trades
some hit rate for correctness, with one topology serialization per request.

Timing remains an estimate. Cycle dwell counts each emitted/modal hole; it is
not a universal controller-dialect or repeated-peck timing qualification.
Structural tool-change counts can still exceed actual changes for operations
that ultimately emit no motion. Arbitrary included G-code remains explicitly
unqualified when unsupported commands are reported. No ShopBot capabilities,
Engrave/V-bit permissions, or safety post guards were broadened.

## Validation

Evidence: [checks.json](simulation-cache-audit-2026-10-18-evidence/checks.json).
Three pre-fix counterexamples failed: form-arc overhang, inch/comma feed and
compact/modal dwell. Targeted tests additionally cover arc fixtures in both
directions (box/cylinder/polygon), holder arc versus chord, effective/shank radii,
STL coordinates, warning context, operation key dependencies, streaming guards,
grouped/spot counts, warmup single counting and preview classification.

Final results: **1,202 core tests PASS** (serial), **15 focused tests PASS** with
profiling, **101 integration cases reported PASS / 100 executed** (optional FANUC
oracle absent). Clippy, formatting, diff checks and host compilation of ivac-wasm
PASS. Exact commands are recorded in checks.json and raw logs.
The stable-hash fixture changes solely because pipeline56 becomes pipeline57;
its earlier mismatch is retained as evidence. Frontend sources are unchanged
in this lot; no new UI or browser acceptance is claimed. The incoming frontend
qualification belongs to the separate audits 11–12 report.

## Handoff / HARD STOP

Await human review **ivac-97zve**. No commit, push, package, tag or publication.
Existing desktop candidates from run39053047404 remain **pipeline54** and do
not contain these fixes. Clean macOS installation, Windows/Ubuntu acceptance
and physical machining remain separate open gates. Reviews ivac-7y9ft and
ivac-80ewz remain open. VINYL ivac-wgovg stays deferred by explicit user decision.
