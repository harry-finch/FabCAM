// Collect one explicitly targeted installer; never publish or label it qualified.
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { basename, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';
import { packageState } from '../frontend/scripts/wasm-package-state.mjs';

export const targets = {
  macos: { target: 'aarch64-apple-darwin', folder: 'dmg', suffix: '.dmg' },
  windows: { target: 'x86_64-pc-windows-msvc', folder: 'nsis', suffix: '-setup.exe' },
  ubuntu: { target: 'x86_64-unknown-linux-gnu', folder: 'deb', suffix: '.deb' },
};

export function collectCandidate({ platform, target, version, bundleDir, outputDir, provenance }) {
  const spec = targets[platform];
  if (!spec || target !== spec.target) throw new Error('Unsupported candidate platform/target pair');
  const dir = join(bundleDir, spec.folder);
  const files = existsSync(dir) ? readdirSync(dir).filter(name => name.endsWith(spec.suffix)) : [];
  if (files.length !== 1 || !files[0].includes(`_${version}_`)) {
    throw new Error(`Expected exactly one ${platform} installer for ${version}; found: ${files.join(', ')}`);
  }
  const src = join(dir, files[0]);
  const bytes = readFileSync(src);
  if (!bytes.length) throw new Error('Empty installer');
  if (existsSync(outputDir) && readdirSync(outputDir).length) {
    throw new Error('Candidate destination must be empty; preserve previous candidates separately');
  }
  mkdirSync(outputDir, { recursive: true });
  copyFileSync(src, join(outputDir, basename(src)));
  const sha256 = createHash('sha256').update(readFileSync(join(outputDir, files[0]))).digest('hex');
  const manifest = {
    format: 1, product: 'FabCAM', version, platform, declared_build_target: target,
    software_test_candidate: true, installation_qualified: false, runtime_qualified: false,
    physical_validation: false, signature_status: 'not certified by this collector',
    provenance, artifact: { file: files[0], bytes: bytes.length, sha256 },
  };
  writeFileSync(join(outputDir, 'candidate.json'), JSON.stringify(manifest, null, 2) + '\n');
  writeFileSync(join(outputDir, 'SHA256SUMS.txt'), `${sha256}  ${files[0]}\n`);
  return manifest;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const root = resolve(fileURLToPath(new URL('..', import.meta.url)));
    const [platform, target, suppliedBundleDir] = process.argv.slice(2);
    const state = packageState(root);
    if (!state.fresh) throw new Error(`WASM package is not verified: ${state.reason}`);
    const version = JSON.parse(readFileSync(join(root, 'crates/ivac-tauri/tauri.conf.json'), 'utf8')).version;
    const git = (...args) => execFileSync('git', args, { cwd: root, encoding: 'utf8' }).trim();
    const manifest = collectCandidate({
      platform, target, version,
      bundleDir: suppliedBundleDir ? resolve(suppliedBundleDir) : join(root, 'target', target, 'release/bundle'),
      outputDir: join(root, 'dist-test/desktop-candidates', platform),
      provenance: { commit: git('rev-parse', 'HEAD'), dirty: !!git('status', '--porcelain'),
        built_on: process.platform, collected_at: new Date().toISOString(),
        wasm: JSON.parse(readFileSync(join(root, 'crates/ivac-wasm/pkg/.fabcam-build.json'), 'utf8')),
      },
    });
    console.log(JSON.stringify(manifest, null, 2));
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
