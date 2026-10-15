#!/usr/bin/env python3
"""Fail closed on paired/matrix outcomes and recompute PERF1-B2 evidence."""
import json
from pathlib import Path
from statistics import median
from perf1_evidence import series_evidence
root = Path(__file__).resolve().parents[1]/'docs/shopbot-modernization'
evidence = root/'perf1-b2'
summary = {}
for case in ['text-Test-medial-cps','profile-many-segments-cps','pocket-islands-cps']:
    sides = {}
    for label in ['control','candidate']:
        records = [json.loads((evidence/'paired'/str(i)/label/(case+'.json')).read_text()) for i in range(3)]
        assert all(len(r['runs']) == 7 and series_evidence(r['runs'])['warm_output_stable'] for r in records)
        assert len({r['response_sha256'] for r in records}) == 1
        assert len({r['request_sha256'] for r in records}) == 1
        assert len({r['binary_sha256'] for r in records}) == 1
        warm = [run for r in records for run in r['runs'][2:]]
        sides[label] = {'cold_ms':[r['runs'][0]['pipeline_ms'] for r in records],
            'warm_ms':[r['pipeline_ms'] for r in warm],
            'remaining_ms':[sum(s['elapsed_ns'] for s in r['spans'] if s['name']=='preview.remaining')/1e6 for r in warm],
            'peak_rss_mib':[r['process_resources']['peak_rss_bytes']/1048576 for r in records],
            'cpu_s':[r['process_resources']['user_s']+r['process_resources']['sys_s'] for r in records],
            'response_sha256':records[0]['response_sha256'], 'request_sha256':records[0]['request_sha256'], 'binary_sha256':records[0]['binary_sha256']}
    assert sides['control']['response_sha256'] == sides['candidate']['response_sha256']
    assert sides['control']['request_sha256'] == sides['candidate']['request_sha256']
    summary[case]=sides
    print(case, {label:{k:round(median(sides[label][k]),3) for k in ['cold_ms','warm_ms','remaining_ms','peak_rss_mib']} for label in sides})
(evidence/'PAIRED_SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n')
comparisons=[]
for path in sorted((evidence/'native-profile').glob('*.json')):
    a=json.loads((root/'perf1-b/native-profile'/path.name).read_text());b=json.loads(path.read_text())
    assert a['request_sha256']==b['request_sha256']
    assert series_evidence(b['runs'])['series_consistent']
    assert len(b['runs'])==7
    assert b['runs'][0]['success']==a['runs'][0]['success']
    if b['runs'][0]['success']: assert a['response_sha256']==b['response_sha256']
    else: assert a['runs'][0]['error']==b['runs'][0]['error']
    comparisons.append({'case':path.stem,'equivalent':True,'success':b['runs'][0]['success'],'response_sha256':b.get('response_sha256')})
assert len(comparisons)==18
(evidence/'output-comparisons.json').write_text(json.dumps(comparisons,indent=2)+'\n')
print('18 full response/refusal comparisons pass')
