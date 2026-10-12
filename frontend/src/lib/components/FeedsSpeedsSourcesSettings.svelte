<script lang="ts">
  import { project } from '../state/project.svelte';
  import { importedSource, type SourceSettings } from '../state/feeds-speeds-sources';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { t } from '../i18n';
  let error = $state('');
  function update(next: SourceSettings) {
    project.updateSettings({ feedsSpeedsSources: next });
    void recommendationSources.refresh();
  }
</script>

<h3>{t('fs35.sources')}</h3>
<p>{t('fs35.sources_help')}</p>
<label
  ><input
    type="checkbox"
    checked={project.data.settings.feedsSpeedsSources.sienciEnabled}
    onchange={(e) =>
      update({
        ...project.data.settings.feedsSpeedsSources,
        sienciEnabled: e.currentTarget.checked,
      })}
  />{t('fs35.sienci')}</label
>
<p>{t('fs35.sienci_scope')}</p>
{#each project.data.settings.feedsSpeedsSources.custom as source (source.id)}
  <label
    ><input
      type="checkbox"
      checked={source.enabled}
      onchange={(e) =>
        update({
          ...project.data.settings.feedsSpeedsSources,
          custom: project.data.settings.feedsSpeedsSources.custom.map((s) =>
            s.id === source.id ? { ...s, enabled: e.currentTarget.checked } : s,
          ),
        })}
    />{source.label}</label
  >
{/each}
<label class="import"
  >{t('fs35.import')}<input
    type="file"
    accept=".json,application/json"
    onchange={async (e) => {
      const file = e.currentTarget.files?.[0];
      if (!file) return;
      try {
        const source = importedSource(await file.text());
        update({
          ...project.data.settings.feedsSpeedsSources,
          custom: [
            ...project.data.settings.feedsSpeedsSources.custom.filter((s) => s.id !== source.id),
            source,
          ],
        });
        error = '';
      } catch {
        error = t('fs.invalid_data');
      }
    }}
  /></label
>
{#if error}<p role="alert">{error}</p>{/if}
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
    gap: 0.5rem;
    margin: 0.6rem 0;
    font-size: 0.85rem;
  }
  p {
    font-size: 0.8rem;
    max-width: 45rem;
  }
  .import {
    display: block;
  }
  input[type='file'] {
    display: block;
    margin-top: 0.4rem;
  }
  summary {
    cursor: pointer;
  }
</style>
