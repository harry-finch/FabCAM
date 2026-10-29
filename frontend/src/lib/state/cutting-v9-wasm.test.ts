import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { authorV10 } from './alpha3.fixture';
it('real WASM JSON preserves incomplete roles and validates collections', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  const request = JSON.parse(
    readFileSync(
      new URL('../../../../tests/fixtures/test1/profile.request.json', import.meta.url),
      'utf8',
    ),
  );
  const p = request.project;
  authorV10({ project: p } as Parameters<typeof authorV10>[0]);
  const op = p.operations[0];
  delete op.params.applied_cutting;
  delete op.params.feed_rate_override;
  delete op.params.plunge_rate_override;
  op.params.stages.main.rpm = null;
  const inspect = JSON.parse(
    wasm.presetCommand(JSON.stringify({ action: 'inspect', project: p, op_id: op.id })),
  );
  expect(inspect.parameters.stages.main.rpm).toBeNull();
  expect(inspect.check.errors.join(' ')).toContain('positive');
  const collection = {
    format: 'fabcam-preset-collection',
    version: 2,
    id: 'lab',
    name: 'Lab',
    presets: [],
  };
  expect(
    JSON.parse(
      wasm.presetCommand(
        JSON.stringify({ action: 'validate_collection', json: JSON.stringify(collection) }),
      ),
    ),
  ).toEqual(collection);
  expect(() =>
    wasm.presetCommand(
      JSON.stringify({
        action: 'validate_collection',
        json: JSON.stringify({ ...collection, id: '../outside' }),
      }),
    ),
  ).toThrow();
  op.params.stages.main.rpm = 1.25;
  expect(() =>
    wasm.presetCommand(JSON.stringify({ action: 'inspect', project: p, op_id: op.id })),
  ).toThrow();
});

it('real WASM reviews and copies a tested preset while preserving the assigned context', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  const p = JSON.parse(
    readFileSync(
      new URL('../../../../tests/fixtures/test1/profile.request.json', import.meta.url),
      'utf8',
    ),
  ).project;
  authorV10({ project: p } as Parameters<typeof authorV10>[0]);
  p.machine.physical_id = 'lab-router';
  p.material = { id: 'plywood', revision: '1' };
  p.tools.forEach((t: { physical_id: string; id: number }) => {
    t.physical_id = `bit-${t.id}`;
  });
  const op = p.operations[0];

  const command = (request: unknown) => JSON.parse(wasm.presetCommand(JSON.stringify(request)));
  const inspected = command({ action: 'inspect', project: p, op_id: op.id });
  expect(inspected.check.errors).toEqual([]);
  const preset = {
    id: 'tested',
    name: 'Tested',
    context: inspected.context,
    parameters: inspected.parameters,
    tested: true,
    test_date: null,
    notes: 'private workshop notes',
  };
  const collection = {
    format: 'fabcam-preset-collection',
    version: 2,
    id: 'lab',
    name: 'Lab',
    presets: [preset],
  };
  op.params.stages.main.feed_mm_min = 900;
  const before = JSON.stringify(p);
  const loaded = command({
    action: 'load',
    project: p,
    op_id: op.id,
    collection,
    preset_id: 'tested',
  });
  expect(loaded.params.stages.main.feed_mm_min).toBe(800);
  expect(JSON.stringify(p)).toBe(before);
  expect(JSON.stringify(loaded.params.stage_preset)).not.toContain('private workshop');
  p.machine.physical_id = 'different-router';
  expect(() =>
    command({ action: 'load', project: p, op_id: op.id, collection, preset_id: 'tested' }),
  ).toThrow();
});

it('real WASM generation adapters require v10 and preserve explicit profile output', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  const req = JSON.parse(
    readFileSync(
      new URL('../../../../tests/fixtures/test1/profile.request.json', import.meta.url),
      'utf8',
    ),
  );
  req.post_processor = 'linuxcnc';
  req.cps_post = null;
  const calls = [
    () => wasm.generate(req),
    () => wasm.generateTwoSided(req),
    () => wasm.generateStreaming(req, () => {}),
  ];
  for (const call of calls) expect(call).toThrow('v10');
  authorV10(req);
  const op = req.project.operations[0];

  const first = wasm.generate(req);
  expect(first.gcode).toContain('S12000');
  expect(first.gcode).toContain('F800');
  req.project.tools[0].speed = 99999;
  req.project.tools[0].feed_rate = 99999;
  expect(wasm.generate(req).gcode).toEqual(first.gcode);
  expect(wasm.generateTwoSided(req).front.gcode).toEqual(first.gcode);
  expect(wasm.generateStreaming(req, () => {}).gcode).toEqual(first.gcode);
  op.params.stages.main.rpm = null;
  for (const call of calls) expect(call).toThrow();
  op.params.stages.main.rpm = 50000;
  req.project.tools[0].max_rpm = 20000;
  for (const call of calls) expect(call).toThrow();
});
