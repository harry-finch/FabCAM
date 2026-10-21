import { describe, it, expect } from 'vitest';
import { comparisonValue, recordFocusIndex } from './cutting-comparison';
import { displayNumber } from './recommendation-preview';

describe('cutting selector comparison presentation', () => {
  it('keeps identical values numeric and permits a provenance-only application', () => {
    const row = comparisonValue(17000, 17000);
    expect(row).toEqual({
      current: 17000,
      proposed: 17000,
      applicable: true,
      retained: false,
      changed: false,
    });
    expect(displayNumber(row.proposed!, 3)).toBe('17,000');
  });
  it('highlights a changed value without calculating or scaling a preset', () => {
    expect(comparisonValue(800, 1620)).toMatchObject({ proposed: 1620, changed: true });
    expect(displayNumber(2.8, 3)).toBe('2.8');
  });
  it('distinguishes a retained optional value from unavailable and inapplicable values', () => {
    expect(comparisonValue(40, undefined)).toMatchObject({
      proposed: 40,
      retained: true,
      changed: false,
    });
    expect(comparisonValue(undefined, null)).toMatchObject({
      proposed: undefined,
      applicable: true,
      retained: false,
    });
    expect(comparisonValue(undefined, 40, false)).toMatchObject({
      current: undefined,
      proposed: undefined,
      applicable: false,
      changed: false,
    });
    expect(comparisonValue(undefined, 40)).toMatchObject({ proposed: 40, changed: true });
  });
  it('does not confuse a zero value with absent data', () => {
    expect(comparisonValue(0, 0)).toMatchObject({ proposed: 0, retained: false, changed: false });
    expect(comparisonValue(0, undefined)).toMatchObject({ proposed: 0, retained: true });
  });
});

describe('compact preset list keyboard focus', () => {
  it('handles many records, boundaries and Home/End without implicit selection', () => {
    expect(recordFocusIndex('ArrowDown', 0, 50)).toBe(1);
    expect(recordFocusIndex('ArrowDown', 49, 50)).toBe(49);
    expect(recordFocusIndex('ArrowUp', 0, 50)).toBe(0);
    expect(recordFocusIndex('Home', 49, 50)).toBe(0);
    expect(recordFocusIndex('End', 0, 50)).toBe(49);
  });
  it('leaves activation and Tab to native buttons and handles empty filters', () => {
    for (const key of ['Enter', ' ', 'Tab', 'Escape'])
      expect(recordFocusIndex(key, 0, 50)).toBeUndefined();
    expect(recordFocusIndex('ArrowDown', 0, 0)).toBeUndefined();
  });
});

import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v3.json';
import type { SourcePack } from './feeds-speeds-sources';
import { emptyPresetFilters, filterPresets, presetRows } from './preset-library';
import { recordSelection } from './cutting-selector';

it('filters a large long-label collection without changing the pinned record', () => {
  const packs = Array.from({ length: 30 }, (_, i) => {
    const pack = structuredClone(reference) as unknown as SourcePack;
    pack.manifest.identity.id = `software-ui-${i}`;
    pack.manifest.name = `Atelier ${i} — ${'Long localized label '.repeat(12)}`;
    return pack;
  });
  const rows = presetRows(packs, new Set(), true);
  expect(rows).toHaveLength(180);
  expect(new Set(rows.map((row) => row.key)).size).toBe(180);
  const selected = recordSelection(rows[75]);
  const before = JSON.stringify(selected);
  expect(
    filterPresets(rows, { ...emptyPresetFilters(), availability: 'all', query: 'no-such-record' }),
  ).toEqual([]);
  const pack = rows[75].pack.manifest.identity;
  const filtered = filterPresets(rows, {
    ...emptyPresetFilters(),
    availability: 'all',
    pack: `${pack.id}@${pack.revision}`,
    query: 'localized label',
  });
  expect(filtered).toHaveLength(6);
  expect(filtered.some((row) => JSON.stringify(recordSelection(row)) === before)).toBe(true);
  expect(JSON.stringify(selected)).toBe(before);
});
