<script lang="ts">
  import { Layers, TriangleAlert } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { remainingView } from '../state/remaining-material';
  import { t } from '../i18n';
  let { stale }: { stale: boolean } = $props();
  const results = $derived(project.gen.generated?.remaining_material);
  const result = $derived(remainingView(results, stale, project.gen.remainingOpId));
  const reasonKeys = {
    metric_milling_only: 'cam4.reason.metric_milling_only',
    cascade_pocket_only: 'cam4.reason.cascade_pocket_only',
    single_flat_cutter_only: 'cam4.reason.single_flat_cutter_only',
    intentional_tabs: 'cam4.reason.intentional_tabs',
    intentional_allowance: 'cam4.reason.intentional_allowance',
    depth_schedule: 'cam4.reason.depth_schedule',
    target_representation: 'cam4.reason.target_representation',
    analytic_arc_path: 'cam4.reason.analytic_arc_path',
    execution_path_unavailable: 'cam4.reason.execution_path_unavailable',
    analysis_budget: 'cam4.reason.analysis_budget',
    missing_target: 'cam4.reason.missing_target',
    stock_depth_context: 'cam4.reason.stock_depth_context',
    target_outside_stock: 'cam4.reason.target_outside_stock',
  } as const;
  function reason() {
    return t(reasonKeys[result?.reason as keyof typeof reasonKeys] ?? 'cam4.unavailable');
  }
</script>

{#if project.gen.generated}
  <div class="remaining-material" aria-label={t('cam4.title')}>
    <Layers size={14} aria-hidden="true" />
    <span>{t('cam4.title')}</span>
    {#if results && results.length > 1}
      <select
        aria-label={t('cam4.operation')}
        value={result?.op_id ?? results[0].op_id}
        onchange={(e) => (project.gen.remainingOpId = Number(e.currentTarget.value))}
      >
        {#each results as r (r.op_id)}
          <option value={r.op_id}
            >{project.data.operations.find((op) => op.id === r.op_id)?.name ??
              `#${r.op_id}`}</option
          >
        {/each}
      </select>
    {/if}
    {#if stale}
      <span>{t('cam4.stale')}</span>
    {:else if result?.status === 'ready'}
      <span class:residual={result.regions.length > 0}>
        {#if result.regions.length > 0}<TriangleAlert size={14} aria-hidden="true" />{/if}
        {t('cam4.metrics', {
          count: result.regions.length,
          area: result.area_mm2.toFixed(2),
          depth: result.depth_mm.toFixed(2),
        })}
      </span>
      <button
        type="button"
        aria-pressed={project.gen.remainingVisible}
        onclick={() => (project.gen.remainingVisible = !project.gen.remainingVisible)}
      >
        {t(project.gen.remainingVisible ? 'cam4.hide' : 'cam4.show')}
      </button>
      <details>
        <summary>{t('cam4.details')}</summary>
        <span>{t('cam4.scope')}</span>
        <span>{t('cam4.largest', { area: result.largest_area_mm2.toFixed(2) })}</span>
      </details>
    {:else}
      <span>{t('cam4.not_analyzed')}</span>
      <details><summary>{t('cam4.details')}</summary><span>{reason()}</span></details>
    {/if}
  </div>
{/if}

<style>
  .remaining-material {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--ui-space-2);
    padding: var(--ui-space-1) var(--ui-space-3);
    background: var(--bg-panel);
    color: var(--text-muted);
    font-size: var(--inspector-font-field);
    border-bottom: 1px solid var(--border);
  }
  .residual {
    color: var(--warn);
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-1);
  }
  button,
  select {
    font-size: inherit;
  }
  select {
    max-width: 200px;
  }
  summary {
    cursor: pointer;
  }
  details[open] {
    flex-basis: 100%;
  }
  details span {
    display: block;
    margin-top: var(--ui-space-1);
  }
</style>
