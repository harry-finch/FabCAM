<script lang="ts">
  import type { components } from '../api/generated';
  import CuttingParameterComparison from './CuttingParameterComparison.svelte';
  import ExperimentalBadge from './ExperimentalBadge.svelte';
  import TrialProvenance from './TrialProvenance.svelte';
  import { t } from '../i18n';
  let {
    preview,
    current,
  }: {
    preview: components['schemas']['TrialPreview'];
    current: (number | undefined)[];
  } = $props();
  const recipe = $derived(preview.operation.params.applied_cutting!.experimental_recipe!);
  const values = $derived(recipe.applied_values);
</script>

<section aria-label={t('trial.use')}>
  <ExperimentalBadge />
  <TrialProvenance source={recipe.source} />
  <CuttingParameterComparison
    pocket={preview.operation.kind.type === 'pocket'}
    {current}
    proposed={[
      values.rpm,
      values.feed_mm_min,
      values.plunge_mm_min,
      values.doc_mm,
      values.stepover_ratio == null ? undefined : values.stepover_ratio * 100,
    ]}
  />
  <details>
    <summary>{t('trial.context')}</summary>
    {#each preview.context_differences as difference (difference)}<p>{difference}</p>{/each}
  </details>
</section>

<style>
  section {
    margin-block: 0.75rem;
  }
  p {
    overflow-wrap: anywhere;
  }
</style>
