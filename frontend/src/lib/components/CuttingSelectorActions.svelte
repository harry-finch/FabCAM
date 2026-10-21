<script lang="ts">
  import { t } from '../i18n';
  let {
    blocked,
    onCancel,
    onApply,
  }: { blocked: boolean; onCancel: () => void; onApply: () => void } = $props();
</script>

<div class="actions">
  <button type="button" class="inspector-action" onclick={onCancel}>{t('common.cancel')}</button>
  <button type="button" class="inspector-action primary" disabled={blocked} onclick={onApply}
    >{t('selector.apply')}</button
  >
</div>

<style>
  .actions {
    display: flex;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
  }
</style>
