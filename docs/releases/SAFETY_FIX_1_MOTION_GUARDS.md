# SAFETY-FIX-1 — motion containment

2026-11-01. Isolated `feature/safety-fix-1-motion-guards`, based on
`16eb34fd07354627b3c3c2f48a227b30ba1cc3c2`. Human acceptance covers bounded
software-qualified safety containment; three local commits are authorized.
No canonical integration, package, publication or hardware operation.

## Source and verdict

Final logical software inventory: **1,325 files**, SHA-256
`367d27b7864caa6a30fb35625e75530ac5024b2ef612303ac8ed1d3cf3f66b4e`.
[Per-file inventory](evidence/safety-fix1/software-files.json) uses the inherited
sorted `path + NUL + file SHA256 + LF` algorithm. Includes generated keys and
synthetic tests, excludes documentation, local evidence, caches and build assets.
The accepted snapshot began uncommitted on that base. Its final software commit
is `e09fdca4fe884af7ff5cec48d473c86b42829f6a`; all 1,325 logical Git files match
the accepted bytes, including resolution of 11 preserved icon symlinks.

**ACCEPTED as bounded software-qualified safety containment; locally consolidated.**
Acceptance does not requalify Helix, unrestricted two-sided machining or legacy
overcut. The final canonical gate passes; no installed/native-human, controller,
mechanical-holding or physical qualification is inferred.

## Reproduced baseline

Fresh executions used the actual baseline CLI, SHA-256
`070d0582bde6aaa14cca4e47c7c331a32b092048ae43f638a85f57896e2f53f2`.
Historical audit evidence remains untouched in the separate safety-audit worktree.
All inputs here are synthetic, not a user's machining project.

- **CAM-02:** central and displaced islands published native Pocket programs;
  independently sampled descending arc point `(28.690291,28.488515,-0.011364)`
  enters retained island material. Outside Profile published a retained-part
  witness `(23.531494,23.642202,-0.011364)`. ShopBot already refused helix.
- **CAM-06:** CLI published both files with empty Manual tabs. Front reaches
  `Z=-2` through 2 mm stock, without a bridge. Published Back glyph cuts occupy
  `X=23.1111…26.6667`, although a Y-axis flip requires `X>60`. Back approach
  `(10,10)` should become `(90,10)`; the emitted start `(56,27)` instead follows
  the untransformed approach. These checks decode actual published modal G-code.
- **CAM-04:** equivalent Inside CW/CCW overcut outputs differ: 44 versus 20
  decoded moves and minimum cutter-center X `10.7071` versus `11.0`. The ShopBot
  CW case also published. This demonstrates a winding-dependent contract, not a
  claim that every extra cut is wrong.

A further read-only review finding was reproduced on baseline: ascending Front
Thread with start `-3`, final `-0.5` and 2 mm stock published both programs and
actual cutting Z `-3`. The final guard includes synthesized start depth.

Raw baseline requests, executable outputs, decoded witnesses and logs stay under
`dist-test/safety-fix-1/`; they are not proposed for Git staging.

## Authoritative refusals

| Guard | Exact bounded restriction |
|---|---|
| Helical entry | Enabled contour-emitter operations with Helix, legacy Profile helix descent, or Trochoidal Pocket's implicit helix. No existing authoritative complete cutter-envelope proof exists; simple pockets are also refused. Thread's intrinsic machining helix is unaffected. |
| Legacy overcut | All enabled legacy Profile `overcut`, both windings and offset sides; explicit CAD relief geometry is unchanged. |
| Front holding | Through Front cuts require resolved, closed, single-path ordinary Profile, Direct entry, no leads or Finishing, MaterialV1 tabs with finite effective width and remaining thickness at least 0.05 mm. Authored mode/active flags alone do not qualify. |
| Combined holding | Overlapping additional Front cuts or unresolved independent placement/source invalidate separate bridge proofs. Additional Front leads, Thread retract envelopes and Drill spot prepasses are also unproven. Bounded disjoint sources remain possible. Text-dependent Front through-cuts are refused because pair preflight does not resolve rendered text. |
| Back inputs | Editable text selected by All/Objects or matching text layers; authored contour approach; Drill patterns; independent raster placement. Unknown object/text association is refused conservatively. GEOM1 edited double-sided generation is refused until qualified. Existing Back planar/heightfield restrictions remain. |
| Effective depth | Synthesized final AND start depth (including ascending Thread), Chamfer depth, explicit depth lists/through allowance, Drill spot and rim-chamfer reach. Halfpipe needs a finite cap strictly below `-1e-9`; V-Carve needs a finite negative cap. Unknown/invalid own-face thickness or nonzero stock-top frame is refused for genuine flip jobs. Existing planar/heightfield validation also remains authoritative. |

The 0.05 mm bridge criterion proves positive retained software geometry above
output precision; it does **not** certify mechanical holding strength. LegacyCenter
tabs, unresolved placements, zero auto counts, empty Manual placements and
multiple offset loops cannot supply that proof. No tabs are added automatically.

No geometry placement algorithm, schema, post capability, source intent or
parameter is changed. V10, `main_finishing_v1`, `pocket_finishing_v2`, ALPHA-2,
strict preset matching and ALPHA-4 advisory reuse retain their contracts.

## Boundaries and diagnostics

The shared buffered pipeline and stream preparation validate before executable
emission. The two-sided orchestrator validates the complete request before either
side is returned; CLI calls it before creating either file. Tauri generation and
transactional export use these core paths. HTTP and real WASM checks below exercise
actual adapters; this is not installed-Tauri or physical-machine qualification.

`PipelineError::MotionGuard` maps to the existing structured error taxonomy and
`safety_guard` metadata, without a serialized schema change. EN/DE diagnostics
explain the refusal and preserve intent. Safety errors serialize their metadata
as JSON-compatible objects in WASM so the worker's JSON channel retains the reason.
Helix help no longer promises an automatic fallback.

Raw HTTP can return structured 4xx for parameter-level errors before headers.
Geometry-dependent empty-holding failure may establish HTTP 200 then **abort with
zero executable bytes**. SSE similarly may establish 200 but emits error, never a
successful result/program. These are transport distinctions, not CNC success.
No transactional-export redesign is claimed.

## Qualification and review

- Focused Astra / Medium read-only review traced core, stream and pair guard
  placement. It found Halfpipe tiny-negative cap, omitted Front text, mutually
  destroyed tabs, deep-start Thread, long leads and external Thread retract
  envelopes. All six were corrected and rereviewed; no further concrete
  bypass was found within that narrow review. No reviewer builds or writes.
- Thirteen synthetic Rust regressions cover island displacement/narrow/simple
  pockets, Inside/Outside Profile, multiple steps, implicit helix, both windings,
  unresolved tabs, valid MaterialV1 holding, Back text/approach/pattern, shallow
  mirrored pairs, derived specialized/start depths and review findings. A positive
  disjoint Front pair remains supported; lead and secondary-envelope variants refuse.
- Old helix/fallback and Trochoidal integration expectations now assert intentional
  refusal. The historical unsafe snapshot digest is retained. No golden output
  was rewritten to disguise a change. Low-level geometric tests remain.
- **109 fixed CNC cases:** 96 accepted responses, including executable bytes,
  decoded motions, warnings and attribution, exactly match the baseline; all
  13 previous refusals match. Zero corpus golden changes or new corpus refusals.
  Separate unsafe audit cases now refuse intentionally.
  [Exact comparison](evidence/safety-fix1/runtime-comparison.json).
- Targeted CLI buffered/direct/pair outputs refuse unsafe requests. New output
  bases contain neither Front nor Back after pair refusal. Positive ordinary,
  Ramp and held-pair outputs remain byte-identical to baseline.
- HTTP buffered, pair, SSE and raw-stream unsafe replays produce no executable
  payload. Nine extra two-sided buffered/progress/direct checks confirm zero-byte
  abort for geometry-dependent raw holding failures.
- Real rebuilt web WASM rejects island/Profile helix, overcut and ineffective
  holding through buffered, pair and progress exports; ordinary Profile and
  shallow mirrored pairs still generate; ordinary Profile executable bytes and
  decoded moves also match native output exactly. Canonical web WASM has CPS disabled;
  ShopBot/CPS qualification is separate and native.
- Browser replay: built-in synthetic example, ordinary generation succeeds;
  selecting Helix retains that value and makes the old program stale; regeneration
  reports the new refusal; Save program refuses the stale old output. Local
  screenshots are `browser-helix-refusal.png` and
  `browser-stale-export-refusal.png` under `dist-test/safety-fix-1/`.
  EN/DE rendering tests pass. Installed macOS human workflows were not rerun.

Final canonical qualification: **20/20 mandatory groups PASS**, **1,675 Rust/Tauri
PASS / 0 FAIL / 1 intentional ignored worker**, **1,567 frontend PASS / 1 existing
optional skip**. The final stable source was gated once. See the
[qualification receipt](evidence/safety-fix1/qualification.json) for all groups,
final CLI/server/WASM SHA-256 identities and raw-log hashes.

The following [qualification-time preservation receipt](evidence/safety-fix1/preservation.json)
predates consolidation; its zero-commit statements describe that checkpoint.
The [acceptance/consolidation receipt](evidence/safety-fix1/consolidation.json)
records the subsequent authorized commits. At qualification, canonical V1.1's
1,318 qualified files retain fingerprint
`974de18341dd476e16699b0dcf9bf76bd5fdc6eaf5b4b70ba11aa5a01be5377d`;
canonical, frozen V1 and audit HEAD/status are unchanged. All 133 inventoried audit
evidence files match their preflight bytes. The implementation real index is empty.
All 1,325 final software files still match the frozen inventory after the gate.
No commit, merge, push, package or physical operation occurred.
The first 20/20 run predates the final start-depth/envelope corrections and is
retained as superseded evidence. Executable source changed afterward, so a second
canonical run on the final frozen fingerprint is required; this is not a repeated
unchanged-source release gate.
The canonical script itself is unchanged. Its generated-key drift comparison uses
an isolated temporary Git index containing only the reviewed generated `keys.ts`;
the real index remains empty. Regeneration must match those qualified bytes; the
drift guard is not disabled.

## Authorized local consolidation

1. `aac60647e3206bd27f5d30aae083fa634ca3e470` — `fix(cam): contain unproven entries and two-sided retention`.
   Nine production paths; only production hunks from shared `warnings.rs`.
2. `e09fdca4fe884af7ff5cec48d473c86b42829f6a` — `test(cam): qualify motion refusals and preserve accepted CNC output`.
   Eleven regression paths including the remaining test-only `warnings.rs` hunks.
   Depends on commit 1. All 1,325 indexed and committed logical files were compared
   with the accepted per-file SHA-256 inventory and working source bytes.
3. `docs(safety): record SAFETY-FIX-1 qualification and review gate` — this report,
   sanitized evidence and current STATUS/roadmap acceptance entries. Depends on
   both preceding commits; its final SHA is supplied in the Git consolidation
   receipt and resolves as the commit adding `evidence/safety-fix1/consolidation.json`.

The software fingerprint is unchanged. No full release-gate rerun was needed or
performed during consolidation. Raw logs, private data, local executable binaries,
caches, compiled frontend/WASM assets and historical audit evidence are excluded
from all three commits and remain locally preserved. Canonical V1.1, frozen V1
and the audit worktree retain their pre-consolidation HEADs and Git states.

## Follow-up and human gate

Re-enabling helix needs a full cutter-swept entry **and repositioning** proof.
Legacy overcut needs an accepted winding/retained-side contract. Additional holding
families need cumulative resolved-material proof; Back editable text, approach,
patterns, raster and edited geometry need complete coordinate transformation.
These are separate qualified milestones, not automatic follow-on work.

Software containment does not qualify a ShopBot controller, fixture, clamp,
operator pause, platform installation or physical cut. Preserve the documented
CPS restrictions and unresolved audit issues outside CAM-02/04/06.
**HARD STOP after accepted local consolidation. No next milestone is authorized.**
