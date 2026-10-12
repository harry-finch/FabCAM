<script lang="ts">
  import LabPresetDialog from '../LabPresetDialog.svelte';
  import { captureLabPreset } from '../../state/feeds-speeds-pack-store';
  import { LAB_PACK_ID, labHasRevision, type LabCapture } from '../../state/lab-presets';
  import { scopeValue } from '../../state/feeds-speeds-explanation';
  import CuttingOriginStatus from './CuttingOriginStatus.svelte';
  import { project, type OpEntry } from '../../state/project.svelte';
  import { applyCuttingCommand } from '../../state/commands';
  import { buildProject } from '../../api/build-project';
  import { cuttingKnowledgeClient } from '../../state/feeds-speeds-pack-store';
  import { packCollection, NORMALIZED_MATERIALS } from '../../state/feeds-speeds-sources';
  import type { FsCamResponse } from '../../api/types';
  import type { components } from '../../api/generated';
  import {
    type DatasetCatalog,
    cuttingContextKey,
    operationRates,
    editOperationRate,
    sourceLink,
    referenceContext,
  } from '../../state/feeds-speeds';
  import { recommendationSources } from '../../state/feeds-speeds-sources.svelte';
  import { t } from '../../i18n';
  type S = components['schemas'];
  let { op }: { op: OpEntry } = $props();
  let labCapture = $state<LabCapture | null>(null);
  let labFingerprint = $state('');
  async function openLabCapture() {
    if (!wire) return;
    const fingerprint = currentLabFingerprint;
    try {
      await recommendationSources.refresh(true);
      const c = await captureLabPreset(wire, op.id);
      c.machine_name = project.data.machine.name ?? c.machine_name;
      if (fingerprint !== currentLabFingerprint) throw new Error(t('fs38.stale'));
      labFingerprint = fingerprint;
      labCapture = c;
      error = '';
    } catch (e) {
      error = String(e);
    }
  }
  function knowledgeName(id: string) {
    return id === LAB_PACK_ID ? t('fs38.sorbonne') : t('fs37.references');
  }
  let review = $state(false);
  let mode = $state<S['FsAggressiveness']>('prudent');
  let responses = $state<
    {
      id: string;
      label: string;
      contents: string;
      catalog: DatasetCatalog;
      result: FsCamResponse;
    }[]
  >([]);
  let responseKey = $state('');
  let busy = $state(false);
  let attempted = $state(false);
  let error = $state('');
  const api = cuttingKnowledgeClient();
  $effect(() => {
    void project.data.settings.feedsSpeedsSources;
    void recommendationSources.refresh();
  });
  const tool = $derived(project.data.tools.find((t) => t.id === op.toolId));
  const mainPass = $derived(op.kind === 'profile' ? 'finish' : 'rough');
  const main = $derived(tool ? operationRates(op, tool, mainPass) : null);
  const finishToolId = $derived('finishToolId' in op ? (op.finishToolId ?? op.toolId) : op.toolId);
  const finishTool = $derived(project.data.tools.find((t) => t.id === finishToolId));
  const secondary = $derived(
    op.kind === 'pocket' && finishTool ? operationRates(op, finishTool, 'finish') : null,
  );
  const separateFinish = $derived(op.kind === 'pocket' && !!secondary);
  const context = $derived(
    referenceContext(project.data.machine, project.data.tools, project.data.fsContext),
  );
  const wire = $derived(
    buildProject({
      transformedImport: project.transformedImport,
      geometryView: project.geometryView,
      stockSizingImport: project.stockSizingImport,
      machine: project.data.machine,
      tools: project.data.tools,
      operations: project.data.operations,
      stock: project.data.stock,
      textLayers: project.data.textLayers,
      workOffset: project.data.workOffset,
      material: project.data.material,
      fsContext: context,
    }),
  );
  const currentLabFingerprint = $derived(JSON.stringify([wire, op.id, project.data.machine.name]));
  const key = $derived(
    JSON.stringify([wire, mode, recommendationSources.rows.map((s) => s.contents), op.id]),
  );
  const stale = $derived(
    !!(op.appliedCutting?.proposal || op.appliedCutting?.knowledge) &&
      op.appliedContextKey !==
        cuttingContextKey(
          op,
          project.data.machine,
          project.data.tools,
          project.data.material,
          project.data.fsContext,
        ),
  );
  const previewStale = $derived(responses.length > 0 && responseKey !== key);
  const suggestions = $derived(
    responses.filter((s) => s.result.recommendation.status === 'available'),
  );
  const accepted = $derived(op.appliedCutting?.proposal);
  const savedKnowledge = $derived(op.appliedCutting?.knowledge);
  const hasRecommendation = $derived(!!accepted || !!savedKnowledge);
  const provenanceMissing = $derived(
    !!savedKnowledge &&
      ![...recommendationSources.builtins, ...recommendationSources.local].some(
        (p) =>
          p.manifest.identity.id === savedKnowledge.selected_pack.id &&
          (p.manifest.identity.revision === savedKnowledge.selected_pack.revision ||
            labHasRevision(p, savedKnowledge.selected_pack.revision)),
      ),
  );
  const materialLabel = $derived(
    [
      ...NORMALIZED_MATERIALS,
      ...recommendationSources.rows.flatMap((s) => s.catalog.materials),
    ].find(
      (m) =>
        m.identity.id === project.data.material?.id &&
        m.identity.revision === project.data.material?.revision,
    )?.label ??
      project.data.material?.label ??
      (project.data.material ? t('fs35.saved_material') : t('fs35.material_missing')),
  );
  const doc = $derived.by(() => {
    if (!op.depthList?.length) return Math.abs(op.step ?? tool?.defaultStep ?? 0);
    let previous = op.startDepth,
      maximum = 0;
    for (const depth of op.depthList) {
      maximum = Math.max(maximum, previous - depth);
      previous = depth;
    }
    return maximum;
  });
  const overlap = $derived(
    'xyOverlap' in op ? (op.xyOverlap ?? tool?.defaultXyOverlap ?? 0.5) : 0.5,
  );
  function editRate(
    toolId: number,
    pass: 'rough' | 'finish',
    field: 'rpm' | 'feed_mm_min' | 'plunge_mm_min',
    value: number,
  ) {
    try {
      project.updateOperation(op.id, {
        appliedCutting: editOperationRate(op, project.data.tools, toolId, pass, field, value),
        feedRateOverride: undefined,
        plungeRateOverride: undefined,
      });
      error = '';
    } catch {
      error = t('fs.positive');
    }
  }
  async function calculate() {
    if (!wire || !api.feedsSpeeds) {
      error = t('fs.unavailable_build');
      return;
    }
    const captured = key;
    busy = true;
    error = '';
    attempted = true;
    try {
      const contents = packCollection(recommendationSources.rows);
      const catalog: DatasetCatalog = {
        machines: recommendationSources.rows.flatMap((s) => s.catalog.machines),
        materials: recommendationSources.rows.flatMap((s) => s.catalog.materials),
        tools: recommendationSources.rows.flatMap((s) => s.catalog.tools),
        description: t('fs37.references'),
      };
      const next = [
        {
          id: 'cutting-knowledge',
          label: t('fs37.references'),
          contents,
          catalog,
          result: await api.feedsSpeeds!({
            project: wire,
            op_id: op.id,
            dataset_json: contents,
            aggressiveness: mode,
            apply: false,
          }),
        },
      ];
      if (captured === key) {
        responses = next;
        responseKey = captured;
      }
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  async function apply(source: (typeof suggestions)[number]) {
    if (!wire || !api.feedsSpeeds || previewStale) return;
    const captured = key;
    busy = true;
    error = '';
    try {
      const result = await api.feedsSpeeds({
        project: wire,
        op_id: op.id,
        dataset_json: source.contents,
        aggressiveness: mode,
        apply: true,
      });
      if (captured !== key || !result.applied_op?.params.applied_cutting) return;
      const native = result.applied_op;
      const patch: Partial<OpEntry> = {
        appliedCutting: native.params.applied_cutting!,
        step: native.params.step ?? op.step,
        feedRateOverride: undefined,
        plungeRateOverride: undefined,
      };
      if (op.kind === 'pocket' && native.kind.type === 'pocket')
        Object.assign(patch, { xyOverlap: native.kind.pocket.xy_overlap });
      patch.appliedContextKey = cuttingContextKey(
        { ...op, ...patch } as OpEntry,
        project.data.machine,
        project.data.tools,
        project.data.material,
        context,
      );
      project.history.exec(
        applyCuttingCommand(op.id, patch, project.data.material, context),
        project.target(),
      );
      review = false;
      responses = [];
      attempted = false;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  const fields = ['rpm', 'feed_mm_min', 'plunge_mm_min'] as const;
  function fieldName(field: (typeof fields)[number]) {
    return t(field === 'rpm' ? 'fs.rpm' : field === 'feed_mm_min' ? 'fs.feed' : 'fs.plunge');
  }
  function origin(field: (typeof fields)[number]) {
    return field === 'rpm' ? 'rpm' : field === 'feed_mm_min' ? 'feed' : 'plunge';
  }
  function rejectionReason(reason: S['FsUnavailableReason']) {
    switch (reason.kind) {
      case 'invalid_request':
        return t('fs36.reason.invalid_request');
      case 'dataset_mismatch':
        return t('fs36.reason.dataset_mismatch');
      case 'missing_material':
        return t('fs36.reason.missing_material');
      case 'unsupported_tool':
        return t('fs36.reason.unsupported_tool');
      case 'unsupported_operation':
        return t('fs36.reason.unsupported_operation');
      case 'missing_cutting_data':
        return t('fs36.reason.missing_cutting_data');
      case 'ambiguous_profiles':
        return t('fs36.reason.ambiguous_profiles');
      case 'unrepresentable_output':
        return t('fs36.reason.unrepresentable_output');
    }
  }
  function scopeField(field: string) {
    switch (field) {
      case 'mode':
        return t('fs.mode');
      case 'machine':
        return t('fs35.machine');
      case 'material':
        return t('fs.material');
      case 'tool':
        return t('fs35.tool');
      case 'operation':
        return t('fs35.operation');
      case 'doc_mm':
        return t('fs36.pass_depth');
      case 'thickness_mm':
        return t('stock.thickness');
      case 'cutting_family':
        return t('fs36.scope.cutting_family');
      case 'diameter_mm':
        return t('fs36.scope.diameter_mm');
      case 'flute_count':
        return t('fs36.scope.flute_count');
      case 'entry':
        return t('fs36.scope.entry');
      case 'engagement':
        return t('fs36.scope.engagement');
      default:
        return field;
    }
  }
  function evidence(value: S['FsEvidence']) {
    return t(
      value === 'reference_data'
        ? 'fs35.reference'
        : value === 'tested'
          ? 'fs35.tested'
          : value === 'validated'
            ? 'fs35.validated'
            : 'fs35.theoretical',
    );
  }
</script>

<section class="cutting-section" aria-label={t('fs35.cutting')}>
  <h3>{t('fs35.cutting')}</h3>
  {#if op.appliedCutting && (!main || (op.kind === 'pocket' && !secondary))}<p role="alert">
      {t('fs35.missing_rates')}
    </p>{/if}
  {#each fields as field (field)}
    <label class="inspector-field"
      ><span>{fieldName(field)}</span><input
        type="number"
        min="1"
        max="4294967295"
        step="1"
        value={main?.[field] ?? ''}
        oninput={(e) => editRate(op.toolId, mainPass, field, Number(e.currentTarget.value))}
      /><span class="unit">{field === 'rpm' ? 'rpm' : 'mm/min'}</span>
      <CuttingOriginStatus origin={main?.origins[origin(field)]} /></label
    >
  {/each}
  <label class="inspector-field"
    ><span>{t('fs36.pass_depth')}</span><input
      type="number"
      min="0.01"
      step="0.1"
      value={doc || ''}
      disabled={!!op.depthList?.length}
      oninput={(e) => {
        const n = Number(e.currentTarget.value);
        if (n > 0) project.updateOperation(op.id, { step: -n });
      }}
    /><span class="unit">mm</span><CuttingOriginStatus
      origin={op.appliedCutting?.doc_origin}
    /></label
  >
  {#if op.depthList?.length}<p>{t('fs35.schedule')}</p>{/if}
  {#if op.kind === 'pocket'}<label class="inspector-field"
      ><span>{t('fs.stepover')}</span><input
        type="number"
        min="1"
        max="99"
        step="1"
        value={((1 - overlap) * 100).toFixed(1)}
        oninput={(e) => {
          const n = Number(e.currentTarget.value);
          if (n > 0 && n < 100) project.updateOperation(op.id, { xyOverlap: 1 - n / 100 });
        }}
      /><span class="unit">%</span><CuttingOriginStatus
        origin={op.appliedCutting?.stepover_origin}
      /></label
    >{/if}
  {#if separateFinish && secondary}
    <details>
      <summary>{t('fs35.finishing')}</summary>
      <p>{finishTool?.name}</p>
      {#each fields as field (field)}<label class="inspector-field"
          ><span>{fieldName(field)}</span><input
            type="number"
            min="1"
            step="1"
            value={secondary[field]}
            oninput={(e) => editRate(finishToolId, 'finish', field, Number(e.currentTarget.value))}
          /><span class="unit">{field === 'rpm' ? 'rpm' : 'mm/min'}</span><CuttingOriginStatus
            origin={secondary.origins[origin(field)]}
          /></label
        >{/each}
    </details>
  {/if}
  <div class="assistant-state">
    {#if stale}
      <div class="stale" role="status" title={t('fs35.kept')}>
        <strong>⚠ {t('fs35.outdated')}</strong>
        <span>{t('fs36.kept')}</span>
      </div>
    {/if}
    {#if accepted && !savedKnowledge && !review}<p class="source-meta">
        {t('fs35.source')}: {accepted.provenance.profile.source.author} · {evidence(
          accepted.source_evidence,
        )}
      </p>{/if}
    {#if savedKnowledge && !review}<p class="source-meta">
        {knowledgeName(savedKnowledge.selected_pack.id)} · {savedKnowledge.compatible_sources}
        {savedKnowledge.compatible_sources === 1
          ? t('fs37.compatible_source')
          : t('fs37.compatible_sources')} · {evidence(savedKnowledge.evidence)}
      </p>{/if}
    {#if provenanceMissing}<p class="source-meta" role="status">
        {t('fs37.provenance_missing')}
      </p>{/if}
    {#if !review}<button
        type="button"
        class="inspector-action review-action"
        onclick={() => {
          review = true;
          error = '';
        }}
        >{stale
          ? t('fs35.recalculate')
          : hasRecommendation
            ? t('fs35.review')
            : t('fs35.suggest')}</button
      >{/if}
  </div>
  {#if review}
    <div
      class="recommendation"
      role="region"
      aria-label={hasRecommendation ? t('fs36.recalculate_parameters') : t('fs35.recommendation')}
    >
      <h4>{hasRecommendation ? t('fs36.recalculate_parameters') : t('fs35.recommendation')}</h4>
      <dl>
        <dt>{t('fs.material')}</dt>
        <dd>{materialLabel}</dd>
        {#if op.kind === 'pocket'}<dt>{t('fs36.engagement')}</dt>
          <dd>{((1 - overlap) * 100).toFixed(1)} % · {t('fs35.nominal')}</dd>{/if}
      </dl>
      {#if op.kind === 'profile'}<label class="check-row"
          ><input
            type="checkbox"
            checked={op.fsFullWidth ?? false}
            onchange={(e) =>
              project.updateOperation(op.id, { fsFullWidth: e.currentTarget.checked })}
          />{t('fs.full_width')}</label
        >{/if}
      <label class="inspector-field mode-field"
        ><span>{t('fs.mode')}</span><select bind:value={mode}
          ><option value="prudent">{t('fs.prudent')}</option><option value="normal"
            >{t('fs.normal')}</option
          ><option value="rapid">{t('fs.rapid')}</option></select
        ></label
      >
      {#if suggestions.length && !previewStale}
        {#each suggestions as source (source.id)}
          {#if source.result.recommendation.status === 'available'}
            {@const proposal = source.result.recommendation.recommendation}
            <p class="source-meta">
              {t('fs35.source')}: {knowledgeName(
                source.result.knowledge?.summary.selected_pack.id ?? '',
              )} · {source.result.knowledge?.summary.compatible_sources ?? 1}
              {(source.result.knowledge?.summary.compatible_sources ?? 1) === 1
                ? t('fs37.compatible_source')
                : t('fs37.compatible_sources')} · {evidence(proposal.source_evidence)}
            </p>
          {/if}
        {/each}
      {:else if savedKnowledge}
        <p class="source-meta">
          {t('fs36.previous_source')}: {savedKnowledge.selected_pack.id} · {evidence(
            savedKnowledge.evidence,
          )}
        </p>
      {:else if accepted}
        <p class="source-meta">
          {t('fs36.previous_source')}: {accepted.provenance.profile.source.author} · {evidence(
            accepted.source_evidence,
          )}
        </p>
      {:else if recommendationSources.rows.length}
        <p class="source-meta">
          {t('fs35.source')}: {t('fs37.references')} · {recommendationSources.rows.length}
          {recommendationSources.rows.length === 1
            ? t('fs37.enabled_source')
            : t('fs37.enabled_sources')} ·
          {t('fs36.evidence_pending')}
        </p>
      {/if}
      {#if recommendationSources.loading}<p role="status">
          {t('fs35.loading')}
        </p>{:else if recommendationSources.error}<p role="status">
          {t('fs35.source_unavailable')}
        </p>{:else if !recommendationSources.rows.length}<p role="status">
          {t('fs35.sources_disabled')}
        </p>{/if}
      {#if !project.data.material}<p role="status">{t('fs35.define_material')}</p>{/if}
      <button
        type="button"
        class="inspector-action"
        disabled={busy || !wire || !project.data.material || !recommendationSources.rows.length}
        onclick={calculate}>{hasRecommendation ? t('fs35.recalculate') : t('fs.propose')}</button
      >
      <details>
        <summary>{t('fs36.calculation_context')}</summary>
        <dl>
          <dt>{t('fs35.machine')}</dt>
          <dd>{project.data.machine.name}</dd>
          <dt>{t('fs35.tool')}</dt>
          <dd>{tool?.name} · Ø{tool?.diameter} mm · {tool?.flutes}F</dd>
          <dt>{t('fs35.operation')}</dt>
          <dd>{op.kind === 'profile' ? t('fs35.profile') : t('fs35.pocket')}</dd>
          <dt>{t('fs36.pass_depth')}</dt>
          <dd>{doc} mm</dd>
        </dl>
      </details>
      {#if previewStale}<p role="status">{t('fs35.preview_changed')}</p>{/if}
      {#if attempted && !busy && !suggestions.length && !error && !previewStale}
        <div class="no-match" role="status">
          {#each responses as source (source.id)}
            {#if source.result.recommendation.status === 'unavailable'}
              <p>{source.label}: {rejectionReason(source.result.recommendation.reason)}</p>
              {#if source.result.recommendation.reason.kind === 'invalid_request'}
                {#each source.result.recommendation.reason.issues as issue, index (index)}<p>
                    {issue.field}: {issue.message}
                  </p>{/each}
              {/if}
              {#if source.result.scope_rejections?.length}
                <p class="scope-heading">{t('fs36.closest_scopes')}</p>
                {#each source.result.scope_rejections as rejection, rejectionIndex (rejectionIndex)}
                  <ul class="scope-reasons">
                    {#each rejection.mismatches as mismatch (mismatch.field)}
                      <li>
                        <strong>{scopeField(mismatch.field)}</strong><br />{t('fs36.actual')}: {scopeValue(
                          mismatch.actual,
                          source.catalog,
                          t('fs36.unknown'),
                        )}{mismatch.field.endsWith('_mm') ? ' mm' : ''}<br />{t('fs36.required')}: {scopeValue(
                          mismatch.required,
                          source.catalog,
                          t('fs36.unknown'),
                        )}{mismatch.field.endsWith('_mm') ? ' mm' : ''}
                      </li>
                    {/each}
                  </ul>
                {/each}
              {/if}
            {/if}
          {/each}
          <p>{t('fs36.no_transfer')}</p>
        </div>
      {/if}
      {#each suggestions as source (source.id)}
        {#if source.result.recommendation.status === 'available'}
          {@const proposal = source.result.recommendation.recommendation}
          <article>
            <h4>{t('fs35.suggested')}</h4>
            {#if suggestions.length > 1}<p class="source-meta">{source.label}</p>{/if}
            <dl>
              <dt>{t('fs.rpm')}</dt>
              <dd>{proposal.parameters.rpm} rpm</dd>
              <dt>{t('fs.feed')}</dt>
              <dd>{proposal.parameters.feed_mm_min} mm/min</dd>
              <dt>{t('fs.plunge')}</dt>
              <dd>{proposal.parameters.plunge_mm_min} mm/min</dd>
              <dt>{t('fs36.pass_depth')}</dt>
              <dd>{proposal.parameters.doc_mm} mm</dd>
              {#if proposal.parameters.stepover_ratio != null}<dt>{t('fs.stepover')}</dt>
                <dd>{(proposal.parameters.stepover_ratio * 100).toFixed(1)} %</dd>{/if}
              <dt>{t('fs.chipload')}</dt>
              <dd>
                {proposal.parameters.effective_chipload_mm_tooth.toFixed(4)} mm/{t('fs.tooth')}
              </dd>
            </dl>
            {#if source.result.knowledge?.disagreement}<p role="status">
                {t('fs37.disagreement')}
              </p>{/if}
            {#if proposal.applicability === 'adjusted_outside_evidence'}<p role="status">
                {t('fs.adjusted')}
              </p>{/if}
            {#if proposal.diagnostics.length}<p>{t('fs.unknowns')}</p>{/if}
            <button
              type="button"
              class="inspector-action primary"
              disabled={busy || previewStale}
              onclick={() => apply(source)}>{t('fs35.apply')}</button
            >
            <details>
              <summary>{t('fs.source')}</summary>
              {#if source.result.knowledge}
                <p>{source.result.knowledge.selection_reason}</p>
                {#if source.result.knowledge.reference_chipload_range}<p>
                    {t('fs37.reference_range')}: {source.result.knowledge.reference_chipload_range.min.toFixed(
                      4,
                    )}–{source.result.knowledge.reference_chipload_range.max.toFixed(4)} mm/{t(
                      'fs.tooth',
                    )}
                  </p>{/if}
                {#if source.result.knowledge.disagreement}<p role="status">
                    {t('fs37.disagreement')}
                  </p>{/if}
                <pre>{JSON.stringify(source.result.knowledge.records, null, 2)}</pre>
              {/if}
              <p>{proposal.provenance.profile.source.permission_or_license}</p>
              <a
                href={sourceLink(proposal.provenance.profile.source.reference) ?? undefined}
                target="_blank"
                rel="noreferrer">{t('fs.source_link')}</a
              >
              <p>{proposal.provenance.profile.source.notes}</p>
              <pre>{JSON.stringify(proposal.provenance, null, 2)}</pre>
            </details>
          </article>
        {/if}
      {/each}
      <button
        type="button"
        class="inspector-action"
        onclick={() => {
          review = false;
          responses = [];
          attempted = false;
          error = '';
        }}>{t('common.cancel')}</button
      >
    </div>
  {/if}
  {#if accepted && (!review || !suggestions.length || previewStale)}<details>
      <summary>{t('fs.source')}</summary>
      <p>{accepted.provenance.profile.source.permission_or_license}</p>
      <a
        href={sourceLink(accepted.provenance.profile.source.reference) ?? undefined}
        target="_blank"
        rel="noreferrer">{t('fs.source_link')}</a
      >
      <p>{accepted.provenance.profile.source.notes}</p>
      <pre>{JSON.stringify(op.appliedCutting, null, 2)}</pre>
    </details>{/if}
  {#if savedKnowledge && !accepted}<details>
      <summary>{t('fs.source')}</summary>
      <p>{savedKnowledge.selected_pack.id} · {savedKnowledge.selected_pack.revision}</p>
      <p>{savedKnowledge.selected_record} · {evidence(savedKnowledge.evidence)}</p>
      <p>{savedKnowledge.policy}</p>
    </details>{/if}
  {#if error}<p role="alert">{error}</p>{/if}
  <button type="button" class="inspector-action" disabled={!wire || busy} onclick={openLabCapture}
    >{t('fs38.save_action')}</button
  >
</section>

{#if labCapture}<LabPresetDialog
    capture={labCapture}
    materialName={materialLabel}
    fingerprint={labFingerprint}
    currentFingerprint={currentLabFingerprint}
    onClose={() => (labCapture = null)}
  />{/if}

<style>
  .scope-reasons {
    margin: var(--ui-space-1) 0 var(--ui-space-2);
    padding-left: var(--ui-space-4);
    font-size: var(--inspector-font-field);
    overflow-wrap: anywhere;
  }
  .scope-reasons li {
    margin: var(--ui-space-2) 0;
  }
  .scope-heading {
    color: var(--text-muted);
  }
  .no-match {
    border-left: 2px solid var(--warn);
    padding-left: var(--ui-space-2);
    margin-top: var(--ui-space-2);
  }
  .cutting-section {
    border-top: 1px solid var(--border);
    padding-top: var(--ui-space-2);
    margin: var(--ui-space-3) 0;
  }
  h3,
  h4 {
    font-size: var(--inspector-font-meta);
    font-weight: 600;
    color: var(--text-muted);
    margin: 0 0 var(--ui-space-2);
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }
  p,
  dl,
  .check-row {
    font-size: var(--inspector-font-field);
    line-height: 1.35;
  }
  p {
    margin: var(--ui-space-1) 0;
  }
  .check-row {
    display: flex;
    align-items: center;
    gap: var(--ui-space-1);
    margin: var(--ui-space-2) 0;
  }
  .check-row input {
    width: auto;
    flex-shrink: 0;
  }
  dl {
    display: grid;
    grid-template-columns: minmax(5rem, 1fr) minmax(0, 1.5fr);
    gap: var(--ui-space-1) var(--ui-space-2);
    margin: var(--ui-space-2) 0;
  }
  dd {
    margin: 0;
    overflow-wrap: anywhere;
    font-variant-numeric: tabular-nums;
  }
  .inspector-field.mode-field {
    grid-template-columns: minmax(0, 1fr) minmax(5rem, 1fr);
  }
  .recommendation {
    border-top: 1px solid var(--border);
    padding-top: var(--ui-space-3);
    margin-top: var(--ui-space-3);
  }
  .assistant-state {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--ui-space-1) var(--ui-space-2);
    align-items: start;
    margin-top: var(--ui-space-2);
  }
  .stale {
    grid-column: 1;
    font-size: var(--inspector-font-meta);
  }
  .stale strong {
    display: block;
    color: var(--warn);
    font-weight: 500;
  }
  .stale span {
    display: block;
    color: var(--text-muted);
    margin-top: var(--ui-space-1);
  }
  .source-meta {
    grid-column: 1;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .review-action {
    grid-column: 2;
    grid-row: 1 / span 2;
  }
  pre {
    font-size: var(--inspector-font-meta);
    overflow: auto;
    max-height: 16rem;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  details {
    margin: var(--ui-space-2) 0;
  }
  summary {
    cursor: pointer;
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
    min-height: 24px;
    display: list-item;
  }
  article {
    border-top: 1px solid var(--border);
    margin: var(--ui-space-3) 0;
    padding-top: var(--ui-space-2);
  }
  @container cam-inspector (max-width: 260px) {
    .review-action {
      grid-column: 1 / -1;
      grid-row: auto;
      justify-self: start;
    }
  }
</style>
