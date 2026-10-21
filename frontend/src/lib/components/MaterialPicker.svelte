<script lang="ts">
  import { project } from '../state/project.svelte';
  import type { MaterialIdentity } from '../state/project-types';
  let {
    value,
    onchange,
    context = 'job',
  }: {
    value?: MaterialIdentity;
    context?: 'job' | 'preset';
    onchange: (value: MaterialIdentity | undefined) => void;
  } = $props();
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { NORMALIZED_MATERIALS } from '../state/feeds-speeds-sources';
  import { t } from '../i18n';
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  const materials = $derived([
    ...new Map(
      [
        ...NORMALIZED_MATERIALS,
        ...recommendationSources.rows.flatMap((s) => s.catalog.materials),
      ].map((m) => [JSON.stringify(m.identity), m]),
    ).values(),
  ]);
  let custom = $state(false);
  const selected = $derived(
    JSON.stringify(value ? { id: value.id, revision: value.revision } : null),
  );
</script>

<label class="material"
  ><span>{t('fs.material')}</span><select
    value={selected}
    onchange={(e) => {
      if (e.currentTarget.value === 'custom') {
        custom = true;
        return;
      }
      custom = false;
      onchange(
        materials.find((m) => JSON.stringify(m.identity) === e.currentTarget.value)
          ? {
              ...JSON.parse(e.currentTarget.value),
              label: materials.find((m) => JSON.stringify(m.identity) === e.currentTarget.value)!
                .label,
            }
          : undefined,
      );
    }}
    ><option value="null"
      >{t(context === 'job' ? 'fs35.select_material' : 'preset2.choose_material')}</option
    >{#each materials as item (JSON.stringify(item.identity))}<option
        value={JSON.stringify(item.identity)}>{item.label}</option
      >{/each}{#if value && !materials.some((m) => JSON.stringify(m.identity) === selected)}<option
        value={selected}>{value.id}</option
      >{/if}<option value="custom">{t('fs35.other_material')}</option></select
  ></label
>
{#if custom}<label class="material"
    ><span>{t('fs35.material_name')}</span><input
      type="text"
      value={value?.id ?? ''}
      onchange={(e) => {
        const name = e.currentTarget.value.trim();
        if (name) onchange({ id: name, revision: 'job-1', label: name });
      }}
    /></label
  >
  <p>{t('fs35.custom_material_help')}</p>{/if}
<p>{t(context === 'job' ? 'fs35.job_material' : 'preset2.material_help')}</p>

<style>
  .material {
    display: grid;
    grid-template-columns: 6rem 1fr;
    gap: 0.5rem;
    margin: 0.5rem 0;
    font-size: 0.85rem;
  }
  select {
    width: 100%;
    min-width: 0;
  }
  p {
    font-size: 0.75rem;
    color: var(--text-muted);
  }
</style>
