from pathlib import Path
import subprocess,json,hashlib,time,re,statistics,sys
root=Path.cwd();label=sys.argv[1];exe=root/'dist-test/perf-1'/('baseline-ivac' if label=='baseline' else 'candidate-ivac');out=root/'dist-test/perf-1'/('native-'+label);out.mkdir(exist_ok=True);rows=[];sha=lambda b:hashlib.sha256(b).hexdigest()
for fixture in sorted((root/'tests/fixtures/perf1').glob('*.json')):
 data=fixture.read_bytes();v=json.loads(data);times=[];rss=[];response=None;code=None
 for n in range(8):
  start=time.perf_counter();r=subprocess.run(['/usr/bin/time','-l',str(exe),'generate','--request',str(fixture)],capture_output=True,timeout=60);elapsed=(time.perf_counter()-start)*1000;actual=json.loads(r.stdout) if r.returncode==0 else {'error':r.stderr.decode().split('\n')[0]}
  if n==0:response=actual;code=r.returncode;(out/(fixture.stem+'.response.json')).write_text(json.dumps(actual)+'\n')
  else:assert response==actual,(fixture,n);times.append(elapsed);match=re.search(rb'(\d+)\s+maximum resident set size',r.stderr);rss.append(int(match[1]) if match else None)
 row={'name':fixture.stem,'request_sha256':sha(data),'accepted':code==0,'times_ms':times,'median_ms':statistics.median(times),'rss_bytes':rss,'max_rss_bytes':max(x or 0 for x in rss),'segments':len(v['project']['segments']),'moves':len(response.get('toolpath',[])),'response_sha256':sha(json.dumps(response,sort_keys=True,separators=(',',':')).encode()),'program_sha256':sha(response['gcode'].encode()) if code==0 else None,'decoded_sha256':sha(json.dumps(response.get('toolpath'),sort_keys=True).encode()) if code==0 else None};rows.append(row);print(fixture.stem,row['accepted'],round(row['median_ms'],2),row['moves'],flush=True)
(root/'dist-test/perf-1'/('native-'+label+'.json')).write_text(json.dumps({'binary_sha256':sha(exe.read_bytes()),'one_warmup_seven_trials':True,'includes_process_startup':True,'rows':rows},indent=2)+'\n')
