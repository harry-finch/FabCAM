<script lang="ts">
  import { onMount, type Snippet } from 'svelte';
  import { attachAnchoredPopover, handlePopoverKey } from './anchored-popover';
  let {
    anchor,
    id,
    label,
    onClose,
    children,
  }: {
    anchor: HTMLElement;
    id: string;
    label: string;
    onClose: (restoreFocus?: boolean) => void;
    children: Snippet;
  } = $props();
  const topLayer = typeof HTMLElement !== 'undefined' && 'showPopover' in HTMLElement.prototype;
  let panel = $state<HTMLDivElement>();
  let layer = $state<HTMLElement>();
  onMount(() =>
    panel && layer ? attachAnchoredPopover(panel, anchor, onClose, layer) : undefined,
  );
</script>

{#if topLayer}
  <div bind:this={layer} class="popover-layer" popover="manual">
    <div
      {id}
      bind:this={panel}
      class="panel"
      role="dialog"
      aria-label={label}
      aria-modal="true"
      tabindex="-1"
      onkeydown={(event) => handlePopoverKey(event, panel, onClose)}
    >
      {@render children()}
    </div>
  </div>
{:else}
  <dialog
    bind:this={layer}
    class="popover-layer"
    aria-label={label}
    onkeydown={(event) => handlePopoverKey(event, panel, onClose)}
  >
    <div {id} bind:this={panel} class="panel" tabindex="-1">
      {@render children()}
    </div>
  </dialog>
{/if}

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-2);
    box-sizing: border-box;
    padding: var(--ui-space-2);
    min-height: 0;
    color: var(--text);
    background: var(--bg-panel);
    font-size: var(--inspector-font-field);
    line-height: 1.35;
  }
  .panel {
    position: fixed;
    inset: auto;
    margin: 0;
    width: min(440px, calc(100vw - 16px));
    max-height: min(480px, calc(100vh - 16px));
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    box-shadow: 0 6px 24px var(--shadow-modal);
  }
  .popover-layer[popover]:not(:popover-open) {
    display: none;
  }
  .panel:focus {
    outline: none;
  }
  .popover-layer {
    position: fixed;
    inset: 0;
    margin: 0;
    padding: 0;
    border: 0;
    width: 100vw;
    height: 100vh;
    max-width: none;
    max-height: none;
    background: transparent;
  }
  dialog.popover-layer:not([open]) {
    display: none;
  }
  dialog::backdrop {
    background: transparent;
  }
</style>
