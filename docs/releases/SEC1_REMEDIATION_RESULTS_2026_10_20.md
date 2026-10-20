# SEC1 remediation — preparation and release-policy decision

2026-10-20. **Preparation only; no corrections or new qualification claimed.**
Inspected source: `6043bfefcca7b455e58eab7c980970f885ab6695`.
Tracking: `ivac-dqlol`. Original review: `ivac-btkaq` / human gate `ivac-1ahyt`.

The user approved bundled-only CPS execution for V1, accurate transactional
publication reporting and strict source-revision release qualification, including
separate commits after successful qualification. Push, merge, tag, packaging,
publication and hardware operation remain prohibited.

## Mandatory decision before workflow restructuring

The request explicitly says:

> If restructuring GitHub workflows introduces substantial release-policy changes,
> present the proposed design and HARD STOP before implementing that part.

The current tag-triggered `release-desktop.yml` creates a draft prerelease after
`needs: build`. CI runs independently on the same tag. Merely adding local checks
does not link the release to the full CI result, and a tag event does not encode
the separate human publication decision. The proposed fix below changes this
workflow contract and therefore requires the requested review.

**Recommended design, not implemented:**

1. Reuse a single qualification workflow through `workflow_call`, retaining
   ordinary PR/manual CI. Both packaging workflows invoke that workflow on their
   own source revision. Required job results must explicitly equal `success`;
   cancellation, failure, missing results and skipped mandatory jobs block the
   aggregate gate. Do not infer qualification from an unrelated green run.
2. Include a strict software gate, schema/version/codegen checks, existing
   cross-platform Rust jobs, frontend checks, TEST1 and the remaining required
   checks. Keep missing optional external test oracles visible and separate from
   skipped mandatory jobs. The exact mandatory job inventory must be documented
   with the implementation, not reconstructed from display names at publication.
3. Preserve manual candidate creation as test-artifact production. Candidates
   carry an exact source SHA, version, target, workflow run identity, qualification
   identity and installer checksums. Packaging success alone never sets software,
   installation, runtime or physical acceptance. Refuse a dirty CI source instead
   of merely recording it as dirty.
4. Make tag-triggered builds artifact-only; remove automatic draft-release
   creation from that path. Introduce a separate manual draft-promotion action
   naming the exact qualified build run and source SHA. It verifies the required
   workflow/repository, completed successful qualification and build results,
   matching SHA/version, manifest and transferred artifact checksums.
   Do not rebuild or select the newest artifact implicitly.
5. Retain explicit human approval before that promotion, preferably through a
   protected GitHub environment. Repository-side reviewer protection requires
   separate setup/verification; a YAML environment name alone is not proof of an
   enforced approval. Until the approval mechanism is established, promotion
   remains unavailable. Creating a draft is itself an external publication action.
6. Keep installed macOS/Windows/Ubuntu acceptance, physical ShopBot qualification
   and final stable-release publication approval separate from software CI.
   Neither a green CI run nor draft promotion completes these gates.

Decision requested: approve this separation of qualification, artifact-only
packaging and explicitly approved manual draft promotion. A smaller alternative
is retaining tag-triggered draft creation behind shared qualification and a
verified protected environment; that retains the existing trigger but makes
repository approval configuration a mandatory dependency.

No workflow, environment, secret, tag or release has been changed.

## Verified implementation findings

### SEC1-01

`ivac-cps/src/library.rs` already contains an authoritative `include_str!`
registry with `grbl` and `shopbot`. However, the public runtime accepts script
strings, `Engine` is public, and `pipeline::resolve_cps_source` accepts inline
scripts and filesystem paths. Tauri/HTTP inspection, CLI `posts inspect` and
feature-enabled WASM inspection pass external source to the evaluator.
`CpsPostConfig.svelte` automatically inspects the stored external script when
its form mounts. Hiding the import button alone would not close this finding.

Implementation direction within the approved product policy:

- Resolve bundled identifiers only through the compiled registry, using an
  opaque trusted-post reference whose source cannot be supplied by a caller.
  Narrow production runtime access; no production feature/environment escape
  hatch for arbitrary source. Preserve low-level runtime regression coverage in
  test-only code rather than deleting it.
- Preserve serialized external selections and their properties when loading and
  saving projects. Refuse their inspection/generation before JavaScript evaluation
  (including identical external copies); do not silently substitute another post.
- Remove automatic external inspection and file import from the V1 form. Display
  the unavailable selection and allow an explicit supported-post selection.
- Test all transport entry points and supported bundled output. Include renamed,
  spoofed-ID, modified, unchanged external-copy and saved-project cases; assert
  refusal before evaluation rather than relying on a script timeout.

The runtime currently transfers properties with `JsValue::from_json`. A textual
search of shipped post/prelude production JS found no `eval(...)` or
`new Function(...)` call; this is an initial observation, not a completed
configuration-injection audit. Property application, prototype-sensitive keys
and all reachable APIs still require implementation-time review and tests.

### SEC1-04

`StagedFile::publish(true)` successfully creates the final hard link and then
returns the result of fallible `temporary.close()`. CLI uses `?` for the first
side and reports the second side as unpublished on any error. Consequently a
cleanup failure can misreport a completed final file.

Implementation direction: return a structured successful publication outcome
with optional cleanup warning; reserve errors for failures before final
publication. Propagate it through CLI and native/frontend consumers together.
A cleanup warning on the first side must not prevent attempting the already
staged second side. A second-side publication failure remains explicitly partial,
with exact per-side outcomes. Fault injection must prove final bytes, staging
state and reported status for each branch. Preserve no-clobber pairs, old files,
no destructive rollback and the absence of pair-wide atomicity.

### SEC1-08

Confirmed by static inspection:

- `pre-release.sh::skip` does not set failure, yet the final banner can say
  “all gates green — ok to release”.
- The codegen subshells do not fail fast, allowing a successful final `git diff`
  to hide an earlier generator failure.
- The local script lacks `version-check`.
- `wasm-size-guard.sh` uses GNU `stat -c` and does not propagate its failure;
  absent wasm-pack returns success.
- `release-desktop.yml` release creation depends only on packaging.
- `desktop-candidates.yml` has frontend/package/version checks and a source
  manifest, but not the complete CI dependency.
- The collector records a dirty source without rejecting it.

Bounded script fixes remain part of batch C: mandatory-tool failures, correct
codegen exit propagation, reproducible byte-size measurement and validated
numeric budget, local version checks and a software-only success summary.
Workflow restructuring follows the decision above.

## Preservation, checks and handoff

Read `AGENTS.md`, `CLAUDE.md`, documentation index, STATUS, V1 readiness, the SEC1
review and relevant architecture/validation/ShopBot/documentation skills.
Inspected Git status, log, worktrees, remotes, tracker and the code/workflows
named above. No build or test was run: static preparation cannot establish that
any security blocker is fixed.

Tracked source was clean at entry. The separate `codex/sec1-roadmap` worktree and
active documentation issue `ivac-pknlj` were observed. Its roadmap work and all
local-only evidence are left untouched; this report is a separate file.
NUMSAFE3 and earlier human gates remain independent.

No implementation commits exist for this remediation. All three batches,
integrated regression/output comparisons and the three authorized qualified
commits remain to be completed. SEC1-02/03/05/06/07/09 retain the original review's
scope and limitations; no broad Tauri redesign or unrestricted runtime is started.

**HARD STOP at the workflow-design decision, before implementation.** This report
must be extended with actual corrections, commands, results, limitations and
commit hashes when work resumes; it is not the requested final qualification
receipt. Existing NUMSAFE3 passing results are not SEC1 test results.

## Approved implementation follow-up — batch A

The user approved the release architecture above; its design stop is superseded.
Batch A now enforces compiled-registry execution in `ivac-cps::run` and refuses
all source-taking production APIs before engine construction. `Engine` is private.
Existing arbitrary-source probes are retained as crate unit tests, not exposed by
any production feature. Explicit integration tests exercise the production library
where `cfg(test)` is absent. Core inline/path selections are refused without reading
paths; HTTP/Tauri/WASM/CLI inspection inherit the runtime refusal. Listings use only
registry inspection. Properties must be primitive JSON and cannot use prototype
keys. The inspector no longer calls external inspection on mount or import; saved
settings remain unchanged until explicit selection. ShopBot allowlists are unchanged.

Targeted validation (macOS software only): `cargo test --offline -p ivac-cps
-p ivac-server` (48 runtime probes, 18 server tests, plus production trust tests;
final trust run 3 PASS); `cargo test --offline -p ivac-core --all-features --lib`
1219 PASS; core CPS (final 6), SAFE-Z (15), ShopBot M1 (11), M2 (27), provenance
(12) passed. Tauri `sec1_tests` passed. Frontend typechecking passed; CPS form,
project-loading and i18n targeted checks passed (22 PASS/1 existing skip), with a
further 13-test form run covering unavailable external metadata and preservation.
Bundled GRBL golden digest and ShopBot semantic guards retained.

During development, an over-broad test edit affected two bundled cache assertions;
restored their original assertions and reran the entire 1219-test core library
successfully. An engine privacy compile error in the old diagnostic example was
resolved by retaining that probe as a unit-only test. Clippy required moving helpers
above the test module. Initial frontend commands selected the host pnpm wrapper;
reran with the project's existing pnpm 10 path. No dependency policy changed.
Full integrated validation and output corpus comparison remain pending below.

## Batch B — truthful publication state

Batch A commit: `5e789e5`.
`PublishedFile { path, cleanupWarning }` now represents successful publication;
`Err` means the attempt did not publish the final filename. A post-hard-link cleanup
failure returns success with a warning. CLI reports that warning and continues the
already staged pair; a later publication failure accurately reports partial output.
Tauri returns the same structured outcome; frontend reports “Published …” rather
than “save failed”. No byte transformation, rollback or pair overwrite was added.

`cargo test --offline -p ivac-file-export`: 11 PASS, including private injected
cleanup failures and a second-side no-clobber failure. First test run exposed macOS
`/var` versus `/private/var` canonical-path comparison in the new assertion; fixed
the assertion and reran successfully. `cargo check --offline -p ivac-cli -p
ivac-tauri` passed. Rebuilt CLI and `python3 scripts/numsafe3-cli-check.py` passed
preservation, byte equality, staging and pair checks. Frontend actual export service
checks: 24 PASS, including publication-with-warning; project replacement consumers
are updated to the shared result. OS installation acceptance remains unqualified.
