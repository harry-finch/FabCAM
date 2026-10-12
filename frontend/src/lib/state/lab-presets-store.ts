import { project } from './project.svelte';
import { recommendationSources } from './feeds-speeds-sources.svelte';
import { commitLabPack } from './feeds-speeds-pack-store';
import { LAB_PACK_ID } from './lab-presets';
import { packReference, type SourcePack } from './feeds-speeds-sources';
export async function persistLabPack(pack: SourcePack, expectedRevision: string | null) {
  const saved = await commitLabPack(pack, expectedRevision);
  const settings = project.data.settings.feedsSpeedsSources;
  const enabled = settings.local?.find((r) => r.id === LAB_PACK_ID)?.enabled ?? true;
  project.updateSettings({
    feedsSpeedsSources: {
      ...settings,
      local: [
        ...(settings.local ?? []).filter((r) => r.id !== LAB_PACK_ID),
        packReference(saved, enabled),
      ],
    },
  });
  await recommendationSources.refresh(true);
}
