<script lang="ts">
  import type { components } from '../api/generated';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { trialPresentation } from '../state/trial-presentation';
  import { t } from '../i18n';
  let { source }: { source: components['schemas']['FsSelectedRecord'] } = $props();
  const view = $derived(
    trialPresentation(source, [...recommendationSources.builtins, ...recommendationSources.local]),
  );
</script>

<div class="provenance">
  <p>{view?.name ?? t('trial.unknown_name')}</p>
  {#if view?.original}
    <p class="secondary">
      {t('trial.derived')}: {view.original.publisher}{view.original.material
        ? ` · ${view.original.material}`
        : ''} · {t(`fs.${view.original.mode}`)}
    </p>
  {:else if view}<p class="secondary">{t('fs.source')}: {view.publisher}</p>{/if}
  <details>
    <summary>{t('trial.technical')}</summary>
    <p>{t('trial.status')}</p>
    <p>
      {source.pack.id} @{source.pack.revision}<br />{source.record_id}<br />{source.profile.id} @{source
        .profile.revision}
    </p>
    {#if view?.original}<p>
        {t('trial.derived')}: {view.original.pack.id} @{view.original.pack.revision}<br />{view
          .original.profile.id} @{view.original.profile.revision}
      </p>{/if}
    {#if !view && !recommendationSources.loading}<p>{t('trial.source_missing')}</p>{/if}
    <p class="secondary">{t('trial.source_evidence')}</p>
  </details>
</div>

<style>
  .provenance {
    margin-block: var(--ui-space-2);
  }
  p {
    margin: var(--ui-space-1) 0;
    overflow-wrap: anywhere;
  }
  .secondary {
    color: var(--text-muted);
  }
</style>
