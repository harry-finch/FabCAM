<script lang="ts">
  import { t } from '../i18n';
  import Modal from './Modal.svelte';
  import { confirmStore } from '../state/confirm.svelte';
  import type { SourcePack } from '../state/feeds-speeds-sources';
  import { revisePreset, LAB_POLICY, type LabReview } from '../state/lab-presets';
  import { persistLabPack } from '../state/lab-presets-store';
  import { exportLocalSourcePack } from '../services/file_ops';
  let { pack }: { pack: SourcePack } = $props();
  let search = $state(''),
    selected = $state(''),
    showDeleted = $state(false),
    busy = $state(false),
    error = $state('');
  let reviewPack = $state<SourcePack | null>(null),
    reviewId = $state(''),
    reviewNotes = $state(''),
    reviewConfirmed = $state(false);
  const rows = $derived(
    pack.records.filter(
      (r) =>
        r.lab &&
        (showDeleted || !r.lab.deleted) &&
        `${r.lab.name} ${r.lab.state}`.toLowerCase().includes(search.toLowerCase()),
    ),
  );
  const record = $derived(pack.records.find((r) => r.profile.id === selected));
  const profile = $derived(
    pack.dataset.profile_set.profiles.find(
      (p) =>
        p.identity.id === record?.profile.id && p.identity.revision === record?.profile.revision,
    ),
  );
  let notes = $derived(record?.lab?.observations ?? '');
  async function change(id: string, changes: Parameters<typeof revisePreset>[3], base = pack) {
    busy = true;
    error = '';
    try {
      await persistLabPack(
        revisePreset(base, id, new Date().toISOString(), changes),
        base.manifest.identity.revision,
      );
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  async function remove() {
    const base = pack,
      id = selected,
      name = record?.lab?.name ?? id;
    if (
      await confirmStore.ask({
        title: t('fs38.delete'),
        body: `${name}. ${t('fs38.delete_help')}`,
        primaryLabel: t('fs38.delete'),
        cancelLabel: t('common.cancel'),
        danger: true,
      })
    )
      await change(id, { deleted: true }, base);
  }
  async function promote() {
    if (!reviewPack || !reviewConfirmed) return;
    const review: LabReview = {
      policy: LAB_POLICY,
      reviewed_at: new Date().toISOString(),
      notes: reviewNotes,
      acceptable_finish_and_behavior: true,
      no_overheating_chatter_or_overload: true,
      suitable_for_lab_use: true,
    };
    await change(reviewId, { review }, reviewPack);
    if (!error) reviewPack = null;
  }
  async function backup() {
    try {
      await exportLocalSourcePack(pack);
    } catch (e) {
      error = String(e);
    }
  }
  function catalogLabel(kind: 'machines' | 'materials' | 'tools', id: string) {
    return pack.dataset[kind].find((item) => item.identity.id === id)?.label || id;
  }
  function evidence(state: string) {
    return state === 'draft'
      ? t('fs38.draft')
      : state === 'validated'
        ? t('fs35.validated')
        : t('fs35.tested');
  }
</script>

<details class="manager">
  <summary
    >{t('fs38.manage')} · {pack.records.filter((r) => r.lab && !r.lab.deleted).length}</summary
  >
  <label>{t('fs38.search')}<input type="search" bind:value={search} /></label>
  <label class="check"
    ><input type="checkbox" bind:checked={showDeleted} />{t('fs38.show_deleted')}</label
  >
  <label
    >{t('fs38.preset')}<select bind:value={selected}
      ><option value="">{t('fs38.select_preset')}</option>{#each rows as r (r.profile.id)}<option
          value={r.profile.id}
          >{r.lab?.name} · {evidence(r.lab?.state ?? 'draft')}{r.lab?.deleted
            ? ` · ${t('fs38.deleted')}`
            : ''}</option
        >{/each}</select
    ></label
  >
  {#if record?.lab && profile}
    <p class="meta">{evidence(record.lab.state)} · {record.lab.updated_at.slice(0, 10)}</p>
    <details>
      <summary>{t('fs36.calculation_context')}</summary>
      <dl>
        <dt>{t('fs35.machine')}</dt>
        <dd>
          {profile.scope.machine.kind === 'exact'
            ? catalogLabel('machines', profile.scope.machine.identity.id)
            : ''}
        </dd>
        <dt>{t('fs.material')}</dt>
        <dd>
          {profile.scope.material.kind === 'exact'
            ? catalogLabel('materials', profile.scope.material.identity.id)
            : ''}
        </dd>
        <dt>{t('fs35.tool')}</dt>
        <dd>
          {catalogLabel('tools', profile.scope.tool?.id ?? '')} · Ø{profile.scope.diameter_mm.max} mm
          · {profile.scope.flute_count}F
        </dd>
        <dt>{t('fs35.operation')}</dt>
        <dd>
          {profile.scope.operation === 'pocket' ? t('fs35.pocket') : t('fs35.profile')} · {profile
            .scope.engagement.kind === 'full_width'
            ? t('fs38.full_width')
            : profile.scope.engagement.kind === 'nominal'
              ? `${t('fs35.nominal')} · ${profile.scope.engagement.ratio.max * 100}%`
              : t('fs38.unknown_engagement')}
        </dd>
        <dt>{t('fs36.pass_depth')}</dt>
        <dd>{profile.scope.doc_mm?.max} mm</dd>
        <dt>{t('fs38.actual_values')}</dt>
        <dd>
          {#if profile.data.kind === 'exact_values'}{profile.data.rpm} rpm · {profile.data
              .feed_mm_min} / {profile.data.plunge_mm_min} mm/min{/if}
        </dd>
      </dl>
      <p class="meta">{record.lab.finishing_context}</p>
    </details>
    <details>
      <summary>{t('fs38.details')}</summary>
      <p class="meta">
        {record.lab.tested_at ?? t('fs38.draft')} · {record.lab.operator} · {record.lab
          .job_reference}
      </p>
      <p class="meta">{record.lab.detailed_notes}</p>
    </details>
    <label>{t('fs38.observations')}<textarea rows="2" bind:value={notes}></textarea></label>
    <button
      class="inspector-action"
      disabled={busy || notes === record.lab.observations}
      onclick={() => change(selected, { notes })}>{t('fs38.save_notes')}</button
    >
    {#if record.lab.deleted}<button
        class="inspector-action"
        disabled={busy}
        onclick={() => change(selected, { deleted: false })}>{t('fs38.restore')}</button
      >
    {:else}
      <label class="check"
        ><input
          type="checkbox"
          checked={record.lab.enabled}
          disabled={busy || record.lab.state === 'draft'}
          onchange={(e) => change(selected, { enabled: e.currentTarget.checked })}
        />{t('fs38.use_preset')}</label
      >
      {#if record.lab.state === 'tested'}<button
          class="inspector-action"
          disabled={busy}
          onclick={() => {
            reviewPack = pack;
            reviewId = selected;
            reviewNotes = '';
            reviewConfirmed = false;
          }}>{t('fs38.promote')}</button
        >{/if}
      <button class="inspector-action" disabled={busy} onclick={remove}>{t('fs38.delete')}</button>
    {/if}
    {#if record.lab.review}<p class="meta">
        {record.lab.review.policy} · {record.lab.review.reviewed_at.slice(0, 10)}<br />{record.lab
          .review.notes}
      </p>{/if}
    <details>
      <summary>{t('fs38.history')} · {record.lab_history?.length ?? 0}</summary>
      <p class="meta">
        {record.profile.id} · {t('fs38.created')}: {record.lab.created_at.slice(0, 10)}
      </p>
      {#each record.lab_history ?? [] as h (h.profile.identity.revision)}<p class="meta">
          {h.profile.identity.revision} · {evidence(h.metadata.state)} · {h.metadata.updated_at}<br
          />{h.metadata.observations}{#if h.profile.data.kind === 'exact_values'}<br />{h.profile
              .data.rpm} rpm · {h.profile.data.feed_mm_min} / {h.profile.data.plunge_mm_min} mm/min{/if}
        </p>{/each}
    </details>
  {/if}
  <p class="meta">{t('fs38.backup_help')}</p>
  <button class="inspector-action" disabled={busy} onclick={backup}>{t('fs38.export')}</button>
  {#if error}<p role="alert">{error}</p>{/if}
</details>
{#if reviewPack}
  <Modal
    onClose={() => {
      if (!busy) reviewPack = null;
    }}
    width="min(450px, 95vw)"
    ariaLabelledBy="lab-review-title"
  >
    <div class="review">
      <h2 id="lab-review-title">{t('fs38.promote')}</h2>
      <p>{t('fs38.validation_policy')}</p>
      <label>{t('fs38.review_notes')}<textarea rows="3" bind:value={reviewNotes}></textarea></label>
      <label class="check"
        ><input type="checkbox" bind:checked={reviewConfirmed} />{t(
          'fs38.validation_confirm',
        )}</label
      >
      {#if error}<p role="alert">{error}</p>{/if}
      <footer>
        <button disabled={busy} onclick={() => (reviewPack = null)}>{t('common.cancel')}</button
        ><button
          class="primary"
          disabled={busy || !reviewConfirmed || !reviewNotes.trim()}
          onclick={promote}>{t('fs38.confirm_promotion')}</button
        >
      </footer>
    </div>
  </Modal>
{/if}

<style>
  .review {
    padding: var(--ui-space-3);
  }
  .manager {
    margin-top: var(--ui-space-2);
  }
  label {
    display: grid;
    gap: var(--ui-space-1);
    margin: var(--ui-space-2) 0;
    font-size: var(--inspector-font-field);
  }
  input,
  select,
  textarea {
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
    font: inherit;
  }
  .check {
    display: flex;
    align-items: center;
    gap: var(--ui-space-2);
  }
  .check input {
    width: auto;
  }
  summary,
  .meta {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  p {
    overflow-wrap: anywhere;
  }
  dl {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: var(--ui-space-1);
    font-size: var(--inspector-font-meta);
  }
  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }
  h2 {
    font-size: var(--inspector-font-field);
  }
  footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--ui-space-2);
    margin-top: var(--ui-space-3);
  }
  button {
    margin: var(--ui-space-1) var(--ui-space-1) var(--ui-space-1) 0;
  }
  p[role='alert'] {
    color: var(--danger);
  }
</style>
