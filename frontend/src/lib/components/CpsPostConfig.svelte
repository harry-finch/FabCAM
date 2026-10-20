<script lang="ts">
  // `.cps` post picker + auto-generated properties form. Mounted by the
  // machine dialog when the dialect is `cps`; edits flow back through
  // the dialog's draft (`onchange`) so undo/cancel behave like every
  // other machine setting.
  //
  // V1 executes compiled bundled posts only. External settings are retained
  // for compatibility but never inspected on mount or silently replaced.

  import { t } from '../i18n/i18n.svelte';
  import { defaultClient } from '../api/http';
  import type { CpsPostConfig } from '../state/project-types';
  import type { PostListEntry } from '../api/types';
  import {
    formControls,
    sparseProperties,
    selectionMetadata,
    type PropertyControl,
  } from './cps-post-form';

  let {
    value,
    onchange,
  }: {
    value: CpsPostConfig | undefined;
    onchange: (next: CpsPostConfig | undefined) => void;
  } = $props();

  const client = defaultClient();

  let bundled = $state<PostListEntry[]>([]);
  const meta = $derived(selectionMetadata(value, bundled));
  let loadError = $state<string | null>(null);

  // Load the bundled library once; a transport without CPS support
  // simply reports the error (the option is hidden upstream anyway).
  $effect(() => {
    let cancelled = false;
    void (async () => {
      if (!client.listPosts) {
        loadError = t('machine.cps.unsupported');
        return;
      }
      try {
        const list = await client.listPosts();
        if (!cancelled) bundled = list;
      } catch (e) {
        if (!cancelled) loadError = e instanceof Error ? e.message : String(e);
      }
    })();
    return () => {
      cancelled = true;
    };
  });

  const controls = $derived<PropertyControl[]>(
    meta ? formControls(meta, value?.properties ?? {}) : [],
  );

  function selectBundled(id: string) {
    loadError = null;
    onchange(id ? { source: 'bundled', bundledId: id, properties: {} } : undefined);
  }

  function setProperty(name: string, next: boolean | number | string) {
    if (!value || !meta) return;
    onchange({
      ...value,
      properties: sparseProperties(meta, { ...value.properties, [name]: next }),
    });
  }
</script>

<div class="cps">
  <label title={t('machine.cps.post.title')}>
    {t('machine.cps.post')}
    <span class="field">
      <select
        value={value?.source === 'bundled' ? (value.bundledId ?? '') : ''}
        onchange={(e) => selectBundled((e.currentTarget as HTMLSelectElement).value)}
      >
        <option value="">{t('machine.cps.post.none')}</option>
        {#each bundled as entry (entry.id)}
          <option value={entry.id}>{entry.meta.description || entry.id}</option>
        {/each}
      </select>
    </span>
  </label>

  <div class="row">
    <button type="button" disabled title={t('machine.cps.external_unavailable')}>
      {t('machine.cps.open_file')}
    </button>
    {#if value?.source === 'file'}
      <span class="filename" title={value.filename}>{value.filename}</span>
    {/if}
  </div>

  {#if value?.source === 'file'}
    <p class="err">{t('machine.cps.external_unavailable')}</p>
  {/if}

  {#if loadError}
    <p class="err">{loadError}</p>
  {/if}

  {#if meta}
    <p class="meta">
      {meta.description}{meta.vendor ? ` — ${meta.vendor}` : ''} · .{meta.extension}
    </p>
    {#if controls.length > 0}
      <div class="section-title">{t('machine.cps.properties')}</div>
      {#each controls as control (control.name)}
        <label title={control.description}>
          {control.title}
          <span class="field">
            {#if control.kind === 'bool'}
              <input
                type="checkbox"
                checked={control.value as boolean}
                onchange={(e) =>
                  setProperty(control.name, (e.currentTarget as HTMLInputElement).checked)}
              />
            {:else if control.kind === 'enum'}
              <select
                value={String(control.value)}
                onchange={(e) =>
                  setProperty(control.name, (e.currentTarget as HTMLSelectElement).value)}
              >
                {#each control.values ?? [] as option (option.id)}
                  <option value={option.id}>{option.title}</option>
                {/each}
              </select>
            {:else if control.kind === 'number' || control.kind === 'integer'}
              <input
                type="number"
                step={control.kind === 'integer' ? 1 : 'any'}
                value={control.value as number}
                onchange={(e) => {
                  const raw = (e.currentTarget as HTMLInputElement).value;
                  const parsed = control.kind === 'integer' ? parseInt(raw, 10) : parseFloat(raw);
                  if (Number.isFinite(parsed)) setProperty(control.name, parsed);
                }}
              />
            {:else}
              <input
                type="text"
                value={String(control.value)}
                onchange={(e) =>
                  setProperty(control.name, (e.currentTarget as HTMLInputElement).value)}
              />
            {/if}
          </span>
        </label>
      {/each}
    {/if}
  {/if}
</div>

<style>
  .cps {
    display: grid;
    gap: var(--ui-space-2);
    min-width: 0;
  }
  label {
    display: grid;
    grid-template-columns: minmax(0, 9rem) minmax(0, 1fr);
    align-items: center;
    gap: var(--ui-space-2);
    font-size: 0.8rem;
  }
  .field {
    min-width: 0;
  }
  input:not([type='checkbox']),
  select {
    box-sizing: border-box;
    min-height: var(--inspector-control-height);
    max-width: 100%;
    min-width: 0;
    padding: var(--ui-space-1) var(--ui-space-2);
    background: var(--bg-input);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    font: inherit;
  }
  select,
  input[type='text'] {
    width: 100%;
  }
  input[type='number'] {
    width: 96px;
  }
  input[type='checkbox'] {
    accent-color: var(--accent);
  }
  .section-title {
    padding-top: var(--ui-space-2);
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .filename {
    font-size: 0.85em;
    opacity: 0.8;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .meta {
    margin: 0;
    font-size: 0.85em;
    opacity: 0.8;
  }
  .err {
    margin: 0;
    color: var(--danger);
    font-size: 0.85em;
  }
</style>
