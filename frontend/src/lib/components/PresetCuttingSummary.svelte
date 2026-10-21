<script lang="ts">
  import { t } from '../i18n';
  import type { PresetRow } from '../state/preset-library';
  let { row }: { row: PresetRow } = $props();
  const values = $derived([
    [t('fs.rpm'), `${row.profile.data.rpm} rpm`],
    [
      t(row.profile.data.kind === 'exact_values' ? 'fs.feed' : 'fs.chipload'),
      row.profile.data.kind === 'exact_values'
        ? `${row.profile.data.feed_mm_min} mm/min`
        : `${row.profile.data.chipload_mm_tooth} mm/tooth`,
    ],
    [t('fs.plunge'), `${row.profile.data.plunge_mm_min} mm/min`],
    [t('fs36.pass_depth'), `${row.profile.data.doc_mm} mm`],
    ...(row.profile.data.stepover_ratio == null
      ? []
      : [[t('fs.stepover'), `${row.profile.data.stepover_ratio * 100}%`]]),
  ]);
</script>

<dl class="cutting-summary">
  {#each values as [label, value] (label)}<div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>{/each}
</dl>

<style>
  .cutting-summary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2) var(--ui-space-4);
    margin: var(--ui-space-2) 0;
    font-variant-numeric: tabular-nums;
  }
  dt {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  dd {
    margin: var(--ui-space-1) 0 0;
    font-size: var(--inspector-font-field);
  }
</style>
