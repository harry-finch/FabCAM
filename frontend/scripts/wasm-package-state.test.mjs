import { test } from 'node:test';
import assert from 'node:assert/strict';
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  rmSync,
  utimesSync,
  copyFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import {
  assets,
  packageDir,
  inputFingerprint,
  packageState,
  recordBuild,
} from './wasm-package-state.mjs';

function fixture(t) {
  const root = mkdtempSync(join(tmpdir(), 'fabcam-wasm-'));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const dir = packageDir(root);
  mkdirSync(dir, { recursive: true });
  mkdirSync(join(root, 'frontend/scripts'), { recursive: true });
  for (const script of ['ensure-wasm-fresh.mjs', 'wasm-package-state.mjs']) {
    copyFileSync(new URL(script, import.meta.url), join(root, 'frontend/scripts', script));
  }
  writeFileSync(join(root, 'Cargo.lock'), 'locked dependency 1');
  for (const asset of assets) writeFileSync(join(dir, asset), 'binding');
  writeFileSync(join(dir, 'ivac_wasm_bg.wasm'), Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]));
  writeFileSync(
    join(dir, 'package.json'),
    JSON.stringify({ main: 'ivac_wasm.js', types: 'ivac_wasm.d.ts', files: assets.slice(0, 3) }),
  );
  return root;
}
for (const missing of assets)
  test(`incomplete package rejected: ${missing}`, (t) => {
    const root = fixture(t);
    recordBuild(root, inputFingerprint(root));
    rmSync(join(packageDir(root), missing));
    assert.equal(packageState(root).complete, false);
  });
test('content fingerprints detect old timestamps, changed bindings and lockfile; unchanged package passes', (t) => {
  const root = fixture(t);
  recordBuild(root, inputFingerprint(root));
  assert.equal(packageState(root).fresh, true);
  writeFileSync(join(root, 'Cargo.lock'), 'locked dependency 2');
  utimesSync(join(root, 'Cargo.lock'), new Date(0), new Date(0));
  assert.equal(packageState(root).fresh, false);
  recordBuild(root, inputFingerprint(root));
  writeFileSync(join(packageDir(root), 'ivac_wasm.js'), 'different binding');
  assert.equal(packageState(root).fresh, false);
});
test('concurrent source edit prevents stamping successful old build', (t) => {
  const root = fixture(t),
    before = inputFingerprint(root);
  writeFileSync(join(root, 'Cargo.lock'), 'changed during build');
  assert.throws(() => recordBuild(root, before), /changed during build/);
  assert.equal(packageState(root).fresh, false);
});
test('release CLI fails closed without toolchain; only explicit dev accepts complete stale package', (t) => {
  const root = fixture(t);
  const run = (args) =>
    spawnSync(process.execPath, [join(root, 'frontend/scripts/ensure-wasm-fresh.mjs'), ...args], {
      env: { ...process.env, PATH: join(root, 'no-toolchain') },
      encoding: 'utf8',
    });
  assert.equal(run([]).status, 1);
  assert.equal(run(['--dev']).status, 0);
  recordBuild(root, inputFingerprint(root));
  assert.equal(run([]).status, 0);
  rmSync(join(packageDir(root), 'ivac_wasm.js'));
  assert.equal(run(['--dev']).status, 1);
});
test('empty assets and malformed manifest are never complete', (t) => {
  const root = fixture(t),
    dir = packageDir(root);
  const original = readFileSync(join(dir, 'ivac_wasm.js'));
  writeFileSync(join(dir, 'ivac_wasm.js'), '');
  assert.equal(packageState(root).complete, false);
  writeFileSync(join(dir, 'ivac_wasm.js'), original);
  writeFileSync(join(dir, 'package.json'), '{}');
  assert.equal(packageState(root).complete, false);
});

test('embedded font/data changes and file deletion invalidate the package', (t) => {
  const root = fixture(t);
  const dir = join(root, 'crates/ivac-core/assets/fonts');
  mkdirSync(dir, { recursive: true });
  const font = join(dir, 'reference.svg');
  writeFileSync(font, 'original glyphs');
  recordBuild(root, inputFingerprint(root));
  writeFileSync(font, 'changed glyphs');
  assert.equal(packageState(root).fresh, false);
  recordBuild(root, inputFingerprint(root));
  rmSync(font);
  assert.equal(packageState(root).fresh, false);
});

test('neutral cutting and preset sources remain freshness inputs after recommendation removal', (t) => {
  const root = fixture(t);
  const dir = join(root, 'crates/ivac-core/src');
  mkdirSync(dir, { recursive: true });
  for (const name of ['cutting_parameters.rs', 'presets.rs', 'project.rs']) {
    recordBuild(root, inputFingerprint(root));
    const path = join(dir, name);
    writeFileSync(path, 'current neutral contract');
    assert.equal(packageState(root).fresh, false);
    recordBuild(root, inputFingerprint(root));
    writeFileSync(path, 'changed neutral contract');
    utimesSync(path, new Date(0), new Date(0));
    assert.equal(packageState(root).fresh, false);
    recordBuild(root, inputFingerprint(root));
    rmSync(path);
    assert.equal(packageState(root).fresh, false);
  }
});

test('unused old source-pack files are not required runtime inputs', (t) => {
  const root = fixture(t);
  recordBuild(root, inputFingerprint(root));
  const dir = join(root, 'data/feeds-speeds');
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, 'private-old-pack.json'), 'unchanged user data');
  assert.equal(packageState(root).fresh, true);
});
