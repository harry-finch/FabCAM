<script lang="ts">
  import ProgramPostNotice from './ProgramPostNotice.svelte';
  import { project } from '../../state/project.svelte';
  import { workspace } from '../../state/workspace.svelte';
  const cps = $derived(
    (project.data.machine.gcodeDialect ?? workspace.get().last_post_processor) === 'cps',
  );
  /// Probe op-properties: G38.2 probe move along the selected axis.
  /// No tool / source needed — the operator wires the probe to the machine.
  /// Styles inherited from OpPropertiesPanel's :global(.props ...) rules.
  import type { OpField, OpFieldValue, ProbeOp } from '../../state/project.svelte';
  import { t } from '../../i18n';

  interface Props {
    op: ProbeOp;
    patch: <K extends OpField>(field: K, value: OpFieldValue<K>) => void;
  }
  let { op, patch }: Props = $props();
</script>

<ProgramPostNotice kind="probe" />

<label class="row">
  <span>{t('ops.probe.name.label')}</span>
  <input
    type="text"
    value={op.name}
    oninput={(e) => patch('name', (e.currentTarget as HTMLInputElement).value)}
  />
</label>
<label class="row" title={t('ops.probe.axis.help')}>
  <span>{t('ops.probe.axis.label')}</span>
  <select
    value={op.axis ?? 'z'}
    onchange={(e) => patch('axis', (e.currentTarget as HTMLSelectElement).value as 'x' | 'y' | 'z')}
  >
    <option value="x">X</option>
    <option value="y">Y</option>
    <option value="z">Z</option>
  </select>
</label>
<label class="row" title={t('ops.probe.distance.help')}>
  <span>{t('ops.probe.distance.label')}</span>
  <input
    type="number"
    step="0.1"
    value={op.distanceMm ?? -10}
    oninput={(e) =>
      patch('distanceMm', Number.parseFloat((e.currentTarget as HTMLInputElement).value))}
  />
</label>
<label class="row" title={t('ops.probe.feed.help')}>
  <span>{t('ops.probe.feed.label')}</span>
  <input
    type="number"
    step="10"
    min="1"
    value={op.feedMmMin ?? 100}
    oninput={(e) =>
      patch('feedMmMin', Number.parseInt((e.currentTarget as HTMLInputElement).value, 10))}
  />
</label>
{#if !cps}<p class="hint hint-pause">
    {t('ops.probe.emits.hint')}
    <code>G38.2 {(op.axis ?? 'z').toUpperCase()}{op.distanceMm ?? -10} F{op.feedMmMin ?? 100}</code
    >{t('ops.probe.halts.hint')}
  </p>{/if}
