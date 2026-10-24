# QUAL-1A.4 compact evidence

Read [the production integration report](../QUAL_1A_4_PRODUCTION_INTEGRATION.md)
for scope, exact revisions, acceptance limits and the human hard stop.

- `FINAL_SOURCE.json`: exact source/tree identity and retained executable hashes.
- `CHANGED_FILES.json`: first two commits' changed-file hashes; the later
  diagnostic-only correction is identified by FINAL_SOURCE and its focused commit.
- `DEPENDENCY.json`, `features-*`: official libm provenance and effective runtime features.
- `PUBLIC_NUMERIC_FINAL.json`, `INPUT_BITS.json`: 328 public numeric cases and the
  126 corrected dense coordinate occurrences. Inspector acceptance is not generation.
- `PUBLIC_CAM_REFUSALS_CORRECTED.json`: valid control plus exact CAM validation
  reasons after isolating the numeric lexeme placeholder. Earlier misleading
  early-parser refusals remain in the raw evidence and are excluded from proof.
- `REPLAY.json`, `CPS.json`, `STANDALONE_CLI.json`, `CLI_CORPUS.json`, `WORKER.json`:
  immutable reviewed generation/refusal comparisons and known Profile exception.
- `PRESETS.json`: 48 expected old/new runtime outcomes; unchanged original bytes.
- `QUALIFICATION.json`, final full logs, TEST1 and oracle logs: counts, failures,
  unchanged guards and exact commands detailed in the report. Earlier logs are
  retained separately; do not add repeated runs together as unique coverage.
- `PERFORMANCE_SUMMARY.json`, `benchmark.py`: protocol, all three configurations,
  parser/build/size/resources. The benchmark requires retained sibling artifacts
  and a fresh owned output directory. It is not an installed-platform benchmark.
- `UI.json`, `ui-*`: real interactive observations, screenshots and transfer denial.
  A screenshot is not proof of an installed/native export or a physical cut.
- `PRESERVATION_PRE_FF.json`, `INTEGRATION.json`: protected files and actual safe
  local integration. Raw starting hashes remain in the ignored directory.
- `RAW_SHA256SUMS.txt`: retained raw evidence manifest, excluding its own manifest
  and any later final handoff receipt. `SHA256SUMS.txt` covers this compact directory
  except itself.

Raw evidence root:
`/Users/mugen/Documents/01_Projets/FabCAM/ivacam/dist-test/qual1a4/`.
This includes all original failed attempts, full responses, TEST1 worker records,
binaries, timing samples and input hashes. No historical expected output was
rewritten. Historical optional external FANUC/DXF references were absent; their
self-skipping tests do not count as actual oracle coverage.

The final report-only commit adds no runtime behavior. No approval or release
follows from these receipts; mandatory gates remain 17/20.
