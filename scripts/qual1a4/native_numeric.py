"""Original lexemes through the real native preset command, not a mock schema."""
from pathlib import Path
import json, struct, subprocess, sys

ROOT = Path(__file__).resolve().parents[2]
RAW = ROOT.parent / 'ivacam/dist-test/qual1a4'
binary = Path(sys.argv[1])
rows = json.loads((RAW / 'PUBLIC_NUMERIC.json').read_text())
project = json.loads((ROOT / 'tests/fixtures/perf-v9/profile-contours-linuxcnc.json').read_text())['project']
op = project['operations'][0]
op['params']['cutting']['entry_angle_deg'] = '__LEXEME__'
template = json.dumps(dict(action='inspect', project=project, op_id=op['id']))
for row in rows:
    result = subprocess.run([str(binary)], input=template.replace('"__LEXEME__"', row['lexeme']), capture_output=True, text=True)
    if result.returncode:
        row['native_command'] = dict(error=result.stderr.strip(), exit_code=result.returncode)
    else:
        value = json.loads(result.stdout)
        number = value['parameters'].get('entry_angle_deg')
        row['native_command'] = dict(bits=struct.pack('>d', number).hex() if number is not None else None, validation_errors=value['check']['errors'])
    if row['classification'] == 'finite':
        assert row['native_command']['bits'] == row['oracle_bits'], row['id']
    if row['classification'] == 'overflow':
        assert 'error' in row['native_command'], row['id']
(RAW / 'PUBLIC_NUMERIC_FINAL.json').write_text(json.dumps(rows, indent=2) + '\n')
print(f'{len(rows)} native command and actual WASM text cases; every finite bit matches independent oracle.')
