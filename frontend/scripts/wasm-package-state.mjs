import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

export const assets = [
  'ivac_wasm_bg.wasm',
  'ivac_wasm.js',
  'ivac_wasm.d.ts',
  'ivac_wasm_bg.wasm.d.ts',
  'package.json',
];
const stampName = '.fabcam-build.json';
const inputs = [
  'Cargo.toml',
  'Cargo.lock',
  'rust-toolchain.toml',
  'rust-toolchain',
  '.cargo',
  'crates/ivac-core/assets',
  'data/feeds-speeds',
  ...['ivac-wasm', 'ivac-core', 'ivac-feeds-speeds'].flatMap((name) =>
    ['src', 'Cargo.toml', 'build.rs'].map((file) => `crates/${name}/${file}`),
  ),
  'frontend/scripts/ensure-wasm-fresh.mjs',
  'frontend/scripts/wasm-package-state.mjs',
];
const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const packageDir = (root) => join(root, 'crates/ivac-wasm/pkg');

// Hash contents and names (including absence), never filesystem timestamps.
export function inputFingerprint(root) {
  const hash = createHash('sha256');
  function visit(relative) {
    const path = join(root, relative);
    hash.update(relative + '\0');
    if (!existsSync(path)) {
      hash.update('missing\0');
      return;
    }
    if (statSync(path).isDirectory()) {
      for (const name of readdirSync(path).sort()) visit(relative + '/' + name);
    } else hash.update(readFileSync(path));
  }
  inputs.forEach(visit);
  return hash.digest('hex');
}

export function assetHashes(root) {
  const dir = packageDir(root);
  const hashes = {};
  for (const asset of assets) {
    const path = join(dir, asset);
    if (!existsSync(path) || !statSync(path).isFile() || statSync(path).size === 0) {
      throw new Error(`Missing or empty WASM asset: ${asset}`);
    }
    hashes[asset] = digest(readFileSync(path));
  }
  const pkg = JSON.parse(readFileSync(join(dir, 'package.json'), 'utf8'));
  if (
    (pkg.module ?? pkg.main) !== 'ivac_wasm.js' ||
    pkg.types !== 'ivac_wasm.d.ts' ||
    !['ivac_wasm_bg.wasm', 'ivac_wasm.js', 'ivac_wasm.d.ts'].every((a) => pkg.files?.includes(a))
  ) {
    throw new Error('WASM package manifest does not declare the expected bindings');
  }
  const magic = readFileSync(join(dir, 'ivac_wasm_bg.wasm')).subarray(0, 8);
  if (!magic.equals(Buffer.from([0, 97, 115, 109, 1, 0, 0, 0])))
    throw new Error('Invalid WASM header');
  return hashes;
}

export function packageState(root) {
  let hashes;
  try {
    hashes = assetHashes(root);
  } catch (error) {
    return { complete: false, fresh: false, reason: error.message };
  }
  try {
    const stamp = JSON.parse(readFileSync(join(packageDir(root), stampName), 'utf8'));
    const fresh =
      stamp.version === 1 &&
      stamp.input === inputFingerprint(root) &&
      assets.every((a) => stamp.assets?.[a] === hashes[a]);
    return {
      complete: true,
      fresh,
      reason: fresh ? 'verified' : 'source or asset fingerprint changed',
    };
  } catch {
    return { complete: true, fresh: false, reason: 'missing build fingerprint' };
  }
}

export function recordBuild(root, before) {
  if (inputFingerprint(root) !== before) throw new Error('WASM inputs changed during build; retry');
  const stamp = { version: 1, input: before, assets: assetHashes(root) };
  writeFileSync(join(packageDir(root), stampName), JSON.stringify(stamp, null, 2) + '\n');
}
