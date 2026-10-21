<script lang="ts">
  import { t } from '../i18n';
  import { displayNumber } from '../state/recommendation-preview';
  import { comparisonValue } from '../state/cutting-comparison';
  let {
    current,
    proposed,
    pocket = false,
  }: {
    current: (number | undefined)[];
    proposed: (number | null | undefined)[];
    pocket?: boolean;
  } = $props();
  const labels = ['fs.rpm', 'fs.feed', 'fs.plunge', 'fs36.pass_depth', 'fs.stepover'] as const;
  const units = ['rpm', 'mm/min', 'mm/min', 'mm', '%'];
  const rows = $derived(
    labels.map((_, i) => comparisonValue(current[i], proposed[i], i !== 4 || pocket)),
  );
  const changed = $derived(rows.some((row) => row.changed));
</script>

<div class="table-scroll">
  <table aria-label={t('fs35.recommendation')}>
    <thead
      ><tr
        ><th>{t('preset3.parameter')}</th><th>{t('preset3.current')}</th><th
          title={t('fs35.suggested')}>{t('preset3.proposed')}</th
        ></tr
      ></thead
    >
    <tbody
      >{#each labels as label, i (label)}
        {@const row = rows[i]}
        <tr class:changed={row.changed} class:inactive={!row.applicable}>
          <th>{t(label)} <small>{units[i]}</small></th>
          <td title={row.applicable && row.current == null ? t('selector.unavailable') : undefined}
            >{!row.applicable
              ? t('selector.na')
              : row.current == null
                ? '—'
                : displayNumber(row.current, 3)}</td
          >
          <td
            title={row.retained
              ? t('selector.retained')
              : row.applicable && row.proposed == null
                ? t('selector.unavailable')
                : undefined}
            >{!row.applicable
              ? t('selector.na')
              : row.proposed == null
                ? '—'
                : displayNumber(row.proposed, 3)}{#if row.retained}<small
                aria-label={t('selector.retained')}
              >
                *</small
              >{/if}</td
          >
        </tr>
      {/each}</tbody
    >
  </table>
</div>
<p class="summary" role="status">{t(changed ? 'selector.changes' : 'selector.no_changes')}</p>
{#if rows.some((row) => row.retained)}<p class="summary">* {t('selector.retained')}</p>{/if}

<style>
  .table-scroll {
    overflow: auto;
    margin: var(--ui-space-2) 0;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--inspector-font-field);
    line-height: 1.35;
    font-variant-numeric: tabular-nums;
  }
  th,
  td {
    text-align: right;
    padding: var(--ui-space-1) var(--ui-space-2);
    border-bottom: 1px solid var(--border);
  }
  th {
    font-weight: 500;
  }
  thead th,
  small,
  .summary {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  th:first-child {
    text-align: left;
  }
  small {
    font-weight: normal;
  }
  .changed td:last-child {
    background: var(--accent-tint);
    font-weight: 600;
  }
  .inactive {
    color: var(--text-muted);
  }
  .summary {
    margin: var(--ui-space-1) 0;
    line-height: 1.35;
  }
</style>
