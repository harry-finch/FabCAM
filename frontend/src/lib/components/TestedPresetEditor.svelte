<script lang="ts">
  import Modal from './Modal.svelte';
  import { project } from '../state/project.svelte';
  import { inspectCutting } from '../state/cutting-workflow';
  import { attestationKey, editTestedPreset } from '../state/preset-library-v9';
  import type { TestedPreset } from '../state/preset-collections';
  let {
    initial,
    original,
    onSave,
    onClose,
  }: {
    initial: TestedPreset;
    original: TestedPreset | null;
    onSave: (value: TestedPreset) => Promise<void>;
    onClose: () => void;
  } = $props();
  // The parent remounts this editor for every explicit edit action.
  // svelte-ignore state_referenced_locally
  let draft = $state<TestedPreset>(structuredClone(initial));
  let attested = $state(''),
    error = $state(''),
    busy = $state(false);
  const key = $derived(attestationKey(draft));
  const needsAttestation = $derived(!original || attestationKey(original) !== key);
  async function useOperation() {
    const id = project.sel.selectedOpId;
    if (id == null) return;
    busy = true;
    error = '';
    try {
      const current = await inspectCutting(project, id);
      if (current.check.errors.length) throw new Error(current.check.errors.join('; '));
      if (!current.context) throw new Error(current.context_error ?? 'Complete identities first');
      draft.context = current.context;
      draft.parameters = current.parameters;
      draft.test_date = null;
      attested = '';
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  async function save() {
    busy = true;
    error = '';
    try {
      await onSave(editTestedPreset(original, draft, attested));
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
  width="min(620px, 95vw)"
  ariaLabelledBy="tested-preset-editor-title"
>
  <header>
    <h2 id="tested-preset-editor-title">Edit tested preset</h2>
    <button onclick={onClose} disabled={busy} aria-label="Close preset editor">×</button>
  </header>
  <form
    onsubmit={(e) => {
      e.preventDefault();
      void save();
    }}
  >
    <label>Name<input bind:value={draft.name} maxlength="160" required /></label>
    <p>{draft.context.machine_label} · {draft.context.material_label}</p>
    <p class="meta">
      Machine identity: {draft.context.machine_id}. This claim concerns the recorded configuration;
      it does not qualify the current project.
    </p>
    <button type="button" onclick={useOperation} disabled={busy || project.sel.selectedOpId == null}
      >Use selected operation's configuration and values</button
    >
    <div class="rates">
      <table>
        <thead
          ><tr><th>Pass / tool</th><th>rpm</th><th>Feed mm/min</th><th>Plunge mm/min</th></tr
          ></thead
        ><tbody>
          {#each draft.parameters.rates as rate (rate.role)}<tr
              ><th>{rate.role} / T{rate.tool_id}</th>
              <td
                ><input
                  aria-label={`${rate.role} spindle rpm`}
                  type="number"
                  min="1"
                  step="1"
                  bind:value={rate.rpm}
                  required
                /></td
              >
              <td
                ><input
                  aria-label={`${rate.role} feed mm/min`}
                  type="number"
                  min="1"
                  step="1"
                  bind:value={rate.feed_mm_min}
                  required
                /></td
              >
              <td
                ><input
                  aria-label={`${rate.role} plunge mm/min`}
                  type="number"
                  min="1"
                  step="1"
                  bind:value={rate.plunge_mm_min}
                  required
                /></td
              >
            </tr>{/each}
        </tbody>
      </table>
    </div>
    {#if draft.parameters.schedule.kind === 'step'}
      <label
        >Depth step (negative mm; blank when unused)<input
          type="number"
          step="any"
          max="-0.000000001"
          value={draft.parameters.schedule.step ?? ''}
          onchange={(e) => {
            if (draft.parameters.schedule.kind === 'step')
              draft.parameters.schedule.step =
                e.currentTarget.value === '' ? null : Number(e.currentTarget.value);
          }}
        /></label
      >
      <label
        >Finish depth step (negative mm; optional)<input
          type="number"
          step="any"
          max="-0.000000001"
          value={draft.parameters.schedule.finish_step ?? ''}
          onchange={(e) => {
            if (draft.parameters.schedule.kind === 'step')
              draft.parameters.schedule.finish_step =
                e.currentTarget.value === '' ? null : Number(e.currentTarget.value);
          }}
        /></label
      >
    {:else}
      <label
        >Depth schedule (descending mm, comma separated)<input
          value={draft.parameters.schedule.depths.join(', ')}
          onchange={(e) => {
            if (draft.parameters.schedule.kind === 'explicit')
              draft.parameters.schedule.depths = e.currentTarget.value
                .split(',')
                .map((v) => Number(v.trim()));
          }}
        /></label
      >
    {/if}
    {#if draft.parameters.pocket_overlap != null}<label
        >Pocket overlap (%)<input
          type="number"
          min="0.01"
          max="99.99"
          step="any"
          value={draft.parameters.pocket_overlap * 100}
          onchange={(e) => (draft.parameters.pocket_overlap = Number(e.currentTarget.value) / 100)}
        /></label
      >{/if}
    {#if draft.parameters.planar_stepover_mm != null}<label
        >Surface stepover (mm)<input
          type="number"
          min="0.0001"
          step="any"
          bind:value={draft.parameters.planar_stepover_mm}
        /></label
      >{/if}
    <label
      >Test date (optional)<input
        type="date"
        value={draft.test_date ?? ''}
        onchange={(e) => (draft.test_date = e.currentTarget.value || null)}
      /></label
    >
    <label
      >Workshop notes (optional)<textarea
        maxlength="4096"
        value={draft.notes ?? ''}
        onchange={(e) => (draft.notes = e.currentTarget.value || null)}
      ></textarea></label
    >
    {#if needsAttestation}<label class="claim"
        ><input
          type="checkbox"
          checked={attested === key}
          onchange={(e) => (attested = e.currentTarget.checked ? key : '')}
        />I confirm these exact values were tested in the recorded workshop configuration.</label
      >{:else}<p class="meta">Unchanged values retain the original author's attestation.</p>{/if}
    {#if error}<p role="alert">{error}</p>{/if}
    <footer>
      <button type="button" onclick={onClose} disabled={busy}>Cancel</button><button
        disabled={busy || (needsAttestation && attested !== key)}>Save preset</button
      >
    </footer>
  </form>
</Modal>

<style>
  header,
  footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  header {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-subtle);
  }
  h2 {
    font-size: 14px;
    margin: 0;
  }
  form {
    display: grid;
    gap: 10px;
    padding: 16px;
    overflow: auto;
    min-height: 0;
  }
  label {
    display: grid;
    gap: 4px;
    font-size: 12px;
  }
  p {
    margin: 0;
    font-size: 12px;
    overflow-wrap: anywhere;
  }
  .meta {
    color: var(--text-muted);
  }
  .claim {
    display: flex;
    align-items: flex-start;
  }
  .rates {
    overflow: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }
  th,
  td {
    padding: 4px;
    text-align: left;
  }
  td input {
    width: 100%;
    min-width: 75px;
  }
  textarea {
    min-height: 60px;
  }
</style>
