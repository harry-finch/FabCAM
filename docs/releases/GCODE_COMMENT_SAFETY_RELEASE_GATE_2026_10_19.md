# G-code comment safety and release gate — 2026-10-19

Tracking: **ivac-4ql18**. Follows the [ADR-0025 follow-up](ADR0025_FOLLOWUP_RESULTS_2026_10_19.md).
Software checkpoint; no installer, release, controller or hardware operation.
Committing this work does not accept any open review gate.

## 1. G-code comment safety

Defect (pre-existing since upstream): native posts wrapped user text in comments
without sanitizing it. LinuxCNC tool-change prompts nested parentheses —
`(pause: swap to tool 2 (Fraise filetage M6))` — which LinuxCNC documents as
invalid; a name such as `x) T5 (y` left `T5` executable for any parser that
closes a comment at the first `)`; newlines in tool, operation, group, pause or
cycle-marker text could start new executable lines; text beginning `MSG,`,
`LOGOPEN,`, `PROBEOPEN` … (or `py,` after `;`) would become a LinuxCNC active
comment.

Correction — one function, [`gcode::comment_text`](../../crates/ivac-core/src/gcode.rs),
per dialect style:

- control characters (including CR/LF/TAB) → space, in every style;
- `Paren` (LinuxCNC): `(`/`)` → `[`/`]`, so text can neither close nor nest;
- `Semicolon` (GRBL comments, `; OP`/group/cycle-marker/include headers):
  parentheses are inert and kept;
- LinuxCNC active-comment prefixes are defused with a leading `_`.

Applied in `linuxcnc::comment`, `grbl::comment` and the three raw `;` lines that
embed user text (cycle-marker label, group name, include path). The GRBL raw
rewriter no longer rewrites parentheses that already sit after a `;`, so it does
not split a sanitized comment into extra lines. Bundled ShopBot already reduces
comments to printable ASCII on one `'` line; generic CPS posts own their output.
Machine commands are unchanged: the full suites pass without any golden update,
and the new test compares every executable line with a benign control.

Regression tests: `comment_text_cannot_close_nest_or_break_a_comment` (unit),
`semicolon_comments_keep_their_inert_parentheses_on_one_line` (GRBL rewriter) and
[`tests/comment_safety.rs`](../../crates/ivac-core/tests/comment_safety.rs):
LinuxCNC and GRBL, tool/operation/group names, pause message and cycle label set
to `Fraise filetage M6`, `x) T5 (y`, `Fraise (M6) ((nested))`, newline-injected
`G0 Z-50`/`T9 M6`, `;`/CR/TAB, `)\n(MSG,boom`, `MSG,boom`, `py,import os`,
NUL/ESC and Unicode. Executable lines equal the control, provenance stays
`resolved`, no control characters, one balanced non-nested comment per LinuxCNC
comment line, no active comment.

`comments: false` wording (EN/DE): optional descriptive comments only; operator
instructions, operation markers and tool-provenance markers are always emitted.
The output contract is unchanged.

## 2. Release Clippy gate

The reported 67 pedantic errors were only the first failing crates
(`ivac-feeds-speeds`, `ivac-cps`); dependents were never compiled. Root cause:
`pre-release.sh` passes `-W clippy::pedantic` on the command line, which
overrides the documented `[workspace.lints]` allow-list (root `Cargo.toml`). The
script now re-applies exactly that list after the group flag. Under the
documented policy the workspace had 308 pedantic sites:

- machine-applicable fixes via `clippy --fix` and the JSON suggestions of
  behaviour-neutral lints (literal separators, `X::default()`, `clone_from`,
  explicit single variants, `#[must_use]`, `From` casts, closures, `;`);
- one identical match arm merged, `run_post_inner` borrows its cancel flag,
  `write!` instead of `push_str(&format!)`;
- the rest (exact float comparisons that are the contract, range-checked casts,
  owned Tauri/API arguments, a wire struct with flags) gets a **local**
  `#[allow]` on the function, struct or test module with a one-line reason.
  Integration tests that assert exact literals get a file-level allow. No global
  suppression; no tolerance was introduced where exact equality is behaviour.

## 3. Qualification (all parallel, current tree)

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | pass |
| Clippy strict, default and `--all-features` (`-D warnings`, CI) | pass |
| Pedantic gate as in `pre-release.sh` (incl. Tauri) | **pass** |
| `cargo xtask schema-check` / `version-check`; TS and i18n codegen drift | pass / pass; no drift |
| `cargo test --workspace` | **1601 passed, 0 failed** (50 binaries) |
| CI `--all-features --exclude ivac-tauri` | **1596 passed, 0 failed**, 1 ignored (`test1_case`, harness-only) |
| `cargo test -p ivac-tauri` | 13 passed |
| Frontend eslint, prettier, svelte-check, tsc, Node, vitest, production build | pass; 0/0; 10/10; 1438 passed / 1 skip; build OK |
| `wasm-pack build` + `wasm-size-guard.sh` | pass (cps adds 0 KiB) |
| TEST1 release `--cases 32` / `--cases 256` | 256/256 / 1824/1824; lifecycle 32/32 |

Named suites in the run: comment safety 1, tool provenance 12, SAFE-Z (hatch 14,
extended 15), ShopBot M1 11 / M2 27 / guards 8, CPS posts 5, V-Carve 10, GEOM1 23,
UX3 8.

**Not available or failing:** `cargo-deny` not installed (not run);
`scripts/check-readme-parity.sh` fails — README.md and README_de.md heading
structures differ, unchanged since `73dca6e` (pre-existing, not addressed);
`pre-release.sh` itself was not run end-to-end because `pnpm` is not on PATH
here — each of its steps was run individually above. No controller, installer
or physical qualification. This is not a release-readiness claim.
