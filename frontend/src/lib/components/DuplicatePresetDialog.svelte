<script lang="ts">
  import { untrack } from 'svelte';
  import { suggestMachineName } from '../state/tool_naming';
  import { materialPickerLabel } from '../state/material-picker-presentation';
  import RecipeValuesEditor from './RecipeValuesEditor.svelte';
  import { recipeInputs, recipeData } from '../state/recipe-values';
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
    adaptationWarnings,
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
  let inputs = $state(recipeInputs(source.profile.data));
  const editedData = $derived(recipeData(source.profile.data, inputs));
  const previous = $derived(packs.find((p) => p.manifest.identity.id === destination));
  const target = $derived({
    machine,
    tool: tools[Number(toolIndex)] && toolIndex !== '' ? tools[Number(toolIndex)] : undefined,
    material,
  });
  const warnings = $derived(adaptationWarnings(source, target));
  const problems = $derived(
    editedData ? adaptationProblems(source, target, ownershipConfirmed, editedData) : ['values'],
  );
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
    values: 'preset2.problem_values',
    limits: 'preset2.problem_limits',
  };
  // Any edit invalidates the explicit review of the previous form values.
  const formFingerprint = $derived(
    JSON.stringify({ destination, toolIndex, material, name, notes, ownershipConfirmed, inputs }),
  );
  $effect(() => {
    void formFingerprint;
    reviewed = false;
  });
  // Presentation only: exact values remain in tooltips and the identity disclosure.
  function differenceLabel(field: string, side: 'before' | 'after', exact: string): string {
    if (field === 'diameter' || field === 'flutes' || exact === '—') return exact;
    if (side === 'after') {
      if (field === 'machine') return machine.name?.trim() || suggestMachineName(machine);
      if (field === 'tool') return target.tool?.name || '—';
      if (material) return materialPickerLabel(material, material.label || '', t);
    }
    const entries =
      field === 'machine'
        ? source.pack.dataset.machines
        : field === 'tool'
          ? source.pack.dataset.tools
          : source.pack.dataset.materials;
    const entry = entries.find((item) => `${item.identity.id}@${item.identity.revision}` === exact);
    if (field === 'material' && entry) return materialPickerLabel(entry.identity, entry.label, t);
    return entry && entry.label !== entry.identity.id
      ? entry.label
      : t(`preset2.field_${field}` as MsgKey);
  }
  async function save() {
    if (!previous || !editedData || stale || problems.length || !reviewed || busy) return;
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
        { name, reviewed, ownershipConfirmed, notes, data: editedData },
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
  <div class="copy-dialog" aria-busy={busy}>
    <header>
      <h2 id="preset-copy-title">{t('preset2.title')}</h2>
      <p
        class="source"
        title={`${source.name} · ${source.pack.manifest.identity.id}@${source.pack.manifest.identity.revision}`}
      >
        {source.name} · {source.pack.manifest.name}
      </p>
    </header>
    <div class="copy-content">
      <p class="notice">{t('preset2.help')}</p>
      <fieldset disabled={busy}>
        <label class="field-row"
          ><span>{t('v11.pack_choose')}</span><select bind:value={destination}
            ><option value="">{t('v11.pack_choose')}</option
            >{#each packs as pack (pack.manifest.identity.id)}<option
                value={pack.manifest.identity.id}>{pack.manifest.name}</option
              >{/each}</select
          ></label
        >
        <div class="pack-create">
          <LabPackCreate
            onCreated={(pack) => {
              packs = [...packs, pack];
              destination = pack.manifest.identity.id;
            }}
          />
        </div>
        <label class="field-row"
          ><span>{t('fs38.name')}</span><input bind:value={name} maxlength="100" required /></label
        >
        <div
          class="field-row machine"
          title={machine.fsLocalIdentity
            ? `${machine.fsLocalIdentity.id}@${machine.fsLocalIdentity.revision}`
            : undefined}
        >
          <span title={t('preset2.machine')}>{t('preset2.field_machine')}</span><span
            >{machine.name?.trim() || suggestMachineName(machine)}</span
          >
        </div>
        <label class="field-row"
          ><span>{t('presets.tool')}</span><select bind:value={toolIndex}
            ><option value="">{t('preset2.choose_tool')}</option
            >{#each tools as tool, index (index)}<option value={String(index)}
                >{tool.name} · Ø{tool.diameter} mm · {tool.flutes} {t('tools.col.flutes')}</option
              >{/each}</select
          ></label
        >
        <MaterialPicker
          value={material}
          context="preset"
          onchange={(value) => (material = value)}
        />
        <h3>{t('preset2.values')}</h3>
        <RecipeValuesEditor source={source.profile.data} bind:inputs />
        <p>{t('preset2.edit_help')}</p>
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
                    title={difference.before}
                    >{differenceLabel(difference.field, 'before', difference.before)}</td
                  ><td title={difference.after}
                    >{differenceLabel(difference.field, 'after', difference.after)}</td
                  ></tr
                >{/each}</tbody
            >
          </table>
        </div>
        <details class="identities">
          <summary>{t('preset2.identities')}</summary>
          <dl>
            {#each differences as difference (difference.field)}
              <dt>{t(`preset2.field_${difference.field}` as MsgKey)}</dt>
              <dd>{difference.before} → {difference.after}</dd>
            {/each}
          </dl>
        </details>
        {#if source.profile.reductions.length}<p class="notice">{t('preset2.reductions')}</p>{/if}
        {#if localCopyAttestation(source)}<label class="check"
            ><input type="checkbox" bind:checked={ownershipConfirmed} />{t(
              'preset2.ownership',
            )}</label
          >{/if}
        <label class="field-row observations"
          ><span>{t('fs38.observations')}</span><textarea bind:value={notes} rows="2"
          ></textarea></label
        >
        {#if warnings.length}<div class="warnings">
            <p>{t('preset2.context_warning')}</p>
            <ul>
              {#each warnings as warning (warning)}<li>
                  {t(warning === 'machine' ? 'preset2.warn_machine' : 'preset2.warn_material')}
                </li>{/each}
            </ul>
          </div>{/if}
        {#if problems.length}<ul class="problems">
            {#each problems as problem (problem)}<li>{t(problemKeys[problem])}</li>{/each}
          </ul>{/if}
        {#if stale}<p role="alert">{t('preset2.stale')}</p>{/if}
        <label class="check"
          ><input type="checkbox" bind:checked={reviewed} />{t('preset2.review')}</label
        >
      </fieldset>
      {#if error}<p role="alert">{error}</p>{/if}
    </div>
    <footer>
      <button type="button" disabled={busy} onclick={onClose}>{t('common.cancel')}</button><button
        class="primary"
        type="button"
        onclick={() => void save()}
        disabled={busy || stale || problems.length > 0 || !previous || !name.trim() || !reviewed}
        >{t('preset2.save')}</button
      >
    </footer>
  </div>
</Modal>

<style>
  .copy-dialog {
    display: flex;
    flex-direction: column;
    max-height: calc(90vh - 2px);
    min-width: 0;
    font-size: var(--inspector-font-field);
    line-height: 1.4;
  }
  .copy-dialog header,
  .copy-dialog footer {
    flex: none;
    padding: var(--ui-space-3);
    background: var(--bg-panel);
  }
  .copy-dialog header {
    border-bottom: 1px solid var(--border);
  }
  .copy-dialog footer {
    border-top: 1px solid var(--border);
  }
  .copy-dialog h2 {
    margin: 0 0 var(--ui-space-1);
    font-size: var(--inspector-font-field);
    font-weight: 600;
  }
  .copy-dialog h3 {
    margin: var(--ui-space-2) 0 0;
    font-size: var(--inspector-font-meta);
    font-weight: 600;
    color: var(--text-section);
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }
  .copy-dialog p {
    margin: 0;
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .source {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .copy-content {
    overflow: auto;
    min-height: 0;
    padding: var(--ui-space-3);
    display: grid;
    gap: var(--ui-space-3);
  }
  .copy-dialog fieldset {
    display: grid;
    gap: var(--ui-space-2);
    border: 0;
    padding: 0;
    margin: 0;
    min-width: 0;
  }
  .copy-dialog label,
  .copy-dialog .field-row {
    font-size: var(--inspector-font-field);
    min-width: 0;
  }
  .field-row {
    display: grid;
    grid-template-columns: 6rem minmax(0, 1fr);
    align-items: center;
    gap: var(--ui-space-2);
  }
  .machine span:last-child {
    overflow-wrap: anywhere;
  }
  .pack-create {
    padding-left: calc(6rem + var(--ui-space-2));
    min-width: 0;
  }
  .copy-dialog input:not([type='checkbox']),
  .copy-dialog select,
  .copy-dialog textarea {
    color: var(--text);
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: var(--ui-space-1) var(--ui-space-2);
    font: inherit;
    min-width: 0;
    width: 100%;
    box-sizing: border-box;
    min-height: var(--inspector-control-height);
  }
  .copy-dialog .check {
    display: flex;
    align-items: flex-start;
    gap: var(--ui-space-2);
    line-height: 1.4;
  }
  /* Inspector descendant rules also reach nested modals. Never stretch an attestation. */
  .copy-dialog .check input[type='checkbox'] {
    flex: 0 0 13px;
    width: 13px;
    height: 13px;
    min-width: 13px;
    min-height: 13px;
    padding: 0;
    margin: 2px 0 0;
    accent-color: var(--accent);
  }
  .copy-dialog table {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
    font-size: var(--inspector-font-meta);
  }
  .copy-dialog th,
  .copy-dialog td {
    text-align: left;
    border-bottom: 1px solid var(--border);
    padding: var(--ui-space-1) var(--ui-space-2);
    overflow-wrap: anywhere;
    vertical-align: top;
  }
  .copy-dialog thead th:first-child {
    width: 22%;
  }
  .copy-dialog tbody th {
    font-weight: 500;
  }
  .warnings {
    border-left: 2px solid var(--warn);
    padding-left: var(--ui-space-2);
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .warnings ul {
    margin: var(--ui-space-1) 0 0;
    padding-left: var(--ui-space-4);
  }
  .identities {
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
    min-width: 0;
  }
  .identities summary {
    cursor: pointer;
    min-height: 24px;
  }
  .identities dl {
    display: grid;
    grid-template-columns: 6rem minmax(0, 1fr);
    gap: var(--ui-space-1) var(--ui-space-2);
  }
  .identities dd {
    margin: 0;
    overflow-wrap: anywhere;
  }
  .problems {
    margin: 0;
    padding: var(--ui-space-2) var(--ui-space-2) var(--ui-space-2) var(--ui-space-5);
    border-left: 2px solid var(--warn);
    font-size: var(--inspector-font-meta);
    color: var(--text-muted);
  }
  .copy-dialog footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--ui-space-2);
  }
  .copy-dialog button {
    cursor: pointer;
    font-size: var(--inspector-font-field);
    color: var(--text);
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: var(--ui-space-1) var(--ui-space-2);
    min-height: var(--inspector-control-height);
  }
  .copy-dialog button.primary {
    background: var(--accent-tint);
    border-color: var(--accent);
    font-weight: 600;
  }
  .copy-dialog button:hover:not(:disabled) {
    background: var(--bg-elevated);
  }
  .copy-dialog button:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .copy-dialog [role='alert'] {
    color: var(--danger);
  }
  @media (max-width: 480px) {
    .field-row {
      grid-template-columns: minmax(0, 1fr);
      gap: var(--ui-space-1);
    }
    .pack-create {
      padding-left: 0;
    }
  }
</style>
