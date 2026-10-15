<script lang="ts">
  import { project } from '../../state/project.svelte';
  import type { OpField, OpFieldValue, VCarveOp } from '../../state/project.svelte';
  import { vcarveStrategy, vcarveStrategyPatch } from '../../state/vcarve-strategy-ui';
  import { t } from '../../i18n';

  interface Props {
    op: VCarveOp;
    /// Kind-aware patch — see ChamferSection for rationale.
    patch: <K extends OpField>(field: K, value: OpFieldValue<K>) => void;
  }
  let { op, patch }: Props = $props();
  const strategy = $derived(vcarveStrategy(op.fullMedialAxis));
</script>

<fieldset>
  <legend>{t('ops.vcarve.legend')}</legend>
  <!-- Tool-kind warning is now emitted by OpPropertiesPanel against
       the central op_tool_constraint helper — see k94n. -->

  <!--
    Carve mode is the most consequential V-Carve setting (it changes the
    toolpath structure entirely), so it sits at the top of the section
    rather than buried inside Advanced. Stored as the
    `fullMedialAxis: boolean | undefined` field — undefined means the
    default perimeter-only mode.
  -->
  <label class="strategy-choice">
    <span>{t('ops.vcarve.mode.label')}</span>
    <select
      value={strategy.value}
      onkeydown={(event) => event.stopPropagation()}
      aria-describedby={`vcarve-strategy-help-${op.id}`}
      onchange={(event) =>
        patch(
          'fullMedialAxis',
          vcarveStrategyPatch(event.currentTarget.value as 'perimeter' | 'medial').fullMedialAxis,
        )}
    >
      <option value="perimeter">{t('ops.vcarve.mode.perimeter')}</option>
      <option value="medial">{t('ops.vcarve.mode.medial_axis')}</option>
    </select>
  </label>
  <p class="hint mode-scope" id={`vcarve-strategy-help-${op.id}`}>
    {t(strategy.description)}
    {t(strategy.scope)}
  </p>
  <details class="subsection">
    <summary>{t('ops.vcarve.advanced.summary')}</summary>
    <label class="row" title={t('ops.vcarve.max_width.help')}>
      <span>{t('ops.vcarve.max_width.label')}</span>
      <div class="num-cell">
        <input
          type="number"
          step="0.1"
          min="0"
          placeholder={t('ops.vcarve.max_width.placeholder')}
          value={op.carveMaxWidthMm ?? ''}
          onchange={(e) => {
            const v = parseFloat((e.currentTarget as HTMLInputElement).value);
            patch('carveMaxWidthMm', isNaN(v) || v <= 0 ? undefined : v);
          }}
        />
        <span class="unit">mm</span>
      </div>
    </label>
    <label class="row" title={t('ops.vcarve.source_inset.help')}>
      <span>{t('ops.vcarve.source_inset.label')}</span>
      <div class="num-cell">
        <input
          type="number"
          step="0.05"
          min="0"
          placeholder="0"
          value={op.sourceInsetMm ?? ''}
          onchange={(e) => {
            const v = parseFloat((e.currentTarget as HTMLInputElement).value);
            patch('sourceInsetMm', isNaN(v) || v <= 0 ? undefined : v);
          }}
        />
        <span class="unit">mm</span>
      </div>
    </label>
    <div class="row" title={t('ops.vcarve.inlay.unavailable')}>
      <span>{t('ops.vcarve.inlay_plug.label')}</span>
      <button
        type="button"
        disabled
        onclick={() => {
          // Historical duplication behavior retained; UI stays unavailable until
          // the separate paired-inlay milestone qualifies it.
          const dup = project.duplicateOperation(op.id);
          if (dup)
            project.updateOperation(dup.id, { name: `${op.name} (plug)`, sourceInsetMm: 0.1 });
        }}>{t('ops.vcarve.duplicate_as_inlay_plug')}</button
      >
    </div>
    <p class="hint">{t('ops.vcarve.inlay.unavailable')}</p>
  </details>
</fieldset>

<style>
  .strategy-choice {
    display: grid;
    gap: var(--ui-space-1);
    font-size: 0.78rem;
  }
</style>
