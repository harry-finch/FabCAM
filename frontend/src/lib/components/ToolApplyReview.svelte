<script lang="ts">
  import { t } from '../i18n';
  import type { toolLibraryImpacts } from '../state/tool-library-review';
  import ToolImpactList from './ToolImpactList.svelte';
  let {
    isInventory,
    impacts,
    reviewCurrent,
    disabled,
    onCancel,
    onApply,
  }: {
    isInventory: boolean;
    impacts: ReturnType<typeof toolLibraryImpacts>;
    reviewCurrent: boolean;
    disabled: boolean;
    onCancel: () => void;
    onApply: () => void;
  } = $props();
</script>

<section class="review" aria-label={t('ux6b.apply_title')}>
  <div class="summary" role="status">
    <strong>{t('ux6b.apply_title')}</strong>
    <p>{isInventory ? t('ux6b.apply_scope') : t('ux6b.project_scope')}</p>
    <ToolImpactList {impacts} />
    {#if isInventory && !impacts.length}<p>{t('ux6b.no_sync')}</p>{/if}
    {#if !reviewCurrent}<p>{t('ux6b.review_changed')}</p>{/if}
  </div>
  <div class="actions">
    <button class="btn-secondary" onclick={onCancel}>{t('common.cancel')}</button>
    <button class="btn-primary" {disabled} onclick={onApply}>{t('common.apply')}</button>
  </div>
</section>

<style>
  .review {
    flex-shrink: 0;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elevated);
    font-size: 0.8rem;
  }
  .summary {
    max-height: min(240px, 30vh);
    overflow: auto;
    padding: 10px 12px 0;
  }
  .actions {
    display: flex;
    gap: 6px;
    padding: 8px 12px;
  }
</style>
