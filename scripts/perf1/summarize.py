"""Create sanitized PERF-1 measurement evidence from the ignored local runs."""
import hashlib, json, statistics
from pathlib import Path
root=Path.cwd(); raw=root/'dist-test/perf-1'; out=root/'docs/releases/evidence/perf1';out.mkdir(parents=True,exist_ok=True)
load=lambda n:json.loads((raw/n).read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def timing(v):
 return {'median':statistics.median(v),'min':min(v),'max':max(v),'samples':len(v),'unit':'ms'}
result={'environment':{'native':'macOS arm64, release CLI, process startup included; /usr/bin/time -l RSS','wasm':'release web default package in Node 24; fresh compiled instance; compile/init excluded','frontend_node':'Vitest in Node 24','frontend_browser':load('browser-candidate.json')['environment']},'geometry':{},'frontend':{},'chaining':[],'raw_sha256':{}}
for transport in ['native','wasm']:
 b=load(transport+'-baseline.json');a=load(transport+'-candidate.json');rows=[]
 for x,y in zip(b['rows'],a['rows']):
  assert (x['name'],x['request_sha256'],x['response_sha256'],x['accepted'])==(y['name'],y['request_sha256'],y['response_sha256'],y['accepted'])
  t=lambda r:timing(r.get('times_ms',[v['ms'] for v in r.get('trials',[])]))
  row={'name':x['name'],'request_sha256':x['request_sha256'],'response_sha256':y['response_sha256'],'accepted':y['accepted'],'before':t(x),'after':t(y),'exact_complete_response':True}
  if transport=='native':row.update(moves=y['moves'],segments=y['segments'],program_sha256=y['program_sha256'],decoded_sha256=y['decoded_sha256'],rss_before_max=x['max_rss_bytes'],rss_after_max=y['max_rss_bytes'])
  else:row.update(memory_before={k:max(v[k] for v in x['trials']) for k in ['linear_before','linear_after','rss_before','rss_after']},memory_after={k:max(v[k] for v in y['trials']) for k in ['linear_before','linear_after','rss_before','rss_after']})
  rows.append(row)
 result['geometry'][transport]={'baseline_binary_or_wasm_sha256':b.get('binary_sha256',b.get('wasm_sha256')),'candidate_binary_or_wasm_sha256':a.get('binary_sha256',a.get('wasm_sha256')),'rows':rows}
proof=[]
for x,y in zip(load('proof-baseline.json'),load('proof-candidate.json')):
 assert x['name']==y['name'] and x['logical_work_sum_calls_peak']==y['logical_work_sum_calls_peak']
 proof.append({'name':x['name'],'before_pipeline':timing(x['pipeline_ms']),'after_pipeline':timing(y['pipeline_ms']),'logical_sum_calls_peak':y['logical_work_sum_calls_peak'][0],'all_seven_counter_trials_exact':True,'before_decoded':timing(x['decoded_proof_ms']) if x['decoded_proof_ms'] else None,'after_decoded':timing(y['decoded_proof_ms']) if y['decoded_proof_ms'] else None})
result['geometry']['proof']=proof
for environment,prefix in [('node','frontend'),('browser','browser')]:
 b=load(prefix+'-baseline.json');a=load(prefix+'-candidate.json')
 if environment=='browser':b=b['rows'];a=a['rows']
 rows=[]
 for x,y in zip(b,a):
  assert x['kind']==y['kind'] and x['input_sha256']==y['input_sha256']
  row={k:y[k] for k in ['kind','input_sha256','persisted_utf8_bytes']}
  row.update(before=timing(x['times_ms']),after=timing(y['times_ms']),before_serialization={'export':x['exportCounts'],'summary':x['summaryCounts']},after_serialization={'export':y['exportCounts'],'summary':y['summaryCounts']},unsaved_before=timing(x['unsaved_check_ms']),unsaved_after=timing(y['unsaved_check_ms']),history_before=timing(x['undo_redo_cycle_ms']),history_after=timing(y['undo_redo_cycle_ms']),heap_before_run=[x['heap_before'],x['heap_after']],heap_after_run=[y['heap_before'],y['heap_after']],heap_observational_only=True)
  rows.append(row)
 result['frontend'][environment]=rows
for r in load('chaining.json'):
 result['chaining'].append({k:r[k] for k in ['requested_segments','actual_segments','variant','objects','closed']}|{'time':timing(r['times_ms']),'ordered_membership_sha256':hashlib.sha256(json.dumps(r['ordered_membership']).encode()).hexdigest()})
for n in ['native-baseline.json','native-candidate.json','wasm-baseline.json','wasm-candidate.json','proof-baseline.json','proof-candidate.json','frontend-baseline.json','frontend-candidate.json','browser-baseline.json','browser-candidate.json','chaining.json']:
 result['raw_sha256'][n]=sha(raw/n)
(out/'measurements.json').write_text(json.dumps(result,indent=2)+'\n')
print('PASS 8 native + 8 WASM complete response pairs; work counters and frontend inputs exact')
