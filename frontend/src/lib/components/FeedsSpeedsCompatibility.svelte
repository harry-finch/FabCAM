<script lang="ts">
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { project } from '../state/project.svelte';
  import { isLabPackId } from '../state/lab-presets';
  import { referenceIdentity } from '../state/cutting-identities';
  import type { components } from '../api/generated';
  import { t } from '../i18n';
  type Identity = components['schemas']['FsIdentity'];
  let {
    kind,
    value,
    localIdentity,
    onChange,
    onLocalChange,
  }: {
    kind: 'machines' | 'tools';
    value?: Identity | null;
    localIdentity?: Identity | null;
    onChange: (identity: Identity | null) => void;
    onLocalChange: (identity: Identity) => void;
  } = $props();
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  const sources = $derived(recommendationSources.rows);
  const items = $derived([
    ...new Map(
      sources
        .filter((s) => !isLabPackId(s.pack?.manifest.identity.id ?? ''))
        .flatMap((s) =>
          s.catalog[kind].map((item) => ({ ...item, source: s.pack?.manifest.name ?? s.label })),
        )
        .map((item) => [JSON.stringify(item.identity), item]),
    ).values(),
  ]);
  const labItems = $derived([
    ...new Map(
      sources
        .filter((s) => isLabPackId(s.pack?.manifest.identity.id ?? ''))
        .flatMap((s) => s.catalog[kind])
        .map((item) => [JSON.stringify(item.identity), item]),
    ).values(),
  ]);
  const selected = $derived(JSON.stringify(referenceIdentity(value)));
  const local = $derived(localIdentity ?? value);
  const selectedLocal = $derived(JSON.stringify(local ?? null));
</script>

<section class="cutting-identity">
  <h4>{t(kind === 'machines' ? 'v11.identity_machine' : 'v11.identity_tool')}</h4>
  <p>{t('v11.identity_automatic')}</p>
  <label
    >{t(kind === 'machines' ? 'v11.lab_machine' : 'v11.lab_tool')}
    <select
      value={selectedLocal}
      onchange={(e) => onLocalChange(JSON.parse(e.currentTarget.value))}
    >
      {#if !labItems.some((item) => JSON.stringify(item.identity) === selectedLocal)}
        <option value={selectedLocal}
          >{t(kind === 'machines' ? 'v11.this_machine' : 'v11.this_tool')}</option
        >
      {/if}
      {#each labItems as item (JSON.stringify(item.identity))}
        <option value={JSON.stringify(item.identity)}>{item.label}</option>
      {/each}
    </select>
  </label>
  <p>{t('v11.lab_identity_help')}</p>
  <label
    >{t('v11.pack_reference')}
    <select value={selected} onchange={(e) => onChange(JSON.parse(e.currentTarget.value))}>
      <option value="null">{t('v11.no_reference')}</option>
      {#each items as item (JSON.stringify(item.identity))}
        <option value={JSON.stringify(item.identity)}>{item.label} · {item.source}</option>
      {/each}
      {#if referenceIdentity(value) && !items.some((item) => JSON.stringify(item.identity) === selected)}
        <option value={selected}>{value?.id}</option>
      {/if}
    </select>
  </label>
  <p>{t('v11.reference_independent')}</p>
  <details>
    <summary>{t('v11.reference_help')}</summary>
    <p>{t('fs35.compatibility_help')}</p>
    <p>{kind === 'machines' ? t('fs39.machine_setup') : t('fs39.tool_setup')}</p>
  </details>
  {#if local}<details>
      <summary>{t('fs38.identity')}</summary><small>{local.id} · {local.revision}</small>
    </details>{/if}
</section>

<style>
  h4 {
    margin: 0.6rem 0;
  }
  .cutting-identity,
  details {
    font-size: 0.8rem;
  }
  details {
    margin: 0.6rem 0;
  }
  summary {
    cursor: pointer;
  }
  p {
    max-width: 40rem;
  }
  small {
    display: block;
    margin-top: 0.4rem;
    overflow-wrap: anywhere;
    opacity: 0.7;
  }
  select {
    display: block;
    width: 100%;
    max-width: 35rem;
    margin: 0.4rem 0;
  }
</style>
