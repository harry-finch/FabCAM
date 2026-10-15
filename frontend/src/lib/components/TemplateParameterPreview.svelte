<script lang="ts">
  import type { OperationTemplate, TemplateTool } from '../state/operation-templates';
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

{#snippet rates(tool: TemplateTool, pass: 'rough' | 'finish')}
  <dl>
    <div>
      <dt>{t('surface.rpm')}</dt>
      <dd>{tool[pass].rpm} {units.rpm}</dd>
    </div>
    <div>
      <dt>{t('surface.feed_mm_min')}</dt>
      <dd>{tool[pass].feed_mm_min} {units.feed}</dd>
    </div>
    <div>
      <dt>{t('surface.plunge_mm_min')}</dt>
      <dd>{tool[pass].plunge_mm_min} {units.feed}</dd>
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
  {#if differentFinish}<h4>{t('templates.preview.rough')}</h4>{/if}
  {@render rates(template.tool, 'rough')}
  {#if differentFinish}<h4>{t('templates.preview.finish')}</h4>
    {@render rates(finish, 'finish')}{/if}
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
