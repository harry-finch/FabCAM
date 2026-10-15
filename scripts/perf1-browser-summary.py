#!/usr/bin/env python3
"""Summarize captured public PERF1 console markers; reject incomplete runs."""
import json
from pathlib import Path
from statistics import median

root = Path(__file__).resolve().parents[1] / 'docs/shopbot-modernization/perf1-b'
summary = {}
for label in ['control', 'candidate']:
    logs = json.loads((root / f'browser-{label}.json').read_text())
    runs = {}
    for entry in logs:
        event = json.loads(entry['message'].split('PERF1 ', 1)[1])
        runs.setdefault(event['id'], {})[event['stage']] = event
    assert sorted(runs) == list(range(1, 8)), (label, list(runs))
    rows = []
    for run_id, stages in sorted(runs.items()):
        def t(stage):
            return stages[stage]['elapsed_ms']
        assert stages['response_received']['moves'] == 18874
        rows.append({'id': run_id, 'response_ms': t('response_received'),
                     'first_frame_ms': t('first_3d_frame'),
                     'relief_frame_ms': t('first_3d_frame_after_simulation'),
                     'debounce_ms': t('simulation_timer_fired') - t('simulation_scheduled'),
                     'configure_ms': t('simulation_configure_end') - t('simulation_configure_begin'),
                     'replay_ms': t('simulation_replayed') - t('simulation_replay_begin')})
    summary[label] = {'runs': rows, 'warm_medians': {
        key: median(row[key] for row in rows[2:]) for key in rows[0] if key != 'id'}}
(root / 'BROWSER_SUMMARY.json').write_text(json.dumps(summary, indent=2) + '\n')
for label, data in summary.items():
    print(label, data['warm_medians'])
