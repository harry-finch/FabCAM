<script lang="ts">
  import { untrack } from 'svelte';
  import Modal from './Modal.svelte';
  import MaterialPicker from './MaterialPicker.svelte';
  import LabPackCreate from './LabPackCreate.svelte';
  import { t } from '../i18n';
  import type { MsgKey } from '../i18n/keys';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import { recommendationSources } from '../state/feeds-speeds-sources.svelte';
  import { currentLocalPacks, isLabPackId } from '../state/lab-presets';
  import { persistLabPack } from '../state/lab-presets-store';
  import type { PresetRow } from '../state/preset-library';
  import type { MaterialIdentity, MachineSettings, ToolEntry } from '../state/project-types';
  import {
    adaptationProblems,
    adaptationDifferences,
    duplicatePresetAsDraft,
    localCopyAttestation,
  } from '../state/preset-adaptation';
  let { row, onClose }: { row: PresetRow; onClose: () => void } = $props();
  // These snapshots deliberately do not follow new source/destination revisions.
  const source = untrack(() => JSON.parse(JSON.stringify(row)) as PresetRow);
  const machine = JSON.parse(JSON.stringify(project.data.machine)) as MachineSettings;
  const machineFingerprint = JSON.stringify(project.data.machine);
  const candidates = [...project.data.tools, ...workspace.get().tool_inventory];
  const tools = candidates
    .filter(
      (tool, index) =>
        candidates.findIndex((t) => JSON.stringify(t) === JSON.stringify(tool)) === index,
    )
    .map((tool) => JSON.parse(JSON.stringify(tool)) as ToolEntry);
  const initialPacks = currentLocalPacks(recommendationSources.local).filter(
    (p) =>
      isLabPackId(p.manifest.identity.id) &&
      p.manifest.visibility === 'private' &&
      p.manifest.distribution === 'local_only',
  );
  let packs = $state(initialPacks);
  let destination = $state(initialPacks.length === 1 ? initialPacks[0].manifest.identity.id : '');
  let toolIndex = $state('');
  let material = $state<MaterialIdentity | undefined>(
    project.data.material ? { ...project.data.material } : undefined,
  );
  let name = $state(`${source.name} (copy)`),
    notes = $state(''),
    reviewed = $state(false),
    ownershipConfirmed = $state(false),
    busy = $state(false),
    error = $state('');
  const previous = $derived(packs.find((p) => p.manifest.identity.id === destination));
  const target = $derived({
    machine,
    tool: tools[Number(toolIndex)] && toolIndex !== '' ? tools[Number(toolIndex)] : undefined,
    material,
  });
  const problems = $derived(adaptationProblems(source, target, ownershipConfirmed));
  const differences = $derived(adaptationDifferences(source, target));
  const stale = $derived(machineFingerprint !== JSON.stringify(project.data.machine));
  const problemKeys: Record<string, MsgKey> = {
    identity: 'preset2.problem_identity',
    process: 'preset2.problem_process',
    machine: 'preset2.problem_machine',
    material: 'preset2.problem_material',
    tool: 'preset2.problem_tool',
    geometry: 'preset2.problem_geometry',
    rights: 'preset2.problem_rights',
    source: 'preset2.problem_source',
  };
  // Any edit invalidates the explicit review of the previous form values.
  const formFingerprint = $derived(
    JSON.stringify({ destination, toolIndex, material, name, notes, ownershipConfirmed }),
  );
  $effect(() => {
    void formFingerprint;
    reviewed = false;
  });
  async function save() {
    if (!previous || stale || problems.length || !reviewed || busy) return;
    busy = true;
    error = '';
    try {
      await recommendationSources.refresh(true);
      if (recommendationSources.error) throw new Error(recommendationSources.error);
      const fresh = [...recommendationSources.builtins, ...recommendationSources.local].find(
        (p) =>
          p.manifest.identity.id === source.pack.manifest.identity.id &&
          p.manifest.identity.revision === source.pack.manifest.identity.revision,
      );
      const enabled = recommendationSources.rows.some(
        (r) =>
          r.pack?.manifest.identity.id === source.pack.manifest.identity.id &&
          r.pack?.manifest.identity.revision === source.pack.manifest.identity.revision,
      );
      if (
        !fresh ||
        !enabled ||
        JSON.stringify(fresh) !== JSON.stringify(source.pack) ||
        machineFingerprint !== JSON.stringify(project.data.machine)
      )
        throw new Error(t('preset2.stale'));
      // A library change requires reopening; do not silently copy an outdated cutter.
      void workspace.version;
      if (
        !target.tool ||
        ![...project.data.tools, ...workspace.get().tool_inventory].some(
          (tool) => JSON.stringify(tool) === JSON.stringify(target.tool),
        )
      )
        throw new Error(t('preset2.stale'));
      const pack = duplicatePresetAsDraft(
        previous,
        source,
        target,
        { name, reviewed, ownershipConfirmed, notes },
        new Date().toISOString(),
        `preset-${crypto.randomUUID()}`,
      );
      await persistLabPack(pack, previous.manifest.identity.revision, {
        sourceGuard: recommendationSources.local.some(
          (p) => p.manifest.identity.id === source.pack.manifest.identity.id,
        )
          ? source.pack.manifest.identity
          : null,
      });
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
  ariaLabelledBy="preset-copy-title"
  width="720px"
  maxHeight="90vh"
>
  <header>
    <h2 id="preset-copy-title">{t('preset2.title')}</h2>
    <p>{source.name} · {source.pack.manifest.name} @{source.pack.manifest.identity.revision}</p>
  </header>
  <p class="notice">{t('preset2.help')}</p>
  <div class="copy-form">
    <fieldset disabled={busy}>
      <label
        >{t('v11.pack_choose')}<select bind:value={destination}
          ><option value="">{t('v11.pack_choose')}</option
          >{#each packs as pack (pack.manifest.identity.id)}<option
              value={pack.manifest.identity.id}>{pack.manifest.name}</option
            >{/each}</select
        ></label
      >
      <LabPackCreate
        onCreated={(pack) => {
          packs = [...packs, pack];
          destination = pack.manifest.identity.id;
        }}
      />
      <label>{t('fs38.name')}<input bind:value={name} maxlength="100" required /></label>
      <p>
        {t('preset2.machine')}:
        <strong>{machine.name || machine.fsLocalIdentity?.id || '—'}</strong>
      </p>
      <label
        >{t('presets.tool')}<select bind:value={toolIndex}
          ><option value="">{t('preset2.choose_tool')}</option
          >{#each tools as tool, index (index)}<option value={String(index)}
              >{tool.name} · Ø{tool.diameter} mm · {tool.flutes} {t('tools.col.flutes')}</option
            >{/each}</select
        ></label
      >
      <MaterialPicker value={material} context="preset" onchange={(value) => (material = value)} />
      <h3>{t('preset2.values')}</h3>
      <p class="values">
        {source.profile.data.rpm} rpm · {source.profile.data.kind === 'exact_values'
          ? `${source.profile.data.feed_mm_min} mm/min`
          : `${source.profile.data.chipload_mm_tooth} mm/tooth`} · {source.profile.data
          .plunge_mm_min} mm/min · DOC {source.profile.data.doc_mm} mm · {source.profile.data
          .stepover_ratio == null
          ? '—'
          : `${source.profile.data.stepover_ratio * 100}%`}
      </p>
      <h3>{t('preset2.differences')}</h3>
      <div class="table-scroll">
        <table>
          <thead
            ><tr
              ><th>{t('preset2.field')}</th><th>{t('preset2.before')}</th><th
                >{t('preset2.after')}</th
              ></tr
            ></thead
          ><tbody
            >{#each differences as difference (difference.field)}<tr
                ><th>{t(`preset2.field_${difference.field}` as MsgKey)}</th><td
                  >{difference.before}</td
                ><td>{difference.after}</td></tr
              >{/each}</tbody
          >
        </table>
      </div>
      {#if source.profile.reductions.length}<p class="notice">{t('preset2.reductions')}</p>{/if}
      {#if localCopyAttestation(source)}<label class="check"
          ><input type="checkbox" bind:checked={ownershipConfirmed} />{t(
            'preset2.ownership',
          )}</label
        >{/if}
      <label>{t('fs38.observations')}<textarea bind:value={notes} rows="2"></textarea></label>
      {#if problems.length}<ul class="problems">
          {#each problems as problem (problem)}<li>{t(problemKeys[problem])}</li>{/each}
        </ul>{/if}
      {#if stale}<p role="alert">{t('preset2.stale')}</p>{/if}
      <label class="check"
        ><input type="checkbox" bind:checked={reviewed} />{t('preset2.review')}</label
      >
    </fieldset>
    {#if error}<p role="alert">{error}</p>{/if}
    <footer>
      <button type="button" disabled={busy} onclick={onClose}>{t('common.cancel')}</button><button
        type="button"
        onclick={() => void save()}
        disabled={busy || stale || problems.length > 0 || !previous || !name.trim() || !reviewed}
        >{t('preset2.save')}</button
      >
    </footer>
  </div>
</Modal>

<style>
  header {
    border-bottom: 1px solid var(--border);
    padding-bottom: var(--ui-space-2);
  }
  h2 {
    margin: 0;
    font-size: 1rem;
  }
  h3 {
    font-size: var(--inspector-font-field);
    margin: var(--ui-space-3) 0 var(--ui-space-1);
  }
  p {
    margin: var(--ui-space-1) 0;
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .notice,
  .problems {
    background: var(--bg-elevated);
    padding: var(--ui-space-3);
    font-size: var(--inspector-font-meta);
  }
  .problems {
    padding-left: 2rem;
  }
  fieldset {
    border: 0;
    padding: 0;
    min-width: 0;
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
  textarea,
  button {
    color: var(--text);
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 6px;
    font: inherit;
    min-width: 0;
  }
  .check {
    display: flex;
    align-items: start;
    gap: var(--ui-space-2);
  }
  .check input {
    flex: none;
  }
  .table-scroll {
    overflow: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--inspector-font-meta);
  }
  th,
  td {
    text-align: left;
    border-bottom: 1px solid var(--border);
    padding: var(--ui-space-2);
    overflow-wrap: anywhere;
  }
  .values {
    font-variant-numeric: tabular-nums;
  }
  footer {
    display: flex;
    justify-content: end;
    gap: var(--ui-space-2);
    margin-top: var(--ui-space-3);
  }
  button {
    cursor: pointer;
  }
  button:disabled {
    opacity: 0.45;
    cursor: default;
  }
  [role='alert'] {
    color: var(--danger);
  }
</style>
