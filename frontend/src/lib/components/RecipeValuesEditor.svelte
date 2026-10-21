<script lang="ts">
  import { t } from '../i18n';
  import { recipeInputs, type RecipeData, type RecipeInputs } from '../state/recipe-values';
  let { source, inputs = $bindable() }: { source: RecipeData; inputs: RecipeInputs } = $props();
  const original = $derived(recipeInputs(source));
  const fields = $derived([
    { key: 'rpm', label: 'fs.rpm', unit: 'rpm' },
    {
      key: 'rate',
      label: source.kind === 'exact_values' ? 'fs.feed' : 'fs.chipload',
      unit: source.kind === 'exact_values' ? 'mm/min' : 'mm/tooth',
    },
    { key: 'plunge', label: 'fs.plunge', unit: 'mm/min' },
    { key: 'depth', label: 'fs36.pass_depth', unit: 'mm' },
    ...(source.stepover_ratio == null
      ? []
      : [{ key: 'stepover', label: 'fs.stepover', unit: '%' }]),
  ] as const);
</script>

<div class="recipe-values">
  <div class="heading">
    <span>{t('preset3.parameter')}</span><span>{t('preset2.before')}</span><span
      >{t('preset2.recipe')}</span
    ><span></span>
  </div>
  {#each fields as field (field.key)}
    {@const key = field.key as keyof RecipeInputs}
    <label class="recipe-field">
      <span>{t(field.label as import('../i18n').MsgKey)}</span>
      <span class="original">{original[key] ?? '—'}</span>
      <input
        type="number"
        bind:value={inputs[key]}
        min="0"
        max={key === 'stepover' ? 100 : undefined}
        step="any"
        required
        aria-label={`${t(field.label as import('../i18n').MsgKey)} (${field.unit})`}
        aria-invalid={inputs[key] == null ||
          !Number.isFinite(inputs[key]) ||
          inputs[key]! <= 0 ||
          (key === 'stepover' && inputs[key]! > 100)}
      />
      <span class="unit">{field.unit}</span>
    </label>
  {/each}
</div>

<style>
  .recipe-values {
    font-size: var(--inspector-font-field);
    min-width: 0;
  }
  .recipe-field,
  .heading {
    display: grid;
    grid-template-columns: minmax(5rem, 1fr) minmax(3rem, 0.7fr) minmax(4rem, 0.8fr) 4rem;
    gap: var(--ui-space-2);
    align-items: center;
  }
  .heading {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
    padding-bottom: var(--ui-space-1);
  }
  .recipe-field {
    padding: var(--ui-space-1) 0;
    border-bottom: 1px solid var(--border);
  }
  .original {
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    text-align: right;
  }
  .recipe-field input {
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
    min-height: var(--inspector-control-height);
    font: inherit;
    color: var(--text);
    background: var(--bg-input);
    padding: var(--ui-space-1);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    text-align: right;
  }
  .recipe-field input[aria-invalid='true'] {
    border-color: var(--danger);
  }
  .unit {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  @media (max-width: 480px) {
    .recipe-field,
    .heading {
      grid-template-columns: minmax(4.5rem, 1fr) 3rem minmax(3.5rem, 1fr) 3rem;
      gap: var(--ui-space-1);
    }
  }
</style>
