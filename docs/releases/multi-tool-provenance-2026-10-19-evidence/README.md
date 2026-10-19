# Phase A evidence — 2026-10-19

Architecture inspection, not implementation or expanded qualification. Source:
HEAD66f229ea21cccf73a76fd45d0e9f9fe3721a41a1 plus existing dirty workspace,
beta.3/pipeline62. `source-before.json` records source/schema hashes for unchanged-source verification.

## Commands and outcomes

From `frontend` (Node24 in PATH):

```
./node_modules/.bin/vitest run src/lib/state/program-qualification.test.ts src/lib/sim/qualification.test.ts src/lib/sim/qualification-wasm.test.ts src/lib/services/qualification-export.test.ts
```

`frontend.log`: 4 files, **37 tests PASS**, including real Node/WASM bounded
verification. Its tiny-fixture elapsed time/heap report is not a large-job benchmark.

From repository root, for each filter below:

```
RUSTC_WRAPPER='' cargo test --offline --release -p ivac-core --features cps,profiling --lib FILTER
```

| FILTER | Log | Result |
|---|---|---|
| warning_cache_rechecks_changed_tool_at_same_index | cache.log | 1 PASS |
| lot7_dowels_refuse_bad_layout_or_invented_tools | dowels.log | 1 PASS |
| mid_section_toolchange_splits | ir.log | 1 PASS |
| dual_tool_pocket_emits_toolchange_and_uses_finish_tool_feed | dual.log | 1 PASS |
| drill_with_spot_first_emits_spot_pre_pass_block | spot.log | 1 PASS |

Read-only counterexample script, from root:

```
PATH=/opt/homebrew/opt/node@24/bin:$PATH node frontend/node_modules/.pnpm/vite-node@2.1.9_@types+node@24.12.2_lightningcss@1.32.0/node_modules/vite-node/vite-node.mjs --config frontend/vitest.config.ts docs/releases/multi-tool-provenance-2026-10-19-evidence/probe.ts
```

`probe.log`: assertions PASS, proving leading program-only tool lookup rejection,
unknown synthetic op lookup rejection and simulator cleanup; requested cell sizes
1.6933333333333331 vs0.1 mm. Mock simulator proves control flow only; it does not
prove holder contact or final output geometry. This is an evidence script outside
test discovery, not a modified qualification fixture. An initial invocation via
nonexistent `.bin/vite-node` failed; the resolved installed runner above succeeded.

No full tests/build/lint, rendered UI, native candidate, controller or physical
qualification claimed. No performance thresholds accepted. Existing broader
results remain in the ADR-0024 checkpoint; they were not rerun here.
