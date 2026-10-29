<script lang="ts">
  import type { OperationTemplate } from '../state/operation-templates';
  import { templateParameterRows } from '../state/operation-template-ui';
  import { t } from '../i18n';
  let { template }: { template: OperationTemplate } = $props();
  const units = { rpm: 'rpm', feed: 'mm/min' };
  const rows = $derived(templateParameterRows(template));
  const finish = $derived(template.finishTool ?? template.tool);
  const differentFinish = $derived(
    !!template.finishTool || JSON.stringify(template.tool.rough) !== JSON.stringify(finish.finish),
  );
</script>

{#snippet rates(values: {
  rpm: number | null;
  feed_mm_min: number | null;
  plunge_mm_min: number | null;
})}
  <dl>
    <div>
      <dt>{t('surface.rpm')}</dt>
      <dd>{values.rpm} {units.rpm}</dd>
    </div>
    <div>
      <dt>{t('surface.feed_mm_min')}</dt>
      <dd>{values.feed_mm_min} {units.feed}</dd>
    </div>
    <div>
      <dt>{t('surface.plunge_mm_min')}</dt>
      <dd>{values.plunge_mm_min} {units.feed}</dd>
    </div>
  </dl>
{/snippet}
<details class="parameter-preview">
  <summary>{t('templates.preview')}</summary>
  <dl>
    {#each rows as row (row.label)}<div>
        <dt>{t(row.label)}</dt>
        <dd>{row.value}</dd>
      </div>{/each}
  </dl>
  {#if template.stages}
    <h4>{t('stages.main')}</h4>
    {@render rates(template.stages.main)}
    {#if template.stages.finishing}
      <h4>{t('stages.finishing')}</h4>
      {@render rates(template.stages.finishing.cutting)}
    {/if}
  {:else}
    {#if differentFinish}<h4>{t('templates.preview.rough')}</h4>{/if}
    {#if template.tool.rough}{@render rates(template.tool.rough)}{/if}
    {#if differentFinish && finish.finish}<h4>{t('templates.preview.finish')}</h4>
      {@render rates(finish.finish)}{/if}
  {/if}
</details>

<style>
  .parameter-preview {
    margin: 8px 0;
    font-size: var(--inspector-font-field);
  }
  summary {
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px 0;
  }
  summary:focus-visible {
    outline: 2px solid var(--accent);
  }
  dl {
    margin: 4px 0;
  }
  dl div {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: 8px;
    padding: 2px 0;
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    font-variant-numeric: tabular-nums;
    overflow-wrap: anywhere;
  }
  h4 {
    margin: 8px 0 2px;
    font-size: inherit;
    font-weight: 600;
  }
</style>
