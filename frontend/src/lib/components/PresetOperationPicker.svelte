<script lang="ts">
  import { untrack } from 'svelte';
  import Modal from './Modal.svelte';
  import FeedsSpeedsSection from './op_properties/FeedsSpeedsSection.svelte';
  import { project } from '../state/project.svelte';
  import { buildProject } from '../api/build-project';
  import { referenceContext } from '../state/feeds-speeds';
  import { cuttingKnowledgeClient } from '../state/feeds-speeds-pack-store';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { packCollection } from '../state/feeds-speeds-sources';
  import { recordSelection, unavailableRecord } from '../state/cutting-selector';
  import type { PresetRow } from '../state/preset-library';
  import { t } from '../i18n';
  let { row, onClose }: { row: PresetRow; onClose: () => void } = $props();
  const selected = untrack(() => recordSelection(row));
  let ids = $state<number[]>([]),
    busy = $state(true),
    error = $state(''),
    chosen = $state<number>();
  const operation = $derived(project.data.operations.find((o) => o.id === chosen));
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
      fsContext: referenceContext(project.data.machine, project.data.tools, project.data.fsContext),
    }),
  );
  const fingerprint = $derived(JSON.stringify([wire, recommendationSources.rows]));
  $effect(() => {
    void fingerprint;
    const snapshot = wire;
    let cancelled = false;
    untrack(() => {
      ids = [];
      busy = true;
      error = '';
      void (async () => {
        try {
          if (!snapshot || !selected || unavailableRecord(row)) return;
          const client = cuttingKnowledgeClient();
          const dataset = packCollection(recommendationSources.rows);
          const compatible: number[] = [];
          for (const op of snapshot.operations) {
            if (op.kind.type !== 'profile' && op.kind.type !== 'pocket') continue;
            const result = await client.feedsSpeeds!({
              project: snapshot,
              op_id: op.id,
              dataset_json: dataset,
              selected_record: selected,
              aggressiveness: row.profile.aggressiveness,
              apply: false,
            });
            if (cancelled) return;
            if (result.recommendation.status === 'available') compatible.push(op.id);
          }
          if (!cancelled) ids = compatible;
        } catch (e) {
          if (!cancelled) error = String(e);
        } finally {
          if (!cancelled) busy = false;
        }
      })();
    });
    return () => {
      cancelled = true;
    };
  });
</script>

{#if operation}
  <FeedsSpeedsSection
    op={operation}
    initialSelection={selected}
    initialAggressiveness={row.profile.aggressiveness}
    selectorOnly
    {onClose}
  />
{:else}
  <Modal {onClose} width="min(480px, 95vw)">
    <div class="props">
      <h3>{t('preset3.operation')}</h3>
      <p>{row.name}</p>
      {#if busy}<p role="status">{t('fs391.calculating')}</p>
      {:else if error}<p role="alert">{error}</p>
      {:else if !ids.length}<p role="status">{t('preset3.none')}</p>
      {:else}{#each project.data.operations.filter((o) => ids.includes(o.id)) as op (op.id)}
          <button type="button" class="inspector-action" onclick={() => (chosen = op.id)}
            >{op.name} · {op.kind}</button
          >
        {/each}{/if}
      <button type="button" class="inspector-action" onclick={onClose}>{t('common.cancel')}</button>
    </div>
  </Modal>
{/if}

<style>
  .props {
    font-size: var(--inspector-font-field);
    line-height: 1.35;
  }
  h3 {
    font-size: var(--inspector-font-meta);
    font-weight: 600;
    color: var(--text-section);
  }
</style>
