<script lang="ts">
  import { t } from '../i18n';
  import { isPresetContextMismatch } from '../state/preset-context-review';
  let { error }: { error: string } = $props();
</script>

{#if error}
  <div class="preset-error" role="alert">
    {#if isPresetContextMismatch(error)}
      <strong>{t('preset.review.title')}</strong>
      <p>{t('preset.review.mismatch')}</p>
    {:else}
      <p>{error}</p>
    {/if}
  </div>
{/if}

<style>
  .preset-error {
    color: var(--error);
    font-size: var(--inspector-font-meta, 12px);
    overflow-wrap: anywhere;
  }
  p {
    margin: 4px 0;
    line-height: 1.4;
  }
</style>
