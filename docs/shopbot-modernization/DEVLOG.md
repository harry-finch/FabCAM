# Engineering log

## 2026-10-10 — M0 qualification

- Initial FabCAM folder contained the supplied skills pack, not a Git checkout.
  Cloned upstream history into `ivacam/`, preserving the original pack outside.
- Verified GitHub account with gh: harry-finch. Created its normal ivaCAM fork,
  configured origin/upstream and created `feature/shopbot-mvp` from upstream
  `9168104b4fc4e02e916740a54bd1e56b5019f662` (version 0.5.0).
- Applied github-upstream-workflow, ivacam-architecture, project-documentation
  and protomax-waterjet (architectural foresight only). Copied the supplied skill
  pack into the checkout for future sessions.
- Inspected upstream agent files, manifests, imports, models, pipeline/CAM,
  simulation, native posts/CPS runtime, frontend persistence/history/profile
  helpers, build scripts and CI. Recorded feature evidence and reuse decisions.
- ShopBot feasibility: existing post seams appear sufficient; final CPS/native
  decision deferred to M2. ProtoMAX assessment B: shared infrastructure reusable,
  explicit process/pierce/quality semantics absent in the current CPS IR.
- Ran existing Node/frontend checks. Results and environment failures are in
  TESTING_STRATEGY. Rust is unavailable; no claim of Rust CAM qualification.
  bd is unavailable; upstream issue records untouched.
- Created the persistent documentation baseline, including roadmap/approval
  policy. No application source, schema or machining behavior changed.
- Baseline commit `15b3d58667599af5a4e445d3cc52c0ae1b7d702a` pushed to the feature
  branch; separate handoff commit records its SHA in STATUS. HTTPS push used
  gh's credential helper transiently; no global Git config changed.
- M0 commit/push state is recorded in STATUS. No PR, default-branch merge, M1
  qualification, ShopBot implementation or waterjet implementation authorized.

STOP at M0; next milestone requires explicit human approval.


## 2026-10-10 — M1 software qualification

- User approved continuing after M0 and installing the tools. Applied
  cam-geometry, cam-validation and project-documentation; inspected existing
  imports, offsets, operation setup, preview and simulation rather than adding
  a new CAM subsystem.
- Installed pinned Rust/WASM tools, Node 24/pnpm 10, sccache and Beads. Repaired
  the pre-existing Homebrew Node after its dependency update. Baseline Rust
  suite with the restored environment: 1,136 pass. See TESTING_STRATEGY.
- Added all eight specified analytic shapes in SVG and DXF, and 11 qualification
  tests for import, profile compensation/Z passes, pocket preservation/coverage/
  allowance, drill depth/peck/retract, disconnected/open paths and determinism.
- Corrected endpoint-only arc bounds, inscribed island round-join clearance,
  and cascade/spiral terminal-strip coverage by capping ring stride to radius.
  Added two arc-bound unit tests and strengthened the existing island test.
  Reproductions, tolerances and limits are in M1_QUALIFICATION.
- Final Rust workspace: 1,149 pass; core without CPS: 1,060 pass. Headless build,
  Clippy, formatting, schema/version, release WASM, frontend check/test/build pass.
  Frontend: 1,044 pass, one optional locale skip. No machine/VCarve qualification.
- Beads installed and original 1,165 issue records imported. Closed only new
  issues ivac-h52u and ivac-n92a; appended their exported records without
  rewriting upstream issue history. Local runtime files excluded from Git.
- No schema, dependency or process-boundary changes; future ProtoMAX assessment
  remains B. More vertices/passes are possible; performance unmeasured.
- M1 work remains uncommitted. Master prompt requires explicit commit/push
  authorization after M0. No M2 implementation, PR or merge started.

STOP at M1; next milestone and Git publication require explicit approval.
