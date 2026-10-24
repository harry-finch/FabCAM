<script lang="ts">
  import { t } from '../i18n';
  import { recordedPresetContext } from '../state/preset-context-review';
  import type { TestedPreset } from '../state/preset-collections';
  let { preset }: { preset: TestedPreset } = $props();
  const review = $derived(recordedPresetContext(preset));
</script>

<details class="recorded-context">
  <summary>{t('preset.review.context')}</summary>
  <p>{t('preset.review.recorded')}</p>
  {#if review.available}
    <dl>
      {#each review.fields as field, index (index)}
        <div>
          <dt>{field.path}</dt>
          <dd>{field.value}</dd>
        </div>
      {/each}
    </dl>
  {:else}
    <p role="status">{t('preset.review.unavailable')}</p>
  {/if}
  <p>{t('preset.review.manual')}</p>
</details>

<style>
  .recorded-context {
    font-size: var(--inspector-font-meta, 12px);
    margin: 8px 0;
    overflow-wrap: anywhere;
  }
  summary {
    cursor: pointer;
    min-height: 24px;
    font-weight: 600;
  }
  p {
    color: var(--text-muted);
    line-height: 1.4;
  }
  dl {
    margin: 8px 0;
    max-height: 360px;
    overflow-y: auto;
  }
  dl > div {
    display: grid;
    grid-template-columns: minmax(0, 3fr) minmax(0, 2fr);
    gap: 8px;
    padding: 5px 0;
    border-bottom: 1px solid var(--border);
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    font-variant-numeric: tabular-nums;
    white-space: pre-wrap;
  }
</style>
