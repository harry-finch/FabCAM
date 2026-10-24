"""Immutable QUAL-1A.4 full-response comparison; never update golden files."""
from pathlib import Path
import hashlib,json,subprocess
root=Path(__file__).resolve().parents[2];primary=root.parent/'ivacam';out=root/'dist-test/qual1b/output-final';out.mkdir(exist_ok=True)
prior=json.loads((primary/'dist-test/qual1a4/REPLAY.json').read_text())['cases']
requests=[]
for case in prior:
 name=case['name']; paths=[primary/'dist-test/qual1a/adversarial'/name,root/'tests/fixtures/perf-v9'/name,primary/'dist-test/qual1a3/adversarial'/name]
 path=next(p for p in paths if p.exists()); assert hashlib.sha256(path.read_bytes()).hexdigest()==case['request_sha256'];requests.append(path)
for path in sorted((root/'tests/fixtures/perf-v9').glob('*-cps.json')):
 if path not in requests: requests.append(path)
rows=[]
for req in requests:
 row={'case':req.name,'sha256':hashlib.sha256(req.read_bytes()).hexdigest()};values=[]
 for label,binary in [('control',primary/'dist-test/qual1a4/perf'),('candidate',root/'dist-test/qual1b/perf-candidate')]:
  target=out/(label+'-'+req.name)
  result=subprocess.run([str(binary),str(req),str(target),'2'],capture_output=True,timeout=60)
  (out/(label+'-'+req.name+'.stderr')).write_bytes(result.stderr)
  if result.returncode != 0:
   values.append((False,result.stderr));continue
  runs=json.loads(target.read_bytes())['runs'];assert all(r.get('identical_to_first',True) for r in runs)
  success=runs[0]['success'];values.append((success,Path(str(target)+'.response.json').read_bytes() if success else runs[0]['error']))
 assert values[0]==values[1],('UNEXPLAINED OUTPUT DIFFERENCE',req.name)
 row.update(success=values[0][0],byte_identical_response_or_refusal=True);rows.append(row)
 (out/'summary.json').write_text(json.dumps(rows,indent=2)+'\n')
 print(req.name,'exact',flush=True)
print(len(rows),'full-response/refusal comparisons')
