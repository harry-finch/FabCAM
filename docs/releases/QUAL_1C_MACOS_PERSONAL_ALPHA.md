# QUAL-1C — macOS Personal Alpha

Date: 2026-10-24. Scope: maintainer-only, software-only macOS evaluation.
No physical machining, public installer, release or cross-platform acceptance.

## Source and preservation

Starting source: `aeae8202b385e46fdee9ff5827f5ca05702d14c3`, the committed
`FabCAM-v1.1` HEAD at preflight. Implementation is QUAL-1B
`fd8fed652ab98ef801595d6cf12f378a8f28bf89`; the difference is qualification
reports/evidence. No executable production implementation change was made for QUAL-1C. Final build
source: `700de73a5cf5a378b8465e5f38950a61d6cce4c3` (bounded comment/test fixes below).
Isolated checkout: `/Users/mugen/Documents/01_Projets/FabCAM/FabCAM-qual-1c`,
branch `codex/qual-1c-personal-alpha`. Report commits do not change the built source.

The primary checkout contains unrelated CAM pocket/diagnostic, drawing placement,
header/style, translations, STATUS/roadmap changes and local evidence. They are
not silently included. A preflight SHA-256 inventory records 8,496 modified or
untracked files. No stash/reset/clean or unrelated staging occurred. Existing
worktrees and idle concurrent chats were inspected; no other agent was delegated.
The separate roadmap commit `ff6486b1795bb95994ad97d7cb799011e276fded` is not an
ancestor of this source. Its work and primary documentation remain untouched.
The newer dated QUAL-1A.4/1B reports supersede older status prose for their source.

## Host, build and artifact

Host: macOS 26.5.2 (25F84), arm64; target `aarch64-apple-darwin`.
CommandLineTools selected; full Xcode is absent, but the native build succeeded.
Rust/cargo 1.88.0; Node 24.21.0; pnpm 10.34.6; Tauri CLI 2.11.3;
locked Tauri Rust 2.11.0; wasm-pack 0.15.0; wasm-bindgen 0.2.120;
wasm-opt 117; cargo-deny 0.20.2. See toolchain receipt.

The first dependency install failed registry DNS in sandbox. An authorized host
retry installed the frozen lockfile successfully. The old temporary Tauri CLI
symlink was broken; CLI 2.11.3 was installed in a separate temporary tools folder,
without changing project dependencies. `RUSTC_WRAPPER=''` disables unavailable
sccache; no source/build-system workaround was introduced.

Commands from the isolated checkout (tool directories on PATH):

```sh
pnpm --dir frontend install --frozen-lockfile
cd crates/ivac-tauri
RUSTC_WRAPPER='' /private/tmp/fabcam-qual1c-tools/node_modules/.bin/tauri \
  build --config tauri.dev-signing.conf.json --bundles app -- --locked
```

Build **PASS**. The normal hook freshly compiled and verified WASM and built Vite
before compiling/bundling native Rust. The source tree was clean of tracked edits.
Initial frontend build label: `v0.4.0-119-gaeae820` (git-describe's historical tag,
not the application semantic version). Application version remains `0.9.0-beta.3`.
No version promotion, DMG, signing credentials or notarization request.

Bundle:
`/Users/mugen/Documents/01_Projets/FabCAM/FabCAM-qual-1c/target/release/bundle/macos/FabCAM.app`

Identity/format: FabCAM / `org.opcenter.ivacam`, Mach-O arm64.
`codesign --verify --deep --strict --verbose=2`: **PASS**, valid on disk and
satisfies designated requirement. Signature: **ad hoc**, hardened-runtime flag,
no TeamIdentifier, **not notarized**. This does not establish Gatekeeper or clean-host
acceptance. Nothing was installed over an existing app.

Initial executable SHA-256 (aeae820; retained separately):
`055ed0b137d1af9b6dbdfc7f8878c496591524d5521cb3fe9a1fddd0153c8626`.
Initial bundle file-manifest SHA-256:
`6458109bdc459940401a52dc37098f898218afd5264c6a62ece7668913cee8f9`.
[Initial manifest](qual-1c-evidence/initial-artifact-manifest.json) specifies the exact algorithm,
all bundle file hashes and absolute path. This is a directory-manifest digest,
not a fictitious installer checksum. Assets and build logs are recorded separately.

## Runtime data boundary and human decision

Existing `~/Library/Application Support/org.opcenter.ivacam` and
`~/Library/WebKit/org.opcenter.ivacam` directories were found without resetting
or inspecting private project/preset contents. No supported per-launch isolated
profile was found. Native workspace is `ivacam/workspace.json` beneath app data;
collections use `presets/collections`; recent files use `settings.json`; the
window-state plugin and WebKit also persist state. Startup alone can write.

The maintainer explicitly chose **a separate macOS test account** instead of
writes after backup in the usual account. Therefore the app was **NOT LAUNCHED**
in this session. No account was created, credentials requested, application-data
backup/reset performed or identity changed. This is a deliberate preservation
boundary, not a desktop startup PASS or an observed startup failure.

Source inspection confirms desktop transport selects native Tauri invoke; ordinary
CAM does not require an HTTP backend or FabSpeed instance. Actual native-engine
initialization remains NOT RUN. Browser `/api/import` availability is unrelated.

## Native authorization findings

Generated-manifest `scripts/check-preset-import-acl.mjs`: **PASS**. `fs:allow-stat`
grants exactly `fs::stat`, adds no path scope; the old ACL counterexample still
fails that grant. The collection importer calls stat before reading and refuses
files larger than 2 MiB. Cancel/read/stat errors do not commit a collection.
Frontend mock tests and native collection storage tests are independent evidence;
actual chooser authorization still requires the separate-account walkthrough.

Existing configured scopes include `$HOME/**` as well as Desktop/Documents/Downloads;
do not claim a chooser-only policy. No scope was broadened. Program publication
keeps canonical-path authorization and transactional semantics. Collection JSON
export uses the existing plugin text writer; do not extrapolate program publication
atomicity to collection exports or claim all failed writes preserve existing bytes.
Use fresh QA destinations and exercise failure cases before accepting that workflow.

## Desktop acceptance matrix

| Area | Actual desktop result | Remaining evidence |
|---|---|---|
| C1 launch/navigation/dialogs/resize/restart | NOT RUN | Separate test account |
| C2 geometry/stock/origin/tool/material/v9 real save-reopen | NOT RUN | Compare actual saved/reopened file and identities |
| C3 explicit roles/rates/limits | NOT RUN in desktop | Software regressions below; no defaults inferred |
| C4 preset create/copy/edit/delete/import/export/collision | NOT RUN in desktop | Real native dialog and file round-trip |
| C5 generation/simulation/report/fresh export | NOT RUN in desktop | Generated-to-saved program comparison and stale refusal |
| C6 native UI cancellation/recovery | NOT RUN in desktop | No inference from QUAL-1B p95 |
| D permission/size/malformed/cancel/failure/restart | NOT RUN in desktop | Static ACL and unit tests are not installed acceptance |
| Native file-change notification in UI | NOT RUN | Watcher unit gate reported separately |

The [test guide](QUAL_1C_MACOS_TEST_GUIDE.md) contains the exact bounded workflow.
No synthetic attestation was represented as a workshop test.

## Current advisory finding

Unlike the old sandbox failure, the host audit fetched a current RustSec database:
`21d909e4c8bd4e14c7aea58bf980c881a0053ab1`, commit timestamp
2026-10-24T10:30:13+02:00. Audit **FAIL**:
`RUSTSEC-2026-0386`, Tokio 1.52.2, unsound `AsyncFd` constructors, potential
I/O-safety violation/use-after-free. The configured bans, licenses and sources
checks pass; their warnings remain in the log.

The advisory lists patched ranges `>=1.51.6,<1.52.0` or `>=1.53.3`.
No direct AsyncFd reference was found in FabCAM crates. That alone does not prove
transitive unreachability, nor waive the configured gate. Tokio is in the native
Tauri dependency graph. No dependency update, advisory suppression or validator
change is included here. A focused dependency correction/requalification needs
human disposition; do not label this a sandbox-only or cosmetic failure.

## Source-bound software qualification

All commands used the isolated source and pinned lockfiles. The unchanged full
`scripts/pre-release.sh` was run with explicit host execution approval: this allows
FSEvents, current advisory fetch and disposable-profile Chrome/CDP. It does not
launch FabCAM or authorize writes to its usual profile.

| Regression | Result and scope |
|---|---|
| Workspace all-features Rust tests | **1,592 PASS, 0 FAIL, 1 intentionally ignored TEST1 worker**; 48 summaries including zero-example doc tests |
| Native Tauri | **15 PASS**, includes 5 watcher tests and 4 collection-storage tests |
| Watcher historical failures | `single_path_emits_event` and `debounce_collapses_rapid_writes` **PASS on host**; no changed assertions/timeouts or fallback |
| Frontend | **1,498 PASS**, 1 existing optional i18n skip, 172 files; package/freshness harness 12 PASS |
| Typecheck / lint | **PASS**, 0 Svelte errors/warnings; ESLint and Prettier pass |
| Real WASM | Fresh normal build and verified package fingerprint; actual WASM v9/adapter tests pass in frontend suite |
| TEST1 + C2 | **256/256 PASS**, 35.931 s; native 224 + frontend 32, unchanged 120 s stage / 10 s worker budget; no timeout |
| Fixed output corpus | **23/23 exact**: 22 buffered requests compare G-code, toolpath, provenance, line index and stats; two-sided case compares both program texts |
| JS prelude/CPS | **46 PASS** |
| ShopBot/CPS Rust | M1 **11**, M2 **31**, CPS **55** pass, including real nested-preflight cancellation/refusal and exact retry |
| Native transactional publication | **11 PASS**, including preflight preservation, cleanup warning and partial pair reporting |
| Native import ACL | **PASS** generated-manifest check and old-ACL counterexample; no installed dialog claim |

Commands beyond the unchanged full gate:

```sh
node scripts/check-preset-import-acl.mjs
node crates/ivac-cps/prelude/tests/run.mjs
node frontend/scripts/ensure-wasm-fresh.mjs
python3 scripts/test1/build.py > /private/tmp/fabcam-qual1c-evidence/test1-binary.txt
python3 scripts/test1/run.py --binary <the returned release worker> \
  --output /private/tmp/fabcam-qual1c-evidence/test1 --frontend
cargo build --locked -p ivac-cli
python3 scripts/qual1a4/corpus.py target/debug/ivac \
  /private/tmp/fabcam-qual1c-evidence/corpus
```

Timing is a software observation under concurrent build/test load, not an installed
GUI response guarantee. Earlier QUAL-1B's 86-response and timing campaigns are
retained source-identical evidence, not falsely reported as repeated here. No output
golden, tolerance, parser, safety rule or production test was changed. The optional
external DXF/reference/shim availability limits of the existing suite remain; this
checkpoint does not establish external CAD or hardware qualification.


## Bounded qualification corrections

Initial canonical gate: **17/20**. Besides the security advisory, strict pedantic
Clippy found a comment needing backticks and then an unchecked `usize as f64`
in the new cancellation fixture. Smoke expected six areas, although Presets makes
seven. Its first run also encountered an old listener on default port 4179;
that run is not trusted as candidate UI evidence. On dedicated ports, the unchanged
smoke reproduced the obsolete six-area failure against the actual built source.

Focused local commits:

- `2e6c021b3382ca03574eed12e649d1a9c6d92e0c`: format `ShopBot` in a Rust doc comment; use checked `u32::try_from`
  followed by `f64::from` in the bounded test fixture. No lint suppression and no
  production conversion change. Strict workspace/all-target/all-feature pedantic
  Clippy PASS; all 55 CPS tests PASS.
- `700de73a5cf5a378b8465e5f38950a61d6cce4c3`: smoke expects exactly seven main
  areas and explicitly walks Presets. Tool-table boot expectation updated too.
  No weakened assertion, skipped section or product UI change. Both canonical
  smoke scripts PASS with no page exceptions on isolated ports/profile.

The full gate script itself is unmodified. Its supported `UI_SMOKE_PORT` and
`UI_SMOKE_CDP_PORT` environment variables avoid unrelated existing listeners.
The existing smoke launcher can silently choose another Vite port if its default
is occupied; use dedicated free ports when reproducing this receipt. No unrelated
preview server was terminated. The final app is rebuilt after these commits.

## Final artifact

Final source: `700de73a5cf5a378b8465e5f38950a61d6cce4c3`.
Native build-script and frontend stamp: `v0.4.0-121-g700de73`, clean build.
The same normal Tauri command rebuilt the app after the qualification fixes.
Strict bundle signature verification **PASS**, still ad hoc and not notarized.
No launch occurred. Final artifact path is the `target/.../FabCAM.app` path above;
the initial app is separately retained in `dist-test/qual1c/initial-aeae820/`.

Final executable SHA-256:
`d5912408fb20f9a0eca01b129247c95fe66eba8dc067c4bf3302e4e0acff331c`.
Final directory file-manifest SHA-256:
`3efe72e4e46ad166a7fb29826924103af78f3511515e7d610749b8f2a1d6b905`.
[Final artifact manifest](qual-1c-evidence/artifact-manifest.json).

## Final canonical gate and evidence

Final `scripts/pre-release.sh` on `700de73`: **19/20 PASS, exit 1**.
Only `cargo-deny` fails on the current Tokio advisory. Formatting, strict pedantic
Clippy, all-feature workspace tests, schema/version/README checks, frozen install,
generated TS/i18n drift, lint, typecheck, frontend tests/build, WASM build/size guard,
canonical Chromium smoke, real CLI trust/export probes, release boundaries and
candidate-identity tests all pass. Final Rust totals remain 1,592 PASS / 0 FAIL /
1 intentional ignored worker; frontend remains 1,498 PASS / 1 optional locale skip.
WASM CPS delta is 1,592 KiB gzip within the unchanged 1,700 KiB budget.

Final canonical invocation used `UI_SMOKE_PORT=18945 UI_SMOKE_CDP_PORT=18946`.
The native watcher host tests pass again. Canonical Chrome checks seven sections,
Presets, Machine sections and 120-tool filtering/sorting/pagination/selection;
no page exceptions. This is browser evidence, **not** a Tauri runtime walkthrough.
The final 23-case corpus was rebuilt/replayed and is exact. TEST1/C2's 256-pass
receipt is from the starting source; its production implementation and property
sources are unchanged by the comment/test-harness fixes. Affected CPS tests and
full workspace suite were rerun on the final source. No new timing claim.

[Final gate matrix](qual-1c-evidence/final-qualification.json),
[full gate log](qual-1c-evidence/final-pre-release-host.log),
[final corpus](qual-1c-evidence/final-corpus-summary.json),
[preservation receipt](qual-1c-evidence/preservation.json).
All 8,496 pre-existing modified/untracked files remain byte-identical; primary
HEAD and path inventory are unchanged. The raw attempt logs, initial failures,
TEST1 cases and corpus responses are retained under `dist-test/qual1c/raw/`,
with a committed hash manifest. Initial and final bundle manifests remain distinct.
The evidence/report commit contains documentation only and is discoverable with
`git log -1 -- docs/releases/QUAL_1C_MACOS_PERSONAL_ALPHA.md`.

## Recommendation and next human decision

**NOT READY to accept QUAL-1C today.** This is not an observed startup failure:
startup has deliberately not been attempted in the usual account. A real native
walkthrough is still required, and the current security gate has an unresolved
memory/I/O-safety advisory. No claim of transitive unreachability has been established.
The built app and software evidence are ready for review; this report does not
waive the advisory for personal use or grant a machining approval.

Tracking: engineering `ivac-ado9a`; security disposition `ivac-qcbfv` remains OPEN.
Human native acceptance is not closed by this report.

Next decisions:

1. Authorize a focused Tokio dependency correction and requalification, or provide
   a separately reviewed, evidenced security disposition. Do not suppress the gate
   or update dependencies silently as part of this handoff.
2. Prepare the separate macOS test account chosen by the maintainer. After the
   security disposition, run the [Personal Alpha guide](QUAL_1C_MACOS_TEST_GUIDE.md)
   and record real launch, project/preset round-trips, generation/simulation,
   cancellation and qualified export. No direct writes to existing user data.
3. Human review determines Personal Alpha acceptance. QUAL-2 requires separate
   authorization and a progressively supervised physical protocol.

No push, merge, rebase, tag, version change, public installer, release or hardware
operation. No modification to frozen V1. Retain the isolated branch: it is based
on the primary HEAD, but no fast-forward was performed into the dirty V1.1 tree.
Reconciliation must preserve the pending roadmap and frontend/CAM work explicitly.
**HARD STOP after this report and its scoped local commits.**
