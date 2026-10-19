# ADR-0025 follow-up — provenance false negatives and full regression qualification

Date: 2026-10-19. Tracking: **ivac-b0js3.1** (child of ivac-b0js3). Software
checkpoint on the uncommitted working tree based on `66f229e`; human review
pending. No commit, push, merge, tag, installer or publication; no controller or
physical qualification. Pipeline version unchanged (63): only provenance
authority/diagnostics changed, never emitted program bytes.

Inputs: [ADR-0025](../shopbot-modernization/adr/ADR-0025-movement-tool-provenance.md),
[Phase A](MULTI_TOOL_PROVENANCE_PHASE_A_2026_10_19.md),
[implementation results](MULTI_TOOL_PROVENANCE_RESULTS_2026_10_19.md) and an
independent Claude audit, all treated as hypotheses. Evidence:
[directory](adr0025-followup-2026-10-19-evidence/).

## 1. Findings

Reproduced through `POST /generate` on a release `ivac-server` built from the
pre-fix tree, replaying the TEST1 C1-0000 request under controlled variations
([`repro_matrix.py`](adr0025-followup-2026-10-19-evidence/repro_matrix.py),
[`before.json`](adr0025-followup-2026-10-19-evidence/before.json)).

| ID | Claim | Verdict | Evidence |
| --- | --- | --- | --- |
| A | Imperial diameters make bundled ShopBot provenance `unavailable` | **Confirmed** | 1/32, 3/32, 7/32, 9/32 in and a 6 mm + 3/32 in rough/finish job: `unavailable` (“Manual load instruction disagrees with planned tool”). 1/64, 3/64, 1/16, 1/8, 5/32, 3/16, 1/4, 1/2 in and metric controls resolve. Cause confirmed: Rust `{:.4}` rounds the exact binary value (2.38124999… → 2.3812); the CPS prelude rounds half away from zero with a nudge (2.3813). |
| B | Tool names containing G-code words cause false detection | **Confirmed, plus two acceptance gaps** | `Fraise filetage M6`, `M6`, `a;b M6` → `unavailable` (LinuxCNC and GRBL). The substring test also *missed* `M06`, and accepted LinuxCNC `(pause: swap to tool 2 (x) T5 (y))` although a first-`)` comment parser sees an executable `T5` there. |
| C | One-tool programs become planned-only with a profile or program-only op | **Partially confirmed** | Homing/Probe/raw include: downgrade is **necessary** — the program preview models neither `G28` nor a probe trigger, and raw text cannot be certified. Custom post profile: templates and axes can change semantics; format-only fields arguably cannot, but whitelisting needs a decision. `optional_stop` (M1): correct refusal. The generic diagnostic misattributed all of these to tool intent — **fixed**. |
| D | Native intents are “independently” verified | **Partially confirmed** | Manifest and markers come from one `tool_intent` call: a consistency check. The executable change envelope is checked independently. ADR wording corrected. |
| E | `comments: false` changes the contract | **Refuted as a change, wording issue confirmed** | Before ADR-0025 the header, `OP` markers and tool-change prompts were already unconditional; markers join them. UI hint “Disable for controllers that reject comments” overstates the option (human decision). |

## 2. Corrections

- **A** — [`shopbot_resolution.rs`](../../crates/ivac-core/src/pipeline/shopbot_resolution.rs):
  `load_instruction` strictly decodes `<number> D=<digits>.<4 digits>;`
  (finite, positive; malformed refuses) and `xyz_renders` accepts the planned
  diameter within the post's rounding bound: ½·10⁻⁴ mm plus 10⁻⁹ mm slack for
  the prelude nudge (< 10⁻⁹ mm below 1 m) and parsing. Tool number, section
  ordinal and project binding are still checked first; diameter agreement never
  establishes identity.
- **B** — [`provenance.rs`](../../crates/ivac-core/src/pipeline/provenance.rs):
  RS274 executable text (`(` comment closed by the first `)`, `;` outside
  parentheses, spaces/tabs insignificant), every letter-number word (`AT5`
  exposes `T5`). `T`, `M6`/`M06` and `M61` are tool-state words. ATC/M6 prompt
  requires exactly one `T<id> M6` line; manual changes need a standalone `M0`;
  manual/ignored envelopes and lines outside transitions refuse any tool-state
  word.
- **C/D** — explicit planned-only reasons in `ToolProvenance.diagnostics`;
  authority rules unchanged. ADR-0025 gains a “Verification scope” section.
- No change to post output, export policy, guards, schemas or WASM API.

## 3. Before / after

Same script on a release server built after the corrections
([`after.json`](adr0025-followup-2026-10-19-evidence/after.json)): 48 cases, 12
authority changes, all intended; zero differences in executable command digests
or emitted tool lines.

| Case | Before | After |
| --- | --- | --- |
| ShopBot single 1/32, 3/32, 7/32, 9/32 in | unavailable | resolved |
| ShopBot rough 6 mm + finish 3/32 in | unavailable | resolved |
| LinuxCNC/GRBL manual M0, names `Fraise filetage M6`, `M6`, `a;b M6` | unavailable | resolved |
| LinuxCNC manual M0, name `x) T5 (y` (executable `T5` after the first `)`) | resolved | **unavailable** |
| `Taraud M06`, `T2 special`, Unicode, parentheses (both posts) | resolved | resolved |
| GRBL ATC (post emits no `T<n> M6`) | planned | planned, with reason |
| Multi-tool `Ignore`; optional stop (M1) | planned | planned, with reason |
| Single tool + Homing / empty post profile | planned | planned, with reason |
| Plain single tool; `comments: false` | resolved | resolved |

Browser review ([`browser-review.json`](adr0025-followup-2026-10-19-evidence/browser-review.json)):
production `dist` over HTTP, pre- and post-fix release backends. A single-tool
3/32 in ShopBot project went from “Full-program verification unavailable” with
**Download .sbp disabled** to “Full-program simulation complete” with export
enabled, at Home (1/532) and End (532/532). The rough/finish 3/32 in ShopBot
project and the LinuxCNC `Fraise filetage M6` project reach “complete” at Home
and End; their export stays blocked by the existing critical dual-tool
manual-change warning (unchanged policy). The download itself was not triggered.

## 4. Test failure triage

HEAD baseline: a temporary detached worktree at `66f229e`, separate target dir.

| Test | Working tree before | At HEAD | Category | Action |
| --- | --- | --- | --- | --- |
| `shopbot_guards::lab_spindle_sequence…` | fail | pass | Metadata-only (`FABTOOL` comments) | Compare controller commands without `'` lines; pin markers around the reviewed spindle-1 sequence |
| `geom1_contract::ui_saved_repair…` | fail | pass | Metadata-only | Marker-free output byte-equal to the **unchanged** GEOM1 reference; markers pinned |
| `snapshots_drill_tabs_whirl::…drill…` | fail | pass | Metadata-only | Marker-free digest equals the historic `b296d632f465ec0e`; markers pinned |
| `ivac-wasm test1_bridge` (`--all-features` only) | fail | not run (cps-gated) | Metadata-only | Marker-free output byte-equal to the unchanged TEST1 references; markers pinned |
| `ux3_qualification::fixture_on_extended_edge…` | fail | **fail** | Outdated expectation | ADR-0023 (approved 2026-10-18) blocks publication on a detected collision; test now expects `FixtureCollision` with the warning in its diagnostic preview |
| `geometry_cache` length tests | fail in parallel | pass alone | Parallel interference | Cache type with private per-test instances; production keeps the global instances; no assertion removed |
| Clippy `examples/vcarve_audit.rs` | fail | fail | Existing unrelated lint | Inline format argument |

[`golden-semantics.json`](adr0025-followup-2026-10-19-evidence/golden-semantics.json):
the five `tests/fixtures/shopbot-m2/*.sbp` goldens refreshed by the ADR-0025
implementation equal their HEAD bytes once `FABTOOL/1` lines are removed.

## 5. Validation

Exact counts: [`validation-summary.json`](adr0025-followup-2026-10-19-evidence/validation-summary.json);
source hashes: [`source-state.json`](adr0025-followup-2026-10-19-evidence/source-state.json).

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --workspace --exclude ivac-tauri --all-targets [--all-features] -- -D warnings` | pass (both) |
| `cargo xtask schema-check` / `version-check` | pass / pass |
| `cargo test --workspace --locked` (parallel, default) | **1598 passed, 0 failed**, 49 binaries |
| `cargo test --workspace --exclude ivac-tauri --all-features` (CI) | **1593 passed, 0 failed**, 1 ignored (`test1_case`, harness-only) |
| `cargo test -p ivac-core --features cps --lib` ×3, parallel | 1208/1208 each |
| Frontend `eslint`, `prettier --check`, `svelte-check`, `tsc` | pass; 0 errors / 0 warnings |
| `node --test` + `vitest run` | 10/10; 159 files, 1438 passed, 1 existing skip |
| `scripts/test1/run.py --cases 32 --budget 120 --frontend` (release) | 256/256; lifecycle 32/32 |
| `scripts/test1/run.py --cases 256 --budget 300 --frontend` (release) | 1824/1824 in 210 s |

Included suites: tool provenance 12, SAFE-Z (hatch 14, extended 15), ShopBot M1 11,
M2 27, guards 8, CPS posts 5, V-Carve 10, GEOM1 23, UX3 8, snapshots 4, WASM CPS 24.
Every count above comes from parallel runs; no serial-only result is reported as
a parallel pass. TEST1 per-case artefacts stay outside the repository; summaries
with seeds (replayable) are kept.

## 6. Remaining limitations and blockers

- `scripts/pre-release.sh` pedantic Clippy gate is red: 67 errors, all in
  `ivac-feeds-speeds`/`ivac-cps` files untouched by this checkpoint.
- *Update ivac-4ql18: both items below are addressed in [comment safety and release gate](GCODE_COMMENT_SAFETY_RELEASE_GATE_2026_10_19.md).*
- **Pre-existing output defect, not fixed (out of scope):** LinuxCNC tool-change
  comments nest parentheses — `(pause: swap to tool 2 (<name>))`,
  `(toolchange: T2 (<name>) for op 1 (<op>))` — since upstream. LinuxCNC's
  G-code documentation disallows nested comments; confirm on a controller or
  simulator before deciding a fix.
- Tauri not built or tested (excluded, as in CI). No controller, installer or
  physical qualification. Imperial tools were checked in software only.
- Diagnostics are English-only, like the rest of core.

## 7. Human decisions needed

1. Accept marker-free comparisons against unchanged references, instead of
   refreshing goldens, for metadata-only differences.
2. Whether format-only post-profile fields may keep resolved provenance.
3. `comments: false` wording or contract (markers and prompts are always emitted).
4. Whether and when to fix the LinuxCNC nested-comment output defect.
5. Approval of this checkpoint; any commit remains separately authorized.

**HARD STOP — awaiting human review before starting the next correction campaign.**
