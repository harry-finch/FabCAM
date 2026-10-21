<script lang="ts">
  import { project } from '../state/project.svelte';
  import { t } from '../i18n';
</script>

{#if project.data.operations.length > 1}
  <!-- Project-level tool-change-order optimization. Groups
           consecutive same-tool ops so a T1/T2/T1 program emits one
           tool change instead of two. Pin individual ops (in the op
           panel) to lock a stability-critical order. -->
  <label class="group-by-tool" title={t('oplist.group_by_tool.title')}>
    <input
      type="checkbox"
      checked={project.data.groupOpsByTool}
      onchange={(e) => project.setGroupOpsByTool((e.currentTarget as HTMLInputElement).checked)}
    />
    {t('oplist.group_by_tool')}
  </label>
{/if}

<style>
  /* Project-level tool-grouping toggle above the ops list. */
  .group-by-tool {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.8rem;
    color: var(--text-muted, var(--text));
    margin: 0 0 0.5rem;
    cursor: pointer;
    user-select: none;
  }
  .group-by-tool input {
    margin: 0;
  }
</style>
