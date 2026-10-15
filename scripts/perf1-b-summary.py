#!/usr/bin/env python3
"""Recompute B1 paired timings and verify all corresponding full-response hashes."""
import hashlib
import json
from pathlib import Path
from statistics import median

root = Path(__file__).resolve().parents[1]
evidence = root / 'docs/shopbot-modernization/perf1-b'
rows = []
for case in ['text-Test-medial-cps','profile-many-segments-cps','pocket-islands-cps']:
    series = {}
    for label in ['control','candidate']:
        records = [json.loads((evidence/'paired'/str(i)/label/(case+'.json')).read_text()) for i in range(3)]
        assert all(len(r['runs']) == 7 and all(x['success'] and x['identical_to_first'] for x in r['runs']) for r in records)
        series[label] = {
            'cold_ms': [r['runs'][0]['pipeline_ms'] for r in records],
            'warm_ms': [x['pipeline_ms'] for r in records for x in r['runs'][2:]],
            'rss_mib': [r['process_resources']['peak_rss_bytes']/1048576 for r in records],
            'cpu_s': [r['process_resources']['user_s']+r['process_resources']['sys_s'] for r in records],
            'response_sha256': records[0]['response_sha256'],
            'binary_sha256': records[0]['binary_sha256'],
        }
        assert len({r['response_sha256'] for r in records}) == 1
    assert series['control']['response_sha256'] == series['candidate']['response_sha256']
    c,n = series['control'],series['candidate']
    rows.append({'case':case, **series, 'warm_gain_percent':100*(1-median(n['warm_ms'])/median(c['warm_ms'])), 'cold_median_change_percent':100*(median(n['cold_ms'])/median(c['cold_ms'])-1)})
(evidence/'PAIRED_SUMMARY.json').write_text(json.dumps(rows,indent=2)+'\n')
for r in rows:
    print(r['case'],[(label,round(median(r[label]['cold_ms']),2),round(median(r[label]['warm_ms']),2),round(median(r[label]['rss_mib']),1)) for label in ['control','candidate']],round(r['warm_gain_percent'],2),'% warm gain',round(r['cold_median_change_percent'],2),'% cold change')
