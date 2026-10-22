import json,subprocess,difflib,hashlib
from pathlib import Path
root=Path.cwd();out=root/'dist-test/fs-reset-cli/comparison';out.mkdir(exist_ok=True)
geometry=root/'tests/fixtures/shopbot-m1/rectangle_100x50.dxf'
rows=[]
for post in ['linuxcnc','grbl','hpgl','cps']:
 req=json.loads((root/'tests/fixtures/cli-v9/profile.template.json').read_text())
 req['post_processor']=post
 if post=='cps':req['cps_post']['source']['id']='grbl'
 if post!='cps':req.pop('cps_post')
 tool=req['project']['tools'][0];tool.update(diameter=3,feed_rate=800,plunge_rate=100,speed=18000)
 op=req['project']['operations'][0];op['params'].update(depth=-2,step=-1)
 for r in op['params']['cutting']['rates']:r.update(rpm=18000,feed_mm_min=800,plunge_mm_min=100)
 # Native default machine used by the former geometry route; review values remain explicit.
 req['project']['machine']={'unit':'mm','mode':'mill','comments':True,'arcs':True}
 path=out/f'{post}.template.json';path.write_text(json.dumps(req,indent=2)+'\n')
 args=['generate',str(geometry),'--post',post]
 if post=='cps':args+=['--post-id','grbl']
 old=subprocess.run([str(root/'target/debug/ivac-cli-baseline'),*args],capture_output=True,check=True)
 new=subprocess.run([str(root/'target/debug/ivac'),'generate',str(geometry),'--request',str(path)],capture_output=True,check=True)
 oldj=json.loads(old.stdout);newj=json.loads(new.stdout)
 (out/f'{post}.before.json').write_bytes(old.stdout);(out/f'{post}.after.json').write_bytes(new.stdout)
 diff=''.join(difflib.unified_diff(oldj['gcode'].splitlines(True),newj['gcode'].splitlines(True),fromfile='approved-old-cli',tofile='explicit-v9-cli'))
 (out/f'{post}.diff').write_text(diff)
 rows.append({'post':post,'exact_equal':oldj['gcode']==newj['gcode'],'before_sha256':hashlib.sha256(oldj['gcode'].encode()).hexdigest(),'after_sha256':hashlib.sha256(newj['gcode'].encode()).hexdigest(),'before_stats':oldj['stats'],'after_stats':newj['stats'],'diff':diff})
(out/'receipt.json').write_text(json.dumps(rows,indent=2)+'\n')
for r in rows:print(r['post'],r['exact_equal'],len(r['diff'].splitlines()),'diff lines')
