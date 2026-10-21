<script lang="ts">
  import { t } from '../i18n';
  import type { PresetRow } from '../state/preset-library';
  import { rangeText } from '../state/preset-library';
  import type { MsgKey } from '../i18n/keys';
  let { row }: { row: PresetRow } = $props();
  const fields = $derived([
    [t('fs35.machine'), row.machine],
    [t('fs.material'), row.material],
    [t('presets.tool'), row.tool],
    [t('presets.diameter'), `${rangeText(row.profile.scope.diameter_mm)} mm`],
    [t('tools.col.flutes'), row.profile.scope.flute_count ?? '—'],
    [t('fs35.operation'), row.profile.scope.operation.replaceAll('_', ' ')],
    [t('presets.entry'), row.profile.scope.entry?.replaceAll('_', ' ') ?? '—'],
    [t('presets.thickness'), `${rangeText(row.profile.scope.thickness_mm)} mm`],
    [t('presets.aggressiveness'), row.profile.aggressiveness],
  ]);
  const statusKeys: Record<string, MsgKey> = {
    draft: 'fs38.draft',
    tested: 'fs35.tested',
    validated: 'fs35.validated',
    theoretical: 'fs35.theoretical',
    reference_data: 'fs35.reference',
    deleted: 'presets.deleted',
  };
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex (Focusable record region supports native keyboard scrolling.) -->
<section class="record" aria-label={t('presets.details')} tabindex="0">
  <h2>{row.name}</h2>
  <p class="status">
    {t(statusKeys[row.state])} · {t(
      row.record?.lab ? 'presets.local_readonly' : 'presets.reference_readonly',
    )}
  </p>
  {#if !row.enabled}<p class="notice">{t('presets.disabled')}</p>{/if}
  {#if !row.milling}<p class="notice">{t('presets.milling_only')}</p>{/if}
  {#if row.record?.lab && !row.record.lab.enabled}<p class="notice">
      {t('presets.inactive_record')}
    </p>{/if}
  <h3>{t('presets.context')}</h3>
  <dl>
    {#each fields as field (field[0])}<dt>{field[0]}</dt>
      <dd>{field[1]}</dd>{/each}
  </dl>
  <h3>{t('presets.stored_values')}</h3>
  <dl>
    <dt>{t('fs.rpm')}</dt>
    <dd>{row.profile.data.rpm} rpm</dd>
    <dt>{t('fs.feed')}</dt>
    <dd>
      {row.profile.data.kind === 'exact_values' ? `${row.profile.data.feed_mm_min} mm/min` : '—'}
    </dd>
    {#if row.profile.data.kind === 'chipload_target'}<dt>{t('presets.chipload')}</dt>
      <dd>{row.profile.data.chipload_mm_tooth} mm/tooth</dd>{/if}
    <dt>{t('fs.plunge')}</dt>
    <dd>{row.profile.data.plunge_mm_min} mm/min</dd>
    <dt>{t('fs36.pass_depth')}</dt>
    <dd>{row.profile.data.doc_mm} mm</dd>
    <dt>{t('fs.stepover')}</dt>
    <dd>
      {row.profile.data.stepover_ratio == null
        ? '—'
        : `${Number((row.profile.data.stepover_ratio * 100).toFixed(4))} %`}
    </dd>
  </dl>
  {#if row.profile.data.kind === 'chipload_target'}<p>{t('presets.no_calculation')}</p>{/if}
  <h3>{t('presets.provenance')}</h3>
  <dl>
    <dt>{t('presets.pack')}</dt>
    <dd>
      {row.pack.manifest.name}<small
        >{row.pack.manifest.identity.id} @{row.pack.manifest.identity.revision}</small
      >
    </dd>
    <dt>{t('presets.record')}</dt>
    <dd>{row.profile.identity.id} @{row.profile.identity.revision}</dd>
    <dt>{t('presets.author')}</dt>
    <dd>{row.profile.source.author} · {row.profile.source.date}</dd>
    <dt>{t('presets.source')}</dt>
    <dd>{row.profile.source.reference}</dd>
    <dt>{t('presets.license')}</dt>
    <dd>{row.profile.source.permission_or_license}</dd>
  </dl>
  <p>{row.pack.manifest.attribution}</p>
  {#if row.profile.source.notes}
    {#if row.profile.source.notes.length > 1200}<details>
        <summary>{t('preset2.source_notes')}</summary>
        <p class="notes">{row.profile.source.notes}</p>
      </details>
    {:else}<p class="notes">{row.profile.source.notes}</p>{/if}
  {/if}
  {#if row.record?.lab}
    <h3>{t('presets.trial')}</h3>
    <dl>
      <dt>{t('fs38.operator')}</dt>
      <dd>{row.record.lab.operator || '—'}</dd>
      <dt>{t('fs38.job_reference')}</dt>
      <dd>{row.record.lab.job_reference || '—'}</dd>
      <dt>{t('fs38.test_date')}</dt>
      <dd>{row.record.lab.tested_at ?? '—'}</dd>
    </dl>
    <p class="notes">{row.record.lab.observations}</p>
    <p class="notes">{row.record.lab.detailed_notes}</p>
    {#if row.record.lab.review}<p class="notes">
        {row.record.lab.review.reviewed_at} · {row.record.lab.review.notes}
      </p>{/if}
  {/if}
  {#if row.profile.source.evidence_record}<p>
      {row.profile.source.evidence_record.trial_reference}<br />{row.profile.source.evidence_record
        .validation_criteria_reference ?? ''}
    </p>{/if}
  <details>
    <summary>{t('presets.identities')}</summary>
    <pre>{JSON.stringify(row.profile.scope, null, 2)}</pre>
  </details>
  {#if row.record?.lab_history?.length}<details>
      <summary>{t('fs38.history')} · {row.record.lab_history.length}</summary>
      {#each row.record.lab_history as previous (previous.profile.identity.revision)}<p
          class="notes"
        >
          {previous.profile.identity.revision} · {previous.metadata.state} · {previous.metadata
            .updated_at}<br />{previous.metadata.observations}
        </p>{/each}
    </details>{/if}
</section>

<style>
  .record {
    min-width: 0;
    overflow: auto;
    padding: var(--ui-space-4);
    background: var(--bg-panel);
  }
  h2 {
    font-size: 1rem;
    margin: 0 0 var(--ui-space-2);
    overflow-wrap: anywhere;
  }
  h3 {
    font-size: var(--inspector-font-meta);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid var(--border);
    margin: var(--ui-space-4) 0 var(--ui-space-2);
    padding-bottom: var(--ui-space-1);
  }
  p,
  dl,
  details {
    font-size: var(--inspector-font-field);
    line-height: 1.45;
  }
  p,
  dd {
    overflow-wrap: anywhere;
  }
  .status,
  small,
  dt {
    color: var(--text-muted);
  }
  .notice {
    color: var(--warn);
  }
  dl {
    display: grid;
    grid-template-columns: minmax(90px, 1fr) minmax(0, 2fr);
    gap: var(--ui-space-2);
  }
  dd {
    margin: 0;
  }
  small {
    display: block;
  }
  .notes {
    white-space: pre-wrap;
  }
  pre {
    font-size: var(--inspector-font-meta);
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  summary {
    cursor: pointer;
  }
</style>
