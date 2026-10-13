<script lang="ts">
  import type { components } from '../../api/generated';
  import { t } from '../../i18n';
  import { adaptationKind, displayNumber } from '../../state/recommendation-preview';
  type S = components['schemas'];
  let {
    knowledge,
    summary,
    detail,
    embedded = false,
  }: {
    knowledge?: S['FsKnowledgeExplanation'] | null;
    summary?: S['FsKnowledgeSummary'] | null;
    detail?: S['FsDerivation'] | null;
    embedded?: boolean;
  } = $props();
  const record = $derived(
    knowledge?.records.find(
      (r) =>
        r.record_id === knowledge?.summary.selected_record &&
        r.pack.id === knowledge?.summary.selected_pack.id,
    ),
  );
  const derivation = $derived(
    knowledge?.records.find(
      (r) =>
        r.record_id === knowledge?.summary.selected_record &&
        r.pack.id === knowledge?.summary.selected_pack.id,
    )?.derivation ?? detail,
  );
  const quality = $derived(
    knowledge?.summary.match_quality ?? summary?.match_quality ?? detail?.quality ?? null,
  );
  function qualityLabel(value: S['FsMatchQuality']) {
    switch (value) {
      case 'close':
        return t('fs39.close');
      case 'approximate':
        return t('fs39.approximate');
      case 'extrapolated':
        return t('fs39.extrapolated');
      case 'interpolated':
        return t('fs39.interpolated');
      case 'generic':
        return t('fs39.generic');
      case 'theoretical':
        return t('fs39.theoretical');
      case 'exact':
        return t('fs39.exact');
      default:
        return t('fs36.unknown');
    }
  }
</script>

{#snippet explanation()}
  {#if record?.origin.material_taxon && record.rank[1] > 0}<p>
      {t('fs39.generic')}: {record.origin.material_taxon.replaceAll('.', ' / ')}
    </p>{/if}
  {#if derivation}
    <dl>
      <dt>{t('fs35.tool')}</dt>
      <dd>
        Ø{derivation.requested_diameter_mm} mm
        {#if derivation.interpolation_weight == null}
          · {t('fs39.relative_gap')}: {(derivation.relative_delta * 100).toFixed(1)} %{/if}
      </dd>
      <dt>{t('fs35.operation')}</dt>
      <dd>
        {derivation.slot_basis_for_pocket ? t('fs39.slot_basis') : t('fs39.same_operation')}
      </dd>
      <dt>{t('fs35.machine')}</dt>
      <dd>
        {derivation.machine_transfer ? t('fs39.router_transfer') : t('fs39.same_machine')}
      </dd>
      <dt>{t('fs.material')}</dt>
      <dd>
        {derivation.material_transfer ? t('fs39.material_transfer') : t('fs39.same_material')}
      </dd>
      <dt>{t('fs39.method')}</dt>
      <dd>
        {derivation.method === 'normalized_linear_interpolation'
          ? t('fs39.interpolation_method')
          : t('fs39.normalized_method')}
      </dd>
    </dl>
    {#if derivation.interpolation_weight != null}<p>
        {t('fs39.weight')}: {derivation.interpolation_weight.toFixed(3)}
      </p>{/if}
    <p>{t('fs39.reference_only')}</p>
  {:else if quality}
    <p>{qualityLabel(quality)}</p>
  {/if}
{/snippet}
{#if embedded}
  <div class="explanation-body">{@render explanation()}</div>
{:else if quality}
  <p class="basis" class:caution={quality === 'extrapolated'} role="status">
    {qualityLabel(quality)}
    {#if derivation && adaptationKind(derivation) === 'diameter'}
      · {t('fs39.adapted_from')} Ø{derivation.source_diameters_mm
        .map((v) => displayNumber(v, 3))
        .join(' / Ø')} → Ø{displayNumber(derivation.requested_diameter_mm, 3)} mm
    {:else if derivation?.slot_basis_for_pocket}
      · {t('fs391.slot_basis')}
    {/if}
  </p>
  {#if knowledge || detail}<details>
      <summary>{t('fs39.why')}</summary>{@render explanation()}
    </details>{/if}
{/if}

<style>
  .explanation-body {
    font-size: var(--inspector-font-field);
  }
  .basis {
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
    margin: var(--ui-space-2) 0;
  }
  .caution {
    color: var(--warn);
  }
  details {
    margin: var(--ui-space-2) 0;
    font-size: var(--inspector-font-meta);
  }
  summary {
    cursor: pointer;
    color: var(--text-muted);
  }
  dl {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 2fr);
    gap: var(--ui-space-1);
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }
</style>
