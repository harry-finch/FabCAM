/** Read-only source-record projection. Never resolves, calculates or applies cutting values. */
import type { components } from '../api/generated';
import type { SourcePack } from './feeds-speeds-sources';
import { materialPresentation, type MaterialFamily } from './material-presentation';
import { currentLocalPacks } from './lab-presets';
type S = components['schemas'];
export type PresetRow = {
  key: string;
  pack: SourcePack;
  profile: S['FsProfile'];
  record?: S['FsRecordOrigin'];
  enabled: boolean;
  milling: boolean;
  name: string;
  material: string;
  machine: string;
  tool: string;
  materialFamilies: string[];
  displayFamily: MaterialFamily;
  state: string;
  search: string;
};
export function identityLabel(identity: S['FsIdentity'], entries: S['CatalogEntry'][]): string {
  return (
    entries.find((e) => e.identity.id === identity.id && e.identity.revision === identity.revision)
      ?.label ?? `${identity.id} @${identity.revision}`
  );
}
function selectorLabel(selector: S['FsIdentitySelector'], entries: S['CatalogEntry'][]): string {
  return selector.kind === 'exact'
    ? identityLabel(selector.identity, entries)
    : selector.kind === 'family'
      ? selector.id
      : '*';
}
export function presetRows(
  packs: SourcePack[],
  enabled: ReadonlySet<string>,
  milling: boolean,
): PresetRow[] {
  return currentLocalPacks(packs).flatMap((pack) =>
    pack.dataset.profile_set.profiles.map((profile) => {
      const record = pack.records.find(
        (r) =>
          r.profile.id === profile.identity.id && r.profile.revision === profile.identity.revision,
      );
      const identity =
        profile.scope.material.kind === 'exact' ? profile.scope.material.identity : undefined;
      const presentation = identity ? materialPresentation(identity) : undefined;
      const material = selectorLabel(profile.scope.material, pack.dataset.materials);
      const materialFamilies =
        profile.scope.material.kind === 'family'
          ? [profile.scope.material.id]
          : profile.scope.material.kind === 'exact'
            ? pack.dataset.profile_set.material_families
                .filter((f) =>
                  f.members.some(
                    (m) =>
                      profile.scope.material.kind === 'exact' &&
                      m.id === profile.scope.material.identity.id &&
                      m.revision === profile.scope.material.identity.revision,
                  ),
                )
                .map((f) => f.id)
            : [];
      // Only recorded membership/taxonomy is shown. No alias, family or cutting equivalence is inferred.
      if (record?.material_taxon) materialFamilies.push(record.material_taxon);
      const row = {
        key: JSON.stringify([
          pack.manifest.identity.id,
          pack.manifest.identity.revision,
          profile.identity.id,
          profile.identity.revision,
        ]),
        pack,
        profile,
        record,
        enabled: enabled.has(`${pack.manifest.identity.id}@${pack.manifest.identity.revision}`),
        milling,
        name: record?.lab?.name ?? record?.original_classification ?? profile.identity.id,
        material,
        machine: selectorLabel(profile.scope.machine, pack.dataset.machines),
        tool: profile.scope.tool
          ? identityLabel(profile.scope.tool, pack.dataset.tools)
          : profile.scope.cutting_family,
        materialFamilies: [...new Set(materialFamilies)],
        displayFamily: presentation?.family ?? 'unknown',
        state: record?.lab?.deleted ? 'deleted' : (record?.lab?.state ?? profile.evidence),
      };
      return {
        ...row,
        search: JSON.stringify([
          row.name,
          row.material,
          presentation?.aliases,
          row.machine,
          row.tool,
          `${profile.scope.flute_count ?? ''} flutes`,
          profile.scope,
          profile.source,
          record,
          pack.manifest.name,
          pack.manifest.identity,
        ])
          .normalize('NFD')
          .replace(/[\u0300-\u036f]/g, '')
          .toLowerCase(),
      };
    }),
  );
}
export type PresetFilters = {
  query: string;
  pack: string;
  materialFamily: string;
  toolFamily: string;
  state: string;
  availability: 'enabled' | 'all' | 'disabled';
};
export const emptyPresetFilters = (): PresetFilters => ({
  query: '',
  pack: '',
  materialFamily: '',
  toolFamily: '',
  state: '',
  availability: 'enabled',
});
export function filterPresets(rows: PresetRow[], f: PresetFilters): PresetRow[] {
  const words = f.query
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .trim()
    .split(/\s+/)
    .filter(Boolean);
  return rows.filter(
    (r) =>
      words.every((w) => r.search.includes(w)) &&
      (!f.pack ||
        `${r.pack.manifest.identity.id}@${r.pack.manifest.identity.revision}` === f.pack) &&
      (!f.materialFamily ||
        (f.materialFamily.startsWith('display:')
          ? r.displayFamily === f.materialFamily.slice(8)
          : f.materialFamily === '__unspecified'
            ? !r.materialFamilies.length
            : r.materialFamilies.includes(f.materialFamily))) &&
      (!f.toolFamily || r.profile.scope.cutting_family === f.toolFamily) &&
      (!f.state || r.state === f.state) &&
      (f.availability === 'all' ||
        (f.availability === 'disabled' ? !r.enabled : r.enabled && r.milling)),
  );
}
export function rangeText(value?: S['FsRange'] | null): string {
  return value == null
    ? '—'
    : value.min === value.max
      ? String(value.min)
      : `${value.min}–${value.max}`;
}
