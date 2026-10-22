# Explicit v9 performance fixtures

Software-only inputs, authored for FS-RESET-3 from the preserved PERF1 requests.
The manifest binds both source and current bytes. Geometry, stock, text, post,
tool assignment and former effective numerical values are retained. Explicit
cutting roles were seeded once using the test fixture constructor at f4896e4;
these saved numbers are now the sole generation inputs. No production conversion
or recommendation fallback is involved. Empty old pocket overlap becomes its
previous effective 0.5 at authoring only. Missing-layer cases retain their missing-source warning and no cutting moves (LinuxCNC retains a header retract); a
successful response envelope must never count as successful cutting.

Use `scripts/perf1-native.py --requests tests/fixtures/perf-v9` with explicit
`--binary`, `--output`, `--work`. WASM and cancellation scripts accept current
requests and require new output directories; historical evidence is not overwritten.
The frontend authoring script writes fresh developer candidates under dist-test,
using a test-only constructor; it does not migrate user files. Review new inputs
and hashes before replacing these fixtures. Retain native/WASM 1e-10 observation
bound, byte-exact programs, exact non-coordinate metadata and 200 ms p95 cancellation.
