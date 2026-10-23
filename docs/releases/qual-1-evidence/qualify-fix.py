import subprocess,json,hashlib,datetime
from pathlib import Path
root=Path(__file__).resolve().parents[2]; p=root/'dist-test/qual1'; exe=p/'perf-fixed'
# Deliberately one post-fix campaign; the previous failed campaign is retained.
subprocess.run(['cp',str(root/'target/release/examples/perf1_probe'),str(exe)],check=True)
e=p/'cancel-fixed';e.mkdir(exist_ok=True);(e/'warmup').mkdir(exist_ok=True)
(e/'conditions.json').write_text(json.dumps({'protocol':'One post-fix campaign, identical fixed 70 observations and six discarded warmups; no task-owned compilation concurrently. Control=pre-fix 932ee4f production, candidate=64-record polling fix. No retry based on result. Browser idle; background host load uncontrolled.','time':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2)+'\n')
for label,binary in [('control',p/'perf-current'),('candidate',exe)]:
 for case,delay in [('text-Test-medial-cps',50),('profile-many-segments-cps',50),('text-Test-medial-cps',1)]:
  subprocess.run([str(binary),str(root/'tests/fixtures/perf-v9'/(case+'.json')),str(e/'warmup'/f'{label}-{case}-{delay}.json'),'1','cancel',str(delay)],check=True)
with (p/'cancel-fixed.log').open('w') as log:r=subprocess.run(['python3',str(root/'scripts/perf1-cancel.py'),'--control',str(p/'perf-current'),'--candidate',str(exe),'--output',str(e/'observations')],stdout=log,stderr=subprocess.STDOUT)
(p/'cancel-fixed-exit.json').write_text(json.dumps({'exit_code':r.returncode})+'\n')
# No retry on latency failure. Continue independent output-equivalence checks.
rows=[]; out=p/'perf-fixed-output';out.mkdir(exist_ok=True)
for item in json.loads((root/'tests/fixtures/perf-v9/manifest.json').read_text()):
 name=item['name']; req=root/'tests/fixtures/perf-v9'/name; result=out/name
 assert hashlib.sha256(req.read_bytes()).hexdigest()==item['sha256']
 subprocess.run([str(exe),str(req),str(result),'2'],check=True,timeout=90)
 a=json.loads(result.read_text()); b=json.loads((root/'dist-test/fs-reset3/perf-functional'/name).read_text()); ok=a['runs'][0]['success'];assert ok==b['runs'][0]['success']
 row={'case':name,'success':ok,'request_sha256':item['sha256']}
 if ok:
  x=json.loads(Path(str(result)+'.response.json').read_text());y=json.loads((root/'dist-test/fs-reset3/perf-functional-responses'/(name+'.response.json')).read_text())
  row['equal']={k:x.get(k)==y.get(k) for k in ['gcode','toolpath','tool_provenance','gcode_index','stats']};assert all(row['equal'].values()),row
  assert all(t['identical_to_first'] for t in a['runs'])
 else:
  row['error']=a['runs'][0]['error'];assert all(t['error']==row['error'] for t in a['runs']+b['runs'])
 rows.append(row)
(p/'FIX_OUTPUT_COMPARISON.json').write_text(json.dumps(rows,indent=2)+'\n')
print('Output comparison: all',len(rows),'exact; cancellation exit',r.returncode)
