<script lang="ts">
  import RecordedPresetContext from './RecordedPresetContext.svelte';
  import PresetReviewError from './PresetReviewError.svelte';
  import { t } from '../i18n';
  import { Plus, Import, Ellipsis, FolderOpen, Search } from '@lucide/svelte';
  import { presetLibraryState } from '../state/cutting-ui';
  import { cuttingRoles } from '../state/cutting-parameters';
  import { onDestroy } from 'svelte';
  import Modal from './Modal.svelte';
  import TestedPresetEditor from './TestedPresetEditor.svelte';
  import { project } from '../state/project.svelte';
  import {
    listCollections,
    validateCollection,
    saveCollection,
    deleteCollection,
    type PresetCollection,
    type TestedPreset,
  } from '../state/preset-collections';
  import {
    testedRows,
    filterTestedRows,
    emptyFilters,
    importChoice,
    parameterSummary,
    type TestedRow,
  } from '../state/preset-library-v9';
  import { inspectCutting, preparePresetLoad } from '../state/cutting-workflow';
  import { importCollectionFile, exportCollectionFile } from '../services/preset-collection-files';
  let { active }: { active: boolean } = $props();
  let collections = $state<{ data: PresetCollection; token: string }[]>([]);
  let filters = $state(emptyFilters()),
    selectedKey = $state(''),
    collectionId = $state('');
  let error = $state(''),
    message = $state(''),
    busy = $state(false),
    page = $state(0);
  let naming = $state<'create' | 'rename' | null>(null),
    name = $state('');
  let deletion = $state<{
    collection: PresetCollection;
    token: string;
    preset?: TestedPreset;
  } | null>(null);
  let incoming = $state<PresetCollection | null>(null);
  let editing = $state<{
    initial: TestedPreset;
    original: TestedPreset | null;
    collection: PresetCollection;
    token: string;
  } | null>(null);
  let preview = $state<Awaited<ReturnType<typeof preparePresetLoad>> | null>(null);
  const rows = $derived(testedRows(collections.map((c) => c.data)));
  const filtered = $derived(filterTestedRows(rows, filters));
  const view = $derived(
    presetLibraryState(
      collections.map((c) => c.data),
      filters,
      filtered.length,
    ),
  );
  const pages = $derived(Math.max(1, Math.ceil(filtered.length / 50)));
  const shown = $derived(
    filtered.slice(Math.min(page, pages - 1) * 50, (Math.min(page, pages - 1) + 1) * 50),
  );
  const selected = $derived(rows.find((r) => r.key === selectedKey));
  const collection = $derived(collections.find((c) => c.data.id === collectionId));
  const selectedOperation = $derived(
    project.data.operations.find((op) => op.id === project.sel.selectedOpId),
  );
  const canCapture = $derived(
    project.data.machine.mode === 'mill' &&
      !!selectedOperation &&
      cuttingRoles(selectedOperation).length > 0,
  );
  const collision = $derived(
    incoming ? collections.find((c) => c.data.id === incoming?.id) : undefined,
  );
  let alive = true,
    sequence = 0;
  onDestroy(() => {
    alive = false;
    sequence++;
  });
  async function refresh() {
    const request = ++sequence;
    const stored = await listCollections();
    const parsed = await Promise.all(
      stored.map(async (r) => ({ data: await validateCollection(r.json), token: r.token })),
    );
    if (alive && request === sequence) {
      collections = parsed;
      if (!parsed.some((c) => c.data.id === collectionId)) collectionId = parsed[0]?.data.id ?? '';
      if (filters.collection && !parsed.some((c) => c.data.id === filters.collection))
        filters.collection = '';
    }
  }
  async function run(action: () => Promise<void>) {
    busy = true;
    error = '';
    message = '';
    try {
      await action();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  $effect(() => {
    if (active) void run(refresh);
  });
  $effect(() => {
    void JSON.stringify(filters);
    page = 0;
  });
  async function saveName() {
    await run(async () => {
      const target = collection;
      if (naming === 'rename' && !target) throw new Error('Select a collection');
      const data: PresetCollection =
        naming === 'rename'
          ? { ...target!.data, name: name.trim() }
          : {
              format: 'fabcam-preset-collection',
              version: 1,
              id: crypto.randomUUID(),
              name: name.trim(),
              presets: [],
            };
      await saveCollection(data, naming === 'rename' ? target!.token : null);
      naming = null;
      collectionId = data.id;
      filters = { ...emptyFilters(), collection: data.id };
      await refresh();
    });
  }
  function edit(row: TestedRow, duplicate = false) {
    const stored = collections.find((c) => c.data.id === row.collection.id)!;
    const original = structuredClone($state.snapshot(row.preset));
    const initial = structuredClone($state.snapshot(original));
    if (duplicate) {
      initial.id = crypto.randomUUID();
      initial.name = `${initial.name.slice(0, 145)} (copy)`;
    }
    editing = {
      initial,
      original,
      collection: structuredClone($state.snapshot(row.collection)),
      token: stored.token,
    };
  }
  async function createFromOperation() {
    const target = collection,
      id = project.sel.selectedOpId;
    if (!target || id == null) return;
    const result = await inspectCutting(project, id);
    if (result.check.errors.length) throw new Error(result.check.errors.join('; '));
    if (!result.context) throw new Error(result.context_error ?? 'Complete identities');
    editing = {
      initial: {
        id: crypto.randomUUID(),
        name: project.data.operations.find((o) => o.id === id)?.name ?? 'Preset',
        context: result.context,
        parameters: result.parameters,
        tested: true,
        test_date: null,
        notes: null,
      },
      original: null,
      collection: structuredClone($state.snapshot(target.data)),
      token: target.token,
    };
  }
  async function saveEdited(preset: TestedPreset) {
    if (!editing) throw new Error('Editor closed');
    const c = structuredClone($state.snapshot(editing.collection)),
      index = c.presets.findIndex((p) => p.id === preset.id);
    if (index < 0) c.presets.push(preset);
    else c.presets[index] = preset;
    await saveCollection(c, editing.token);
    await refresh();
  }
  async function confirmDelete() {
    const target = deletion;
    if (!target) return;
    await run(async () => {
      if (target.preset)
        await saveCollection(
          {
            ...target.collection,
            presets: target.collection.presets.filter((p) => p.id !== target.preset!.id),
          },
          target.token,
        );
      else await deleteCollection(target.collection.id, target.token);
      deletion = null;
      await refresh();
    });
  }
  async function importSelected(choice: 'new' | 'replace' | 'copy') {
    if (!incoming) return;
    const plan = importChoice($state.snapshot(incoming), $state.snapshot(collision), choice);
    await saveCollection(plan.data, plan.expected);
    incoming = null;
    collectionId = plan.data.id;
    filters = { ...emptyFilters(), collection: plan.data.id };
    await refresh();
    message = 'Imported author attestations; current workshop testing has not been established.';
  }
  function select(row: TestedRow) {
    selectedKey = row.key;
    collectionId = row.collection.id;
    preview = null;
  }
  function navigate(event: KeyboardEvent, index: number) {
    const next =
      event.key === 'ArrowDown'
        ? Math.min(index + 1, shown.length - 1)
        : event.key === 'ArrowUp'
          ? Math.max(0, index - 1)
          : event.key === 'Home'
            ? 0
            : event.key === 'End'
              ? shown.length - 1
              : -1;
    if (next < 0) return;
    event.preventDefault();
    select(shown[next]);
    const buttons = (event.currentTarget as HTMLElement)
      .closest('table')
      ?.querySelectorAll<HTMLButtonElement>('[data-record]');
    buttons?.[next]?.focus();
  }
</script>

<div class="library">
  <header>
    <div>
      <h1>{t('fsreset.ux.presets')}</h1>
      <p class="scope">{t('fsreset.ux.subtitle')}</p>
    </div>
    <div class="actions">
      <button
        class="btn-secondary"
        disabled={busy}
        onclick={() => {
          naming = 'create';
          name = '';
        }}><Plus size={14} />{t('fsreset.ui.new.collection')}</button
      >
      <button
        class="btn-secondary"
        disabled={busy}
        onclick={() =>
          void run(async () => {
            incoming = await importCollectionFile();
          })}><Import size={14} />{t('fsreset.ui.import.collection')}</button
      >
    </div>
  </header>
  {#if collections.length}
    <div class="collection-bar">
      <label class="browse-collection"
        >{t('fsreset.ui.collection')}<select
          aria-label={t('fsreset.ux.browse')}
          value={filters.collection}
          onchange={(e) => {
            filters.collection = e.currentTarget.value;
            if (filters.collection) collectionId = filters.collection;
          }}
        >
          <option value="">{t('fsreset.ux.all')}</option>
          {#each collections as c (c.data.id)}<option value={c.data.id}>{c.data.name}</option
            >{/each}
        </select></label
      >
      <span class="scope collection-count">{view.scopeCount} presets</span>
      {#if collection}<details class="collection-menu">
          <summary title={`Manage collection: ${collection.data.name}`}
            ><Ellipsis size={16} />{t('fsreset.ux.manage')}
            <span>{collection.data.name}</span></summary
          >
          <div class="management">
            <button
              class="btn-secondary"
              disabled={busy}
              onclick={() => {
                naming = 'rename';
                name = collection!.data.name;
              }}>{t('fsreset.ui.rename')}</button
            >
            <button
              class="btn-secondary"
              disabled={busy}
              onclick={() =>
                void run(async () => {
                  if (await exportCollectionFile(collection!.data))
                    message = 'Collection export requested';
                })}>{t('fsreset.ui.export')}</button
            >
            <button
              class="btn-secondary"
              disabled={busy}
              onclick={() =>
                (deletion = {
                  collection: structuredClone($state.snapshot(collection!.data)),
                  token: collection!.token,
                })}>{t('fsreset.ui.delete.collection')}</button
            >
            <button class="btn-secondary" disabled={busy} onclick={() => void run(refresh)}
              >{t('fsreset.ui.refresh')}</button
            >
          </div>
        </details>{/if}
      {#if canCapture && collection}<button
          class="btn-secondary"
          disabled={busy}
          onclick={() => void run(createFromOperation)}
          >{t('fsreset.ui.save.selected.operation.as.preset')}</button
        >{/if}
    </div>
  {/if}
  {#if view.scopeCount > 0}
    <div class="filters">
      <label
        >{t('fsreset.ui.search')}<input
          type="search"
          bind:value={filters.query}
          placeholder={t('fsreset.ui.name.identity.material')}
        /></label
      >
      <label
        >{t('fsreset.ui.machine')}<select bind:value={filters.machine}
          ><option value="">{t('fsreset.ui.all')}</option
          >{#each [...new Map(rows.map( (r) => [r.preset.context.machine_id, r.preset.context.machine_label], )).entries()] as [id, label] (id)}<option
              value={id}>{label}</option
            >{/each}</select
        ></label
      >
      <label
        >{t('fsreset.ui.tool')}<select bind:value={filters.tool}
          ><option value="">{t('fsreset.ui.all')}</option
          >{#each [...new Set(rows.map((r) => r.tools))] as tool (tool)}<option>{tool}</option
            >{/each}</select
        ></label
      >
      <label
        >{t('fsreset.ui.material')}<select bind:value={filters.material}
          ><option value="">{t('fsreset.ui.all')}</option
          >{#each [...new Map(rows.map( (r) => [r.preset.context.material_id, r.preset.context.material_label], )).entries()] as [id, label] (id)}<option
              value={id}>{label}</option
            >{/each}</select
        ></label
      >
      <label
        >{t('fsreset.ui.operation')}<select bind:value={filters.operation}
          ><option value="">{t('fsreset.ui.all')}</option
          >{#each [...new Set(rows.map((r) => r.operation))] as op (op)}<option>{op}</option
            >{/each}</select
        ></label
      >
      <button
        class="btn-secondary"
        onclick={() => (filters = { ...emptyFilters(), collection: filters.collection })}
        >{t('fsreset.ux.clear')}</button
      >
    </div>
  {/if}
  <PresetReviewError {error} />{#if message}<p role="status">{message}</p>{/if}
  {#if view.kind !== 'results'}
    <section class="empty-state" aria-label={t('fsreset.ux.state')}>
      {#if view.kind === 'no-collections'}
        <FolderOpen size={24} />
        <h2>{t('fsreset.ux.no_collections')}</h2>
        <p>
          {t('fsreset.ux.create_help')}
        </p>
      {:else if view.kind === 'empty-collection'}
        <FolderOpen size={24} />
        <h2>{t('fsreset.ux.empty')}</h2>
        <p>{t('fsreset.ux.empty_help')}</p>
        {#if !canCapture}<p class="scope">
            {t('fsreset.ux.operation_help')}
          </p>{/if}
      {:else}
        <Search size={24} />
        <h2>{t('fsreset.ux.no_matches')}</h2>
        <button
          class="btn-secondary"
          onclick={() => (filters = { ...emptyFilters(), collection: filters.collection })}
          >{t('fsreset.ux.clear')}</button
        >
      {/if}
    </section>
  {:else}
    <div class="content">
      <div class="table">
        <table>
          <thead
            ><tr
              ><th>{t('fsreset.ui.preset')}</th><th>{t('fsreset.ui.collection')}</th><th
                >{t('fsreset.ui.machine')}</th
              ><th>{t('fsreset.ui.tool')}</th><th>{t('fsreset.ui.material')}</th><th
                >{t('fsreset.ui.operation')}</th
              ></tr
            ></thead
          ><tbody>
            {#each shown as row, index (row.key)}<tr class:selected={selectedKey === row.key}
                ><td
                  ><button
                    class="btn-secondary"
                    title={row.preset.name}
                    data-record
                    aria-pressed={selectedKey === row.key}
                    onclick={() => select(row)}
                    onkeydown={(e) => navigate(e, index)}>{row.preset.name}</button
                  ></td
                ><td title={row.collection.name}>{row.collection.name}</td><td
                  title={row.preset.context.machine_label}>{row.preset.context.machine_label}</td
                ><td title={row.tools}>{row.tools}</td><td title={row.preset.context.material_label}
                  >{row.preset.context.material_label}</td
                ><td>{row.operation}</td></tr
              >{/each}
          </tbody>
        </table>
      </div>
      {#if selected && filtered.some((r) => r.key === selected.key)}<aside
          aria-label={t('fsreset.ux.details')}
        >
          <h2>{selected.preset.name}</h2>
          <p>{selected.collection.name}</p>
          <p>
            Recorded machine: {selected.preset.context.machine_label} ({selected.preset.context
              .machine_id})
          </p>
          <p>Tools: {selected.tools}</p>
          <p>Material: {selected.preset.context.material_label}</p>
          <p>
            Author attestation: tested{selected.preset.test_date
              ? ` on ${selected.preset.test_date}`
              : ''}. Not proof of testing on the current workshop machine.
          </p>
          {#each selected.preset.parameters.rates as r (r.role)}<p>
              {r.role} · T{r.tool_id}: {r.rpm} rpm · feed {r.feed_mm_min} · plunge {r.plunge_mm_min} mm/min
            </p>{/each}
          {#each parameterSummary(selected.preset.parameters) as detail (detail)}<p>
              {detail}
            </p>{/each}
          <RecordedPresetContext preset={selected.preset} />
          {#if selected.preset.notes}<p class="notes">{selected.preset.notes}</p>{/if}
          <div class="actions">
            <button class="btn-secondary" disabled={busy} onclick={() => edit(selected)}
              >{t('fsreset.ui.edit')}</button
            ><button class="btn-secondary" disabled={busy} onclick={() => edit(selected, true)}
              >{t('fsreset.ui.duplicate')}</button
            ><button
              class="btn-secondary"
              disabled={busy}
              onclick={() => {
                const stored = collections.find((c) => c.data.id === selected.collection.id)!;
                deletion = {
                  collection: structuredClone($state.snapshot(stored.data)),
                  token: stored.token,
                  preset: structuredClone($state.snapshot(selected.preset)),
                };
              }}>{t('fsreset.ui.delete.preset')}</button
            >
          </div>
          <button
            class="btn-secondary"
            disabled={busy || project.sel.selectedOpId == null}
            title={project.sel.selectedOpId == null ? t('fsreset.ux.select_operation') : undefined}
            onclick={() =>
              void run(async () => {
                preview = null;
                preview = await preparePresetLoad(
                  project,
                  project.sel.selectedOpId!,
                  selected.collection,
                  selected.preset.id,
                );
              })}>{t('fsreset.ui.preview.load.into.selected.operation')}</button
          >
          {#if project.sel.selectedOpId == null}<p class="scope">
              {t('fsreset.ux.select_operation')}
            </p>{/if}
          {#if preview}<section class="preview">
              <h3>{t('fsreset.ui.reviewed.replacement')}</h3>
              {#each preview.after as rate (rate.role)}<p>
                  {rate.role}: {preview.before.find((r) => r.role === rate.role)?.rpm ?? '—'} → {rate.rpm}
                  rpm; feed {preview.before.find((r) => r.role === rate.role)?.feed_mm_min ?? '—'} → {rate.feed_mm_min};
                  plunge {preview.before.find((r) => r.role === rate.role)?.plunge_mm_min ?? '—'} → {rate.plunge_mm_min}
                  mm/min
                </p>{/each}
              {#each parameterSummary(preview.parameters) as detail (detail)}<p>{detail}</p>{/each}
              <button
                class="btn-secondary"
                disabled={busy}
                onclick={() =>
                  void run(async () => {
                    project.history.exec(preview!.command, project.target());
                    preview = null;
                    message = 'Values copied into the reviewed operation';
                  })}>{t('fsreset.ui.replace.operation.parameters')}</button
              ><button class="btn-secondary" onclick={() => (preview = null)}
                >{t('fsreset.ui.cancel')}</button
              >
            </section>{/if}
        </aside>{/if}
    </div>
    <footer>
      <span>{filtered.length} presets</span>{#if pages > 1}<span
          >Page {Math.min(page, pages - 1) + 1}/{pages}</span
        ><button class="btn-secondary" disabled={page <= 0} onclick={() => page--}
          >{t('fsreset.ui.previous')}</button
        ><button class="btn-secondary" disabled={page >= pages - 1} onclick={() => page++}
          >{t('fsreset.ui.next')}</button
        >{/if}
    </footer>
  {/if}
</div>
{#if naming}<Modal
    onClose={() => {
      if (!busy) naming = null;
    }}
    ariaLabelledBy="collection-name-title"
    width="min(420px, 95vw)"
    ><form
      class="dialog"
      onsubmit={(e) => {
        e.preventDefault();
        void saveName();
      }}
    >
      <h2 id="collection-name-title">
        {naming === 'create' ? 'New collection' : 'Rename collection'}
      </h2>
      <label>{t('fsreset.ui.name')}<input bind:value={name} required maxlength="160" /></label
      >{#if error}<p role="alert">
          {error}
        </p>{/if}<button class="btn-secondary" disabled={busy}
        >{t('fsreset.ui.save.collection')}</button
      ><button class="btn-secondary" type="button" disabled={busy} onclick={() => (naming = null)}
        >{t('fsreset.ui.cancel')}</button
      >
    </form></Modal
  >{/if}
{#if deletion}<Modal
    onClose={() => {
      if (!busy) deletion = null;
    }}
    ariaLabelledBy="delete-preset-title"
    width="min(420px, 95vw)"
    ><div class="dialog">
      <h2 id="delete-preset-title">
        Delete {deletion.preset ? 'preset' : 'collection'} “{deletion.preset?.name ??
          deletion.collection.name}”?
      </h2>
      <p>
        {deletion.preset ? 'This preset' : 'This collection and its presets'} will be removed from the
        library. Existing operations keep their copied values. Export a backup first if needed.
      </p>
      <PresetReviewError {error} /><button
        class="btn-danger"
        disabled={busy}
        onclick={confirmDelete}>{t('fsreset.ui.delete')}</button
      ><button class="btn-secondary" disabled={busy} onclick={() => (deletion = null)}
        >{t('fsreset.ui.cancel')}</button
      >
    </div></Modal
  >{/if}
{#if incoming}<Modal
    onClose={() => {
      if (!busy) incoming = null;
    }}
    ariaLabelledBy="import-collection-title"
    width="min(520px, 95vw)"
    ><div class="dialog">
      <h2 id="import-collection-title">Import “{incoming.name}”</h2>
      <p>
        {incoming.presets.length} presets. Imported attestations belong to their recorded workshop configurations;
        importing does not establish local testing.
      </p>
      <PresetReviewError {error} />{#if collision}<p>
          Collection ID already exists as “{collision.data.name}”. Replace that reviewed collection
          or create an independent copy.
        </p>
        <button
          class="btn-secondary"
          disabled={busy}
          onclick={() => void run(() => importSelected('replace'))}
          >Replace “{collision.data.name}”</button
        ><button
          class="btn-secondary"
          disabled={busy}
          onclick={() => void run(() => importSelected('copy'))}
          >{t('fsreset.ui.import.independent.copy')}</button
        >{:else}<button
          class="btn-secondary"
          disabled={busy}
          onclick={() => void run(() => importSelected('new'))}
          >{t('fsreset.ui.import.collection')}</button
        >{/if}<button class="btn-secondary" disabled={busy} onclick={() => (incoming = null)}
        >{t('fsreset.ui.cancel')}</button
      >
    </div></Modal
  >{/if}
{#if editing}<TestedPresetEditor
    initial={editing.initial}
    original={editing.original}
    onSave={saveEdited}
    onClose={() => (editing = null)}
  />{/if}

<style>
  .library {
    height: 100%;
    min-height: 0;
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3);
    overflow: auto;
  }
  header,
  footer,
  .collection-bar,
  .filters,
  .actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    align-items: center;
  }
  header {
    justify-content: space-between;
  }
  header > div:first-child {
    display: grid;
    gap: 4px;
  }
  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    min-height: var(--inspector-control-height);
    font-size: 12px;
  }
  .browse-collection {
    flex: 0 1 280px;
  }
  .browse-collection select {
    max-width: 100%;
    width: 100%;
  }
  .collection-count {
    font-size: 12px;
  }
  .collection-menu {
    margin-left: auto;
    max-width: 100%;
  }
  .collection-menu summary {
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    font-size: 12px;
    min-height: 28px;
  }
  .collection-menu summary span {
    max-width: 220px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .management {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding-block: 6px;
  }
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 220px;
    padding: 32px 16px;
    text-align: center;
    color: var(--text-muted);
    border-top: 1px solid var(--border);
  }
  .empty-state h2 {
    color: var(--text);
  }
  .empty-state p {
    max-width: 440px;
    line-height: 1.5;
  }
  header h1 {
    margin: 0 auto 0 0;
    font-size: 16px;
  }
  h2 {
    font-size: 14px;
    margin: 0;
    overflow-wrap: anywhere;
  }
  h3 {
    font-size: 13px;
    margin: 0;
  }
  p {
    font-size: 12px;
    margin: 0;
    overflow-wrap: anywhere;
  }
  .scope {
    color: var(--text-muted);
  }
  [role='alert'] {
    color: var(--error);
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 3px;
    font-size: 12px;
    min-width: 0;
  }
  select,
  input {
    max-width: 240px;
    min-width: 100px;
  }
  .filters {
    padding: 8px 0;
    border-block: 1px solid var(--border);
  }
  .content {
    display: flex;
    min-height: 180px;
    flex: 1;
    overflow: hidden;
    gap: 12px;
  }
  .table {
    flex: 1;
    overflow: auto;
    min-width: 0;
  }
  table {
    width: 100%;
    min-width: 720px;
    table-layout: fixed;
    border-collapse: collapse;
    font-size: 12px;
  }
  th,
  td {
    text-align: left;
    padding: 5px 8px;
    border-bottom: 1px solid var(--border);
    max-width: 220px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  th:first-child {
    width: 24%;
  }
  th {
    position: sticky;
    top: 0;
    background: var(--bg-panel);
  }
  td button {
    text-align: left;
    justify-content: flex-start;
    min-height: 22px;
    max-width: 100%;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    border: 0;
    background: transparent;
    padding: 3px 0;
    color: inherit;
  }
  tr.selected {
    background: var(--accent-tint);
  }
  aside {
    flex: 0 0 330px;
    overflow: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
    border-left: 1px solid var(--border);
    padding-left: 12px;
  }
  .notes {
    white-space: pre-wrap;
  }
  .preview {
    display: grid;
    gap: 8px;
    border-top: 1px solid var(--border);
    padding-top: 10px;
  }
  .dialog {
    padding: 16px;
    display: grid;
    gap: 12px;
    overflow: auto;
  }
  @media (max-width: 800px) {
    .content {
      flex-direction: column;
      overflow: auto;
    }
    .table {
      flex: none;
      max-height: 45vh;
    }
    aside {
      flex: none;
      border-left: 0;
      border-top: 1px solid var(--border);
      padding: 10px 0;
    }
    .filters label {
      flex: 1 1 130px;
    }
    select,
    input {
      max-width: 100%;
    }
  }
</style>
