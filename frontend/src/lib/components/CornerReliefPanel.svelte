<script lang="ts">
  import { t } from '../i18n';
  import { onDestroy } from 'svelte';
  import { Scissors, X } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { defaultClient } from '../api/http';
  import type { CornerReliefRequest, ReliefMode, RemovalSide } from '../api/types';
  import { combineImports } from '../state/file-transform';
  import {
    cornerReliefUi as ui,
    closeCornerRelief,
    cornerKey,
    toggleCorner,
  } from '../state/corner-relief-ui.svelte';
  import {
    effectiveDiameter,
    evaluatedImport,
    reliefCommand,
    reliefSourceKey,
    reliefError,
    reliefOperationError,
    reliefMarkerError,
    type CornerReliefModifier,
  } from '../state/corner-relief';
  let toolId = $state(0),
    mode = $state<ReliefMode>('dogbone'),
    side = $state<RemovalSide>('interior');
  let selectedObjects = $state<number[]>([]),
    busy = $state(false),
    error = $state(''),
    responseKey = $state('');
  let epoch = 0;
  // The layer panel unmounts when Operations/Text becomes active. Its draft and
  // canvas click interception must end together; late previews must not revive it.
  onDestroy(() => {
    ++epoch;
    closeCornerRelief();
  });
  const client = defaultClient();
  const source = $derived(combineImports(project.data.imports));
  const sourceKey = $derived(reliefSourceKey(project.data.imports));
  const tools = $derived(
    project.data.tools.filter((t) => t.kind === 'endmill' && effectiveDiameter(t) > 0),
  );
  const tool = $derived(tools.find((t) => t.id === toolId));
  const stale = $derived(
    reliefError(project.data.cornerRelief, project.data.imports, project.data.tools),
  );
  const request = $derived<CornerReliefRequest | null>(
    source && tool
      ? {
          segments: source.segments,
          objects: source.objects ?? [],
          selected_objects: selectedObjects,
          selected_corners: ui.corners,
          mode,
          removal_side: side,
          tool_diameter_mm: effectiveDiameter(tool),
        }
      : null,
  );
  const key = $derived(JSON.stringify([request, sourceKey, project.documentEpoch]));
  function begin(edit = false) {
    if (project.data.drawing) {
      project.setError('Reset drawing repairs before applying corner relief.');
      return;
    }
    const old = edit ? project.data.cornerRelief : undefined;
    selectedObjects = old?.recipe.selected_objects ?? [...project.sel.selectedObjects];
    toolId = old?.toolId ?? tools[0]?.id ?? 0;
    mode = old?.recipe.mode ?? 'dogbone';
    side = old?.recipe.removal_side ?? 'interior';
    ui.corners = old?.recipe.selected_corners.map((c) => ({ ...c })) ?? [];
    ui.editing = true;
    error = '';
  }
  $effect(() => {
    const active = ui.editing,
      req = request,
      current = key;
    const token = ++epoch;
    ui.original = source;
    ui.response = null;
    responseKey = '';
    if (!active || !req || !req.selected_objects.length) {
      busy = false;
      return;
    }
    busy = true;
    ui.diameter = req.tool_diameter_mm;
    void client
      .cornerRelief(req)
      .then((response) => {
        if (token !== epoch) return;
        ui.response = response;
        responseKey = current;
        error = response.error ?? '';
      })
      .catch((e) => {
        if (token === epoch) error = String(e);
      })
      .finally(() => {
        if (token === epoch) busy = false;
      });
  });
  let previousDocument = $state(project.documentEpoch);
  $effect(() => {
    if (previousDocument !== project.documentEpoch) {
      previousDocument = project.documentEpoch;
      closeCornerRelief();
    }
  });
  function apply() {
    if (
      !source ||
      !request ||
      !ui.response ||
      responseKey !== key ||
      busy ||
      ui.response.error ||
      !ui.corners.length
    )
      return;
    const next: CornerReliefModifier = {
      version: 1,
      sourceKey,
      toolId,
      diameter: request.tool_diameter_mm,
      recipe: {
        mode,
        removal_side: side,
        selected_objects: selectedObjects,
        selected_corners: ui.corners.map((c) => ({ ...c })),
      },
      evaluated: evaluatedImport(source, ui.response),
    };
    const problem = reliefOperationError(next, project.data.operations);
    if (problem) {
      error = problem;
      return;
    }
    project.history.exec(reliefCommand(project.data.cornerRelief, next), project.target());
    closeCornerRelief();
  }
  function remove() {
    const problem = reliefMarkerError(project.data.cornerRelief, project.data.operations);
    if (problem) {
      project.setError(problem);
      return;
    }
    project.history.exec(reliefCommand(project.data.cornerRelief, undefined), project.target());
    closeCornerRelief();
  }
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === 'Escape' && ui.editing) {
      e.preventDefault();
      closeCornerRelief();
    }
  }}
/>
<div class="relief-panel">
  {#if !ui.editing}
    <button
      class="entry"
      onclick={() => begin(!!project.data.cornerRelief)}
      disabled={project.data.machine.unit !== 'mm' ||
        !source ||
        (!project.data.cornerRelief && !project.sel.selectedObjects.size)}
      title={t('cad1.entry_tip')}
      ><Scissors size={14} />
      {project.data.cornerRelief ? t('cad1.edit') : t('cad1.title')}</button
    >
    {#if project.data.cornerRelief}<button class="quiet" onclick={remove}>{t('cad1.remove')}</button
      >{/if}
    {#if !project.data.cornerRelief && !project.sel.selectedObjects.size}
      <p class="hint">{t('cad1.select_contours_hint')}</p>
    {/if}
    {#if stale}<p class="warning">{stale}</p>{/if}
  {:else}
    <div class="heading">
      <strong>{t('cad1.title')}</strong><button
        class="quiet"
        onclick={closeCornerRelief}
        aria-label={t('cad1.cancel_title')}><X size={14} /></button
      >
    </div>
    {#if !selectedObjects.length}
      <p class="hint" role="status">{t('cad1.empty_contours')}</p>
      <button onclick={closeCornerRelief}>{t('cad1.select_contours')}</button>
    {:else}
      <label
        >{t('cad1.cutter')}
        <select bind:value={toolId}
          >{#each tools as t (t.id)}<option value={t.id}
              >{t.name} · Ø{effectiveDiameter(t)} mm</option
            >{/each}</select
        ></label
      >
      <label
        >{t('cad1.mode')}
        <select bind:value={mode}
          ><option value="dogbone">{t('cad1.dogbone')}</option><option value="tbone_incoming"
            >{t('cad1.incoming')}</option
          ><option value="tbone_outgoing">{t('cad1.outgoing')}</option></select
        ></label
      >
      <label
        >{t('cad1.side')}
        <select bind:value={side}
          ><option value="interior">{t('cad1.inside')}</option><option value="exterior"
            >{t('cad1.outside')}</option
          ></select
        ></label
      >
      <p class="hint">{t('cad1.hint')}</p>
      <div class="actions">
        <button
          onclick={() => {
            ui.corners = (ui.response?.candidates ?? [])
              .filter((c) => !c.reason)
              .map((c) => c.reference);
          }}
          disabled={busy || !ui.response}>{t('cad1.select')}</button
        ><button onclick={() => (ui.corners = [])}>{t('cad1.clear')}</button>
      </div>
      <div class="corners" aria-label={t('cad1.selection')}>
        {#each ui.response?.candidates ?? [] as c (cornerKey(c.reference))}
          <label
            class="corner"
            title={c.reason ?? t('cad1.toggle')}
            onmouseenter={() => (ui.highlighted = cornerKey(c.reference))}
            onmouseleave={() => (ui.highlighted = null)}
          >
            <input
              type="checkbox"
              checked={ui.corners.some((x) => cornerKey(x) === cornerKey(c.reference))}
              disabled={!!c.reason}
              onfocus={() => (ui.highlighted = cornerKey(c.reference))}
              onblur={() => (ui.highlighted = null)}
              onchange={() => toggleCorner(c.reference)}
            />
            <span
              >{t('cad1.corner', {
                object: c.reference.object_id,
                corner: c.reference.corner_index + 1,
              })}<small>X {c.vertex.x.toFixed(2)} · Y {c.vertex.y.toFixed(2)} mm</small
              >{#if c.reason}<small>{c.reason}</small>{/if}</span
            >
          </label>
        {/each}
      </div>
      {#if !tools.length}<p class="warning">
          {t('cad1.no_tool')}
        </p>{/if}
      {#if error}<p class="warning" role="status">{error}</p>{/if}
      <div class="actions">
        <span>{busy ? t('cad1.busy') : t('cad1.selected', { count: ui.corners.length })}</span
        ><button onclick={closeCornerRelief}>{t('cad1.cancel')}</button><button
          class="primary"
          onclick={apply}
          disabled={busy ||
            !ui.corners.length ||
            !ui.response ||
            !!ui.response.error ||
            responseKey !== key}>{t('cad1.apply')}</button
        >
      </div>
    {/if}
  {/if}
</div>

<style>
  .relief-panel {
    padding: 8px 10px;
    border-top: 1px solid var(--border);
    font-size: 12px;
  }
  button,
  select {
    font: inherit;
    min-height: 28px;
    color: var(--text);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 3px;
  }
  button {
    padding: 3px 7px;
    cursor: pointer;
  }
  button:disabled {
    opacity: 0.45;
    cursor: default;
  }
  .entry,
  .heading,
  .actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .heading {
    justify-content: space-between;
    margin-bottom: 8px;
  }
  .entry {
    display: inline-flex;
  }
  .quiet {
    background: transparent;
    border: 0;
  }
  .primary {
    background: var(--accent);
    color: white;
  }
  .actions {
    margin-top: 8px;
    flex-wrap: wrap;
  }
  .actions span {
    margin-right: auto;
  }
  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
    margin: 4px 0;
  }
  select {
    min-width: 0;
    max-width: 75%;
    flex: 1;
  }
  .hint {
    color: var(--text-muted);
    line-height: 1.4;
  }
  .warning {
    color: var(--warn);
    line-height: 1.4;
  }
  .corners {
    max-height: 200px;
    overflow: auto;
  }
  .corner {
    justify-content: flex-start;
    align-items: flex-start;
  }
  .corner small {
    display: block;
    color: var(--text-muted);
    font-size: 11px;
  }
  :focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
