<script lang="ts">
  import ProgramPostNotice from './ProgramPostNotice.svelte';
  import { project } from '../../state/project.svelte';
  import { workspace } from '../../state/workspace.svelte';
  const cps = $derived(
    (project.data.machine.gcodeDialect ?? workspace.get().last_post_processor) === 'cps',
  );
  /// Pause op-properties: name + operator message. No tool, no
  /// source, no geometry — the op is a program-flow stop.
  /// Styles inherited from OpPropertiesPanel's :global(.props ...) rules.
  import type { OpField, OpFieldValue, PauseOp } from '../../state/project.svelte';
  import { t } from '../../i18n';

  interface Props {
    op: PauseOp;
    patch: <K extends OpField>(field: K, value: OpFieldValue<K>) => void;
  }
  let { op, patch }: Props = $props();
</script>

<ProgramPostNotice kind="pause" />

<label class="row">
  <span>{t('ops.pause.name.label')}</span>
  <input
    type="text"
    value={op.name}
    oninput={(e) => patch('name', (e.currentTarget as HTMLInputElement).value)}
  />
</label>
<label class="row" title={cps ? t('uxa.pause_cps') : t('ops.pause.message.help')}>
  <span>{t('ops.pause.message.label')}</span>
  <input
    type="text"
    value={op.message ?? ''}
    placeholder={t('ops.pause.message.placeholder')}
    oninput={(e) => patch('message', (e.currentTarget as HTMLInputElement).value)}
  />
</label>
{#if !cps}<p class="hint hint-pause">
    {t('ops.pause.emits.hint')} <code>M5</code>{t('ops.pause.emits_message.hint')}
    <code>M0</code>{t('ops.pause.emits_and.hint')}
    <code>M3</code>{t('ops.pause.emits_slot.hint')}
  </p>{/if}
