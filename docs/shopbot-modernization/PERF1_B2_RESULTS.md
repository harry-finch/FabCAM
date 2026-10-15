# PERF1-B2 — Remaining-material kernel reuse

2026-10-15 · software qualification complete; human review pending · software only.

## Scope and invariants

User accepted B1 and authorized B2. The existing PipelineCache now owns an
independent four-entry / 8 MiB LRU of the pure planar subtraction results.
[ADR-0019](adr/ADR-0019-remaining-material-kernel-reuse.md) describes exact keys,
retention accounting, concurrency and cancellation limits.

Every eligibility guard still executes: execution representation, milling/units,
strategy, tool kind, tabs, allowance, source representation, move budget, target,
stock depth and XY envelope. Only qualified kernel work can hit. Result regions
are cloned as native coordinates; area/status/reason are freshly assembled.
No geometry tolerance, numerical algorithm, post, simulation or UI change.

Keys cover exact full ordered target regions/holes/IDs, returned movements,
cutter radius and plane depth. Unsupported operations are not cached. B1 CPS
reuse remains separate and unchanged. No deferred/background analysis is added.

## Reproduction

Control is retained B1 release cps+profiling executable, whose hash matches the
B1 evidence manifest. Candidate uses the same features/profile. Three fresh
process pairs, alternating order, seven calls per process; first is process-cold,
second warmup, last five warm (n=15/side). OS caches are not flushed. No benchmark
runs alongside compilation. Same original PERF1-A request bytes and hashes.

```
python3 scripts/perf1-paired.py --control /private/tmp/fabcam-perf1-b2/control --candidate target/release/examples/perf1_probe --requests docs/shopbot-modernization/perf1/requests --output docs/shopbot-modernization/perf1-b2/paired --work /private/tmp/fabcam-perf1-b2/paired
python3 scripts/perf1-native.py --binary target/release/examples/perf1_probe --requests docs/shopbot-modernization/perf1/requests --output docs/shopbot-modernization/perf1-b2/native-profile --work /private/tmp/fabcam-perf1-b2/profile --repeats 7
python3 scripts/perf1-b2-summary.py
```

The summary rejects mixed outcomes and mismatched requests/full-response hashes.
Initial work/new inputs and >4 distinct qualified pockets need computation;
there is no universal cold or multi-pocket speedup claim. Retention bounds exclude
allocator overhead and transient key/kernel/response memory. Peak process RSS is
not a long-session leak proof. Physical machining remains unqualified.

## Paired native results

| Case | B1 warm | B2 warm | B1 cold | B2 cold |
| --- | ---: | ---: | ---: | ---: |
| text-Test-medial-cps | 14.961 ms | 15.359 ms | 822.983 ms | 830.158 ms |
| profile-many-segments-cps | 8.340 ms | 8.007 ms | 1259.864 ms | 1247.191 ms |
| pocket-islands-cps | 338.370 ms | 0.625 ms | 409.988 ms | 413.449 ms |

Pocket remaining stage: **337.845→0.222 ms**, 99.93% reduction (target ≥50%).
Total repeat pipeline: **338.370→0.625 ms**, 99.82% reduction. Cold pocket
median409.988→413.449ms (+0.84%); no cold speedup claim. Text/profile differences
are small single-session variation, not intended B2 improvements.

Pocket process peak RSS medians 32.33→31.92 MiB; seven-call process CPU user+system medians 2.44→0.41 s. These are process measurements, not allocation counts or proof of long-session behavior.

All18 native cases repeated7times preserve B1 full response hashes (14 returning,
including2 empty-source diagnostic responses) or exact refusal (4). Analysis,
program, warnings, preview and indices are included in the response identity.
[Paired raw summary](perf1-b2/PAIRED_SUMMARY.json),
[output checks](perf1-b2/output-comparisons.json).

## Validation

Core release cps+profiling:1085passed, serial for existing global-cache test
constraints. Includes four new B2 tests: bounded exact LRU/clear/oversize, concurrent
native-coordinate preservation, kernel-input changes, full pipeline stock/tool/
depth invalidation plus disabled-operation refusal. Existing analytical remaining
material, Halfpipe and V-Carve regressions pass. Integration: CAD1 1, ShopBot27,
V-Carve10 passed. Clippy with warnings denied, Rustfmt, full offline WASM/frontend
and release native builds pass. Existing frontend chunk/import warnings remain.
No frontend production code changed; no new UI feature or physical qualification.

Direct WASM binding qualification:9 LinuxCNC cases ×7 calls;7 returning cases
have identical programs and remaining-material structures versus native, with
stable repeats;2 refusal series remain refusals. On the many-segment arc fixture,
cross-runtime interpolated XYZ differs by at most2.8422e-14mm (the retained A WASM
movement structure is identical). This is recorded explicitly, not described as
bit-identical cross-runtime motion. No key or machining tolerance is relaxed.
Reproduce via `PATH=/opt/homebrew/opt/node@24/bin:$PATH python3 scripts/perf1-b2-wasm-check.py`.
[WASM comparisons](perf1-b2/wasm/COMPARISONS.json).

## Actual browser and UX checks

Existing CAM4 square-pocket fixture (38 moves), not the heavier islands benchmark;
release frontend18795, retained B1 API18796 versus B2 API18797, 1280×720.
Three serial generations per side; last two are smoke repeat observations only.
HTTP response medians30.05→29.20ms, first3D frame64.05→63.65ms, rendered simulation
1124.85→1121.00ms. This small fixture shows no material end-to-end gain; the
large native islands-stage result must not be presented as a universal UI gain.
[Raw observations and summary](perf1-b2/BROWSER_SUMMARY.json).

Both versions show4remaining regions,30.99mm²,Z−2mm. Changing stock thickness
10→1mm immediately marks analysis stale and disables stale export. Regeneration
reports Not analyzed and the explicit stock-top/depth-within-stock requirement,
plus a critical stock warning. The existing export policy after fresh generation
is unchanged; Not analyzed is not a new blanket export block. Undo restores10mm;
regeneration restores4regions,30.99mm². Overlay checked visually; original saved
fixture unchanged. [Final overlay](perf1-b2/PERF1-B2-pocket.png),
[unsupported diagnostic](perf1-b2/UI-stock-diagnostic.txt).

The first temporary control tab retained an old dynamic-import URL across the
frontend rebuild and failed to load3D. Both test tabs were recreated after the
build; that setup failure is excluded from final three-run logs. No production
asset-loading change is included here.

## Handoff

Engineering ivac-dlrm complete. **Human results gate ivac-s5xy OPEN — HARD STOP.**
No next optimization, VM rewrite, commit/push or machine execution authorized.
Remaining independent follow-up: bounded CPS interruption ivac-sxmw; any further
simulation work needs new evidence/scope approval. No invented PERF1-B3 milestone.
