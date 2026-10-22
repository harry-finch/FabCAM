<script lang="ts">
  import { t } from '../i18n';
  import {
    materialPresentation,
    materialIdentityKey,
    materialFamilyKeys,
    type MaterialOption,
    type MaterialFamily,
  } from '../state/material-presentation';
  import { materialPage, materialPickerLabel } from '../state/material-picker-presentation';
  import { focusMaterialResult, recordFocusIndex } from './material-browser-focus';
  let {
    options,
    selected,
    selectedLabel,
    onselect,
    onclose,
  }: {
    options: MaterialOption[];
    selected?: { id: string; revision: string };
    selectedLabel?: string;
    onselect: (item: MaterialOption) => void;
    onclose: () => void;
  } = $props();
  let query = $state('');
  let family = $state('');
  let source = $state('');
  let page = $state(0);
  let results = $state<HTMLDivElement>();
  let focused = $state(0);
  const sources = $derived(
    [...new Map(options.flatMap((o) => o.sources).map((s) => [s.key, s])).values()].sort((a, b) =>
      a.key < b.key ? -1 : 1,
    ),
  );
  const result = $derived(materialPage(options, query, family, source, page));
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
  function searchKeys(event: KeyboardEvent) {
    escape(event);
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      const index = event.key === 'ArrowDown' ? 0 : result.visible.length - 1;
      focusMaterialResult(results, index);
    }
  }
  function move(event: KeyboardEvent, index: number) {
    escape(event);
    const next = recordFocusIndex(event.key, index, result.visible.length);
    if (next == null) return;
    event.preventDefault();
    focused = next;
    focusMaterialResult(results, next);
  }
</script>

<div class="browser">
  <div class="search-field">
    <label
      >{t('presets.search')}<input type="search" bind:value={query} onkeydown={searchKeys} /></label
    >
    {#if selectedLabel}<span class="current" title={selectedLabel}
        >{t('preset3.current')}: {selectedLabel}</span
      >{/if}
  </div>
  <details class="advanced">
    <summary>{t(family || source ? 'mat.filters_active' : 'mat.filters')}</summary>
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
  </details>
  <div class="results" bind:this={results} aria-label={t('fs.material')}>
    {#each result.visible as item, index (item.key)}
      <button
        type="button"
        data-material-option
        class="inspector-action"
        aria-pressed={selected ? item.key === materialIdentityKey(selected) : false}
        tabindex={index === Math.min(focused, result.visible.length - 1) ? 0 : -1}
        onfocus={() => (focused = index)}
        onkeydown={(e) => move(e, index)}
        onclick={() => onselect(item)}
        title={[
          `${item.identity.id} @${item.identity.revision}`,
          t('mat.original'),
          ...item.labels,
          ...item.sources.map((s) => `${s.name} · ${s.key}`),
        ].join('\n')}
      >
        <strong>{materialPickerLabel(item.identity, item.labels[0], t)}</strong>
        <span
          >{materialPresentation(item.identity)
            .keys.slice(0, -1)
            .map((key) => t(key))
            .join(' / ') || t(materialFamilyKeys[item.family])} · {item.sources.length
            ? item.sources.map((s) => s.name).join(' · ')
            : t(item.family === 'unknown' ? 'mat.current_only' : 'mat.builtin')}</span
        >
      </button>
    {:else}<p role="status">{t('presets.no_match')}</p>{/each}
  </div>
  <div class="pager">
    <span>{result.count} · {result.page + 1}/{result.pages}</span><button
      type="button"
      disabled={result.page === 0}
      onclick={() => page--}>{t('tools.pager.prev')}</button
    ><button type="button" disabled={result.page >= result.pages - 1} onclick={() => page++}
      >{t('tools.pager.next')}</button
    >
  </div>
</div>

<style>
  .browser {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    gap: var(--ui-space-1);
  }
  .search-field {
    position: relative;
    flex: none;
  }
  .current {
    position: absolute;
    top: 0;
    right: 0;
    max-width: 75%;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
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
    font-size: var(--inspector-font-field);
  }
  .advanced {
    flex: none;
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  summary {
    cursor: pointer;
    min-height: 24px;
    display: list-item;
  }
  .filters {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--ui-space-2);
    margin-bottom: var(--ui-space-1);
  }
  .results {
    min-height: 40px;
    max-height: 240px;
    overflow: auto;
    flex: 1 1 auto;
    overscroll-behavior: contain;
  }
  .results button {
    display: flex;
    flex-direction: column;
    width: 100%;
    text-align: left;
    gap: 2px;
    padding: var(--ui-space-1) var(--ui-space-2);
    border: 1px solid transparent;
    border-bottom-color: var(--border);
    border-radius: 0;
    min-height: 36px;
    font-size: var(--inspector-font-field);
  }
  .results strong,
  .results span {
    display: block;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
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
  p {
    margin: var(--ui-space-1) 0;
  }
  strong {
    font-weight: 500;
  }
  .pager {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--ui-space-1);
    flex: none;
  }
  .pager span {
    margin-right: auto;
  }
</style>
