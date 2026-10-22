<script lang="ts">
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
  const pages = $derived(Math.max(1, Math.ceil(filtered.length / 50)));
  const shown = $derived(
    filtered.slice(Math.min(page, pages - 1) * 50, (Math.min(page, pages - 1) + 1) * 50),
  );
  const selected = $derived(rows.find((r) => r.key === selectedKey));
  const collection = $derived(collections.find((c) => c.data.id === collectionId));
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
      await refresh();
    });
  }
  function edit(row: TestedRow, duplicate = false) {
    const stored = collections.find((c) => c.data.id === row.collection.id)!;
    const original = structuredClone(row.preset);
    const initial = structuredClone(original);
    if (duplicate) {
      initial.id = crypto.randomUUID();
      initial.name = `${initial.name.slice(0, 145)} (copy)`;
    }
    editing = {
      initial,
      original,
      collection: structuredClone(row.collection),
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
      collection: structuredClone(target.data),
      token: target.token,
    };
  }
  async function saveEdited(preset: TestedPreset) {
    if (!editing) throw new Error('Editor closed');
    const c = structuredClone(editing.collection),
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
    const plan = importChoice(incoming, collision, choice);
    await saveCollection(plan.data, plan.expected);
    incoming = null;
    collectionId = plan.data.id;
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
    <h1>Tested presets</h1>
    <button disabled={busy} onclick={() => void run(refresh)}>Refresh</button><button
      disabled={busy}
      onclick={() =>
        void run(async () => {
          incoming = await importCollectionFile();
        })}>Import collection</button
    >
  </header>
  <p class="scope">
    Workshop-tested configurations in named collections. Author attestations are separate from
    software checks and do not qualify this project's machine.
  </p>
  <div class="collection-bar">
    <label
      >Collection<select bind:value={collectionId}
        ><option value="">Choose…</option>{#each collections as c (c.data.id)}<option
            value={c.data.id}>{c.data.name}</option
          >{/each}</select
      ></label
    >
    <button
      disabled={busy}
      onclick={() => {
        naming = 'create';
        name = '';
      }}>New collection</button
    >
    <button
      disabled={busy || !collection}
      onclick={() => {
        naming = 'rename';
        name = collection!.data.name;
      }}>Rename</button
    >
    <button
      disabled={busy || !collection}
      onclick={() =>
        void run(async () => {
          if (await exportCollectionFile(collection!.data)) message = 'Collection export requested';
        })}>Export</button
    >
    <button
      disabled={busy || !collection}
      onclick={() =>
        (deletion = { collection: structuredClone(collection!.data), token: collection!.token })}
      >Delete collection</button
    >
    <button
      disabled={busy || !collection || project.sel.selectedOpId == null}
      onclick={() => void run(createFromOperation)}>Save selected operation as preset</button
    >
  </div>
  <div class="filters">
    <label
      >Search<input
        type="search"
        bind:value={filters.query}
        placeholder="Name, identity, material…"
      /></label
    >
    <label
      >Collection<select bind:value={filters.collection}
        ><option value="">All</option>{#each collections as c (c.data.id)}<option value={c.data.id}
            >{c.data.name}</option
          >{/each}</select
      ></label
    >
    <label
      >Machine<select bind:value={filters.machine}
        ><option value="">All</option
        >{#each [...new Map(rows.map( (r) => [r.preset.context.machine_id, r.preset.context.machine_label], )).entries()] as [id, label] (id)}<option
            value={id}>{label}</option
          >{/each}</select
      ></label
    >
    <label
      >Tool<select bind:value={filters.tool}
        ><option value="">All</option
        >{#each [...new Set(rows.map((r) => r.tools))] as tool (tool)}<option>{tool}</option
          >{/each}</select
      ></label
    >
    <label
      >Material<select bind:value={filters.material}
        ><option value="">All</option
        >{#each [...new Map(rows.map( (r) => [r.preset.context.material_id, r.preset.context.material_label], )).entries()] as [id, label] (id)}<option
            value={id}>{label}</option
          >{/each}</select
      ></label
    >
    <label
      >Operation<select bind:value={filters.operation}
        ><option value="">All</option
        >{#each [...new Set(rows.map((r) => r.operation))] as op (op)}<option>{op}</option
          >{/each}</select
      ></label
    >
  </div>
  {#if error}<p role="alert">{error}</p>{/if}{#if message}<p role="status">{message}</p>{/if}
  <div class="content">
    <div class="table">
      <table>
        <thead
          ><tr
            ><th>Preset</th><th>Collection</th><th>Machine</th><th>Tool</th><th>Material</th><th
              >Operation</th
            ></tr
          ></thead
        ><tbody>
          {#each shown as row, index (row.key)}<tr class:selected={selectedKey === row.key}
              ><td
                ><button
                  data-record
                  aria-pressed={selectedKey === row.key}
                  onclick={() => select(row)}
                  onkeydown={(e) => navigate(e, index)}>{row.preset.name}</button
                ></td
              ><td>{row.collection.name}</td><td>{row.preset.context.machine_label}</td><td
                >{row.tools}</td
              ><td>{row.preset.context.material_label}</td><td>{row.operation}</td></tr
            >{/each}
        </tbody>
      </table>
      {#if !shown.length}<p>
          No tested presets match. Create a collection, then save an operation you have actually
          tested, or import a current collection.
        </p>{/if}
    </div>
    {#if selected}<aside>
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
        {#if selected.preset.notes}<p class="notes">{selected.preset.notes}</p>{/if}
        <div class="actions">
          <button disabled={busy} onclick={() => edit(selected)}>Edit</button><button
            disabled={busy}
            onclick={() => edit(selected, true)}>Duplicate</button
          ><button
            disabled={busy}
            onclick={() => {
              const stored = collections.find((c) => c.data.id === selected.collection.id)!;
              deletion = {
                collection: structuredClone(stored.data),
                token: stored.token,
                preset: structuredClone(selected.preset),
              };
            }}>Delete preset</button
          >
        </div>
        <button
          disabled={busy || project.sel.selectedOpId == null}
          onclick={() =>
            void run(async () => {
              preview = null;
              preview = await preparePresetLoad(
                project,
                project.sel.selectedOpId!,
                selected.collection,
                selected.preset.id,
              );
            })}>Preview load into selected operation</button
        >
        {#if preview}<section class="preview">
            <h3>Reviewed replacement</h3>
            {#each preview.after as rate (rate.role)}<p>
                {rate.role}: {preview.before.find((r) => r.role === rate.role)?.rpm ?? '—'} → {rate.rpm}
                rpm; feed {preview.before.find((r) => r.role === rate.role)?.feed_mm_min ?? '—'} → {rate.feed_mm_min};
                plunge {preview.before.find((r) => r.role === rate.role)?.plunge_mm_min ?? '—'} → {rate.plunge_mm_min}
                mm/min
              </p>{/each}
            {#each parameterSummary(preview.parameters) as detail (detail)}<p>{detail}</p>{/each}
            <button
              disabled={busy}
              onclick={() =>
                void run(async () => {
                  project.history.exec(preview!.command, project.target());
                  preview = null;
                  message = 'Values copied into the reviewed operation';
                })}>Replace operation parameters</button
            ><button onclick={() => (preview = null)}>Cancel</button>
          </section>{/if}
      </aside>{/if}
  </div>
  <footer>
    <span>{filtered.length} presets · page {Math.min(page, pages - 1) + 1}/{pages}</span><button
      disabled={page <= 0}
      onclick={() => page--}>Previous</button
    ><button disabled={page >= pages - 1} onclick={() => page++}>Next</button>
  </footer>
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
      <label>Name<input bind:value={name} required maxlength="160" /></label>{#if error}<p
          role="alert"
        >
          {error}
        </p>{/if}<button disabled={busy}>Save collection</button><button
        type="button"
        disabled={busy}
        onclick={() => (naming = null)}>Cancel</button
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
      {#if error}<p role="alert">{error}</p>{/if}<button disabled={busy} onclick={confirmDelete}
        >Delete</button
      ><button disabled={busy} onclick={() => (deletion = null)}>Cancel</button>
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
      {#if error}<p role="alert">{error}</p>{/if}{#if collision}<p>
          Collection ID already exists as “{collision.data.name}”. Replace that reviewed collection
          or create an independent copy.
        </p>
        <button disabled={busy} onclick={() => void run(() => importSelected('replace'))}
          >Replace “{collision.data.name}”</button
        ><button disabled={busy} onclick={() => void run(() => importSelected('copy'))}
          >Import independent copy</button
        >{:else}<button disabled={busy} onclick={() => void run(() => importSelected('new'))}
          >Import collection</button
        >{/if}<button disabled={busy} onclick={() => (incoming = null)}>Cancel</button>
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
    gap: 10px;
    padding: 14px;
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
  header h1 {
    margin: 0 auto 0 0;
    font-size: 18px;
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
    border-block: 1px solid var(--border-subtle);
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
    border-collapse: collapse;
    font-size: 12px;
  }
  th,
  td {
    text-align: left;
    padding: 7px;
    border-bottom: 1px solid var(--border-subtle);
    max-width: 220px;
    overflow-wrap: anywhere;
  }
  th {
    position: sticky;
    top: 0;
    background: var(--bg-panel);
  }
  td button {
    text-align: left;
    overflow-wrap: anywhere;
  }
  tr.selected {
    background: var(--bg-hover);
  }
  aside {
    flex: 0 0 330px;
    overflow: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
    border-left: 1px solid var(--border-subtle);
    padding-left: 12px;
  }
  .notes {
    white-space: pre-wrap;
  }
  .preview {
    display: grid;
    gap: 8px;
    border-top: 1px solid var(--border-subtle);
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
      border-top: 1px solid var(--border-subtle);
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
