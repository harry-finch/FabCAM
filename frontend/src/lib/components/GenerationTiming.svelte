<script lang="ts">
  import { elapsedSeconds, type GenerationRun } from '../state/generation-run';
  import { t } from '../i18n';
  import { Clock3 } from '@lucide/svelte';
  let { run, stale = false }: { run: GenerationRun; stale?: boolean } = $props();
  let now = $state(performance.now());
  $effect(() => {
    if (run.outcome !== 'running') return;
    now = performance.now();
    const timer = setInterval(() => {
      now = performance.now();
    }, 250);
    return () => clearInterval(timer);
  });
  const label = $derived(
    run.outcome === 'running'
      ? t('ux7.running')
      : stale || run.outcome === 'obsolete'
        ? t('ux7.obsolete')
        : run.outcome === 'success'
          ? t('ux7.success')
          : run.outcome === 'cancelled'
            ? t('ux7.cancelled')
            : run.outcome === 'blocked'
              ? t('ux7.blocked')
              : t('ux7.failed'),
  );
</script>

<span
  class="generation-timing"
  class:failed={run.outcome === 'failed' || run.outcome === 'blocked'}
  title={t('ux7.help')}
>
  <Clock3 size={14} aria-hidden="true" />
  <span>{label} <span class="seconds">{elapsedSeconds(run, now)} s</span></span>
</span>

<style>
  .generation-timing {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-1);
    font-size: 0.78rem;
    color: var(--text-muted);
  }
  .seconds {
    font-variant-numeric: tabular-nums;
    display: inline-block;
    min-width: 4.5ch;
  }
  .failed {
    color: var(--warn);
  }
</style>
