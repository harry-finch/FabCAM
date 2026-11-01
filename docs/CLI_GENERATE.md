# Explicit buffered CLI generation — V10

## Current request and examples

Run from the repository root on the source identified by [STATUS](shopbot-modernization/STATUS.md).
Build the native CLI with `cargo build --locked -p ivac-cli`. Set `IVAC` to the
binary from that source and `DOC_OUT` to a disposable output directory:

```sh
IVAC=./target/debug/ivac
DOC_OUT=$(mktemp -d)
"$IVAC" --help
"$IVAC" generate --request tests/fixtures/alpha32/scoped-pocket.request.json > "$DOC_OUT/pocket.response.json"
"$IVAC" generate --request tests/fixtures/alpha3-v10/profile.request.json > "$DOC_OUT/profile.response.json"
```

These are existing **V10 software fixtures**, not a V9 fixture renamed or a
physically tested recipe. The Profile fixture uses bundled CPS and needs a
CPS-enabled native build; Pocket uses LinuxCNC and explicit scoped Finishing.
Output is JSON, not an instruction to run any machine. Review geometry, actual
tools, stock, work origin, fixtures and every cutting value before actual use.

A request is a `PipelineRequest`, **not** the saved frontend ProjectFile envelope.
It contains `project.cutting_version: 10`, geometry, tools, machine and operations,
plus optional `post_processor`/`cps_post`. Profile/ordinary Pocket require explicit
Main and nullable optional Finishing under `main_finishing_v1` or Pocket-only
`pocket_finishing_v2`; other families retain their actual rate roles. See the
[cutting contract](CUTTING_CONTRACT.md) for nested discriminators, scopes and refusals.

V9/missing/future contracts and retired `fs_context`, `applied_cutting`,
`fs_full_width` fields refuse. No implicit migration, rate seeding or calculation.
Some CLI parse/usage diagnostics still contain the historical text “v9”; the actual
reader calls the current V10 validator. That diagnostic wording is a known source
prose gap, not permission to submit V9 or a documentation-only code correction.

## Explicit geometry template

Convenience import supports exactly one enabled Profile with explicit source All;
segments, text layers and relief sources must initially be empty. Machine, tool,
source, depth, Main DOC and SAFE-Z must be spelled out. Import supplies only geometry,
not tools, cutting values or a machine. No implicit object/layer remapping.

The following prepares an explicit V10 template and small synthetic physical-unit
SVG in the disposable directory, leaving tracked fixtures untouched:

```sh
python3 - "$DOC_OUT" <<'PYDOC'
import json, pathlib, sys
out = pathlib.Path(sys.argv[1])
r = json.loads(pathlib.Path('tests/fixtures/alpha3-v10/profile.request.json').read_text())
r['project']['segments'] = []
r['project']['text_layers'] = []
r['project']['relief_sources'] = []
r['post_processor'] = 'linuxcnc'
r['cps_post'] = None
(out / 'profile.template.json').write_text(json.dumps(r))
(out / 'square.svg').write_text('<svg xmlns="http://www.w3.org/2000/svg" width="20mm" height="20mm"><path d="M2 2L12 2L12 12L2 12Z"/></svg>')
PYDOC
"$IVAC" generate "$DOC_OUT/square.svg" --request "$DOC_OUT/profile.template.json" > "$DOC_OUT/import.response.json"
```

The request alone selects post/properties. Omitted post uses the shared LinuxCNC
default; `cps_post` conflicts with a non-CPS post. External CPS paths/source never
execute through this command. Removed geometry/post flags and unknown/duplicate
flags refuse; edit/review the request instead.

## Output and safety boundary

Successful stdout is one `PipelineResponse` with program, decoded/planned path,
stats, warnings, provenance/index and supported diagnostics. Tracing goes to stderr.
No JSON/program is published before generation succeeds. Shell redirection itself
is not transactional; output I/O failure can interrupt the response file.

`stream-gcode request.json --output file` retains per-file transactional publication
and its existing stream/refusal/resource limits. `--two-sided` requires an output
base, retains motion restrictions and may report partial publication if a later
side fails; it is not pair-atomic. Buffered generate adds no two-sided CLI mode.
SAFETY-FIX-1 refusals, output/simulation limits and post restrictions remain.
No controller, installed-native or physical acceptance follows from CLI success.
