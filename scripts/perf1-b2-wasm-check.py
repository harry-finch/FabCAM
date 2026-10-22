#!/usr/bin/env python3
"""Re-run direct WASM binding checks and retain explicit cross-backend deltas."""
import argparse,subprocess,json
from pathlib import Path
root=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser()
p.add_argument('--requests',type=Path,default=root/'tests/fixtures/perf-v9')
p.add_argument('--output',type=Path,required=True)
p.add_argument('--native-records',type=Path,required=True)
p.add_argument('--native-responses',type=Path,required=True)
args=p.parse_args()
r=args.output;r.mkdir(parents=True,exist_ok=True)
checks=[]
for request in sorted(args.requests.glob('*-linuxcnc.json')):
    out=r/request.name
    subprocess.run(['node',str(root/'frontend/scripts/perf1-wasm.mjs'),str(request),str(out),'7'],check=True)
    b=json.loads(out.read_text());a=json.loads((args.native_records/request.name).read_text())
    assert b['request_sha256']==a['request_sha256']
    assert all(x['success']==a['runs'][0]['success'] for x in b['runs'])
    row={'case':request.stem,'success':b['runs'][0]['success']}
    if row['success']:
        assert all(x['identical_to_first'] for x in b['runs'])
        n=json.loads((args.native_responses/(request.name+'.response.json')).read_text())
        w=json.loads(Path(str(out)+'.response.json').read_text())
        assert len(n['toolpath'])==len(w['toolpath'])
        delta=max((abs(a[p][k]-b[p][k]) for a,b in zip(n['toolpath'],w['toolpath']) for p in ['from','to'] for k in ['x','y','z']),default=0)
        row.update(program_equal=n['gcode']==w['gcode'],moves_exactly_equal=n['toolpath']==w['toolpath'],max_xyz_delta_mm=delta,remaining_equal=n['remaining_material']==w['remaining_material'])
        assert row['program_equal'] and row['remaining_equal']
        # Cross-runtime arc tessellation has existing floating roundoff. This
        # observation bound is not a geometry tolerance change or cache key.
        assert delta<1e-10
        assert all({k:v for k,v in a.items() if k not in ['from','to']}=={k:v for k,v in b.items() if k not in ['from','to']} for a,b in zip(n['toolpath'],w['toolpath']))
    else:
        assert all(x['error']==b['runs'][0]['error'] for x in b['runs'])
        assert b['runs'][0]['error']==a['runs'][0]['error']
    checks.append(row)
(r/'COMPARISONS.json').write_text(json.dumps(checks,indent=2)+'\n')
print(json.dumps(checks,indent=2))
