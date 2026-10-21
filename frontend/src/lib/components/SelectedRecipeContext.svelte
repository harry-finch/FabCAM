<script lang="ts">
  import type { components } from '../api/generated';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { trialPresentation } from '../state/trial-presentation';
  import ExperimentalBadge from './ExperimentalBadge.svelte';
  import { t } from '../i18n';
  let {
    source,
    operation,
    trial,
    blocked,
    error,
  }: {
    source: components['schemas']['FsSelectedRecord'];
    operation: string;
    trial: boolean;
    blocked: boolean;
    error: string;
  } = $props();
  const view = $derived(
    trialPresentation(source, [...recommendationSources.builtins, ...recommendationSources.local]),
  );
</script>

<div class="selected-context" aria-label={t('selector.selected_preset')}>
  <div class="identity">
    <strong title={view?.name}>{view?.name ?? t('trial.unknown_name')}</strong><span
      title={operation}>→ {operation}</span
    >
  </div>
  {#if trial}<ExperimentalBadge />{/if}
  <div class="availability" role="status">
    {error || (blocked ? t('trial.review_required') : t('trial.ready'))}
  </div>
</div>

<style>
  .selected-context {
    margin-top: var(--ui-space-2);
    font-size: var(--inspector-font-field);
  }
  .identity {
    display: flex;
    gap: var(--ui-space-2);
    min-width: 0;
  }
  .identity > * {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .identity strong {
    flex: 1;
  }
  .identity span {
    flex: 1;
    color: var(--text-muted);
  }
  .availability {
    color: var(--text-muted);
    overflow-wrap: anywhere;
    max-height: 3.6em;
    overflow: auto;
  }
</style>
