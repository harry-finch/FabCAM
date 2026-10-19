<script lang="ts">
  import { project } from '../../state/project.svelte';
  import { workspace } from '../../state/workspace.svelte';
  import { operationCapability } from '../../state/inspector-policy';
  import type { OpKind } from '../../state/op_types';
  import { t } from '../../i18n';
  let { kind }: { kind: OpKind } = $props();
  const post = $derived(project.data.machine.gcodeDialect ?? workspace.get().last_post_processor);
  const reason = $derived(operationCapability(kind, post, project.data.machine));
</script>

{#if reason}<p class="warn-chip" role="status">{t(reason)}</p>
{:else if post === 'cps' && kind === 'pause'}<p class="hint">{t('uxa.pause_cps')}</p>{/if}
