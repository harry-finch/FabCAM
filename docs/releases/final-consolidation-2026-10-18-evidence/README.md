# Final consolidation evidence

Software only. `integrity.json` verifies selected previous source hashes and three
historical archive manifests. `source-SHA256SUMS.txt` is the current broader source
reference; verify from repository root. `verification.json` records final source
recheck and current report local-link checks. `focused-tests.log` has29 PASS.

`resource-replay/` is deliberately FAILED at20167 moves against20000, no timeout;
not a newly passing case or safety proof. It preserves request, response, stderr,
seed and generator summary. Its programs are test data, not machine instructions.
The replay command and inherited-vs-fresh test distinction are in the parent report.

Verify this directory's manifest from this directory with `shasum -a 256 -c SHA256SUMS.txt`.
