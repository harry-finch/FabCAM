# QUAL-1B bounded qualification

These scripts are developer diagnostics, not product entry points. They require
the preserved local `ivacam/dist-test/qual1a4` reference binaries and A1/A3/A4
immutable requests. Work in a fresh isolated checkout; never overwrite prior
campaign directories. Release settings: locked dependencies, Rust 1.88.0,
`RUSTC_WRAPPER=''`, `CARGO_INCREMENTAL=0`, `ivac-core` features `cps,profiling`.

1. Build the unmodified candidate `perf1_probe` and canonical `ivac-cli` separately;
   copy them into `dist-test/qual1b/perf-candidate` and `ivac-candidate`.
2. `python3 scripts/qual1b/compare-outputs.py` compares 86 complete native
   responses/refusals to A4. Also run `scripts/qual1a4/corpus.py` with the candidate
   CLI and a new output directory for the 23 frozen references.
3. `python3 scripts/qual1b/prepare-diagnostic.py` creates a separate archived source
   copy with timestamp/IR diagnostics. It adds no path or switch to production.
   Build `perf1_probe` using that copy's Cargo manifest with the same settings and
   retain it as `perf-diagnostic-candidate`. Then copy the generated
   `diagnostic-control-post.cps` over the copy's `crates/ivac-cps/posts/shopbot.cps`,
   rebuild with identical settings, retain as `perf-diagnostic-control`. The
   control post comes from qualified A4 SHA `8e2c9ce`; all other code is identical.
4. Use one complete and one record-65 injection pilot per binary/fixture to verify
   phase markers (`QUAL1B_INJECT='QUAL1B record65'`). Optional `QUAL1B_IR` captures
   IR only in pilots; exclude that I/O from measured campaigns. These environment
   variables exist only in the unshipped diagnostic source copy.
5. Finish all builds and tests before `python3 scripts/qual1b/measure.py`.
   It runs exactly two campaigns, 240 observations plus 20 discarded warmups each,
   alternating control/candidate order. It saves all observations then exits
   nonzero if any candidate cancellation p95 exceeds the unchanged 200 ms target.
   Do not rerun to select a favorable outcome. It refuses existing campaign folders.

Production fixed-delay measurements use actual native signalling from another
thread. Diagnostic injection writes the same real token synchronously from the
existing log callback to pin a preflight location; its timing is reported separately.
The normal test suite also signals from another thread inside deterministic phase
windows. The pre-cancelled diagnostic timer starts after the flag was set; it measures
handling time of an already-cancelled request, not time from the earlier flag write.

Keep initial failed harness attempts, complete raw outputs, source/binary hashes,
and host conditions. Baseline timing may vary without changing prior failed findings.
Native results are not WASM latency, installed-native or physical qualification.
See `docs/releases/QUAL_1B_COOPERATIVE_CANCELLATION.md` for scope and acceptance.
