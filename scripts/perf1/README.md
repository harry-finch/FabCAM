# PERF-1 bounded manual measurements

Run from the repository root on an isolated writable PERF-1 checkout. Never run
paired measurements alongside builds, other benchmarks or a release gate. Node,
pnpm and the pinned Rust toolchain must be on PATH. Raw output belongs in ignored
`dist-test/perf-1/`; no private drawings, build products or raw logs are versioned.
See [source-bound results](../../docs/releases/PERF_1_MEASURED_OPTIMIZATION.md).

- The eight `tests/fixtures/perf1/*.json` are synthetic V10 pipeline requests.
  Build the unchanged accepted starting revision and candidate with
  `cargo build --locked --release -p ivac-cli`; retain separate immutable executables
  as `dist-test/perf-1/baseline-ivac` and `candidate-ivac`. Record their SHA-256.
  `python3 scripts/perf1/bench-native.py baseline` (then `candidate`) performs one
  warmup and seven fresh CLI runs per case. macOS `/usr/bin/time -l` reports RSS;
  wall time includes startup, not compilation. The host must permit time's sysctl.
- Build both production default packages with
  `wasm-pack build crates/ivac-wasm --target web --release` in their respective
  source trees. Preserve separate `baseline-wasm` / `candidate-wasm` packages under
  the same ignored directory. `python3 scripts/perf1/bench-wasm.py baseline` (then
  `candidate`) measures seven fresh instances after one warmup. Compile/init are
  excluded; JS conversion and generation are included. Node runs the release web
  package, not a debug or CPS-enabled package. Linear memory and RSS are recorded.
- `python3 scripts/perf1/paired-frontend.py` temporarily substitutes **only**
  `program-qualification.ts` with its committed starting revision, runs the shared
  Vitest benchmark and builds both production browser harnesses. It restores the
  candidate in `finally`. Do not edit the file concurrently. The baseline-only
  environment skips the candidate serialization-count regression, not safety tests.
  Serve the output on loopback with the production CSP, open each
  `browser-{baseline,candidate}/bench/perf1.html?api=wasm`, and retain visible JSON.
  This measures synchronous diagnostics after edits, not a complete rendered UI
  gesture. Five warmups, seven rounds of 20 edits; separate real snapshot/unsaved
  checks and history cycles. Heap samples are observational, not a leak proof.
- The two ignored Rust `stage_polygon::perf1` manual tests require
  `PERF1_NATIVE_PROOF_OUT` / `PERF1_CHAIN_OUT`. Run a release unit binary with
  `--ignored stage_polygon::perf1 --test-threads=1`. The proof test needs original
  native response files from the first bullet. For original-math proof timings,
  use the starting production source with only the same test instrumentation;
  record that temporary source identity. Budget counters compile out of production.
- `compare-corpus.py --help` documents the hash-bound accepted SEC runtime replay.
  It requires existing local evidence; it neither fabricates missing requests nor
  updates goldens. Original request/program hashes and complete responses/refusals
  must match. Source equality is insufficient to claim execution if evidence is
  absent.

Chaining is measured only: its existing spatial index, tolerances, coordinates,
identities and matching order are unchanged. Manual timing tests remain excluded
from ordinary CI because machine-speed assertions would be unstable; their
correctness/equivalence assertions are exercised explicitly during PERF-1.
