import { parseDatasetCatalog, type DatasetCatalog } from './feeds-speeds';
export type RecommendationSource = {
  id: string;
  label: string;
  enabled: boolean;
  contents: string;
};
export type SourceSettings = { sienciEnabled: boolean; custom: RecommendationSource[] };
export type LoadedSource = { id: string; label: string; contents: string; catalog: DatasetCatalog };
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
export async function loadConfiguredSources(
  settings: SourceSettings,
  readSienci: () => Promise<string>,
): Promise<LoadedSource[]> {
  const rows: RecommendationSource[] = settings.custom.filter((s) => s.enabled);
  if (settings.sienciEnabled)
    rows.unshift({ id: 'sienci', label: 'Sienci', enabled: true, contents: await readSienci() });
  return rows.map((s) => ({ ...s, catalog: parseDatasetCatalog(s.contents) }));
}
