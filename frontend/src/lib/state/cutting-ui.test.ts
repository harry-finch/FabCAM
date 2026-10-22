import { expect, it } from 'vitest';
import { presetLibraryState, cuttingWarningGroups, cuttingRoleHint } from './cutting-ui';
import { emptyFilters } from './preset-library-v9';
import type { PresetCollection } from './preset-collections';

const collection = (id: string, count: number) =>
  ({ id, presets: Array.from({ length: count }, () => ({})) }) as PresetCollection;

it('distinguishes no collections, an empty selected collection, and filtered-out results', () => {
  const f = emptyFilters();
  expect(presetLibraryState([], f, 0)).toEqual({ kind: 'no-collections', scopeCount: 0 });
  const collections = [collection('empty', 0), collection('workshop', 200)];
  expect(presetLibraryState(collections, { ...f, collection: 'empty' }, 0).kind).toBe(
    'empty-collection',
  );
  expect(presetLibraryState(collections, { ...f, query: 'no match' }, 0)).toEqual({
    kind: 'no-matches',
    scopeCount: 200,
  });
  expect(presetLibraryState(collections, f, 200)).toEqual({ kind: 'results', scopeCount: 200 });
  expect(presetLibraryState([collection('empty', 0)], f, 0).kind).toBe('empty-collection');
});

it('groups identical role diagnostics without inventing which ratings are missing', () => {
  const message =
    'some machine/tool limits are unknown; verify spindle, feed and usable flute length';
  const warnings = [
    `Rough: ${message}`,
    `Finish: ${message}`,
    'Spot: different warning',
    'raw warning',
  ];
  const before = [...warnings];
  expect(cuttingWarningGroups(warnings)).toEqual([
    { message, roles: ['Rough', 'Finish'] },
    { message: 'different warning', roles: ['Spot'] },
    { message: 'raw warning', roles: [] },
  ]);
  expect(warnings).toEqual(before);
  expect(cuttingWarningGroups([])).toEqual([]);
});

it('describes Profile contour ownership only, leaving other operation roles explicit', () => {
  expect(cuttingRoleHint('profile', 'finish')).toContain('control the Profile contour');
  expect(cuttingRoleHint('profile', 'rough')).toContain('Stored main values');
  for (const kind of ['pocket', 'drill', 'chamfer', 'relief_mill'])
    expect(cuttingRoleHint(kind, 'rough')).toBeNull();
});
