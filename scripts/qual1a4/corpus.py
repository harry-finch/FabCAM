"""Read-only comparison of every frozen explicit output fixture via the public CLI."""
from pathlib import Path
import hashlib
import json
import subprocess
import sys

binary=Path(sys.argv[1]).resolve()
out=Path(sys.argv[2]).resolve();out.mkdir(exist_ok=True)
root=Path(__file__).resolve().parents[2]
fields=['gcode','toolpath','tool_provenance','gcode_index','stats']
rows=[]
for path in sorted((root/'tests/fixtures/fs-reset3-output').glob('*.json')):
    if path.name=='SHA256SUMS.json':continue
    fixture=json.loads(path.read_text());request=out/path.name
    request.write_text(json.dumps(fixture['request'])+'\n')
    row=dict(case=path.name,fixture_sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    if 'two_sided' in fixture:
        r=subprocess.run([str(binary),'stream-gcode',str(request),'--two-sided','--output',str(out/'two-sided')],capture_output=True,timeout=60)
        row['returncode']=r.returncode
        row['fields']={}
        for side in ('front','back'):
            matches=[p for p in out.glob('two-sided*') if side in p.name and p.suffix!='json']
            row['fields'][side+'.gcode']=len(matches)==1 and matches[0].read_text()==fixture['two_sided'][side]['gcode']
    else:
        r=subprocess.run([str(binary),'generate','--request',str(request)],capture_output=True,timeout=60)
        row['returncode']=r.returncode
        (out/(path.stem+'.response.json')).write_bytes(r.stdout)
        value=json.loads(r.stdout) if r.returncode==0 else {}
        row['fields']={f:value.get(f)==fixture['response'].get(f) for f in fields}
    (out/(path.stem+'.stderr')).write_bytes(r.stderr)
    rows.append(row)
    print(path.name,row['fields'],flush=True)
receipt=dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),cases=rows,
             scope='22 public buffered v9 requests and one two-sided publication; no installed-native or physical claim.')
(out/'summary.json').write_text(json.dumps(receipt,indent=2)+'\n')
sys.exit(0 if all(x['returncode']==0 and all(x['fields'].values()) for x in rows) else 1)
