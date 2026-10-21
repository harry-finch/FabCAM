<script lang="ts">
  import { t } from '../i18n';
  import {
    filterMaterials,
    materialDisplay,
    materialIdentityKey,
    materialFamilyKeys,
    type MaterialOption,
    type MaterialFamily,
  } from '../state/material-presentation';
  import { recordFocusIndex } from '../state/cutting-comparison';
  let {
    options,
    selected,
    onselect,
    onclose,
  }: {
    options: MaterialOption[];
    selected?: { id: string; revision: string };
    onselect: (item: MaterialOption) => void;
    onclose: () => void;
  } = $props();
  let query = $state('');
  let family = $state('');
  let source = $state('');
  let page = $state(0);
  let buttons = $state<HTMLButtonElement[]>([]);
  let focused = $state(0);
  const sources = $derived(
    [...new Map(options.flatMap((o) => o.sources).map((s) => [s.key, s])).values()].sort((a, b) =>
      a.key < b.key ? -1 : 1,
    ),
  );
  const shown = $derived(filterMaterials(options, query, family, source));
  const pages = $derived(Math.max(1, Math.ceil(shown.length / 30)));
  const visible = $derived(
    shown.slice(Math.min(page, pages - 1) * 30, (Math.min(page, pages - 1) + 1) * 30),
  );
  $effect(() => {
    void query;
    void family;
    void source;
    page = 0;
    focused = 0;
  });
  function escape(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.stopPropagation();
      event.preventDefault();
      onclose();
    }
  }
  function move(event: KeyboardEvent, index: number) {
    escape(event);
    const next = recordFocusIndex(event.key, index, visible.length);
    if (next == null) return;
    event.preventDefault();
    focused = next;
    buttons[next]?.focus();
  }
</script>

<div class="browser">
  <label>{t('presets.search')}<input type="search" bind:value={query} onkeydown={escape} /></label>
  <div class="filters">
    <label
      >{t('presets.material_family')}<select bind:value={family}
        ><option value="">{t('presets.all')}</option
        >{#each Object.keys(materialFamilyKeys) as key (key)}<option value={key}
            >{t(materialFamilyKeys[key as MaterialFamily])}</option
          >{/each}</select
      ></label
    >
    <label
      >{t('presets.pack')}<select bind:value={source}
        ><option value="">{t('presets.all')}</option>{#each sources as s (s.key)}<option
            value={s.key}>{s.name} · {s.key}</option
          >{/each}</select
      ></label
    >
  </div>
  <p>{t('mat.help')}</p>
  <div class="results" aria-label={t('fs.material')}>
    {#each visible as item, index (item.key)}
      <button
        type="button"
        bind:this={buttons[index]}
        class="inspector-action"
        aria-pressed={selected ? item.key === materialIdentityKey(selected) : false}
        tabindex={index === Math.min(focused, visible.length - 1) ? 0 : -1}
        onfocus={() => (focused = index)}
        onkeydown={(e) => move(e, index)}
        onclick={() => onselect(item)}
        title={[
          t('mat.original'),
          ...item.labels,
          ...item.sources.map((s) => `${s.name} · ${s.key}`),
        ].join('\n')}
      >
        <strong>{materialDisplay(item.identity, item.labels[0], t)}</strong>
        <span>{item.identity.id} @{item.identity.revision}</span>
        <span
          >{item.sources.length
            ? item.sources.map((s) => `${s.name} · ${s.key}`).join(' · ')
            : t(item.family === 'unknown' ? 'mat.unknown' : 'mat.builtin')}</span
        >
        {#if item.broad}<span>{t('mat.broad')}</span>{/if}
      </button>
    {:else}<p role="status">{t('presets.no_match')}</p>{/each}
  </div>
  <div class="pager">
    <span>{shown.length} · {Math.min(page, pages - 1) + 1}/{pages}</span><button
      type="button"
      disabled={page === 0}
      onclick={() => page--}>{t('tools.pager.prev')}</button
    ><button type="button" disabled={page >= pages - 1} onclick={() => page++}
      >{t('tools.pager.next')}</button
    >
  </div>
</div>

<style>
  .browser {
    min-width: 0;
    padding: var(--ui-space-2) 0;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-1);
    min-width: 0;
    font-size: var(--inspector-font-meta);
  }
  input,
  select {
    width: 100%;
    min-width: 0;
  }
  .filters {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(110px, 1fr));
    gap: var(--ui-space-2);
    margin-top: var(--ui-space-2);
  }
  .results {
    max-height: 240px;
    overflow: auto;
  }
  .results button {
    display: flex;
    flex-direction: column;
    width: 100%;
    text-align: left;
    gap: var(--ui-space-1);
    margin-bottom: var(--ui-space-1);
    white-space: normal;
    overflow-wrap: anywhere;
  }
  .results button[aria-pressed='true'] {
    background: var(--accent-tint);
    border-color: var(--accent);
  }
  button:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
  span,
  p {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  strong {
    font-weight: 500;
  }
  .pager {
    display: flex;
    justify-content: space-between;
    gap: var(--ui-space-1);
    margin-top: var(--ui-space-2);
  }
</style>
