from pathlib import Path
import subprocess,json,statistics,sys,hashlib
root=Path.cwd();label=sys.argv[1];pkg=root/'dist-test/perf-1'/('baseline-wasm' if label=='baseline' else 'candidate-wasm');rows=[];out=root/'dist-test/perf-1'/('wasm-'+label);out.mkdir(exist_ok=True)
for fixture in sorted((root/'tests/fixtures/perf1').glob('*.json')):
 trials=[];expected=None
 for n in range(8):
  r=subprocess.run(['node',str(root/'scripts/perf1/wasm-one.mjs'),str(pkg/'ivac_wasm.js'),str(fixture)],capture_output=True,timeout=60);assert r.returncode==0,r.stderr;v=json.loads(r.stdout)
  if n==0:expected=v['response'];(out/(fixture.stem+'.response.json')).write_text(json.dumps(expected)+'\n')
  else:assert expected==v['response'];trials.append({k:v[k] for k in ['ms','linear_before','linear_after','rss_before','rss_after']})
 rows.append({'name':fixture.stem,'accepted':v['accepted'],'request_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'trials':trials,'median_ms':statistics.median(t['ms'] for t in trials),'response_sha256':hashlib.sha256(json.dumps(expected,sort_keys=True,separators=(',',':')).encode()).hexdigest()});print(fixture.stem,v['accepted'],round(rows[-1]['median_ms'],2),flush=True)
(root/'dist-test/perf-1'/('wasm-'+label+'.json')).write_text(json.dumps({'wasm_sha256':hashlib.sha256((pkg/'ivac_wasm_bg.wasm').read_bytes()).hexdigest(),'one_warmup_seven_fresh_instances':True,'compile_init_excluded':True,'rows':rows},indent=2)+'\n')
