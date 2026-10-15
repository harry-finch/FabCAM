<script lang="ts">
  import { ChevronDown, FilePlus2, FolderOpen, Import, X, FileText } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import { openAny, addDrawing } from '../services/file_ops';
  import { openRecentProject } from '../services/workspace-session.svelte';
  import { paletteShortcut } from '../state/palette-shortcut';
  import { t } from '../i18n';
  let {
    onNew,
    onClose,
    onReport,
    onPalette,
  }: { onNew: () => void; onClose: () => void; onReport: () => void; onPalette: () => void } =
    $props();
  let menu: HTMLDetailsElement;
  const recents = $derived.by(() => {
    void workspace.version;
    return workspace.get().recent_projects;
  });
  function run(action: () => void) {
    menu.open = false;
    menu.querySelector('summary')?.focus();
    action();
  }
  function outside(event: MouseEvent) {
    if (menu && !menu.contains(event.target as Node)) menu.open = false;
  }
  function escape(event: KeyboardEvent) {
    if (event.key === 'Escape' && menu?.open) {
      menu.open = false;
      menu.querySelector('summary')?.focus();
    }
  }
</script>

<svelte:window onclick={outside} onkeydown={escape} />
<details bind:this={menu} class="project-menu">
  <summary>{t('shell.file_menu')}<ChevronDown size={14} /></summary>
  <div class="commands">
    <button disabled={project.loading} onclick={() => run(onNew)}
      ><FilePlus2 size={15} />{t('shell.new')}</button
    >
    <button disabled={project.loading} onclick={() => run(() => void openAny())}
      ><FolderOpen size={15} />{t('shell.open')}</button
    >
    <button disabled={project.loading} onclick={() => run(() => void addDrawing())}
      ><Import size={15} />{t('shell.import')}</button
    >
    <button onclick={() => run(onPalette)}
      >{t('palette.menu')} <small>{paletteShortcut()}</small></button
    >
    <hr />
    <span class="label">{t('shell.recents')}</span>
    {#each recents.slice(0, 6) as recent (recent.path)}
      <button
        disabled={project.loading}
        title={recent.path}
        onclick={() => run(() => void openRecentProject(recent.path))}>{recent.filename}</button
      >
    {:else}<span class="label">{t('shell.no_recents')}</span>{/each}
    {#if recents.length}<button onclick={() => run(() => workspace.clearRecentProjects())}
        >{t('recent.clear')}</button
      >{/if}
    <hr />
    <button
      disabled={!project.documentActive}
      title={t('app.report.tip')}
      onclick={() => run(onReport)}><FileText size={15} />{t('menu.report')}</button
    >
    <button disabled={!project.documentActive || project.loading} onclick={() => run(onClose)}
      ><X size={15} />{t('shell.close')}</button
    >
  </div>
</details>

<style>
  .project-menu {
    position: relative;
    flex-shrink: 0;
    font-size: 0.8rem;
  }
  summary {
    list-style: none;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    cursor: pointer;
    padding: 0.3rem 0.6rem;
    border: 1px solid var(--border);
    border-radius: 3px;
    background: var(--bg-elevated);
  }
  summary::-webkit-details-marker {
    display: none;
  }
  .commands {
    position: absolute;
    top: calc(100% + 3px);
    left: 0;
    z-index: var(--z-dropdown);
    display: flex;
    flex-direction: column;
    width: min(20rem, 85vw);
    padding: 0.3rem;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    box-shadow: 0 4px 14px #0005;
  }
  button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    text-align: left;
    background: none;
    border: 0;
    font-size: inherit;
    overflow-wrap: anywhere;
  }
  button :global(svg) {
    flex-shrink: 0;
  }
  button:hover {
    background: var(--bg-panel);
  }
  hr {
    width: 100%;
    border: 0;
    border-top: 1px solid var(--border);
    margin: 0.25rem 0;
  }
  .label {
    color: var(--text-muted);
    padding: 0.2rem 0.5rem;
    font-size: 0.75rem;
  }
</style>
