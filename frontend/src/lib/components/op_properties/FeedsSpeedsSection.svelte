<script lang="ts">
  import { materialDisplay, materialOptions } from '../../state/material-presentation';
  import './op-properties.css';
  import { rejectionReason, scopeField, evidence } from '../../state/cutting-selector-labels';
  import PresetBrowser from '../PresetBrowser.svelte';
  import CuttingParameterComparison from '../CuttingParameterComparison.svelte';
  import {
    recordSelection,
    unavailableRecord,
    applyReviewedCutting,
    type SelectedRecord,
  } from '../../state/cutting-selector';
  import { freshCuttingSources } from '../../state/cutting-selector-sources';
  import type { PresetRow } from '../../state/preset-library';
  import PresetIdentityHelp from './PresetIdentityHelp.svelte';
  import { missingPresetIdentities } from '../../state/preset-identities';
  import { untrack, tick, onDestroy } from 'svelte';
  import CuttingSelectorFrame from '../CuttingSelectorFrame.svelte';
  import {
    PreviewRequests,
    displayNumber,
    distinctEngagement,
  } from '../../state/recommendation-preview';
  import { cuttingOriginPresentation } from '../../state/cutting-origin-presentation';
  import CuttingRecommendationDetails from './CuttingRecommendationDetails.svelte';
  import LabPresetDialog from '../LabPresetDialog.svelte';
  import { captureLabPreset } from '../../state/feeds-speeds-pack-store';
  import { isLabPackId, labHasRevision, type LabCapture } from '../../state/lab-presets';
  import { scopeValue } from '../../state/feeds-speeds-explanation';
  import CuttingOriginStatus from './CuttingOriginStatus.svelte';
  import { project, type OpEntry } from '../../state/project.svelte';
  import { applyCuttingCommand } from '../../state/commands';
  import { buildProject } from '../../api/build-project';
  import { cuttingKnowledgeClient } from '../../state/feeds-speeds-pack-store';
  import { packCollection } from '../../state/feeds-speeds-sources';
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
  let {
    op,
    initialSelection,
    initialAggressiveness = 'prudent',
    selectorOnly = false,
    onClose,
  }: {
    op: OpEntry;
    initialSelection?: SelectedRecord;
    initialAggressiveness?: S['FsAggressiveness'];
    selectorOnly?: boolean;
    onClose?: () => void;
  } = $props();
  let discovery = $state<'recommended' | 'browse'>(
    untrack(() => (initialSelection ? 'browse' : 'recommended')),
  );
  let selectedRecord = $state<SelectedRecord | undefined>(untrack(() => initialSelection));
  let selectedUnavailable = $state(false);
  let selectedMode = $state<S['FsAggressiveness']>(untrack(() => initialAggressiveness));
  let overwriteAcknowledged = $state(false);

  function selectPreset(row: PresetRow) {
    requests.cancel();
    responses = [];
    error = '';
    busy = false;
    attempted = false;
    selectedRecord = recordSelection(row);
    selectedMode = row.profile.aggressiveness;
    mode = selectedMode;
    selectedUnavailable = unavailableRecord(row);
    overwriteAcknowledged = false;
  }
  let labCapture = $state<LabCapture | null>(null);
  let labFingerprint = $state('');
  let showMissingIdentities = $state(false);
  async function openLabCapture() {
    if (!wire) return;
    showMissingIdentities = missingIdentities.length > 0;
    if (showMissingIdentities) {
      error = '';
      return;
    }
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
    return (
      recommendationSources.rows.find((row) => row.pack?.manifest.identity.id === id)?.pack
        ?.manifest.name ?? t('fs37.references')
    );
  }
  let reviewTrigger = $state<HTMLButtonElement>();
  const requests = new PreviewRequests();
  onDestroy(() => requests.cancel());
  let review = $state(untrack(() => selectorOnly));
  let fullWidth = $state(untrack(() => op.fsFullWidth ?? false));
  function cancelReview() {
    requests.cancel();
    review = false;
    responses = [];
    attempted = false;
    busy = false;
    error = '';
    onClose?.();
    void tick().then(() => reviewTrigger?.focus());
  }
  let mode = $state<S['FsAggressiveness']>(untrack(() => initialAggressiveness));
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
  let previewOperation: OpEntry | undefined;
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
  const hasManual = $derived(
    [
      main?.origins.rpm,
      main?.origins.feed,
      main?.origins.plunge,
      op.appliedCutting?.doc_origin,
      op.appliedCutting?.stepover_origin,
    ].includes('manual') ||
      op.feedRateOverride != null ||
      op.plungeRateOverride != null ||
      (!op.appliedCutting && (op.step != null || (op.kind === 'pocket' && op.xyOverlap != null))),
  );
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
  const missingIdentities = $derived(wire ? missingPresetIdentities(wire, op.toolId) : []);
  const currentLabFingerprint = $derived(JSON.stringify([wire, op.id, project.data.machine.name]));
  const key = $derived(
    JSON.stringify([
      wire,
      mode,
      recommendationSources.rows.map((s) => s.contents),
      op.id,
      discovery,
      selectedRecord,
      selectedUnavailable,
      project.data.settings.feedsSpeedsSources,
    ]),
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
  function sourceName(id: string): string {
    return (
      [...recommendationSources.builtins, ...recommendationSources.local].find(
        (p) => p.manifest.identity.id === id,
      )?.manifest.name ?? id
    );
  }
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
    project.data.material
      ? materialDisplay(
          project.data.material,
          materialOptions(recommendationSources.rows, project.data.material).find(
            (m) =>
              m.identity.id === project.data.material?.id &&
              m.identity.revision === project.data.material?.revision,
          )?.labels[0] ||
            project.data.material.label ||
            project.data.material.id,
          t,
        )
      : t('fs35.material_missing'),
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
      (discovery === 'recommended' || (!!selectedRecord && !selectedUnavailable)) &&
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
    if (discovery === 'browse' && (!selectedRecord || selectedUnavailable)) return;
    overwriteAcknowledged = false;
    if (!wire || !api.feedsSpeeds) {
      error = t('fs.unavailable_build');
      return;
    }
    const revision = requests.start();
    const operation = op;
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
            selected_record: discovery === 'browse' ? selectedRecord : undefined,
          }),
        },
      ];
      if (review && requests.owns(revision) && captured === key) {
        previewOperation = operation;
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
    if (!wire || !api.feedsSpeeds || previewStale || (hasManual && !overwriteAcknowledged)) return;
    if (previewOperation !== op || !project.data.operations.includes(op)) {
      error = t('preset3.stale');
      return;
    }
    const captured = key;
    const operation = op;
    const previewSources = [...recommendationSources.rows];
    const settings = project.data.settings.feedsSpeedsSources;
    const revision = requests.start();
    busy = true;
    error = '';
    try {
      await applyReviewedCutting({
        previewSources,
        readSources: () => freshCuttingSources(settings, recommendationSources.builtins),
        isCurrent: () =>
          review &&
          requests.owns(revision) &&
          captured === key &&
          operation === op &&
          project.data.operations.includes(operation),
        requiresAcknowledgement: hasManual,
        acknowledged: overwriteAcknowledged,
        validate: () =>
          api.feedsSpeeds!({
            project: wire!,
            op_id: op.id,
            dataset_json: source.contents,
            aggressiveness: mode,
            apply: true,
            selected_record: discovery === 'browse' ? selectedRecord : undefined,
          }),
        commit: (result) => {
          if (!result.applied_op?.params.applied_cutting)
            throw new Error(t('fs36.reason.missing_cutting_data'));
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
        },
      });
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
</script>

<section class="cutting-section" class:props={selectorOnly} aria-label={t('fs35.cutting')}>
  {#if !selectorOnly}
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
              oninput={(e) =>
                editRate(finishToolId, 'finish', field, Number(e.currentTarget.value))}
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
          class="inspector-action primary"
          bind:this={reviewTrigger}
          onclick={() => {
            fullWidth = op.fsFullWidth ?? false;
            discovery = 'recommended';
            selectedRecord = undefined;
            overwriteAcknowledged = false;
            review = true;
            error = '';
          }}>{t('preset3.choose')}</button
        >{/if}
    </div>
    {#if showMissingIdentities}<PresetIdentityHelp
        missing={missingIdentities}
        toolName={tool?.name ?? `#${op.toolId}`}
      />{/if}
    {#if stale}<div class="stale" role="status" title={t('fs35.kept')}>
        <strong>{t('fs35.outdated')}</strong><span>{t('fs36.kept')}</span>
      </div>{/if}
    {#if savedKnowledge}<p class="source-meta">
        {t(
          isLabPackId(savedKnowledge.selected_pack.id)
            ? 'v11.saved_source_lab'
            : 'v11.saved_source_pack',
          { name: sourceName(savedKnowledge.selected_pack.id) },
        )}
      </p>{/if}
    {#if provenanceMissing}<p class="source-meta" role="status">
        {t('fs37.provenance_missing')}
      </p>{/if}
  {/if}
  {#if review}
    <CuttingSelectorFrame
      onClose={cancelReview}
      context={`${op.kind === 'profile' ? t('fs35.profile') : t('fs35.pocket')} · Ø${tool ? displayNumber(tool.diameter, 3) : '—'} mm · ${tool?.flutes ?? '—'}F · ${materialLabel} · ${project.data.machine.name ?? ''}`}
      {busy}
    >
      <div class="discovery" aria-label={t('preset3.choose')}>
        <button
          type="button"
          class="inspector-action"
          title={t('fs35.suggest')}
          aria-pressed={discovery === 'recommended'}
          onclick={() => {
            discovery = 'recommended';
            responses = [];
            error = '';
          }}>{t('preset3.recommended')}</button
        >
        <button
          type="button"
          class="inspector-action"
          aria-pressed={discovery === 'browse'}
          onclick={() => {
            discovery = 'browse';
            mode = selectedMode;
            responses = [];
            error = '';
          }}>{t('preset3.browse')}</button
        >
      </div>
      {#if discovery === 'browse'}<PresetBrowser
          selected={selectedRecord}
          onselect={selectPreset}
          compatibility={busy
            ? 'checking'
            : previewStale || error || !attempted
              ? 'unchecked'
              : suggestions.length
                ? 'compatible'
                : 'incompatible'}
        />
        {#if selectedUnavailable}<p role="status">{t('preset3.unavailable')}</p>{/if}
      {/if}
      <p class="source-meta">{t('selector.job_material')}</p>
      {#if op.kind === 'profile'}<label class="check-row"
          ><input type="checkbox" bind:checked={fullWidth} />{t('fs.full_width')}</label
        >{/if}
      {#if discovery === 'recommended'}<label class="inspector-field mode-field"
          ><span>{t('fs.mode')}</span><select bind:value={mode}>
            <option value="prudent">{t('fs.prudent')}</option><option value="normal"
              >{t('fs.normal')}</option
            ><option value="rapid">{t('fs.rapid')}</option>
          </select></label
        >
      {/if}
      {#if busy}<p class="source-meta" role="status">{t('fs391.calculating')}</p>{/if}
      {#if recommendationSources.loading}<p role="status">{t('fs35.loading')}</p>
      {:else if recommendationSources.error}<p role="alert">{t('fs35.source_unavailable')}</p>
      {:else if !recommendationSources.rows.length}<p role="status">
          {t('fs35.sources_disabled')}
        </p>{/if}
      {#if !project.data.material}<p role="status">{t('selector.material_missing')}</p>{/if}
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
                {discovery === 'browse' ? t('preset3.similar') : ''}
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
              </details>
            {/if}
          {/each}
        </div>
      {/if}

      {#each suggestions as source (source.id)}
        {#if source.result.recommendation.status === 'available' && !previewStale}
          {#if discovery === 'browse'}<p role="status">{t('preset3.compatible')}</p>{/if}
          {@const proposal = source.result.recommendation.recommendation}
          {@const derived = proposal.provenance.derivation}
          <p class="source-meta" role="status">
            <strong
              >{t(
                isLabPackId(source.result.knowledge?.summary.selected_pack.id ?? '')
                  ? 'v11.source_lab'
                  : 'v11.source_pack',
                {
                  name: sourceName(
                    source.result.knowledge?.summary.selected_pack.id ??
                      proposal.provenance.dataset.id,
                  ),
                },
              )}</strong
            >
          </p>
          {#if isLabPackId(source.result.knowledge?.summary.selected_pack.id ?? '')}<p
              class="source-meta"
            >
              {t('v11.lab_priority')}
            </p>{/if}
          <CuttingParameterComparison
            pocket={op.kind === 'pocket'}
            current={[
              main?.rpm,
              main?.feed_mm_min,
              main?.plunge_mm_min,
              doc,
              op.kind === 'pocket' ? (1 - overlap) * 100 : undefined,
            ]}
            proposed={[
              proposal.parameters.rpm,
              proposal.parameters.feed_mm_min,
              proposal.parameters.plunge_mm_min,
              proposal.parameters.doc_mm,
              proposal.parameters.stepover_ratio == null
                ? undefined
                : proposal.parameters.stepover_ratio * 100,
            ]}
          />
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
                : evidence(proposal.source_evidence)} · {proposal.provenance.profile.source.author}
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
          <CuttingRecommendationDetails
            {proposal}
            knowledge={source.result.knowledge}
            {tool}
            machineName={project.data.machine.name ?? ''}
            {materialLabel}
            {doc}
            {overlap}
            {fullWidth}
            pocket={op.kind === 'pocket'}
            {evidence}
          />
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
        <button
          type="button"
          class="inspector-action"
          disabled={busy}
          onclick={async () => {
            await recommendationSources.refresh(true);
            await calculate();
          }}>{t('fs35.recalculate')}</button
        >{/if}
      {#if hasManual && suggestions.length}<label class="check-row"
          ><input type="checkbox" bind:checked={overwriteAcknowledged} />{t(
            'preset3.overwrite',
          )}</label
        >{/if}
      {#snippet footer()}
        <div class="recommendation-footer">
          <button type="button" class="inspector-action" onclick={cancelReview}
            >{t('common.cancel')}</button
          >
          <button
            type="button"
            class="inspector-action primary"
            disabled={busy ||
              previewStale ||
              !suggestions.length ||
              !!error ||
              (hasManual && !overwriteAcknowledged)}
            onclick={() => {
              if (suggestions[0]) void apply(suggestions[0]);
            }}>{t('selector.apply')}</button
          >
        </div>
      {/snippet}
    </CuttingSelectorFrame>
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
  .discovery {
    display: flex;
    gap: 4px;
    margin: var(--ui-space-2) 0;
  }
  .discovery [aria-pressed='true'] {
    border-color: var(--accent);
    background: var(--accent-tint);
  }

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
  h3 {
    font-size: var(--inspector-font-meta);
    font-weight: 600;
    color: var(--text-section);
    margin: 0 0 var(--ui-space-2);
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }
  p,
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
  .inspector-field.mode-field {
    grid-template-columns: minmax(0, 1fr) minmax(5rem, 1fr);
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
  .source-meta strong {
    display: block;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
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
  .extrapolated {
    color: var(--warn);
  }
  .caution-copy {
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  .recommendation-footer {
    display: flex;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
  }
</style>
