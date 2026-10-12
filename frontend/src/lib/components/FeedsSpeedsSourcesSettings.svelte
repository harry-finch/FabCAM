<script lang="ts">
  import { onMount } from 'svelte';
  onMount(() => {
    void recommendationSources.refresh(true);
  });
  import LabPresetManager from './LabPresetManager.svelte';
  import { currentLocalPacks, LAB_PACK_ID } from '../state/lab-presets';
  import { project } from '../state/project.svelte';
  import {
    packReference,
    type SourceSettings,
    type SourcePack,
  } from '../state/feeds-speeds-sources';
  import { importLocalPack, MAX_PACK_BYTES } from '../state/feeds-speeds-pack-store';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { t } from '../i18n';
  function evidenceSummary(pack: SourcePack) {
    return [...new Set(pack.dataset.profile_set.profiles.map((p) => p.evidence))]
      .map((value) =>
        t(
          value === 'reference_data'
            ? 'fs35.reference'
            : value === 'tested'
              ? 'fs35.tested'
              : value === 'validated'
                ? 'fs35.validated'
                : 'fs35.theoretical',
        ),
      )
      .join(' / ');
  }
  let error = $state('');
  let importing = $state(false);
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  function update(next: SourceSettings) {
    project.updateSettings({ feedsSpeedsSources: next });
    void recommendationSources.refresh();
  }
  const settings = $derived(project.data.settings.feedsSpeedsSources);
  function bundledEnabled(id: string) {
    return (
      settings.bundled ??
      (settings.sienciEnabled
        ? recommendationSources.builtins.map((p) => p.manifest.identity.id)
        : [])
    ).includes(id);
  }
</script>

<h3>{t('fs35.sources')}</h3>
<p>{t('fs37.sources_help')}</p>
<h4>{t('fs37.built_in')}</h4>
{#each recommendationSources.builtins as pack (pack.manifest.identity.id)}
  <label
    ><input
      type="checkbox"
      checked={bundledEnabled(pack.manifest.identity.id)}
      onchange={(e) => {
        const ids =
          settings.bundled ??
          (settings.sienciEnabled
            ? recommendationSources.builtins.map((p) => p.manifest.identity.id)
            : []);
        update({
          ...settings,
          bundled: e.currentTarget.checked
            ? [...ids, pack.manifest.identity.id]
            : ids.filter((id) => id !== pack.manifest.identity.id),
        });
      }}
    />{pack.manifest.name}</label
  >
  <p class="meta">{t('fs37.public')} · {evidenceSummary(pack)} · {pack.manifest.license}</p>
  <details>
    <summary>{t('fs.source')}</summary>
    <p>{pack.manifest.attribution}</p>
    <p>{pack.manifest.notes}</p>
  </details>
{/each}
<h4>{t('fs37.local')}</h4>
{#each currentLocalPacks(recommendationSources.local) as pack (pack.manifest.identity.id === LAB_PACK_ID ? LAB_PACK_ID : pack.manifest.identity.id + '@' + pack.manifest.identity.revision)}
  <label
    ><input
      type="checkbox"
      checked={settings.local?.some(
        (r) =>
          r.id === pack.manifest.identity.id &&
          r.version === pack.manifest.identity.revision &&
          r.enabled,
      ) ?? false}
      onchange={(e) =>
        update({
          ...settings,
          local: [
            ...(settings.local ?? []).filter(
              (r) =>
                r.id !== pack.manifest.identity.id || r.version !== pack.manifest.identity.revision,
            ),
            packReference(pack, e.currentTarget.checked),
          ],
        })}
    />{pack.manifest.name}</label
  >
  <p class="meta">
    {t('fs37.local_only')} · {evidenceSummary(pack)} · {pack.manifest.identity.revision} · {pack
      .manifest.license}
  </p>
  <details>
    <summary>{t('fs.source')}</summary>
    <p>{pack.manifest.attribution}</p>
    <p>{pack.manifest.notes}</p>
  </details>
  {#if pack.manifest.identity.id === LAB_PACK_ID}<LabPresetManager {pack} />{/if}
{/each}
{#if !recommendationSources.local.length}<p class="meta">{t('fs37.no_local')}</p>{/if}
<label class="import"
  >{t('fs37.import')}<input
    type="file"
    disabled={importing}
    accept=".json,application/json"
    onchange={async (e) => {
      const input = e.currentTarget;
      const file = input.files?.[0];
      if (!file) return;
      importing = true;
      try {
        if (file.size > MAX_PACK_BYTES) throw new Error(t('fs37.too_large'));
        const pack = await importLocalPack(await file.text());
        const next = {
          ...settings,
          local: [
            ...(settings.local ?? []).filter(
              (r) => pack.manifest.identity.id !== LAB_PACK_ID || r.id !== LAB_PACK_ID,
            ),
            packReference(pack),
          ],
        };
        project.updateSettings({ feedsSpeedsSources: next });
        await recommendationSources.refresh(true);
        error = '';
      } catch (err) {
        error = `${t('fs.invalid_data')} ${String(err)}`;
      } finally {
        importing = false;
        input.value = '';
      }
    }}
  /></label
>
<p class="meta">{t('fs37.privacy')}</p>
{#if recommendationSources.loading}<p role="status">{t('fs35.loading')}</p>{/if}
{#if error || recommendationSources.error}<p role="alert">
    {error || recommendationSources.error}
  </p>{/if}
<details>
  <summary>{t('fs35.source_details')}</summary>
  <p>{t('fs35.source_scope_help')}</p>
  <a
    href={`${import.meta.env.BASE_URL}feeds-speeds/CC-BY-SA-4.0.txt`}
    target="_blank"
    rel="noreferrer">{t('fs.reference_license')}</a
  >
</details>

<style>
  label {
    display: flex;
    align-items: center;
    gap: var(--ui-space-2);
    margin: var(--ui-space-2) 0;
    font-size: var(--inspector-font-field);
  }
  h4 {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
    text-transform: uppercase;
    margin: var(--ui-space-3) 0 var(--ui-space-1);
  }
  p {
    font-size: var(--inspector-font-field);
    max-width: 45rem;
    overflow-wrap: anywhere;
  }
  .meta,
  summary {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .import {
    display: block;
  }
  input[type='file'] {
    display: block;
    margin-top: var(--ui-space-1);
  }
  summary {
    cursor: pointer;
    min-height: 24px;
  }
</style>
