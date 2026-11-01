from pathlib import Path
import subprocess,os,statistics,json
root=Path.cwd();source=root/'frontend/src/lib/state/program-qualification.ts';candidate=source.read_bytes();baseline=subprocess.check_output(['git','show','62b8a6bcf8a19bc98e57af0a254adf0ae78508eb:frontend/src/lib/state/program-qualification.ts']);env=dict(os.environ)
try:
 for phase,data in [('baseline',baseline),('candidate',candidate)]:
  source.write_bytes(data);e=dict(env,PERF1_BENCHMARK_OUT=str(root/f'dist-test/perf-1/frontend-{phase}.json'),PERF1_BROWSER_OUT=str(root/f'dist-test/perf-1/browser-{phase}'))
  if phase=='baseline':e['PERF1_BENCHMARK_BASELINE']='1'
  with (root/f'dist-test/perf-1/frontend-{phase}-paired.log').open('wb') as log:subprocess.run(['pnpm','exec','vitest','run','src/lib/state/perf1-benchmark.test.ts'],cwd=root/'frontend',env=e,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=90)
  with (root/f'dist-test/perf-1/browser-{phase}-build.log').open('wb') as log:subprocess.run(['pnpm','exec','vite','build','--config','bench/perf1.vite.config.ts'],cwd=root/'frontend',env=e,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=90)
finally:source.write_bytes(candidate)
b=json.loads((root/'dist-test/perf-1/frontend-baseline.json').read_text());a=json.loads((root/'dist-test/perf-1/frontend-candidate.json').read_text())
for x,y in zip(b,a):print(x['kind'],round(statistics.median(x['times_ms']),2),round(statistics.median(y['times_ms']),2),'ms',x['exportCounts']['serializations'],'->',y['exportCounts']['serializations'],'export serializations',flush=True)
