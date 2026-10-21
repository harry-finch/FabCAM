<script lang="ts">
  import { onMount, tick, untrack } from 'svelte';
  import { Search, Wrench, FileStack, Terminal, ArrowLeft, X } from '@lucide/svelte';
  import { paletteKey } from '../state/palette-keyboard';
  import { focusPaletteDestination } from '../state/palette-focus';
  import Modal from './Modal.svelte';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import { t, i18n } from '../i18n';
  import { bindPaletteShell, paletteBusy, type PaletteShell } from '../state/palette-shell';
  import { paletteDispatch, type ActionState } from '../state/palette-dispatch';
  import {
    commandCatalog,
    TOOL_ALIASES,
    TEMPLATE_KIND_ALIASES,
    type PaletteRecord,
  } from '../state/palette-catalog';
  import { buildSearchIndex, searchIndex, parseSearch } from '../state/palette-search';
  import { assignmentProblem } from '../state/palette-tool';
  import { selectOperationToolCommand } from '../state/operation-tool';
  import { effectiveModes } from '../state/tool_family';
  import type { ToolEntry } from '../state/project-types';
  let { shell }: { shell: PaletteShell } = $props();
  let open = $state(false),
    query = $state(''),
    cursor = $state(0);
  let restoreFocus = $state(true),
    error = $state(''),
    revision = $state(0);
  let detail = $state<PaletteRecord | null>(null);
  let offered: ActionState | undefined;
  let offeredId = '';

  let target: {
    epoch: number;
    historyVersion: number;
    opId: number | null;
    tool: ToolEntry;
    origin: 'inventory' | 'project';
  } | null = null;
  const index = $derived.by(() => {
    void workspace.version;
    void i18n.locale;
    const entries: PaletteRecord[] = commandCatalog();
    for (const origin of ['inventory', 'project'] as const) {
      for (const tool of origin === 'inventory'
        ? workspace.get().tool_inventory
        : project.data.tools) {
        entries.push({
          id: `tool:${origin}:${tool.id}`,
          category: 'tool',
          label: tool.name,
          origin,
          toolId: tool.id,
          aliases: `${TOOL_ALIASES[tool.kind]} ${tool.comment ?? ''} T${tool.id}`,
          diameter: tool.diameter,
          flutes: tool.flutes,
          angle: ['v_bit', 'cone', 'engraver'].includes(tool.kind) ? tool.tipAngleDeg : undefined,
          relevance: 0,
        });
      }
    }
    for (const template of workspace.get().operation_templates)
      entries.push({
        id: `template:${template.id}`,
        category: 'template',
        label: template.name,
        aliases: TEMPLATE_KIND_ALIASES[template.settings.kind],
        diameter: template.tool.diameter,
        action: 'templates',
        argument: template.id,
      });
    for (const recent of workspace.get().recent_projects)
      entries.push({
        id: `command:recent:${recent.path}`,
        category: 'command',
        label: `${t('palette.recent')}: ${recent.filename}`,
        aliases: recent.filename,
        action: 'recent',
        argument: recent.path,
      });
    return buildSearchIndex(entries);
  });
  const results = $derived(searchIndex(index, query));
  const selected = $derived(results[cursor]);
  const filters = $derived(parseSearch(query));
  const currentTool = $derived.by(() => {
    if (!detail) return undefined;
    void workspace.version;
    return (
      detail.origin === 'inventory' ? workspace.get().tool_inventory : project.data.tools
    ).find((tool) => tool.id === detail?.toolId);
  });
  const operation = $derived(
    project.data.operations.find((op) => op.id === project.sel.selectedOpId),
  );
  const assignReason = $derived.by(() => {
    return paletteBusy()
      ? t('palette.busy')
      : !project.documentActive
        ? t('palette.no_document')
        : (() => {
            const reason = assignmentProblem(
              operation,
              currentTool,
              effectiveModes(project.data.machine),
              true, // The shared selection command preserves the saved cutting values.
            );
            return reason ? t(reason) : undefined;
          })();
  });
  function actionState(record: PaletteRecord) {
    void revision;
    return record.action ? paletteDispatch.state(record.action, record.argument) : undefined;
  }
  function reason(record: PaletteRecord) {
    return record.category === 'tool'
      ? undefined
      : (actionState(record)?.reason ??
          (!actionState(record)
            ? t(
                record.action === 'drawing-cleanup'
                  ? 'uxc.palette_layers'
                  : ['templates', 'capture-template'].includes(record.action ?? '')
                    ? 'uxc.palette_operations'
                    : 'palette.unavailable',
              )
            : undefined));
  }
  function captureOffer() {
    const result = results[cursor];
    offeredId = result?.id ?? '';
    offered = result ? actionState(result) : undefined;
    target = null;
    if (result?.category === 'tool' && result.origin) {
      const tool = (
        result.origin === 'inventory' ? workspace.get().tool_inventory : project.data.tools
      ).find((tool) => tool.id === result.toolId);
      if (tool)
        target = {
          epoch: project.documentEpoch,
          historyVersion: project.historyVersion,
          opId: project.sel.selectedOpId,
          origin: result.origin,
          tool: JSON.parse(JSON.stringify(tool)),
        };
    }
  }
  $effect(() => {
    void query;
    void cursor;
    if (open) untrack(captureOffer);
  });
  onMount(() => {
    const unsubscribe = paletteDispatch.subscribe(() => revision++);
    const dispose = bindPaletteShell(shell);
    return () => {
      dispose();
      unsubscribe();
    };
  });
  export function show() {
    if (open || document.querySelector('[role="dialog"][aria-modal="true"]')) return;
    query = '';
    cursor = 0;
    detail = null;
    error = '';
    restoreFocus = true;
    open = true;
    captureOffer();
  }
  function dismiss() {
    open = false;
    void tick().then(() => {
      if (document.activeElement === document.body) void focusPaletteDestination();
    });
  }
  function move(next: number) {
    cursor = Math.max(0, Math.min(Math.min(results.length, 100) - 1, next));
    error = '';
    captureOffer();
    void tick().then(() =>
      document.getElementById(`palette-result-${cursor}`)?.scrollIntoView({ block: 'nearest' }),
    );
  }
  function toolStillCurrent() {
    if (
      !target ||
      target.epoch !== project.documentEpoch ||
      target.historyVersion !== project.historyVersion ||
      target.opId !== project.sel.selectedOpId
    )
      return false;
    const tool = (
      target.origin === 'inventory' ? workspace.get().tool_inventory : project.data.tools
    ).find((tool) => tool.id === target?.tool.id);
    return !!tool && JSON.stringify(tool) === JSON.stringify(target.tool);
  }
  async function execute(record = selected) {
    if (!record) return;
    if (record.id !== offeredId) {
      error = t('palette.changed');
      return;
    }
    if (record.category === 'tool') {
      if (!toolStillCurrent()) {
        error = t('palette.changed');
        return;
      }
      detail = record;
      void tick().then(() =>
        document.querySelector<HTMLElement>('.command-palette .back')?.focus(),
      );
      return;
    }
    if (!record.action || !offered || reason(record)) {
      error = reason(record) ?? t('palette.unavailable');
      return;
    }
    const current = actionState(record);
    if (!current || current.token !== offered.token) {
      error = t('palette.changed');
      return;
    }
    restoreFocus = false;
    open = false;
    try {
      const dispatch = paletteDispatch.execute(record.action, offered, record.argument);
      void focusPaletteDestination();
      const result = await dispatch;
      if (result !== 'done') project.setError(t('palette.changed'));
      if (document.activeElement === document.body) void focusPaletteDestination();
    } catch (e) {
      project.setError(e instanceof Error ? e.message : String(e));
    }
  }
  function assign() {
    if (!toolStillCurrent()) {
      error = t('palette.changed');
      return;
    }
    if (assignReason || !target || !operation) {
      error = assignReason ?? t('palette.changed');
      return;
    }
    project.history.exec(
      selectOperationToolCommand(operation.id, 'toolId', target.tool, target.origin),
      project.target(),
    );
    restoreFocus = false;
    open = false;
    shell.navigate('project');
    shell.sidebar('operations');
    void focusPaletteDestination();
  }
  function reviewOperation() {
    restoreFocus = false;
    open = false;
    shell.navigate('project');
    shell.sidebar('operations');
    void focusPaletteDestination();
  }
  function key(event: KeyboardEvent) {
    const action = paletteKey(event, !detail, (event.target as HTMLElement)?.tagName === 'INPUT');
    if (action === 'pass') return;
    event.stopPropagation();
    if (action === 'ignore') return;
    event.preventDefault();
    if (action === 'up' || action === 'down') move(cursor + (action === 'down' ? 1 : -1));
    if (action === 'run') void execute();
    if (action === 'close') dismiss();
  }
</script>

{#if open}
  <Modal
    onClose={dismiss}
    {restoreFocus}
    width="min(640px, 94vw)"
    maxHeight="80vh"
    initialFocus="#palette-query"
    ariaLabelledBy="palette-title"
    modalClass="command-palette"
  >
    <!-- Keyboard handling is shared by the search and explicit tool controls. -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="palette" onkeydown={key}>
      <header>
        <h2 id="palette-title">{t('palette.title')}</h2>
        <button class="dlg-close" aria-label={t('common.close')} onclick={dismiss}
          ><X size={16} /></button
        >
      </header>
      {#if detail && currentTool}
        <button
          class="back"
          onclick={() => {
            detail = null;
            error = '';
            void tick().then(() => document.getElementById('palette-query')?.focus());
          }}><ArrowLeft size={14} />{t('palette.back')}</button
        >
        <section class="tool-detail">
          <h3>{currentTool.name}</h3>
          <p>
            {t(detail.origin === 'inventory' ? 'palette.inventory' : 'palette.project')} · T{currentTool.id}
            · {currentTool.kind.replaceAll('_', ' ')}
          </p>
          <dl>
            <dt>{t('palette.diameter')}</dt>
            <dd>{currentTool.diameter} mm</dd>
            <dt>{t('palette.flutes')}</dt>
            <dd>{currentTool.flutes}</dd>
            {#if currentTool.tipAngleDeg !== undefined}<dt>{t('palette.angle')}</dt>
              <dd>{currentTool.tipAngleDeg}°</dd>{/if}
          </dl>
          {#if assignReason}<p class="reason">{assignReason}</p>{:else}<p>
              {t(operation?.appliedCutting ? 'palette.manual' : 'palette.legacy')}
            </p>{/if}
          {#if operation}<p>{operation.name}</p>{/if}
          <div class="actions">
            <button disabled={!!assignReason} onclick={assign}>{t('palette.assign')}</button><button
              disabled={!operation}
              onclick={reviewOperation}>{t('palette.review_operation')}</button
            >
          </div>
        </section>
      {:else}
        <div class="search">
          <Search size={16} /><input
            id="palette-query"
            aria-label={t('palette.search')}
            placeholder={t('palette.search')}
            bind:value={query}
            oninput={() => {
              cursor = 0;
              error = '';
            }}
            role="combobox"
            aria-expanded="true"
            aria-controls="palette-results"
            aria-autocomplete="list"
            aria-activedescendant={selected ? `palette-result-${cursor}` : undefined}
          />
        </div>
        {#if filters.diameter !== undefined || filters.flutes !== undefined || filters.angle !== undefined}<p
            class="filters"
          >
            {t('palette.filters')}
            {filters.diameter !== undefined ? `Ø ${filters.diameter} mm` : ''}
            {filters.flutes !== undefined ? `${filters.flutes} ${t('palette.flutes')}` : ''}
            {filters.angle !== undefined ? `${filters.angle}°` : ''}
          </p>{/if}
        <div id="palette-results" role="listbox" aria-label={t('palette.results')} class="results">
          {#each results.slice(0, 100) as result, i (result.id)}
            <!-- Rows remain selectable to explain unavailable actions. Search owns keyboard focus. -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <div
              role="option"
              tabindex="-1"
              id="palette-result-{i}"
              aria-selected={cursor === i}
              aria-disabled={!!reason(result)}
              class:chosen={cursor === i}
              class:unavailable={!!reason(result)}
              onclick={() => {
                move(i);
                void execute(result);
              }}
            >
              {#if result.category === 'tool'}<Wrench
                  size={15}
                />{:else if result.category === 'template'}<FileStack size={15} />{:else}<Terminal
                  size={15}
                />{/if}
              <div class="identity">
                <span>{result.label}</span>{#if reason(result)}<small>{reason(result)}</small
                  >{:else if result.category === 'tool'}<small
                    >{t(result.origin === 'inventory' ? 'palette.inventory' : 'palette.project')} · T{result.toolId}
                    · Ø {result.diameter} mm · {result.flutes}
                    {t('palette.flutes')}</small
                  >{/if}
              </div>
              <small class="category">{result.shortcut ?? t(`palette.${result.category}`)}</small>
            </div>
          {:else}<p class="empty">{t('palette.no_results')}</p>{/each}
        </div>
      {/if}
      {#if error}<p class="error" role="alert">{error}</p>{/if}
      <footer>
        {t('palette.keyboard')}
        {#if results.length > 100}<span
            >{t('palette.result_count', { shown: 100, total: results.length })}</span
          >{/if}
      </footer>
    </div>
  </Modal>
{/if}

<style>
  header,
  .search,
  .actions,
  .back {
    display: flex;
    align-items: center;
    gap: var(--space-2, 8px);
  }
  header {
    justify-content: space-between;
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
  }
  h2,
  h3 {
    font-size: 0.9rem;
    margin: 0;
    font-weight: 600;
  }
  .search {
    padding: 8px 12px;
    color: var(--text-muted);
  }
  input {
    flex: 1;
    min-width: 0;
    height: 30px;
  }
  .results {
    max-height: min(48vh, 360px);
    overflow: auto;
    border-top: 1px solid var(--border);
  }
  [role='option'] {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 12px;
    cursor: pointer;
    border-left: 2px solid transparent;
  }
  [role='option']:hover {
    background: var(--bg-elevated);
  }
  .chosen {
    background: var(--bg-elevated);
    border-left-color: var(--accent) !important;
  }
  .identity {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow-wrap: anywhere;
  }
  small,
  footer,
  p {
    color: var(--text-muted);
    font-size: 0.75rem;
  }
  .unavailable .identity > span {
    color: var(--text-muted);
  }
  .category {
    flex: none;
    font-size: 0.7rem;
  }
  footer {
    padding: 8px 12px;
    border-top: 1px solid var(--border);
  }
  .empty,
  .filters,
  .error {
    padding: 8px 12px;
    margin: 0;
  }
  .error,
  .reason {
    color: var(--warning-text, var(--text));
  }
  .tool-detail {
    padding: 12px;
  }
  .back {
    margin: 8px 12px;
  }
  dl {
    display: grid;
    grid-template-columns: 1fr 1fr;
    font-size: 0.8rem;
    gap: 6px;
    max-width: 320px;
  }
  dd {
    margin: 0;
  }
</style>
