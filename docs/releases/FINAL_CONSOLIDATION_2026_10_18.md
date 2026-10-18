# Final SAFE-Z / TEST1 consolidation — 2026-10-18

FabCAM 0.9.0-beta.2, source pipeline61, local uncommitted tree based on
`2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b`. Existing installers: pipeline54.
Implementation task ivac-tca4h; human consolidation review **ivac-l6ouh OPEN**.
ADR-0023 review ivac-8371y remains distinct: permission to consolidate is not a
record of the suggested manual UX exercise. No commit, push, package or publication.

## Conclusion and proposed disposition

The bounded software evidence supports preparing new candidates after human review.
No new supported-scope safety defect was established in this consolidation.
This is not universal SAFE-Z qualification, a complete-pocket proof, a performance
certification or release approval. The remaining unknowns below are explicit limits;
none is silently reclassified as safe. TEST1/SAFE-Z overall tickets remain open for
review of this bounded disposition, rather than an unspecified demand to prove every
possible machine/setup. No production code, thresholds or pipeline version changed.

## Contract-to-evidence matrix

| Contract | Evidence on current source | Qualified boundary / remaining gate |
|---|---|---|
| Z01 | `hatch_safe_z::supported_pocket_and_drill_keep_air_clearance`, decoded Profile transitions; TEST1 D2 | Bounded supported flat-tool jobs; no general previously-cleared-stock model. |
| Z02 | `fixture_frame_translation_and_cache_mutation_preserve_warning_semantics`; TEST1 A2 | Common XY translation and unchanged Z; not arbitrary machine-frame transformations. |
| Z03 | `shopbot_invalid_stock_clearance_and_existing_z_origin_refuse`; finite-input regressions | Numeric nonzero origin Z remains refused; no new origin convention. |
| Z04 | `known_stock_refuses_clearance_at_or_below_top`, stock-edit/cache and zero-byte stream tests | Known top18/rapid5 refuses regardless of work-envelope warning settings. Historical red case corrected. |
| Z05 | Positive-control decoded jobs, finite/equality/negative tests, custom rounding guard | Clearance must survive output precision; bounded air paths only, not unknown first movement. |
| Z06 | Sloped/constant-height analytic box/cylinder witnesses; complete/partial/flute-body cases; ADR-0023 buffered refusal | Conservative declared-body envelope, possible false positives. Missing dimensions remain unknown; unknown alone is not globally refused. No whole-machine or undeclared-fixture guarantee. |
| Z07 | Independently decoded two-tool transitions with5/8mm clearances; station/home-park refusals | Tested upward vertical retract continuing a cut, not arbitrary channel clearance. Initial axes unknown and controller zero/operator setup mandatory. Custom machine stations remain unsupported. |
| Z08 | Fixture edit/cache restoration; zero-byte raw streams; progress refusal; HTTP/native WASM/Tauri diagnostics; frontend ownership/freshness/export tests | Software adapters and local HTTP UX evidenced. Browser WASM and packaged runtime, including real save dialogs, remain platform tests. |

TEST1 A1/A2/B1/B2/C1/C2/D1/D2 retain the exact bounded generator/oracles documented
in [TEST1 results](TEST1_RESULTS_2026_10_18.md):256 cases, scale0.25,20,000 moves,
10s per native case,120s stage budget, no widened tolerances. D2's generated stock
cases are supplemented by the named fixed Z06–Z08 tests above; they are not claimed
to generate every fixture/transition configuration. B1/B2 target witnesses are not
complete area coverage; medial-axis sampling remains explicitly limited.
The local27.48s result does not establish CI-runner or desktop performance.

## Remaining reserves and release impact

### Large cascade pocket — ivac-53ky9

Fresh replay of the preserved B1/index6/seed4925316778106641894/scale1 case again
returns **20,167 moves**, failing the unchanged20,000 test budget in0.892s without
timeout. This is an expected reproduced failure, never a passing TEST1 case.
The check lives in `tests/test1_properties.rs::run` after successful generation:
**the application has no20,000-move refusal implied by this test**. The failure
precedes that case's geometric oracle; this replay therefore supplies no new
clearance proof. The historical18 expanded failures remain preserved.

Recommended disposition: keep the ticket open as a resource/performance reserve,
not a demonstrated safety defect and not a blocker to building test candidates.
Before release acceptance, include this frozen pocket in representative desktop
performance/runtime checks (generation, preview, cancel, repeated runs and memory).
A freeze, OOM, unusable essential flow, wrong output or partial export makes it a
release blocker. Do not silently expand TEST1, simplify output, relax its budget or
promise a universal size limit. Broader optimization requires its own checkpoint.

### Relief/Waterline cancellation — ivac-42ylx

Source inspection confirms cooperative checks in drop-cutter kernel/rows/cells and
waterline levels/triangle traversal, but kernel sorting, stitching and Boolean
clearing have uninterruptible spans. Pre-cancel tests cannot prove end-to-end p95.
No new timing campaign was run here, and historical CPS latency is not substituted
for these workloads. **<=200ms p95 remains UNQUALIFIED**.

The existing [V1 performance policy](FABCAM_V1_SCOPE.md#performance-acceptance-proposal)
makes faster cancellation conditional unless an essential flow freezes; discarding
cancelled/stale/partial results remains mandatory. Recommended disposition: retain
this open measurement reserve for representative packaged-runtime checks; it does
not by itself prevent candidate preparation. A supported-flow freeze or valid partial
result is blocking. This is not permission to advertise the200ms target or remove it.

## Verification and source reference

[Evidence](final-consolidation-2026-10-18-evidence/integrity.json): all27 recorded
ADR-0023 source hashes match; archive manifests ADR-0023 14/14, TEST1 21/21 and
SAFE-Z extended18/18 match their bytes. HATCH's older directory has no archive
manifest and is not claimed checksum-verified. Its focused tests were rerun.

Fresh checks: **14 HATCH/clearance +15 SAFE-Z PASS**, including4 shared oracle
self-checks in the latter count. Command:

```sh
RUSTC_WRAPPER='' cargo test --offline --release -p ivac-core --features cps,profiling --test safe_z_extended --test hatch_safe_z -- --test-threads=1
CARGO_NET_OFFLINE=true python3 scripts/test1/build.py
python3 scripts/test1/run.py --binary <built executable> --output <fresh directory> --replay docs/releases/test1-2026-10-18-evidence/budget-counterexample.json
```

The last command is expected to exit1 with `move_budget:actual=20167`; raw replay
and its summary are retained, not merged into the passing campaign. The previously
recorded full regression results (1204 core,1367 frontend +10 packaging,256 TEST1,
adapters and CPS) are **inherited evidence**, not rerun totals for this checkpoint.
No production changes justify a wholesale rerun. New source edits invalidate the
relevant reference and require affected tests again.

`source-SHA256SUMS.txt` fingerprints940 source/build/test files (including untracked
implementation files), excludes docs/tracker, digest
`8afa22e76187a1c31d2acbfc2f8df35bd6eceeb40f95d548bd98a47dba0b63a6`.
This is a review reference, not an immutable commit or a packaged artifact identity.
Prior evidence records only27 selected source files; the broader fingerprint created
here does not retroactively prove all940 files were hashed at the prior run.

## Handoff / gates

1. Human review ivac-l6ouh: approve bounded software disposition and conditional
   reserves; resolve ADR-0023 UX review ivac-8371y explicitly. Suggested manual check:
   collision blocks output but keeps preview; move fixture and regenerate succeeds;
   Undo and regenerate blocks again. No machine operation needed.
2. Only after separate authorization: commit/push the reviewed tree and dispatch
   three candidate builds. Verify CI, source identity and package SHA-256; pipeline54
   installers cannot qualify pipeline61.
3. Clean installation/runtime on M3/macOS14.6, Windows11 x64 and actual Ubuntu amd64
   version recorded by tester; browser download/Gatekeeper Open Anyway on macOS,
   preserved data/save/reopen/generate/preview/export and representative performance.
   Ad-hoc signing retained; Developer ID/notarization optional post-V1.
4. Supervised ShopBot protocol and remaining GEOM1/FS human UX evidence, then RC
   supported/refused matrix and explicit release go/no-go. Publication is separate.

**HARD STOP.** No platform, physical or release gate is closed by this report.
