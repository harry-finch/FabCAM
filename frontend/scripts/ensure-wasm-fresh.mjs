// Release builds fail closed on missing, stale or mixed WASM assets.
// --dev alone permits a complete stale package when no wasm-pack is available.
import { readFileSync } from 'node:fs';
import { join, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { packageState, inputFingerprint, recordBuild } from './wasm-package-state.mjs';

const TAG = '[ensure-wasm]';
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const state = packageState(repoRoot);
if (state.fresh) {
  console.log(`${TAG} complete ivac-wasm package fingerprint verified`);
  process.exit(0);
}
const pkgExists = state.complete;
const before = inputFingerprint(repoRoot);

function hasWasmPack() {
  return spawnSync('wasm-pack --version', { stdio: 'ignore', shell: true }).status === 0;
}

if (!hasWasmPack()) {
  if (process.argv.includes('--dev') && state.complete) {
    console.warn(
      `${TAG} DEV ONLY: using stale WASM (${state.reason}); release build will refuse it.`,
    );
    process.exit(0);
  }
  console.error(
    `${TAG} ERROR: ${state.reason}; wasm-pack is required to produce a verified package. Install: cargo install wasm-pack --locked`,
  );
  process.exit(1);
}

// Prefer wasm-pack's `--mode no-install`: it builds with a wasm-bindgen and
// wasm-opt already on PATH instead of downloading pinned copies from GitHub at
// build time. We want that whenever we can get it — it's the only way to build
// on a network-less host (F-Droid's buildserver cuts the network during the
// build), and it sidesteps the flaky download that has corrupted concurrent
// builds. The catch: the wasm-bindgen CLI must match the `wasm-bindgen` crate
// EXACTLY, or the bindgen step errors. So we only take the no-install path when
// a matching CLI *and* a wasm-opt are present; otherwise we fall back to the
// normal (downloading) mode, which is fine anywhere there's a network.
function probe(cmd) {
  return spawnSync(cmd, { encoding: 'utf8', shell: true });
}
function crateWasmBindgenVersion() {
  try {
    const m = readFileSync(join(repoRoot, 'Cargo.lock'), 'utf8').match(
      /name = "wasm-bindgen"\nversion = "([^"]+)"/,
    );
    return m && m[1];
  } catch {
    return null;
  }
}

const wantBindgen = crateWasmBindgenVersion();
const bindgenProbe = probe('wasm-bindgen --version');
const haveBindgen =
  bindgenProbe.status === 0 && (bindgenProbe.stdout.match(/wasm-bindgen (\S+)/) || [])[1];
const haveOpt = probe('wasm-opt --version').status === 0;
const noInstall = Boolean(haveOpt && wantBindgen && haveBindgen === wantBindgen);

if (!noInstall) {
  // Explain why we're taking the slower, network-dependent path so a failed
  // offline build (or a version-skew bug) is diagnosable.
  if (haveBindgen && wantBindgen && haveBindgen !== wantBindgen) {
    console.warn(
      `${TAG} wasm-bindgen ${haveBindgen} on PATH ≠ crate ${wantBindgen}; ` +
        `falling back to wasm-pack's downloaded toolchain. To build offline: ` +
        `cargo install wasm-bindgen-cli --version ${wantBindgen} --locked`,
    );
  } else {
    console.warn(
      `${TAG} no matching wasm-bindgen/wasm-opt on PATH — wasm-pack will download them. ` +
        `For an offline/deterministic build install: ` +
        `cargo install wasm-bindgen-cli --version ${wantBindgen ?? 'X'} --locked  +  your distro's binaryen`,
    );
  }
}

const cmd =
  'wasm-pack build crates/ivac-wasm --target web --release' +
  (noInstall ? ' --mode no-install' : '') +
  ' -- --locked';
console.log(
  `${TAG} ivac-wasm pkg ${pkgExists ? 'is stale' : 'is missing'} — rebuilding ` +
    `(${noInstall ? 'offline, --mode no-install' : 'downloading toolchain'})…`,
);
// cwd carries any spaces in the repo path; the command string itself has no
// spaced arguments, so shell:true is safe cross-platform.
const res = spawnSync(cmd, { cwd: repoRoot, stdio: 'inherit', shell: true });
if (res.status !== 0) process.exit(res.status ?? 1);
try {
  recordBuild(repoRoot, before);
  if (!packageState(repoRoot).fresh) throw new Error('WASM package verification failed');
  console.log(`${TAG} rebuilt package verified`);
} catch (error) {
  console.error(`${TAG} ERROR: ${error.message}`);
  process.exit(1);
}
