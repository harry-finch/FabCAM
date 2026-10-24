"""Final executable replay against immutable QUAL-1A.3 reviewed artifacts.

Requires the retained local evidence layout; never rewrites historical outputs.
The Profile internal-path exception remains explicit; no output normalization.
"""
from pathlib import Path
import hashlib
import json
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
PRIMARY = ROOT.parent / 'ivacam'
OLD = PRIMARY / 'dist-test/qual1a3'
OUT = Path(sys.argv[1]).resolve()
NODE = '/opt/homebrew/opt/node@24/bin/node'


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    start = time.monotonic()
    prior = json.loads((OLD / 'REPLAY.json').read_text())['cases']
    rows = []
    for previous in prior:
        name = previous['name']
        candidates = [PRIMARY / 'dist-test/qual1a/adversarial' / name,
                      ROOT / 'tests/fixtures/perf-v9' / name, OLD / 'adversarial' / name]
        request = next(p for p in candidates if p.exists())
        assert sha(request) == previous['request_sha256'], name
        row = {'name': name, 'request_sha256': sha(request), 'runtimes': {}}
        values = {}
        for label in ['candidate-native', 'candidate-wasm']:
            dest = OUT / 'replay' / label / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            cmd = [str(OUT / 'perf'), str(request), str(dest), '3'] if label.endswith('native') else [NODE, str(ROOT / 'frontend/scripts/perf1-wasm.mjs'), str(request), str(dest), '3']
            r = subprocess.run(cmd, capture_output=True, timeout=30)
            Path(str(dest)+'.stderr').write_bytes(r.stderr)
            runs = json.loads(dest.read_text())['runs'] if r.returncode == 0 else []
            success = bool(runs and runs[0]['success'])
            assert success == previous[label]['success'], (name, label, r.stderr)
            assert all(x.get('identical_to_first', True) for x in runs), (name, label)
            if success:
                value = json.loads(Path(str(dest)+'.response.json').read_text())
                reviewed = json.loads((OLD/'replay'/label/(name+'.response.json')).read_text())
                assert value == reviewed, ('UNREVIEWED OUTPUT CHANGE', name, label)
                values[label] = value
            else:
                error = runs[0].get('error') if runs else r.stderr.decode()
                # Only the diagnostic stack's checkout path differs. Keep raw stderr;
                # do not normalize a parser/CAM error or any successful output.
                prior_root = str(ROOT.parent / 'FabCAM-qual-1a3')
                compared = error.replace(str(ROOT / 'frontend/scripts/perf1-wasm.mjs'), prior_root + '/frontend/scripts/perf1-wasm.mjs') if isinstance(error, str) else error
                assert compared == previous[label]['error'], ('REFUSAL CHANGE', name, label, error)
            row['runtimes'][label] = {'success': success, 'reviewed_result_exact': True, 'exit': r.returncode}
        if values:
            a, b = values['candidate-native'], values['candidate-wasm']
            row['parity'] = {f: a[f] == b[f] for f in ['gcode','toolpath','gcode_index','stats']}
            assert all(row['parity'][f] for f in ['gcode','gcode_index','stats'])
            if not row['parity']['toolpath']:
                assert name == 'profile-many-segments-linuxcnc.json'
            r = subprocess.run([str(OUT/'ivac'),'generate','--request',str(request)],capture_output=True,timeout=30)
            assert r.returncode == 0 and json.loads(r.stdout) == a, ('PUBLIC CLI', name)
            row['public_cli_exact'] = True
        else:
            r = subprocess.run([str(OUT/'ivac'),'generate','--request',str(request)],capture_output=True,timeout=30)
            assert r.returncode != 0 and not r.stdout, ('PUBLIC REFUSAL', name)
            row['public_cli_refusal'] = r.stderr.decode()
        rows.append(row)
        (OUT/'REPLAY.json').write_text(json.dumps({'seconds':time.monotonic()-start,'cases':rows},indent=2)+'\n')
        print(name, 'reviewed output/refusal exact', flush=True)
    assert len(rows) == 77
    print('77 reviewed cases; original 40 successes/21 refusals plus 16 nearby cases.')


if __name__ == '__main__':
    main()
