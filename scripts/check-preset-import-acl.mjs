// Run after cargo check -p ivac-tauri generates the dependency ACL manifests.
import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
const root = new URL('../', import.meta.url);
const read = (path) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
const manifests = read('crates/ivac-tauri/gen/schemas/acl-manifests.json');
const cap = read('crates/ivac-tauri/capabilities/default.json');
function commands(id) {
  const split = id.lastIndexOf(':');
  const plugin = id.slice(0, split), key = id.slice(split + 1);
  const m = manifests[plugin];
  assert.ok(m, `Unknown plugin ${plugin}`);
  const set = key === 'default' ? m.default_permission : m.permission_sets[key];
  if (set) return set.permissions.flatMap(p => commands(p.includes(':') ? p : `${plugin}:${p}`));
  const p = m.permissions[key];
  assert.ok(p, `Unknown permission ${id}`);
  return (p.commands.allow ?? []).map(c => `${plugin}::${c}`);
}
const ids = cap.permissions.map(p => typeof p === 'string' ? p : p.identifier);
assert.equal(ids.filter(id => id === 'fs:allow-stat').length, 1);
assert.deepEqual(commands('fs:allow-stat'), ['fs::stat']);
assert.equal(typeof cap.permissions.find(p => p === 'fs:allow-stat'), 'string');
const allowed = ids.flatMap(commands);
assert.ok(allowed.includes('fs::stat'));
assert.ok(allowed.includes('fs::read_text_file'));
// Regression counterexample: fs:default and dialog permission do not grant stat.
assert.ok(!ids.filter(id => id !== 'fs:allow-stat').flatMap(commands).includes('fs::stat'));
assert.deepEqual(manifests.fs.permissions['allow-stat'].scope ?? {}, {});
console.log('PASS: stat command granted without added path scope; old ACL counterexample reproduced.');
