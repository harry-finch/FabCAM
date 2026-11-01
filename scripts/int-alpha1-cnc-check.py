"""Compare the hash-bound ALPHA corpus against an immutable accepted ALPHA-4 CLI.

Reads only synthetic qualification inputs. Detailed requests/responses stay local.
Usage: python3 scripts/int-alpha1-cnc-check.py [ALPHA-3.2 evidence worktree]
"""
from pathlib import Path
import copy
import hashlib
import json
import subprocess
import sys
import re

root = Path(__file__).resolve().parents[1]
evidence_tree = Path(sys.argv[1]) if len(sys.argv) > 1 else root.parent / 'FabCAM-alpha-3-2'
out = root / 'dist-test/int-alpha1/cnc'
out.mkdir(parents=True, exist_ok=True)
baseline = root / 'dist-test/int-alpha1/baseline-ivac'
candidate = root / 'target/debug/ivac'
sha = lambda data: hashlib.sha256(data).hexdigest()
expected = json.loads((root / 'docs/releases/evidence/alpha4/qualification.json').read_text())['binary_sha256']['target/debug/ivac']
assert sha(baseline.read_bytes()) == expected, 'accepted ALPHA-4 CLI snapshot differs'
receipt = json.loads((root / 'docs/releases/evidence/alpha32/runtime-comparison.json').read_text())
cases = []
for row in receipt['cases']:
    p = evidence_tree / 'dist-test/alpha32/runtime' / (row['name'] + '.request.json')
    raw = p.read_bytes()
    assert sha(raw) == row['requestSha256'], row['name']
    cases.append(('alpha32-' + row['name'], json.loads(raw), row['accepted']))
retained = json.loads((root / 'docs/releases/evidence/alpha32/retained-corpus.json').read_text())
for name in sorted({r['fixture'] for r in retained['rows']}):
    v = json.loads((evidence_tree / 'dist-test/alpha32/retained' / name).read_text())
    cases.append(('retained-' + Path(name).stem, v['v10_request'], True))
# The ALPHA-4 requests are reproducible disposable products of its committed runtime check.
for p in sorted((root.parent / 'FabCAM-alpha-4/dist-test/alpha4/runtime').glob('*.request.json')):
    cases.append(('alpha4-' + p.stem.removesuffix('.request'), json.loads(p.read_text()), 'target-machine-limit' not in p.name and 'shopbot' not in p.name))
rows = []
def run(binary, payload, label):
    path = out / (label + '.request.json')
    path.write_text(json.dumps(payload))
    r = subprocess.run([str(binary), 'generate', '--request', str(path)], capture_output=True, timeout=60)
    return r.returncode == 0, json.loads(r.stdout) if r.returncode == 0 else r.stderr.decode()
for name, req, accepted in cases:
    ok, control = run(baseline, req, name + '-accepted')
    assert ok == accepted, (name, 'baseline expectation changed', control if not ok else '')
    integrated_ok, actual = run(candidate, req, name + '-integrated')
    assert integrated_ok == ok and actual == control, (name, 'unmodified input differential')
    changed = copy.deepcopy(req)
    for tool in changed['project']['tools']:
        tool.update(substrate='HSS-Co custom', coating='uncoated')
    metadata_ok, metadata = run(candidate, changed, name + '-metadata')
    assert metadata_ok == ok, (name, 'metadata changed acceptance')
    if ok:
        for b in metadata['tool_provenance']['bindings']:
            assert b['tool'].pop('substrate') == 'HSS-Co custom'
            assert b['tool'].pop('coating') == 'uncoated'
        assert metadata == control, (name, 'metadata changed output beyond descriptive tool snapshot')
        rows.append({'name': name, 'accepted': True, 'request_sha256': sha(json.dumps(req).encode()), 'program_sha256': sha(control['gcode'].encode()), 'moves': len(control['toolpath']), 'all_response_fields_exact': True, 'metadata_only_in_tool_snapshots': True})
    else:
        # Adding descriptive JSON fields changes only the parser's physical column.
        location = r' at line [0-9]+ column [0-9]+(?=\n|$)'
        assert re.sub(location, '', metadata) == re.sub(location, '', control), (name, 'refusal reason changed')
        rows.append({'name': name, 'accepted': False, 'unchanged_input_exact_refusal': True, 'metadata_refusal_reason_exact': True, 'parser_location_only_difference': metadata != control})
    if name == 'retained-two-sided':
        side_outputs = []
        for binary, payload, label in [(baseline, req, 'accepted'), (candidate, req, 'integrated'), (candidate, changed, 'metadata')]:
            path = out / ('two-' + label + '.json'); path.write_text(json.dumps(payload))
            base = out / ('two-' + label)
            result = subprocess.run([str(binary), 'stream-gcode', str(path), '--two-sided', '--output', str(base)], capture_output=True, timeout=60)
            assert result.returncode == 0, result.stderr.decode()
            files = {side: next(x for x in out.glob(base.name + '*') if side in x.name and x.suffix != '.json').read_bytes() for side in ['front','back']}
            side_outputs.append(files)
        assert side_outputs[0] == side_outputs[1] == side_outputs[2]
        rows[-1]['both_streamed_sides_exact'] = True
summary = {'baseline_source': '78be45928f8c63ccb284787f90d9d5c065765717', 'baseline_binary_sha256': sha(baseline.read_bytes()), 'candidate_binary_sha256': sha(candidate.read_bytes()), 'synthetic_only': True, 'policy': 'Exact complete native response for unchanged inputs; metadata variant differs only in the two explicitly asserted descriptive snapshot fields. No golden updates.', 'cases': rows}
(root / 'docs/releases/evidence/int-alpha1/cnc-comparison.json').write_text(json.dumps(summary, indent=2)+'\n')
print(f'{len(rows)} exact baseline/candidate/metadata comparisons; two-sided stream preserved')
