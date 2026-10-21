<script lang="ts">
  import { nextDrawingOperationId } from '../state/drawing';
  import { paletteDispatch } from '../state/palette-dispatch';
  import { paletteContext } from '../state/palette-shell';
  import { onMount, tick } from 'svelte';
  import { MoreHorizontal, TriangleAlert } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import {
    captureTemplate,
    instantiateTemplate,
    type OperationTemplate,
  } from '../state/operation-templates';
  import {
    searchTemplates,
    templateBindingProblem,
    templateToolDifferences,
  } from '../state/operation-template-ui';
  import { workingToolChoices } from '../state/tool_inventory';
  import { addOperationCommand, addToolCommand } from '../state/commands';
  import { computeFootprint } from '../sim/footprint';
  import { t, type MsgKey } from '../i18n';
  import Modal from './Modal.svelte';
  import TemplateParameterPreview from './TemplateParameterPreview.svelte';
  import './op_properties/op-properties.css';

  let { onCreated }: { onCreated: () => void } = $props();
  let open = $state(false);
  let view = $state<'browse' | 'save' | 'rename' | 'delete'>('browse');
  let selected = $state('');
  let query = $state('');
  let name = $state('');
  let source = $state('');
  let toolId = $state('');
  let finishId = $state('');
  let error = $state('');
  let epoch = $state(0);
  let captureId = $state<number | null>(null);
  let managingId = $state('');
  let returnToLibrary = $state(true);
  let removed = $state<OperationTemplate | null>(null);
  let busy = $state(false);
  let content = $state<HTMLDivElement>();
  const entries = $derived.by(() => {
    void workspace.version;
    return workspace.get().operation_templates;
  });
  const filtered = $derived(searchTemplates(entries, query));
  const template = $derived(entries.find((entry) => entry.id === selected));
  const managing = $derived(entries.find((entry) => entry.id === managingId));
  const original = $derived(project.data.operations.find((op) => op.id === captureId));
  const current = $derived(
    project.data.operations.find((op) => op.id === project.sel.selectedOpId),
  );
  const tools = $derived.by(() => {
    void workspace.version;
    return workingToolChoices(project.data.tools, workspace.get().tool_inventory).filter(
      (tool) => tool.kind === 'endmill' && !tool.whirl,
    );
  });
  const drawingIds = $derived(
    (project.transformedImport?.object_meta ?? []).map((object) => object.id),
  );
  const availableIds = $derived(
    (project.geometryView?.object_meta ?? []).map((object) => object.id),
  );
  const selectionIds = $derived([...project.sel.selectedObjects]);
  const binding = $derived({
    nextId: nextDrawingOperationId(project.data.operations, project.data.drawing),
    toolId: Number(toolId),
    finishToolId: finishId ? Number(finishId) : undefined,
    tools,
    objectIds: source === 'selected' ? selectionIds : source === 'drawing' ? drawingIds : [],
    availableObjectIds: availableIds,
    mode: project.data.machine.mode,
    ...(source === 'stock' && (project.stockSizingImport || project.data.stock.mode === 'manual')
      ? {
          stockZone: computeFootprint(
            project.stockSizingImport,
            project.data.stock,
            project.data.machine.workArea,
          ),
        }
      : {}),
  });
  const problem = $derived(
    epoch !== project.documentEpoch || !project.documentActive
      ? 'templates.document_changed'
      : templateBindingProblem(template, binding),
  );
  const differences = $derived(template ? templateToolDifferences(template, binding) : []);
  const titleKey = $derived(
    view === 'save'
      ? 'templates.capture'
      : view === 'rename'
        ? 'templates.rename'
        : view === 'delete'
          ? 'templates.remove'
          : 'templates.title',
  );

  async function focusView() {
    await tick();
    content?.querySelector<HTMLElement>('[data-initial-focus]')?.focus();
  }
  function reset() {
    epoch = project.documentEpoch;
    selected = '';
    query = '';
    source = '';
    toolId = '';
    finishId = '';
    error = '';
    removed = null;
    view = 'browse';
    managingId = '';
    open = true;
  }
  export function show(templateId?: string) {
    reset();
    if (templateId && entries.some((entry) => entry.id === templateId)) choose(templateId);
    void focusView();
  }
  export function saveOperation(id: number) {
    reset();
    returnToLibrary = false;
    startSave(id);
  }
  function captureProblem() {
    if (!project.documentActive) return t('palette.no_document');
    if (!current) return t('palette.select_operation');
    try {
      captureTemplate(current, project.data.tools, current.name, 'palette-review');
      return undefined;
    } catch (e) {
      return message(e);
    }
  }
  onMount(() => {
    const browse = paletteDispatch.register('templates', {
      state: (id) => {
        const context = paletteContext();
        return {
          ...context,
          reason:
            context.reason ??
            (!project.documentActive
              ? t('palette.no_document')
              : id && !entries.some((entry) => entry.id === id)
                ? t('palette.changed')
                : undefined),
        };
      },
      run: (id) => show(id),
    });
    const capture = paletteDispatch.register('capture-template', {
      state: () => {
        const context = paletteContext();
        return {
          ...context,
          reason: context.reason ?? captureProblem(),
        };
      },
      run: () => saveOperation(current!.id),
    });
    return () => {
      browse();
      capture();
    };
  });
  function startSave(id: number) {
    captureId = id;
    name = project.data.operations.find((op) => op.id === id)?.name ?? '';
    view = 'save';
    error = '';
    void focusView();
  }
  function saveCurrent() {
    if (!current) return;
    returnToLibrary = true;
    startSave(current.id);
  }
  function choose(id: string) {
    selected = id;
    // Current explicit canvas selection is a useful geometry default. Tools remain explicit.
    source =
      entries.find((entry) => entry.id === id)?.settings.kind !== 'planar_surface' &&
      selectionIds.length > 0 &&
      selectionIds.every((id) => availableIds.includes(id))
        ? 'selected'
        : '';
    toolId = '';
    finishId = '';
    error = '';
  }
  function manage(id: string, action: 'rename' | 'delete') {
    managingId = id;
    name = entries.find((entry) => entry.id === id)?.name ?? '';
    returnToLibrary = true;
    view = action;
    error = '';
    void focusView();
  }
  function cancel() {
    if (busy) return;
    if (view !== 'browse' && returnToLibrary) {
      view = 'browse';
      error = '';
      void focusView();
    } else open = false;
  }
  function message(e: unknown) {
    const key = e instanceof Error ? e.message : '';
    return key.startsWith('templates.') ? t(key as MsgKey) : t('templates.storage_error');
  }
  async function persist(action: () => void, after: () => void) {
    busy = true;
    error = '';
    try {
      action();
      await workspace.save();
      after();
    } catch (e) {
      error = message(e);
    } finally {
      busy = false;
    }
  }
  async function save() {
    await persist(
      () => {
        if (epoch !== project.documentEpoch || !original)
          throw new Error('templates.document_changed');
        const saved = captureTemplate(original, project.data.tools, name, crypto.randomUUID());
        workspace.upsertOperationTemplate(saved);
        selected = saved.id;
      },
      () => {
        if (returnToLibrary) {
          view = 'browse';
          query = '';
          choose(selected);
          void focusView();
        } else open = false;
      },
    );
  }
  async function renameEntry() {
    if (!managing) return;
    const entry = managing;
    await persist(
      () => workspace.upsertOperationTemplate({ ...entry, name: name.trim() }),
      () => {
        view = 'browse';
        query = '';
        void focusView();
      },
    );
  }
  async function remove() {
    if (!managing) return;
    const entry = managing;
    await persist(
      () => {
        removed = JSON.parse(JSON.stringify(entry));
        workspace.removeOperationTemplate(entry.id);
        if (selected === entry.id) choose('');
      },
      () => {
        view = 'browse';
        void focusView();
      },
    );
  }
  async function restoreRemoved() {
    if (!removed) return;
    const entry = removed;
    await persist(
      () => workspace.upsertOperationTemplate(entry),
      () => {
        choose(entry.id);
        query = '';
        removed = null;
      },
    );
  }
  function apply() {
    error = '';
    try {
      if (problem) throw new Error(problem);
      const op = instantiateTemplate(template!, binding);
      project.history.beginTransaction('Create operation from template');
      try {
        for (const tool of tools.filter(
          (tool) =>
            (tool.id === op.toolId || ('finishToolId' in op && tool.id === op.finishToolId)) &&
            !project.data.tools.some((t) => t.id === tool.id),
        ))
          project.history.exec(addToolCommand(tool), project.target());
        project.history.exec(addOperationCommand(op), project.target());
        project.history.commitTransaction();
      } catch (e) {
        project.history.cancelTransaction(project.target());
        throw e;
      }
      project.sel.selectedOpId = op.id;
      onCreated();
      open = false;
    } catch (e) {
      error = message(e);
    }
  }
  function onMenuKey(e: KeyboardEvent) {
    const menu = (e.target as HTMLElement).closest('details.template-menu');
    if (e.key === 'Escape' && menu?.hasAttribute('open')) {
      e.stopPropagation();
      e.preventDefault();
      menu.removeAttribute('open');
      menu.querySelector<HTMLElement>('summary')?.focus();
    }
  }
  function dismissMenus(e: MouseEvent) {
    if ((e.target as HTMLElement)?.closest?.('.template-menu')) return;
    content
      ?.querySelectorAll('details.template-menu[open]')
      .forEach((menu) => menu.removeAttribute('open'));
  }
</script>

<svelte:window onclick={dismissMenus} />
{#if open}
  <Modal onClose={cancel} width="520px" maxHeight="85vh" ariaLabelledBy="operation-templates-title">
    <div bind:this={content} class="props templates-body" onkeydown={onMenuKey} role="presentation">
      <header>
        <h2 id="operation-templates-title">{t(titleKey)}</h2>
        <button
          type="button"
          class="dlg-close"
          disabled={busy}
          aria-label={t('templates.close')}
          onclick={cancel}>×</button
        >
      </header>
      {#if view === 'browse'}
        <div class="library-toolbar">
          <input
            type="search"
            data-initial-focus
            aria-label={t('templates.search')}
            placeholder={t('templates.search')}
            bind:value={query}
          />
          <button
            type="button"
            class="inspector-action"
            disabled={busy || !current}
            title={!current ? t('templates.no_selected_op') : t('templates.save_current')}
            onclick={saveCurrent}>{t('templates.save_current')}</button
          >
        </div>
        {#if !current}<p class="hint">{t('templates.no_selected_op')}</p>{/if}
        <section aria-label={t('templates.library')}>
          <h3>{t('templates.library')}</h3>
          {#if !entries.length}<p class="hint empty">{t('templates.empty')}</p>
          {:else if !filtered.length}<p class="hint empty">{t('templates.no_matches')}</p>{/if}
          <ul class="template-list">
            {#each filtered as entry (entry.id)}
              <li class:selected={entry.id === selected}>
                <button
                  type="button"
                  class="template-pick"
                  aria-pressed={entry.id === selected}
                  disabled={busy}
                  onclick={() => choose(entry.id)}
                >
                  <span class="template-name">{entry.name}</span>
                  <span class="template-context"
                    >{t(`ops.kind.${entry.settings.kind}`)} · {t('templates.source_tool', {
                      diameter: entry.tool.diameter,
                    })}</span
                  >
                </button>
                <details class="template-menu">
                  <summary
                    title={t('templates.manage', { name: entry.name })}
                    aria-label={t('templates.manage', { name: entry.name })}
                    ><MoreHorizontal size={16} /></summary
                  >
                  <div class="template-menu-actions">
                    <button type="button" disabled={busy} onclick={() => manage(entry.id, 'rename')}
                      >{t('templates.rename')}</button
                    >
                    <button type="button" disabled={busy} onclick={() => manage(entry.id, 'delete')}
                      >{t('templates.remove')}</button
                    >
                  </div>
                </details>
              </li>
            {/each}
          </ul>
          {#if removed}<button
              type="button"
              class="inspector-action"
              disabled={busy}
              onclick={restoreRemoved}>{t('templates.undo_remove')}</button
            >{/if}
        </section>
        {#if template}
          <section class="reuse" aria-label={t('templates.bind')}>
            <h3>{t('templates.bind')}</h3>
            <p class="selected-name">{template.name}</p>
            <p class="hint">
              {template.settings.kind === 'planar_surface'
                ? t('templates.reuse_surface')
                : t('templates.reuse_explanation')}
            </p>
            <label class="row"
              ><span>{t('templates.geometry')}</span><select bind:value={source}>
                <option value="">{t('templates.geometry_placeholder')}</option>
                {#if template.settings.kind === 'planar_surface'}<option value="stock"
                    >{t('templates.stock')}</option
                  >
                {:else}<option value="selected" disabled={!selectionIds.length}
                    >{t('templates.selection', { count: selectionIds.length })}</option
                  >
                  <option value="drawing" disabled={!drawingIds.length}
                    >{t('templates.drawing', { count: drawingIds.length })}</option
                  >{/if}
              </select></label
            >
            <label class="row"
              ><span>{t('templates.tool')}</span><select
                bind:value={toolId}
                title={t('v11.tool_auto')}
              >
                <option value="">{t('templates.tool_placeholder')}</option>
                {#each tools as tool (tool.id)}<option value={String(tool.id)}
                    >{tool.name} · Ø{tool.diameter} {t('machine.unit.mm')}</option
                  >{/each}
              </select></label
            >
            {#if template.finishTool}<label class="row"
                ><span>{t('templates.finish_tool')}</span><select
                  bind:value={finishId}
                  title={t('v11.tool_auto')}
                >
                  <option value="">{t('templates.tool_placeholder')}</option>
                  {#each tools as tool (tool.id)}<option
                      value={String(tool.id)}
                      disabled={String(tool.id) === toolId}
                      >{tool.name} · Ø{tool.diameter} {t('machine.unit.mm')}</option
                    >{/each}
                </select></label
              >{/if}
            {#key selected}<TemplateParameterPreview {template} />{/key}
            <p class="hint">{t('templates.review_values')}</p>
            {#each differences as difference (difference.role)}<p class="context-difference">
                <TriangleAlert size={12} aria-hidden="true" />
                {t('templates.diameter_difference', {
                  role: t(difference.role === 'rough' ? 'templates.tool' : 'templates.finish_tool'),
                  source: difference.sourceDiameter,
                  target: difference.targetDiameter,
                })}
              </p>{/each}
          </section>
        {/if}
        {#if problem}<p class="hint requirement" aria-live="polite">{t(problem)}</p>{/if}
      {:else if view === 'save'}
        <p class="source-name">
          {t('templates.source_operation')}:
          <strong>{original?.name ?? t('templates.no_selected_op')}</strong>
        </p>
        <p class="hint">{t('templates.capture_explanation')}</p>
        <label class="row"
          ><span>{t('templates.name')}</span><input
            data-initial-focus
            maxlength="100"
            bind:value={name}
          /></label
        >
      {:else if view === 'rename'}
        <p class="source-name">{managing?.name}</p>
        <label class="row"
          ><span>{t('templates.name')}</span><input
            data-initial-focus
            maxlength="100"
            bind:value={name}
          /></label
        >
      {:else}
        <p class="source-name">{managing?.name}</p>
        <p>{t('templates.delete_confirmation')}</p>
      {/if}
      {#if error}<p role="alert" class="warn-chip">{error}</p>{/if}
      <footer>
        <button
          type="button"
          class="inspector-action"
          disabled={busy}
          data-initial-focus={view === 'delete' ? '' : undefined}
          onclick={cancel}>{t('templates.cancel')}</button
        >
        {#if view === 'browse'}<button
            type="button"
            class="inspector-action primary"
            disabled={busy || !!problem}
            onclick={apply}>{t('templates.add')}</button
          >
        {:else if view === 'save'}<button
            type="button"
            class="inspector-action primary"
            disabled={busy || !name.trim() || !original || epoch !== project.documentEpoch}
            onclick={save}>{t('templates.save')}</button
          >
        {:else if view === 'rename'}<button
            type="button"
            class="inspector-action primary"
            disabled={busy || !name.trim() || !managing}
            onclick={renameEntry}>{t('templates.rename')}</button
          >
        {:else}<button
            type="button"
            class="inspector-action delete-action"
            disabled={busy || !managing}
            onclick={remove}>{t('templates.remove')}</button
          >{/if}
      </footer>
    </div>
  </Modal>
{/if}

<style>
  .templates-body {
    padding: 12px;
    position: relative;
  }
  header,
  .library-toolbar,
  footer {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  header {
    justify-content: space-between;
    margin-bottom: 12px;
  }
  h2 {
    font-size: 0.95rem;
    margin: 0;
  }
  .library-toolbar input {
    min-width: 0;
    flex: 1;
  }
  .library-toolbar button {
    flex: none;
  }
  section {
    margin-top: 12px;
  }
  .template-list {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 180px;
    overflow: auto;
  }
  .template-list li {
    display: flex;
    align-items: center;
    gap: 4px;
    border-left: 2px solid transparent;
  }
  .template-list li.selected {
    border-left-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, var(--bg-panel));
  }
  .template-pick {
    flex: 1;
    min-width: 0;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding: 6px;
    background: transparent;
    color: var(--text);
    border: 0;
    cursor: pointer;
  }
  .template-name {
    font-weight: 600;
    overflow-wrap: anywhere;
  }
  .template-context {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .template-pick:hover {
    background: var(--bg-elevated);
  }
  .template-menu {
    flex: none;
  }
  .template-menu summary {
    cursor: pointer;
    display: flex;
    padding: 4px;
    color: var(--text-muted);
  }
  .template-menu summary::-webkit-details-marker {
    display: none;
  }
  .template-menu-actions {
    position: absolute;
    right: 16px;
    z-index: var(--z-dropdown);
    display: flex;
    flex-direction: column;
    padding: 4px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
  }
  .template-menu-actions button {
    background: transparent;
    color: var(--text);
    border: 0;
    text-align: left;
    padding: 6px 10px;
    cursor: pointer;
    font-size: var(--inspector-font-field);
  }
  .template-menu-actions button:hover {
    background: var(--bg-panel);
  }
  button:focus-visible,
  summary:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
  .reuse {
    border-top: 1px solid var(--border);
    padding-top: 8px;
  }
  .selected-name,
  .source-name {
    font-size: var(--inspector-font-field);
    margin: 4px 0 8px;
    overflow-wrap: anywhere;
  }
  .selected-name {
    font-weight: 600;
  }
  .context-difference :global(svg) {
    vertical-align: middle;
  }
  .context-difference {
    color: var(--warn);
    font-size: var(--inspector-font-meta);
    margin: 6px 0;
  }
  .requirement {
    margin-bottom: 0;
  }
  footer {
    position: sticky;
    bottom: 0;
    z-index: var(--z-anchor);
    background: var(--bg-panel);
    justify-content: flex-end;
    margin-top: 12px;
    padding-top: 8px;
    border-top: 1px solid var(--border);
  }
  .delete-action {
    color: var(--error);
  }
</style>
