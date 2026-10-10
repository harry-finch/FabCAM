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
- M0 commit/push state is recorded in STATUS. No PR, default-branch merge, M1
  qualification, ShopBot implementation or waterjet implementation authorized.

STOP at M0; next milestone requires explicit human approval.
