"""Exactly two fixed campaigns. Production timings separate from instrumented phases."""
from pathlib import Path
import datetime, hashlib, json, math, os, platform, re, statistics, subprocess, time
root=Path(__file__).resolve().parents[2];p=root/'dist-test/qual1b';primary=root.parent/'ivacam'
sha=lambda path:hashlib.sha256(path.read_bytes()).hexdigest()
binaries={'control':primary/'dist-test/qual1a4/perf','candidate':p/'perf-candidate','diagnostic-control':p/'perf-diagnostic-control','diagnostic-candidate':p/'perf-diagnostic-candidate'}
cases=['profile-many-segments-cps','text-Test-medial-cps','square-medial-cps']
counts=dict(zip(cases,[10,20,10]));delays=dict(zip(cases,[50,50,1]))
protocol={'campaigns':2,'warmups':'one discarded run per case/mode/binary each campaign','samples':{'fixed-delay':counts,'preflight-injected':counts,'pre-cancelled':10,'uncancelled':10},'order':'alternate control/candidate each sample; reverse initial order in campaign 2','p95':'nearest rank ceil(.95*n)','target_ms':200,'source':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'runtime_diff_sha256':hashlib.sha256(subprocess.check_output(['git','diff','--','crates/ivac-cps/posts/shopbot.cps'],cwd=root)).hexdigest(),'host':platform.platform(),'start_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'conditions':'release, no task-owned compilation/tests during campaigns; OS/background user load uncontrolled; no retry based on outcome','binaries':{k:sha(v) for k,v in binaries.items()},'fixtures':{c:sha(root/'tests/fixtures/perf-v9'/(c+'.json')) for c in cases}}
(p/'MEASUREMENT_PROTOCOL.json').write_text(json.dumps(protocol,indent=2)+'\n')
def observe(out,case,mode,label,index):
 binary=binaries[('diagnostic-' if mode in ('preflight-injected','pre-cancelled') else '')+label]
 target=out/f'{case}-{mode}-{label}-{index}.json';req=root/'tests/fixtures/perf-v9'/(case+'.json')
 env=dict(os.environ);cmd=[str(binary),str(req),str(target),'1']
 if mode!='uncancelled':cmd+=['cancel',str(delays[case])]
 if mode=='preflight-injected':env['QUAL1B_INJECT']='QUAL1B record65'
 if mode=='pre-cancelled':env.update(QUAL1B_PRE_CANCEL='1',QUAL1B_INJECT='disabled')
 before=time.monotonic();r=subprocess.run(cmd,env=env,capture_output=True,timeout=30);wall=(time.monotonic()-before)*1000
 target.with_suffix('.stderr').write_bytes(r.stderr);assert r.returncode==0,r.stderr
 value=json.loads(target.read_bytes());row={'case':case,'mode':mode,'label':label,'sample':index,'process_wall_ms':wall,'raw':value}
 if mode=='uncancelled':
  assert value['runs'][0]['success'];row['latency_ms']=value['runs'][0]['pipeline_ms']
 else:
  assert value['cancelled'] and not value['returned_valid_result'],value
  row['latency_ms']=value['latency_ms']
  if mode=='pre-cancelled':row['latency_ms']=value['returned_ms']
  phases={};relative={}
  for line in r.stderr.decode().splitlines():
   match=re.fullmatch(r'QUAL1B (.+) Instant \{ tv_sec: (\d+), tv_nsec: (\d+) \}',line)
   if match:phases.setdefault(match[1],int(match[2])*1e9+int(match[3]))
   elif re.fullmatch(r'QUAL1B \S+ [0-9.eE+\-]+',line):
    _,name,t=line.split();relative[name]=float(t)
  row['phase_elapsed_ms']=relative
  if 'init' in relative:
   row['phase_duration_ms']={name:relative[name]-relative.get(previous,0) for name,previous in [('init','run_start'),('eval','init'),('convert','eval'),('execute_return','convert'),('cleanup','execute_return')] if name in relative}
  if mode=='preflight-injected':
   assert all(k in phases for k in ('record65','signalled','detected','pipeline_return')),r.stderr
   row['latency_ms']=(phases['pipeline_return']-phases['signalled'])/1e6
   row['detection_ms']=(phases['detected']-phases['signalled'])/1e6
   row['propagation_cleanup_ms']=(phases['pipeline_return']-phases['detected'])/1e6
   row['preflight_completed']='preflight_done' in phases
   assert row['preflight_completed']==(label=='control')
   if 'preflight_done' in phases:row['preflight_ms']=(phases['preflight_done']-phases['preflight'])/1e6
 target.with_suffix('.receipt.json').write_text(json.dumps(row,indent=2)+'\n');return row
allrows=[]
for campaign in [1,2]:
 out=p/f'campaign-{campaign}';out.mkdir(exist_ok=False);warm=out/'warmup';warm.mkdir()
 matrix=[(c,m,counts[c] if m in ('fixed-delay','preflight-injected') else 10) for m in ['fixed-delay','preflight-injected','uncancelled'] for c in cases]+[(cases[0],'pre-cancelled',10)]
 for case,mode,n in matrix:
  for label in ['control','candidate']:observe(warm,case,mode,label,0)
  for i in range(n):
   order=['control','candidate']
   if (i+campaign)%2==0:order.reverse()
   for label in order:
    row=observe(out,case,mode,label,i);row['campaign']=campaign;allrows.append(row)
  print(campaign,case,mode,'complete',flush=True)
summary=[]
for campaign in [1,2]:
 for case,mode,label in sorted({(r['case'],r['mode'],r['label']) for r in allrows}):
  rows=[r for r in allrows if (r['campaign'],r['case'],r['mode'],r['label'])==(campaign,case,mode,label)]
  values=sorted(r['latency_ms'] for r in rows)
  row=dict(campaign=campaign,case=case,mode=mode,label=label,n=len(values),median_ms=statistics.median(values),p95_ms=values[math.ceil(.95*len(values))-1],max_ms=max(values))
  if mode!='uncancelled':row['target_pass']=row['p95_ms']<=200
  summary.append(row)
(p/'PERFORMANCE.json').write_text(json.dumps({'protocol':protocol,'summary':summary,'observations':allrows},indent=2)+'\n')
print(json.dumps(summary,indent=2))
# Retain evidence even if the unchanged engineering criterion fails.
raise SystemExit(0 if all(r.get('target_pass',True) for r in summary if r['label']=='candidate') else 1)
