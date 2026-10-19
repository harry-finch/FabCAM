# ADR-0025: Explicit movement tool provenance and emitted-intent verification

Status: **APPROVED by the user — 2026-10-19; implemented locally; checkpoint review pending**
Date: 2026-10-19
Scope: Phase A, ivac-b0js3; extends ADR-0024 without changing its execution boundary.

## Context

ToolpathSegment carries op identity but a single op can use rough/finish or
spot/drill/chamfer tools. Interactive simulation selects the primary tool;
complete-program verification currently refuses known ambiguous cases. Synthetic
operations and leading tool-less program events expose further lookup limits.
The owned CPS bridge retains tool number in sections, but preview extraction
loses it. Bundled ShopBot uses actual emitted motion and must retain that authority.
See [independent review and evidence](../../releases/MULTI_TOOL_PROVENANCE_PHASE_A_2026_10_19.md).

## Approved decision

KEEP existing planning, post interfaces, simulator and ADR-0024 worker. EXTEND
rather than replace them with:

1. A nullable typed result-local `tool_ref` per generated movement, plus a
   deterministic immutable binding table containing project ID, program number
   and full generation-time ToolEntry geometry. Unknown/missing is not a default
   tool. References are nonzero; project IDs remain separate and are not renamed.
2. Explicit planned-tool intent through existing post hooks/CPS section boundaries,
   including intra-operation changes. Unique execution section identity is separate
   from operation ID. No geometry, ordering or first-tool heuristic reconstruction.
3. Generated provenance authority (`planned`, `resolved`, `unavailable`), version
   and sparse transition evidence. Owned output adapters validate emitted intent
   against planned bindings before marking movements resolved. Bundled ShopBot
   still decodes final rounded output, with its current supported command subset.
4. A common resolver for playback and headless verification. Unknown or contradictory
   body/tool provenance prevents complete qualification, including rapid moves.
   Existing complete-program versus playback diagnostic separation is unchanged.

Project definition, planned tool, emitted request and mounted physical tool are
four distinct concepts. Software verifies the first three within supported
contracts; physical state remains unknown. Manual load instructions plus stops
are procedural assumptions, not sensor evidence. Structured comments alone never
prove a swap. Validate their order/binding and actual supported change envelope.
Do not qualify multiple planned tools under Ignore or a no-op post change.
Retraction before a change uses the outgoing tool; activation takes effect only
at the validated boundary. No-motion program events need no fabricated cutter.

Derive shared automatic resolution from referenced effective cutting diameters,
within existing manual settings/cell caps and explicit coarsening limitations.
This improves consistency but is not a continuous geometric accuracy guarantee.
Defer batch/CSR/serialization/worker performance refactors and advanced two-sided
qualification. Keep current ShopBot pairing restrictions, including V-bit drill
refusal and single-preloaded-tool lab mode.

## Alternatives considered

- Optional project `tool_id` plus immutable definitions and evidence: viable for
  current project-only tools, but more coupled to persisted IDs. Result-local
  binding is recommended for clear generated identity and geometry ownership.
- Bare `tool_id:u32=0` with op/previous/first-tool fallback: rejected; accepts
  ambiguous legacy traces and masks contradictory evidence.
- Output-only inference: inadequate for custom CPS/no-op change output; bounded
  parsers are still required to corroborate owned post intent.
- IR-only stamping: useful planned preview, insufficient final-output authority.
- Complete pipeline/event rewrite: unnecessary scope and upstream divergence.

## Consequences

Generated API/schema/TypeScript/WASM and constructors change together; pipeline,
preview/cache and qualification revisions must invalidate old generated results.
Bindings, actual output and simulation configuration belong to qualification
identity. Old wire records may deserialize but remain unverified. No source
project migration, new machine-control function or physical-tool assertion.
Reject duplicate IDs/unknown references before qualification rather than silently
rewriting projects. Custom posts may remain planned-only; communicate this
compatibility consequence explicitly. Old clients must not silently use new
provenance as if they understood it; require compatible runtime versions.

The change touches shared generic generated types and post hooks, creating a
bounded upstream merge cost. Tool-binding semantics stay machine-independent;
OpenSBP recognition remains solely in its post/adapter. No speculative waterjet
or knife behavior is added. Unsupported two-sided fixtures/dowels retain their
separate acceptance gate even after tool-reference representation is available.

## Verification scope (follow-up 2026-10-19, ivac-b0js3.1)

What each owned adapter actually proves; see the
[follow-up results](../../releases/ADR0025_FOLLOWUP_RESULTS_2026_10_19.md).

- **Native LinuxCNC/GRBL — consistency, not independence, for intents.** The
  typed manifest and the `FABTOOL/1` markers are recorded by the same
  `tool_intent` call. Their comparison proves the final text (including op-cache
  replay) kept every planned boundary in order; it does not prove the planner
  chose the right tool. **Independently checked**: the executable change
  envelope, read as RS274 words with `( … )` (first `)` closes) and `;`
  comments removed — exactly one `T<id> M6` for ATC/M6 prompt, a standalone
  `M0` for manual changes, no `T`/`M6`/`M61` word in manual/ignored envelopes or
  outside a transition — plus unknown/duplicate identities and reserved-marker
  injection. Tool and operation names only reach comments and never count as
  machine words.
- **Bundled ShopBot — independent decoding.** Planned identity comes from the
  Rust IR sections; the JS post's text is decoded separately: section ordinal,
  program tool number, load instruction, mandatory `PAUSE`, no motion before
  `ACTIVE`. The load-instruction diameter must equal the plan within the post's
  4-decimal half-away rounding (≤ 0.5·10⁻⁴ mm + 10⁻⁹ mm). That agreement is a
  consistency check and never establishes identity.
- **Generic CPS** stays planned-only.
- **`comments: false`** removes informational per-object comments only. The
  header, `OP` markers, operator tool-change prompts and the `FABTOOL/1`
  markers are always emitted; the markers are required metadata of owned
  native output and are not optional commentary. No output contract changed.
- **Planned-only reasons** are now explicit diagnostics (custom post profile,
  machine-coordinate moves, tool-length offsets, Homing/Probe/raw include,
  missing `T<id> M6`, missing standalone `M0`, ignored change). The preview
  models neither `G28` nor a probe trigger, so those programs must not resolve.

## Validation

The Phase A report defines phased implementation and semantic tests: rough/finish
and three-tool drill geometry, holders and outgoing retractions, malformed or
missing emitted changes, legacy/duplicate IDs, program-only events, synthetic
operations, independent OpenSBP decoding, cold/warm cache, resolution/resource
limits, stale worker results, Home/End export invariance and save/reopen.
Preserve TEST1, SAFE-Z, ShopBot M2, CPS and V-Carve suites. Benchmark before
accepting latency claims; tests cannot relax post guards to enlarge scope.

Implementation and bounded software evidence: [checkpoint report](../../releases/MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md).
Native emitted boundaries are compared to a separately recorded typed manifest,
including cache replay; comments cannot reconstruct or substitute for that manifest.

**HARD STOP: human review of this implementation is required before the next checkpoint.**
No commit, push, packaging, publication or hardware use authorized.
