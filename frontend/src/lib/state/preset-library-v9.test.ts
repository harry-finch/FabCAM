import { expect, it } from 'vitest';
import {
  attestationKey,
  editTestedPreset,
  importChoice,
  testedRows,
  filterTestedRows,
  emptyFilters,
  collectionMaterials,
  parameterSummary,
} from './preset-library-v9';
import type { PresetCollection, TestedPreset } from './preset-collections';
const preset = (): TestedPreset => ({
  id: 'test',
  name: 'Birch Ø6',
  context: {
    machine_id: 'lab-router',
    machine_label: 'Router',
    material_id: 'material/wood.plywood.birch',
    material_label: 'Birch',
    signature: JSON.stringify({
      operation: { type: 'pocket' },
      tools: [{ physical_id: 'bit-6' }],
      material: { id: 'material/wood.plywood.birch', revision: '1' },
    }),
  },
  parameters: {
    rates: [{ role: 'rough', tool_id: 1, rpm: 12000, feed_mm_min: 800, plunge_mm_min: 100 }],
    schedule: { kind: 'step', step: -1, finish_step: null },
    pocket_overlap: 0.4,
    planar_stepover_mm: null,
  },
  tested: true,
  test_date: null,
  notes: null,
});
const collection = (): PresetCollection => ({
  format: 'fabcam-preset-collection',
  version: 1,
  id: 'lab',
  name: 'Workshop',
  presets: [preset()],
});
it('editing context, rates or date requires an exact new attestation; names and notes preserve the author claim', () => {
  const original = preset(),
    edited = structuredClone(original);
  edited.name = 'New label';
  edited.notes = 'Preserved original source claim';
  expect(editTestedPreset(original, edited, '').name).toBe('New label');
  for (const mutate of [
    (p: TestedPreset) => (p.parameters.rates[0].rpm = 14000),
    (p: TestedPreset) => (p.context.machine_id = 'other'),
    (p: TestedPreset) => (p.test_date = '2026-10-22'),
  ]) {
    const draft = structuredClone(original);
    mutate(draft);
    expect(() => editTestedPreset(original, draft, '')).toThrow('Confirm');
    const attested = attestationKey(draft);
    expect(editTestedPreset(original, draft, attested)).toEqual(draft);
    draft.parameters.rates[0].feed_mm_min = 999;
    expect(() => editTestedPreset(original, draft, attested)).toThrow('Confirm');
  }
  expect(() => editTestedPreset(null, original, '')).toThrow('Confirm');
  expect(original).toEqual(preset());
});
it('import collisions require explicit choice and bind the reviewed optimistic token', () => {
  const incoming = collection(),
    existing = { data: collection(), token: 'reviewed' };
  expect(() => importChoice(incoming, existing, 'new')).toThrow('Choose');
  expect(() => importChoice(incoming, undefined, 'replace')).toThrow('refresh');
  expect(importChoice(incoming, existing, 'replace')).toEqual({
    data: incoming,
    expected: 'reviewed',
  });
  const copy = importChoice(incoming, existing, 'copy');
  expect(copy.expected).toBeNull();
  expect(copy.data.id).not.toBe(incoming.id);
  expect(copy.data.presets).toEqual(incoming.presets);
  expect(incoming).toEqual(collection());
});
it('display filtering never establishes compatibility or mutates collection and material identities', () => {
  const c = collection(),
    before = JSON.stringify(c),
    rows = testedRows([c]);
  expect(
    filterTestedRows(rows, {
      ...emptyFilters(),
      query: 'birch bit-6',
      machine: 'lab-router',
      material: 'material/wood.plywood.birch',
      operation: 'pocket',
    }),
  ).toHaveLength(1);
  expect(filterTestedRows(rows, { ...emptyFilters(), machine: 'other-router' })).toEqual([]);
  expect(collectionMaterials([c])[0].catalog.materials[0].identity).toEqual({
    id: 'material/wood.plywood.birch',
    revision: '1',
  });
  expect(parameterSummary(c.presets[0].parameters)).toEqual([
    'Depth per pass: 1 mm',
    'Pocket overlap: 40%',
  ]);
  expect(JSON.stringify(c)).toBe(before);
  c.presets[0].context.signature = '{}';
  expect(collectionMaterials([c])[0].catalog.materials).toEqual([]);
  expect(testedRows([c])[0].operation).toBe('Unknown operation');
});
