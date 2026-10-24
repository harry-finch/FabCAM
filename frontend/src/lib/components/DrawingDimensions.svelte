<script lang="ts">
  import { fileDimensions, proportionalScale } from '../state/file-transform';
  import type { ImportEntry } from '../state/project-types';
  import { project } from '../state/project.svelte';
  import { t } from '../i18n';
  let { entry }: { entry: ImportEntry } = $props();
  let invalid = $state(false);
  const dimensions = $derived(fileDimensions(entry));
</script>

{#each ['width', 'height'] as axis (axis)}
  <label>
    <span>{t(axis === 'width' ? 'layers.xform.width' : 'layers.xform.height')}</span>
    <span class="xform-field">
      <input
        type="number"
        min="0"
        step="any"
        disabled={!!project.data.drawing ||
          !!project.data.cornerRelief ||
          dimensions[axis as 'width' | 'height'] <= 0}
        value={Number(dimensions[axis as 'width' | 'height'].toFixed(6))}
        oninput={() => (invalid = false)}
        onchange={(e) => {
          const input = e.currentTarget;
          const scale = proportionalScale(entry, axis as 'width' | 'height', input.valueAsNumber);
          if (scale === null) invalid = true;
          if (scale !== null && scale !== entry.fileTransform.scale)
            project.patchFileTransformForImport(entry.id, { scale });
        }}
      />
      <span class="xform-unit">{t('machine.unit.mm')}</span>
    </span>
  </label>
{/each}
{#if invalid}<p class="xform-size-help" role="alert">{t('layers.xform.invalid_size')}</p>{/if}
<p class="xform-size-help">{t('layers.xform.dimensions_help')}</p>
<p class="xform-size-help">
  {t(
    project.data.drawing || project.data.cornerRelief
      ? 'layers.xform.edited_locked'
      : 'layers.xform.drag_help',
  )}
</p>

<style>
  label {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 4px;
    min-width: 0;
  }
  label > span:first-child {
    font-size: 0.9em;
  }
  .xform-field {
    display: flex;
    align-items: center;
    gap: 4px;
    flex: 1;
    min-width: 0;
  }
  input {
    width: 100%;
    min-width: 0;
  }
  .xform-unit {
    color: var(--text-muted);
    font-size: 0.85em;
  }
  .xform-size-help {
    grid-column: 1 / -1;
    margin: 0 0 6px;
    color: var(--text-muted);
    font-size: 0.85em;
  }
</style>
