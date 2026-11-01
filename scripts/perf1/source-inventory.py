"""Fingerprint the accepted executable inventory plus explicit PERF-1 additions.

Run from root: python3 scripts/perf1/source-inventory.py OUTPUT [--verify].
Historical Markdown inventory entries are documentation, excluded from this
executable fingerprint. Existing tracked non-Markdown blobs outside the inventory
are independently compared with the starting revision.
"""
import hashlib, json, subprocess, sys
from pathlib import Path
root=Path.cwd(); start='62b8a6bcf8a19bc98e57af0a254adf0ae78508eb'; sha=lambda b:hashlib.sha256(b).hexdigest()
old=json.loads((root/'docs/releases/evidence/sec-harden1/software-files.json').read_text())
paths={r['path'] for r in old['files'] if not r['path'].endswith('.md')}
added=subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','-z']).decode().split('\0')
for p in added:
 if p and not p.endswith('.md') and (p.startswith(('scripts/perf1/','frontend/bench/','tests/fixtures/perf1/')) or p in ['crates/ivac-core/src/pipeline/op_drivers/stage_polygon/perf1.rs','frontend/src/lib/state/perf1-benchmark.test.ts']): paths.add(p)
rows=[{'path':p,'sha256':sha((root/p).read_bytes())} for p in sorted(paths)]
fingerprint=sha(''.join(r['path']+'\0'+r['sha256']+'\n' for r in rows).encode())
tracked=subprocess.check_output(['git','diff','--diff-filter=MDR','--name-only',start]).decode().splitlines()
executable_changes=[p for p in tracked if not p.endswith('.md')]
assert set(executable_changes)=={'crates/ivac-core/src/pipeline/op_drivers/stage_polygon.rs','crates/ivac-core/src/pipeline/op_drivers/stage_polygon/pocket.rs','frontend/src/lib/state/program-qualification.ts'},executable_changes
result={'starting_commit':start,'algorithm':'SHA256 of sorted UTF-8 path + NUL + file SHA256 + LF records; Markdown excluded','file_count':len(rows),'software_fingerprint_sha256':fingerprint,'existing_tracked_non_markdown_changes':executable_changes,'files':rows}
out=Path(sys.argv[1])
if '--verify' in sys.argv:
 assert json.loads(out.read_text())==result,'Source differs from frozen inventory'
 print('PASS source frozen:',len(rows),fingerprint)
else:
 out.write_text(json.dumps(result,indent=2)+'\n');print(len(rows),fingerprint)
