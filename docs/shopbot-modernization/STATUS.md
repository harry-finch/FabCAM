# Status — 2026-10-10

Current milestone: M0 complete, with build/test environment limitations below.
Repository audit and documentation committed/pushed; no M1 work,
ShopBot post, machine profile or waterjet implementation has started.

## Git baseline

- Checkout: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
- Account / fork: `harry-finch`, https://github.com/harry-finch/ivacam.
- Origin: `https://github.com/harry-finch/ivacam.git`.
- Upstream: `https://github.com/aalarchiv/ivacam.git`.
- Branch: `feature/shopbot-mvp`.
- Audited upstream SHA: `9168104b4fc4e02e916740a54bd1e56b5019f662`.
- Documentation baseline SHA: `15b3d58667599af5a4e445d3cc52c0ae1b7d702a`,
  pushed to `origin/feature/shopbot-mvp`.
- This handoff is a separate documentation commit recording that exact SHA;
  obtain the final handoff SHA with `git rev-parse HEAD` (a commit cannot contain
  its own hash). Final remote equality and clean tree are checked after push.
- No PR or default-branch merge. M0 application-source diff is empty.
- Commit identity uses verified GitHub public name Harold Finch and account
  noreply address. No global Git identity/configuration was changed.
- HTTPS Git has no credential helper configured on this host. Successful push
  used transient `-c credential.helper= -c 'credential.helper=!gh auth git-credential'`
  with the authenticated gh account; future authorized pushes may need this too.

## Validation

Frontend: 106 test files passed; 1,044 tests passed, 1 skipped. Svelte: zero
errors/warnings. Node CPS prelude: 39 passed. ESLint, TypeScript Node config,
OpenAPI client drift and README parity checks passed.
Rust build/tests could not start: cargo/rustup absent. Frontend build blocked:
missing WASM package and wasm-pack. Details: [TESTING_STRATEGY](TESTING_STRATEGY.md).
No hardware validation, controller preview or VCarve comparison was performed.

## Findings / next action

Keep the existing core, imports, simulation and transports. Existing CPS and
native post interfaces are plausible ShopBot extension seams; M2 must decide.
ProtoMAX assessment: **B — moderate architectural extension**; process events
and quality are not explicit enough today. See FUTURE_PROCESSES.

STOP — waiting for human approval before starting the next milestone.
Next milestone is M1 only. Before claiming CAM correctness,
provide Rust 1.88.0 and execute the existing Rust tests, then the approved M1
corpus. `bd prime` could not run because bd is unavailable; no Beads database
was changed or issue status invented.
