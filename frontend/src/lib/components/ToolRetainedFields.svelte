<script lang="ts">
  import { t } from '../i18n';
  import type { ToolEntry } from '../state/project.svelte';
  import type { MsgKey } from '../i18n/keys';
  import { attrApplies, type ToolAttr } from '../state/tool_family';
  let { tool }: { tool: ToolEntry } = $props();
  const fields: { key: keyof ToolEntry; attr: ToolAttr; label: MsgKey; unit: string }[] = [
    { key: 'tipDiameter', attr: 'tipDiameter', label: 'tools.col.tip_diameter', unit: 'mm' },
    { key: 'tipAngleDeg', attr: 'tipAngleDeg', label: 'tools.col.tip_angle', unit: '°' },
    { key: 'flutes', attr: 'flutes', label: 'tools.col.flutes', unit: '' },
    { key: 'speed', attr: 'speed', label: 'tools.col.speed', unit: 'RPM' },
    { key: 'plungeRate', attr: 'plunge', label: 'tools.col.plunge', unit: 'mm/min' },
    { key: 'defaultStep', attr: 'defaultStep', label: 'tools.col.dflt_step', unit: 'mm' },
    { key: 'dragoff', attr: 'dragoff', label: 'tools.drag.offset', unit: 'mm' },
    {
      key: 'compressionTransitionMm',
      attr: 'compressionTransition',
      label: 'tools.compression.transition',
      unit: 'mm',
    },
    { key: 'threadPitchMm', attr: 'threadPitch', label: 'tools.thread.pitch', unit: 'mm' },
    {
      key: 'cornerRadiusMm',
      attr: 'cornerRadius',
      label: 'tools.bullnose.corner_radius',
      unit: 'mm',
    },
    { key: 'wearOffsetMm', attr: 'wear', label: 'tools.wear.offset', unit: 'mm' },
    { key: 'laserPierceSec', attr: 'laser', label: 'tools.laser.pierce_time', unit: 's' },
    { key: 'laserLeadInMm', attr: 'laser', label: 'tools.laser.lead_in', unit: 'mm' },
    { key: 'pierceHeightMm', attr: 'plasma', label: 'tools.plasma.pierce_height', unit: 'mm' },
    { key: 'cutHeightMm', attr: 'plasma', label: 'tools.plasma.cut_height', unit: 'mm' },
    { key: 'pierceDelaySec', attr: 'plasma', label: 'tools.plasma.pierce_delay', unit: 's' },
    {
      key: 'dragKnifeSelfAlignAngleDeg',
      attr: 'dragoff',
      label: 'tools.drag.self_align',
      unit: '°',
    },
    { key: 'kerfMm', attr: 'laser', label: 'tools.laser.kerf', unit: 'mm' },
    { key: 'fluteLengthMm', attr: 'flutes', label: 'tools.holder.flute_length', unit: 'mm' },
  ];
  const retained = $derived(
    fields.filter(
      (f) =>
        tool[f.key] !== undefined &&
        !attrApplies(f.attr, tool.kind) &&
        !(f.key === 'kerfMm' && attrApplies('plasma', tool.kind)),
    ),
  );
</script>

<details>
  <summary>{t('ux6b.retained')}</summary>
  <p class="hint">{t('ux6b.retained_help')}</p>
  <dl>
    {#each retained as field (field.key)}<dt>{t(field.label)}</dt>
      <dd>{String(tool[field.key])} {field.unit}</dd>{/each}
  </dl>
  {#if tool.formProfileMm?.length && !attrApplies('formProfile', tool.kind)}<p>
      {t('ux6b.retained_profile', { count: tool.formProfileMm.length })}
    </p>{/if}
</details>

<style>
  details {
    margin: 16px 0;
    font-size: 0.8rem;
  }
  .hint {
    color: var(--text-muted);
    line-height: 1.45;
  }
  dl {
    display: grid;
    grid-template-columns: 180px 1fr;
    gap: 6px;
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
  }
</style>
