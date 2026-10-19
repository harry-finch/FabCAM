# ADR-0025 — movement tool provenance implementation

> Follow-up 2026-10-19 (ivac-b0js3.1): [false negatives and full regression](ADR0025_FOLLOWUP_RESULTS_2026_10_19.md). The totals below came from selected suites; `cargo test --workspace` was not run, and six test binaries were red across the default and CI `--all-features` runs (metadata-only `FABTOOL` differences, an outdated ADR-0023 expectation, cache-test interference). Corrected there.

Date: 2026-10-19. Software checkpoint; human review pending. No commit, push,
package, publication or physical qualification. Pipeline 63, project format unchanged.
Baseline and pre-implementation observations: [Phase A](MULTI_TOOL_PROVENANCE_PHASE_A_2026_10_19.md).
Evidence: [directory](tool-provenance-implementation-evidence/).

## Phased changes

1. Added nullable nonzero result-local movement `tool_ref`, immutable full ToolEntry
   bindings, separate project/program identities, authority and sparse transitions.
   Regenerated OpenAPI/TypeScript; pipeline63 and full-sim-2 invalidate prior results.
   WASM consumers require `movement-tool-provenance-1`. Legacy absent references
   remain unknown; duplicate or contradictory identities cannot qualify.
2. Native owned posts record typed planned intents independently of emitted comments.
   The adapter compares the complete ordered manifest and validates the actual tool
   change envelope. Per-operation cache replay carries the independent manifest as
   well as text; this is correctness metadata, not a cache optimization.
   Initial preload is an explicit assumption. Outgoing retracts retain the outgoing
   tool until activation. Numbered native lines retain physical decoder line indices.
   CPS records project identity separately from program tool number. Bundled ShopBot
   checks section ordinal, load instruction, matching tool/diameter and mandatory
   pause against recorded sections, then stamps the actual decoded output. Other
   CPS stays planned-only. Lab single-tool, supported pair and command guards remain.
3. Playback, glyph and ADR-0024 full-program verification resolve generated immutable
   definitions by movement reference, never by operation/first/live project tool.
   WASM receives the complete original definition, including wear and holder.
   Auto grid uses the smallest referenced effective cutting diameter, with existing
   caps, manual overrides and coarsening semantics. Front/back reference namespaces
   remain separate. Unsupported back fixtures/dowels stay unavailable.
4. Integrated tests and browser review; no worker/chunk/serialization/algorithm
   optimization. Complete-program diagnostics remain independent of playback.

## Evidence and regression corrections

- Native rough Ø8 / finish Ø2 in one operation; spot/drill/chamfer with three refs;
  leading tool-less event; duplicate IDs; numbered output; A→B→A reuse and native
  ATC, ManualM6 and ManualM0 strategies; Ignore refuses complete qualification.
- Adversarial unit tests delete tool commands, complete native transition envelopes,
  ShopBot pauses/activation markers, or alter tool numbers/section ordinals. A valid
  control is required alongside each refusal. Source reserved-marker injection fails.
- Actual WASM counterexample uses a short Ø2 finishing body: it produces the expected
  holder contact and a narrower carved footprint. Substituting the rough Ø8 body
  misses that contact. Resolver tests cover wear, full geometry, synthetic op IDs,
  legacy/missing/duplicate/contradictory references and planned-only export refusal.
- Cold/warm public toolpaths and provenance match. Interpreter memo remains geometry
  only; its independent geometry/index equality check is retained.
- All five ShopBot golden programs differ only by structured provenance comments.
  Removing those comments reproduces the exact former bytes, including commands,
  feeds and coordinates: `golden-command-invariance.json`.
- Regression execution exposed and fixed preload numbering, missing compatibility
  capability in two test doubles, and independent intent replay on cache hits.
  No golden motion change or tolerance relaxation was accepted.

A parallel debug core run encountered shared geometry-cache counter interference
(two cache-count assertions); the prescribed serial core run passed all 1205 tests.
The final additional manual-envelope contradiction guard passed focused unit and
all eight multi-tool integration tests afterwards. A debug TEST1 attempt exceeded
the stage budget (no semantic failure); it is retained as non-acceptance evidence.
Use the optimized release TEST1 run for the unchanged bounded timing contract.

Final accepted totals: core1205, multi-tool integration8, frontend1438 +10 Node
(1 existing skip), WASM host22, TEST1 **256/256**, HATCH14, SAFE-Z15,
ShopBot M2 27, CPS5 and V-Carve10. Typecheck/lint/scoped Clippy/build pass.

Final commands and counts are recorded in `validation-summary.json` and log files.
TEST1 uses the existing bounded 256-case harness; SAFE-Z/HATCH, CPS, ShopBot M2,
V-Carve, full core/frontend, real WASM, typecheck, formatting and scoped Clippy are
preserved. All-target Clippy additionally encounters the pre-existing
`examples/vcarve_audit.rs` uninlined-format-args lint; that unrelated example was not
changed. Optional vendor FANUC fixtures remain absent, not physically qualified.

## Visual review

Local HTTP backend + production frontend in an isolated browser tab. Saved source
fixture `multi-tool-review.ivac-project.json` opens/regenerates with one rough/finish
operation and both tool IDs preserved. 171 movements, 306 native lines. Home at
1/171 and End at171/171 both show **Full-program simulation complete**; the report
retains the same existing manual-tool-change warnings. Complete does not mean safe
or ready to machine: existing warning severity/export policy remains authoritative.
The Ignore fixture shows **Full-program verification unavailable**, not completion.
Screenshots and accessibility reports are retained. This was a narrow responsive
browser viewport, not a full desktop/macOS installer acceptance test. Rebuilding
assets during the session briefly required a page reload; final assets were reloaded.

## Qualification boundaries / handoff

- Manual pauses and preload declarations are procedural assumptions, never sensor
  evidence of the mounted cutter. No controller/tool changer support was added.
- Custom/template/raw programs, unsupported native change envelopes and generic CPS
  do not gain resolved authority. Missing provenance may show paths/planned preview
  but cannot satisfy an active complete-simulation export policy.
- Advanced two-sided fixture/dowel registration, arbitrary CPS interpretation and
  continuous swept-solid accuracy remain separate gates. Grid sampling and memory
  caps are unchanged. Three-tool identity tests do not certify every possible drill
  geometry or machine pairing; existing geometric suites supply bounded evidence.
- No large-job latency/RSS claim, performance optimization or hardware test. Test
  elapsed times are reproducibility observations, not benchmark acceptance.
- Existing human UX/release/platform gates and installers remain unchanged.

**HARD STOP — review this bounded implementation before any next checkpoint.**
