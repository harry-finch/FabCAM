<script lang="ts">
  import { t } from '../i18n';
  let { active, count, onActivate }: { active: boolean; count: number; onActivate: () => void } =
    $props();
</script>

<button
  type="button"
  class="sidebar-section-identity drawing-section-toggle"
  onclick={onActivate}
  aria-expanded={active}
>
  <span class="group-name">{t('layers.drawings')}</span>
  <span
    class="sidebar-section-count"
    title={t(count === 1 ? 'layers.documents_count.one' : 'layers.documents_count', {
      count: count,
    })}>{count}</span
  >
</button>

<style>
  .drawing-section-toggle {
    flex: 1;
    border: 0;
    background: transparent;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }
  .group-name {
    color: var(--text-strong);
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
