<script lang="ts">
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import type { OpEntry } from '../state/op_types';
  import {
    selectOperationToolCommand,
    toolFitsOperation,
    type ToolSlot,
  } from '../state/operation-tool';
  import { stockedCopy } from '../state/tool_inventory';
  import { effectiveModes } from '../state/tool_family';
  import { formatLength } from '../cam/units';
  import { t } from '../i18n';
  let { op, slot = 'toolId' }: { op: OpEntry; slot?: ToolSlot } = $props();
  const selected = $derived(
    slot === 'toolId'
      ? op.toolId
      : slot === 'spotToolId' && op.kind === 'drill'
        ? op.spotFirst?.spotToolId
        : 'finishToolId' in op
          ? op.finishToolId === op.toolId
            ? undefined
            : op.finishToolId
          : undefined,
  );
  const modes = $derived(effectiveModes(project.data.machine));
  const stocked = $derived(
    project.data.tools.filter((tool) => toolFitsOperation(op, tool, modes, slot)),
  );
  const inventory = $derived.by(() => {
    void workspace.version;
    return workspace
      .get()
      .tool_inventory.filter(
        (tool) =>
          toolFitsOperation(op, tool, modes, slot) && !stockedCopy(tool, project.data.tools),
      );
  });
  function choose(value: string) {
    if (!value && slot === 'finishToolId') {
      const main = project.data.tools.find((tool) => tool.id === op.toolId);
      if (main)
        project.history.exec(
          selectOperationToolCommand(op.id, slot, main, 'project'),
          project.target(),
        );
      return;
    }
    const [origin, rawId] = value.split(':');
    const tool = (
      origin === 'inventory' ? workspace.get().tool_inventory : project.data.tools
    ).find((tool) => tool.id === Number(rawId));
    if (tool)
      project.history.exec(
        selectOperationToolCommand(
          op.id,
          slot,
          tool,
          origin === 'inventory' ? 'inventory' : 'project',
        ),
        project.target(),
      );
  }
</script>

<select
  aria-label={t(
    slot === 'toolId'
      ? 'opprops.tool'
      : slot === 'spotToolId'
        ? 'ops.drill.spot_tool.label'
        : 'opprops.finish_tool',
  )}
  title={t('v11.tool_auto')}
  value={selected == null ? '' : `project:${selected}`}
  onchange={(e) => choose(e.currentTarget.value)}
>
  {#if slot === 'finishToolId'}<option value="">{t('opprops.finish_tool.same_as_rough')}</option
    >{/if}
  {#if selected != null && !stocked.some((tool) => tool.id === selected)}<option
      value={`project:${selected}`}
      disabled
      >{project.data.tools.find((tool) => tool.id === selected)?.name ?? `#${selected}`} — {t(
        'v11.tool_choose',
      )}</option
    >{/if}
  <optgroup label={t('v11.tool_project')}>
    {#each stocked as tool (tool.id)}<option
        value={`project:${tool.id}`}
        disabled={slot === 'finishToolId' && tool.id === op.toolId}
        >#{tool.id} {tool.name} ({formatLength(tool.diameter, project.data.machine.unit)})</option
      >{/each}
  </optgroup>
  {#if inventory.length}<optgroup label={t('v11.tool_library')}>
      {#each inventory as tool (tool.id)}<option value={`inventory:${tool.id}`}
          >{tool.name} ({formatLength(tool.diameter, project.data.machine.unit)})</option
        >{/each}
    </optgroup>{/if}
</select>

<style>
  select {
    width: 100%;
    min-width: 0;
    flex: 1;
  }
</style>
