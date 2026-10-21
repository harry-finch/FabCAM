<script lang="ts">
  import type { components } from '../../api/generated';
  import type { ToolEntry } from '../../state/project-types';
  import { t } from '../../i18n';
  import { sourceLink } from '../../state/feeds-speeds';
  import {
    displayNumber,
    distinctEngagement,
    adaptationKind,
  } from '../../state/recommendation-preview';
  import RecommendationBasis from './RecommendationBasis.svelte';
  type S = components['schemas'];
  let {
    proposal,
    knowledge,
    tool,
    machineName,
    materialLabel,
    doc,
    overlap,
    fullWidth,
    pocket,
    evidence,
  }: {
    proposal: S['FsRecommendation'];
    knowledge?: S['FsKnowledgeExplanation'] | null;
    tool?: ToolEntry;
    machineName: string;
    materialLabel: string;
    doc: number;
    overlap: number;
    fullWidth: boolean;
    pocket: boolean;
    evidence: (value: S['FsEvidence']) => string;
  } = $props();
  const derived = $derived(proposal.provenance.derivation);
</script>

<details class="explanation">
  <summary>{t('fs39.why')}</summary>
  <dl>
    <dt>{t('fs35.machine')}</dt>
    <dd>{machineName}</dd>
    <dt>{t('fs35.tool')}</dt>
    <dd>{tool?.name}</dd>
    <dt>{t('fs.material')}</dt>
    <dd>{materialLabel}</dd>
    <dt>{t('fs36.pass_depth')}</dt>
    <dd>{displayNumber(doc, 2)} mm · {t('fs391.requested')}</dd>
    <dt>{t('fs36.engagement')}</dt>
    <dd>
      {pocket
        ? `${displayNumber((1 - overlap) * 100, 1)} % · ${t('fs35.nominal')}`
        : fullWidth
          ? t('fs.full_width')
          : t('fs36.unknown')}
    </dd>
    <dt>{t('fs.chipload')}</dt>
    <dd>
      {displayNumber(proposal.parameters.effective_chipload_mm_tooth, 3)} mm/{t('fs.tooth')}
    </dd>
    <dt>{t('fs391.cutting_speed')}</dt>
    <dd>
      {displayNumber((Math.PI * (tool?.diameter ?? 0) * proposal.parameters.rpm) / 1000, 1)} m/min
    </dd>
  </dl>
  {#if pocket && distinctEngagement(1 - overlap, proposal.parameters.stepover_ratio)}<p>
      {t('fs391.engagement_distinct')}
    </p>{/if}
  {#if derived && adaptationKind(derived) === 'diameter'}<p>
      {t('fs39.adapted_from')} Ø{derived.source_diameters_mm
        .map((v) => displayNumber(v, 3))
        .join(' / Ø')} → Ø{displayNumber(derived.requested_diameter_mm, 3)} mm
    </p>{/if}
  <RecommendationBasis {knowledge} detail={derived} embedded />
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
      {proposal.provenance.request.machine.rpm_min ?? '—'}–{proposal.provenance.request.machine
        .rpm_max ?? '—'} rpm ·
      {proposal.provenance.request.machine.max_feed_mm_min ?? '—'} mm/min ({t('fs.feed')}) ·
      {proposal.provenance.request.machine.max_plunge_mm_min ?? '—'} mm/min ({t('fs.plunge')})
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
  <pre>{JSON.stringify({ provenance: proposal.provenance, knowledge: knowledge }, null, 2)}</pre>
</details>

<style>
  details {
    margin: 8px 0;
  }
  summary {
    cursor: pointer;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
    min-height: 24px;
  }
  dl {
    display: grid;
    grid-template-columns: minmax(100px, 1fr) 2fr;
    gap: 4px 8px;
  }
  dt {
    color: var(--text-muted);
  }
  dd {
    margin: 0;
  }
  pre {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    max-height: 200px;
    overflow: auto;
    font-size: 11px;
  }
</style>
