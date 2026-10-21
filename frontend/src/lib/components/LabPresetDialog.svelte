<script lang="ts">
  import LabPackCreate from './LabPackCreate.svelte';
  import Modal from './Modal.svelte';
  import { t } from '../i18n';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import {
    currentLocalPacks,
    isLabPackId,
    closePresets,
    saveCapturedPreset,
    type LabCapture,
    type CaptureForm,
  } from '../state/lab-presets';
  import { persistLabPack } from '../state/lab-presets-store';
  let {
    capture,
    materialName,
    fingerprint,
    currentFingerprint,
    onClose,
  }: {
    capture: LabCapture;
    materialName: string;
    fingerprint: string;
    currentFingerprint: string;
    onClose: () => void;
  } = $props();
  // Pin inspected revisions: concurrent changes must fail closed on save.
  const initialPacks = currentLocalPacks(recommendationSources.local).filter((p) =>
    isLabPackId(p.manifest.identity.id),
  );
  let packs = $state(initialPacks);
  let destination = $state(initialPacks.length === 1 ? initialPacks[0].manifest.identity.id : '');
  const previous = $derived(packs.find((p) => p.manifest.identity.id === destination));
  const duplicates = $derived(closePresets(previous, capture));
  let updateId = $state('');
  function initialForm(): CaptureForm {
    return {
      name: `${materialName} · Ø${capture.request.tool.diameter_mm} · ${capture.request.operation.kind === 'pocket' ? t('fs35.pocket') : t('fs35.profile')}`,
      state: 'draft',
      tested_at: new Date().toISOString().slice(0, 10),
      operator: '',
      job_reference: '',
      observations: '',
      detailed_notes: '',
      physically_tested: false,
      no_critical_failure: false,
    };
  }
  let form = $state<CaptureForm>(initialForm());
  let error = $state(''),
    busy = $state(false);
  const stale = $derived(fingerprint !== currentFingerprint);
  async function save() {
    if (!previous) {
      error = t('v11.pack_choose');
      return;
    }
    if (stale) {
      error = t('fs38.stale');
      return;
    }
    busy = true;
    error = '';
    try {
      const pack = saveCapturedPreset(
        previous,
        capture,
        form,
        new Date().toISOString(),
        `preset-${crypto.randomUUID()}`,
        updateId || undefined,
      );
      await persistLabPack(pack, previous?.manifest.identity.revision ?? null);
      onClose();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<Modal
  onClose={() => {
    if (!busy) onClose();
  }}
  width="min(480px, 95vw)"
  ariaLabelledBy="lab-preset-title"
>
  <div class="capture">
    <header><h2 id="lab-preset-title">{t('fs38.save_title')}</h2></header>
    <label
      >{t('fs38.destination')}
      <select bind:value={destination} disabled={busy} onchange={() => (updateId = '')}>
        <option value="">{t('v11.pack_choose')}</option>
        {#each packs as pack (pack.manifest.identity.id)}<option value={pack.manifest.identity.id}
            >{pack.manifest.name}</option
          >{/each}
      </select>
    </label>
    <LabPackCreate
      onCreated={(pack) => {
        packs = [...packs, pack];
        destination = pack.manifest.identity.id;
        updateId = '';
      }}
    />
    <form
      onsubmit={(e) => {
        e.preventDefault();
        void save();
      }}
    >
      <label>{t('fs38.name')}<input bind:value={form.name} required /></label>
      <label
        >{t('fs38.evidence')}<select bind:value={form.state}
          ><option value="tested">{t('fs35.tested')}</option><option value="draft"
            >{t('fs38.draft')}</option
          ></select
        ></label
      >
      {#if form.state === 'tested'}
        <label
          >{t('fs38.test_date')}<input
            type="date"
            bind:value={form.tested_at}
            max={new Date().toISOString().slice(0, 10)}
            required
          /></label
        >
        <label class="check"
          ><input type="checkbox" bind:checked={form.physically_tested} />{t(
            'fs38.physical_confirm',
          )}</label
        >
        <label class="check"
          ><input type="checkbox" bind:checked={form.no_critical_failure} />{t(
            'fs38.success_confirm',
          )}</label
        >
      {:else}<p class="meta">{t('fs38.draft_help')}</p>{/if}
      <label
        >{t('fs38.observations')}<textarea rows="2" bind:value={form.observations}
        ></textarea></label
      >
      {#if duplicates.length}
        <label
          >{t('fs38.duplicate')}<select bind:value={updateId}
            ><option value="">{t('fs38.save_new')}</option
            >{#each duplicates as d (d.profile.id)}<option value={d.profile.id}
                >{t('fs38.update_existing')}: {d.lab?.name} · {d.lab?.state}</option
              >{/each}</select
          ></label
        >
        <p class="meta">{t('fs38.revision_help')}</p>
      {/if}
      <details>
        <summary>{t('fs38.details')}</summary>
        <label>{t('fs38.operator')}<input bind:value={form.operator} /></label>
        <label>{t('fs38.job_reference')}<input bind:value={form.job_reference} /></label>
        <label
          >{t('fs38.detailed_notes')}<textarea rows="3" bind:value={form.detailed_notes}
          ></textarea></label
        >
        <p class="meta">{capture.finishing_context}</p>
      </details>
      <details>
        <summary>{t('fs36.calculation_context')}</summary>
        <dl>
          <dt>{t('fs35.machine')}</dt>
          <dd>{capture.machine_name || capture.request.machine.identity?.id}</dd>
          <dt>{t('fs35.tool')}</dt>
          <dd>{capture.tool_name}</dd>
          <dt>{t('fs35.operation')}</dt>
          <dd>
            {capture.request.operation.kind === 'pocket' ? t('fs35.pocket') : t('fs35.profile')}
          </dd>
          <dt>{t('fs36.pass_depth')}</dt>
          <dd>{capture.request.operation.doc_mm} mm</dd>
          <dt>{t('fs38.engagement')}</dt>
          <dd>
            {capture.request.operation.engagement.kind === 'known_full_width'
              ? t('fs38.full_width')
              : capture.request.operation.engagement.kind === 'nominal'
                ? t('fs35.nominal')
                : t('fs38.unknown_engagement')}{capture.request.operation.engagement.kind ===
            'nominal'
              ? ` · ${capture.request.operation.engagement.ratio * 100}%`
              : ''}
          </dd>
          <dt>{t('fs38.actual_values')}</dt>
          <dd>{capture.rpm} rpm · {capture.feed_mm_min} mm/min · {capture.plunge_mm_min} mm/min</dd>
        </dl>
      </details>
      {#if stale}<p role="alert">{t('fs38.stale')}</p>{/if}
      {#if error}<p role="alert">{error}</p>{/if}
      <footer>
        <button type="button" disabled={busy} onclick={onClose}>{t('common.cancel')}</button><button
          class="primary"
          type="submit"
          disabled={busy || stale || !previous}>{t('fs38.save')}</button
        >
      </footer>
    </form>
  </div>
</Modal>

<style>
  .capture {
    padding: var(--ui-space-3);
  }
  h2 {
    font-size: var(--inspector-font-field);
    margin: 0 0 var(--ui-space-3);
  }
  form {
    display: grid;
    gap: var(--ui-space-2);
  }
  label {
    display: grid;
    gap: var(--ui-space-1);
    font-size: var(--inspector-font-field);
  }
  input,
  select,
  textarea {
    min-width: 0;
    width: 100%;
    box-sizing: border-box;
    font: inherit;
  }
  .check {
    display: flex;
    align-items: start;
    gap: var(--ui-space-2);
  }
  .check input {
    width: auto;
    margin-top: 2px;
  }
  .meta,
  summary {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  details {
    padding-top: var(--ui-space-2);
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
  footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--ui-space-2);
    margin-top: var(--ui-space-3);
  }
  p[role='alert'] {
    font-size: var(--inspector-font-field);
    color: var(--danger);
  }
</style>
