<script lang="ts">
  import { project } from '../state/project.svelte';
  import { DEFAULT_OSNAP_SETTINGS } from '../canvas/osnap';
  import { t } from '../i18n';
  const snap = $derived(project.data.settings.osnap ?? DEFAULT_OSNAP_SETTINGS);
  const kinds = ['endpoint', 'midpoint', 'intersection', 'center', 'grid'] as const;
</script>

<details class="snap-controls">
  <summary>{t('settings.group.snap')}</summary>
  <div class="options">
    {#each kinds as kind (kind)}<label title={t(`settings.snap.${kind}.help`)}
        ><input
          type="checkbox"
          checked={snap[kind]}
          onchange={(e) =>
            project.updateSettings({ osnap: { ...snap, [kind]: e.currentTarget.checked } })}
        />{t(`settings.snap.${kind}`)}</label
      >{/each}
  </div>
  <label
    >{t('settings.snap.grid_step')}<input
      type="number"
      min="0.001"
      step="1"
      value={snap.gridStepMm}
      onchange={(e) => {
        const n = Number(e.currentTarget.value);
        if (Number.isFinite(n) && n > 0)
          project.updateSettings({ osnap: { ...snap, gridStepMm: n } });
      }}
    />mm</label
  >
  <p>{t('settings.snap.hint')}</p>
</details>

<style>
  .snap-controls {
    border-bottom: 1px solid var(--border);
    padding: var(--ui-space-2);
    margin-bottom: var(--ui-space-2);
    font-size: 0.8rem;
  }
  summary {
    cursor: pointer;
  }
  .options {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
    margin: var(--ui-space-2) 0;
  }
  label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }
  input[type='number'] {
    width: 5rem;
  }
  p {
    color: var(--text-muted);
    margin-bottom: 0;
  }
</style>
