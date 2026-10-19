<script lang="ts">
  import type { ToolEntry } from '../state/project-types';
  import { dimensionState, toolDimensions } from '../state/tool-dimensions';
  import { t } from '../i18n';
  let { tool }: { tool: ToolEntry } = $props();
  const d = $derived(toolDimensions(tool));
  function value(v: number | null | undefined, allowZero = false): string {
    const state = dimensionState(v, allowZero);
    return state === 'known' ? `${v} mm` : t(state === 'unknown' ? 'uxb.unknown' : 'uxb.invalid');
  }
</script>

<figure aria-label={t('uxb.diagram')}>
  <figcaption>{t('uxb.diagram')}</figcaption>
  <svg viewBox="0 0 300 250" role="img" aria-label={t('uxb.schematic')}>
    <line x1="140" y1="15" x2="140" y2="225" class="axis" />
    <rect
      x={140 - d.shank / 2}
      y={210 - d.overall}
      width={d.shank}
      height={d.overall}
      class="shank"
      class:unknown={dimensionState(tool.lengthMm) !== 'known' ||
        dimensionState(tool.shankDiameterMm) !== 'known'}
    />
    <rect
      x={140 - d.diameter / 2}
      y={210 - d.flute}
      width={d.diameter}
      height={d.flute}
      class="cutter"
      class:unknown={dimensionState(tool.fluteLengthMm) !== 'known' ||
        dimensionState(tool.diameter) !== 'known'}
    />
    {#if tool.holder}
      <path d={d.holderPath} class="holder" class:unknown={!d.holderValid} />
    {:else}
      <line x1="95" y1={210 - d.holderBase} x2="185" y2={210 - d.holderBase} class="unknown" />
    {/if}
    <path
      d={`M65 ${210 - d.overall} h12 M71 ${210 - d.overall} V210 M65 210 h12`}
      class="dimension"
    /><text x="58" y="130">L</text>
    <path
      d={`M205 ${210 - d.holderBase} h12 M211 ${210 - d.holderBase} V${210 - d.flute} M205 ${210 - d.flute} h12`}
      class="dimension"
    /><text x="220" y="155">S</text>
    <path
      d={`M175 ${210 - d.flute} h12 M181 ${210 - d.flute} V210 M175 210 h12`}
      class="dimension"
    /><text x="190" y="200">F</text>
    <line
      x1={140 - d.diameter / 2}
      y1="230"
      x2={140 + d.diameter / 2}
      y2="230"
      class="dimension"
    /><text x="140" y="247" text-anchor="middle">D</text>
  </svg>
  <dl>
    <dt>D · {t('ux6b.diameter')}</dt>
    <dd>{value(tool.diameter)}</dd>
    <dt>F · {t('tools.holder.flute_length')}</dt>
    <dd>{value(tool.fluteLengthMm)}</dd>
    <dt>S · {t('uxb.free_shank')}</dt>
    <dd>{value(tool.stickoutLengthMm, true)}</dd>
    <dt>L · {t('tools.holder.overall_length')}</dt>
    <dd>{value(tool.lengthMm)}</dd>
    <dt>{t('tools.holder.shank_diameter')}</dt>
    <dd>{value(tool.shankDiameterMm)}</dd>
    <dt>{t('uxb.holder_size')}</dt>
    <dd>
      {tool.holder
        ? `${value(d.holderDiameter)} / ${value(d.holderLength)}`
        : t('uxb.holder_unknown')}
    </dd>
  </dl>
  {#if d.inconsistent}<p class="warning" role="status">{t('uxb.inconsistent')}</p>{/if}
  <p>{t('uxb.schematic')}</p>
</figure>

<style>
  figure {
    margin: 0;
    padding: 12px;
    border-left: 1px solid var(--border);
    max-width: 340px;
    font-size: 0.75rem;
  }
  figcaption {
    font-weight: 600;
  }
  svg {
    display: block;
    width: 100%;
    max-height: 220px;
  }
  svg text {
    fill: var(--text);
    font-size: 12px;
  }
  .axis {
    stroke: var(--border);
    stroke-dasharray: 3 4;
  }
  .shank,
  .holder {
    fill: var(--bg-elevated);
    stroke: var(--text-muted);
  }
  .cutter {
    fill: color-mix(in srgb, var(--accent) 25%, var(--bg-panel));
    stroke: var(--accent);
  }
  .dimension {
    stroke: var(--text-muted);
    fill: none;
  }
  .unknown {
    stroke: var(--text-muted);
    stroke-dasharray: 4 3;
    fill: none;
  }
  dl {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 6px 12px;
  }
  dt,
  p {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    text-align: right;
  }
  .warning {
    color: var(--warning, #d8a540);
  }
</style>
