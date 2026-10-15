#!/usr/bin/env python3
"""Serial fixed-delay cancellation pairs, fresh process for every observation."""
import argparse,hashlib,json,math,subprocess
from pathlib import Path
from statistics import median
from perf1_evidence import verify_request
p=argparse.ArgumentParser();p.add_argument('--control',required=True);p.add_argument('--candidate',required=True);a=p.parse_args()
root=Path(__file__).resolve().parents[1];e=root/'docs/shopbot-modernization/perf1-cancel';e.mkdir(exist_ok=True)
requests=root/'docs/shopbot-modernization/perf1/requests';manifest={x['name']:x['sha256'] for x in json.loads((requests/'manifest.json').read_text())}
rows=[]
for case,delay,count in [('text-Test-medial-cps',50,20),('profile-many-segments-cps',50,10),('text-Test-medial-cps',1,5)]:
    req=requests/(case+'.json');digest=verify_request(req,manifest[req.name])
    for i in range(count):
        order=[('control',a.control),('candidate',a.candidate)]
        if i%2:order.reverse()
        for label,binary in order:
            out=e/f'{case}-{delay}-{i}-{label}.json';cmd=[str(Path(binary).resolve()),str(req),str(out),'1','cancel',str(delay)]
            subprocess.run(cmd,check=True,capture_output=True,timeout=30)
            record=json.loads(out.read_text());assert record['cancelled'] and not record['returned_valid_result'],record
            record.update(case=case,delay_ms=delay,iteration=i,label=label,request_sha256=digest,binary_sha256=hashlib.sha256(Path(binary).read_bytes()).hexdigest(),command=cmd)
            out.write_text(json.dumps(record,indent=2)+'\n');rows.append(record)
    print(case,delay,'completed',flush=True)
summary=[]
for case,delay in sorted({(r['case'],r['delay_ms']) for r in rows}):
    for label in ['control','candidate']:
        values=sorted(r['latency_ms'] for r in rows if (r['case'],r['delay_ms'],r['label'])==(case,delay,label))
        summary.append(dict(case=case,delay_ms=delay,label=label,n=len(values),median_ms=median(values),p95_ms=values[math.ceil(.95*len(values))-1],max_ms=max(values)))
(e/'NATIVE_SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
assert all(r['p95_ms']<=200 for r in summary if r['label']=='candidate'), 'Candidate misses representative p95 target'
