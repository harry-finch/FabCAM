"""Diagnostic measurements only: no changed parity tolerance or acceptance rule."""
from pathlib import Path
import json,math,difflib,re,hashlib
root=Path('dist-test/qual1/reproduce')
def point(p):return tuple(p[k] for k in ('x','y','z'))
def distance(a,b):return math.sqrt(sum((x-y)**2 for x,y in zip(a,b)))
def seg_distance(p,a,b):
 d=[y-x for x,y in zip(a,b)];l=sum(v*v for v in d);t=max(0.,min(1.,sum((x-y)*z for x,y,z in zip(p,a,d))/l)) if l else 0.
 return distance(p,tuple(x+t*z for x,z in zip(a,d)))
def metrics(v):
 moves=v['toolpath'];points=[point(m['from']) for m in moves]+[point(m['to']) for m in moves]
 return dict(moves=len(moves),extents={k:[min(p[i] for p in points),max(p[i] for p in points)] for i,k in enumerate(('x','y','z'))},length_by_kind={k:sum(distance(point(m['from']),point(m['to'])) for m in moves if m['kind']==k) for k in sorted({m['kind'] for m in moves})},move_counts={k:sum(m['kind']==k for m in moves) for k in sorted({m['kind'] for m in moves})},tool_refs=sorted({str(m.get('tool_ref')) for m in moves}),modal_commands=[s for s in v['gcode'].splitlines() if re.search(r'(^|\s)([TSFM][0-9])',s) and not s.startswith('(')])
results=[]
for name in ['square-medial-linuxcnc','text-Test-medial-linuxcnc']:
 a=json.loads((root/(name+'.native.json.response.json')).read_text());b=json.loads((root/(name+'.wasm.json.response.json')).read_text())
 def keys(v):return [(point(m['to']),m['kind'],m.get('tool_ref')) for m in v['toolpath']]
 ka,kb=keys(a),keys(b);sm=difflib.SequenceMatcher(a=ka,b=kb,autojunk=False);blocks=[]
 for tag,ai,aj,bi,bj in sm.get_opcodes():
  if tag=='equal':continue
  pa=[point(a['toolpath'][max(0,ai-1)]['to'])]+[k[0] for k in ka[ai:min(aj+1,len(ka))]]
  pb=[point(b['toolpath'][max(0,bi-1)]['to'])]+[k[0] for k in kb[bi:min(bj+1,len(kb))]]
  def samples(ps):
   return ps+[tuple(x+t*(y-x) for x,y in zip(u,v)) for u,v in zip(ps,ps[1:]) for t in (.25,.5,.75)]
  def directed(ps,qs):return max((min(seg_distance(p,u,v) for u,v in zip(qs,qs[1:])) for p in samples(ps)),default=0.)
  delta=max(directed(pa,pb),directed(pb,pa))
  blocks.append(dict(kind=tag,native_range=[ai,aj],wasm_range=[bi,bj],native_to=ka[ai:aj],wasm_to=kb[bi:bj],sampled_bidirectional_distance_mm=delta,native_length_mm=sum(distance(u,v) for u,v in zip(pa,pa[1:])),wasm_length_mm=sum(distance(u,v) for u,v in zip(pb,pb[1:]))))
 first=next(i for i,(x,y) in enumerate(zip(ka,kb)) if x!=y)
 na=[{k:v for k,v in m.items() if k!='gcode_line'} for m in a['toolpath'] if m['kind']!='cut'];nb=[{k:v for k,v in m.items() if k!='gcode_line'} for m in b['toolpath'] if m['kind']!='cut']
 row=dict(case=name,native=metrics(a),wasm=metrics(b),first_ordered_divergence=first,non_cut_moves_exact=na==nb,first_endpoint_distance_mm=distance(ka[first][0],kb[first][0]),max_sampled_matched_block_distance_mm=max(x['sampled_bidirectional_distance_mm'] for x in blocks),changed_blocks=blocks,scope='Exact sequence matching; sample at vertices and quarter points of differing blocks. Observational distance, NOT a certified continuous Hausdorff bound or approved machining tolerance.')
 (root/(name+'.geometry.json')).write_text(json.dumps(row,indent=2)+'\n');results.append({k:v for k,v in row.items() if k!='changed_blocks'}|{'changed_blocks':len(blocks)})
(root/'GEOMETRY_SUMMARY.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(results,indent=2))
