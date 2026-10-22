<script lang="ts">
  import OperationToolPicker from '../OperationToolPicker.svelte';
  import { Settings2 } from '@lucide/svelte';
  import { project } from '../../state/project.svelte';
  import type { PlanarSurfaceOp } from '../../state/op_types';
  import { effectiveDiameterMm } from '../../state/tool_wear';
  import { surfaceBounds } from '../../state/planar-surface';
  import CuttingParameters from './CuttingParameters.svelte';
  import { computeFootprint } from '../../sim/footprint';
  import { t } from '../../i18n';
  let { op }: { op: PlanarSurfaceOp } = $props();
  const tool = $derived(project.data.tools.find((tool) => tool.id === op.toolId));
  const diameter = $derived(tool ? effectiveDiameterMm(tool) : 0);
  const bounds = $derived(surfaceBounds(op, diameter));
  const flatTools = $derived(
    project.data.tools.filter((tool) => ['endmill', 'compression'].includes(tool.kind)),
  );
  const invalid = $derived(
    !bounds ||
      !tool ||
      !flatTools.some((t) => t.id === tool.id) ||
      op.surface.stepover_mm > diameter * 0.95 ||
      op.surface.stepover_mm <= 0,
  );
  const dimensions = ['width_mm', 'height_mm', 'stepover_mm'] as const;
  const extras = ['margin_mm', 'overtravel_mm'] as const;
  function changeSurface(patch: Partial<PlanarSurfaceOp['surface']>) {
    project.updateOperation(op.id, { surface: { ...op.surface, ...patch } });
  }
  function origin(axis: number, value: number) {
    const xy = [...op.surface.origin];
    xy[axis] = value;
    changeSurface({ origin: xy });
  }
  function useStock() {
    const b = computeFootprint(
      project.stockSizingImport,
      project.data.stock,
      project.data.machine.workArea,
    );
    changeSurface({
      origin: [b.minX, b.minY],
      width_mm: b.maxX - b.minX,
      height_mm: b.maxY - b.minY,
    });
  }
  const extent = (b: number[]) =>
    `${b[0].toFixed(2)}, ${b[1].toFixed(2)} → ${b[2].toFixed(2)}, ${b[3].toFixed(2)} mm`;
</script>

<label class="row"
  ><span>{t('opprops.name')}</span><input
    value={op.name}
    oninput={(e) => project.updateOperation(op.id, { name: e.currentTarget.value })}
  /></label
>
<label class="row"
  ><span>{t('opprops.tool')}</span>
  <div class="tool-cell">
    <OperationToolPicker {op} />
    <button
      type="button"
      class="tool-edit"
      aria-label={t('opprops.tool.edit')}
      title={t('opprops.tool.edit')}
      onclick={() => (project.sel.toolsDialogFocusId = op.toolId)}><Settings2 size={14} /></button
    >
  </div></label
>
<section aria-label={t('surface.zone')}>
  <h3>{t('surface.zone')}</h3>
  {#each [0, 1] as axis (axis)}<label class="inspector-field"
      ><span>{axis === 0 ? t('surface.origin_x') : t('surface.origin_y')}</span><input
        type="number"
        step="0.1"
        value={op.surface.origin[axis]}
        oninput={(e) => origin(axis, Number(e.currentTarget.value))}
      /><span class="unit">mm</span></label
    >{/each}
  {#each dimensions as field (field)}<label class="inspector-field"
      ><span>{t(`surface.${field}`)}</span><input
        type="number"
        min="0.01"
        step="0.1"
        value={op.surface[field]}
        oninput={(e) => changeSurface({ [field]: Number(e.currentTarget.value) })}
      /><span class="unit">mm</span></label
    >{/each}
  <label class="row"
    ><span>{t('surface.direction')}</span><select
      value={op.surface.direction}
      onchange={(e) => changeSurface({ direction: e.currentTarget.value as 'along_x' | 'along_y' })}
      ><option value="along_x">X</option><option value="along_y">Y</option></select
    ></label
  >
  <button type="button" class="inspector-action" onclick={useStock}>{t('surface.use_stock')}</button
  >
</section>
<section aria-label={t('fs35.cutting')}>
  <h3>{t('fs35.cutting')}</h3>
  <label class="inspector-field"
    ><span>{t('surface.removal')}</span><input
      type="number"
      min="0.01"
      step="0.1"
      value={-op.depth}
      oninput={(e) => project.updateOperation(op.id, { depth: -Number(e.currentTarget.value) })}
    /><span class="unit">mm</span></label
  >
  <label class="inspector-field"
    ><span>{t('fs36.pass_depth')}</span><input
      type="number"
      min="0.01"
      step="0.1"
      value={op.step === null ? '' : -op.step}
      oninput={(e) => project.updateOperation(op.id, { step: -Number(e.currentTarget.value) })}
    /><span class="unit">mm</span></label
  >
</section>
{#key op.id}<CuttingParameters {op} />{/key}
{#if invalid}<p class="warn-chip" role="alert">
    {t('surface.invalid')}
  </p>{/if}
<p class="warn-chip">{t('surface.zero_edges')}</p>
<details>
  <summary>{t('surface.edges')}</summary>
  {#each extras as field (field)}<label class="inspector-field"
      ><span>{t(`surface.${field}`)}</span><input
        type="number"
        min="0"
        step="0.1"
        value={op.surface[field]}
        oninput={(e) => changeSurface({ [field]: Number(e.currentTarget.value) })}
      /><span class="unit">mm</span></label
    >{/each}
  <p class="hint">{t('surface.extra_help')}</p>
  {#if bounds}<p class="hint">{t('surface.center_extent')}: {extent(bounds.center)}</p>
    <p class="hint">{t('surface.cutter_extent')}: {extent(bounds.cutter)}</p>{/if}
  <p class="hint">{t('surface.safe_z', { z: project.data.machine.fastMoveZ })}</p>
</details>
