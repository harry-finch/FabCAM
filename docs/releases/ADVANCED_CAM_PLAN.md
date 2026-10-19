# Advanced CAM — V2+ scope refinement

2026-10-18 · **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED** for these promised complete
capabilities. Preserve V1/ProtoMAX1.5 priority and existing milestone IDs; no roadmap
entry enables inherited unsupported motion or expands qualified matrices.

| Milestone | Goal, reuse and required proof |
| --- | --- |
| REST1 — True 2D Rest Machining | **V2+ HIGH PRIORITY**. Refines existing Rest Machining entry, not duplicate functionality. Smaller-tool cleanup after roughing needs actual residual stock/depth, engagement/accessibility, tool geometry, islands/protected material, entry/exit and order. CAM4 diagnostics may inform but are not an operation or proof that passing residual polygons to Pocket is safe |
| HELIX1 — Helical Bore | **V2+ HIGH PRIORITY**. Dedicated endmill hole clearing: effective usable tool, bore diameter, radius/pitch/depth, entry/plunge/ramp limits, central residual and bottom finish. A peripheral helix does not clear every hole larger than the cutter. Current general Helix refusals remain until capability/output qualification |
| VCARVE2 — Flat-Bottom V-Carve | **V2+**. Complete bounded-depth text/sign/logo clearing combining V-bit boundary and central flat-area clearing, optional roughing tool and residual verification. Reuse qualified V-Carve where valid, independent envelope/pass/coverage tests and consistent preview/output. Distinct from paired inlay fit |

Each begins with architecture/capability audit, scoped input/topology/tool/stock domains,
independent geometric and removal/retention oracles, supported/refused matrix, bounded
computation, saved/manual-value/history/freshness compatibility, decoded emitted motion
and resolved-preview comparison. No new post support from naming an operation; ShopBot
helical/circular representation and linearized paths need actual qualification. Preserve
generic geometry/process/post boundaries. Required release/physical claims remain separate.

Open: residual representation and engagement proof, accessible cleanup domains, complete
bore coverage/pitch/entry, V-bit/roughing tool handoff and flat-bottom coverage, persistence/
UX/capability contracts. No algorithm, library, serialization or machine output chosen now.
Existing inlay and two-sided plans, GEOM/NEST and consolidation remain separately scheduled.

**HARD STOP — human scope/architecture approval before implementation.**

## REST1 refinement — True Area Rest-Machining (2026-10-19)

Existing REST1 identity/target retained, now scoped as A residual→candidate regions,
B safe smaller-tool operation, C guided UX. [Detailed phases/contracts](SMART_WORKSHOP_PLAN.md)
replace no existing qualified algorithm; diagnostic polygons alone remain insufficient.
[Unified V2 priority view](V2_WORKSHOP_PRIORITIES.md) includes STEP/SMART/FIT alongside
existing HELIX1 HIGH/VCARVE2 and other plans. Exact resource/order tradeoffs need human
approval; this is not implementation or silent demotion of prior high-priority work.
