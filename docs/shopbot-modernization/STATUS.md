# Status — 2026-10-10

Current milestone: M0. Repository audit and documentation prepared; no M1 work,
ShopBot post, machine profile or waterjet implementation has started.

## Git baseline

- Checkout: `/Users/mugen/Documents/01_Projets/FabCAM/ivacam`.
- Account / fork: `harry-finch`, https://github.com/harry-finch/ivacam.
- Origin: `https://github.com/harry-finch/ivacam.git`.
- Upstream: `https://github.com/aalarchiv/ivacam.git`.
- Branch: `feature/shopbot-mvp`.
- Audited upstream SHA: `9168104b4fc4e02e916740a54bd1e56b5019f662`.
- Documentation baseline SHA: pending baseline commit; recorded in follow-up
  handoff commit to avoid a commit containing its own hash.
- Remote branch / clean tree: pending final verification.

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

At the M0 gate, request approval for M1 only. Before claiming CAM correctness,
provide Rust 1.88.0 and execute the existing Rust tests, then the approved M1
corpus. `bd prime` could not run because bd is unavailable; no Beads database
was changed or issue status invented.
