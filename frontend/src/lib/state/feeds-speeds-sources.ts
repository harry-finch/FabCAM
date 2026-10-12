import { parseDatasetCatalog, type DatasetCatalog } from './feeds-speeds';
import type { components } from '../api/generated';
export type SourcePack = components['schemas']['FsSourcePack'];
// Legacy preference body, retained only until migration succeeds.
export type RecommendationSource = {
  id: string;
  label: string;
  enabled: boolean;
  contents: string;
};
export type PackReference = { id: string; version: string; enabled: boolean };
export type SourceSettings = {
  sienciEnabled: boolean;
  custom: RecommendationSource[];
  bundled?: string[];
  local?: PackReference[];
};
export type LoadedSource = {
  id: string;
  label: string;
  contents: string;
  catalog: DatasetCatalog;
  pack?: SourcePack;
};
export type PackRegistry = { schema_version: number; packs: { id: string; path: string }[] };
export const DEFAULT_SOURCE_SETTINGS: SourceSettings = { sienciEnabled: true, custom: [] };
export function normalizeSourceSettings(value: unknown): SourceSettings {
  const v = value as Partial<SourceSettings> | null;
  return {
    sienciEnabled: typeof v?.sienciEnabled === 'boolean' ? v.sienciEnabled : true,
    custom: Array.isArray(v?.custom)
      ? v.custom.filter(
          (s) =>
            typeof s?.id === 'string' &&
            typeof s.label === 'string' &&
            typeof s.contents === 'string' &&
            typeof s.enabled === 'boolean',
        )
      : [],
    ...(Array.isArray(v?.bundled)
      ? { bundled: v.bundled.filter((id): id is string => typeof id === 'string') }
      : {}),
    ...(Array.isArray(v?.local)
      ? {
          local: v.local.filter(
            (s) =>
              typeof s?.id === 'string' &&
              typeof s.version === 'string' &&
              typeof s.enabled === 'boolean',
          ),
        }
      : {}),
  };
}
export function importedSource(contents: string): RecommendationSource {
  parseDatasetCatalog(contents);
  const v = JSON.parse(contents);
  const identity = v.profile_set?.identity;
  if (!identity?.id || !identity.revision) throw new Error('Missing source identity');
  return {
    id: `${identity.id}@${identity.revision}`,
    label: v.description,
    enabled: true,
    contents,
  };
}
// Compatibility helper for legacy tests/clients; production uses the generic registry below.
export async function loadConfiguredSources(
  settings: SourceSettings,
  readSienci: () => Promise<string>,
): Promise<LoadedSource[]> {
  const rows = settings.custom.filter((s) => s.enabled);
  if (settings.sienciEnabled)
    rows.unshift({ id: 'sienci', label: 'Sienci', enabled: true, contents: await readSienci() });
  return rows.map((s) => ({ ...s, catalog: parseDatasetCatalog(s.contents) }));
}
export function packReference(pack: SourcePack, enabled = true): PackReference {
  return { id: pack.manifest.identity.id, version: pack.manifest.identity.revision, enabled };
}
export function loadedPack(pack: SourcePack): LoadedSource {
  return {
    id: `${pack.manifest.identity.id}@${pack.manifest.identity.revision}`,
    label: pack.manifest.name,
    contents: JSON.stringify(pack),
    catalog: parseDatasetCatalog(JSON.stringify(pack.dataset)),
    pack,
  };
}
export function enabledPacks(
  settings: SourceSettings,
  builtins: SourcePack[],
  local: SourcePack[],
): LoadedSource[] {
  const ids =
    settings.bundled ?? (settings.sienciEnabled ? builtins.map((p) => p.manifest.identity.id) : []);
  const selected = [
    ...builtins.filter((p) => ids.includes(p.manifest.identity.id)),
    ...local.filter((p) =>
      settings.local?.some(
        (s) =>
          s.id === p.manifest.identity.id &&
          s.version === p.manifest.identity.revision &&
          s.enabled,
      ),
    ),
  ];
  const seen = new Set<string>();
  for (const p of selected) {
    if (seen.has(p.manifest.identity.id))
      throw new Error('Enable only one version of a source pack');
    seen.add(p.manifest.identity.id);
  }
  return selected.map(loadedPack);
}
export function legacyLocalPack(source: RecommendationSource): SourcePack {
  const d = JSON.parse(source.contents);
  return {
    manifest: {
      identity: d.profile_set.identity,
      schema_version: 1,
      name: source.label,
      publisher: 'Imported legacy source',
      source_url: null,
      origin: 'Installation preference migration; original record provenance preserved',
      license: d.license,
      license_url: null,
      attribution: 'See individual original record sources',
      visibility: 'private',
      distribution: 'local_only',
      redistributable: false,
      transformation_permitted: false,
      default_evidence: 'reference_data',
      notes: 'Local-only distribution does not establish third-party usage rights',
    },
    dataset: d,
    records: d.profile_set.profiles.map(
      (p: { identity: { id: string; revision: string }; source: { notes: string } }) => ({
        profile: p.identity,
        source_record_id: p.identity.id,
        original_classification: p.source.notes || p.identity.id,
        material_taxon: null,
        tool: null,
      }),
    ),
  };
}
export function packCollection(rows: LoadedSource[]): string {
  return JSON.stringify({ source_packs: rows.map((s) => s.pack) });
}

export const NORMALIZED_MATERIALS = [
  'wood',
  'wood.mdf',
  'wood.plywood',
  'wood.plywood.poplar',
  'wood.plywood.birch',
  'wood.hardwood',
  'wood.softwood',
  'plastic',
  'plastic.pmma',
  'plastic.pmma.cast',
  'plastic.pmma.extruded',
  'plastic.pom',
  'plastic.hdpe',
  'plastic.polycarbonate',
  'plastic.pvc',
  'metal',
  'metal.aluminium',
  'metal.aluminium.6061',
  'metal.aluminium.7075',
].map((id) => ({
  identity: { id: `material/${id}`, revision: '1' },
  label: id
    .split('.')
    .map((s) => s.toUpperCase())
    .join(' / '),
}));
