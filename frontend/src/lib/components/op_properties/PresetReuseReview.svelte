<script lang="ts">
  import { onDestroy } from 'svelte';
  import { t } from '../../i18n';
  import { project, type OpEntry } from '../../state/project.svelte';
  import type { PresetCollection } from '../../state/preset-collections';
  import {
    preparePresetReuse,
    confirmPresetReuse,
    type ReuseReview,
  } from '../../state/preset-reuse';
  import { reuseFields, type ReuseField, type ReuseStage } from '../../state/preset-reuse-metadata';
  let {
    op,
    collection,
    token,
    presetId,
    onApplied,
  }: {
    op: OpEntry;
    collection: PresetCollection;
    token: string;
    presetId: string;
    onApplied: () => void;
  } = $props();
  const source = $derived(collection.presets.find((p) => p.id === presetId));
  let open = $state(false),
    busy = $state(false),
    error = $state('');
  let sourceStage = $state<ReuseStage>('main'),
    destination = $state<ReuseStage | ''>('');
  let fields = $state<ReuseField[]>([...reuseFields]),
    review = $state<ReuseReview | null>(null);
  let alive = true,
    sequence = 0;
  onDestroy(() => {
    alive = false;
    ++sequence;
  });
  const labels = {
    rpm: 'stages.rpm',
    feed_mm_min: 'stages.feed',
    plunge_mm_min: 'stages.plunge',
    depth_per_pass_mm: 'stages.depth',
  } as const;
  const contextKeys = [
    'operation',
    'stage',
    'machine_label',
    'material_label',
    'tool_kind',
    'diameter_mm',
    'strategy',
    'surface_scope',
  ] as const;
  function invalidate() {
    ++sequence;
    review = null;
    error = '';
  }
  async function preview() {
    if (!destination) {
      error = 'reuse.error.stage';
      return;
    }
    const request = ++sequence;
    busy = true;
    error = '';
    review = null;
    try {
      const next = await preparePresetReuse(
        project,
        op.id,
        { data: collection, token },
        presetId,
        sourceStage,
        destination,
        fields,
      );
      if (alive && request === sequence) review = next;
    } catch (e) {
      if (alive && request === sequence) error = String(e).replace(/^Error: /, '');
    } finally {
      if (alive && request === sequence) busy = false;
    }
  }
  async function apply() {
    if (!review) return;
    busy = true;
    error = '';
    try {
      await confirmPresetReuse(project, review);
      if (alive) onApplied();
    } catch (e) {
      if (alive) {
        error = String(e).replace(/^Error: /, '');
        review = null;
      }
    } finally {
      if (alive) busy = false;
    }
  }
  $effect(() => {
    void op;
    invalidate();
  });
</script>

{#if collection.version === 2 && source?.parameters.stages && op.stages && ['profile', 'pocket'].includes(op.kind)}
  {#if !open}<button
      class="btn-secondary"
      onclick={() => {
        open = true;
        invalidate();
      }}>{t('reuse.action')}</button
    >{:else}
    <section class="reuse-review" aria-label={t('reuse.action')}>
      <div class="actions">
        <strong>{t('reuse.action')}</strong><button
          class="btn-secondary"
          disabled={busy}
          onclick={() => {
            open = false;
            invalidate();
          }}>{t('fsreset.ui.cancel')}</button
        >
      </div>
      <p class="basis">{t('reuse.notice')}</p>
      <label
        >{t('reuse.destination')}<select
          disabled={busy}
          bind:value={destination}
          onchange={invalidate}
        >
          <option value="">{t('reuse.choose_stage')}</option><option value="main"
            >{t('stages.main')}</option
          >
          {#if op.stages.finishing}<option value="finishing">{t('stages.finishing')}</option>{/if}
        </select></label
      >
      <label
        >{t('reuse.source_stage')}<select
          disabled={busy}
          bind:value={sourceStage}
          onchange={invalidate}
        >
          <option value="main">{t('stages.main')}</option
          >{#if source.parameters.stages.finishing}<option value="finishing"
              >{t('stages.finishing')}</option
            >{/if}
        </select></label
      >
      <fieldset disabled={busy}>
        <legend>{t('reuse.fields')}</legend>
        {#each reuseFields as field (field)}<label class="field-choice"
            ><input
              type="checkbox"
              checked={fields.includes(field)}
              onchange={(e) => {
                fields = e.currentTarget.checked
                  ? [...fields, field]
                  : fields.filter((f) => f !== field);
                invalidate();
              }}
            />{t(labels[field])}</label
          >{/each}
      </fieldset>
      <p class="basis">{t('reuse.excluded')}</p>
      <button
        class="btn-secondary"
        disabled={busy || !destination || !fields.length}
        onclick={preview}>{t('reuse.review')}</button
      >
      {#if error}<p class="error" role="alert">{t(error as Parameters<typeof t>[0])}</p>{/if}
      {#if review}
        <table>
          <thead
            ><tr
              ><th>{t('reuse.parameter')}</th><th>{t('reuse.current')}</th><th
                >{t('reuse.proposed')}</th
              ></tr
            ></thead
          ><tbody>
            {#each review.fields as field (field)}<tr
                ><th>{t(labels[field])}</th><td>{review.before[field] ?? '—'}</td><td
                  >{review.record.values[field]}
                  {field === 'rpm' ? 'rpm' : field === 'depth_per_pass_mm' ? 'mm' : 'mm/min'}</td
                ></tr
              >{/each}
          </tbody>
        </table>
        <table>
          <thead
            ><tr
              ><th>{t('reuse.context')}</th><th>{t('reuse.source')}</th><th>{t('reuse.target')}</th
              ></tr
            ></thead
          ><tbody>
            {#each contextKeys as key (key)}<tr
                ><th>{t(`reuse.context.${key}`)}</th><td
                  >{review.record.source.context[key] ?? t('reuse.unknown')}</td
                ><td>{review.record.target_context[key] ?? t('reuse.unknown')}</td></tr
              >{/each}
          </tbody>
        </table>
        <details>
          <summary>{t('reuse.identities')}</summary>
          {#each ['machine_id', 'material_id', 'material_revision', 'tool_id', 'tool_physical_id'] as key (key)}<p
              class="basis"
            >
              {t(`reuse.context.${key}` as Parameters<typeof t>[0])}: {review.record.source.context[
                key
              ] ?? t('reuse.unknown')} → {review.record.target_context[key] ?? t('reuse.unknown')}
            </p>{/each}
          <p class="basis">
            {review.record.source.revision} · {review.record.source.content_sha256}
          </p>
        </details>
        <p class="basis">{t('reuse.notice')}</p>
        <button class="btn-primary" disabled={busy} onclick={apply}>{t('reuse.confirm')}</button>
      {/if}
    </section>
  {/if}
{/if}

<style>
  .reuse-review {
    border-top: 1px solid var(--border);
    padding-top: 0.7rem;
    margin-top: 0.7rem;
    font-size: var(--font-size-ui, 13px);
  }
  .actions,
  .field-choice {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    margin: 0.5rem 0;
  }
  select {
    min-width: 0;
    max-width: 70%;
  }
  fieldset {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
  }
  .basis {
    color: var(--text-muted);
    font-size: 0.9em;
    overflow-wrap: anywhere;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    margin: 0.6rem 0;
    table-layout: fixed;
  }
  th,
  td {
    text-align: left;
    border-bottom: 1px solid var(--border);
    padding: 0.35rem;
    overflow-wrap: anywhere;
    font-size: 0.9em;
  }
  .error {
    color: var(--error);
  }
</style>
