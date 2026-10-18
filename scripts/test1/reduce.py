#!/usr/bin/env python3
"""Bounded failure-preserving reduction. Original remains immutable."""
import argparse,json,pathlib,subprocess,time
p=argparse.ArgumentParser();p.add_argument('--binary',required=True);p.add_argument('--case',required=True);p.add_argument('--output',required=True);a=p.parse_args()
out=pathlib.Path(a.output);out.mkdir(parents=True,exist_ok=False)
original=json.loads(pathlib.Path(a.case).read_text());(out/'original.json').write_text(json.dumps(original,indent=2))
start=time.monotonic();attempts=[]
def run(c):
 f=out/f'input-{len(attempts)}.json';f.write_text(json.dumps(c));dest=out/f'trial-{len(attempts)}'
 try:subprocess.run(['python3',str(pathlib.Path(__file__).with_name('run.py')),'--binary',a.binary,'--replay',str(f),'--output',str(dest),'--budget','10'],timeout=min(12,max(.001,60-(time.monotonic()-start))),capture_output=True)
 except subprocess.TimeoutExpired:return None
 result=dest/f"{c['property']}-{c['index']:04d}"/'result.json'
 if not result.exists():return None
 error=json.loads(result.read_text())['result'].get('Err');attempts.append(dict(input=c,error=error))
 return error.split(':')[0] if error else None
signature=run(original)
if not signature:raise SystemExit('Original does not reproduce a semantic failure; timeout/pass is not a shrink witness.')
best=original.copy()
# Only generator-preserving reductions: labelled-source count and width variation.
for field,values in [('complexity',[2,1]),('scale',[.5,.25,0.])]:
 for value in values:
  if len(attempts)>=128 or time.monotonic()-start>=48:break
  if value>=best[field]:continue
  trial=dict(best,**{field:value})
  if run(trial)==signature:best=trial
(out/'minimized.json').write_text(json.dumps(best,indent=2));(out/'reduction.json').write_text(json.dumps(dict(signature=signature,trials=attempts,elapsed=time.monotonic()-start,limit_trials=128,limit_seconds=60),indent=2))
