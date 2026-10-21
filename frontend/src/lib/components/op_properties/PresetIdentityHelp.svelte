<script lang="ts">
  import { t } from '../../i18n';
  let { missing, toolName }: { missing: ('machine' | 'material' | 'tool')[]; toolName: string } =
    $props();
</script>

{#if missing.length}<div role="alert" class="identity-help">
    {#if missing.length === 1}
      <p>{t(`v11.missing_${missing[0]}`, { name: toolName })}</p>
    {:else}
      <p>{t('v11.preset_missing')}</p>
      <ul>
        {#each missing as item (item)}<li>
            {t(`v11.missing_${item}`, { name: toolName })}
          </li>{/each}
      </ul>
    {/if}
  </div>{/if}

<style>
  .identity-help {
    grid-column: 1;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
    line-height: 1.4;
    margin-top: var(--ui-space-2);
  }
  .identity-help p {
    margin: 0;
  }
  .identity-help ul {
    margin: var(--ui-space-1) 0 0;
    padding-inline-start: 1.25rem;
    font-size: inherit;
  }
</style>
