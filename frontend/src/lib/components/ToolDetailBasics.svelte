<script lang="ts">
  import { t } from '../i18n';
  import type { ToolEntry, ToolKind } from '../state/project.svelte';
  import { KIND_DISPLAY_LABELS } from '../state/tool_family';
  import { fieldApplies, diameterInvalid } from '../state/tool_validation';
  import type { MsgKey } from '../i18n/keys';
  let {
    tool,
    legacy = false,
    onUpdateField,
    onKindChange,
  }: {
    tool: ToolEntry;
    legacy?: boolean;
    onUpdateField: <K extends keyof ToolEntry>(key: K, value: ToolEntry[K]) => void;
    onKindChange: (kind: ToolKind) => void;
  } = $props();
  const kinds = Object.keys(KIND_DISPLAY_LABELS) as ToolKind[];
  const geometry: {
    key: 'diameter' | 'tipDiameter' | 'tipAngleDeg' | 'flutes';
    label: MsgKey;
    unit: string;
    step: number;
    min: number;
  }[] = [
    { key: 'diameter', label: 'tools.col.diameter.title', unit: 'mm', step: 0.1, min: 0.01 },
    { key: 'tipDiameter', label: 'ux6b.tip_diameter', unit: 'mm', step: 0.05, min: 0 },
    { key: 'tipAngleDeg', label: 'ux6b.tip_angle', unit: '°', step: 1, min: 1 },
    { key: 'flutes', label: 'tools.col.flutes', unit: '', step: 1, min: 1 },
  ];
  const defaults: {
    key: 'speed' | 'feedRate' | 'plungeRate' | 'defaultStep';
    applies: string;
    label: MsgKey;
    unit: string;
  }[] = [
    { key: 'speed', applies: 'speed', label: 'tools.col.speed', unit: 'RPM' },
    { key: 'feedRate', applies: 'feedRate', label: 'tools.col.feed', unit: 'mm/min' },
    { key: 'plungeRate', applies: 'plunge', label: 'tools.col.plunge', unit: 'mm/min' },
    { key: 'defaultStep', applies: 'defaultStep', label: 'tools.col.dflt_step', unit: 'mm' },
  ];
</script>

<div class="fields">
  {#if !legacy}
    <label
      >{t('tools.col.name')}<input
        value={tool.name}
        oninput={(e) => onUpdateField('name', e.currentTarget.value)}
      /></label
    >
    <label
      >{t('tools.col.kind')}<select
        value={tool.kind}
        onchange={(e) => onKindChange(e.currentTarget.value as ToolKind)}
        >{#each kinds as kind (kind)}<option value={kind}>{KIND_DISPLAY_LABELS[kind]}</option
          >{/each}</select
      ></label
    >
    {#each geometry as field (field.key)}
      {#if fieldApplies(field.key, tool.kind)}
        <label
          >{field.key === 'diameter'
            ? t('ux6b.diameter')
            : field.key === 'tipAngleDeg' && tool.kind === 'thread_mill'
              ? t('ux6b.flank_angle')
              : t(field.label)}
          <span class="numeric"
            ><input
              type="number"
              min={field.min}
              step={field.step}
              max={field.key === 'tipAngleDeg' ? 179 : undefined}
              value={tool[field.key] ?? ''}
              placeholder="—"
              class:invalid={field.key === 'diameter' && diameterInvalid(tool)}
              onchange={(e) => {
                const value = e.currentTarget.value;
                if (field.key === 'flutes') onUpdateField('flutes', parseInt(value, 10) || 1);
                else if (field.key === 'diameter')
                  onUpdateField('diameter', parseFloat(value) || 0);
                else if (field.key === 'tipDiameter')
                  onUpdateField(
                    'tipDiameter',
                    value === '' || Number(value) < 0 ? undefined : Number(value),
                  );
                else onUpdateField('tipAngleDeg', value === '' ? undefined : Number(value));
              }}
            /><span>{field.unit}</span></span
          >
        </label>
      {/if}
    {/each}
  {:else}
    {#each defaults as field (field.key)}
      {#if fieldApplies(field.applies, tool.kind)}
        <label
          >{t(field.label)}<span class="numeric"
            ><input
              type="number"
              step={field.key === 'defaultStep' ? 0.05 : 1}
              value={tool[field.key] ?? ''}
              placeholder="—"
              onchange={(e) =>
                onUpdateField(
                  field.key,
                  e.currentTarget.value === ''
                    ? field.key === 'defaultStep'
                      ? undefined
                      : 0
                    : Number(e.currentTarget.value),
                )}
            /><span>{field.unit}</span></span
          ></label
        >
      {/if}
    {/each}
  {/if}
</div>

<style>
  .fields {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  label {
    display: grid;
    grid-template-columns: 150px minmax(0, 1fr);
    align-items: center;
    gap: 10px;
    font-size: 0.8rem;
  }
  input,
  select {
    background: var(--bg-input);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 4px 6px;
    min-width: 0;
    font: inherit;
    box-sizing: border-box;
  }
  .numeric {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--text-muted);
  }
  .numeric input {
    width: 96px;
  }
  .invalid {
    border-color: var(--danger);
  }
</style>
