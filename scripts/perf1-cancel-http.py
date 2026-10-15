#!/usr/bin/env python3
"""HTTP/SSE cancellation and exact retry-result parity; run on idle local servers."""
import hashlib,json,math,threading,time,urllib.request
from pathlib import Path
from statistics import median
root=Path(__file__).resolve().parents[1]
out=root/'docs/shopbot-modernization/perf1-cancel'
request=(root/'docs/shopbot-modernization/perf1/requests/text-Test-medial-cps.json').read_bytes()
def post(base,path,data):
    return urllib.request.urlopen(urllib.request.Request(base+path,data=data,headers={'Content-Type':'application/json'}),timeout=30)
def run(port,cancel):
    base=f'http://127.0.0.1:{port}'; events=[]; signal={}; worker=None; result=None
    def stop(token):
        time.sleep(.05); signal['requested']=time.perf_counter()
        with post(base,f'/generate/cancel/{token}',b'{}') as response: signal['ack']=json.load(response)
    with post(base,'/generate/stream',request) as stream:
        event=''; data=''
        for raw in stream:
            line=raw.decode().strip()
            if line.startswith('event:'): event=line[6:].strip()
            elif line.startswith('data:'): data=line[5:].strip()
            elif not line and event:
                payload=json.loads(data);events.append(event)
                if event=='token' and cancel:
                    worker=threading.Thread(target=stop,args=(payload['token_id'],));worker.start()
                if event=='result':result=payload
                if event in ('cancelled','error','result'):signal['returned']=time.perf_counter()
                event='';data=''
    if worker:worker.join()
    if cancel:
        assert signal['ack']=={'ok':True},signal
        assert events[-1]=='cancelled' and 'result' not in events and 'error' not in events,events
        return dict(events=events,latency_ms=(signal['returned']-signal['requested'])*1000,ack=signal['ack'])
    assert events[-1]=='result' and result is not None,events
    return result
rows=[]
for i in range(10):
    for label,port in ([('control',18797),('candidate',18798)] if i%2==0 else [('candidate',18798),('control',18797)]):
        rows.append(dict(label=label,iteration=i,**run(port,True)))
expected=json.loads(Path('/private/tmp/fabcam-perf1-cancel/profile/text-Test-medial-cps.json.response.json').read_text())
for label,port in [('control',18797),('candidate',18798)]:
    actual=run(port,False);assert actual==expected,label
summary=[]
for label in ['control','candidate']:
    values=sorted(r['latency_ms'] for r in rows if r['label']==label)
    summary.append(dict(label=label,n=len(values),median_ms=median(values),p95_ms=values[math.ceil(.95*len(values))-1],max_ms=max(values)))
record=dict(request_sha256=hashlib.sha256(request).hexdigest(),rows=rows,summary=summary,retry_full_response_equals_native=True)
(out/'HTTP_SUMMARY.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(summary,indent=2))
assert summary[1]['p95_ms']<=200
