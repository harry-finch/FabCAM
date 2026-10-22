<script lang="ts">
  import './op_properties/op-properties.css';
  import { tick } from 'svelte';
  import type { MaterialIdentity } from '../state/project-types';
  import { listCollections, validateCollection } from '../state/preset-collections';
  import { collectionMaterials } from '../state/preset-library-v9';
  import type { MaterialSource } from '../state/material-presentation';
  import { materialOptions, materialIdentityKey } from '../state/material-presentation';
  import { materialPickerLabel } from '../state/material-picker-presentation';
  import MaterialBrowser from './MaterialBrowser.svelte';
  import AnchoredPopover from './AnchoredPopover.svelte';
  import { t } from '../i18n';
  let {
    value,
    onchange,
    context = 'job',
  }: {
    value?: MaterialIdentity;
    context?: 'job' | 'preset';
    onchange: (value: MaterialIdentity | undefined) => void;
  } = $props();
  const id = $props.id();
  let browsing = $state(false),
    custom = $state(false),
    customName = $state('');
  let trigger = $state<HTMLButtonElement>();
  let sources = $state<MaterialSource[]>([]);
  let sourceError = $state('');
  $effect(() => {
    if (!browsing) return;
    let cancelled = false;
    void listCollections()
      .then((rows) => Promise.all(rows.map((r) => validateCollection(r.json))))
      .then((rows) => {
        if (!cancelled) {
          sources = collectionMaterials(rows);
          sourceError = '';
        }
      })
      .catch((e) => {
        if (!cancelled) sourceError = String(e);
      });
    return () => {
      cancelled = true;
    };
  });
  const materials = $derived(materialOptions(sources, value));
  const selected = $derived(
    value ? materials.find((m) => m.key === materialIdentityKey(value)) : undefined,
  );
  const label = $derived(
    value
      ? materialPickerLabel(
          value,
          (value.label && value.label !== value.id
            ? value.label
            : selected?.labels.find((label) => label !== value.id)) || value.id,
          t,
        )
      : t('mat.choose'),
  );
  async function closeBrowser(restoreFocus = true) {
    browsing = false;
    await tick();
    if (restoreFocus && trigger?.isConnected) trigger.focus();
  }
</script>

<div class="props">
  <div class="material">
    <span id={`${id}-label`}>{t('fs.material')}</span>
    <button
      type="button"
      class="inspector-action trigger"
      bind:this={trigger}
      aria-labelledby={`${id}-label ${id}-value`}
      aria-haspopup="dialog"
      aria-controls={browsing ? `${id}-popup` : undefined}
      aria-expanded={browsing}
      onclick={() => {
        if (browsing) void closeBrowser();
        else {
          custom = false;
          browsing = true;
        }
      }}
      title={[
        label,
        value ? `${value.id} @${value.revision}` : '',
        t(context === 'job' ? 'fs35.job_material' : 'preset2.material_help'),
      ]
        .filter(Boolean)
        .join('\n')}
      ><span id={`${id}-value`}>{label}</span><span aria-hidden="true">▾</span></button
    >
  </div>
  {#if browsing && trigger}
    <AnchoredPopover
      anchor={trigger}
      id={`${id}-popup`}
      label={t('fs.material')}
      onClose={closeBrowser}
    >
      {#if sourceError}<p role="status">Collection materials unavailable: {sourceError}</p>{/if}
      {#if custom}
        <form
          class="custom-form"
          onsubmit={(event) => {
            event.preventDefault();
            const name = customName.trim();
            if (!name) return;
            onchange({ id: name, revision: 'job-1', label: name });
            void closeBrowser();
          }}
        >
          <label>{t('fs35.material_name')}<input type="text" bind:value={customName} /></label>
          <p>{t('fs35.custom_material_help')}</p>
          <button type="submit" class="inspector-action" disabled={!customName.trim()}
            >{t('mat.use_custom')}</button
          >
        </form>
      {:else}
        <MaterialBrowser
          options={materials}
          selected={value}
          selectedLabel={value ? label : undefined}
          onclose={closeBrowser}
          onselect={(item) => {
            onchange({ ...item.identity, label: item.labels[0] });
            void closeBrowser();
          }}
        />
      {/if}
      <p class="scope" title={t('mat.help')}>
        {t(context === 'job' ? 'mat.job_scope' : 'preset2.material_help')}
      </p>
      <div class="actions">
        {#if custom}<button type="button" class="inspector-action" onclick={() => (custom = false)}
            >{t('mat.back')}</button
          >
        {:else}
          <button
            type="button"
            class="inspector-action"
            onclick={() => {
              onchange(undefined);
              void closeBrowser();
            }}>{t('cad1.clear')}</button
          >
          <button
            type="button"
            class="inspector-action"
            onclick={async () => {
              customName = value?.id ?? '';
              custom = true;
              await tick();
              trigger?.ownerDocument.getElementById(`${id}-popup`)?.querySelector('input')?.focus();
            }}>{t('fs35.other_material')}</button
          >
        {/if}
        <button type="button" class="inspector-action close" onclick={() => void closeBrowser()}
          >{t('common.cancel')}</button
        >
      </div>
    </AnchoredPopover>
  {/if}
</div>

<style>
  .material {
    display: grid;
    grid-template-columns: 6rem minmax(0, 1fr);
    gap: var(--ui-space-2);
    align-items: center;
    margin: var(--ui-space-1) 0;
    font-size: var(--inspector-font-field);
  }
  .trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--ui-space-1);
    width: 100%;
    min-width: 0;
    text-align: left;
  }
  .trigger span:first-child {
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: var(--ui-space-1);
    flex: none;
  }
  .actions button {
    min-width: 0;
  }
  .close {
    margin-left: auto;
  }
  p {
    margin: 0;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
    line-height: 1.35;
  }
  .scope {
    flex: none;
  }
  .custom-form {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-2);
    min-height: 0;
    overflow: auto;
  }
  .custom-form label {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-1);
    font-size: var(--inspector-font-meta);
  }
  .custom-form input {
    width: 100%;
    min-width: 0;
  }
</style>
