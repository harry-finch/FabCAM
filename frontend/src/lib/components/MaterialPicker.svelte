<script lang="ts">
  import './op_properties/op-properties.css';
  import { tick } from 'svelte';
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
  import {
    materialOptions,
    materialIdentityKey,
    materialDisplay,
  } from '../state/material-presentation';
  import MaterialBrowser from './MaterialBrowser.svelte';
  import { t } from '../i18n';
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  const materials = $derived(materialOptions(recommendationSources.rows, value));
  let browsing = $state(false);
  let trigger: HTMLButtonElement;
  async function closeBrowser() {
    browsing = false;
    await tick();
    trigger?.focus();
  }
  let custom = $state(false);
  const selected = $derived(
    value ? materials.find((m) => m.key === materialIdentityKey(value)) : undefined,
  );
</script>

<div class="props">
  <div class="material">
    <span>{t('fs.material')}</span>
    <button
      type="button"
      class="inspector-action"
      bind:this={trigger}
      aria-expanded={browsing}
      onclick={() => (browsing = !browsing)}
      title={value ? `${value.id} @${value.revision}` : undefined}
    >
      {value
        ? materialDisplay(value, value.label || selected?.labels[0] || value.id, t)
        : t('mat.choose')}
    </button>
  </div>
  {#if browsing}
    <MaterialBrowser
      options={materials}
      onclose={closeBrowser}
      selected={value}
      onselect={(item) => {
        onchange({ ...item.identity, label: item.labels[0] });
        custom = false;
        void closeBrowser();
      }}
    />
    <div class="actions">
      <button
        type="button"
        class="inspector-action"
        onclick={() => {
          onchange(undefined);
          browsing = false;
        }}>{t('cad1.clear')}</button
      >
      <button
        type="button"
        class="inspector-action"
        onclick={() => {
          custom = true;
          browsing = false;
        }}>{t('fs35.other_material')}</button
      >
    </div>
  {/if}
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
</div>

<style>
  .material {
    display: grid;
    grid-template-columns: 6rem 1fr;
    gap: 0.5rem;
    margin: 0.5rem 0;
    font-size: 0.85rem;
  }
  button {
    width: 100%;
    min-width: 0;
    white-space: normal;
    overflow-wrap: anywhere;
  }
  .actions {
    display: flex;
    gap: var(--ui-space-2);
    margin-top: var(--ui-space-2);
  }
  p {
    font-size: 0.75rem;
    color: var(--text-muted);
  }
</style>
