# SEC-1 — Tokio security fix and repository consolidation

2026-10-24. Software-only development consolidation. No main merge, version
promotion, public release, installer publication, application launch or hardware.

## Source and disposition

Primary starting `FabCAM-v1.1`: `aeae8202b385e46fdee9ff5827f5ca05702d14c3`.
QUAL-1C source: `700de73a5cf5a378b8465e5f38950a61d6cce4c3`; its report tip is
`9520d98b7966745253c0a0401c800ef4831bc79b`. Three descendants were reviewed:
`2e6c021` changes a doc comment and a checked test-fixture conversion;
`700de73` corrects smoke expectations to seven existing areas and tests Presets;
`9520d98` records source-bound results. No production CAM delta in those commits.

Focused security commit: `f0e474612232bd521e406a0186f044e2512b5ce4`.
Reviewed documentation merge: `135095392c2a5711acc79f5286aae7599214585a`, incorporating
`f9316eb` and `ff6486b1795bb95994ad97d7cb799011e276fded`. The sole merge conflict
was the documentation index's current-checkpoint paragraph. It was reconciled
with completed A4/B and built/software-tested C, without changing historical reports.
ADR-0031 remains Proposed; optional one-way FabSpeed → FabCAM stays future V1.2.

Consolidated executable/build source: **`135095392c2a5711acc79f5286aae7599214585a`**.
The final report commit is documentation/evidence only; its identity and the pushed
ref must be recorded separately from this tested build source.

## Tokio correction and dependency ownership

[RUSTSEC-2026-0386](https://rustsec.org/advisories/RUSTSEC-2026-0386.html) covers
unsafe I/O lifetime assumptions in AsyncFd constructors. The advisory accepts
1.53.3 and newer, or the patched 1.51 branch. The selected **1.53.3** is the smallest
forward patched update from 1.52.2, compatible with existing `^1.40` direct
requirements and Tauri 2.11.0. Tokio's declared Rust requirement remains 1.71;
the pinned build uses Rust 1.88.0. No manifest or Tauri upgrade was necessary.
[RUSTSEC-2026-0384](https://rustsec.org/advisories/RUSTSEC-2026-0384.html) affects
1.54.0's Interval race; that version was not selected.

Only Tokio's version and registry checksum change in Cargo.lock. Cargo metadata
before/after confirms one Tokio instance and no other package-version changes.
An initial `cargo update --offline --precise` also selected different existing
Windows dependency edges; those unrelated resolver changes were excluded before
qualification. The minimal original graph plus corrected Tokio is accepted by
`cargo fetch --locked`, locked metadata and all recorded builds. This was not a
manual checksum guess: the value matches the registry index and fetched archive.

Native direct users are ivac-tauri and ivac-server; transitive users include Tauri,
plugins, axum/hyper and tokio-util/stream. Effective native features include runtime,
macros, time, networking, I/O, synchronization, signal and filesystem support.
Complete reverse trees and native feature trees before/after are in the evidence.
No direct FabCAM AsyncFd reference was found; this is not a claim of transitive
unreachability. The configured audit is still mandatory.

Full current cargo-deny **PASS**: advisories, bans, licenses and sources. RustSec
revision `21d909e4c8bd4e14c7aea58bf980c881a0053ab1`, timestamp
2026-10-24T10:30:13+02:00; fresh host audit began 2026-10-24T15:43:08Z.
Existing warnings/approved maintenance ignores remain unchanged; no advisory was
added to ignore lists, no security feature disabled, no gate modified.

Focused qualification before security commit: Tauri 15 PASS on macOS (all five
watcher tests), HTTP 21, file export 11, CPS 55; strict affected pedantic Clippy,
formatting, generated native ACL/old-ACL counterexample and actual CLI trust/
transactional export probes PASS. All 23 fixed output references are exact.
The sandbox attempt retained its real 13 PASS / 2 watcher FAIL outcome; unchanged
host tests pass. Static ACL/mocks are not installed-native acceptance. Existing `$HOME/**`
filesystem scopes remain; no chooser-only policy is claimed.

## Initial branches and worktrees

Counts compare each initial head with the initial V1.1 head. A branch can have
no unique commits and still contain unique ignored evidence or an active preview.

| Branch | Initial HEAD | Worktree | Dirty/untracked | V1.1-only / branch-only | Disposition |
|---|---|---|---:|---|---|
| `FabCAM-v1.1` | `aeae8202b385e46fdee9ff5827f5ca05702d14c3` | ivacam | 8496 | 0 / 0 | ACTIVE / RECONCILE |
| `codex/docs-consolidation` | `744774d161d1a02b7dda00cdbe70a2360d8279b8` | none | 0 | 68 / 0 | ALREADY INTEGRATED / cleanup candidate |
| `codex/fs-reset-2-repairs` | `d6b08fe82011c54d0166a6559b329804c2e56cee` | FabCAM-fs-reset-2-repairs | 0 | 20 / 0 | ALREADY INTEGRATED / cleanup candidate |
| `codex/qual-1a-deterministic-vcarve` | `7f55b2e8fdc7850234a90e456efa2f02de21c322` | FabCAM-qual-1a | 8 | 6 / 2 | EXPERIMENTAL / HISTORICAL |
| `codex/qual-1a2-numeric-input` | `aa846fc6ac01af586e660e43f13488bef3c8064d` | FabCAM-qual-1a2 | 8 | 6 / 4 | EXPERIMENTAL / HISTORICAL |
| `codex/qual-1a3-motion-presets` | `aa846fc6ac01af586e660e43f13488bef3c8064d` | FabCAM-qual-1a3 | 87 | 6 / 4 | EXPERIMENTAL / HISTORICAL |
| `codex/qual-1a4-production` | `f2b0427b2361ae33064eb9caa5a874716880ee4d` | FabCAM-qual-1a4 | 0 | 2 / 0 | ALREADY INTEGRATED / HISTORICAL |
| `codex/qual-1b-cancellation` | `aeae8202b385e46fdee9ff5827f5ca05702d14c3` | FabCAM-qual-1b | 0 | 0 / 0 | ALREADY INTEGRATED / HISTORICAL |
| `codex/qual-1c-personal-alpha` | `9520d98b7966745253c0a0401c800ef4831bc79b` | FabCAM-qual-1c | 0 | 0 / 3 | MERGE |
| `codex/roadmap-fabspeed` | `ff6486b1795bb95994ad97d7cb799011e276fded` | FabCAM-roadmap-fabspeed | 0 | 6 / 2 | MERGE / RECONCILE |
| `codex/sec1-roadmap` | `7e1c24b937e9bc01aa320537c835ffc30fc9389c` | FabCAM-docs-consolidation | 0 | 60 / 0 | ALREADY INTEGRATED / cleanup candidate |
| `codex/v1-freeze-readiness` | `85a93c4a467ff5965655d5737354d9500c2e94fc` | FabCAM-v1-freeze | 0 | 50 / 0 | HISTORICAL |
| `feature/shopbot-mvp` | `c9d3f3b925087ee042a61158a393c3206c7dd3c0` | none | 0 | 91 / 0 | HISTORICAL |
| `main` | `ef807cf8250ddc4bdf795c75bb9bf9f011db9bca` | none | 0 | 49 / 0 | HISTORICAL |

## Local changes and preservation

Initial primary checkout: 16 modified tracked files, no staged changes, extensive
local-only reports/evidence. SHA-256 inventory covers 8,496 primary files and 103
modified/untracked experimental-worktree files: **8,599 total**. Post-cleanup check
found zero differences before primary integration. Application-data paths were
identified without reading private contents or resetting/writing their profile.
Other FabCAM chats were idle at inventory; no subagents or cross-chat writes.

- Pocket diagnostics: `warnings.rs` adds an analytic narrow-wall coverage
  certificate; `offset_builder.rs` changes corresponding test expectations.
  Alters diagnostic eligibility; no new combined-source acceptance. PRESERVED.
- Fixed-stock transforms, drawing movement/dimensions, arc bounds and history:
  frontend state/canvas/components plus tests; bounded local reports retain pending
  human review. PRESERVED, excluded from the candidate.
- Header/theme/navigation: local styling and controls retain the known hardcoded
  translation failure. PRESERVED, excluded rather than weakening the guard.
- Local STATUS/roadmap: their intent is reconciled in the canonical documents.
  Original bytes are archived and hash-verified in the private workspace receipt;
  only those two documents are intentionally superseded in place.
- All original uploads, user projects, app data, private presets, FabSpeed files,
  historical reports and diagnostic evidence remain preserved. No broad staging,
  stash/reset of user work or application-data cleanup.

Primary integration uses ordinary fast-forward; pending executable files remain
uncommitted, byte-identical and outside the pushed commit. Consequently the dirty
primary working tree is **not** the qualified app source. Reproduce the app from
the clean SEC-1 checkout or the committed source, without folding in those edits.
Frozen `main` and the V1 worktree remain unchanged. A1/A2/A3 numerical prototypes
retain unique diagnostic commits and uncommitted variants; do not merge or delete
them merely because A4 incorporated the accepted approach.

## Cleanup and evidence retention

Path-by-path proposal was recorded before removal. Generated files from removed
worktrees were preserved with hashes; source remains reachable in integrated Git
history. Only normal `git worktree remove` and safe `git branch -d` were used.

| Path relative to workspace | Allocated size before cleanup | Disposition |
|---|---:|---|
| `ivacam/target/debug/incremental` | 547.6 MiB | Verified generated cache or fully integrated clean worktree; removed |
| `FabCAM-qual-1c/target/debug/incremental` | 2256.1 MiB | Verified generated cache or fully integrated clean worktree; removed |
| `FabCAM-sec-1/target/debug/incremental` | 2256.1 MiB | Verified generated cache or fully integrated clean worktree; removed |
| `FabCAM-docs-consolidation` | 164.1 MiB | Verified generated cache or fully integrated clean worktree; removed |
| `FabCAM-fs-reset-2-repairs` | 193.2 MiB | Verified generated cache or fully integrated clean worktree; removed |
| `FabCAM-roadmap-fabspeed` | 191.7 MiB | Verified generated cache or fully integrated clean worktree; removed |

Deleted integrated local branches: `codex/docs-consolidation`,
`codex/sec1-roadmap`, `codex/fs-reset-2-repairs`, `codex/roadmap-fabspeed`.
No remote branch was deleted.
After cleanup and primary fast-forward, nine worktrees remain: primary V1.1,
A1/A2/A3 experimental studies, A4 preview/evidence, B diagnostic evidence, C
historical local bundle, the active SEC-1 qualification checkout, and frozen V1.
`codex/sec-1-consolidation` was created from QUAL-1C for this task; it contains the
qualified source and matching local app. Main and feature/shopbot-mvp have no
additional worktree here. The original 14 branches plus SEC-1 become 11 after
four safe local deletions. The closing report changes only documentation/evidence.

A4's preview, B's diagnostic snapshot and raw evidence,
C's original app/raw evidence, A1/A2/A3 experiments and frozen V1 stay available.
B's raw file hashes match the primary retained copy, but its diagnostic symlink
structure differs from that dereferenced copy; its worktree is conservatively kept.
No entire dist-test directory was deleted.

Observed available-space increase across cleanup windows: about **2.62 GiB**.
This is an observation under concurrent OS activity, not a claim that logical
byte totals equal physical APFS reclamation. New qualification build outputs may
subsequently consume space. Separate before/after receipts retain exact values.
C's 4-file bundle manifest and 1,716 raw records remain exact; both copies of B's
7,661-file manifest remain exact. Original manifests and source IDs are unchanged.

## Final-source qualification

**20/20 mandatory stages PASS**, exit 0, on committed source
`135095392c2a5711acc79f5286aae7599214585a`, clean tracked state. The exact
`scripts/pre-release.sh` is unchanged. Host execution uses dedicated smoke ports
18955/18956 and a disposable Chromium profile; no other preview is stopped.

| Mandatory stage | Result |
|---|---|
| cargo fmt --check | PASS |
| cargo clippy -Dwarnings (with pedantic) | PASS |
| cargo test --workspace | PASS |
| xtask schema-check | PASS |
| xtask version-check | PASS |
| readme parity | PASS |
| pnpm install | PASS |
| codegen drift | PASS |
| i18n keys drift | PASS |
| pnpm run lint | PASS |
| pnpm run check | PASS |
| pnpm run test | PASS |
| pnpm run build | PASS |
| wasm-pack build (web) | PASS |
| wasm cps size guard | PASS |
| cargo-deny check | PASS |
| ui smoke (headless) | PASS |
| CLI trust and transactional export | PASS |
| SEC1 release boundary regressions | PASS |
| candidate identity regressions | PASS |

Workspace all-features Rust: **1,592 PASS / 0 FAIL / 1 intentionally ignored
TEST1 worker**, 48 result summaries. Native Tauri includes all five watcher and
four collection-storage tests. Frontend: **1,498 PASS / 1 existing optional locale
skip**, 172 files. The full unfiltered suites include v9 roles/limits/persistence,
SAFE-Z, NUMSAFE/2/3, provenance, ShopBot/CPS cancellation and transactional export.
Strict workspace/all-target/all-feature pedantic Clippy and formatting pass.

Additional generated ACL/old-capability counterexample and JavaScript CPS tests
pass. Fresh public WASM numeric API: **328 cases** match the independent numeric
oracle; invalid CAM values retain their distinct refusals. The fixed 23-case CLI
corpus is exact, including both two-sided programs. Expanded replay: **77 cases × 3 repeats in native and actual WASM**; all 52
successful responses and 25 expected refusals match the reviewed references.
Programs/indexes/statistics match across runtimes; the already documented Profile
internal-path exception remains explicit and unchanged. Public CLI responses and
refusals agree. TEST1/C2: **256 PASS / 0 FAIL / 0 timeouts**, 26.613 s within the
unchanged limits. JavaScript CPS: **46 PASS**; no new performance claim is inferred.

Optional external FANUC/DXF/reference inputs still have existing availability
limits; an early-returning optional oracle is not newly qualified by aggregate
counts. Browser smoke is not Tauri-installed acceptance. No test, tolerance,
golden, gate or security policy was weakened to obtain these results.


## Local macOS application

**PASS**, rebuilt after all software checks with the supported existing command:

```sh
cd crates/ivac-tauri
tauri build --config tauri.dev-signing.conf.json --bundles app -- --locked
```

Local bundle: `FabCAM-sec-1/target/release/bundle/macos/FabCAM.app`, relative to
the shared workspace. Version remains **0.9.0-beta.3**, identifier
`org.opcenter.ivacam`, Mach-O **arm64**. Native and frontend embed the verified
source stamp `v0.4.0-126-g1350953`; packaged frontend WASM matches the
fresh compiled WASM hash. The historical v0.4.0 git-describe prefix is not an
application-version promotion.

Strict deep signature verification PASS. Signature is **ad hoc**, hardened-runtime
flag, no TeamIdentifier, **not notarized**. No launch or ordinary-profile access.
No DMG, public installer, notarization or Gatekeeper acceptance is claimed.

Executable SHA-256: `a2d2666e70ca8e73f44001b8472d71c61cd9710da67b1e400f719463ecb7eb0f`.
Bundle file-manifest SHA-256: `9a8c617c8ab08b8672f5bcf196a6efaf927c5dc2d326ec3435837e71cfc53f1a`.
[Manifest and algorithm](sec-1-evidence/artifact-manifest.json),
[source/asset binding](sec-1-evidence/source-binding.json).
The original QUAL-1C bundle remains unchanged as historical evidence.


## GitHub publication

Target: only **`origin/FabCAM-v1.1`**, ordinary non-force publication of the
qualified source plus this documentation/evidence commit. Initial host read found
only main and feature/shopbot-mvp; no remote V1.1 branch. The configured origin
and authenticated GitHub account are harry-finch/FabCAM. Sandbox DNS failures did
not indicate invalid host credentials.

The exact final remote SHA and actual push outcome are recorded after publication
in the local [post-push receipt](../../../SEC-1-workspace-evidence/PUSH_RECEIPT.json),
which accompanies this report. Its `verified` field must be true, `remote_head`
must equal the intended `local_head`, and `other_remote_changes` must be empty
before declaring publication complete. This post-commit receipt is local-only:
the report cannot contain its own commit hash. The final user handoff supplies the
same exact remote SHA. An absent/failed receipt means publication is still pending.

The guarded host publisher rechecks repository identity, exact local source,
empty index, ancestry, successful final qualification and remote state immediately
before pushing; a divergent/new remote V1.1 head stops it for renewed review.
It stages or commits nothing, pushes no other branch/tag and never forces history.
It records workflow URLs/status through GitHub after verifying the remote ref.
Current CI triggers are release tags/PR/manual calls, so an ordinary development
branch push need not start CI. No CI success or public release is inferred.

Pre-publication review of the consolidated source: 1,349 unique unpublished blobs,
54,773,900 logical bytes; none larger than 5 MiB. Filename and credential-pattern
review found no private project/config/key or credential match. This is a scoped
heuristic plus reviewed changes, not proof against every possible secret. Final
documentation/evidence additions receive the same review before staging.

## Evidence and reproducibility

[Canonical gate matrix](sec-1-evidence/qualification.json),
[complete stage log](sec-1-evidence/pre-release.log),
[77-case replay](sec-1-evidence/replay-summary.json),
[preservation](sec-1-evidence/preservation.json),
[raw manifest](sec-1-evidence/raw-evidence-manifest.json).
Raw inputs, full responses, binaries, initial failed attempt and original logs
remain under `FabCAM-sec-1/dist-test/sec1`; initial workspace preservation, deleted
worktree generated-file copies and publication receipts remain in the sibling
`SEC-1-workspace-evidence`. They are not private user application data. Published
text log copies map workspace/home prefixes to `$WORKSPACE`/`$HOME` and trim
trailing whitespace; the raw
unmodified files and hashes remain available locally. No raw binaries or private
projects are committed. Reproduction needs these retained source-bound fixtures
for A3/A4 comparisons, cached/pinned toolchains and a permitted host for network,
FSEvents and Chromium. Existing external reference-oracle gaps remain documented.

## Remaining acceptance and hard stop

Next: **Personal Alpha — Actual macOS Launch & Native Workflow Acceptance**.
Use the newly rebuilt app identified in this report in the separate macOS test
account chosen by the maintainer. Follow the
[QUAL-1C checklist](QUAL_1C_MACOS_TEST_GUIDE.md), substituting the SEC-1 bundle
for the historical QUAL-1C path: the old bundle is retained as evidence and still
contains Tokio 1.52.2. Test actual launch/restart,
project and collection round-trips, native dialogs/authorization, explicit values,
generation/simulation/report, cancellation/recovery, fresh and stale export.
Do not use the ordinary maintainer application-data profile without authorization.

A local ad hoc signed app and passing software checks do not establish installed
native workflow, clean-host Gatekeeper or physical CNC acceptance. Preset
attestations remain separate from software validation. Preserve known-limit
refusals, unknown-limit disclosure, SAFE-Z/origin, provenance, resource budgets,
bundled-only CPS, ShopBot restrictions, and both-side publication's non-atomic pair
semantics. CLI streaming is not full-program simulation qualification; stdout is
not transactional. Automatic dowel injection remains excluded.

Supervised QUAL-2 requires separate human authorization. FabSpeed integration,
installer distribution and public release remain out of scope. **HARD STOP after
the qualified branch push and report.**
