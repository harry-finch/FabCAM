<script lang="ts">
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { project } from '../state/project.svelte';
  import type { components } from '../api/generated';
  import { t } from '../i18n';
  type Identity = components['schemas']['FsIdentity'];
  let {
    kind,
    value,
    onChange,
  }: {
    kind: 'machines' | 'tools';
    value?: Identity | null;
    onChange: (identity: Identity | null) => void;
  } = $props();
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  const items = $derived([
    ...new Map(
      recommendationSources.rows
        .flatMap((s) => s.catalog[kind])
        .map((item) => [JSON.stringify(item.identity), item]),
    ).values(),
  ]);
  const selected = $derived(JSON.stringify(value ?? null));
</script>

<details>
  <summary>{t('fs35.compatibility')}</summary>
  <p>{t('fs35.compatibility_help')}</p>
  <label
    >{t('fs35.reference_match')}<select
      value={selected}
      onchange={(e) => onChange(JSON.parse(e.currentTarget.value))}
      ><option value="null">{t('fs.no_match')}</option
      >{#each items as item (JSON.stringify(item.identity))}<option
          value={JSON.stringify(item.identity)}>{item.label}</option
        >{/each}{#if value && !items.some((item) => JSON.stringify(item.identity) === selected)}<option
          value={selected}>{value.id}</option
        >{/if}</select
    ></label
  >
</details>

<style>
  details {
    margin: 0.6rem 0;
    font-size: 0.8rem;
  }
  summary {
    cursor: pointer;
  }
  p {
    max-width: 40rem;
  }
  select {
    display: block;
    width: 100%;
    max-width: 35rem;
    margin: 0.4rem 0;
  }
</style>
