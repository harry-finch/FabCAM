<script lang="ts">
  import { suggestMachineName } from '../state/tool_naming';
  import { machineIdentity } from '../state/machine-identity';
  import { TriangleAlert } from '@lucide/svelte';
  import { isProgramOnlyOp } from '../state/op_types';
  import { project } from '../state/project.svelte';
  import { t } from '../i18n';
  type Area = 'project' | 'machine' | 'tools' | 'settings' | 'help' | 'about';
  let {
    mainTab,
    narrow,
    onNavigate,
    onShowOperation,
  }: {
    mainTab: Area;
    narrow: boolean;
    onNavigate: (tab: Area) => void;
    onShowOperation: () => void;
  } = $props();
  const selectedOpForHint = $derived(
    project.data.operations.find((op) => op.id === project.sel.selectedOpId) ?? null,
  );
  const activeMachineName = $derived(
    machineIdentity(project.data.machine.name, suggestMachineName(project.data.machine)),
  );
  const selectedOperationTool = $derived(
    selectedOpForHint == null
      ? null
      : (project.data.tools.find((tool) => tool.id === selectedOpForHint.toolId) ?? null),
  );
  const operationToolLabel = $derived(
    selectedOpForHint == null
      ? null
      : selectedOperationTool
        ? `T${selectedOperationTool.id} · ${selectedOperationTool.name}`
        : t('app.context.missing_tool', { id: selectedOpForHint.toolId }),
  );
</script>

<nav class="main-tabs" class:hidden={narrow} aria-label={t('app.tab.areas')}>
  <span class="app-brand">
    <img src="/branding/fabcam-symbol.svg" alt="" aria-hidden="true" width="24" height="24" />
    {t('shell.brand')}
  </span>
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'project'}
    onclick={() => onNavigate('project')}>{t('app.tab.project')}</button
  >
  <button
    type="button"
    class="main-tab context-tab"
    class:active={mainTab === 'machine'}
    onclick={() => onNavigate('machine')}
    aria-label={`${t('app.tab.machine')}: ${activeMachineName.fullName}`}
    title={`${activeMachineName.fullName} — ${t('app.tab.machine_tip')}`}
  >
    <span>{t('app.tab.machine')}:</span>
    <span class="context-value">{activeMachineName.shortName}</span>
    {#if activeMachineName.limitation}
      <span class="machine-limitation"
        ><TriangleAlert size={13} aria-hidden="true" />{activeMachineName.limitation}</span
      >
    {/if}
  </button>
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'tools'}
    onclick={() => onNavigate('tools')}
    title={t('app.tab.tools_tip')}>{t('app.tab.tools')}</button
  >
  {#if operationToolLabel != null && selectedOpForHint && !isProgramOnlyOp(selectedOpForHint.kind) && mainTab === 'project'}
    <button
      type="button"
      class="main-tab context-tab"
      onclick={() => {
        if (selectedOperationTool) project.sel.toolsDialogFocusId = selectedOperationTool.id;
        else onShowOperation();
      }}
      aria-label={operationToolLabel == null
        ? t('app.tab.tools')
        : t('app.context.operation_tool_tip', {
            operation: selectedOpForHint?.name ?? '',
            tool: operationToolLabel,
          })}
      title={operationToolLabel == null
        ? t('app.tab.tools_tip')
        : t('app.context.operation_tool_tip', {
            operation: selectedOpForHint?.name ?? '',
            tool: operationToolLabel,
          })}
    >
      <span>{t('app.context.operation_tool')}:</span>
      <span class="context-value">{operationToolLabel}</span>
    </button>
  {/if}
  <span class="main-tabs-flex"></span>
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'settings'}
    onclick={() => onNavigate('settings')}
    title={t('app.tab.settings_tip')}>{t('app.tab.settings')}</button
  >
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'help'}
    onclick={() => onNavigate('help')}
    title={t('app.tab.help_tip')}>{t('app.tab.help')}</button
  >
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'about'}
    onclick={() => onNavigate('about')}
    title={t('app.tab.about_tip')}>{t('app.tab.about')}</button
  >
</nav>

<style>
  .app-brand {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    flex-shrink: 0;
    align-self: center;
    font-size: 0.85rem;
    font-weight: 600;
    padding: 0 0.6rem 0 0;
    color: var(--text-strong);
  }
  .app-brand img {
    display: block;
    width: 24px;
    height: 24px;
    flex-shrink: 0;
  }
  .main-tabs {
    display: flex;
    gap: 0.15rem;
    padding: 0.2rem 0.7rem 0;
    background: var(--bg-navigation);
    border-bottom: 1px solid var(--border);
  }
  .main-tabs-flex {
    flex: 1;
  }
  .context-tab {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    min-width: 0;
    max-width: 17rem;
  }
  .context-tab > span:first-child {
    flex-shrink: 0;
    font-size: 0.74rem;
    color: var(--text-muted);
  }
  .machine-limitation {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
    color: var(--warn);
    font-size: 0.7rem;
  }
  .context-tab:has(.machine-limitation) {
    max-width: 24rem;
  }
  .context-value {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-strong);
  }
  .main-tab {
    background: none;
    border: 1px solid transparent;
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    padding: 0.3rem 0.85rem;
    font-size: 0.82rem;
    color: var(--text-muted);
    cursor: pointer;
  }
  .main-tab:hover {
    color: var(--text);
  }
  .main-tab.active {
    background: var(--bg-panel);
    border-color: var(--border);
    color: var(--text-strong);
    /* Visually merge with the content below by sitting on the strip's
       bottom border. */
    margin-bottom: -1px;
  }

  .main-tabs.hidden {
    display: none;
  }
</style>
