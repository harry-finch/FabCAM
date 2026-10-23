from pathlib import Path
import json,subprocess,hashlib
root=Path('.').resolve(); out=Path('/Users/mugen/Documents/01_Projets/FabCAM/ivacam/dist-test/qual1/final-cli-corpus');out.mkdir(exist_ok=True)
fields=['gcode','toolpath','tool_provenance','gcode_index','stats'];results=[]
for path in sorted((root/'tests/fixtures/fs-reset3-output').glob('*.json')):
 if path.name=='SHA256SUMS.json':continue
 data=json.loads(path.read_text());request=out/path.name;request.write_text(json.dumps(data['request']))
 if 'two_sided' in data:
  base=out/'two-sided';r=subprocess.run([str(root/'target/debug/ivac'),'stream-gcode',str(request),'--two-sided','--output',str(base)],capture_output=True,timeout=60)
  assert r.returncode==0,(path.name,r.stderr.decode())
  files=list(out.glob('two-sided*'));print('two-sided outputs', [p.name for p in files]);
  for side in ('front','back'):
   matches=[p for p in files if side in p.name and p.suffix!='json'];assert len(matches)==1,matches
   assert matches[0].read_text()==data['two_sided'][side]['gcode'], (path.name,side)
  results.append(dict(case=path.name,mode='stream-gcode --two-sided',fields=['front.gcode','back.gcode'],result='exact'));continue
 r=subprocess.run([str(root/'target/debug/ivac'),'generate','--request',str(request)],capture_output=True,timeout=60)
 assert r.returncode==0,(path.name,r.stderr.decode())
 value=json.loads(r.stdout);(out/(path.stem+'.response.json')).write_bytes(r.stdout)
 for field in fields:assert value.get(field)==data['response'].get(field),(path.name,field)
 results.append(dict(case=path.name,mode='generate --request',fields=fields,result='exact'))
receipt=dict(source=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),binary_sha256=hashlib.sha256((root/'target/debug/ivac').read_bytes()).hexdigest(),cases=results)
(out/'summary.json').write_text(json.dumps(receipt,indent=2)+'\n');print(len(results),'PASS; exact output fields; no hardware')
