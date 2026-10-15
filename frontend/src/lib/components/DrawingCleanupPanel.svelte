<script lang="ts">
  import { paletteDispatch } from '../state/palette-dispatch';
  import { onMount, onDestroy } from 'svelte';
  import { buildGenerationProject } from '../state/generation-context';
  import { t } from '../i18n';
  import { ScanLine, X } from '@lucide/svelte';
  import { isContourOp } from '../state/op_types';
  import { project } from '../state/project.svelte';
  import { defaultClient } from '../api/http';
  import type { DrawingResponse, DrawingRequest } from '../api/types';
  import {
    drawingInputKey,
    drawingCommand,
    resetDrawingOperations,
    provenLegacyBinding,
  } from '../state/drawing';
  let open = $state(false),
    busy = $state(false),
    error = $state('');
  let response = $state<DrawingResponse | null>(null),
    preview = $state<DrawingResponse | null>(null);
  let selected = $state<number[]>([]),
    captured = $state('');
  let tolerance = $state(0.1),
    connector = $state(true),
    simplification = $state(0.01),
    analyzeSimplification = $state(false),
    overview = $state(false);
  let epoch = 0;
  onDestroy(() => {
    ++epoch;
  });
  const client = defaultClient();
  const current = $derived(
    JSON.stringify([
      drawingInputKey(project.data.imports),
      project.data.operations,
      project.data.cornerRelief,
      project.documentEpoch,
      project.data.drawing,
      tolerance,
      simplification,
      analyzeSimplification,
      connector,
    ]),
  );
  onMount(() =>
    paletteDispatch.register('drawing-cleanup', {
      state: () => ({
        token: current,
        reason: !project.data.imports.length ? t('cleanup.no_import') : undefined,
      }),
      run: () => {
        open = true;
        return analyze();
      },
    }),
  );
  const stale = $derived(!!captured && captured !== current);
  const conflict = $derived(
    project.data.cornerRelief
      ? 'Remove corner relief before repairing drawing geometry.'
      : project.data.operations.some(
            (o) => isContourOp(o) && (o.tabsActive || o.tabPlacements?.length || o.approachPoint),
          )
        ? 'Remove authored tabs and approach points before repairing geometry.'
        : '',
  );
  // View framing only; all proposed coordinates are supplied by Rust.
  const changedPoints = $derived(
    preview?.changes.flatMap(({ before, after }) => {
      if (!before || !after)
        return [before?.start, before?.end, after?.start, after?.end].filter((p) => p != null);
      return ['start', 'end'].flatMap((key) => {
        const a = before[key as 'start' | 'end'],
          b = after[key as 'start' | 'end'];
        return a.x !== b.x || a.y !== b.y ? [a, b] : [];
      });
    }) ?? [],
  );
  const bounds = $derived(
    !overview && changedPoints.length
      ? {
          min_x: Math.min(...changedPoints.map((p) => p.x)),
          max_x: Math.max(...changedPoints.map((p) => p.x)),
          min_y: Math.min(...changedPoints.map((p) => p.y)),
          max_y: Math.max(...changedPoints.map((p) => p.y)),
        }
      : (preview?.evaluated.bbox ?? response?.evaluated.bbox),
  );
  const box = $derived(
    bounds
      ? `${bounds.min_x - 1} ${-bounds.max_y - 1} ${Math.max(1, bounds.max_x - bounds.min_x) + 2} ${Math.max(1, bounds.max_y - bounds.min_y) + 2}`
      : '0 0 10 10',
  );
  async function request(req: DrawingRequest) {
    if (!client.evaluateDrawing)
      throw new Error('Backend does not support GEOM1. Update it before repairing drawings.');
    const result = await client.evaluateDrawing(req);
    if (result.version !== 1) throw new Error('Unsupported drawing contract revision');
    return result;
  }
  async function analyze() {
    if (!Number.isFinite(tolerance) || tolerance < 0) {
      error = 'Enter a finite, nonnegative tolerance in mm.';
      return;
    }
    const token = ++epoch,
      key = current;
    busy = true;
    error = '';
    preview = null;
    selected = [];
    try {
      const existing = project.data.drawing;
      if (existing && existing.inputKey !== drawingInputKey(project.data.imports))
        throw new Error(
          'Source or transform changed. Reset repairs before analyzing replacement geometry.',
        );
      const next = existing
        ? await request({
            kind: 'evaluate',
            version: 1,
            tolerance_mm: tolerance,
            simplification_mm: analyzeSimplification ? simplification : null,
            drawing: existing.response.drawing,
          })
        : await request({
            kind: 'start',
            version: 1,
            tolerance_mm: tolerance,
            simplification_mm: analyzeSimplification ? simplification : null,
            legacy_offset: { x: project.data.workOffset.x_mm, y: project.data.workOffset.y_mm },
            legacy_text: buildGenerationProject(project)?.text_layers ?? [],
            legacy_auxiliary: (buildGenerationProject(project)?.segments ?? []).slice(
              project.data.imports.reduce((n, e) => n + e.source.segments.length, 0),
            ),
            imports: project.data.imports.map((e) => ({
              id: e.id,
              original: e.source.segments,
              legacy_objects: e.source.objects ?? [],
              pivot: {
                x: (e.source.bbox.min_x + e.source.bbox.max_x) / 2,
                y: (e.source.bbox.min_y + e.source.bbox.max_y) / 2,
              },
              next_entity: 1,
              transform: {
                translate: e.fileTransform.translate,
                rotation_deg: e.fileTransform.rotateDeg,
                scale: e.fileTransform.scale,
                mirror_x: e.fileTransform.mirrorX,
                mirror_y: e.fileTransform.mirrorY,
              },
            })),
          });
      if (token !== epoch || key !== current) return;
      response = next;
      captured = key;
    } catch (e) {
      if (token === epoch) error = String(e);
    } finally {
      if (token === epoch) busy = false;
    }
  }
  async function showPreview() {
    if (!response || stale || conflict || !selected.length) return;
    const token = ++epoch,
      key = current;
    busy = true;
    error = '';
    preview = null;
    try {
      const next = await request({
        kind: 'preview',
        version: 1,
        drawing: response.drawing,
        repairs: selected.flatMap((i) => {
          const r = response?.findings[i].repair;
          return r
            ? [
                r.kind === 'gap' || r.kind === 'join'
                  ? { ...r, connector, tolerance_mm: tolerance }
                  : r,
              ]
            : [];
        }),
        bindings: project.data.operations.map(
          (o) => o.geometryBinding ?? provenLegacyBinding(o, response!),
        ),
      });
      if (token === epoch && key === current) preview = next;
    } catch (e) {
      if (token === epoch) error = String(e);
    } finally {
      if (token === epoch) busy = false;
    }
  }
  function apply() {
    if (!preview || stale || conflict || busy) return;
    const ops = project.data.operations.map((op, i) => ({
      ...op,
      geometryBinding: preview!.bindings[i],
      ...(preview!.bindings[i]?.kind === 'contours'
        ? { sourceObjects: preview!.binding_indices[i]?.map((n) => n + 1) ?? op.sourceObjects }
        : {}),
    }));
    project.history.exec(
      drawingCommand(
        project.data.drawing,
        {
          version: 1,
          originalBindings:
            project.data.drawing?.originalBindings ??
            project.data.operations.map(({ id, sourceObjects, sourceLayers, geometryBinding }) => ({
              id,
              sourceObjects,
              sourceLayers,
              geometryBinding,
            })),
          inputKey: drawingInputKey(project.data.imports),
          response: preview,
        },
        project.data.operations,
        ops,
      ),
      project.target(),
    );
    project.sel.selectedObjects = new Set();
    response = null;
    preview = null;
    captured = '';
    selected = [];
  }
  function reset() {
    if (!project.data.drawing) return;
    try {
      const operations = resetDrawingOperations(
        project.data.drawing,
        project.data.imports,
        project.data.operations,
      );
      project.history.exec(
        drawingCommand(project.data.drawing, undefined, project.data.operations, operations),
        project.target(),
      );
      response = null;
      preview = null;
      captured = '';
      selected = [];
      error = '';
    } catch (e) {
      error = String(e);
    }
  }
  function close() {
    ++epoch;
    open = false;
    busy = false;
    response = null;
    preview = null;
    captured = '';
    selected = [];
  }
</script>

<div class="drawing-cleanup">
  <div class="heading">
    <button
      class="btn btn-small"
      onclick={() => {
        open = !open;
        if (open) void analyze();
        else close();
      }}
      disabled={!project.data.imports.length}><ScanLine size={14} /> {t('cleanup.title')}</button
    >
    {#if project.data.drawing}<span class="muted">{t('cleanup.edited')}</span>{/if}
    {#if open}<button class="btn-icon" aria-label={t('cleanup.close')} onclick={close}
        ><X size={14} /></button
      >{/if}
  </div>
  {#if open}
    <p class="muted">{t('cleanup.intro')}</p>
    {#if conflict}<p role="status">{conflict}</p>{/if}
    {#if stale}<p role="status">{t('cleanup.stale')}</p>{/if}
    {#if error}<p role="alert">{error}</p>{/if}
    <label
      >{t('cleanup.tolerance')} <input type="number" min="0" step="0.01" bind:value={tolerance} />
      {t('machine.unit.mm')}</label
    >
    <label
      >{t('cleanup.method')}
      <select bind:value={connector}
        ><option value={true}>{t('cleanup.connector')}</option><option value={false}
          >{t('cleanup.snap')}</option
        ></select
      ></label
    >
    <label
      ><input type="checkbox" bind:checked={analyzeSimplification} />{t('cleanup.simplify')}</label
    >
    {#if analyzeSimplification}<label
        >{t('cleanup.deviation')}<input
          type="number"
          min="0"
          step="0.001"
          bind:value={simplification}
        />{t('machine.unit.mm')}</label
      >{/if}
    <div class="actions">
      <button class="btn btn-small" onclick={analyze} disabled={busy}>{t('cleanup.analyze')}</button
      >{#if project.data.drawing}<button class="btn btn-small" onclick={reset} disabled={busy}
          >{t('cleanup.reset')}</button
        >{/if}
    </div>
    {#if response}
      <p class="muted">{t('cleanup.limit', { value: tolerance })}</p>
      {#if !response.findings.length}<p>{t('cleanup.empty')}</p>{/if}
      <div class="findings">
        {#each response.findings as finding, i (i)}
          <label
            ><input
              type="checkbox"
              disabled={!finding.repair || busy || stale}
              checked={selected.includes(i)}
              onchange={(e) => {
                selected = e.currentTarget.checked
                  ? [...selected, i]
                  : selected.filter((n) => n !== i);
                preview = null;
              }}
            /><span
              >{t('cleanup.contour', {
                drawing: finding.contour.import,
                contour: finding.contour.entity,
              })}<small>{finding.message}</small></span
            ></label
          >
        {/each}
      </div>
      {#if preview}
        <label><input type="checkbox" bind:checked={overview} />{t('cleanup.overview')}</label>
        <svg viewBox={box} aria-label={t('cleanup.preview_label')} role="img">
          {#each response.evaluated.segments as s, i (i)}{#if s.type === 'LINE'}<line
                x1={s.start.x}
                y1={-s.start.y}
                x2={s.end.x}
                y2={-s.end.y}
                stroke="currentColor"
                stroke-dasharray="3 3"
              />{/if}{/each}
          {#each preview.changes as change (`${change.id.import}/${change.id.entity}`)}{#if change.after}<line
                x1={change.after.start.x}
                y1={-change.after.start.y}
                x2={change.after.end.x}
                y2={-change.after.end.y}
                stroke="var(--accent, #4a90e2)"
              />{/if}{/each}
        </svg>
        <p class="muted">
          {t('cleanup.legend', {
            count: preview.bindings.filter((b) => b.kind === 'unresolved').length,
          })}
        </p>
      {/if}
      <div class="actions">
        <button
          class="btn btn-small"
          onclick={showPreview}
          disabled={busy || stale || !!conflict || !selected.length}>{t('cleanup.preview')}</button
        ><button
          class="btn btn-small"
          onclick={apply}
          disabled={!preview || busy || stale || !!conflict}>{t('cleanup.apply')}</button
        >
      </div>
    {/if}
    {#if busy}<p role="status">{t('cleanup.busy')}</p>{/if}
  {/if}
</div>

<style>
  .drawing-cleanup {
    padding: 10px;
    border-top: 1px solid var(--border, #383838);
    font-size: 12px;
  }
  button,
  select,
  input[type='number'] {
    border: 1px solid var(--border, #484848);
    border-radius: 4px;
    background: var(--bg-input, #242424);
    color: inherit;
    font: inherit;
    padding: 6px 8px;
    min-height: 30px;
  }
  button:not(:disabled):hover {
    border-color: var(--accent, #4a90e2);
  }
  button:disabled {
    opacity: 0.45;
  }
  button:focus-visible,
  select:focus-visible,
  input:focus-visible {
    outline: 2px solid var(--accent, #4a90e2);
    outline-offset: 2px;
  }
  input[type='number'] {
    width: 90px;
    min-width: 0;
  }
  select {
    min-width: 0;
    flex: 1;
  }
  .heading,
  .actions {
    display: flex;
    gap: 8px;
    align-items: center;
    flex-wrap: wrap;
  }
  .heading button:first-child {
    display: inline-flex;
    gap: 6px;
    align-items: center;
  }
  .muted,
  small {
    color: var(--text-muted, #aaa);
  }
  p {
    margin: 8px 0;
    line-height: 1.4;
  }
  .findings {
    max-height: 200px;
    overflow: auto;
    margin: 8px 0;
  }
  label {
    display: flex;
    gap: 6px;
    align-items: start;
    margin: 6px 0;
  }
  small {
    display: block;
  }
  svg {
    width: 100%;
    height: 180px;
    background: var(--bg, #191919);
    margin: 8px 0;
  }
  line {
    stroke-width: 1.5;
    vector-effect: non-scaling-stroke;
    fill: none;
  }
</style>
