<script lang="ts">
  import { t } from '../../i18n';
  let { origin }: { origin?: string | null } = $props();
</script>

<small
  class="field-status"
  class:origin-badge={origin === 'manual' || origin === 'recommendation'}
  class:manual={origin === 'manual'}
  class:suggested={origin === 'recommendation'}
  title={origin === 'manual'
    ? t('fs36.modified_hint')
    : origin === 'recommendation'
      ? t('fs36.suggested_hint')
      : undefined}
>
  {#if origin === 'manual' || origin === 'recommendation'}
    <svg
      aria-hidden="true"
      width="10"
      height="10"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.5"
    >
      {#if origin === 'manual'}<path
          d="M3 11 10.5 3.5 12.5 5.5 5 13H3v-2Z M9.5 4.5l2 2"
        />{:else}<path d="m8 1 1.7 4.8L14.5 8l-4.8 1.7L8 15l-1.7-5.3L1.5 8l4.8-2.2L8 1Z" />{/if}
    </svg>
    <span>{origin === 'manual' ? t('fs35.modified') : t('fs36.suggested')}</span>
  {/if}
</small>

<style>
  .origin-badge {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    justify-self: start;
    padding: 2px 4px;
    border-radius: var(--radius-sm);
    line-height: 1.2;
    color: var(--text-muted);
  }
  .origin-badge.suggested {
    color: var(--status-suggested);
    background: color-mix(in srgb, var(--status-suggested) 10%, transparent);
  }
  .origin-badge.manual {
    color: var(--status-modified);
    background: color-mix(in srgb, var(--status-modified) 10%, transparent);
  }
  svg {
    flex-shrink: 0;
  }
</style>
