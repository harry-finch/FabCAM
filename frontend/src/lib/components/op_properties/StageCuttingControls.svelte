<script lang="ts">
  import { t } from '../../i18n';
  import { project, type OpEntry } from '../../state/project.svelte';
  import {
    withPocketScope,
    type CuttingStage,
    type MainFinishing,
    type PocketScope,
  } from '../../state/cutting-stages';
  let { op }: { op: OpEntry } = $props();
  let pendingScope = $state<PocketScope | null>(null);
  const scope = $derived(
    op.stages?.contract === 'pocket_finishing_v2' ? op.stages.finishing?.scope : null,
  );
  const proposed = $derived(
    pendingScope && op.stages?.finishing ? withPocketScope(op.stages, pendingScope) : null,
  );
  const scopeLabels = {
    walls_only: 'stages.walls_only',
    floor_only: 'stages.floor_only',
    walls_and_floor: 'stages.walls_and_floor',
  } as const;
  const fields = [
    { key: 'rpm' as const, label: 'stages.rpm', unit: 'rpm', step: 1 },
    { key: 'feed_mm_min' as const, label: 'stages.feed', unit: 'mm/min', step: 1 },
    { key: 'plunge_mm_min' as const, label: 'stages.plunge', unit: 'mm/min', step: 1 },
    { key: 'depth_per_pass_mm' as const, label: 'stages.depth', unit: 'mm', step: 0.1 },
  ] as const;
  const rows = $derived(
    op.stages
      ? [
          { name: 'main' as const, label: t('stages.main'), value: op.stages.main },
          ...(op.stages.finishing
            ? [
                {
                  name: 'finishing' as const,
                  label: t('stages.finishing'),
                  value: op.stages.finishing.cutting,
                },
              ]
            : []),
        ]
      : [],
  );
  function update(stages: MainFinishing) {
    project.updateOperation(op.id, {
      stages,
      toolId: stages.main.tool_id,
      finishToolId: stages.finishing?.cutting.tool_id,
    });
  }
  function edit(stage: 'main' | 'finishing', field: keyof CuttingStage, value: string) {
    if (!op.stages) return;
    const next = structuredClone($state.snapshot(op.stages));
    const target = stage === 'main' ? next.main : next.finishing?.cutting;
    if (target) {
      Object.assign(target, { [field]: value === '' ? null : Number(value) });
      update(next);
    }
  }
  function toggle(on: boolean) {
    if (!op.stages) return;
    const next = structuredClone($state.snapshot(op.stages));
    const finishing = on
      ? {
          cutting: {
            tool_id: next.main.tool_id,
            rpm: null,
            feed_mm_min: null,
            plunge_mm_min: null,
            depth_per_pass_mm: null,
          },
          lateral_allowance_mm: 0.2,
          floor_allowance_mm: 0,
        }
      : null;
    if (next.contract === 'pocket_finishing_v2')
      next.finishing = finishing ? { ...finishing, scope: 'walls_only' } : null;
    else next.finishing = finishing;
    pendingScope = null;
    update(next);
  }
  function allowance(field: 'lateral_allowance_mm' | 'floor_allowance_mm', value: string) {
    if (!op.stages?.finishing || value === '') return;
    const next = structuredClone($state.snapshot(op.stages));
    next.finishing![field] = Number(value);
    update(next);
  }
</script>

{#if !op.stages}<p class="error">{t('stages.missing')}</p>{:else}
  {#each rows as stage (stage.name)}
    {#if stage.name === 'finishing'}
      {#if op.kind === 'pocket'}
        <label class="row"
          ><span>{t('stages.scope')}</span><select
            aria-label={t('stages.scope')}
            value={pendingScope ?? scope ?? 'legacy'}
            onchange={(e) => {
              pendingScope = e.currentTarget.value as PocketScope;
            }}
          >
            {#if !scope}<option value="legacy" disabled>{t('stages.legacy_scope')}</option>{/if}
            {#each Object.entries(scopeLabels) as [value, label] (value)}<option {value}
                >{t(label)}</option
              >{/each}
          </select></label
        >
        {#if proposed?.finishing && op.stages.finishing}
          <div class="scope-preview" role="status">
            <strong>{t('stages.scope_preview')}</strong>
            <p>
              {t('stages.lateral')}: {op.stages.finishing.lateral_allowance_mm} → {proposed
                .finishing.lateral_allowance_mm} mm<br />{t('stages.floor')}: {op.stages.finishing
                .floor_allowance_mm} → {proposed.finishing.floor_allowance_mm} mm
            </p>
            <p>{t('stages.scope_reset')}</p>
            <button
              class="btn-secondary"
              onclick={() => {
                update(proposed);
                pendingScope = null;
              }}>{t('stages.scope_apply')}</button
            >
            <button
              class="btn-secondary"
              onclick={() => {
                pendingScope = null;
              }}>{t('stages.scope_cancel')}</button
            >
          </div>
        {/if}
        <p class="hint">
          {scope
            ? t(
                scope === 'walls_only'
                  ? 'stages.walls_help'
                  : scope === 'floor_only'
                    ? 'stages.floor_help'
                    : 'stages.both_help',
              )
            : t('stages.legacy_scope')}
        </p>
      {/if}
      <div class="stage-heading">
        <strong>{t('stages.finishing')}</strong><button
          class="btn-secondary"
          onclick={() => {
            const next = structuredClone($state.snapshot(op.stages!));
            next.finishing!.cutting = { ...next.main, tool_id: next.finishing!.cutting.tool_id };
            update(next);
          }}>{t('stages.copy')}</button
        >
      </div>
    {:else}<strong class="stage-heading">{t('stages.main')}</strong>{/if}
    <label class="row"
      ><span>{stage.name === 'main' ? t('stages.main_tool') : t('stages.finish_tool')}</span><select
        aria-label={`${stage.name === 'main' ? t('stages.main_tool') : t('stages.finish_tool')}`}
        value={stage.value.tool_id}
        onchange={(e) => edit(stage.name, 'tool_id', e.currentTarget.value)}
        >{#each project.data.tools as tool (tool.id)}<option value={tool.id}
            >#{tool.id} {tool.name}</option
          >{/each}</select
      ></label
    >
    <div class="stage-fields">
      {#each fields as field (field.key)}<label
          ><span>{t(field.label)}</span><input
            aria-label={`${stage.label} ${t(field.label)}`}
            type="number"
            min={field.step === 1 ? 1 : 0.000001}
            step={field.step}
            value={stage.value[field.key] ?? ''}
            placeholder={t('stages.required')}
            onchange={(e) => edit(stage.name, field.key, e.currentTarget.value)}
          /><span class="unit">{field.unit}</span></label
        >{/each}
    </div>
    {#if stage.name === 'main'}
      <label class="finish-toggle"
        ><input
          type="checkbox"
          checked={!!op.stages.finishing}
          onchange={(e) => toggle(e.currentTarget.checked)}
        />
        {t('stages.toggle')}</label
      >
      {#if !op.stages.finishing}<p class="hint">{t('stages.off_help')}</p>{/if}
    {/if}
  {/each}
  {#if op.stages.finishing}
    {#if scope !== 'floor_only'}<label class="row"
        ><span>{t('stages.lateral')}</span>
        <div class="num-cell">
          <input
            aria-label={t('stages.lateral')}
            type="number"
            min="0"
            step="0.05"
            value={op.stages.finishing.lateral_allowance_mm}
            onchange={(e) => allowance('lateral_allowance_mm', e.currentTarget.value)}
          /><span class="unit">{fields[3].unit}</span>
        </div></label
      >
    {/if}
    {#if op.kind === 'pocket' && scope !== 'walls_only'}<label class="row"
        ><span>{t('stages.floor')}</span>
        <div class="num-cell">
          <input
            aria-label={t('stages.floor')}
            type="number"
            min="0"
            step="0.05"
            value={op.stages.finishing.floor_allowance_mm}
            onchange={(e) => allowance('floor_allowance_mm', e.currentTarget.value)}
          /><span class="unit">{fields[3].unit}</span>
        </div></label
      >{/if}
    <p class="hint">{t('stages.allowance_help')}</p>
    <p class="hint">{t(scope ? 'stages.pocket_proof_help' : 'stages.proof_help')}</p>
    {#if op.stages.finishing.lateral_allowance_mm === 0 && op.stages.finishing.floor_allowance_mm === 0}<p
        class="hint"
      >
        <strong>{t('stages.repeat')}</strong>
      </p>{/if}
    {#if op.stages.finishing.cutting.tool_id === op.stages.main.tool_id}<p
        class="hint"
        class:error={op.stages.finishing.cutting.rpm !== op.stages.main.rpm}
      >
        {t('stages.rpm_help')}
      </p>{/if}
  {/if}
  <details class="subsection">
    <summary>{t('stages.retained_title')}</summary><label class="row"
      ><span>{t('stages.retained')}</span>
      <div class="num-cell">
        <input
          aria-label={t('stages.retained')}
          type="number"
          min="0"
          step="0.05"
          value={op.retainedStockMm ?? 0}
          onchange={(e) => {
            if (e.currentTarget.value !== '')
              project.updateOperation(op.id, { retainedStockMm: Number(e.currentTarget.value) });
          }}
        /><span class="unit">{fields[3].unit}</span>
      </div></label
    >
    <p class="hint">{t('stages.retained_help')}</p>
  </details>
{/if}

<style>
  .scope-preview {
    border: 1px solid var(--border, #555);
    padding: 10px;
    margin: 10px 0;
    font-size: 12px;
    line-height: 1.5;
  }
  .scope-preview button {
    margin: 4px 8px 0 0;
  }
  .stage-heading {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin: 6px 0 10px;
    font-size: 12px;
  }
  .stage-fields {
    display: grid;
    gap: 6px;
    margin: 8px 0 12px;
  }
  .stage-fields label {
    display: grid;
    grid-template-columns: minmax(110px, 1fr) minmax(65px, 1fr) 48px;
    align-items: center;
    gap: 8px;
    font-size: 12px;
  }
  input,
  select {
    min-width: 0;
    max-width: 100%;
  }
  .unit,
  .hint {
    color: var(--text-muted, #999);
    font-size: 11px;
  }
  .hint {
    line-height: 1.45;
    margin: 8px 0;
  }
  .finish-toggle {
    display: flex;
    gap: 8px;
    align-items: center;
    margin: 12px 0 8px;
    font-size: 12px;
  }
  .finish-toggle input {
    flex: 0 0 auto;
    width: auto;
  }
  .error {
    color: var(--danger, #ef8a8a);
  }
</style>
