<script lang="ts">
  import type { VCarveOp, OpField, OpFieldValue } from '../../state/project.svelte';
  import { incompatibleVCarveSettings } from '../../state/vcarve-controls';
  import { t } from '../../i18n';
  let {
    op,
    patch,
  }: {
    op: VCarveOp;
    patch: <K extends OpField>(field: K, value: OpFieldValue<K>) => void;
  } = $props();
  let incompatible = $derived(incompatibleVCarveSettings(op));
</script>

{#if incompatible.length}
  <fieldset aria-label={t('ops.vcarve.incompatible.title')}>
    <legend>{t('ops.vcarve.incompatible.title')}</legend>
    <p class="incompatibility-help" role="alert">{t('ops.vcarve.incompatible.help')}</p>
    {#each incompatible as setting (setting.field)}
      <div class="row">
        <span>{t(setting.label)}</span>
        <div class="num-cell">
          <output
            >{setting.field === 'multiPassRefine'
              ? t('ops.vcarve.incompatible.enabled')
              : `${setting.value} mm`}</output
          >
          <button
            type="button"
            class="reset-link"
            aria-label={t('ops.vcarve.incompatible.clear', { setting: t(setting.label) })}
            onclick={() => patch(setting.field, setting.reset)}
            >{t('ops.vcarve.incompatible.reset')}</button
          >
        </div>
      </div>
    {/each}
  </fieldset>
{/if}

<style>
  .incompatibility-help {
    color: var(--error);
    font-size: 0.74rem;
    white-space: normal;
    margin: 0.25rem 0;
  }
  output {
    overflow-wrap: anywhere;
  }
</style>
