<script lang="ts">
  import { materialFamilyKeys, type MaterialFamily } from '../state/material-presentation';
  import { t } from '../i18n';
  import { recordFocusIndex } from '../state/cutting-comparison';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import {
    presetRows,
    filterPresets,
    emptyPresetFilters,
    type PresetRow,
  } from '../state/preset-library';
  import {
    unavailableRecord,
    recordSelection,
    type SelectedRecord,
  } from '../state/cutting-selector';
  import PresetRecordDetails from './PresetRecordDetails.svelte';
  import DuplicatePresetDialog from './DuplicatePresetDialog.svelte';
  let {
    selected,
    onselect,
    compatibility = 'unchecked',
  }: {
    selected?: SelectedRecord;
    onselect: (row: PresetRow) => void;
    compatibility?: 'unchecked' | 'checking' | 'compatible' | 'incompatible';
  } = $props();
  let recordButtons = $state<HTMLButtonElement[]>([]);
  let focused = $state(0);
  const isSelected = (row: PresetRow) =>
    JSON.stringify(selected) === JSON.stringify(recordSelection(row));
  function moveFocus(event: KeyboardEvent, index: number) {
    const next = recordFocusIndex(event.key, index, visible.length);
    if (next == null) return;
    event.preventDefault();
    focused = next;
    recordButtons[next]?.focus();
  }
  function status(row: PresetRow) {
    if (unavailableRecord(row)) return t('selector.inspect');
    if (!isSelected(row) || compatibility === 'unchecked') return t('selector.check');
    return t(
      compatibility === 'checking'
        ? 'selector.checking'
        : compatibility === 'compatible'
          ? 'selector.compatible'
          : 'selector.incompatible',
    );
  }
  let viewportWidth = $state(1024);
  const narrow = $derived(viewportWidth <= 700);
  let filters = $state({ ...emptyPresetFilters(), availability: 'all' as const });

  let page = $state(0);
  let inspecting = $state<PresetRow>();
  let copying = $state<PresetRow>();
  const rows = $derived(
    presetRows(
      [...recommendationSources.builtins, ...recommendationSources.local],
      new Set(recommendationSources.rows.map((r) => r.id)),
      true,
    ),
  );
  const shown = $derived(filterPresets(rows, filters));
  const pages = $derived(Math.max(1, Math.ceil(shown.length / 50)));
  const visible = $derived(
    shown.slice(Math.min(page, pages - 1) * 50, (Math.min(page, pages - 1) + 1) * 50),
  );
  $effect(() => {
    void JSON.stringify(filters);

    page = 0;
  });
  const packs = $derived([
    ...new Map(
      rows.map((r) => [
        `${r.pack.manifest.identity.id}@${r.pack.manifest.identity.revision}`,
        r.pack.manifest.name,
      ]),
    ).entries(),
  ]);
  const families = $derived([...new Set(rows.map((r) => `display:${r.displayFamily}`))].sort());
  const tools = $derived([...new Set(rows.map((r) => r.profile.scope.cutting_family))]);
  const states = $derived([...new Set(rows.map((r) => r.state))]);
</script>

<svelte:window bind:innerWidth={viewportWidth} />

<div class="filters">
  <label>{t('presets.search')}<input type="search" bind:value={filters.query} /></label>
  <label
    >{t('presets.pack')}<select bind:value={filters.pack}
      ><option value="">{t('presets.all')}</option>{#each packs as [id, name] (id)}<option
          value={id}>{name}</option
        >{/each}</select
    ></label
  >
  <details class="secondary" open={!narrow}>
    <summary
      >{t('selector.filters')}{#if filters.materialFamily || filters.toolFamily || filters.state}
        · {t('selector.filtered')}{/if}</summary
    >
    <div class="secondary-filters">
      <label
        >{t('presets.material_family')}<select bind:value={filters.materialFamily}
          ><option value="">{t('presets.all')}</option>{#each families as value (value)}<option
              {value}
              >{value.startsWith('display:')
                ? t(materialFamilyKeys[value.slice(8) as MaterialFamily])
                : value}</option
            >{/each}</select
        ></label
      >
      <label
        >{t('presets.tool_family')}<select bind:value={filters.toolFamily}
          ><option value="">{t('presets.all')}</option>{#each tools as value (value)}<option
              >{value}</option
            >{/each}</select
        ></label
      >
      <label
        >{t('presets.validation')}<select bind:value={filters.state}
          ><option value="">{t('presets.all')}</option>{#each states as value (value)}<option
              >{value}</option
            >{/each}</select
        ></label
      >
    </div>
  </details>
</div>
<p class="list-help" id="preset-list-help">{t('selector.list_help')}</p>
<div class="records" aria-label={t('presets.records')}>
  <div class="columns" aria-hidden="true">
    <span>{t('selector.material_tool')}</span><span>{t('fs.mode')}</span><span
      >{t('presets.pack')}</span
    ><span>{t('selector.compatibility')}</span>
  </div>
  {#each visible as row, index (row.key)}
    <button
      bind:this={recordButtons[index]}
      type="button"
      class="record inspector-action"
      tabindex={index === Math.min(focused, visible.length - 1) ? 0 : -1}
      aria-describedby="preset-list-help"
      aria-pressed={isSelected(row)}
      class:chosen={isSelected(row)}
      title={`${row.name} · ${row.material} · ${row.tool} · ${row.state} · ${row.pack.manifest.identity.id}@${row.pack.manifest.identity.revision}`}
      onkeydown={(event) => moveFocus(event, index)}
      onfocus={() => (focused = index)}
      onclick={() => {
        inspecting = row;
        onselect(row);
      }}
    >
      <span class="identity"
        ><strong>{row.name}</strong><small>{row.material} · {row.tool}</small></span
      >
      <span
        >{t(
          row.profile.aggressiveness === 'prudent'
            ? 'fs.prudent'
            : row.profile.aggressiveness === 'normal'
              ? 'fs.normal'
              : 'fs.rapid',
        )}</span
      >
      <span
        class="pack"
        title={`${row.pack.manifest.identity.id}@${row.pack.manifest.identity.revision} · ${row.state}`}
        >{row.pack.manifest.name}<small>{row.state}</small></span
      >
      <span title={unavailableRecord(row) ? t('preset3.unavailable') : undefined}
        >{status(row)}</span
      >
    </button>
  {:else}<p role="status">
      {t(
        recommendationSources.loading
          ? 'fs35.loading'
          : recommendationSources.error
            ? 'fs35.source_unavailable'
            : 'presets.no_match',
      )}
    </p>{/each}
</div>
{#if selected && !shown.some(isSelected)}<p class="list-help" role="status">
    {t('selector.filtered_selection')}
  </p>{/if}
{#if pages > 1}<div class="pagination">
    <span>{Math.min(page, pages - 1) + 1} / {pages}</span><button
      type="button"
      disabled={page === 0}
      onclick={() => page--}>{t('tools.pager.prev')}</button
    ><button type="button" disabled={page >= pages - 1} onclick={() => page++}
      >{t('tools.pager.next')}</button
    >
  </div>{/if}
{#if inspecting}<details>
    <summary>{t('presets.details')}: {inspecting.name}</summary><PresetRecordDetails
      row={inspecting}
    />
    <button type="button" class="inspector-action" onclick={() => (copying = inspecting)}
      >{t('preset2.title')}</button
    >
  </details>
{/if}
{#if copying}<DuplicatePresetDialog row={copying} onClose={() => (copying = undefined)} />{/if}

<style>
  .filters {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 8px;
  }
  .secondary {
    grid-column: span 3;
    min-width: 0;
  }
  .secondary summary {
    display: none;
    cursor: pointer;
    color: var(--text-muted);
  }
  .secondary-filters {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--ui-space-2);
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
    font-size: var(--inspector-font-meta);
    line-height: 1.35;
  }
  input,
  select {
    width: 100%;
    min-width: 0;
    font-size: var(--inspector-font-field);
  }
  .records {
    max-height: 190px;
    overflow: auto;
    margin: var(--ui-space-2) 0;
  }
  .columns,
  .record {
    display: grid;
    grid-template-columns: minmax(180px, 2fr) 70px minmax(110px, 1fr) 90px;
    align-items: center;
    gap: var(--ui-space-2);
  }
  .columns {
    position: sticky;
    top: 0;
    z-index: 1;
    padding: var(--ui-space-1) var(--ui-space-2);
    background: var(--bg-panel);
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .records .record {
    width: 100%;
    text-align: left;
    border: 1px solid transparent;
    border-bottom-color: var(--border);
    border-radius: 0;
    padding: var(--ui-space-1) var(--ui-space-2);
    font-size: var(--inspector-font-field);
    line-height: 1.35;
    min-height: 40px;
  }
  .record span,
  .record strong,
  .record small {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .record strong,
  .record small {
    display: block;
  }
  .record strong {
    font-weight: 500;
  }
  .record small,
  .list-help {
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .list-help {
    margin: var(--ui-space-1) 0;
    line-height: 1.35;
  }
  .record:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
  @media (max-width: 700px) {
    .filters {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
    .secondary {
      grid-column: 1 / -1;
    }
    .secondary summary {
      display: list-item;
      min-height: 24px;
    }
    .secondary-filters {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
    .columns,
    .record {
      grid-template-columns: minmax(130px, 2fr) 60px minmax(80px, 1fr) 80px;
      gap: var(--ui-space-1);
    }
  }
  .pagination,
  details {
    font-size: var(--inspector-font-meta);
  }
  .records .record.chosen {
    border-color: var(--accent);
    background: var(--accent-tint);
  }
</style>
