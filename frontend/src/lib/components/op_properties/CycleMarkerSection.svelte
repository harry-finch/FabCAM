<script lang="ts">
  import ProgramPostNotice from './ProgramPostNotice.svelte';
  import { project } from '../../state/project.svelte';
  import { workspace } from '../../state/workspace.svelte';
  const cps = $derived(
    (project.data.machine.gcodeDialect ?? workspace.get().last_post_processor) === 'cps',
  );
  /// CycleMarker op-properties: comment-only marker. Pendants and
  /// gcode viewers that index by program line can jump to the next marker.
  /// Styles inherited from OpPropertiesPanel's :global(.props ...) rules.
  import type { CycleMarkerOp, OpField, OpFieldValue } from '../../state/project.svelte';
  import { t } from '../../i18n';

  interface Props {
    op: CycleMarkerOp;
    patch: <K extends OpField>(field: K, value: OpFieldValue<K>) => void;
  }
  let { op, patch }: Props = $props();
</script>

<ProgramPostNotice kind="cycle_marker" />

<label class="row">
  <span>{t('ops.cycle_marker.name.label')}</span>
  <input
    type="text"
    value={op.name}
    oninput={(e) => patch('name', (e.currentTarget as HTMLInputElement).value)}
  />
</label>
<label class="row" title={t('ops.cycle_marker.label.help')}>
  <span>{t('ops.cycle_marker.label.label')}</span>
  <input
    type="text"
    value={op.label ?? ''}
    placeholder={t('ops.cycle_marker.label.placeholder')}
    oninput={(e) => patch('label', (e.currentTarget as HTMLInputElement).value)}
  />
</label>
{#if !cps}<p class="hint hint-pause">
    {t('ops.cycle_marker.emits.hint')} <code>; --- {op.label ?? ''} ---</code>{t(
      'ops.cycle_marker.no_motion.hint',
    )}
  </p>{/if}
