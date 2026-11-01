"""Replay a hash-bound SEC-HARDEN-1 local corpus without changing its evidence."""
import argparse, hashlib, json, subprocess
from pathlib import Path
p=argparse.ArgumentParser(__doc__)
p.add_argument('--runtime',type=Path,required=True)
p.add_argument('--receipt',type=Path,required=True)
p.add_argument('--baseline',type=Path,required=True)
p.add_argument('--candidate',type=Path,required=True)
p.add_argument('--output',type=Path,required=True)
a=p.parse_args(); sha=lambda b:hashlib.sha256(b).hexdigest(); rows=[]
for row in json.loads(a.receipt.read_text())['fixed_corpus']:
 name=row['name']; request=a.runtime/(name+'-candidate.request.json')
 assert sha(request.read_bytes())==row['request_sha256'],name
 results=[subprocess.run([str(exe.resolve()),'generate','--request',str(request.resolve())],capture_output=True,timeout=60) for exe in [a.baseline,a.candidate]]
 x,y=results; assert x.returncode==y.returncode,name
 assert (y.returncode==0)==row['accepted'],name
 if row['accepted']:
  before=json.loads(x.stdout); after=json.loads(y.stdout)
  assert before==after,(name,'complete response changed')
  assert sha(after['gcode'].encode())==row['program_sha256'],(name,'accepted SEC output changed')
  retained=json.loads((a.runtime/(name+'-candidate.stdout')).read_bytes())
  assert after==retained,(name,'accepted SEC response changed')
  digest=sha(json.dumps(after,sort_keys=True,separators=(',',':')).encode())
 else:
  assert x.stderr==y.stderr,(name,'refusal changed')
  assert not y.stdout.strip(),(name,'program emitted')
  digest=sha(y.stderr)
 rows.append(dict(row,exact_complete_response_or_refusal=True,response_sha256=digest))
assert len(rows)==109,len(rows)
a.output.write_text(json.dumps({'baseline_binary_sha256':sha(a.baseline.read_bytes()),'candidate_binary_sha256':sha(a.candidate.read_bytes()),'rows':rows},indent=2)+'\n')
print('PASS 109 original hash-bound complete responses/refusals; accepted SEC bytes preserved')
