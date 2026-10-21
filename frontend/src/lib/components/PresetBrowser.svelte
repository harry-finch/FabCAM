<script lang="ts">
  import { trialEligible } from '../state/experimental-recipe';
  import { materialFamilyKeys, type MaterialFamily } from '../state/material-presentation';
  import { t } from '../i18n';
  import { recordFocusIndex } from '../state/cutting-comparison';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import {
    presetRows,
    rangeText,
    filterPresets,
    emptyPresetFilters,
    type PresetRow,
  } from '../state/preset-library';
  import {
    unavailableRecord,
    recordSelection,
    type SelectedRecord,
  } from '../state/cutting-selector';
  import PresetCuttingSummary from './PresetCuttingSummary.svelte';
  import PresetRecordDetails from './PresetRecordDetails.svelte';
  import DuplicatePresetDialog from './DuplicatePresetDialog.svelte';
  let {
    selected,
    onselect,
    ontrial,
    compatibility = 'unchecked',
  }: {
    selected?: SelectedRecord;
    onselect: (row: PresetRow) => void;
    ontrial?: (row: PresetRow) => void;
    compatibility?: 'unchecked' | 'checking' | 'compatible' | 'incompatible';
  } = $props();
  const helpId = $props.id();
  let records = $state<HTMLDivElement>();
  let focused = $state(0);
  const isSelected = (row: PresetRow) =>
    JSON.stringify(selected) === JSON.stringify(recordSelection(row));
  function moveFocus(event: KeyboardEvent, index: number) {
    const next = recordFocusIndex(event.key, index, visible.length);
    if (next == null) return;
    event.preventDefault();
    focused = next;
    records?.querySelectorAll<HTMLButtonElement>('button[data-preset-option]')[next]?.focus();
  }
  function status(row: PresetRow) {
    if (row.record?.lab?.state === 'draft') return t('fs38.draft');
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
  let filters = $state({ ...emptyPresetFilters(), availability: 'all' as const });

  let page = $state(0);
  let copying = $state<PresetRow>();
  const rows = $derived(
    presetRows(
      [...recommendationSources.builtins, ...recommendationSources.local],
      new Set(recommendationSources.rows.map((r) => r.id)),
      true,
    ),
  );
  const inspecting = $derived(rows.find(isSelected));
  const shown = $derived(filterPresets(rows, filters));
  const pages = $derived(Math.max(1, Math.ceil(shown.length / 50)));
  const visible = $derived(
    shown.slice(Math.min(page, pages - 1) * 50, (Math.min(page, pages - 1) + 1) * 50),
  );
  $effect(() => {
    void JSON.stringify(filters);
    page = 0;
    focused = 0;
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

<div class="filters">
  <label>{t('presets.search')}<input type="search" bind:value={filters.query} /></label>
  <details class="secondary">
    <summary
      >{t(
        'selector.filters',
      )}{#if filters.pack || filters.materialFamily || filters.toolFamily || filters.state}
        · {t('selector.filtered')}{/if}</summary
    >
    <div class="secondary-filters">
      <label
        >{t('presets.pack')}<select bind:value={filters.pack}
          ><option value="">{t('presets.all')}</option>{#each packs as [id, name] (id)}<option
              value={id}>{name}</option
            >{/each}</select
        ></label
      >
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
{#if inspecting}
  <section class="selection" aria-label={t('selector.selected_preset')}>
    <div class="selection-head">
      <strong title={inspecting.name}>{inspecting.name}</strong>
      <button type="button" class="inspector-action" onclick={() => (copying = inspecting)}
        >{t('preset2.title')}</button
      >
    </div>
    <PresetCuttingSummary row={inspecting} />
    {#if ontrial && trialEligible(inspecting)}<button
        type="button"
        class="inspector-action"
        onclick={() => ontrial?.(inspecting)}>{t('trial.use')}</button
      >{/if}
    {#if unavailableRecord(inspecting) && !trialEligible(inspecting)}<p
        class="list-help"
        role="status"
        title={t('preset3.unavailable')}
      >
        {t('selector.copy_only')}
      </p>{/if}
    {#key inspecting.key}
      <details class="record-details">
        <summary>{t('presets.details')}</summary><PresetRecordDetails row={inspecting} />
      </details>
    {/key}
  </section>
{/if}
<p class="keyboard-help" id={helpId}>{t('selector.list_help')}</p>
<div
  class="records"
  bind:this={records}
  aria-label={t('presets.records')}
  title={t('selector.list_help')}
>
  {#each visible as row, index (row.key)}
    <button
      data-preset-option
      aria-describedby={helpId}
      type="button"
      class="record inspector-action"
      tabindex={index === Math.min(focused, visible.length - 1) ? 0 : -1}
      aria-pressed={isSelected(row)}
      class:chosen={isSelected(row)}
      title={`${row.name} · ${row.material} · ${row.tool} · ${row.state} · ${row.pack.manifest.identity.id}@${row.pack.manifest.identity.revision}`}
      onkeydown={(event) => moveFocus(event, index)}
      onfocus={() => (focused = index)}
      onclick={() => {
        onselect(row);
      }}
    >
      <span class="identity" title={`${t('selector.material_tool')}: ${row.material} · ${row.tool}`}
        ><strong>{row.name}</strong><small
          >{row.pack.manifest.name} · Ø{rangeText(row.profile.scope.diameter_mm)} mm · {row.profile
            .scope.flute_count ?? '—'}F</small
        ></span
      >
      <span
        class="compatibility"
        aria-label={`${t('selector.compatibility')}: ${status(row)}`}
        title={trialEligible(row)
          ? t('trial.status')
          : unavailableRecord(row)
            ? t('preset3.unavailable')
            : undefined}>{status(row)}</span
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
{#if copying}<DuplicatePresetDialog row={copying} onClose={() => (copying = undefined)} />{/if}

<style>
  .keyboard-help {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
  .filters {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: end;
    gap: var(--ui-space-2);
  }
  .secondary {
    min-width: 0;
  }
  .secondary[open] {
    grid-column: 1 / -1;
  }
  .secondary summary,
  .record-details summary {
    cursor: pointer;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
    min-height: 24px;
  }
  .secondary-filters {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: var(--ui-space-2);
    margin: var(--ui-space-1) 0;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-1);
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
  .selection {
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    margin: var(--ui-space-2) 0;
    padding: var(--ui-space-2) 0;
  }
  .selection-head {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
  }
  .selection-head strong {
    flex: 1;
    min-width: 160px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: var(--inspector-font-field);
    font-weight: 500;
  }
  .selection-head button {
    flex: none;
  }
  .records {
    max-height: 190px;
    overflow: auto;
    margin: var(--ui-space-2) 0;
    overscroll-behavior: contain;
  }
  .records .record {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--ui-space-2);
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
  .compatibility {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .record:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
  .pagination {
    font-size: var(--inspector-font-meta);
  }
  .records .record.chosen {
    border-color: var(--accent);
    background: var(--accent-tint);
  }
</style>
