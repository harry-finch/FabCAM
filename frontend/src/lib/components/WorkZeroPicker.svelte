<script lang="ts">
  import { project } from '../state/project.svelte';
  import { computeFootprint } from '../sim/footprint';
  import { workZeroPoint } from '../state/preparation';
  import { t } from '../i18n';
  let target = $state('stock');
  const bbox = $derived(project.transformedImport?.bbox);
  const bounds = $derived(
    target === 'drawing'
      ? bbox
        ? { minX: bbox.min_x, minY: bbox.min_y, maxX: bbox.max_x, maxY: bbox.max_y }
        : null
      : computeFootprint(
          project.stockSizingImport,
          project.data.stock,
          project.data.machine.workArea,
        ),
  );
</script>

<div class="zero-picker">
  <label
    >{t('uxc.zero_target')}<select bind:value={target}
      ><option value="stock">{t('uxc.stock')}</option><option value="drawing"
        >{t('uxc.drawing')}</option
      ></select
    ></label
  >
  <div class="points" role="group" aria-label={t('uxc.zero_target')}>
    {#each [1, 0.5, 0] as y (y)}{#each [0, 0.5, 1] as x (x)}
        {@const point = bounds ? workZeroPoint(bounds, x, y) : null}
        <button
          type="button"
          disabled={!point}
          aria-pressed={!!point &&
            Math.abs(project.data.workOffset.x_mm - point.x_mm) < 1e-8 &&
            Math.abs(project.data.workOffset.y_mm - point.y_mm) < 1e-8}
          title={point
            ? `X ${point.x_mm.toFixed(2)} · Y ${point.y_mm.toFixed(2)} mm`
            : t('uxc.no_drawing')}
          aria-label={point
            ? `X ${point.x_mm.toFixed(2)} · Y ${point.y_mm.toFixed(2)} mm`
            : t('uxc.no_drawing')}
          onclick={() => {
            if (point) project.setWorkOffset(point);
          }}>●</button
        >
      {/each}{/each}
  </div>
  <p>{t('uxc.xy_only')} · Z {project.data.workOffset.z_mm.toFixed(2)} mm</p>
</div>

<style>
  .zero-picker {
    grid-column: 1 / -1;
    min-width: 0;
    margin: var(--ui-space-3) 0;
    padding: var(--ui-space-2);
    border: 1px solid var(--border);
    border-radius: 4px;
  }
  label {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
    align-items: center;
    font-size: 0.8rem;
  }
  .points {
    display: grid;
    grid-template-columns: repeat(3, 30px);
    gap: 4px;
    margin-top: var(--ui-space-2);
  }
  button {
    min-height: 28px;
    color: var(--accent);
  }
  button[aria-pressed='true'] {
    background: var(--accent);
    color: var(--bg-app);
  }
  button:focus-visible {
    outline: 2px solid var(--accent);
  }
  p {
    color: var(--text-muted);
    font-size: 0.75rem;
    margin-bottom: 0;
  }
</style>
