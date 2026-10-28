<script lang="ts">
  import { paletteDispatch } from '../state/palette-dispatch';
  import { onMount, onDestroy } from 'svelte';
  import { buildGenerationProject } from '../state/generation-context';
  import { t } from '../i18n';
  import type { MsgKey } from '../i18n/keys';
  import {
    cleanupError,
    cleanupBlockReason,
    cleanupTechnicalDetails,
  } from '../state/cleanup-feedback';
  import { ScanLine, X } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { defaultClient } from '../api/http';
  import type { ScopedResponse, ScopedRequest, DrawingRequest } from '../api/types';
  import {
    drawingInputKey,
    drawingCommand,
    resetDrawingOperations,
    provenLegacyBinding,
  } from '../state/drawing';
  let previewInput = $state<Extract<ScopedRequest, { kind: 'preview' }>['input'] | null>(null);
  let highlighted = $state<number | null>(null);
  let previewFailed = $state(false);
  let errorDetails = $state('');
  let open = $state(false),
    busy = $state(false),
    error = $state('');
  let response = $state<ScopedResponse | null>(null),
    preview = $state<ScopedResponse | null>(null);
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
      project.data.workOffset,
      project.data.textLayers,
      project.data.stock,
      project.data.fixtures,
      project.data.reliefSources,
      project.data.groupOpsByTool,
      project.sourceFileStaleNotice,
      project.data.tools,
      project.data.machine,
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
    project.data.cornerRelief ? 'Remove corner relief before repairing drawing geometry.' : '',
  );
  const previewReason = $derived(
    cleanupBlockReason(
      {
        busy,
        stale,
        conflict,
        selected: selected.length,
        preview: !!preview,
        failed: previewFailed,
      },
      'preview',
    ),
  );
  const applyReason = $derived(
    cleanupBlockReason(
      {
        busy,
        stale,
        conflict,
        selected: selected.length,
        preview: !!preview,
        failed: previewFailed,
      },
      'apply',
    ),
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
  const diagnosticEdges = $derived(
    new Set(
      highlighted == null
        ? []
        : (response?.topology.issues[highlighted]?.edges ?? []).map(
            (id) => `${id.import}/${id.entity}`,
          ),
    ),
  );
  const edgeIds = $derived(
    response?.drawing.contours.flatMap((c) =>
      c.edges.map((e) => `${e.id.import}/${e.id.entity}`),
    ) ?? [],
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
  const frame = $derived.by(() => {
    if (!bounds) return { x: 0, y: 0, scale: 1 };
    const width = bounds.max_x - bounds.min_x;
    const height = bounds.max_y - bounds.min_y;
    const span = Math.max(width, height, 1e-8);
    const padding = !overview && changedPoints.length ? span * 0.5 : 1;
    return {
      x: bounds.min_x + width / 2,
      y: bounds.min_y + height / 2,
      scale: 1000 / (span + 2 * padding),
    };
  });
  // Display transform only. Normalize the SVG coordinate range because WebKit
  // can drop non-scaling strokes under extreme micro-gap viewBox magnification.
  const viewX = (x: number) => (x - frame.x) * frame.scale + 500;
  const viewY = (y: number) => -(y - frame.y) * frame.scale + 500;
  async function scopedRequest(req: ScopedRequest) {
    if (!client.evaluateDrawingV2)
      throw new Error(
        'Backend does not support scoped drawing v2. Update it before repairing drawings.',
      );
    const result = await client.evaluateDrawingV2(req);
    if (result.version !== 2 || result.drawing.version !== 1)
      throw new Error('Unsupported scoped drawing revision');
    return result;
  }
  const request = (input: DrawingRequest) => scopedRequest({ kind: 'analyze', version: 2, input });
  function context() {
    if (!response) throw new Error('Analyze before preview');
    const machining = buildGenerationProject(project, true, true);
    if (!machining) throw new Error('Missing project context');
    return JSON.parse(
      JSON.stringify({
        drawing: response.drawing,
        project: machining,
        bindings: project.data.operations.map((o) => ({
          operation: o.id,
          binding: o.geometryBinding ?? provenLegacyBinding(o, response!),
        })),
        document: JSON.parse(current),
      }),
    ) as Extract<ScopedRequest, { kind: 'preview' }>['input']['context'];
  }
  async function analyze() {
    if (busy) return;
    if (!Number.isFinite(tolerance) || tolerance < 0) {
      error = 'Enter a finite, nonnegative tolerance in mm.';
      return;
    }
    const token = ++epoch,
      key = current;
    busy = true;
    error = '';
    errorDetails = '';
    preview = null;
    previewInput = null;
    highlighted = null;
    previewFailed = false;
    selected = [];
    response = null;
    captured = '';
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
            legacy_text: buildGenerationProject(project, true, true)?.text_layers ?? [],
            legacy_auxiliary: (buildGenerationProject(project, true, true)?.segments ?? []).slice(
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
      if (token === epoch) {
        error = cleanupError(e);
        errorDetails = cleanupTechnicalDetails(e);
      }
    } finally {
      if (token === epoch) busy = false;
    }
  }
  async function showPreview() {
    if (busy || !response || stale || conflict || !selected.length) return;
    const token = ++epoch,
      key = current;
    busy = true;
    error = '';
    errorDetails = '';
    previewFailed = false;
    preview = null;
    try {
      const input = {
        context: context(),
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
      };
      const next = await scopedRequest({ kind: 'preview', version: 2, input });
      if (token === epoch && key === current) {
        preview = next;
        previewInput = input;
      }
    } catch (e) {
      if (token === epoch) {
        error = cleanupError(e);
        errorDetails = cleanupTechnicalDetails(e);
        previewFailed = true;
      }
    } finally {
      if (token === epoch) busy = false;
    }
  }
  async function apply() {
    if (!preview || !previewInput || stale || conflict || busy) return;
    const key = current,
      token = ++epoch;
    busy = true;
    try {
      const checked = await scopedRequest({
        kind: 'apply',
        version: 2,
        preview: previewInput,
        current: context(),
      });
      if (token !== epoch || key !== current)
        throw new Error('Drawing preview is stale; analyze again');
      if (!checked.local_edit_accepted) throw new Error('Backend did not accept this local edit');
      preview = checked;
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
            version: 2,
            originalBindings:
              project.data.drawing?.originalBindings ??
              project.data.operations.map(
                ({ id, sourceObjects, sourceLayers, geometryBinding }) => ({
                  id,
                  sourceObjects,
                  sourceLayers,
                  geometryBinding,
                }),
              ),
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
      previewInput = null;
    } catch (e) {
      error = cleanupError(e);
      errorDetails = cleanupTechnicalDetails(e);
    } finally {
      if (token === epoch) busy = false;
    }
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
      errorDetails = '';
    } catch (e) {
      error = cleanupError(e);
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
    {#if error}<p role="alert">{error}</p>
      {#if errorDetails && errorDetails !== error}<details>
          <summary>{t('cleanup.technical_details')}</summary>
          <pre>{errorDetails}</pre>
        </details>{/if}{/if}
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
      <p class="muted">{t('cleanup.scoped_note')}</p>
      {#if !response.topology.complete}<p role="alert">
          {t('cleanup.topology_incomplete')}
        </p>{/if}
      {#each response.topology.issues as issue, i (i)}
        <button
          class="btn btn-small"
          onclick={() => {
            highlighted = i;
            overview = true;
          }}
          >{issue.contours.length
            ? issue.contours.map((id) => `Drawing ${id.import} · contour ${id.entity}`).join(' / ')
            : t('cleanup.global_issue')}: {issue.message}</button
        >
      {/each}
      {#each response.continuity.filter((c) => !c.exact) as c (`${c.contour.import}/${c.contour.entity}`)}
        <p class="muted">
          {t('cleanup.continuity_gap', {
            contour: c.contour.entity,
            gap: c.max_gap_mm.toPrecision(6),
          })}
        </p>
      {/each}
      <div class="findings">
        {#each response.findings as finding, i (i)}
          <label
            ><input
              type="checkbox"
              disabled={finding.repair?.kind !== 'gap' ||
                busy ||
                stale ||
                !response.topology.complete}
              checked={selected.includes(i)}
              onchange={(e) => {
                selected = e.currentTarget.checked
                  ? [...selected, i]
                  : selected.filter((n) => n !== i);
                preview = null;
                previewFailed = false;
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
      {#if preview || highlighted !== null}
        <label><input type="checkbox" bind:checked={overview} />{t('cleanup.overview')}</label>
        <svg viewBox="0 0 1000 1000" aria-label={t('cleanup.preview_label')} role="img">
          {#each response.evaluated.segments as s, i (i)}{#if s.type === 'LINE'}<line
                x1={viewX(s.start.x)}
                y1={viewY(s.start.y)}
                x2={viewX(s.end.x)}
                y2={viewY(s.end.y)}
                stroke={diagnosticEdges.has(edgeIds[i]) ? 'var(--danger, #ef6767)' : 'currentColor'}
                stroke-width={diagnosticEdges.has(edgeIds[i]) ? 0.5 : 0.1}
                stroke-dasharray={diagnosticEdges.has(edgeIds[i]) ? undefined : '3 3'}
              />{/if}{/each}
          {#each preview?.changes ?? [] as change (`${change.id.import}/${change.id.entity}`)}{#if change.after}<line
                x1={viewX(change.after.start.x)}
                y1={viewY(change.after.start.y)}
                x2={viewX(change.after.end.x)}
                y2={viewY(change.after.end.y)}
                stroke="var(--accent, #4a90e2)"
              />{/if}{/each}
        </svg>
        <p class="muted">
          {t('cleanup.legend', {
            count: (preview?.bindings ?? []).filter((b) => b.kind === 'unresolved').length,
          })}
        </p>
      {/if}
      <div class="actions">
        <button class="btn btn-small" onclick={showPreview} disabled={!!previewReason}
          >{t('cleanup.preview')}</button
        ><button class="btn btn-small" onclick={apply} disabled={!!applyReason}
          >{t('cleanup.apply')}</button
        >
      </div>
    {/if}
    {#if response && previewReason}<p role="status">
        {t('cleanup.preview')}: {t(previewReason as MsgKey)}
      </p>{/if}
    {#if response && applyReason}<p role="status">
        {t('cleanup.apply')}: {t(applyReason as MsgKey)}
      </p>{/if}
    {#if busy}<p role="status">{t('cleanup.busy')}</p>{/if}
  {/if}
</div>

<style>
  pre {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
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
