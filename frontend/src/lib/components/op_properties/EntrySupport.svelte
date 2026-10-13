<script lang="ts">
  import { project, type OpEntry } from '../../state/project.svelte';
  import { workspace } from '../../state/workspace.svelte';
  import { shopbotEntrySupport } from '../../state/entry-support';
  import { generationContextKey } from '../../state/generation-context';
  import { t } from '../../i18n';
  let {
    op,
    onlyBlocked = false,
    hideBlocked = false,
  }: { op: OpEntry; onlyBlocked?: boolean; hideBlocked?: boolean } = $props();
  const support = $derived(
    shopbotEntrySupport(
      op,
      project.data.machine,
      workspace.get().last_post_processor,
      project.data.workOffset,
    ),
  );
  const resolution = $derived(
    project.gen.inputContextKey ===
      generationContextKey(
        project,
        project.data.machine.gcodeDialect ?? workspace.get().last_post_processor,
      )
      ? project.gen.generated?.warnings?.find(
          (w) => w.kind === 'entry_representation' && w.op_id === op.id,
        )
      : undefined,
  );
  const blocked = $derived(
    support === 'origin' || support === 'helix' || support === 'tabs' || support === 'leads',
  );
</script>

{#if support && support !== 'direct' && (!onlyBlocked || blocked) && (!hideBlocked || !blocked)}
  <p class="hint" class:warn={blocked} role={blocked ? 'alert' : 'status'}>
    {t(
      support === 'origin'
        ? 'ux1.entry.origin'
        : support === 'helix'
          ? 'ux1.entry.helix'
          : support === 'tabs'
            ? 'ux1.entry.tabs'
            : support === 'leads'
              ? 'ux1.entry.leads'
              : 'ux1.entry.ramp',
    )}
  </p>
{/if}

{#if !onlyBlocked && support === 'ramp' && resolution}
  <details class="inspector-disclosure">
    <summary>{t('ux1.entry.compatibility')}</summary>
    <p class="hint">
      {t('ux1.entry.resolved', {
        requested: t('opprops.plunge.ramp'),
        resolved:
          resolution.params?.resolved === 'linearized_ramp'
            ? t('ux1.entry.linearized_ramp')
            : t('opprops.plunge.ramp'),
      })}
    </p>
  </details>
{/if}
