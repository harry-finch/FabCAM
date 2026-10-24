# QUAL-1A.4 production qualification

These diagnostics preserve the reviewed QUAL-1A.3 outputs and compare fresh
production binaries; they do not create a new CAM implementation or tolerance.
They require the retained sibling `ivacam/dist-test/qual1a*` evidence layout.
Use a new owned output directory, with freshly built `ivac`, `perf1_probe` copied
as `perf`, `qual1a2_command` as `preset-command`, and actual fresh WASM in this
checkout. Do not overwrite a previous qualification run.

`replay.py OUTPUT` executes the 77 frozen requests three times through native
and actual WASM and compares their complete outputs with QUAL-1A.3. It also
checks canonical CLI output. Every successful program must match exactly;
the pre-existing Profile internal-path difference remains explicit. Only the
checkout pathname in a JSON.parse diagnostic stack is mapped for comparison;
raw stderr and every substantive error remain retained, without normalization.

`corpus.py CLI OUTPUT` compares all 23 immutable CLI output references, including
both two-sided files. Streaming does not gain buffered simulation qualification.
`public_numeric.mjs OUTPUT` and `native_numeric.py COMMAND` exercise original
lexemes through actual preset commands; the latter uses the recorded local
QUAL-1A.4 output directory. Copy `qual1a4-numeric.json` there as `NUMERIC.json`
first. The 328 expectations come from the independent exact rational oracle,
including all 126 discrepant coordinate occurrences; the same cases run in the
ordinary Rust suite. Inspection of invalid angles is not generation acceptance.

Full source qualification additionally runs the unchanged pre-release script,
unfiltered all-target workspace tests, TEST1/C2, frontend suite and actual
WASM, numerical/storage/transport tests, and accepted independent geometry/order
oracles. A failed gate remains failed. See the source-bound report for exact
commands, fingerprints, performance, UI and installed-host limitations.
