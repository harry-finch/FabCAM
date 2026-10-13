<script lang="ts">
  import { FilePlus2, FolderOpen, Import, ArrowRight } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import { isDesktop } from '../state/desktop';
  import { t } from '../i18n';
  import { openAny, addDrawing, loadSample, SAMPLES } from '../services/file_ops';
  import {
    sessionUi,
    acceptReopen,
    dismissReopen,
    openRecentProject,
  } from '../services/workspace-session.svelte';

  let { onNew, onHelp }: { onNew: () => void; onHelp: () => void } = $props();
  const recents = $derived.by(() => {
    void workspace.version;
    return workspace.get().recent_projects;
  });
</script>

<main class="welcome" aria-label={t('shell.welcome')} aria-busy={project.loading}>
  <div class="entry">
    <h1>{t('shell.brand')}</h1>
    <p class="intro">{t('shell.welcome_hint')}</p>
    <div class="actions">
      <button class="btn-primary" onclick={onNew} disabled={project.loading}
        ><FilePlus2 size={16} />{t('shell.new')}</button
      >
      <button class="btn-secondary" onclick={() => void openAny()} disabled={project.loading}
        ><FolderOpen size={16} />{t('shell.open')}</button
      >
      <button class="btn-secondary" onclick={() => void addDrawing()} disabled={project.loading}
        ><Import size={16} />{t('shell.import')}</button
      >
    </div>
    {#if project.loading}<p role="status">{project.loadingMessage ?? t('shell.loading')}</p>{/if}
    {#if sessionUi.reopenPrompt}
      <div class="reopen">
        <span>{t('shell.resume', { name: sessionUi.reopenPrompt.filename })}</span>
        <button class="btn-secondary" onclick={() => void acceptReopen()} disabled={project.loading}
          >{t('shell.reopen')}</button
        >
        <button class="btn-secondary" onclick={dismissReopen}>{t('shell.dismiss')}</button>
      </div>
    {/if}
    <h2>{t('shell.recents')}</h2>
    {#if recents.length}
      <ul>
        {#each recents.slice(0, 6) as recent (recent.path)}
          <li>
            <button
              class="recent"
              title={recent.path}
              onclick={() => void openRecentProject(recent.path)}
              disabled={project.loading}
            >
              <span>{recent.filename}<small>{recent.path}</small></span><ArrowRight size={16} />
            </button>
          </li>
        {/each}
      </ul>
    {:else}
      <p class="muted">{t('shell.no_recents')}</p>
    {/if}
    {#if !isDesktop()}<p class="muted">{t('shell.browser_files')}</p>{/if}
    <div class="secondary">
      <button
        class="btn-secondary"
        onclick={() => void loadSample(SAMPLES[0].url)}
        disabled={project.loading}>{t('shell.example')}</button
      >
      <button class="btn-secondary" onclick={onHelp}>{t('shell.getting_started')}</button>
    </div>
  </div>
</main>

<style>
  .welcome {
    flex: 1;
    min-height: 0;
    overflow: auto;
    display: grid;
    align-content: center;
    padding: 2rem;
    background: var(--bg);
  }
  .entry {
    width: min(100%, 42rem);
    margin: auto;
  }
  h1 {
    font-size: 1.8rem;
    font-weight: 600;
    margin: 0 0 0.4rem;
    color: var(--text-strong);
  }
  h2 {
    font-size: 0.85rem;
    font-weight: 600;
    margin: 2rem 0 0.6rem;
  }
  .intro,
  .muted {
    color: var(--text-muted);
    font-size: 0.82rem;
  }
  .actions,
  .secondary,
  .reopen {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }
  .actions {
    margin-top: 1.3rem;
  }
  .secondary {
    margin-top: 1.5rem;
  }
  .reopen {
    margin-top: 1.2rem;
  }
  button {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    font-size: 0.82rem;
  }
  ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .recent {
    width: 100%;
    justify-content: space-between;
    border: 0;
    border-bottom: 1px solid var(--border);
    border-radius: 0;
    padding: 0.65rem 0;
    background: none;
    text-align: left;
  }
  .recent span {
    min-width: 0;
    overflow-wrap: anywhere;
  }
  .recent small {
    display: block;
    color: var(--text-muted);
    font-size: 0.75rem;
  }
  .recent :global(svg) {
    flex-shrink: 0;
  }
  .secondary button {
    background: none;
    border-color: transparent;
    color: var(--text-muted);
  }
  @media (max-width: 600px) {
    .welcome {
      padding: 1rem;
      align-content: start;
    }
  }
</style>
