#!/usr/bin/env python3
"""Resolve current default+desktop permissions; no native app or data is modified."""
import json
from pathlib import Path
root = Path('crates/ivac-tauri')
manifest = json.loads((root/'gen/schemas/acl-manifests.json').read_text())
capabilities = [json.loads(p.read_text()) for p in sorted((root/'capabilities').glob('*.json'))]
allowed, seen = set(), set()
def walk(item):
    key = item if isinstance(item,str) else item['identifier']
    if key in seen: return
    seen.add(key)
    plugin, permission = key.split(':',1)
    data = manifest.get(plugin,{})
    entry = data.get('default_permission') if permission=='default' else data.get('permission_sets',{}).get(permission,data.get('permissions',{}).get(permission))
    if not entry: return
    allowed.update(plugin+'::'+name for name in entry.get('commands',{}).get('allow',[]))
    for child in entry.get('permissions',[]): walk(child if ':' in child else plugin+':'+child)
for cap in capabilities:
    for item in cap['permissions']: walk(item)
print(json.dumps({'capabilities':[c['identifier'] for c in capabilities], 'enabled_permission_identifiers':sorted(seen),'allowed_fs_commands':sorted(x for x in allowed if x.startswith('fs::')),'stat_command_allowed':'fs::stat' in allowed,'native_import_calls_stat':'await stat(path)' in Path('frontend/src/lib/services/preset-collection-files.ts').read_text(),'limit':'Static command permission expansion, not an installed native dialog test. Dialog-selected path scope cannot grant a missing command.'},indent=2))
