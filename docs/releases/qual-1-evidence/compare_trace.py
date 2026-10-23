import json,math,collections
from pathlib import Path
root=Path(__file__).resolve().parent
def diffs(a,b,path=''):
 if type(a)!=type(b) and not isinstance(a,(int,float)): yield path,a,b; return
 if isinstance(a,dict):
  for k in sorted(a.keys()|b.keys()): yield from diffs(a.get(k),b.get(k),path+'/'+k)
 elif isinstance(a,list):
  if len(a)!=len(b): yield path+'/length',len(a),len(b)
  for i,(x,y) in enumerate(zip(a,b)): yield from diffs(x,y,path+'/'+str(i))
 elif a!=b:
  # delaunator uses usize::MAX as the EMPTY halfedge sentinel on each target.
  if 'halfedges' in path and {a,b}=={18446744073709551615,4294967295}: return
  yield path,a,b
all_results={}
for case in ['square-medial-linuxcnc','text-Test-medial-linuxcnc']:
 a=json.loads((root/'trace'/(case+'.native.json.trace.json')).read_text()); b=json.loads((root/'trace'/(case+'.wasm.json.trace.json')).read_text())
 summary=collections.defaultdict(lambda:dict(records=0,differing_records=0,numeric_max_abs=0,first=None))
 for i,(x,y) in enumerate(zip(a,b)):
  assert x['stage']==y['stage']
  s=summary[x['stage']]; s['records']+=1; d=list(diffs(x['data'],y['data']))
  if d:
   s['differing_records']+=1
   if s['first'] is None:s['first']={'record':i,'differences':d[:8]}
   for path,v,w in d:
    if isinstance(v,(int,float)) and isinstance(w,(int,float)):s['numeric_max_abs']=max(s['numeric_max_abs'],abs(v-w))
 all_results[case]=summary
 print(case,json.dumps(summary,indent=2))
(root/'trace'/'STAGE_SUMMARY.json').write_text(json.dumps(all_results,indent=2)+'\n')
