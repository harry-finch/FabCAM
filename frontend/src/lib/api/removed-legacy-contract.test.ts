import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';

it('current transports do not register legacy recommendation or source-pack entry points', () => {
  const source = (path: string) => readFileSync(new URL(path, import.meta.url), 'utf8');
  for (const file of [
    './client.ts',
    './http.ts',
    './tauri.ts',
    './wasm-worker-client.ts',
    './wasm.worker.ts',
    './wasm-worker-protocol.ts',
    './wasm.ts',
  ]) {
    expect(source(file), file).not.toMatch(/feedsSpeeds|validateSourcePack|captureLabPreset/);
  }
  expect(source('../../../../crates/ivac-tauri/src/lib.rs')).not.toMatch(
    /source_packs::|feeds_speeds_cmd/,
  );
  expect(source('../../../../crates/ivac-server/src/main.rs')).not.toContain('/feeds-speeds');
  expect(source('../../../../schema/openapi.yaml')).not.toMatch(
    /FsCam|FsSourcePack|FsContext|AppliedCutting/,
  );
});

it('fresh real WASM has no obsolete exports and refuses old payload fields', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  for (const name of ['feedsSpeeds', 'validateSourcePack', 'captureLabPreset'])
    expect(wasm[name]).toBeUndefined();
  const req = JSON.parse(
    readFileSync(
      new URL('../../../../tests/fixtures/cli-v9/profile.request.json', import.meta.url),
      'utf8',
    ),
  );
  req.post_processor = 'linuxcnc';
  delete req.cps_post;
  expect(wasm.version().capabilities).not.toContain('feeds-speeds');
  expect(wasm.generate(req).gcode).toContain('M');
  const entryPoints = [
    (r: unknown) => wasm.generate(r),
    (r: unknown) => wasm.generateTwoSided(r),
    (r: unknown) =>
      wasm.generateStreaming(r, () => {
        throw new Error('must refuse before events');
      }),
    (r: unknown) => wasm.generateDrawing({ machining: r }),
  ];
  for (const value of [{}, null, undefined]) {
    req.project.fs_context = value;
    for (const generate of entryPoints) expect(() => generate(req)).toThrow(/fs_context/);
  }
  delete req.project.fs_context;
  for (const field of ['applied_cutting', 'fs_full_width']) {
    for (const value of [{}, null, false, undefined]) {
      req.project.operations[0].params[field] = value;
      for (const generate of entryPoints) expect(() => generate(req)).toThrow(new RegExp(field));
    }
    delete req.project.operations[0].params[field];
  }
});
