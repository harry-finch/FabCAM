<script lang="ts">
  import { project } from '../state/project.svelte';
  import { t } from '../i18n';
</script>

<label title={t('preview.toolpaths.tip')}>
  <input
    type="checkbox"
    checked={project.gen.toolpath3dVisible}
    onchange={(e) => (project.gen.toolpath3dVisible = e.currentTarget.checked)}
  />
  <span>{t('preview.toolpaths.label')}</span>
</label>

<style>
  label {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-1);
    padding: var(--ui-space-1) var(--ui-space-2);
    font-size: var(--inspector-font-field);
    color: var(--text-muted);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg-panel);
    cursor: pointer;
  }
  input {
    margin: 0;
  }
</style>
