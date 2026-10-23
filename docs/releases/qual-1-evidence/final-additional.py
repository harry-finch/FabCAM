from pathlib import Path
import subprocess,json,hashlib,os,shutil
root=Path('/Users/mugen/Documents/01_Projets/FabCAM/qual-1-checkout'); evidence=Path('/Users/mugen/Documents/01_Projets/FabCAM/ivacam/dist-test/qual1'); old=evidence.parent/'fs-reset3';statuses=[]
def run(name,cmd):
 with (evidence/(name+'.log')).open('w') as f:r=subprocess.run(cmd,cwd=root,stdout=f,stderr=subprocess.STDOUT)
 statuses.append({'name':name,'command':cmd,'exit_code':r.returncode});(evidence/'additional-exits.json').write_text(json.dumps(statuses,indent=2)+'\n');return r.returncode
run('final-workspace',['cargo','test','--locked','--offline','--workspace','--all-features','--no-fail-fast'])
with (evidence/'test1-build.stderr').open('w') as f:r=subprocess.run(['python3','scripts/test1/build.py'],cwd=root,stdout=subprocess.PIPE,stderr=f,text=True)
(evidence/'test1-build.stdout').write_text(r.stdout);assert r.returncode==0
binary=r.stdout.strip().splitlines()[-1];(evidence/'test1-binary.json').write_text(json.dumps({'path':binary,'sha256':hashlib.sha256(Path(binary).read_bytes()).hexdigest()})+'\n')
run('final-test1',['python3','scripts/test1/run.py','--binary',binary,'--output',str(evidence/'final-test1'),'--frontend'])
run('final-real-wasm',['pnpm','--dir','frontend','exec','vitest','run','src/lib/sim/qualification-wasm.test.ts','src/lib/state/cutting-v9-wasm.test.ts','src/lib/api/cutting-contract.test.ts'])
# Same fixed 23 pre-removal cases, current production CLI, fresh destinations.
s=(old/'final-cli-compare.py').read_text().replace("out=root/'dist-test/fs-reset3/cli-corpus-final'",'out=Path('+repr(str(evidence/'final-cli-corpus'))+')')
(evidence/'final-cli-compare.py').write_text(s)
run('final-cli-corpus',['python3',str(evidence/'final-cli-compare.py')])
# Every WASM case; compare same backend to established FS3 responses, retain cross-backend facts.
rows=[];w=evidence/'final-wasm';w.mkdir(exist_ok=True);n=evidence/'perf-fixed-output';meta=evidence/'native-records';meta.mkdir(exist_ok=True)
for req in sorted((root/'tests/fixtures/perf-v9').glob('*-linuxcnc.json')):
 out=w/req.name;subprocess.run(['node','frontend/scripts/perf1-wasm.mjs',str(req),str(out),'3'],cwd=root,check=True,timeout=90)
 a=json.loads((old/('wasm-after-'+req.name)).read_text());b=json.loads(out.read_text());native=json.loads((n/req.name).read_text()); native['request_sha256']=hashlib.sha256(req.read_bytes()).hexdigest();(meta/req.name).write_text(json.dumps(native)+'\n')
 ok=b['runs'][0]['success'];assert ok==a['runs'][0]['success'];row={'case':req.name,'success':ok,'wasm_sha256':b['wasm_sha256'],'request_sha256':b['request_sha256']}
 if ok:
  x=json.loads(Path(str(out)+'.response.json').read_text());y=json.loads((old/('wasm-after-'+req.name+'.response.json')).read_text());nn=json.loads((n/(req.name+'.response.json')).read_text())
  row['before_after_equal']={k:x.get(k)==y.get(k) for k in ['gcode','toolpath','tool_provenance','gcode_index','stats']};assert all(row['before_after_equal'].values())
  row.update(native_wasm_program_equal=nn['gcode']==x['gcode'],native_moves=len(nn['toolpath']),wasm_moves=len(x['toolpath']))
 else:row['same_error']=a['runs'][0]['error']==b['runs'][0]['error'];assert row['same_error']
 rows.append(row)
(evidence/'FINAL_WASM_COMPARISON.json').write_text(json.dumps(rows,indent=2)+'\n')
run('final-strict-parity',['python3','scripts/perf1-b2-wasm-check.py','--output',str(evidence/'strict-parity'),'--native-records',str(meta),'--native-responses',str(n)])
