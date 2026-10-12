import { newestLabPack, LAB_PACK_ID } from './lab-presets';
import { project } from './project.svelte';
import {
  enabledPacks,
  legacyLocalPack,
  packReference,
  type LoadedSource,
  type SourcePack,
  type PackRegistry,
} from './feeds-speeds-sources';
import { readLocalPacks, importLocalPack, validatePack } from './feeds-speeds-pack-store';
class RecommendationSources {
  rows = $state<LoadedSource[]>([]);
  builtins = $state<SourcePack[]>([]);
  local = $state<SourcePack[]>([]);
  error = $state('');
  loading = $state(false);
  private key = '';
  private built: Promise<SourcePack[]> | undefined;
  async refresh(force = false): Promise<void> {
    const settings = project.data.settings.feedsSpeedsSources;
    const key = JSON.stringify(settings);
    if (!force && key === this.key) return;
    this.key = key;
    this.loading = true;
    this.error = '';
    this.rows = [];
    try {
      this.built ??= (async () => {
        const response = await fetch(`${import.meta.env.BASE_URL}feeds-speeds/registry.v1.json`);
        if (!response.ok) throw new Error('Bundled source registry unavailable');
        const registry = (await response.json()) as PackRegistry;
        if (registry.schema_version !== 1 || registry.packs.length > 32)
          throw new Error('Unsupported source registry');
        const ids = new Set<string>();
        return Promise.all(
          registry.packs.map(async (row) => {
            if (ids.has(row.id) || !/^packs\/[a-zA-Z0-9_.-]+\.json$/.test(row.path))
              throw new Error('Invalid bundled source entry');
            ids.add(row.id);
            const response = await fetch(`${import.meta.env.BASE_URL}feeds-speeds/${row.path}`);
            if (!response.ok) throw new Error('Bundled source unavailable');
            const pack = await validatePack(await response.text());
            const m = pack.manifest;
            if (
              m.identity.id !== row.id ||
              m.visibility !== 'public' ||
              m.distribution !== 'bundled' ||
              !m.redistributable ||
              !m.transformation_permitted
            )
              throw new Error('Invalid public source distribution');
            return pack;
          }),
        );
      })();
      const builtins = await this.built;
      let local = (await readLocalPacks()).map((json) => JSON.parse(json) as SourcePack);
      // Migrate legacy bodies only after each validated app-local write succeeds.
      if (settings.custom.length) {
        const refs = [...(settings.local ?? [])];
        for (const source of settings.custom) {
          const legacy = legacyLocalPack(source);
          const installed =
            local.find(
              (p) =>
                p.manifest.identity.id === legacy.manifest.identity.id &&
                p.manifest.identity.revision === legacy.manifest.identity.revision,
            ) ?? (await importLocalPack(JSON.stringify(legacy)));
          refs.push(packReference(installed, source.enabled));
        }
        project.updateSettings({ feedsSpeedsSources: { ...settings, custom: [], local: refs } });
        this.key = '';
        return this.refresh();
      }
      local = await Promise.all(local.map((p) => validatePack(JSON.stringify(p))));
      const head = newestLabPack(local);
      const labRef = settings.local?.find((r) => r.id === LAB_PACK_ID);
      if (head && labRef && labRef.version !== head.manifest.identity.revision) {
        project.updateSettings({
          feedsSpeedsSources: {
            ...settings,
            local: [
              ...(settings.local ?? []).filter((r) => r.id !== LAB_PACK_ID),
              packReference(head, labRef.enabled),
            ],
          },
        });
        this.key = '';
        return this.refresh();
      }
      if (key === this.key) {
        this.builtins = builtins;
        this.local = local;
        this.rows = enabledPacks(settings, builtins, local);
      }
    } catch (e) {
      if (key === this.key) {
        this.error = String(e);
        this.key = '';
        this.built = undefined;
      }
    } finally {
      if (key === this.key || this.key === '') this.loading = false;
    }
  }
}
export const recommendationSources = new RecommendationSources();
