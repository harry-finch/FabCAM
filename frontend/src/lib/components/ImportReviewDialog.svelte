<script lang="ts">
  /// Import summary (ADR-0026): source, units, resulting size, geometry
  /// count and grouped importer warnings. Confirm mode gates the open-file
  /// flows; view mode re-opens the summary of an imported source. Mounted
  /// once at the App root; renders nothing when no review is pending.
  import Modal from './Modal.svelte';
  import { importReview } from '../state/import-review';
  import {
    UNIT_SCALES,
    choiceForScale,
    groupImportWarnings,
    parseCustomScale,
    previewSize,
    unitChoicesFor,
    unitStatus,
    unitsAdjustable,
    type UnitChoice,
    type WarningGroup,
  } from '../state/import-units';
  import { notificationText } from '../api/error-notification';
  import { t } from '../i18n';

  const STATUS_LABEL = {
    unknown: () => t('import.units.status.unknown'),
    declared: () => t('import.units.status.declared'),
    physical: () => t('import.units.status.physical'),
    assumed: () => t('import.units.status.assumed'),
    explicit: () => t('import.units.status.explicit'),
  } as const;
  const STATUS_HINT = {
    unknown: () => t('import.units.status_hint.unknown'),
    declared: () => t('import.units.status_hint.declared'),
    physical: () => t('import.units.status_hint.physical'),
    assumed: () => t('import.units.status_hint.assumed'),
    explicit: () => t('import.units.status_hint.explicit'),
  } as const;
  const CHOICE_LABEL: Record<UnitChoice, () => string> = {
    mm: () => t('import.units.choice.mm'),
    px96: () => t('import.units.choice.px96'),
    px72: () => t('import.units.choice.px72'),
    inch: () => t('import.units.choice.inch'),
    custom: () => t('import.units.choice.custom'),
  };

  const req = $derived($importReview);
  const r = $derived(req?.result);
  const confirmMode = $derived(!!req?.rescale);
  const adjustable = $derived(!!r && confirmMode && unitsAdjustable(r));

  let choice = $state<UnitChoice>('mm');
  let customText = $state('');
  let busy = $state(false);
  let error = $state<string | null>(null);
  let seeded: object | null = null;

  // Seed the controls from each new result (identity-checked, not reactive
  // to the user's own edits).
  $effect(() => {
    if (r && r !== seeded) {
      seeded = r;
      choice = choiceForScale(r.unit_scale, r.format);
      customText = String(r.unit_scale);
      error = null;
      busy = false;
    }
  });

  const scale = $derived.by((): number | null => {
    if (!r) return null;
    if (!adjustable) return r.unit_scale;
    return choice === 'custom' ? parseCustomScale(customText) : UNIT_SCALES[choice];
  });
  const size = $derived(r && scale !== null ? previewSize(r, scale) : null);
  const groups = $derived(r ? groupImportWarnings(r.warnings) : []);
  const total = $derived(groups.reduce((n, g) => n + g.count, 0));

  function mm(v: number): string {
    return v.toLocaleString(undefined, { maximumFractionDigits: v < 10 ? 3 : 2 });
  }

  function warningText(g: WarningGroup): string {
    switch (g.kind) {
      case 'unsupported':
        return t('import.warn.unsupported', { entity: g.entity ?? '?', layer: g.layer ?? '?' });
      case 'missing_block':
        return t('import.warn.missing_block', { block: g.block ?? '?' });
      case 'geometry':
        return t('import.warn.geometry', { entity: g.entity ?? '?', layer: g.layer ?? '—' });
      case 'info':
        return t('import.warn.info');
      case 'units':
        return t('import.warn.units');
      default:
        return g.message;
    }
  }

  function cancel() {
    importReview.finish(null);
  }

  async function confirm() {
    if (!req || !r) return;
    if (!adjustable) {
      importReview.finish(r);
      return;
    }
    if (scale === null || !req.rescale) {
      error = t('import.units.custom_invalid');
      return;
    }
    // Re-import with the explicit scale even for 1:1: the confirmed choice is
    // recorded as `explicit` and curve sampling keeps its budget.
    busy = true;
    error = null;
    try {
      importReview.finish(await req.rescale(scale));
    } catch (e) {
      error = notificationText(e);
      busy = false;
    }
  }
</script>

{#if req && r}
  <Modal
    onClose={cancel}
    width="min(34rem, 96vw)"
    maxHeight="min(42rem, 92vh)"
    ariaLabelledBy="import-review-title"
  >
    <header class="header">
      <h2 id="import-review-title">{t('import.review.title')}</h2>
      <span class="file" title={r.filename}>{r.filename}</span>
    </header>
    <div class="body">
      <dl class="facts">
        <dt>{t('import.review.type')}</dt>
        <dd>{r.format.toUpperCase()}</dd>
        <dt>{t('import.review.detected')}</dt>
        <dd>
          <span class="source">{r.unit_source || t('import.units.source_unknown')}</span>
          <span
            class="status"
            class:assumed={unitStatus(r) === 'assumed' || unitStatus(r) === 'unknown'}
            title={STATUS_HINT[unitStatus(r)]()}>{STATUS_LABEL[unitStatus(r)]()}</span
          >
        </dd>
        <dt>{t('import.review.scale')}</dt>
        <dd>
          {scale !== null ? t('import.review.scale_value', { mm: mm(scale) }) : '—'}
        </dd>
        <dt>{t('import.review.size')}</dt>
        <dd class="size" data-testid="import-size">
          {size ? t('import.review.size_value', { w: mm(size.width), h: mm(size.height) }) : '—'}
        </dd>
        <dt>{t('import.review.geometry')}</dt>
        <dd>
          <!-- The HTTP transport does not return object metadata: segments only. -->
          {r.object_meta?.length
            ? t('import.review.geometry_value', {
                objects: r.object_meta.length,
                segments: r.segments.length,
              })
            : t('import.review.geometry_segments', { segments: r.segments.length })}
        </dd>
      </dl>

      {#if adjustable}
        <fieldset class="units">
          <legend title={t('import.units.legend_hint')}>{t('import.units.legend')}</legend>
          {#each unitChoicesFor(r.format) as c (c)}
            <label class="choice">
              <input type="radio" name="import-units" value={c} bind:group={choice} />
              <span>{CHOICE_LABEL[c]()}</span>
            </label>
          {/each}
          {#if choice === 'custom'}
            <label class="custom">
              <span>{t('import.units.custom_label')}</span>
              <input
                type="text"
                inputmode="decimal"
                bind:value={customText}
                aria-invalid={scale === null}
                aria-describedby="import-units-note"
              />
            </label>
          {/if}
          <p id="import-units-note" class="note">
            {scale === null ? t('import.units.custom_invalid') : t('import.units.note')}
          </p>
        </fieldset>
      {/if}

      <section class="warnings" aria-label={t('import.review.warnings')}>
        <h3>
          {total === 0
            ? t('import.review.no_warnings')
            : t('import.review.warning_count', { n: total })}
        </h3>
        {#if groups.length > 0}
          <ul>
            {#each groups as g (g.message)}
              <li class:info={g.kind === 'info'}>
                <details>
                  <summary>
                    <span class="msg">{warningText(g)}</span>
                    {#if g.count > 1}<span class="count">×{g.count}</span>{/if}
                  </summary>
                  <code>{g.message}</code>
                </details>
              </li>
            {/each}
          </ul>
        {/if}
      </section>
      {#if error}<p class="error" role="alert">{error}</p>{/if}
    </div>
    <div class="actions">
      {#if confirmMode}
        <button type="button" class="btn-secondary" onclick={cancel} disabled={busy}
          >{t('import.review.cancel')}</button
        >
        <button
          type="button"
          class="btn-primary"
          onclick={confirm}
          disabled={busy || scale === null}
        >
          {adjustable ? t('import.review.confirm_units') : t('import.review.confirm')}
        </button>
      {:else}
        <button type="button" class="btn-secondary" onclick={cancel}
          >{t('import.review.close')}</button
        >
      {/if}
    </div>
  </Modal>
{/if}

<style>
  .header {
    display: flex;
    align-items: baseline;
    gap: 0.6rem;
    padding: 0.7rem 0.9rem 0.3rem;
  }
  h2 {
    margin: 0;
    font-size: 1.05rem;
    color: var(--text-strong);
  }
  .file {
    color: var(--text-muted);
    font-size: 0.85rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .body {
    padding: 0.2rem 0.9rem 0.6rem;
    overflow: auto;
  }
  .facts {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 0.25rem 0.8rem;
    margin: 0 0 0.6rem;
    font-size: 0.85rem;
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    color: var(--text);
    min-width: 0;
    overflow-wrap: anywhere;
  }
  .size {
    font-variant-numeric: tabular-nums;
    color: var(--text-strong);
  }
  .status {
    margin-left: 0.4rem;
    padding: 0 0.35rem;
    border-radius: 3px;
    font-size: 0.75rem;
    background: var(--accent-tint);
  }
  .status.assumed {
    background: var(--warn-bg);
    color: var(--warn);
  }
  .units {
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0.35rem 0.6rem 0.4rem;
    margin: 0 0 0.6rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem 0.9rem;
    font-size: 0.85rem;
  }
  legend {
    color: var(--text-muted);
    padding: 0 0.25rem;
  }
  .choice,
  .custom {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
  }
  .custom input {
    width: 7rem;
  }
  .custom input[aria-invalid='true'] {
    border-color: var(--danger);
  }
  .note {
    flex-basis: 100%;
    margin: 0.2rem 0 0;
    color: var(--text-muted);
    font-size: 0.78rem;
  }
  .warnings h3 {
    margin: 0 0 0.3rem;
    font-size: 0.85rem;
    color: var(--text-strong);
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    font-size: 0.82rem;
  }
  li {
    border-left: 2px solid var(--warn);
    padding: 0.15rem 0 0.15rem 0.5rem;
    margin-bottom: 0.2rem;
  }
  li.info {
    border-left-color: var(--border);
  }
  summary {
    cursor: pointer;
    display: flex;
    gap: 0.4rem;
  }
  .count {
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }
  code {
    display: block;
    margin-top: 0.2rem;
    color: var(--text-muted);
    white-space: pre-wrap;
    font-size: 0.75rem;
  }
  .error {
    color: var(--danger);
    font-size: 0.82rem;
    margin: 0.4rem 0 0;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 0.5rem 0.9rem 0.7rem;
    border-top: 1px solid var(--border);
  }
</style>
