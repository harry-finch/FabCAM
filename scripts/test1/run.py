#!/usr/bin/env python3
"""TEST1 native workers; build separately, never mistake omitted C2 for qualification."""
import argparse, hashlib, json, os, pathlib, platform, subprocess, time
p=argparse.ArgumentParser()
p.add_argument('--binary',required=True);p.add_argument('--output',required=True)
p.add_argument('--cases',type=int,default=32);p.add_argument('--budget',type=float,default=120)
p.add_argument('--frontend', action='store_true');p.add_argument('--replay');p.add_argument('--property',choices=['A1','A2','B1','B2','C1','D1','D2'])
a=p.parse_args()
if not 1 <= a.cases <= 512 or not 0 < a.budget <= 600:p.error('cases must be 1..512 and budget (0,600] seconds')
out=pathlib.Path(a.output);out.mkdir(parents=True,exist_ok=False)
mask=(1<<64)-1
def splitmix(x):
 x=(x+0x9e3779b97f4a7c15)&mask;x=((x^(x>>30))*0xbf58476d1ce4e5b9)&mask;x=((x^(x>>27))*0x94d049bb133111eb)&mask
 return x^(x>>31)
cases=[]
for prop,ident in [('A1',1),('A2',2),('B1',3),('B2',4),('C1',5),('D1',7),('D2',8)]:
 if a.property and prop!=a.property:continue
 for i in range(a.cases):cases.append(dict(property=prop,index=i,seed=splitmix(0x5445535431000001^(ident<<32)^i),complexity=1+i%4,scale=0.25))
if a.replay:cases=[json.loads(pathlib.Path(a.replay).read_text())]
start=time.monotonic();results=[]
for c in cases:
 d=out/f"{c['property']}-{c['index']:04d}";d.mkdir();f=d/'case.json';f.write_text(json.dumps(c,indent=2)+'\n')
 remaining=a.budget-(time.monotonic()-start)
 if remaining<=0:results.append(dict(case=c,status='stage_timeout'));break
 env=dict(os.environ,TEST1_CASE=str(f.resolve()),TEST1_OUT=str(d.resolve()))
 t=time.monotonic()
 try:
  r=subprocess.run([str(pathlib.Path(a.binary).resolve()),'--ignored','--exact','test1_case','--nocapture'],env=env,capture_output=True,timeout=min(10,remaining))
  (d/'stdout.txt').write_bytes(r.stdout);(d/'stderr.txt').write_bytes(r.stderr);status='pass' if r.returncode==0 else 'fail'
 except subprocess.TimeoutExpired as e:
  status='timeout';(d/'stdout.txt').write_bytes(e.stdout or b'');(d/'stderr.txt').write_bytes(e.stderr or b'')
 results.append(dict(case=c,status=status,elapsed_seconds=time.monotonic()-t))
if a.frontend:
 remaining=a.budget-(time.monotonic()-start)
 report=out.resolve()/'frontend.json'
 status='stage_timeout'
 if remaining>0:
  try:
   r=subprocess.run(['node','node_modules/vitest/vitest.mjs','run','src/lib/state/test1-lifecycle.test.ts','--reporter=json','--outputFile='+str(report)],cwd='frontend',capture_output=True,timeout=min(10,remaining))
   (out/'frontend.stdout.txt').write_bytes(r.stdout);(out/'frontend.stderr.txt').write_bytes(r.stderr)
   data=json.loads(report.read_text()) if report.exists() else {}
   status='pass' if r.returncode==0 and data.get('numPassedTests')==32 and data.get('numTotalTests')==32 else 'fail'
  except subprocess.TimeoutExpired:status='timeout'
 for i in range(32):results.append(dict(case=dict(property='C2',index=i),status=status))
summary=dict(generator='SplitMix64-v1',platform=platform.platform(),planned=len(cases)+(32 if a.frontend else 0),executed=len(results),elapsed_seconds=time.monotonic()-start,results=results,scope='bounded software properties; full SAFE-Z qualification NOT included')
(out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
files=sorted(f for f in out.rglob('*') if f.is_file())
(out/'SHA256SUMS.txt').write_text(''.join(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f.relative_to(out))+'\n' for f in files))
print(json.dumps({k:v for k,v in summary.items() if k!='results'}));print({s:sum(r['status']==s for r in results) for s in ['pass','fail','timeout','stage_timeout']})
raise SystemExit(0 if len(results)==summary['planned'] and all(r['status']=='pass' for r in results) else 1)
