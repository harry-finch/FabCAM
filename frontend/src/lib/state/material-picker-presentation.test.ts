import { describe, expect, it } from 'vitest';
import en from '../i18n/messages/en.json';
import type { MsgKey } from '../i18n';
import { materialPickerLabel, materialPage } from './material-picker-presentation';
import { materialOptions, materialIdentityKey } from './material-presentation';
import type { MaterialSource } from './material-presentation';
const translate = (key: MsgKey) => en[key];
const label = (id: string, fallback = id, revision = '1') =>
  materialPickerLabel({ id, revision }, fallback, translate);

describe('compact material labels preserve exact classification boundaries', () => {
  it.each([
    ['material/metal.aluminium', 'Aluminium — Generic'],
    ['material/metal.aluminium.6061', 'Aluminium 6061'],
    ['material/metal.aluminium.7075', 'Aluminium 7075'],
    ['material/plastic.pmma', 'PMMA — Generic'],
    ['material/plastic.pmma.cast', 'PMMA — Cast'],
    ['material/plastic.pmma.extruded', 'PMMA — Extruded'],
    ['material/wood.plywood.birch', 'Plywood — Birch'],
    ['material/wood.mdf', 'MDF'],
  ])('renders %s concisely', (id, expected) => expect(label(id)).toBe(expected));
  it('keeps removed manufacturer categories as custom identities without inventing a variant', () => {
    expect(label('sienci-softwood-soft-plywood-mdf')).toBe('Custom / unavailable material');
  });
  it('does not classify unknown IDs/revisions or expose technical IDs as collapsed names', () => {
    expect(label('private-material', 'Original human label')).toBe('Original human label');
    expect(label('private-material')).toBe('Custom / unavailable material');
    expect(label('material/metal.aluminium.6061', 'Private 6061', 'different')).toBe(
      'Private 6061',
    );
    expect(label('material/wood.not-reviewed')).toBe('Custom / unavailable material');
  });
});

const source = (id: string, count: number): MaterialSource => ({
  id,
  label: `Software source ${id}`,
  catalog: {
    materials: Array.from({ length: count }, (_, i) => ({
      identity: { id: `material-fixture-${i}`, revision: '7' },
      label: `Homonymous long label ${'é'.repeat(100)}`,
    })),
  },
});
describe('bounded material browser projection', () => {
  it('keeps 30 results per page, all distinct identities and both source occurrences', () => {
    const options = materialOptions([source('a@1', 65), source('b@2', 65)]);
    const before = JSON.stringify(options);
    const first = materialPage(options, 'homonymous', 'unknown', '', 0);
    const last = materialPage(options, 'homonymous', 'unknown', '', 99);
    expect(first).toMatchObject({ count: 65, page: 0, pages: 3 });
    expect(first.visible).toHaveLength(30);
    expect(last.visible).toHaveLength(5);
    expect(last.page).toBe(2);
    expect(first.visible[0].sources.map((s) => s.key)).toEqual(['a@1', 'b@2']);
    expect(new Set(options.map((o) => o.key)).size).toBe(options.length);
    expect(JSON.stringify(options)).toBe(before);
  });
  it('preserves controlled aliases, case/accent search and one-result/empty filtering without mutation', () => {
    const current = Object.freeze({
      id: 'unavailable-id',
      revision: 'legacy-7',
      label: 'Échantillon',
    });
    const options = materialOptions([], current);
    const selectedKey = materialIdentityKey(current);
    expect(materialPage(options, 'ACRYLIQUE', '', '', 0).visible).toHaveLength(3);
    expect(materialPage(options, 'echantillon', 'unknown', '', 0).visible[0].key).toBe(selectedKey);
    expect(materialPage(options, '6061', '', '', 99)).toMatchObject({
      count: 1,
      page: 0,
      pages: 1,
    });
    expect(materialPage(options, '', '', 'disabled-source@1', 0).visible).toEqual([]);
    expect(materialPage(options, 'no match', '', '', 0)).toMatchObject({
      count: 0,
      visible: [],
      page: 0,
      pages: 1,
    });
    expect(materialIdentityKey(current)).toBe(selectedKey);
    expect(current.label).toBe('Échantillon');
  });
});
