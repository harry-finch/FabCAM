<script lang="ts">
  import { onMount } from 'svelte';
  onMount(() => {
    void recommendationSources.refresh(true);
  });
  import LabPackCreate from './LabPackCreate.svelte';
  import LabPresetManager from './LabPresetManager.svelte';
  import { currentLocalPacks, isLabPackId } from '../state/lab-presets';
  import { project } from '../state/project.svelte';
  import {
    packReference,
    type SourceSettings,
    type SourcePack,
  } from '../state/feeds-speeds-sources';
  import {
    removeLocalSource,
    importLocalPack,
    MAX_PACK_BYTES,
  } from '../state/feeds-speeds-pack-store';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { t } from '../i18n';
  import { confirmStore } from '../state/confirm.svelte';
  function evidenceSummary(pack: SourcePack) {
    return (
      [...new Set(pack.dataset.profile_set.profiles.map((p) => p.evidence))]
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
        .join(' / ') || t('v11.pack_empty')
    );
  }
  let error = $state('');
  let importing = $state(false);
  let removing = $state(false);
  async function removeSource(pack: SourcePack) {
    const id = pack.manifest.identity.id;
    const head = recommendationSources.local
      .filter((p) => p.manifest.identity.id === id)
      .map((p) => p.manifest.identity.revision)
      .sort()
      .at(-1);
    if (!head || removing) return;
    const accepted = await confirmStore.ask({
      title: t('v11.source_remove'),
      body: t('v11.source_remove_help', { name: pack.manifest.name }),
      primaryLabel: t('v11.source_remove'),
      cancelLabel: t('common.cancel'),
      danger: true,
    });
    if (!accepted) return;
    removing = true;
    try {
      await removeLocalSource(id, head);
      const latest = project.data.settings.feedsSpeedsSources;
      project.updateSettings({
        feedsSpeedsSources: { ...latest, local: (latest.local ?? []).filter((r) => r.id !== id) },
      });
      error = '';
    } catch (e) {
      error = String(e);
    } finally {
      await recommendationSources.refresh(true);
      removing = false;
    }
  }
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
<p class="meta">{t('v11.source_bundled')}</p>
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
<LabPackCreate />
{#each currentLocalPacks(recommendationSources.local) as pack (isLabPackId(pack.manifest.identity.id) ? pack.manifest.identity.id : pack.manifest.identity.id + '@' + pack.manifest.identity.revision)}
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
  <button type="button" disabled={removing || importing} onclick={() => removeSource(pack)}
    >{t('v11.source_remove')}</button
  >
  {#if isLabPackId(pack.manifest.identity.id)}<LabPresetManager {pack} />{/if}
{/each}
{#if !recommendationSources.local.length}<p class="meta">{t('fs37.no_local')}</p>{/if}
<label class="import"
  >{t('fs37.import')}<input
    type="file"
    disabled={importing || removing}
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
              (r) => !isLabPackId(pack.manifest.identity.id) || r.id !== pack.manifest.identity.id,
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
