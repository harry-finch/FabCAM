import { project } from './project.svelte';
import { recommendationSources } from './feeds-speeds-sources.svelte';
import { commitLabPack } from './feeds-speeds-pack-store';
import { packReference, type SourcePack } from './feeds-speeds-sources';
export async function persistLabPack(pack: SourcePack, expectedRevision: string | null) {
  const saved = await commitLabPack(pack, expectedRevision);
  const settings = project.data.settings.feedsSpeedsSources;
  const id = saved.manifest.identity.id;
  const enabled = settings.local?.find((r) => r.id === id)?.enabled ?? true;
  project.updateSettings({
    feedsSpeedsSources: {
      ...settings,
      local: [...(settings.local ?? []).filter((r) => r.id !== id), packReference(saved, enabled)],
    },
  });
  await recommendationSources.refresh(true);
}
