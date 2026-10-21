<script lang="ts">
  import { FlaskConical } from '@lucide/svelte';
  import { t } from '../i18n';
  let { modified = false }: { modified?: boolean } = $props();
</script>

<span class="experimental" role="status"
  ><FlaskConical size={14} aria-hidden="true" /><span
    >{t('trial.status')}{modified ? ` · ${t('trial.modified')}` : ''}</span
  ></span
>

<style>
  .experimental {
    display: inline-flex;
    align-items: baseline;
    gap: var(--ui-space-1);
    color: var(--warning, #d8a540);
    font-weight: 500;
  }
  .experimental :global(svg) {
    flex: none;
    align-self: center;
  }
</style>
