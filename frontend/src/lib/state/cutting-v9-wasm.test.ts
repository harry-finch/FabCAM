import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
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
  p.cutting_version = 9;
  const op = p.operations[0];
  delete op.params.applied_cutting;
  delete op.params.feed_rate_override;
  delete op.params.plunge_rate_override;
  op.params.step = -1;
  op.params.cutting = {
    rates: ['rough', 'finish'].map((role) => ({
      role,
      tool_id: op.tool_id,
      rpm: null,
      feed_mm_min: 800,
      plunge_mm_min: 100,
    })),
  };
  const inspect = JSON.parse(
    wasm.presetCommand(JSON.stringify({ action: 'inspect', project: p, op_id: op.id })),
  );
  expect(inspect.parameters.rates[0].rpm).toBeNull();
  expect(inspect.check.errors.join(' ')).toContain('positive');
  const collection = {
    format: 'fabcam-preset-collection',
    version: 1,
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
  op.params.cutting.rates[0].rpm = 1.25;
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
  p.cutting_version = 9;
  p.machine.physical_id = 'lab-router';
  p.material = { id: 'plywood', revision: '1' };
  p.tools.forEach((t: { physical_id: string; id: number }) => {
    t.physical_id = `bit-${t.id}`;
  });
  const op = p.operations[0];
  op.params.step = -1;
  op.params.cutting = {
    rates: ['rough', 'finish'].map((role) => ({
      role,
      tool_id: op.tool_id,
      rpm: 12000,
      feed_mm_min: 800,
      plunge_mm_min: 100,
    })),
  };
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
    version: 1,
    id: 'lab',
    name: 'Lab',
    presets: [preset],
  };
  op.params.cutting.rates[0].feed_mm_min = 900;
  const before = JSON.stringify(p);
  const loaded = command({
    action: 'load',
    project: p,
    op_id: op.id,
    collection,
    preset_id: 'tested',
  });
  expect(loaded.params.cutting.rates[0].feed_mm_min).toBe(800);
  expect(JSON.stringify(p)).toBe(before);
  expect(JSON.stringify(loaded.params.cutting.preset_snapshot)).not.toContain('private workshop');
  p.machine.physical_id = 'different-router';
  expect(() =>
    command({ action: 'load', project: p, op_id: op.id, collection, preset_id: 'tested' }),
  ).toThrow();
});
