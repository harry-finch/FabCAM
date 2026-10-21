<script lang="ts">
  import { withLocalIdentity } from '../state/cutting-identities';
  import {
    project,
    type ToolEntry,
    type ToolKind,
    type HolderShape,
  } from '../state/project.svelte';
  import { untrack } from 'svelte';
  import { t } from '../i18n';
  import Modal from './Modal.svelte';
  import { DialogDraft } from './dialog-draft.svelte';
  import * as fileOps from '../services/file_ops';
  import {
    effectiveModes,
    KIND_DISPLAY_LABELS,
    toolCompatibleWithAnyMode,
  } from '../state/tool_family';
  import { workspace } from '../state/workspace.svelte';
  import { seedInventoryFromProject, syncStockedFromInventory } from '../state/tool_inventory';
  import { isAutoToolName, suggestToolName } from '../state/tool_naming';
  import ToolRowExpandedEditor from './ToolRowExpandedEditor.svelte';
  import ToolLibraryFilters from './ToolLibraryFilters.svelte';
  import ToolRetainedFields from './ToolRetainedFields.svelte';
  import ToolDetailBasics from './ToolDetailBasics.svelte';
  import ToolCopyComparison from './ToolCopyComparison.svelte';
  import ToolApplyReview from './ToolApplyReview.svelte';
  import ToolDimensionDiagram from './ToolDimensionDiagram.svelte';
  import { Plus, Trash2 } from '@lucide/svelte';
  import { deepEqual } from './dialog-draft';
  import { changeToolKind, toolLibraryImpacts } from '../state/tool-library-review';
  import { stockTool } from '../state/tool_inventory';
  import {
    applyToolTableView,
    EMPTY_TOOL_VIEW,
    pageOfTool,
    paginateToolRows,
    type ToolSortKey,
    type ToolTableView,
  } from '../state/tool_table';
  import { defaultToolForMode } from '../state/tool_mode_defaults';
  import { applyPresetPatch } from '../state/tool_presets';
  import ToolCalibrationDialog from './ToolCalibrationDialog.svelte';
  import { rowInvalid, fieldApplies } from '../state/tool_validation';

  interface Props {
    open: boolean;
    onClose: () => void;
    /// Render as a first-class tab panel instead of a modal: no Modal
    /// wrapper, no × / Cancel (the component stays mounted across tab
    /// switches, so an in-progress draft survives), footer becomes
    /// Apply / Revert.
    embedded?: boolean;
    /// Project copies and local inventory are distinct stores. Inventory Apply
    /// reviews existing same-ID propagation; IDs are not physical identities.
    source?: 'project' | 'inventory';
  }
  let { open, onClose, embedded = false, source = 'project' }: Props = $props();
  const active = $derived(open || embedded);

  const isInventory = $derived(source === 'inventory');
  /// The list this editor seeds from / commits to.
  const backingTools = $derived.by(() => {
    if (source === 'inventory') {
      void workspace.version;
      return workspace.get().tool_inventory;
    }
    return project.data.tools;
  });

  const dd = new DialogDraft<ToolEntry[]>();
  const draft = $derived(dd.draft ?? []);
  let selectedId = $state<number | null>(null);
  let section = $state<'geometry' | 'mounting' | 'availability' | 'advanced'>('geometry');
  const selectedIndex = $derived(draft.findIndex((t) => t.id === selectedId));
  const selectedTool = $derived(draft[selectedIndex]);
  let baseline = $state<ToolEntry[]>([]);
  let baselineStocked = $state<ToolEntry[]>([]);
  let deleting = $state<number | null>(null);
  let pendingImport = $state<ToolEntry[] | null>(null);
  let importing = $state(false);
  let reviewToken = $state<string | null>(null);
  const conflict = $derived(
    dd.isDirty &&
      (!deepEqual(backingTools, baseline) ||
        (isInventory && !deepEqual(project.data.tools, baselineStocked))),
  );
  const currentToken = $derived(
    JSON.stringify([draft, backingTools, project.data.tools, project.data.operations, source]),
  );
  const impacts = $derived(
    toolLibraryImpacts(draft, isInventory ? project.data.tools : baseline, project.data.operations),
  );
  const reviewCurrent = $derived(reviewToken !== null && reviewToken === currentToken);
  const removedReferences = $derived(
    !isInventory
      ? project.data.operations.filter((op) => !draft.some((t) => t.id === op.toolId))
      : [],
  );
  const savedProfiles = $derived.by(() => {
    void workspace.version;
    return workspace.get().machine_profiles;
  });
  const selectedUses = $derived(project.data.operations.filter((op) => op.toolId === selectedId));
  const selectedCopies = $derived(
    savedProfiles.filter((p) => p.tools.some((t) => t.id === selectedId)),
  );
  const selectedStocked = $derived(project.data.tools.find((t) => t.id === selectedId));
  const cannotRemoveStocked = $derived(
    dd.isDirty || !selectedStocked || selectedUses.length > 0 || project.data.tools.length <= 1,
  );

  function selectTool(id: number) {
    selectedId = id;
    deleting = null;
  }
  function navigate(event: KeyboardEvent) {
    const host = event.currentTarget as HTMLElement;
    const buttons = Array.from(
      host.parentElement?.querySelectorAll<HTMLButtonElement>('button[data-tool-id]') ?? [],
    );
    const i = buttons.indexOf(host as HTMLButtonElement);
    const next =
      event.key === 'ArrowDown'
        ? Math.min(i + 1, buttons.length - 1)
        : event.key === 'ArrowUp'
          ? Math.max(i - 1, 0)
          : event.key === 'Home'
            ? 0
            : event.key === 'End'
              ? buttons.length - 1
              : -1;
    if (next >= 0) {
      event.preventDefault();
      buttons[next]?.focus();
    }
  }
  async function importTools(mode: 'replace' | 'add') {
    if (importing || conflict) return;
    importing = true;
    const token = currentToken;
    try {
      const next = await fileOps.importToolset(mode, $state.snapshot(draft));
      if (!next) return;
      if (currentToken !== token) {
        project.setError(
          'Tool state changed during import. Review current changes and import again.',
        );
        return;
      }
      if (mode === 'replace') pendingImport = next;
      else dd.draft = next.map((t) => withLocalIdentity(t, 'tool'));
    } finally {
      importing = false;
    }
  }
  function acceptImport() {
    if (!pendingImport || conflict) return;
    dd.draft = pendingImport.map((t) => withLocalIdentity(t, 'tool'));
    selectedId = pendingImport[0]?.id ?? null;
    pendingImport = null;
    clearFilters();
  }
  function addToMachine() {
    if (dd.isDirty || !selectedTool || !toolCompatibleWithAnyMode(selectedTool.kind, machineModes))
      return;
    const copy = stockTool($state.snapshot(selectedTool), $state.snapshot(project.data.tools));
    if (copy) project.addTool(copy);
  }
  function removeFromMachine() {
    if (dd.isDirty || !selectedStocked || selectedUses.length || project.data.tools.length <= 1)
      return;
    project.removeTool(selectedStocked.id);
  }
  let showIncompatible = $state(false);
  let calibratingIdx = $state<number | null>(null);
  function applyCalibration(idx: number, wearOffsetMm: number, dateIso: string) {
    dd.draft = draft.map((t, i) =>
      i === idx
        ? {
            ...t,
            wearOffsetMm: wearOffsetMm === 0 ? undefined : wearOffsetMm,
            lastCalibrated: dateIso,
          }
        : t,
    );
  }
  const machineModes = $derived(effectiveModes(project.data.machine));
  const incompatibleCount = $derived(
    isInventory ? 0 : draft.filter((t) => !toolCompatibleWithAnyMode(t.kind, machineModes)).length,
  );
  function rowVisible(tool: ToolEntry): boolean {
    if (isInventory) return true; // the shop inventory is machine-agnostic
    return showIncompatible || toolCompatibleWithAnyMode(tool.kind, machineModes);
  }

  // ── table view: sort / filter / pagination ─────────────────────────
  // View-only — never reorders the stored library. Rows are wrapped as
  // { tool, i } with i = the ORIGINAL draft index, so every edit
  // handler keeps mutating the right entry regardless of sort order.
  let view = $state<ToolTableView>({ ...EMPTY_TOOL_VIEW });
  let page = $state(0);
  const viewedRows = $derived(
    applyToolTableView(
      draft.map((tool, i) => ({ tool, i })).filter((r) => rowVisible(r.tool)),
      view,
    ),
  );
  const paged = $derived(paginateToolRows(viewedRows, page));

  function clearFilters() {
    view.query = '';
    view.kind = '';
    view.mode = '';
    page = 0;
  }
  /// Make a specific tool visible: drop filters that would hide it and
  /// jump to its page under the current sort.
  function revealTool(id: number) {
    const tool = draft.find((t) => t.id === id);
    if (tool && !rowVisible(tool)) showIncompatible = true;
    clearFilters();
    const rows = applyToolTableView(
      draft.map((tool, i) => ({ tool, i })).filter((r) => rowVisible(r.tool)),
      { ...view, query: '', kind: '', mode: '' },
    );
    page = pageOfTool(rows, id) ?? 0;
  }

  $effect(() => {
    if (!active) return;
    // Tracked deps: ONLY the backing store (deep snapshot) + the
    // project tools the inventory seeds from. Everything below runs
    // untracked — the previous version read dd.isDirty (which
    // deep-reads dd.draft) and then WROTE dd.draft via dd.open(), so
    // every clone write re-invalidated the effect: an infinite loop
    // that froze the whole app the moment the tab mounted.
    const backing = $state.snapshot(backingTools) as ToolEntry[];
    const projectTools = $state.snapshot(project.data.tools) as ToolEntry[];
    untrack(() => {
      // Embedded panels stay mounted, so external tool changes (undo,
      // stocking from the Machine tab) re-run this — refresh a CLEAN
      // draft to stay in sync, but never clobber in-progress edits.
      if (embedded && dd.isDirty) return;
      let tools = backing;
      if (isInventory && tools.length === 0 && projectTools.length > 0) {
        // First use of the shop inventory on an installation that
        // predates it: seed from the current project's tools so the
        // user starts from what they already configured. Deferred —
        // workspace.version is $state and must not bump synchronously
        // inside an effect body.
        const seeded = seedInventoryFromProject(projectTools);
        queueMicrotask(() => workspace.setToolInventory(seeded));
        tools = seeded;
      }
      dd.open(tools);
      baseline = structuredClone(tools);
      baselineStocked = structuredClone(projectTools);
      if (!tools.some((t) => t.id === selectedId)) selectedId = tools[0]?.id ?? null;
      showIncompatible = false;
      calibratingIdx = null;
    });
  });

  let hasInvalidRow = $derived(draft.length === 0 || draft.some(rowInvalid));

  function close() {
    if (dd.requestClose()) onClose();
  }

  $effect(() => {
    const focusId = project.sel.toolsDialogFocusId;
    if (isInventory || !active || focusId == null) return;
    // The focus target may be mode-filtered out (an op still
    // referencing a mill tool on a plasma machine) — reveal it.
    const target = draft.find((t) => t.id === focusId);
    if (target && !rowVisible(target)) showIncompatible = true;
    if (target) revealTool(focusId);
    if (target) selectedId = focusId;
  });

  function commit() {
    // Refuse to commit while any row has an invalid numeric field. The
    // OK button is also disabled in that state — belt-and-braces so a
    // keyboard / programmatic invocation can't smuggle a zero-rate tool
    // through.
    if (
      draft.some(rowInvalid) ||
      conflict ||
      !reviewCurrent ||
      removedReferences.length ||
      importing ||
      pendingImport
    )
      return;
    // Deep-snapshot so the command system receives plain objects —
    // Svelte 5 `$state` proxies inside `draft[i]` can trip up the
    // `structuredClone` call inside replaceToolsCommand on some
    // production builds, which would silently abort and leave the
    // dialog open.
    if (draft.length > 0) {
      const snap = JSON.parse(JSON.stringify(draft)) as typeof draft;
      try {
        if (isInventory) {
          workspace.setToolInventory(snap);
          // Propagate the edits into same-id stocked copies so the
          // machine's loadout (and its profile, via the mirror)
          // follows the inventory definition. One undoable step.
          const synced = syncStockedFromInventory(
            snap,
            JSON.parse(JSON.stringify(project.data.tools)) as typeof draft,
          );
          if (synced) project.replaceTools(synced);
        } else {
          project.replaceTools(snap);
        }
      } catch (e) {
        project.setError(`Tool library apply failed: ${String(e)}`);
        return;
      }
    }
    // Embedded (tab) mode: Apply commits and stays — re-baseline the
    // draft instead of closing.
    baseline = $state.snapshot(backingTools);
    baselineStocked = $state.snapshot(project.data.tools);
    reviewToken = null;
    if (embedded) dd.markClean();
    else onClose();
  }

  function revert() {
    dd.open(backingTools);
    baseline = $state.snapshot(backingTools);
    baselineStocked = $state.snapshot(project.data.tools);
    reviewToken = null;
    pendingImport = null;
    deleting = null;
  }

  function addTool() {
    const nextId = (draft.reduce((m, t) => Math.max(m, t.id), 0) || 0) + 1;
    // Seed the PRIMARY mode's signature kind — a new tool on a plasma
    // machine starts as a torch, not an endmill the mode filter would
    // immediately hide.
    dd.draft = [...draft, defaultToolForMode(project.data.machine.mode, nextId)];
    revealTool(nextId);
    selectedId = nextId;
    section = 'geometry';
  }

  function removeAt(idx: number) {
    if (
      draft.length <= 1 ||
      (!isInventory && project.data.operations.some((o) => o.toolId === draft[idx]?.id))
    )
      return;
    dd.draft = draft.filter((_, i) => i !== idx);
    selectedId = draft[0]?.id ?? null;
    deleting = null;
  }

  function updateField<K extends keyof ToolEntry>(idx: number, key: K, value: ToolEntry[K]) {
    dd.draft = draft.map((t, i) => {
      if (i !== idx) return t;
      // Auto-naming, editor-autocomplete style: while a row's name is
      // empty or still equals its own suggestion, setting edits keep
      // the name in sync ("3mm endmill" follows the diameter). A name
      // the user typed is never rewritten.
      const wasAuto = key !== 'name' && isAutoToolName(t);
      const next = { ...t, [key]: value };
      if (wasAuto) next.name = suggestToolName(next);
      return next;
    });
  }

  function onKindChange(idx: number, kind: ToolKind) {
    dd.draft = draft.map((tool, i) => (i === idx ? changeToolKind(tool, kind) : tool));
  }

  function applyPreset(idx: number, label: string) {
    const patch = applyPresetPatch(draft[idx], label);
    if (!patch) return;
    dd.draft = draft.map((t, i) => (i === idx ? { ...t, ...patch } : t));
  }

  function setHolder(idx: number, holder: HolderShape | undefined) {
    dd.draft = draft.map((t, i) => (i === idx ? { ...t, holder } : t));
  }
</script>

{#snippet shell()}
  {#if !embedded}<header>
      <h2 id="tools-title">{isInventory ? t('uxb.library') : t('uxb.project')}</h2>
      <button class="dlg-close" onclick={close} aria-label={t('common.close')}>×</button>
    </header>{/if}
  <div class="table-actions">
    <strong>{isInventory ? t('uxb.library') : t('uxb.project')}</strong>
    {#if isInventory}<button
        class="tc-file"
        disabled={!project.data.tools.length}
        onclick={() => {
          project.sel.toolsDialogFocusId = project.data.tools[0]?.id ?? null;
        }}>{t('uxb.edit_project')}</button
      >{/if}
    <button class="tc-file" onclick={() => fileOps.exportToolset($state.snapshot(draft))}
      >{t('ux6b.export_draft')}</button
    >
    <button
      class="tc-file"
      disabled={importing || conflict || pendingImport !== null}
      onclick={() => importTools('replace')}>{t('tools.file.load_replace')}</button
    >
    <button
      class="tc-file"
      disabled={importing || conflict || pendingImport !== null}
      onclick={() => importTools('add')}>{t('tools.file.load_add')}</button
    >
    <span class="hint">{isInventory ? t('ux6b.inventory_scope') : t('ux6b.project_scope')}</span>
  </div>
  <ToolLibraryFilters
    bind:view
    total={draft.length}
    shown={paged.total}
    onFilterChange={() => (page = 0)}
    {clearFilters}
  />

  {#if conflict}<div class="notice" role="alert">
      {t('ux6b.conflict')}
      <button class="btn-secondary" onclick={revert}>{t('common.revert')}</button>
    </div>{/if}
  {#if pendingImport}<div class="notice" role="status">
      <strong>{t('ux6b.replace_title')}</strong>
      <p>{t('ux6b.replace_help', { before: draft.length, after: pendingImport.length })}</p>
      <button class="btn-secondary" onclick={() => (pendingImport = null)}
        >{t('common.cancel')}</button
      >
      <button class="btn-primary" disabled={conflict} onclick={acceptImport}
        >{t('ux6b.replace_draft')}</button
      >
    </div>{/if}
  {#if reviewToken !== null}
    <ToolApplyReview
      {isInventory}
      {impacts}
      {reviewCurrent}
      disabled={!reviewCurrent ||
        conflict ||
        hasInvalidRow ||
        removedReferences.length > 0 ||
        pendingImport !== null}
      onCancel={() => (reviewToken = null)}
      onApply={commit}
    />
  {/if}
  <div class="body">
    <aside>
      <label class="sort"
        >{t('ux6b.sort')}<select
          value={view.sortKey ?? ''}
          onchange={(e) => {
            view.sortKey = (e.currentTarget.value || null) as ToolSortKey | null;
            page = 0;
          }}
          ><option value="">{t('ux6b.library_order')}</option><option value="name"
            >{t('tools.col.name')}</option
          ><option value="diameter">{t('ux6b.diameter')}</option><option value="kind"
            >{t('tools.col.kind')}</option
          ></select
        ></label
      >
      <div class="tool-list" aria-label={t('tools.title')}>
        {#each paged.rows as { tool } (tool.id)}<button
            type="button"
            data-tool-id={tool.id}
            aria-pressed={selectedId === tool.id}
            onclick={() => selectTool(tool.id)}
            onkeydown={navigate}
            title={tool.name}
          >
            <strong>#{tool.id} {tool.name}</strong>
            <span
              >{KIND_DISPLAY_LABELS[tool.kind]} · Ø{tool.diameter} mm{#if fieldApplies('flutes', tool.kind)}
                · {tool.flutes} {t('tools.col.flutes')}{/if}</span
            >
            {#if rowInvalid(tool)}<span class="invalid-label">{t('ux6b.needs_review')}</span>{/if}
          </button>{/each}
      </div>
      {#if paged.total === 0}<p class="hint">{t('tools.no_match')}</p>{/if}
      {#if incompatibleCount > 0}<button
          class="tc-file"
          onclick={() => (showIncompatible = !showIncompatible)}
          >{showIncompatible ? t('tools.hide_incompatible') : t('tools.show_all')} ({incompatibleCount})</button
        >{/if}
      <button class="add" onclick={addTool}
        ><Plus size={16} aria-hidden="true" />{t('ux6b.add_tool')}</button
      >
      {#if paged.pageCount > 1}
        <div class="table-pager">
          <button type="button" disabled={paged.page === 0} onclick={() => (page = paged.page - 1)}
            >{t('tools.pager.prev')}</button
          >
          <span>{t('tools.pager.status', { page: paged.page + 1, total: paged.pageCount })}</span>
          <button
            type="button"
            disabled={paged.page >= paged.pageCount - 1}
            onclick={() => (page = paged.page + 1)}>{t('tools.pager.next')}</button
          >
        </div>
      {/if}
    </aside>
    <section class="detail">
      {#if selectedTool}
        {@const tool = selectedTool}
        {@const i = selectedIndex}
        <header>
          <div>
            <h2>#{tool.id} {tool.name}</h2>
            <p class="hint">{t('ux6b.draft_scope')}</p>
          </div>
          <button
            class="tc-file"
            disabled={draft.length <= 1 || (!isInventory && selectedUses.length > 0)}
            onclick={() => (deleting = tool.id)}
            ><Trash2 size={14} aria-hidden="true" />{isInventory
              ? t('ux6b.delete_library')
              : t('ux6b.remove_project')}</button
          >
        </header>
        {#if deleting === tool.id}<div class="notice">
            <p>{isInventory ? t('ux6b.delete_help') : t('ux6b.remove_help')}</p>
            <p>
              {t('ux6b.copy_counts', {
                operations: selectedUses.length,
                profiles: selectedCopies.length,
              })}
            </p>
            <button class="btn-secondary" onclick={() => (deleting = null)}
              >{t('common.cancel')}</button
            ><button class="btn-danger" onclick={() => removeAt(i)}>{t('ux6b.remove_draft')}</button
            >
          </div>{/if}
        {#if !paged.rows.some((row) => row.tool.id === tool.id)}
          <div class="notice">
            {t('ux6b.filtered_selection')}
            <button class="tc-file" onclick={() => revealTool(tool.id)}
              >{t('ux6b.reveal_selection')}</button
            >
          </div>
        {/if}
        <nav aria-label={t('ux6b.sections')}>
          {#each [['geometry', t('ux6b.geometry')], ['mounting', t('ux6b.mounting')], ['availability', t('ux6b.availability')], ['advanced', t('ux6b.advanced')]] as item (item[0])}<button
              aria-pressed={section === item[0]}
              onclick={() => (section = item[0] as typeof section)}>{item[1]}</button
            >{/each}
        </nav>
        <div class="detail-body">
          <div
            class="detail-layout"
            class:with-diagram={section === 'geometry' || section === 'mounting'}
          >
            <div class="detail-fields">
              {#if section === 'geometry'}
                <ToolDetailBasics
                  {tool}
                  onUpdateField={(k, v) => updateField(i, k, v)}
                  onKindChange={(kind) => onKindChange(i, kind)}
                />
                <p class="hint">{t('ux6b.geometry_help')}</p>
              {/if}
              {#if section === 'mounting'}<p class="hint">{t('ux6b.mounting_help')}</p>{/if}
              {#if section === 'availability'}
                <div class="availability">
                  <h3>{t('ux6b.current_machine')}</h3>
                  <p>{selectedStocked ? t('ux6b.same_id_copy') : t('ux6b.not_stocked')}</p>
                  {#if selectedStocked && !deepEqual(tool, selectedStocked)}<p class="notice">
                      {t('ux6b.copy_differs')}
                    </p>
                    <ToolCopyComparison
                      before={{ label: t('uxb.project'), ...selectedStocked }}
                      after={{ label: isInventory ? t('uxb.library') : t('uxb.after'), ...tool }}
                    />{/if}
                  <p>
                    {toolCompatibleWithAnyMode(tool.kind, machineModes)
                      ? t('ux6b.compatible')
                      : t('ux6b.incompatible')}
                  </p>
                  {#if isInventory}<div class="actions">
                      <button
                        class="tc-file"
                        disabled={dd.isDirty ||
                          !toolCompatibleWithAnyMode(tool.kind, machineModes) ||
                          (selectedStocked !== undefined && deepEqual(tool, selectedStocked))}
                        onclick={addToMachine}
                        ><Plus size={16} aria-hidden="true" />{t('ux6b.add_machine')}</button
                      ><button
                        class="tc-file"
                        disabled={cannotRemoveStocked}
                        onclick={removeFromMachine}>{t('ux6b.remove_machine')}</button
                      >
                    </div>{/if}
                  <p class="hint">{t('ux6b.association_help')}</p>
                  <h3>{t('ux6b.operations')}</h3>
                  <ul>
                    {#each selectedUses as op (op.id)}<li>{op.name}</li>{:else}<li>
                        {t('ux6b.no_operations')}
                      </li>{/each}
                  </ul>
                  <h3>{t('ux6b.profile_copies')}</h3>
                  <ul>
                    {#each selectedCopies as profile (profile.id)}<li>{profile.name}</li>{:else}<li>
                        {t('ux6b.no_profiles')}
                      </li>{/each}
                  </ul>
                  <p class="hint">{t('ux6b.copy_help')}</p>
                </div>
              {:else}
                <ToolRowExpandedEditor
                  {tool}
                  {section}
                  onUpdateField={(k, v) => updateField(i, k, v)}
                  onApplyPreset={(label) => applyPreset(i, label)}
                  onSetHolder={(h) => setHolder(i, h)}
                  onCalibrate={() => (calibratingIdx = i)}
                />
              {/if}
              {#if section === 'advanced'}
                <details>
                  <summary>{t('ux6b.legacy')}</summary>
                  <p class="hint">{t('fs35.fallback_help')}</p>
                  <p class="hint">{tool.speed} RPM · {tool.feedRate} / {tool.plungeRate} mm/min</p>
                  <ToolDetailBasics
                    {tool}
                    legacy
                    onUpdateField={(k, v) => updateField(i, k, v)}
                    onKindChange={(kind) => onKindChange(i, kind)}
                  />
                  <ToolRowExpandedEditor
                    {tool}
                    section="legacy"
                    onUpdateField={(k, v) => updateField(i, k, v)}
                    onApplyPreset={(label) => applyPreset(i, label)}
                    onSetHolder={(h) => setHolder(i, h)}
                    onCalibrate={() => (calibratingIdx = i)}
                  />
                </details>
                <ToolRetainedFields {tool} />
              {/if}
            </div>
            {#if section === 'geometry' || section === 'mounting'}<ToolDimensionDiagram
                {tool}
              />{/if}
          </div>
        </div>
      {:else}<p class="hint">{t('ux6b.select_tool')}</p>{/if}
    </section>
  </div>
  {#if removedReferences.length}<p class="validation-msg">
      {t('ux6b.missing_refs')}: {removedReferences.map((o) => o.name).join(', ')}
    </p>{/if}
  <footer>
    {#if dd.confirmingDiscard}
      <span class="discard-prompt">{t('common.discard_unsaved')}</span>
      <button class="btn-secondary" onclick={() => dd.cancelDiscard()}
        >{t('common.keep_editing')}</button
      >
      <button class="btn-danger" onclick={close}>{t('common.discard')}</button>
    {:else}
      <span class="sep"></span>
      {#if hasInvalidRow}
        <!-- Surface why OK is greyed out so the user knows which inputs need fixing. -->
        <span class="validation-msg" role="status">{t('tools.validation_msg')}</span>
      {/if}
      {#if embedded}
        <button class="btn-secondary" onclick={revert} disabled={!dd.isDirty}
          >{t('common.revert')}</button
        >
        <button
          class="btn-primary"
          onclick={() => (reviewToken = currentToken)}
          disabled={hasInvalidRow ||
            !dd.isDirty ||
            conflict ||
            removedReferences.length > 0 ||
            importing ||
            pendingImport !== null}
          title={hasInvalidRow ? t('tools.apply.invalid.title') : ''}
          >{t('ux6b.review_apply')}</button
        >
      {:else}
        <button class="btn-secondary" onclick={close}>{t('common.cancel')}</button>
        <button
          class="btn-primary"
          onclick={() => (reviewToken = currentToken)}
          disabled={hasInvalidRow ||
            conflict ||
            removedReferences.length > 0 ||
            importing ||
            pendingImport !== null}
          title={hasInvalidRow ? t('tools.save.invalid.title') : ''}
          >{t('ux6b.review_apply')}</button
        >
      {/if}
    {/if}
  </footer>
{/snippet}

{#if embedded}
  <section class="embedded-shell">{@render shell()}</section>
{:else if open}
  <Modal
    onClose={close}
    persistKey="tool-library"
    width="min(960px, 96vw)"
    draggable
    resizable
    ariaLabelledBy="tools-title"
  >
    {@render shell()}
  </Modal>
{/if}
{#if active && calibratingIdx != null && draft[calibratingIdx]}
  {@const calTool = draft[calibratingIdx]}
  <ToolCalibrationDialog
    open
    toolName={calTool.name}
    nominalDiameterMm={calTool.diameter}
    currentWearOffsetMm={calTool.wearOffsetMm ?? 0}
    onApply={(wear, date) => {
      if (calibratingIdx != null) applyCalibration(calibratingIdx, wear, date);
    }}
    onClose={() => (calibratingIdx = null)}
  />
{/if}

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    padding: 12px;
    border-bottom: 1px solid var(--border);
  }
  h2 {
    font-size: 0.95rem;
    margin: 0;
    overflow-wrap: anywhere;
  }
  .embedded-shell {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    background: var(--bg-panel);
  }
  .body {
    display: grid;
    grid-template-columns: 300px minmax(0, 1fr);
    flex: 1;
    min-height: 0;
  }
  aside {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    border-right: 1px solid var(--border);
    min-height: 0;
    overflow: auto;
  }
  .tool-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
    overflow: auto;
    min-height: 100px;
  }
  .tool-list button {
    text-align: left;
    background: transparent;
    color: var(--text);
    border: 1px solid transparent;
    border-radius: 3px;
    padding: 8px;
    font-size: 0.8rem;
  }
  .tool-list strong,
  .tool-list span {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .tool-list span {
    font-size: 0.72rem;
    color: var(--text-muted);
    margin-top: 4px;
  }
  .tool-list .invalid-label {
    color: var(--danger);
  }
  .detail {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
  }
  .detail-body {
    padding: 16px;
    overflow: auto;
    flex: 1;
    min-height: 0;
  }
  .detail-layout {
    display: grid;
    gap: 24px;
    align-items: start;
  }
  .detail-fields {
    min-width: 0;
    max-width: 640px;
  }
  .with-diagram {
    grid-template-columns: minmax(250px, 640px) minmax(240px, 340px);
  }
  @media (max-width: 1350px) {
    .with-diagram {
      grid-template-columns: minmax(0, 1fr);
    }
  }
  nav {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 8px 12px;
    border-bottom: 1px solid var(--border);
  }
  nav button,
  .add {
    border: 1px solid var(--border);
    border-radius: 3px;
    background: var(--bg-elevated);
    color: var(--text);
    padding: 4px 8px;
    font-size: 0.8rem;
    min-height: 28px;
  }
  nav button[aria-pressed='true'],
  .tool-list button[aria-pressed='true'] {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 12%, var(--bg-panel));
  }
  nav button[aria-pressed='true'] {
    box-shadow: inset 0 -2px 0 var(--accent);
  }
  .tc-file,
  .add,
  .actions {
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }
  /* Match the compact geometry/mounting inspectors instead of inheriting
     document-sized paragraphs and browser-default list spacing. */
  .availability {
    font-size: 0.8rem;
    line-height: 1.45;
    overflow-wrap: anywhere;
  }
  .availability h3 {
    margin: 16px 0 8px;
    padding-bottom: 6px;
    border-bottom: 1px solid var(--border);
    color: var(--text-section);
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .availability h3:first-child {
    margin-top: 0;
  }
  .availability p {
    margin: 8px 0;
  }
  .availability .notice {
    padding: 8px 10px;
    border-bottom: 0;
    border-left: 2px solid var(--border);
    font-size: inherit;
    max-height: none;
    overflow: visible;
  }
  .availability .actions {
    display: flex;
    flex-wrap: wrap;
    margin: 12px 0 8px;
  }
  .availability .tc-file {
    min-height: 28px;
    white-space: normal;
    text-align: left;
  }
  .availability ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .availability li {
    padding: 6px 0;
  }
  .availability li + li {
    border-top: 1px solid var(--border);
  }
  .hint {
    color: var(--text-muted);
    font-size: 0.76rem;
    line-height: 1.45;
  }
  .notice {
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elevated);
    font-size: 0.8rem;
    max-height: 220px;
    overflow: auto;
  }
  .notice button {
    margin-right: 6px;
  }
  .sort {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.75rem;
  }
  details {
    margin: var(--ui-space-3) 0;
    border-top: 1px solid var(--border);
  }
  details > summary {
    cursor: pointer;
    padding: var(--ui-space-2) 0;
    color: var(--text-muted);
    font-size: 0.78rem;
    font-weight: 400;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    gap: 6px;
    padding: 8px 12px;
    border-top: 1px solid var(--border);
    background: var(--bg-elevated);
  }
  .validation-msg {
    color: var(--danger);
    font-size: 0.78rem;
  }
  @media (max-width: 1050px) {
    .body {
      grid-template-columns: 220px minmax(0, 1fr);
    }
  }
  select {
    background: var(--bg-input);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0.18rem 0.32rem;
    font-size: 0.78rem;
    min-width: 0;
    width: 100%;
    box-sizing: border-box;
  }
  /* Two header rows: file actions on the activated-tab surface, then
     the filter row in a lighter tone acting as a divider before the
     table. */
  .table-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.7rem;
    background: var(--bg-panel);
    font-size: 0.78rem;
    flex-wrap: wrap;
  }
  .tc-file {
    background: var(--bg-panel);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0.2rem 0.55rem;
    font-size: 0.74rem;
    cursor: pointer;
    white-space: nowrap;
  }
  .table-pager {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.7rem;
    padding: 0.35rem 0.7rem;
    border-top: 1px solid var(--border);
    background: var(--bg-elevated);
    font-size: 0.78rem;
  }
  .table-pager button {
    background: var(--bg-panel);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0.15rem 0.55rem;
    font-size: 0.74rem;
    cursor: pointer;
  }
  .table-pager button:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
