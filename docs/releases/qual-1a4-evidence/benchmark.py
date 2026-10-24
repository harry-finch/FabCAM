"""Three preserved release configurations, serial rotated process order.
Run only when compilation and other CPU-heavy diagnostics have completed.
"""
from pathlib import Path
import gzip,hashlib,json,statistics as S,subprocess,sys
ROOT=Path('/Users/mugen/Documents/01_Projets/FabCAM/FabCAM-qual-1a4');PRIMARY=ROOT.parent/'ivacam';RAW=PRIMARY/'dist-test/qual1a4';OLD=PRIMARY/'dist-test/qual1a';OUT=RAW/'performance';OUT.mkdir(exist_ok=False)
NODE='/opt/homebrew/opt/node@24/bin/node'
BACKENDS={'production-native':PRIMARY/'dist-test/qual1/perf-fixed','experimental-native':PRIMARY/'dist-test/qual1a2/perf-candidate','integrated-native':RAW/'perf','production-wasm':ROOT.parent/'qual-1-checkout','experimental-wasm':ROOT.parent/'FabCAM-qual-1a2','integrated-wasm':ROOT}
REQUESTS={n:ROOT/'tests/fixtures/perf-v9'/n for n in ['square-medial-linuxcnc.json','text-Test-medial-linuxcnc.json','text-Test-medial-cps.json']};REQUESTS['dense-circle.json']=OLD/'adversarial/dense-circle.json'
rows=[]
for round in range(5):
 order=['production','experimental','integrated'];order=order[round%3:]+order[:round%3]
 for name,request in REQUESTS.items():
  for runtime in ['native','wasm']:
   if runtime=='wasm' and '-cps' in name:continue
   for variant in order:
    label=variant+'-'+runtime;backend=BACKENDS[label];result=OUT/f'{round}-{label}-{name}'
    cmd=[str(backend),str(request),str(result),'7'] if runtime=='native' else [NODE,str(backend/'frontend/scripts/perf1-wasm.mjs'),str(request),str(result),'7']
    r=subprocess.run([sys.executable,str(ROOT.parent/'FabCAM-qual-1a3/scripts/qual1a/measure_process.py'),*cmd],capture_output=True,text=True,timeout=60)
    result.with_suffix('.resource.txt').write_text(r.stdout+r.stderr);assert r.returncode==0,r.stderr
    d=json.loads(result.read_text());assert all(x['success'] and x['identical_to_first'] for x in d['runs'])
    if runtime=='native':assert d['profile']=='release' and d['instrumented'] is False
    timer='pipeline_ms' if runtime=='native' else 'generate_ms';resources=json.loads(r.stdout)['resources']
    response_path=Path(str(result)+'.response.json');data=response_path.read_bytes();response=json.loads(data)
    # Preserve one full result per backend/input; each later response must match.
    preserved=OUT/f'{label}-{name}.response.json.gz'
    if preserved.exists():assert gzip.decompress(preserved.read_bytes())==data
    else:preserved.write_bytes(gzip.compress(data,mtime=0))
    response_path.unlink()
    rows.append(dict(round=round,backend=label,case=name,parse_ms=d['parse_ms'],init_ms=d.get('init_ms'),cold_ms=d['runs'][0][timer],warm_ms=[x[timer] for x in d['runs'][2:]],process_user_s=resources.get('user_s'),process_sys_s=resources.get('sys_s'),peak_rss_bytes=resources.get('peak_rss_bytes'),moves=len(response['toolpath']),program_bytes=len(response['gcode'].encode()),request_sha256=hashlib.sha256(request.read_bytes()).hexdigest(),response_sha256=hashlib.sha256(data).hexdigest()))
    print(round,label,name,flush=True)
summary=[]
for name in REQUESTS:
 for runtime in ['native','wasm']:
  selected=[x for x in rows if x['case']==name and x['backend'].endswith(runtime)]
  if not selected:continue
  result=dict(case=name,runtime=runtime)
  for variant in ['production','experimental','integrated']:
   group=[x for x in selected if x['backend'].startswith(variant)]
   result[variant]={k:S.median([x[k] for x in group]) if all(x[k] is not None for x in group) else None for k in ['parse_ms','init_ms','cold_ms','process_user_s','process_sys_s','peak_rss_bytes','moves','program_bytes']}
   result[variant]['warm_median_ms']=S.median(v for x in group for v in x['warm_ms'])
  summary.append(result)
wasm={}
for label,base in BACKENDS.items():
 if label.endswith('wasm'):
  b=(base/'crates/ivac-wasm/pkg/ivac_wasm_bg.wasm').read_bytes();wasm[label]=dict(bytes=len(b),gzip_bytes=len(gzip.compress(b,mtime=0)),sha256=hashlib.sha256(b).hexdigest())
# Isolated parsing cost: same dependency versions, only float_roundtrip differs.
large=json.loads(REQUESTS['dense-circle.json'].read_text());large['project']['segments']*=128
large_path=OUT/'large-parser-only.request.json';large_path.write_text(json.dumps(large,separators=(',',':')))
parser=[]
for case,path in [('ordinary',REQUESTS['dense-circle.json']),('large-parser-only',large_path)]:
 for round in range(5):
  for label in (['production','experimental','integrated'][round%3:]+['production','experimental','integrated'][:round%3]):
   r=subprocess.run([str((RAW/'number-final' if label=='integrated' else PRIMARY/'dist-test/qual1a2'/('number-'+('baseline' if label=='production' else 'candidate')))),'--bench',str(path),'22'],capture_output=True,text=True,check=True)
   d=json.loads(r.stdout);parser.append(dict(case=case,round=round,variant=label,**d))
(RAW/'PERFORMANCE.json').write_text(json.dumps(dict(rows=rows,summary=summary,wasm=wasm,parser=parser,protocol='5 rotated processes per backend/case; 7 calls each: cold, warmup, five warm. Native parse/pipeline separate, WASM init/JSON.parse/generate separate. Parser-only benchmark 5 rotated triplets x22 parses; first two excluded for warm summaries. Repeated-segment large input is parse-only, not a CAM validity or memory-budget waiver.'),indent=2)+'\n')
