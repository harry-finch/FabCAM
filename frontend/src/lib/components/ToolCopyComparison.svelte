<script lang="ts">
  import { KIND_DISPLAY_LABELS } from '../state/tool_family';
  import type { ToolKind } from '../state/op_types';
  let {
    before,
    after,
  }: {
    before: { label: string; name: string; kind: ToolKind; diameter: number };
    after: { label: string; name: string; kind: ToolKind; diameter: number };
  } = $props();
</script>

<div class="comparison">
  {#each [before, after] as copy, i (i)}
    <div>
      <small>{copy.label}</small><strong>{copy.name}</strong><span
        >{KIND_DISPLAY_LABELS[copy.kind]} · Ø{copy.diameter} mm</span
      >
    </div>
  {/each}
</div>

<style>
  .comparison {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
    margin: 8px 0;
  }
  .comparison > div {
    border-left: 3px solid var(--warning, #d8a540);
    padding: 4px 10px;
    overflow-wrap: anywhere;
  }
  strong,
  span,
  small {
    display: block;
  }
  small {
    color: var(--text-muted);
    margin-bottom: 4px;
  }
</style>
