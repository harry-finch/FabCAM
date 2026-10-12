<script lang="ts">
  import { project } from '../state/project.svelte';
  import { setMaterialCommand } from '../state/commands';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { t } from '../i18n';
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  const materials = $derived([
    ...new Map(
      recommendationSources.rows
        .flatMap((s) => s.catalog.materials)
        .map((m) => [JSON.stringify(m.identity), m]),
    ).values(),
  ]);
  let custom = $state(false);
  const selected = $derived(
    JSON.stringify(
      project.data.material
        ? { id: project.data.material.id, revision: project.data.material.revision }
        : null,
    ),
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
      project.history.exec(
        setMaterialCommand(
          materials.find((m) => JSON.stringify(m.identity) === e.currentTarget.value)
            ? {
                ...JSON.parse(e.currentTarget.value),
                label: materials.find((m) => JSON.stringify(m.identity) === e.currentTarget.value)!
                  .label,
              }
            : undefined,
        ),
        project.target(),
      );
    }}
    ><option value="null">{t('fs35.select_material')}</option
    >{#each materials as item (JSON.stringify(item.identity))}<option
        value={JSON.stringify(item.identity)}>{item.label}</option
      >{/each}{#if project.data.material && !materials.some((m) => JSON.stringify(m.identity) === selected)}<option
        value={selected}>{project.data.material.id}</option
      >{/if}<option value="custom">{t('fs35.other_material')}</option></select
  ></label
>
{#if custom}<label class="material"
    ><span>{t('fs35.material_name')}</span><input
      type="text"
      value={project.data.material?.id ?? ''}
      onchange={(e) => {
        const name = e.currentTarget.value.trim();
        if (name)
          project.history.exec(
            setMaterialCommand({ id: name, revision: 'job-1', label: name }),
            project.target(),
          );
      }}
    /></label
  >
  <p>{t('fs35.custom_material_help')}</p>{/if}
<p>{t('fs35.job_material')}</p>

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
