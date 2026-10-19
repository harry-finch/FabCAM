<script lang="ts">
  import { project, type OpEntry } from '../../state/project.svelte';
  import { t } from '../../i18n';
  let { op }: { op: OpEntry } = $props();
  const tool = $derived(project.data.tools.find((tool) => tool.id === op.toolId));
  const drill = $derived(op.kind === 'drill');
  const rpm = $derived(tool ? (drill ? (tool.speedDrill ?? tool.speed) : tool.speed) : null);
  const feed = $derived(
    op.feedRateOverride ?? (drill ? (tool?.feedRateDrill ?? tool?.feedRate) : tool?.feedRate),
  );
  const plunge = $derived(
    op.plungeRateOverride ??
      (drill ? (tool?.plungeRateDrill ?? tool?.plungeRate) : tool?.plungeRate),
  );
  function rate(field: 'feedRateOverride' | 'plungeRateOverride', value: string) {
    if (value === '') project.updateOperation(op.id, { [field]: undefined });
    else {
      const n = Number(value);
      if (Number.isInteger(n) && n > 0) project.updateOperation(op.id, { [field]: n });
    }
  }
</script>

<fieldset>
  <legend>{t('uxa.saved_rates')}</legend>
  {#if op.appliedCutting}<p class="warn-chip">{t('uxa.unsupported_applied')}</p>{:else}
    <div class="row"><span>{t('fs.rpm')}</span><span>{rpm ?? '—'} rpm</span></div>
    <label class="row"
      ><span>{t('fs.feed')}</span>
      <div class="num-cell">
        <input
          type="number"
          min="1"
          step="1"
          value={op.feedRateOverride ?? ''}
          placeholder={String(feed ?? '—')}
          onchange={(e) => rate('feedRateOverride', e.currentTarget.value)}
        /><span class="unit">mm/min</span>
      </div></label
    >
    <label class="row"
      ><span>{t('fs.plunge')}</span>
      <div class="num-cell">
        <input
          type="number"
          min="1"
          step="1"
          value={op.plungeRateOverride ?? ''}
          placeholder={String(plunge ?? '—')}
          onchange={(e) => rate('plungeRateOverride', e.currentTarget.value)}
        /><span class="unit">mm/min</span>
      </div></label
    >
    <p class="hint">{t('uxa.rates_help')}</p>
  {/if}
  <button
    type="button"
    class="reset-link"
    onclick={() => (project.sel.toolsDialogFocusId = op.toolId)}>{t('opprops.tool.edit')}</button
  >
</fieldset>
