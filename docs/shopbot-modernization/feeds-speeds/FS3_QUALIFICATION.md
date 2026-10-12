# FS3 — CAM integration and UX qualification

2026-10-12. FS2 was approved and FS3 explicitly authorized by the user.
Engineering handoff; human FS3 approval remains pending. No FS4 work or physical
cutting qualification is included. Changes remain uncommitted on feature/shopbot-mvp.

## Delivered behavior

The existing milling Profile/Pocket panel now offers explicit dataset loading,
Machine → Material → Cutter → Operation → Calculate → Apply. Calculate is a
preview; Apply atomically saves the operation and identity bindings through the
existing undo history. Source/evidence and aggressiveness are distinct. Source,
license and versions are available in an advanced disclosure. English/German
translations follow the existing application locales.

Only the six licensed FS2 reference profiles ship. Their LongMill source identity
must be explicitly selected; they do not describe the lab ShopBot. Unknown
profile engagement and unsupported pockets return unavailable. The Profile
checkbox explicitly asserts full-width slotting; geometry does not infer it.
The supplied fixture is a SOFTWARE scenario using source reference identities,
not a machining file or local machine/tool equivalence assertion.

## Ownership and persistence

`ivac-core/src/feeds_speeds.rs` adapts current tool geometry, native depth
schedule, overlap, entry, stock thickness, identities and known physical limits.
Explicit depth lists use their maximum incremental DOC. An Apply that would
contradict an explicit schedule is refused. Pocket overlap is the complement
of stepover. No cutting constants were added to CAM algorithms.

Operation AppliedCutting stores complete positive rate slots per tool/pass,
origins, accepted context and full recommendation/provenance. Native step and
pocket overlap remain the sole geometry authorities. Profile consumes finish
rates; Pocket applies a rough proposal and snapshots other tool/pass rates as
legacy values. Reapply preserves existing secondary manual roles; changed tools
require complete new roles. Missing saved roles refuse generation without
falling back to a changed tool library. Legacy projects retain tool inheritance.

Manual RPM/feed/plunge edits update saved slots immediately and mark origins
manual. Geometry/DOC/overlap edits mark geometry origins manual and context
stale. Changes in tool, machine, material or operation context warn; they never
recalculate values automatically. Known cutter maxima are intersected with
machine maxima, including on explicit rebinding. Existing generation clamps
still protect hard limits and may differ from the requested saved values.

Portable FS-bearing projects use `kind: ivac-project-fs`, version 2. Legacy
projects without FS fields keep `kind: ivac-project`, version 1. The distinct
kind makes old readers reject new projects (old readers ignored version alone).
Unsupported formats and malformed saved rate roles are refused before clearing
an open project. Reopening requires no dataset or engine recalculation.

## Runtime boundary

HTTP POST /feeds-speeds, Tauri feeds_speeds_cmd and WASM feedsSpeeds expose the
same request/response schema. Only deliberate Calculate/Apply calls load data
and invoke the pure engine. Native posts, CPS, tool changes and motion emission
consume saved rates. CPS section metadata uses resolved active-pass rates;
recorder plunge classification follows actual pass changes. Applied/context
projects bypass the existing toolpath cache pending a complete context key;
this trades caching performance for correct current limits/warnings.

The independent engine gains optional JSON Schema derives only; its default
normal dependencies remain Serde/serde_json. Non-milling processes cannot use
applied milling data. Waterjet and drag-knife do not depend on recommendations.

## Software evidence

- Serial all-features Rust workspace, excluding Tauri: **1,218 passed**, including native
  and ShopBot/CPS golden regressions. Final adapter additions and recorder test
  also passed independently after that broad run.
- Nine CAM adapter tests: preview purity, explicit Apply, native serialization,
  manual persistence without data, inherited legacy defaults, invalid/missing
  roles and process rejection, context/depth schedules, native generated rates,
  CPS resolved limits, explicit changed-tool reapply and pocket rough/finish
  ownership. One additional recorder active-plunge test passed.
- Engine: 38 synthetic cases, 11 dataset cases, one API doctest pass.
- Frontend: 1,056 pass, one optional skip, 109 files. Ten new state cases cover
  format migration/guards, malformed rate roles, manual geometry/rates,
  serialized reopen, staleness, atomic undo, dataset catalog and known maxima.
- Rust Clippy warnings denied; schema drift check; Svelte/TypeScript; formatting;
  targeted ESLint; frontend/WASM release build and native server/Tauri command
  compilation all pass. Svelte/TypeScript reports zero errors/warnings;
  targeted ESLint and Prettier pass; Git whitespace and cargo fmt pass.
- Real in-app browser software review: fixture open, unknown-engagement refusal,
  explicit slot confirmation, 17,000 RPM / 1,620 feed / 810 plunge / 2.8 DOC
  preview and explicit Apply. Saved project uses the guarded FS format.
  Manual finish feed 1,500 survives saved v2 reopen without dataset; actual
  downloaded 37-line NGC contains S17000, F1500 and F810. Input edits commit
  immediately, fixing a change-event-only persistence gap found during review.
  Operator staleness/undo steps remain part of the human protocol.

Browser review identified and fixed ignored WASM imports that left a bare module
specifier in release output, and disclosure state that collapsed during edits.
No native desktop bundle was delivered in FS3; the M4-r2 desktop app remains the
previous machine-qualified application. Native transports compile; browser
WASM exercises actual FS3 code. Windows/Linux and physical calibration remain
unverified.

## Human gate

Use [the software protocol](FS3_TEST_LOGICIEL.md) and
[fixture](FS3-software-fixture.ivac-project.json). Record operator observations
before approving FS3. Approval permits separately scoped FS4 preparation;
physical trials and locally Tested/Validated presets require their own evidence.
No new commit, push, PR or upstream synchronization is authorized.

Engineering ivac-v1k1 closed; human acceptance is tracked in ivac-vbaa.
The actual browser-saved manual project is retained as
[software evidence](FS3-software-manual-saved.ivac-project.json).
