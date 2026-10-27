<script lang="ts">
  import { suggestMachineName } from '../state/tool_naming';
  import { machineIdentity } from '../state/machine-identity';
  import {
    TriangleAlert,
    Cpu,
    Monitor,
    Sun,
    Moon,
    CircleHelp,
    Settings,
    Info,
  } from '@lucide/svelte';
  import { isProgramOnlyOp } from '../state/op_types';
  import { project } from '../state/project.svelte';
  import { t } from '../i18n';
  import ShortcutHelp from './ShortcutHelp.svelte';
  let showHelp = $state(false);
  const buildIdentity = `${typeof __IVAC_PKG_VERSION__ === 'string' ? __IVAC_PKG_VERSION__ : '?'} · ${typeof __IVAC_BUILD_VERSION__ === 'string' ? __IVAC_BUILD_VERSION__ : 'unknown'}`;
  const shortBuildIdentity = buildIdentity.replace(/v[^ ]+-\d+-g([0-9a-f]+)(-dirty)?$/, '$1$2');
  const themes = [
    { value: 'auto', Icon: Monitor },
    { value: 'light', Icon: Sun },
    { value: 'dark', Icon: Moon },
  ] as const;
  type Area = 'project' | 'machine' | 'tools' | 'presets' | 'settings' | 'help' | 'about';
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
    <span class="brand-description">{t('shell.description')}</span>
  </span>
  <button
    class="build-identity"
    title={`${t('about.build')}: ${buildIdentity}`}
    aria-label={`${t('about.build')}: ${buildIdentity}`}
    onclick={() => onNavigate('about')}>{shortBuildIdentity}</button
  >

  <button
    type="button"
    class="main-tab context-tab machine-context"
    class:active={mainTab === 'machine'}
    aria-current={mainTab === 'machine' ? 'page' : undefined}
    onclick={() => onNavigate('machine')}
    aria-label={`${t('app.tab.machine')}: ${activeMachineName.fullName}`}
    title={`${activeMachineName.fullName} — ${t('app.tab.machine_tip')}`}
  >
    <Cpu size={16} aria-hidden="true" />
    <span>{t('app.tab.machine')}</span>
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
    class:active={mainTab === 'project'}
    aria-current={mainTab === 'project' ? 'page' : undefined}
    onclick={() => onNavigate('project')}>{t('app.tab.project')}</button
  >
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'tools'}
    aria-current={mainTab === 'tools' ? 'page' : undefined}
    onclick={() => onNavigate('tools')}
    title={t('app.tab.tools_tip')}>{t('app.tab.tools')}</button
  >
  <button
    type="button"
    class="main-tab"
    class:active={mainTab === 'presets'}
    aria-current={mainTab === 'presets' ? 'page' : undefined}
    onclick={() => onNavigate('presets')}>{t('presets.title')}</button
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
  <div class="header-controls">
    <select
      class="language-control"
      aria-label={t('settings.appearance.language')}
      title={t('settings.appearance.language')}
      value={project.data.settings.language}
      onchange={(event) =>
        project.updateSettings({ language: event.currentTarget.value as 'auto' | 'en' | 'de' })}
    >
      <option value="auto">{t('settings.appearance.language.auto')}</option>
      <option value="en">{t('settings.appearance.language.en')}</option>
      <option value="de">{t('settings.appearance.language.de')}</option>
    </select>
    <div class="theme-control" role="group" aria-label={t('settings.appearance.theme')}>
      {#each themes as choice (choice.value)}
        <button
          type="button"
          class="utility-button"
          class:chosen={project.data.settings.theme === choice.value}
          aria-pressed={project.data.settings.theme === choice.value}
          aria-label={t(`settings.appearance.theme.${choice.value}`)}
          title={t(`settings.appearance.theme.${choice.value}`)}
          onclick={() => project.updateSettings({ theme: choice.value })}
          ><choice.Icon size={12} aria-hidden="true" /></button
        >
      {/each}
    </div>
    <button
      type="button"
      class="utility-button"
      class:chosen={mainTab === 'settings'}
      aria-label={t('app.tab.settings')}
      title={t('app.tab.settings_tip')}
      onclick={() => onNavigate('settings')}><Settings size={16} aria-hidden="true" /></button
    >
    <button
      type="button"
      class="utility-button"
      class:chosen={mainTab === 'about'}
      aria-label={t('app.tab.about')}
      title={t('app.tab.about_tip')}
      onclick={() => onNavigate('about')}><Info size={16} aria-hidden="true" /></button
    >
    <button
      type="button"
      class="utility-button"
      aria-label={t('app.tab.help')}
      title={t('app.tab.help_tip')}
      aria-haspopup="dialog"
      onclick={() => (showHelp = true)}><CircleHelp size={16} aria-hidden="true" /></button
    >
  </div>
</nav>

{#if showHelp}
  <ShortcutHelp onClose={() => (showHelp = false)} />
{/if}

<style>
  .build-identity {
    flex-shrink: 0;
    font-size: 10px;
    color: var(--muted);
    background: var(--bg);
    border: 1px solid var(--border);
    padding: 2px 5px;
  }

  .app-brand {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
    align-self: center;
    font-size: 14px;
    font-weight: 700;
    letter-spacing: -0.01em;
    padding: 0 0.6rem 0 0;
    color: var(--header-text);
  }
  .app-brand img {
    display: block;
    width: 24px;
    height: 24px;
    flex-shrink: 0;
    box-sizing: border-box;
    border: 1px solid #354759;
    border-radius: 7px;
    background: #181f2a;
  }
  .brand-description {
    font-size: 11.5px;
    font-weight: 400;
    letter-spacing: normal;
    color: var(--header-muted);
    white-space: nowrap;
  }
  @media (max-width: 1200px) {
    .brand-description {
      display: none;
    }
  }
  .main-tabs {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    /* The app's existing 2px identity line completes FabSpeed's 42px header. */
    min-height: 40px;
    box-sizing: border-box;
    padding: 0 12px;
    background: var(--bg-app, var(--bg));
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
  .context-tab > span:not(.context-value):not(.machine-limitation) {
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
    align-self: stretch;
    border: 0;
    border-bottom: 2px solid transparent;
    border-radius: 0;
    padding: 0.3rem 0.65rem;
    white-space: nowrap;
    font-size: 12.5px;
    font-weight: 500;
    color: var(--header-muted);
    cursor: pointer;
  }
  .main-tab:hover {
    color: var(--text);
  }
  .main-tab.active {
    border-bottom-color: var(--accent);
    color: var(--text-strong);
  }
  .main-tab:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -3px;
  }
  .machine-context {
    align-self: center;
    min-height: 30px;
    margin: 0 0.6rem 0 0.3rem;
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-panel);
  }
  .machine-context.active {
    border-color: var(--accent);
  }
  .machine-context .context-value {
    font-weight: 600;
  }

  .header-controls {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
    margin-left: 12px;
  }
  .theme-control {
    display: flex;
    box-sizing: border-box;
    height: 24px;
    gap: 2px;
    padding: 2px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-input);
  }
  .utility-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 24px;
    min-width: 24px;
    padding: 0 6px;
    border: 0;
    border-radius: 3px;
    background: transparent;
    color: var(--header-muted);
    cursor: pointer;
  }
  .theme-control .utility-button {
    height: 18px;
    width: 24px;
    min-width: 24px;
    padding: 0 6px;
  }
  .theme-control .chosen {
    box-shadow: inset 0 0 0 1px var(--border);
  }
  .utility-button:hover,
  .utility-button.chosen {
    background: var(--bg-elevated);
    color: var(--header-text);
  }
  .utility-button:focus-visible,
  .language-control:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
  .language-control {
    width: 108px;
    height: 24px;
    box-sizing: border-box;
    padding: 0 6px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-input);
    color: var(--header-text);
    font-size: 11.5px;
  }
  .main-tabs.hidden {
    display: none;
  }
</style>
