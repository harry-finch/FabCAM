<script lang="ts">
  import type { Snippet } from 'svelte';
  import Modal from './Modal.svelte';
  import { t } from '../i18n';
  let {
    onClose,
    context,
    busy,
    children,
    footer,
    selection,
  }: {
    onClose: () => void;
    context: string;
    busy: boolean;
    children: Snippet;
    footer: Snippet;
    selection?: Snippet;
  } = $props();
</script>

<Modal
  {onClose}
  width="min(960px, 95vw)"
  maxHeight="85vh"
  ariaLabelledBy="cutting-recommendation-title"
>
  <div class="selector" aria-busy={busy}>
    <header>
      <h4 id="cutting-recommendation-title">{t('selector.title')}</h4>
      <p title={context}>{context}</p>
      {#if selection}{@render selection()}{/if}
    </header>
    <div class="content">{@render children()}</div>
    <footer>{@render footer()}</footer>
  </div>
</Modal>

<style>
  .selector {
    display: flex;
    flex-direction: column;
    max-height: calc(85vh - 2px);
    font-size: var(--inspector-font-field);
    line-height: 1.35;
  }
  header,
  footer {
    flex: none;
    padding: var(--ui-space-3);
  }
  header {
    border-bottom: 1px solid var(--border);
  }
  h4 {
    margin: 0 0 var(--ui-space-1);
    font-size: var(--inspector-font-field);
    font-weight: 600;
  }
  p {
    margin: 0;
    color: var(--text-muted);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .content {
    overflow: auto;
    min-height: 0;
    padding: 0 var(--ui-space-3) var(--ui-space-2);
  }
  footer {
    border-top: 1px solid var(--border);
    background: var(--bg-panel);
  }
</style>
