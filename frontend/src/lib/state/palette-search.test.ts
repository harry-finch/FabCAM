import { describe, it, expect } from 'vitest';
import { buildSearchIndex, searchIndex, parseSearch, type SearchRecord } from './palette-search';
const records: SearchRecord[] = [
  {
    id: 'command:pocket',
    category: 'command',
    label: 'Add Pocket operation',
    aliases: 'poche Tasche',
  },
  { id: 'tool:inventory:1', category: 'tool', label: '6mm upcut', diameter: 6, flutes: 2 },
  { id: 'tool:project:1', category: 'tool', label: '6.35 mm upcut', diameter: 6.35, flutes: 1 },
  {
    id: 'tool:inventory:2',
    category: 'tool',
    label: 'single flute cutter',
    diameter: 3.175,
    flutes: 1,
  },
  {
    id: 'tool:inventory:3',
    category: 'tool',
    label: 'V-bit 60',
    aliases: 'v bit',
    diameter: 6,
    angle: 60,
  },
  {
    id: 'tool:inventory:4',
    category: 'tool',
    label: 'V-bit 60 unknown metadata',
    aliases: 'v bit',
    diameter: 6,
  },
];
const index = buildSearchIndex(records);
describe('palette metadata search', () => {
  it.each(['poche', 'Tasche', 'pocket', 'pokcet', 'pocke'])(
    'finds local aliases and bounded typos: %s',
    (query) => expect(searchIndex(index, query)[0].id).toBe('command:pocket'),
  );
  it('keeps exact before typo irrespective of relevance', () => {
    const ranked = buildSearchIndex([
      { id: 'a', category: 'command' as const, label: 'Save', relevance: 0 },
      { id: 'b', category: 'command' as const, label: 'Sage', relevance: 99 },
    ]);
    expect(searchIndex(ranked, 'save').map((row) => row.id)).toEqual(['a', 'b']);
  });
  it('filters real diameter without approximate substitutions', () =>
    expect(searchIndex(index, '6mm upcut').map((r) => r.id)).toEqual(['tool:inventory:1']));
  it.each(['3.175 single flute', '3,175 single flute', '1/8in', '0.125 inch'])(
    'uses exact physical units: %s',
    (query) => expect(searchIndex(index, query).map((r) => r.id)).toEqual(['tool:inventory:2']),
  );
  it('does not infer angles from labels', () =>
    expect(searchIndex(index, 'V-bit 60').map((r) => r.id)).toEqual(['tool:inventory:3']));
  it('preserves origin collisions and numeric IDs', () =>
    expect(searchIndex(index, 'upcut')).toHaveLength(2));
  it('never fuzzes numbers', () => expect(searchIndex(index, '6.3mm')).toEqual([]));
  it('leaves bare integers textual', () => expect(parseSearch('60').diameter).toBeUndefined());
  it('does not mutate source objects', () => {
    const before = JSON.stringify(records);
    searchIndex(index, 'Tasche');
    expect(JSON.stringify(records)).toBe(before);
  });
  it('breaks ties deterministically independent of input order', () =>
    expect(searchIndex(buildSearchIndex([...records].reverse()), '').map((r) => r.id)).toEqual(
      searchIndex(index, '').map((r) => r.id),
    ));
  it('rejects unknown flute metadata', () =>
    expect(searchIndex(index, '6mm single flute')).toEqual([]));
  it('preserves accent normalization', () =>
    expect(
      searchIndex(
        buildSearchIndex([{ id: 'x', category: 'command', label: 'Ébauche' }]),
        'ebauche',
      ),
    ).toHaveLength(1));
});

it('ranks the actual name before a generic kind alias', () => {
  const index = buildSearchIndex([
    {
      id: 'a',
      category: 'template' as const,
      label: 'UX4.1 saved from operation',
      aliases: 'pocket',
    },
    {
      id: 'z',
      category: 'template' as const,
      label: 'UX4.1 pocket — retained values',
      aliases: 'pocket',
    },
  ]);
  expect(searchIndex(index, 'UX4.1 pocket')[0].id).toBe('z');
});
