<script lang="ts">
  import { CircleX, ScanSearch } from '@lucide/svelte';
  import { project } from '../state/project.svelte';
  import { fixtureCollisions } from '../state/fixture-collisions';
  import { fixtureFocus } from '../state/fixture-focus.svelte';
  import { t } from '../i18n';
  let { stale }: { stale: boolean } = $props();
  const collisions = $derived(
    fixtureCollisions(
      project.data.fixtures,
      project.gen.generated?.warnings ?? [],
      project.gen.simDiagnostics?.warnings ?? [],
      stale,
    ),
  );
  function show() {
    const first = collisions[0];
    project.selectFixture(first.fixtureId);
    fixtureFocus.focus(first.fixtureId);
  }
</script>

{#if collisions.length}
  <div class="collision-feedback" role="alert">
    <CircleX size={16} aria-hidden="true" />
    <strong>{t('ux21.collision')}</strong>
    <span>{t('ux21.reason', { name: collisions.map((c) => c.name).join(', ') })}</span>
    {#if project.gen.generated?.warnings?.some((w) => w.kind === 'fixture_path_collision')}
      <span>{t('ux21.blocked')}</span>
    {/if}
    <button type="button" onclick={show} title={t('ux21.show')}>
      <ScanSearch size={14} aria-hidden="true" />{t('ux21.show')}
    </button>
  </div>
{/if}

<style>
  .collision-feedback {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--ui-space-2);
    padding: var(--ui-space-2) var(--ui-space-3);
    border-bottom: 1px solid var(--error);
    color: var(--error);
    background: var(--bg-panel);
    font-size: var(--inspector-font-field);
  }
  button {
    display: inline-flex;
    align-items: center;
    gap: var(--ui-space-1);
    margin-left: auto;
  }
</style>
