<script lang="ts">
  import { trialEligible } from '../state/experimental-recipe';
  import { materialFamilyKeys, type MaterialFamily } from '../state/material-presentation';
  import PresetOperationPicker from './PresetOperationPicker.svelte';
  let using = $state<PresetRow>();
  import Splitter from './Splitter.svelte';
  import { t } from '../i18n';
  import type { MsgKey } from '../i18n/keys';
  import { project } from '../state/project.svelte';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { effectiveModes } from '../state/tool_family';
  import {
    presetRows,
    filterPresets,
    emptyPresetFilters,
    rangeText,
  } from '../state/preset-library';
  import DuplicatePresetDialog from './DuplicatePresetDialog.svelte';
  import type { PresetRow } from '../state/preset-library';
  let copying = $state<PresetRow | undefined>();
  import PresetRecordDetails from './PresetRecordDetails.svelte';
  let { active }: { active: boolean } = $props();
  let filters = $state(emptyPresetFilters());
  let showDetails = $state(true);
  let contentWidth = $state(0),
    tableWidth = $state(0),
    detailsWidth = $state(340);
  const maxDetailsWidth = $derived(Math.min(640, Math.max(260, contentWidth * 0.6)));
  const effectiveDetailsWidth = $derived(Math.min(detailsWidth, maxDetailsWidth));
  const columns: { id: string; label: MsgKey; weight: number; min: number }[] = [
    { id: 'name', label: 'fs38.name', weight: 0.22, min: 130 },
    { id: 'material', label: 'fs.material', weight: 0.18, min: 100 },
    { id: 'tool', label: 'presets.tool', weight: 0.18, min: 100 },
    { id: 'diameter', label: 'presets.diameter', weight: 0.08, min: 72 },
    { id: 'flutes', label: 'tools.col.flutes', weight: 0.05, min: 52 },
    { id: 'rpm', label: 'fs.rpm', weight: 0.08, min: 76 },
    { id: 'feed', label: 'fs.feed', weight: 0.09, min: 90 },
    { id: 'doc', label: 'fs36.pass_depth', weight: 0.06, min: 80 },
    { id: 'stepover', label: 'fs.stepover', weight: 0.06, min: 76 },
  ];
  let columnWidths = $state<Record<string, number>>({});
  // Fit the initial table to the viewport; manual widths remain explicit pixel choices.
  const spareWidth = $derived(Math.max(0, tableWidth - columns.reduce((sum, c) => sum + c.min, 0)));
  const widths = $derived(
    columns.map(
      (c, index) => columnWidths[c.id] ?? c.min + (index < 3 ? (spareWidth * c.weight) / 0.58 : 0),
    ),
  );
  const totalWidth = $derived(widths.reduce((total, width) => total + width, 0));
  function resizeColumn(index: number, delta: number) {
    const column = columns[index];
    columnWidths = {
      ...columnWidths,
      [column.id]: Math.max(column.min, Math.min(720, widths[index] + delta)),
    };
  }
  function resetColumn(id: string) {
    const next = { ...columnWidths };
    delete next[id];
    columnWidths = next;
  }
  let page = $state(0),
    selectedKey = $state('');
  $effect(() => {
    if (active) {
      void project.data.settings.feedsSpeedsSources;
      void recommendationSources.refresh(true);
    }
  });
  const rows = $derived(
    presetRows(
      [...recommendationSources.builtins, ...recommendationSources.local],
      new Set(recommendationSources.rows.map((row) => row.id)),
      effectiveModes(project.data.machine).includes('mill'),
    ),
  );
  const filtered = $derived(filterPresets(rows, filters));
  const pages = $derived(Math.max(1, Math.ceil(filtered.length / 50)));
  const shown = $derived(
    filtered.slice(Math.min(page, pages - 1) * 50, (Math.min(page, pages - 1) + 1) * 50),
  );
  const selected = $derived(shown.find((row) => row.key === selectedKey) ?? shown[0]);
  const packOptions = $derived([
    ...new Map(
      [...recommendationSources.builtins, ...recommendationSources.local].map((pack) => [
        `${pack.manifest.identity.id}@${pack.manifest.identity.revision}`,
        pack.manifest.name,
      ]),
    ).entries(),
  ]);
  const families = $derived([...new Set(rows.map((row) => `display:${row.displayFamily}`))].sort());
  const tools = $derived([...new Set(rows.map((row) => row.profile.scope.cutting_family))].sort());
  const states = $derived([...new Set(rows.map((row) => row.state))].sort());
  const filterKey = $derived(JSON.stringify(filters));
  $effect(() => {
    void filterKey;
    page = 0;
  });
  const statusKeys: Record<string, MsgKey> = {
    draft: 'fs38.draft',
    tested: 'fs35.tested',
    validated: 'fs35.validated',
    theoretical: 'fs35.theoretical',
    reference_data: 'fs35.reference',
    deleted: 'presets.deleted',
  };
  function navigateRow(event: KeyboardEvent, index: number) {
    const next =
      event.key === 'ArrowDown'
        ? Math.min(index + 1, shown.length - 1)
        : event.key === 'ArrowUp'
          ? Math.max(index - 1, 0)
          : event.key === 'Home'
            ? 0
            : event.key === 'End'
              ? shown.length - 1
              : -1;
    if (next < 0) return;
    event.preventDefault();
    selectedKey = shown[next].key;
    const buttons = (event.currentTarget as HTMLElement)
      .closest('table')
      ?.querySelectorAll<HTMLButtonElement>('button[data-record]');
    buttons?.[next]?.focus();
  }
</script>

{#if copying}<DuplicatePresetDialog row={copying} onClose={() => (copying = undefined)} />{/if}

{#if using}<PresetOperationPicker row={using} onClose={() => (using = undefined)} />{/if}
<div class="library">
  <header>
    <div>
      <h1>{t('presets.title')}</h1>
      <p>{t('presets.readonly_help')}</p>
    </div>
    <button
      type="button"
      disabled={!selected || recommendationSources.loading}
      onclick={() => (using = selected ? JSON.parse(JSON.stringify(selected)) : undefined)}
      >{selected && trialEligible(selected) ? t('trial.use') : t('preset3.use')}</button
    >
    <button
      type="button"
      disabled={!selected || recommendationSources.loading}
      onclick={() => (copying = selected ? JSON.parse(JSON.stringify(selected)) : undefined)}
      >{t('preset2.title')}</button
    >
    <button type="button" aria-expanded={showDetails} onclick={() => (showDetails = !showDetails)}
      >{t('presets.details')}</button
    >
    <button
      type="button"
      disabled={recommendationSources.loading}
      onclick={() => void recommendationSources.refresh(true)}>{t('presets.refresh')}</button
    >
  </header>
  <div class="filters">
    <label
      >{t('presets.search')}<input
        type="search"
        bind:value={filters.query}
        placeholder={t('presets.search_hint')}
      /></label
    >
    <label
      >{t('presets.pack')}<select bind:value={filters.pack}
        ><option value="">{t('presets.all')}</option>{#each packOptions as [id, name] (id)}<option
            value={id}>{name}</option
          >{/each}</select
      ></label
    >
    <label
      >{t('presets.material_family')}<select bind:value={filters.materialFamily}
        ><option value="">{t('presets.all')}</option>{#each families as family (family)}<option
            value={family}
            >{family.startsWith('display:')
              ? t(materialFamilyKeys[family.slice(8) as MaterialFamily])
              : family}</option
          >{/each}</select
      ></label
    >
    <label
      >{t('presets.tool_family')}<select bind:value={filters.toolFamily}
        ><option value="">{t('presets.all')}</option>{#each tools as tool (tool)}<option
            value={tool}>{tool.replaceAll('_', ' ')}</option
          >{/each}</select
      ></label
    >
    <label
      >{t('presets.validation')}<select bind:value={filters.state}
        ><option value="">{t('presets.all')}</option>{#each states as state (state)}<option
            value={state}>{t(statusKeys[state])}</option
          >{/each}</select
      ></label
    >
    <label
      >{t('presets.availability')}<select bind:value={filters.availability}
        ><option value="enabled">{t('presets.enabled')}</option><option value="all"
          >{t('presets.all_sources')}</option
        ><option value="disabled">{t('presets.disabled')}</option></select
      ></label
    >
    <button type="button" onclick={() => (filters = emptyPresetFilters())}
      >{t('presets.clear')}</button
    >
  </div>
  {#if recommendationSources.loading}<p role="status" class="message">{t('fs35.loading')}</p>
  {:else if recommendationSources.error}<p role="alert" class="message error">
      {recommendationSources.error}
    </p>
  {:else}
    <div class="summary" role="status">
      {filtered.length} / {rows.length}
      {t('presets.records')}
      {#if rows.some((row) => !row.enabled)}
        · {rows.filter((row) => !row.enabled).length} {t('presets.disabled')}{/if}
    </div>
    {#if !rows.length}<p class="message">{t('presets.empty')}</p>
    {:else if !filtered.length}<p class="message">
        {t('presets.no_match')}{#if !effectiveModes(project.data.machine).includes('mill')}
          {t('presets.milling_only')}{/if}
      </p>
    {:else}
      <div
        class="content"
        class:full={!showDetails}
        bind:clientWidth={contentWidth}
        style:--details-width={`${effectiveDetailsWidth}px`}
      >
        <div class="table-panel">
          <div class="table-scroll" bind:clientWidth={tableWidth}>
            <table aria-label={t('presets.title')} style:width={`${totalWidth}px`}>
              <colgroup
                >{#each columns as column, index (column.id)}<col
                    style:width={`${widths[index]}px`}
                  />{/each}</colgroup
              >
              <thead
                ><tr
                  >{#each columns as column, index (column.id)}
                    <th scope="col"
                      >{t(column.label)}
                      <div class="column-resize">
                        <Splitter
                          direction="horizontal"
                          title={t('presets.resize_column', { name: t(column.label) })}
                          value={widths[index]}
                          min={column.min}
                          max={720}
                          onResize={(delta) => resizeColumn(index, delta)}
                          onReset={() => resetColumn(column.id)}
                        />
                      </div></th
                    >
                  {/each}</tr
                ></thead
              >
              <tbody
                >{#each shown as row, index (row.key)}<tr
                    class:selected={selected?.key === row.key}
                  >
                    <td
                      ><button
                        data-record
                        aria-label={`${row.name} · ${rangeText(row.profile.scope.diameter_mm)} mm · ${row.pack.manifest.name}`}
                        type="button"
                        aria-pressed={selected?.key === row.key}
                        tabindex={selected?.key === row.key ? 0 : -1}
                        onclick={() => {
                          selectedKey = row.key;
                          showDetails = true;
                        }}
                        onkeydown={(e) => navigateRow(e, index)}
                        title={row.name}>{row.name}</button
                      ><small
                        >{row.pack.manifest.name}{#if !row.enabled}
                          · {t('presets.disabled')}{/if}</small
                      ></td
                    >
                    <td title={row.material}>{row.material}</td><td title={row.tool}>{row.tool}</td
                    ><td>{rangeText(row.profile.scope.diameter_mm)} mm</td><td
                      >{row.profile.scope.flute_count ?? '—'}</td
                    >
                    <td>{row.profile.data.rpm} rpm</td><td
                      >{row.profile.data.kind === 'exact_values'
                        ? `${row.profile.data.feed_mm_min} mm/min`
                        : '—'}</td
                    ><td>{row.profile.data.doc_mm} mm</td><td
                      >{row.profile.data.stepover_ratio == null
                        ? '—'
                        : `${Number((row.profile.data.stepover_ratio * 100).toFixed(4))} %`}</td
                    >
                  </tr>{/each}</tbody
              >
            </table>
          </div>
          <footer>
            <span>{Math.min(page, pages - 1) + 1} / {pages}</span><button
              type="button"
              disabled={page === 0}
              onclick={() => page--}>{t('tools.pager.prev')}</button
            ><button type="button" disabled={page >= pages - 1} onclick={() => page++}
              >{t('tools.pager.next')}</button
            >
          </footer>
        </div>
        {#if selected && showDetails}
          <div class="details-resize">
            <Splitter
              direction="horizontal"
              title={t('presets.resize_details')}
              value={contentWidth - effectiveDetailsWidth}
              min={contentWidth - maxDetailsWidth}
              max={contentWidth - 260}
              onResize={(delta) =>
                (detailsWidth = Math.max(
                  260,
                  Math.min(maxDetailsWidth, effectiveDetailsWidth - delta),
                ))}
              onReset={() => (detailsWidth = 340)}
            />
          </div>
          <PresetRecordDetails row={selected} />
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .library {
    display: flex;
    flex-direction: column;
    min-height: 0;
    height: 100%;
    color: var(--text);
    background: var(--bg-panel);
  }
  header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--ui-space-3);
    padding: var(--ui-space-3) var(--ui-space-4);
    border-bottom: 1px solid var(--border);
  }
  header > div {
    flex: 1;
  }
  h1 {
    font-size: 1rem;
    margin: 0;
  }
  header p {
    margin: var(--ui-space-1) 0 0;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .filters {
    display: flex;
    flex-wrap: wrap;
    align-items: end;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3) var(--ui-space-4);
  }
  label {
    display: grid;
    gap: var(--ui-space-1);
    min-width: 100px;
    flex: 1;
    font-size: var(--inspector-font-meta);
  }
  label:first-child {
    flex: 2;
    min-width: 200px;
  }
  button,
  input,
  select {
    background: var(--bg-input);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 5px 7px;
    font: inherit;
    min-width: 0;
  }
  button {
    cursor: pointer;
    font-size: var(--inspector-font-meta);
  }
  button:disabled {
    opacity: 0.45;
    cursor: default;
  }
  button:hover:not(:disabled) {
    border-color: var(--accent);
  }
  .summary {
    padding: 0 var(--ui-space-4) var(--ui-space-2);
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .message {
    padding: var(--ui-space-4);
    font-size: var(--inspector-font-field);
  }
  .error {
    color: var(--danger);
  }
  .content {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 2px var(--details-width);
    flex: 1;
    min-height: 0;
    border-top: 1px solid var(--border);
  }
  .content.full {
    grid-template-columns: minmax(0, 1fr);
  }
  .table-panel {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    border-right: 1px solid var(--border);
  }
  .table-scroll {
    overflow: auto;
    flex: 1;
    min-height: 0;
  }
  table {
    border-collapse: separate;
    border-spacing: 0;
    table-layout: fixed;
    font-size: var(--inspector-font-meta);
    font-variant-numeric: tabular-nums;
  }
  th {
    position: sticky;
    top: 0;
    background: var(--bg-elevated);
    text-align: left;
    white-space: nowrap;
    z-index: 1;
  }
  .column-resize {
    position: absolute;
    right: 5px;
    top: 6px;
    bottom: 6px;
    display: flex;
  }
  .details-resize {
    display: flex;
    min-height: 0;
  }
  th,
  td {
    padding: var(--ui-space-2);
    border-bottom: 1px solid var(--border);
  }
  th {
    padding-right: 12px;
  }
  th,
  td {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  td button {
    display: block;
    border: 0;
    background: transparent;
    padding: 0;
    text-align: left;
    width: 100%;
    font: inherit;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  small {
    display: block;
    color: var(--text-muted);
    margin-top: 3px;
  }
  tr.selected {
    background: color-mix(in srgb, var(--accent) 16%, var(--bg-panel));
  }
  footer {
    display: flex;
    justify-content: end;
    align-items: center;
    gap: var(--ui-space-2);
    padding: var(--ui-space-2);
    font-size: var(--inspector-font-meta);
  }
  @media (max-width: 1000px) {
    .details-resize {
      display: none;
    }
    .content {
      grid-template-columns: minmax(0, 1fr);
      overflow: auto;
    }
    .table-scroll {
      max-height: 45vh;
    }
    .table-panel {
      flex: none;
      border-right: 0;
    }
    .library {
      overflow: auto;
    }
  }
</style>
