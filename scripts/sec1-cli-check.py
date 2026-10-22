#!/usr/bin/env python3
"""Real production CLI boundary probes in private temporary files; no JS executes."""
from pathlib import Path
import json
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
binary = root / 'target/debug/ivac'
fixture = json.loads((root / 'tests/fixtures/cli-v9/profile.request.json').read_text())
with tempfile.TemporaryDirectory(prefix='fabcam-sec1-cli-') as tmp:
    path = Path(tmp) / 'shopbot.cps'
    request = Path(tmp) / 'request.json'
    fixture['cps_post']['source'] = {'kind': 'path', 'path': str(path)}
    request.write_text(json.dumps(fixture))
    for text in ['while(true){}', 'throw new Error("EXECUTED")', (root / 'crates/ivac-cps/posts/shopbot.cps').read_text()]:
        path.write_text(text)
        for args in [['posts', 'inspect', str(path)], ['generate', '--request', str(request)]]:
            result = subprocess.run([str(binary), *args], capture_output=True, timeout=10)
            assert result.returncode != 0 and b'External CPS' in result.stderr, result.stderr
            assert not result.stdout, result.stdout
    result = subprocess.run([str(binary), 'posts'], capture_output=True, timeout=10)
    assert result.returncode == 0 and b'shopbot' in result.stdout and b'grbl' in result.stdout, result.stderr
print('PASS: CLI inspection/generation refuse renamed, malicious and exact-copy external posts; bundled listing succeeds.')
