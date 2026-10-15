<script lang="ts">
  import { t } from '../i18n';
  import { KIND_DISPLAY_LABELS } from '../state/tool_family';
  import type { ToolKind } from '../state/project.svelte';
  import type { ToolTableView } from '../state/tool_table';
  let {
    view = $bindable(),
    total,
    shown,
    onFilterChange,
    clearFilters,
  }: {
    view: ToolTableView;
    total: number;
    shown: number;
    onFilterChange: () => void;
    clearFilters: () => void;
  } = $props();
  const kindLabels = KIND_DISPLAY_LABELS;
  const kindOptions = Object.keys(kindLabels) as ToolKind[];
  const filtersActive = $derived(view.query.trim() !== '' || view.kind !== '' || view.mode !== '');
</script>

<div class="table-filters">
  <input
    type="text"
    class="tc-search"
    placeholder={t('tools.search.placeholder')}
    value={view.query}
    oninput={(e) => {
      view.query = (e.currentTarget as HTMLInputElement).value;
      onFilterChange();
    }}
    title={t('tools.search.title')}
  />
  <label class="tc-filter">
    <span>{t('tools.filter.kind')}</span>
    <select
      value={view.kind}
      onchange={(e) => {
        view.kind = (e.currentTarget as HTMLSelectElement).value as typeof view.kind;
        onFilterChange();
      }}
    >
      <option value="">{t('tools.filter.all')}</option>
      {#each kindOptions as k (k)}
        <option value={k}>{kindLabels[k]}</option>
      {/each}
    </select>
  </label>
  <label class="tc-filter" title={t('tools.filter.runs_on.title')}>
    <span>{t('tools.filter.runs_on')}</span>
    <select
      value={view.mode}
      onchange={(e) => {
        view.mode = (e.currentTarget as HTMLSelectElement).value as typeof view.mode;
        onFilterChange();
      }}
    >
      <option value="">{t('tools.filter.runs_on.any')}</option>
      <option value="mill">{t('tools.filter.runs_on.mill')}</option>
      <option value="laser">{t('tools.filter.runs_on.laser')}</option>
      <option value="drag">{t('tools.filter.runs_on.drag')}</option>
      <option value="plasma">{t('tools.filter.runs_on.plasma')}</option>
    </select>
  </label>
  {#if filtersActive}
    <button type="button" class="tc-clear" onclick={clearFilters}>{t('tools.filter.clear')}</button>
  {/if}
  <span class="tc-count"
    >{shown === total
      ? total === 1
        ? t('tools.count.one', { count: total })
        : t('tools.count.many', { count: total })
      : t('tools.count.filtered', { shown: shown, total: total })}</span
  >
</div>

<style>
  .table-filters {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.35rem 0.7rem;
    background: var(--bg-elevated);
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    font-size: 0.78rem;
    flex-wrap: wrap;
  }
  .tc-search {
    width: 14rem;
    max-width: 40vw;
  }
  .tc-filter {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    color: var(--text-muted);
  }
  .tc-clear {
    background: var(--bg-panel);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0.15rem 0.5rem;
    font-size: 0.72rem;
    cursor: pointer;
  }
  .tc-count {
    margin-left: auto;
    color: var(--text-muted);
  }
  input,
  select {
    background: var(--bg-input);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 4px 6px;
    font-size: 0.78rem;
    min-width: 0;
    box-sizing: border-box;
  }
</style>
