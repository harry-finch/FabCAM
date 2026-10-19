<script lang="ts">
  import { untrack, tick, onDestroy } from 'svelte';
  import Modal from '../Modal.svelte';
  import MaterialPicker from '../MaterialPicker.svelte';
  import type { MaterialIdentity } from '../../state/project-types';
  import { setMaterialCommand } from '../../state/commands';
  let draftMaterial = $state<MaterialIdentity | undefined>();
  const materialPending = $derived(
    JSON.stringify(draftMaterial) !== JSON.stringify(project.data.material),
  );
  import {
    PreviewRequests,
    displayNumber,
    distinctEngagement,
    adaptationKind,
  } from '../../state/recommendation-preview';
  import { cuttingOriginPresentation } from '../../state/cutting-origin-presentation';
  import RecommendationBasis from './RecommendationBasis.svelte';
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
    if (id === LAB_PACK_ID) return t('fs38.sorbonne');
    return (
      recommendationSources.rows.find((row) => row.pack?.manifest.identity.id === id)?.pack
        ?.manifest.publisher ?? t('fs37.references')
    );
  }
  let reviewTrigger = $state<HTMLButtonElement>();
  const requests = new PreviewRequests();
  onDestroy(() => requests.cancel());
  let review = $state(false);
  let fullWidth = $state(false);
  function cancelReview() {
    requests.cancel();
    review = false;
    responses = [];
    attempted = false;
    busy = false;
    error = '';
    void tick().then(() => reviewTrigger?.focus());
  }
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
      operations: project.data.operations.map((entry) =>
        entry.id === op.id && entry.kind === 'profile' && review
          ? { ...entry, fsFullWidth: fullWidth }
          : entry,
      ),
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
  const originView = $derived(
    cuttingOriginPresentation([
      main?.origins.rpm,
      main?.origins.feed,
      main?.origins.plunge,
      op.appliedCutting?.doc_origin,
      ...(op.kind === 'pocket' ? [op.appliedCutting?.stepover_origin] : []),
    ]),
  );
  const finishOriginView = $derived(
    cuttingOriginPresentation(secondary ? Object.values(secondary.origins) : []),
  );
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
  $effect(() => {
    const captured = key;
    if (
      review &&
      !materialPending &&
      wire &&
      project.data.material &&
      !recommendationSources.loading &&
      recommendationSources.rows.length
    ) {
      untrack(() => {
        void calculate(captured);
      });
    }
  });
  async function calculate(captured = key) {
    if (!wire || !api.feedsSpeeds) {
      error = t('fs.unavailable_build');
      return;
    }
    const revision = requests.start();
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
      if (review && requests.owns(revision) && captured === key) {
        responses = next;
        responseKey = captured;
      }
    } catch (e) {
      if (review && requests.owns(revision) && captured === key) error = String(e);
    } finally {
      if (requests.owns(revision)) busy = false;
    }
  }
  async function apply(source: (typeof suggestions)[number]) {
    if (!wire || !api.feedsSpeeds || previewStale || materialPending) return;
    const captured = key;
    const revision = requests.start();
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
      if (
        !review ||
        materialPending ||
        !requests.owns(revision) ||
        captured !== key ||
        !result.applied_op?.params.applied_cutting
      )
        return;
      const native = result.applied_op;
      const patch: Partial<OpEntry> = {
        appliedCutting: native.params.applied_cutting!,
        ...(op.kind === 'profile' ? { fsFullWidth: fullWidth } : {}),
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
      cancelReview();
    } catch (e) {
      if (review && requests.owns(revision) && captured === key) error = String(e);
    } finally {
      if (requests.owns(revision)) busy = false;
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
      case 'diameter_proximity':
        return t('fs39.relative_gap');
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
  {#if originView.shared}<p class="source-meta" role="status">
      {t(
        originView.shared === 'recommendation'
          ? 'fs391.applied_values'
          : originView.shared === 'manual'
            ? 'fs391.modified_values'
            : 'fs391.mixed_values',
      )}
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
      {#if originView.show(main?.origins[origin(field)])}<CuttingOriginStatus
          origin={main?.origins[origin(field)]}
        />{/if}</label
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
    /><span class="unit">mm</span
    >{#if originView.show(op.appliedCutting?.doc_origin)}<CuttingOriginStatus
        origin={op.appliedCutting?.doc_origin}
      />{/if}</label
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
      /><span class="unit">%</span
      >{#if originView.show(op.appliedCutting?.stepover_origin)}<CuttingOriginStatus
          origin={op.appliedCutting?.stepover_origin}
        />{/if}</label
    >{/if}
  {#if separateFinish && secondary}
    <details>
      <summary>{t('fs35.finishing')}</summary>
      <p>{finishTool?.name}</p>
      {#if finishOriginView.shared}<p class="source-meta">
          {t(
            finishOriginView.shared === 'recommendation'
              ? 'fs391.applied_values'
              : finishOriginView.shared === 'manual'
                ? 'fs391.modified_values'
                : 'fs391.mixed_values',
          )}
        </p>{/if}
      {#each fields as field (field)}<label class="inspector-field"
          ><span>{fieldName(field)}</span><input
            type="number"
            min="1"
            step="1"
            value={secondary[field]}
            oninput={(e) => editRate(finishToolId, 'finish', field, Number(e.currentTarget.value))}
          /><span class="unit">{field === 'rpm' ? 'rpm' : 'mm/min'}</span
          >{#if finishOriginView.show(secondary.origins[origin(field)])}<CuttingOriginStatus
              origin={secondary.origins[origin(field)]}
            />{/if}</label
        >{/each}
    </details>
  {/if}
  <div class="inspector-action-row">
    <button
      type="button"
      class="inspector-action"
      disabled={!wire || busy}
      onclick={openLabCapture}
      title={t('fs391.preset_help')}>{t('fs391.save_preset')}</button
    >
    {#if !review}<button
        type="button"
        class="inspector-action"
        bind:this={reviewTrigger}
        onclick={() => {
          fullWidth = op.fsFullWidth ?? false;
          draftMaterial = project.data.material ? { ...project.data.material } : undefined;
          review = true;
          error = '';
        }}>{t('fs391.review_recommendation')}</button
      >{/if}
  </div>
  {#if stale}<div class="stale" role="status" title={t('fs35.kept')}>
      <strong>{t('fs35.outdated')}</strong><span>{t('fs36.kept')}</span>
    </div>{/if}
  {#if provenanceMissing}<p class="source-meta" role="status">
      {t('fs37.provenance_missing')}
    </p>{/if}
  {#if review}
    <Modal
      onClose={cancelReview}
      width="min(440px, 95vw)"
      maxHeight="85vh"
      ariaLabelledBy="cutting-recommendation-title"
    >
      <div class="recommendation" aria-busy={busy}>
        <h4 id="cutting-recommendation-title">{t('fs35.recommendation')}</h4>
        <MaterialPicker
          value={draftMaterial}
          onchange={(value) => {
            draftMaterial = value;
            requests.cancel();
            responses = [];
            attempted = false;
            error = '';
            busy = false;
          }}
        />
        {#if materialPending}
          <p role="status">{t('uxc.material_draft')}</p>
          <button
            type="button"
            class="inspector-action"
            onclick={() =>
              project.history.exec(setMaterialCommand(draftMaterial), project.target())}
            >{t('uxc.material_apply')}</button
          >
        {/if}
        <p class="context-line">
          {t('fs35.saved_material')}: {materialLabel} · {op.kind === 'profile'
            ? t('fs35.profile')
            : t('fs35.pocket')} · Ø{tool ? displayNumber(tool.diameter, 3) : '—'} mm · {tool?.flutes ??
            '—'}F
        </p>
        {#if op.kind === 'profile'}<label class="check-row"
            ><input type="checkbox" bind:checked={fullWidth} />{t('fs.full_width')}</label
          >{/if}
        <label class="inspector-field mode-field"
          ><span>{t('fs.mode')}</span><select bind:value={mode}>
            <option value="prudent">{t('fs.prudent')}</option><option value="normal"
              >{t('fs.normal')}</option
            ><option value="rapid">{t('fs.rapid')}</option>
          </select></label
        >
        {#if busy}<p class="source-meta" role="status">{t('fs391.calculating')}</p>{/if}
        {#if recommendationSources.loading}<p role="status">{t('fs35.loading')}</p>
        {:else if recommendationSources.error}<p role="alert">{t('fs35.source_unavailable')}</p>
        {:else if !recommendationSources.rows.length}<p role="status">
            {t('fs35.sources_disabled')}
          </p>{/if}
        {#if !project.data.material}<p role="status">{t('uxc.choose_material_here')}</p>{/if}
        {#if attempted && !busy && !suggestions.length && !error && !previewStale}
          <div
            class="no-match"
            class:blocking={responses.some(
              (s) =>
                s.result.recommendation.status === 'unavailable' &&
                ['invalid_request', 'unrepresentable_output'].includes(
                  s.result.recommendation.reason.kind,
                ),
            )}
            role="alert"
          >
            {#each responses as source (source.id)}
              {#if source.result.recommendation.status === 'unavailable'}
                <p>
                  {source.result.recommendation.reason.kind === 'missing_cutting_data'
                    ? t('fs391.no_reference')
                    : rejectionReason(source.result.recommendation.reason)}
                </p>
                {#if source.result.recommendation.reason.kind === 'invalid_request'}
                  {#each source.result.recommendation.reason.issues as issue, index (index)}<p>
                      {issue.field}: {issue.message}
                    </p>{/each}
                {/if}
                <details>
                  <summary>{t('fs391.rejection_details')}</summary>
                  <p>{rejectionReason(source.result.recommendation.reason)}</p>
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
                            )}{mismatch.field.endsWith('_mm') ? ' mm' : ''}<br />{t(
                              'fs36.required',
                            )}: {scopeValue(
                              mismatch.required,
                              source.catalog,
                              t('fs36.unknown'),
                            )}{mismatch.field.endsWith('_mm') ? ' mm' : ''}
                          </li>
                        {/each}
                      </ul>
                    {/each}
                  {/if}
                </details>
              {/if}
            {/each}
          </div>
        {/if}

        {#each suggestions as source (source.id)}
          {#if source.result.recommendation.status === 'available' && !previewStale}
            {@const proposal = source.result.recommendation.recommendation}
            {@const derived = proposal.provenance.derivation}
            <dl class="suggested-values" aria-label={t('fs35.suggested')}>
              <dt>{t('fs.rpm')}</dt>
              <dd>{displayNumber(proposal.parameters.rpm)} <span>rpm</span></dd>
              <dt>{t('fs.feed')}</dt>
              <dd>{displayNumber(proposal.parameters.feed_mm_min)} <span>mm/min</span></dd>
              <dt>{t('fs.plunge')}</dt>
              <dd>{displayNumber(proposal.parameters.plunge_mm_min)} <span>mm/min</span></dd>
              <dt>{t('fs36.pass_depth')}</dt>
              <dd>{displayNumber(proposal.parameters.doc_mm, 2)} <span>mm</span></dd>
              {#if proposal.parameters.stepover_ratio != null}<dt>{t('fs.stepover')}</dt>
                <dd>
                  {displayNumber(proposal.parameters.stepover_ratio * 100, 1)} <span>%</span>
                </dd>{/if}
            </dl>
            {#if op.kind === 'pocket' && distinctEngagement(1 - overlap, proposal.parameters.stepover_ratio)}<p
                class="source-meta"
              >
                {t('fs36.engagement')}: {displayNumber((1 - overlap) * 100, 1)} % · {t(
                  'fs35.nominal',
                )}
              </p>{/if}
            <p class="source-meta" class:extrapolated={derived?.quality === 'extrapolated'}>
              {derived?.quality === 'extrapolated'
                ? t('fs39.extrapolated')
                : derived
                  ? t('fs391.adapted_reference')
                  : evidence(proposal.source_evidence)} · {proposal.provenance.profile.source
                .author}
            </p>
            {#if derived}<p class="source-meta">{evidence(proposal.source_evidence)}</p>{/if}
            {#if proposal.diagnostics.length || proposal.applicability === 'adjusted_outside_evidence'}<p
                class="caution-copy"
                role="status"
              >
                {t('fs391.review_limits')}
              </p>{/if}
            {#if source.result.knowledge?.disagreement}<p class="extrapolated" role="status">
                {t('fs37.disagreement')}
              </p>{/if}
            <details class="explanation">
              <summary>{t('fs39.why')}</summary>
              <dl>
                <dt>{t('fs35.machine')}</dt>
                <dd>{project.data.machine.name}</dd>
                <dt>{t('fs35.tool')}</dt>
                <dd>{tool?.name}</dd>
                <dt>{t('fs.material')}</dt>
                <dd>{materialLabel}</dd>
                <dt>{t('fs36.pass_depth')}</dt>
                <dd>{displayNumber(doc, 2)} mm · {t('fs391.requested')}</dd>
                <dt>{t('fs36.engagement')}</dt>
                <dd>
                  {op.kind === 'pocket'
                    ? `${displayNumber((1 - overlap) * 100, 1)} % · ${t('fs35.nominal')}`
                    : fullWidth
                      ? t('fs.full_width')
                      : t('fs36.unknown')}
                </dd>
                <dt>{t('fs.chipload')}</dt>
                <dd>
                  {displayNumber(proposal.parameters.effective_chipload_mm_tooth, 3)} mm/{t(
                    'fs.tooth',
                  )}
                </dd>
                <dt>{t('fs391.cutting_speed')}</dt>
                <dd>
                  {displayNumber(
                    (Math.PI * (tool?.diameter ?? 0) * proposal.parameters.rpm) / 1000,
                    1,
                  )} m/min
                </dd>
              </dl>
              {#if op.kind === 'pocket' && distinctEngagement(1 - overlap, proposal.parameters.stepover_ratio)}<p
                >
                  {t('fs391.engagement_distinct')}
                </p>{/if}
              {#if derived && adaptationKind(derived) === 'diameter'}<p>
                  {t('fs39.adapted_from')} Ø{derived.source_diameters_mm
                    .map((v) => displayNumber(v, 3))
                    .join(' / Ø')} → Ø{displayNumber(derived.requested_diameter_mm, 3)} mm
                </p>{/if}
              <RecommendationBasis knowledge={source.result.knowledge} detail={derived} embedded />
              {#if proposal.diagnostics.length}<p>{t('fs.unknowns')}</p>
                <ul>
                  {#each proposal.diagnostics as diagnostic, index (index)}<li>
                      {diagnostic.kind === 'unknown_limit'
                        ? `${t('fs391.unknown_limit')}: ${diagnostic.field}`
                        : t(`fs391.${diagnostic.kind}`)}
                      {#if diagnostic.kind === 'low_effective_chipload'}
                        {displayNumber(diagnostic.chipload_mm_tooth, 5)} / {displayNumber(
                          diagnostic.target_chipload_mm_tooth,
                          5,
                        )} mm/tooth
                      {/if}
                    </li>{/each}
                </ul>{/if}
              <dl>
                <dt>{t('fs391.known_limits')}</dt>
                <dd>
                  {proposal.provenance.request.machine.rpm_min ?? '—'}–{proposal.provenance.request
                    .machine.rpm_max ?? '—'} rpm ·
                  {proposal.provenance.request.machine.max_feed_mm_min ?? '—'} mm/min ({t(
                    'fs.feed',
                  )}) ·
                  {proposal.provenance.request.machine.max_plunge_mm_min ?? '—'} mm/min ({t(
                    'fs.plunge',
                  )})
                </dd>
              </dl>
            </details>
            <details>
              <summary>{t('fs.source')}</summary>
              <p>{proposal.provenance.dataset.id} · {proposal.provenance.dataset.revision}</p>
              <p>{proposal.provenance.profile.identity.id}</p>
              <p>
                {evidence(proposal.source_evidence)} · {proposal.provenance.profile.source
                  .permission_or_license}
              </p>
              <a
                href={sourceLink(proposal.provenance.profile.source.reference) ?? undefined}
                target="_blank"
                rel="noreferrer">{t('fs.source_link')}</a
              >
              <p>{proposal.provenance.profile.source.notes}</p>
              <pre>{JSON.stringify(
                  { provenance: proposal.provenance, knowledge: source.result.knowledge },
                  null,
                  2,
                )}</pre>
            </details>
          {/if}
        {/each}
        {#if (accepted || savedKnowledge) && (!suggestions.length || previewStale)}<details>
            <summary>{t('fs391.saved_recommendation')}</summary>
            {#if savedKnowledge}<p>
                {knowledgeName(savedKnowledge.selected_pack.id)} · {savedKnowledge.selected_pack
                  .revision} · {evidence(savedKnowledge.evidence)}
              </p>{/if}
            {#if accepted}<p>{accepted.provenance.profile.source.permission_or_license}</p>
              <a
                href={sourceLink(accepted.provenance.profile.source.reference) ?? undefined}
                target="_blank"
                rel="noreferrer">{t('fs.source_link')}</a
              >{/if}
            <pre>{JSON.stringify(op.appliedCutting, null, 2)}</pre>
          </details>{/if}
        {#if error}<p role="alert">{error}</p>
          <button type="button" class="inspector-action" disabled={busy} onclick={() => calculate()}
            >{t('fs35.recalculate')}</button
          >{/if}
        <footer class="recommendation-footer">
          <button type="button" class="inspector-action" onclick={cancelReview}
            >{t('common.cancel')}</button
          >
          <button
            type="button"
            class="inspector-action primary"
            disabled={busy || materialPending || previewStale || !suggestions.length || !!error}
            onclick={() => {
              if (suggestions[0]) void apply(suggestions[0]);
            }}>{t('fs35.apply')}</button
          >
        </footer>
      </div>
    </Modal>
  {/if}
  {#if error}<p role="alert">{error}</p>{/if}
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
  .no-match.blocking {
    border-left-color: var(--error);
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
    color: var(--text-section);
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
    padding: var(--ui-space-3);
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
  .context-line {
    color: var(--text-muted);
    margin-bottom: var(--ui-space-3);
  }
  .suggested-values {
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    padding: var(--ui-space-3) 0;
    gap: var(--ui-space-2);
  }
  .suggested-values dd {
    text-align: right;
  }
  .suggested-values dd span {
    color: var(--text-muted);
  }
  .extrapolated {
    color: var(--warn);
  }
  .caution-copy {
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .recommendation-footer {
    position: sticky;
    bottom: -12px;
    background: var(--bg-panel, var(--bg));
    display: flex;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
    padding: var(--ui-space-3) 0;
    border-top: 1px solid var(--border);
    margin-top: var(--ui-space-3);
  }
</style>
