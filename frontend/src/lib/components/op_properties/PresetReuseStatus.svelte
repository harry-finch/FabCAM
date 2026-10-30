<script lang="ts">
  import { t } from '../../i18n';
  import { project, type OpEntry } from '../../state/project.svelte';
  import { reuseRecords, reuseState } from '../../state/preset-reuse-metadata';
  let { op }: { op: OpEntry } = $props();
  const records = $derived(
    reuseRecords(project.data.serializedExtensions).filter((r) => r.operation_id === op.id),
  );
  const disclosures = $derived(
    records.map((record) => ({ record, status: reuseState(op, record, project.data) })),
  );
</script>

{#each disclosures as { record: r, status } (r.stage)}
  {#if status !== 'unassociated'}<p class="reuse-status">
      {t('reuse.disclosure', {
        stage: t(r.stage === 'main' ? 'stages.main' : 'stages.finishing'),
        source: `${r.source.collection_name} — ${r.source.preset_name}`,
        values: Object.entries(r.values)
          .map(
            ([field, value]) =>
              `${t(({ rpm: 'stages.rpm', feed_mm_min: 'stages.feed', plunge_mm_min: 'stages.plunge', depth_per_pass_mm: 'stages.depth' } as const)[field as keyof typeof r.values])} ${value}`,
          )
          .join(', '),
        state: t(`reuse.state.${status}`),
      })}
    </p>{/if}
{/each}

<style>
  .reuse-status {
    color: var(--text-muted);
    font-size: 0.9em;
    margin: 0.4rem 0;
  }
</style>
