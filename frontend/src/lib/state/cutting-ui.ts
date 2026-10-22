/** Presentation only; these projections never authorize application or generation. */
import type { PresetCollection } from './preset-collections';
import type { PresetFilters } from './preset-library-v9';
import type { CuttingRole } from './cutting-parameters';

export function presetLibraryState(
  collections: PresetCollection[],
  filters: PresetFilters,
  matches: number,
) {
  const scopeCount = collections
    .filter((c) => !filters.collection || c.id === filters.collection)
    .reduce((n, c) => n + c.presets.length, 0);
  const kind = !collections.length
    ? 'no-collections'
    : !scopeCount
      ? 'empty-collection'
      : !matches
        ? 'no-matches'
        : 'results';
  return { kind, scopeCount } as const;
}

export const cuttingRoleLabels: Record<CuttingRole, string> = {
  rough: 'Main / roughing',
  finish: 'Finishing',
  drill: 'Drilling',
  spot: 'Spot drilling',
  chamfer: 'Hole chamfer',
};

export function cuttingRoleHint(kind: string, role?: CuttingRole): string | null {
  if (kind !== 'profile') return null;
  return role === 'finish'
    ? 'These values control the Profile contour.'
    : 'Stored main values; the Profile contour uses Finishing.';
}

/** Group identical backend messages only; a generic warning cannot identify a missing limit. */
export function cuttingWarningGroups(warnings: string[]) {
  const groups = new Map<string, { message: string; roles: string[] }>();
  for (const warning of warnings) {
    const match = /^(Rough|Finish|Drill|Spot|Chamfer): (.+)$/.exec(warning);
    const message = match?.[2] ?? warning;
    const key = `${match ? 'role' : 'raw'}:${message}`;
    const group = groups.get(key) ?? { message, roles: [] };
    if (match && !group.roles.includes(match[1])) group.roles.push(match[1]);
    groups.set(key, group);
  }
  return [...groups.values()];
}
