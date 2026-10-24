import { expect, it } from 'vitest';
import { isPresetContextMismatch, recordedPresetContext } from './preset-context-review';
import type { TestedPreset } from './preset-collections';

function fixture(): TestedPreset {
  return {
    id: 'old',
    name: 'Synthetic QA only',
    tested: true,
    test_date: '2026-10-01',
    notes: 'Never machined',
    context: {
      machine_id: 'recorded-machine',
      machine_label: 'Old machine',
      material_id: 'plywood',
      material_label: 'Recorded material',
      signature: JSON.stringify({
        machine: 'recorded-machine',
        machine_setup: { physical_id: 'recorded-machine', work_area: { x: 1200 }, unit: 'mm' },
        mode: 'mill',
        material: { id: 'plywood', revision: 'old-revision' },
        operation: { type: 'pocket', pocket: { nocontour: false } },
        plunge: { kind: 'direct' },
        tools: [
          {
            role: 'rough',
            physical_id: 'rough-bit',
            geometry: { diameter: 15.12270614261456, flute_length_mm: null },
          },
          {
            role: 'finish',
            physical_id: 'finish-bit',
            geometry: { diameter: 3, tip_angle_deg: 60 },
          },
        ],
        preserved_extension: { nested: ['<img src=x onerror=alert(1)>', false, null] },
      }),
    },
    parameters: {
      rates: [{ role: 'rough', tool_id: 1, rpm: 12000, feed_mm_min: 800, plunge_mm_min: 100 }],
      schedule: { kind: 'step', step: -1 },
      pocket_overlap: null,
      planar_stepover_mm: null,
    },
  };
}

it('recognizes only the exact compatibility error and preserves other safety explanations', () => {
  const message = 'Preset context differs: review machine, material, tool geometry and operation';
  expect(isPresetContextMismatch(message)).toBe(true);
  expect(isPresetContextMismatch(`Error: ${message}`)).toBe(true);
  for (const error of [
    'RPM exceeds tool limit',
    'Preset role is not supported',
    'Invalid collection',
    `prefix ${message}`,
    `${message}; RPM invalid`,
    '',
  ])
    expect(isPresetContextMismatch(error)).toBe(false);
});

it('presents every saved leaf, including actual roles, geometry, revision and extension fields without mutation', () => {
  const p = fixture(),
    before = JSON.stringify(p);
  const review = recordedPresetContext(p);
  expect(review.available).toBe(true);
  if (!review.available) throw new Error('fixture');
  const fields = Object.fromEntries(review.fields.map((field) => [field.path, field.value]));
  expect(fields['machine']).toBe('recorded-machine');
  expect(fields['machine setup / work area / x']).toBe('1200');
  expect(fields['material / revision']).toBe('old-revision');
  expect(fields['operation / pocket / nocontour']).toBe('false');
  expect(fields['tools / [1] / geometry / diameter']).toBe('15.12270614261456');
  expect(fields['tools / [1] / geometry / flute length mm']).toBe('null');
  expect(fields['tools / [2] / physical id']).toBe('finish-bit');
  expect(fields['tools / [2] / role']).toBe('finish');
  expect(fields['preserved extension / nested / [1]']).toBe('<img src=x onerror=alert(1)>');
  expect(review.fields).toHaveLength(21);
  expect(JSON.stringify(p)).toBe(before);
});

it('never substitutes current display labels for recorded values or grants a new test claim', () => {
  const p = fixture();
  const before = recordedPresetContext(p);
  p.context.machine_label = 'Current display name';
  p.name = 'Renamed copy';
  p.id = 'duplicate';
  expect(recordedPresetContext(p)).toEqual(before);
  expect(p.test_date).toBe('2026-10-01');
  expect(p.notes).toBe('Never machined');
});

it('bounds malformed, missing and inconsistent contexts without inventing values', () => {
  for (const signature of ['bad JSON', '{}', 'null', '[]', '{"machine":"wrong"}']) {
    const p = fixture();
    p.context.signature = signature;
    expect(recordedPresetContext(p).available).toBe(false);
    expect(p.context.signature).toBe(signature);
  }
  const p = fixture();
  p.context.material_id = 'different';
  expect(recordedPresetContext(p)).toEqual({ available: false, reason: 'unsupported' });
});

it('refuses unsafe numeric presentation and excessive size, nesting or row counts', () => {
  const p = fixture();
  const saved = JSON.parse(p.context.signature);
  for (const value of [
    1e100,
    { x: Array(2050).fill(1) },
    Array.from({ length: 18 }).reduce<unknown>((v) => ({ nested: v }), 1),
  ]) {
    p.context.signature = JSON.stringify({ ...saved, extra: value });
    expect(recordedPresetContext(p)).toEqual({ available: false, reason: 'limit' });
  }
  p.context.signature = p.context.signature.replace('1', '1e400');
  expect(recordedPresetContext(p).available).toBe(false);
  p.context.signature = 'é'.repeat(32769);
  expect(recordedPresetContext(p)).toEqual({ available: false, reason: 'limit' });
});
