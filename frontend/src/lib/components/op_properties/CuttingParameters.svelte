<script lang="ts">
  import { t } from '../../i18n';
  import { parameterSummary } from '../../state/preset-library-v9';
  import { onDestroy } from 'svelte';
  import { project, type OpEntry } from '../../state/project.svelte';
  import {
    cuttingRoles,
    usesEntryAngle,
    type CuttingRole,
    type RoleRates,
  } from '../../state/cutting-parameters';
  import {
    inspectCutting,
    cuttingTargetKey,
    preparePresetLoad,
    captureTestedPreset,
    presetStatus,
    type CuttingInspection,
  } from '../../state/cutting-workflow';
  import {
    listCollections,
    validateCollection,
    saveCollection,
    type PresetCollection,
  } from '../../state/preset-collections';
  import { testedRows } from '../../state/preset-library-v9';
  import './op-properties.css';
  let { op }: { op: OpEntry } = $props();
  let selection = $state<{ id: number; role: CuttingRole } | null>(null);
  const roles = $derived(cuttingRoles(op));
  const requestedRole = $derived(
    selection?.id === op.id ? selection.role : op.kind === 'profile' ? 'finish' : roles[0]?.role,
  );
  const activeRole = $derived(
    roles.some((r) => r.role === requestedRole) ? requestedRole : roles[0]?.role,
  );
  const slot = $derived(op.cutting?.rates.find((r) => r.role === activeRole));
  const key = $derived(cuttingTargetKey(project.data, op.id));
  let inspection = $state<CuttingInspection | null>(null);
  let error = $state('');
  let busy = $state(false);
  let panel = $state<'load' | 'save' | null>(null);
  let collections = $state<{ data: PresetCollection; token: string }[]>([]);
  let prepared = $state<Awaited<ReturnType<typeof preparePresetLoad>> | null>(null);
  let selected = $state('');
  let name = $state('');
  let collectionName = $state('Workshop');
  let attestedKey = $state('');
  let message = $state('');
  let sequence = 0;
  let alive = true;
  onDestroy(() => {
    alive = false;
    sequence++;
  });
  $effect(() => {
    void key;
    const request = ++sequence;
    inspection = null;
    prepared = null;
    attestedKey = '';
    void inspectCutting(project, op.id)
      .then((v) => {
        if (alive && request === sequence) inspection = v;
      })
      .catch((e) => {
        if (alive && request === sequence) error = String(e);
      });
  });
  function edit(field: 'rpm' | 'feed_mm_min' | 'plunge_mm_min', value: string) {
    const rates: RoleRates[] = roles.map(
      (r) =>
        op.cutting?.rates.find((s) => s.role === r.role) ?? {
          ...r,
          rpm: null,
          feed_mm_min: null,
          plunge_mm_min: null,
        },
    );
    project.updateOperation(op.id, {
      cutting: {
        ...op.cutting,
        rates: rates.map((r) =>
          r.role === activeRole ? { ...r, [field]: value === '' ? null : Number(value) } : r,
        ),
      },
    });
  }
  async function open(next: 'load' | 'save') {
    panel = next;
    error = '';
    message = '';
    prepared = null;
    attestedKey = '';
    busy = true;
    name = op.name;
    try {
      const rows = await listCollections();
      const parsed = await Promise.all(
        rows.map(async (r) => ({ data: await validateCollection(r.json), token: r.token })),
      );
      if (!alive) return;
      collections = parsed;
      selected = parsed[0]?.data.id ?? '__new__';
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  async function review(collection: PresetCollection, id: string) {
    busy = true;
    error = '';
    prepared = null;
    try {
      const value = await preparePresetLoad(project, op.id, collection, id);
      if (alive) prepared = value;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  function apply() {
    if (!prepared) return;
    try {
      project.history.exec(prepared.command, project.target());
      panel = null;
      prepared = null;
      message = 'Parameters copied into this operation';
    } catch (e) {
      error = String(e);
    }
  }
  async function save() {
    busy = true;
    error = '';
    try {
      const preset = await captureTestedPreset(project, op.id, name, attestedKey);
      const stored = collections.find((c) => c.data.id === selected);
      const collection: PresetCollection = stored
        ? JSON.parse(JSON.stringify(stored.data))
        : {
            format: 'fabcam-preset-collection',
            version: 1,
            id: crypto.randomUUID(),
            name: collectionName.trim(),
            presets: [],
          };
      collection.presets.push(preset);
      await saveCollection(collection, stored?.token ?? null);
      if (alive) {
        panel = null;
        message = `Saved in ${collection.name}`;
        attestedKey = '';
      }
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  const label: Record<CuttingRole, string> = {
    rough: 'Main / roughing',
    finish: 'Finishing',
    drill: 'Drilling',
    spot: 'Spotting',
    chamfer: 'Hole chamfer',
  };
  const fields = [
    { key: 'rpm' as const, label: 'Spindle speed', unit: 'rpm' },
    { key: 'feed_mm_min' as const, label: 'Feed rate', unit: 'mm/min' },
    { key: 'plunge_mm_min' as const, label: 'Plunge rate', unit: 'mm/min' },
  ];
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === 'Escape' && panel) {
      panel = null;
      prepared = null;
      attestedKey = '';
    }
  }}
/>
{#if roles.length}
  <fieldset class="cutting">
    <legend>{t('fsreset.ui.cutting.parameters')}</legend>
    {#if roles.length > 1}<label class="row"
        ><span>{t('fsreset.ui.pass')}</span><select
          aria-label={t('fsreset.ui.cutting.pass')}
          value={activeRole}
          onchange={(e) => (selection = { id: op.id, role: e.currentTarget.value as CuttingRole })}
          >{#each roles as r (r.role)}<option value={r.role}>{label[r.role]} · T{r.tool_id}</option
            >{/each}</select
        ></label
      >{/if}
    {#if op.kind === 'profile'}<p class="basis">
        Profile contours use the finishing values. Main values are stored separately; they do not
        set the contour feed.
      </p>{/if}
    {#each fields as field (field.key)}<label class="row"
        ><span>{field.label}</span><input
          aria-label={field.label}
          type="number"
          min="1"
          step="1"
          value={slot?.[field.key] ?? ''}
          placeholder={t('fsreset.ui.required')}
          onchange={(e) => edit(field.key, e.currentTarget.value)}
        /><span class="unit">{field.unit}</span></label
      >{/each}
    {#if ['profile', 'pocket', 'engrave'].includes(op.kind) && !op.depthList?.length}<label
        class="row"
        ><span>{t('fsreset.ui.depth.per.pass')}</span><input
          aria-label={t('fsreset.ui.depth.per.pass')}
          type="number"
          min="0.000001"
          step="0.1"
          value={op.step === null ? '' : Math.abs(op.step)}
          onchange={(e) =>
            project.updateOperation(op.id, {
              step: e.currentTarget.value === '' ? null : -Number(e.currentTarget.value),
            })}
        /><span class="unit">{t('fsreset.ui.mm')}</span></label
      >{/if}
    {#if usesEntryAngle(op)}<label class="row"
        ><span>{t('fsreset.ui.entry.angle2')}</span><input
          type="number"
          min="1"
          max={op.kind === 'drill' ? 45 : 89}
          step="any"
          value={op.cutting?.entry_angle_deg ?? ''}
          onchange={(e) =>
            project.updateOperation(op.id, {
              cutting: {
                ...op.cutting!,
                entry_angle_deg:
                  e.currentTarget.value === '' ? null : Number(e.currentTarget.value),
              },
            })}
        /><span class="unit">°</span></label
      >{/if}
    <p class="basis">{presetStatus(op, inspection)}</p>
    {#if inspection?.check.errors.length}<ul class="errors">
        {#each inspection.check.errors as issue, index (index)}<li>{issue}</li>{/each}
      </ul>{/if}
    {#if inspection?.check.warnings.length}<details>
        <summary>{t('fsreset.ui.unknown.machine.tool.limits')}</summary
        >{#each inspection.check.warnings as warning, index (index)}<p class="basis">
            {warning}
          </p>{/each}
      </details>{/if}
    <div class="actions">
      <button onclick={() => open('load')} disabled={busy}>{t('fsreset.ui.load.preset')}</button
      ><button onclick={() => open('save')} disabled={busy}>{t('fsreset.ui.save.preset')}</button>
    </div>
    {#if panel}
      <section
        class="editor"
        aria-label={panel === 'load' ? 'Load tested preset' : 'Save tested preset'}
      >
        <div class="actions">
          <strong>{panel === 'load' ? 'Load tested preset' : 'Save tested preset'}</strong><button
            onclick={() => {
              panel = null;
              prepared = null;
              attestedKey = '';
            }}>{t('fsreset.ui.cancel')}</button
          >
        </div>
        {#if panel === 'load'}
          <p class="basis">
            {t('fsreset.ui.choose.an.exact.machine.tool.and.material.configuration')}
          </p>
          <div class="choices">
            {#each testedRows(collections.map((c) => c.data)) as row (row.key)}<button
                class="choice"
                disabled={busy}
                onclick={() => review(row.collection, row.preset.id)}
                ><strong>{row.preset.name}</strong><small
                  >{row.collection.name} · {row.preset.context.machine_label} · {row.tools} · {row
                    .preset.context.material_label} · {row.operation}</small
                ></button
              >{/each}
          </div>
          {#if !busy && !collections.some((c) => c.data.presets.length)}<p class="basis">
              {t('fsreset.ui.no.tested.presets.yet.save.a.tested.configuration')}
            </p>{/if}
          {#if prepared}<table>
              <thead
                ><tr><th>{t('fsreset.ui.pass')}</th><th>{t('fsreset.ui.current.preset')}</th></tr
                ></thead
              ><tbody
                >{#each prepared.after as after (after.role)}{@const before = prepared.before.find(
                    (v) => v.role === after.role,
                  )}<tr
                    ><td>{label[after.role]}</td><td
                      >{before?.rpm ?? '—'} → {after.rpm} rpm<br />{before?.feed_mm_min ?? '—'} → {after.feed_mm_min}
                      mm/min<br />{before?.plunge_mm_min ?? '—'} → {after.plunge_mm_min} mm/min plunge</td
                    ></tr
                  >{/each}</tbody
              >
            </table>
            <p class="basis">
              Depth per pass: {op.step === null ? '—' : Math.abs(op.step)} → {prepared.patch.step ==
              null
                ? 'explicit schedule'
                : Math.abs(prepared.patch.step)} mm. Depth list: {(op.depthList ?? []).join(', ') ||
                'none'} → {(prepared.patch.depthList ?? []).join(', ') || 'none'}. Finish step: {('finishStep' in
              op
                ? op.finishStep
                : undefined) ?? 'none'} → {('finishStep' in prepared.patch
                ? prepared.patch.finishStep
                : undefined) ?? 'none'}.
            </p>
            {#if op.kind === 'pocket'}<p class="basis">
                Overlap: {(op.xyOverlap ?? 0) * 100}% → {('xyOverlap' in prepared.patch
                  ? (prepared.patch.xyOverlap ?? 0)
                  : (op.xyOverlap ?? 0)) * 100}%.
              </p>{/if}
            {#if op.kind === 'planar_surface' && 'surface' in prepared.patch}<p class="basis">
                Stepover: {op.surface.stepover_mm} → {prepared.patch.surface?.stepover_mm} mm.
              </p>{/if}
            {#each parameterSummary(prepared.parameters) as line (line)}<p class="basis">
                {line}
              </p>{/each}
            <button class="primary" onclick={apply}>{t('fsreset.ui.replace.parameters')}</button>
          {/if}
        {:else}
          <label class="row"
            ><span>{t('fsreset.ui.preset.name')}</span><input
              bind:value={name}
              maxlength="160"
            /></label
          >
          <label class="row"
            ><span>{t('fsreset.ui.collection')}</span><select bind:value={selected}
              >{#each collections as c (c.data.id)}<option value={c.data.id}>{c.data.name}</option
                >{/each}<option value="__new__">{t('fsreset.ui.new.collection2')}</option></select
            ></label
          >
          {#if selected === '__new__'}<label class="row"
              ><span>{t('fsreset.ui.collection.name')}</span><input
                bind:value={collectionName}
                maxlength="160"
              /></label
            >{/if}
          {#if inspection?.context_error}<p class="errors">{inspection.context_error}</p>{/if}
          <label class="attestation"
            ><input
              type="checkbox"
              checked={attestedKey === key}
              onchange={(e) => (attestedKey = e.currentTarget.checked ? key : '')}
            />{t('fsreset.ui.i.tested.these.values.with.this.machine.tool')}</label
          >
          <p class="basis">
            {t('fsreset.ui.software.validation.does.not.establish.a.workshop.test')}
          </p>
          <button class="primary" disabled={busy || attestedKey !== key} onclick={save}
            >{t('fsreset.ui.save.tested.preset')}</button
          >
        {/if}
      </section>
    {/if}
    {#if error}<p class="errors" role="alert">{error}</p>{/if}
    {#if message}<p class="basis" role="status">{message}</p>{/if}
  </fieldset>
{/if}

<style>
  .cutting {
    min-width: 0;
  }
  .row {
    display: grid;
    grid-template-columns: minmax(85px, 1fr) minmax(60px, 1fr) auto;
    gap: 6px;
    align-items: center;
    margin: 5px 0;
  }
  .row select {
    grid-column: 2/-1;
  }
  .row input {
    min-width: 0;
    width: 100%;
    box-sizing: border-box;
  }
  .actions {
    display: flex;
    justify-content: space-between;
    gap: 6px;
    margin: 8px 0;
    align-items: center;
  }
  .basis {
    font-size: 11px;
    color: var(--text-secondary, #aaa);
    overflow-wrap: anywhere;
  }
  .errors {
    font-size: 11px;
    color: var(--error, #f88);
    padding-left: 14px;
  }
  .editor {
    border-top: 1px solid var(--border, #444);
    margin-top: 8px;
    padding-top: 4px;
  }
  .choices {
    max-height: 200px;
    overflow: auto;
  }
  .choice {
    display: block;
    width: 100%;
    text-align: left;
    margin-bottom: 4px;
  }
  .choice small {
    display: block;
    font-size: 10px;
    color: var(--text-secondary, #aaa);
  }
  .attestation {
    display: flex;
    align-items: flex-start;
    gap: 6px;
    font-size: 12px;
    margin: 10px 0;
  }
  table {
    width: 100%;
    font-size: 11px;
    border-collapse: collapse;
  }
  td,
  th {
    text-align: left;
    padding: 5px;
    border-bottom: 1px solid var(--border, #444);
  }
</style>
