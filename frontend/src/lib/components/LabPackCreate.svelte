<script lang="ts">
  import { t } from '../i18n';
  import { createLabPack } from '../state/lab-presets';
  import { persistLabPack } from '../state/lab-presets-store';
  import type { SourcePack } from '../state/feeds-speeds-sources';
  let { onCreated }: { onCreated?: (pack: SourcePack) => void } = $props();
  let open = $state(false),
    name = $state(''),
    busy = $state(false),
    error = $state('');
  async function create() {
    busy = true;
    error = '';
    try {
      const pack = createLabPack(name, new Date().toISOString(), `lab-${crypto.randomUUID()}`);
      await persistLabPack(pack, null);
      onCreated?.(pack);
      name = '';
      open = false;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

{#if open}
  <form
    onsubmit={(e) => {
      e.preventDefault();
      void create();
    }}
  >
    <label
      >{t('v11.pack_name')}<input
        bind:value={name}
        required
        maxlength="100"
        disabled={busy}
      /></label
    >
    <p>{t('v11.pack_private')}</p>
    <div class="actions">
      <button
        class="btn-secondary"
        type="button"
        disabled={busy}
        onclick={() => {
          open = false;
          error = '';
        }}>{t('common.cancel')}</button
      >
      <button class="btn-primary" type="submit" disabled={busy || !name.trim()}
        >{t('v11.pack_create')}</button
      >
    </div>
    {#if error}<p role="alert">{error}</p>{/if}
  </form>
{:else}
  <button class="btn-secondary" type="button" onclick={() => (open = true)}
    >{t('v11.pack_create')}</button
  >
{/if}

<style>
  button {
    font-size: var(--inspector-font-field);
    padding: var(--ui-space-1) var(--ui-space-2);
    min-height: var(--inspector-control-height);
  }
  form {
    display: grid;
    gap: var(--ui-space-2);
    margin: var(--ui-space-2) 0;
  }
  label {
    display: grid;
    gap: var(--ui-space-1);
    font-size: var(--inspector-font-field);
  }
  input {
    width: 100%;
    box-sizing: border-box;
    font: inherit;
  }
  p {
    margin: 0;
    color: var(--text-muted);
    font-size: var(--inspector-font-meta);
  }
  p[role='alert'] {
    color: var(--danger);
  }
  .actions {
    display: flex;
    gap: var(--ui-space-2);
  }
</style>
