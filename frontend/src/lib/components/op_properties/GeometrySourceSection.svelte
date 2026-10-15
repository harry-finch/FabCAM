<script lang="ts">
  import { project, type OpEntry, type SourceCombine } from '../../state/project.svelte';
  import { t } from '../../i18n';
  import { STOCK_OUTLINE_LAYER } from '../../state/stock-outline';
  import {
    effectiveSourceMode,
    sourceModeLabels,
    sourceSignature,
    sourceModePatch,
    layerProvenance,
    combineChoices,
    combineHelp,
    contourHelp,
    canInterpretContours,
    type GeometrySourceMode,
    type SourceMemory,
  } from '../../state/geometry-source-ui';

  let { op }: { op: OpEntry } = $props();
  let memory = $state<SourceMemory>({});
  let draft = $state<{ signature: string; mode: GeometrySourceMode } | null>(null);
  const mode = $derived(
    draft?.signature === sourceSignature(op)
      ? draft.mode
      : op.sourceObjects?.length
        ? 'objects'
        : op.sourceLayers !== null
          ? 'layers'
          : 'all',
  );
  const pending = $derived(draft?.signature === sourceSignature(op));
  const layers = $derived(project.geometryView?.layers.filter((l) => l.segment_count > 0) ?? []);

  function remember() {
    if (op.sourceLayers?.length) memory.layers = [...op.sourceLayers];
    if (op.sourceObjects?.length) memory.objects = [...op.sourceObjects];
  }
  function chooseMode(next: GeometrySourceMode) {
    remember();
    const patch = sourceModePatch(next, memory);
    if (patch) {
      draft = null;
      project.updateOperation(op.id, patch);
    } else draft = { signature: sourceSignature(op), mode: next };
  }
  function chooseLayers(value: string) {
    if (!value) return;
    const names = JSON.parse(value) as string[];
    remember();
    draft = null;
    project.updateOperation(op.id, { sourceLayers: names, sourceObjects: undefined });
  }
  function layerLabel(name: string) {
    const p = layerProvenance(name, project.data.imports);
    const label =
      name === STOCK_OUTLINE_LAYER
        ? t('opprops.geometry.stock')
        : p.syntheticSvg
          ? t('opprops.geometry.svg')
          : name;
    const source =
      p.drawings.length === 1
        ? p.drawings[0].filename
        : p.drawings.length > 1
          ? t('opprops.geometry.drawings', { count: p.drawings.length })
          : '';
    return source ? `${label} · ${source}` : label;
  }
  function layerTitle(name: string) {
    if (name === STOCK_OUTLINE_LAYER) return t('opprops.geometry.stock.help');
    const p = layerProvenance(name, project.data.imports);
    return t('opprops.geometry.layer_identity', {
      name,
      files: p.drawings.map((d) => d.filename).join(', '),
    });
  }
</script>

{#if op.geometryBinding?.kind === 'unresolved'}
  <p role="alert">{t('cleanup.reselect')}</p>
{/if}

<fieldset>
  <legend>{t('opprops.source')}</legend>
  {#if project.data.stock.flip != null}
    <label class="row" title={t('opprops.side.title')}>
      <span>{t('opprops.side')}</span>
      <select
        onkeydown={(event) => event.stopPropagation()}
        value={op.side ?? 'front'}
        onchange={(event) =>
          project.updateOperation(op.id, {
            side: event.currentTarget.value === 'back' ? 'back' : undefined,
          })}
      >
        <option value="front">{t('opprops.side.front')}</option>
        <option value="back">{t('opprops.side.back')}</option>
      </select>
    </label>
  {/if}
  <label class="row" title={t('opprops.geometry.source.help')}>
    <span>{t('opprops.source.mode')}</span>
    <select
      onkeydown={(event) => event.stopPropagation()}
      value={mode}
      onchange={(event) => chooseMode(event.currentTarget.value as GeometrySourceMode)}
    >
      <option value="all">{t('opprops.source.mode.all')}</option>
      <option value="layers">{t('opprops.source.mode.layer')}</option>
      <option value="objects">{t('opprops.source.mode.objects')}</option>
    </select>
  </label>
  {#if mode === 'layers'}
    <label class="row" title={t('opprops.geometry.layer.help')}>
      <span
        >{t(
          op.sourceLayers && op.sourceLayers.length > 1
            ? 'opprops.geometry.layers'
            : 'opprops.source.layer',
        )}</span
      >
      <select
        onkeydown={(event) => event.stopPropagation()}
        value={op.sourceLayers?.length ? JSON.stringify(op.sourceLayers) : ''}
        onchange={(event) => chooseLayers(event.currentTarget.value)}
      >
        <option value="" disabled>{t('opprops.source.layer.pick')}</option>
        {#if op.sourceLayers && op.sourceLayers.length > 1}
          <option value={JSON.stringify(op.sourceLayers)}>{op.sourceLayers.join(' + ')}</option>
        {/if}
        {#each layers as layer (layer.name)}
          <option value={JSON.stringify([layer.name])} title={layerTitle(layer.name)}
            >{layerLabel(layer.name)}</option
          >
        {/each}
        {#each [...new Set(op.sourceLayers ?? [])] as name (name)}
          {#if !layers.some((layer) => layer.name === name) && !project.data.textLayers.some((text) => name === `__text_${text.id}`)}
            <option value={JSON.stringify([name])}
              >{t('opprops.geometry.unavailable', { name })}</option
            >
          {/if}
        {/each}
        {#if project.data.textLayers.length}
          <optgroup label={t('opprops.source.text_group')}>
            {#each project.data.textLayers as text (text.id)}
              <option value={JSON.stringify([`__text_${text.id}`])}>{text.name}</option>
            {/each}
          </optgroup>
        {/if}
      </select>
    </label>
  {/if}
  {#if pending}
    <p class="hint warn">
      {t('opprops.geometry.pending', { source: t(sourceModeLabels[effectiveSourceMode(op)]) })}
    </p>
  {:else if effectiveSourceMode(op) === 'objects'}
    <p class="hint" title={t('opprops.geometry.ids', { ids: op.sourceObjects!.join(', ') })}>
      {t('opprops.source.objects_selected', { count: op.sourceObjects!.length })}
    </p>
  {:else if mode === 'layers' && !op.sourceLayers?.length}
    <p class="hint warn">{t('opprops.geometry.empty_layers')}</p>
  {:else if effectiveSourceMode(op) === 'all'}
    <p class="hint">{t('opprops.source.all_chains')}</p>
  {/if}
  {#if canInterpretContours(op)}
    <label class="row" title={t(contourHelp(op))}>
      <span>{t('opprops.combine')}</span>
      <select
        onkeydown={(event) => event.stopPropagation()}
        value={op.sourceCombine ?? 'auto'}
        aria-describedby={`geometry-help-${op.id}`}
        onchange={(event) =>
          project.updateOperation(op.id, {
            sourceCombine: event.currentTarget.value as SourceCombine,
          })}
      >
        {#each combineChoices as choice (choice)}
          <option value={choice} title={t(combineHelp[choice])}
            >{t(`opprops.combine.${choice}`)}</option
          >
        {/each}
      </select>
    </label>
  {/if}
  <p class="hint" id={`geometry-help-${op.id}`}>
    {t(contourHelp(op))}
    {#if op.sourceCombine === 'difference' && canInterpretContours(op) && effectiveSourceMode(op) === 'objects'}
      {t('opprops.geometry.first_operand', { id: op.sourceObjects![0] })}
    {/if}
  </p>
  <button
    class="from-selection"
    class:ghost={project.sel.selectedObjects.size === 0}
    type="button"
    disabled={project.sel.selectedObjects.size === 0}
    aria-label={project.sel.selectedObjects.size === 0
      ? t('opprops.from_selection.disabled_aria')
      : t('opprops.from_selection.count', { count: project.sel.selectedObjects.size })}
    title={project.sel.selectedObjects.size === 0
      ? t('opprops.from_selection.disabled_aria')
      : t('opprops.from_selection.title')}
    onclick={() => {
      remember();
      draft = null;
      project.updateOperation(op.id, {
        sourceLayers: null,
        sourceObjects: [...project.sel.selectedObjects],
      });
    }}
  >
    {project.sel.selectedObjects.size === 0
      ? t('opprops.from_selection')
      : t('opprops.from_selection.count', { count: project.sel.selectedObjects.size })}
  </button>
</fieldset>
