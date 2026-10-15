<script lang="ts">
  import { t } from '../i18n';
  import { project } from '../state/project.svelte';
  import { workspace } from '../state/workspace.svelte';
  import { duplicateProfile, profileFromCurrent } from '../state/machine_profiles';
  import { assignmentReview, newMachineDraft } from '../state/machine-profile-review';
  import { machineIdentity } from '../state/machine-identity';
  import { suggestMachineName } from '../state/tool_naming';
  import type { MachineSettings } from '../state/project-types';
  import type { MachineProfile } from '../state/workspace';
  import MachineDialog from './MachineDialog.svelte';
  import MachineTooling from './MachineTooling.svelte';

  const profiles = $derived.by(() => {
    void workspace.version;
    return workspace.get().machine_profiles;
  });
  let selectedId = $state<string | null>(null);
  const selected = $derived(profiles.find((p) => p.id === selectedId));
  const isProject = $derived(selectedId === null || selectedId === project.data.machineProfileId);
  const source = $derived(isProject ? project.data.machine : selected?.machine);
  const title = $derived(
    selected?.name || project.data.machine.name?.trim() || suggestMachineName(project.data.machine),
  );
  const viewedIdentity = $derived(machineIdentity(title, title));
  const assigned = $derived(profiles.find((p) => p.id === project.data.machineProfileId));
  let section = $state<'general' | 'output' | 'tools' | 'advanced'>('general');
  let editor = $state<MachineDialog>();
  let dirty = $state(false);
  let conflict = $state(false);
  let pending = $state<(() => void) | null>(null);
  let assignment = $state<MachineProfile | null>(null);
  let deleting = $state(false);
  let query = $state('');
  let creating = $state(false);
  let step = $state(0);
  let fresh = $state<MachineSettings>(newMachineDraft());
  const shown = $derived(
    profiles.filter((p) => p.name.toLowerCase().includes(query.toLowerCase())),
  );
  const impacts = $derived(
    assignment ? assignmentReview(assignment, project.data.tools, project.data.operations) : [],
  );

  function guarded(action: () => void) {
    if (creating) return;
    assignment = null;
    deleting = false;
    if (dirty) pending = action;
    else action();
  }
  function finishPending(save: boolean) {
    if (save && !editor?.saveChanges()) return;
    if (!save) editor?.discardChanges();
    const action = pending;
    pending = null;
    action?.();
  }
  function inspect(id: string | null) {
    guarded(() => {
      selectedId = id;
      section = 'general';
    });
  }
  function save(machine: MachineSettings) {
    if (isProject) project.setMachine(machine);
    else if (selected)
      workspace.upsertMachineProfile({
        ...$state.snapshot(selected),
        name: machine.name?.trim() || suggestMachineName(machine),
        machine,
      });
    else throw new Error('The inspected profile no longer exists.');
  }
  function copyCurrent() {
    const p = profileFromCurrent(
      $state.snapshot(project.data.machine),
      $state.snapshot(project.data.tools),
      profiles,
    );
    workspace.upsertMachineProfile(p);
    selectedId = p.id;
  }
  function duplicate() {
    if (!selected) return;
    const p = duplicateProfile($state.snapshot(selected), profiles);
    workspace.upsertMachineProfile(p);
    selectedId = p.id;
  }
  function remove() {
    if (!selected) return;
    workspace.deleteMachineProfile(selected.id);
    if (selected.id === project.data.machineProfileId) project.detachMachineProfile();
    deleting = false;
    selectedId = null;
  }
  function create() {
    const p = profileFromCurrent($state.snapshot(fresh), [], profiles);
    workspace.upsertMachineProfile(p);
    creating = false;
    selectedId = p.id;
    section = 'general';
  }
  function navigate(event: KeyboardEvent) {
    if (!['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const buttons = Array.from(
      event.currentTarget instanceof HTMLElement
        ? event.currentTarget.parentElement!.querySelectorAll<HTMLButtonElement>('[data-profile]')
        : [],
    );
    const index = buttons.indexOf(event.target as HTMLButtonElement);
    const next =
      event.key === 'Home'
        ? 0
        : event.key === 'End'
          ? buttons.length - 1
          : Math.max(0, Math.min(buttons.length - 1, index + (event.key === 'ArrowDown' ? 1 : -1)));
    buttons[next]?.focus();
  }
</script>

<div class="machine-ws">
  <aside aria-label={t('ux6.machine_profiles')}>
    <h2>{t('ux6.machine_profiles')}</h2>
    <input
      aria-label={t('ux6.find_a_machine_profile')}
      placeholder={t('ux6.find_profile')}
      bind:value={query}
    />
    <div
      class="profile-list"
      role="group"
      aria-label="Inspect profiles; Enter selects, arrows move focus"
    >
      <button
        data-profile
        disabled={creating}
        onkeydown={navigate}
        aria-pressed={selectedId === null}
        onclick={() => inspect(null)}
        >{t('ux6.current_project')}<small
          >{project.data.machine.name?.trim() || suggestMachineName(project.data.machine)}</small
        ></button
      >
      {#each shown as p (p.id)}
        <button
          data-profile
          disabled={creating}
          onkeydown={navigate}
          aria-pressed={selectedId === p.id}
          title={p.name}
          onclick={() => inspect(p.id)}
          ><span>{p.name}</span>{#if p.id === project.data.machineProfileId}<small
              >{t('ux6.assigned_to_project')}</small
            >{/if}</button
        >
      {/each}
      {#if shown.length === 0}<p>{t('ux6.no_matching_saved_profiles')}</p>{/if}
    </div>
    <button
      disabled={creating}
      onclick={() =>
        guarded(() => {
          fresh = newMachineDraft();
          step = 0;
          creating = true;
        })}>{t('ux6.new_profile')}</button
    >
    <button
      disabled={creating}
      title={t('ux6.copy_committed_machine_settings_and_tools_does_not_assign')}
      onclick={() => guarded(copyCurrent)}>{t('ux6.copy_current_setup')}</button
    >
    <p>{t('ux6.profiles_are_stored_on_this_installation_project_files_keep')}</p>
  </aside>
  <main class="detail">
    <header>
      <div>
        <h2 {title}>{title}</h2>
        {#if viewedIdentity.limitation}<small class="limitation">{viewedIdentity.limitation}</small
          >{/if}
        <p>
          {t('ux6.project_assignment')}<strong
            >{assigned?.name ??
              (project.data.machineProfileId
                ? 'Missing local profile — embedded settings retained'
                : 'Project-local settings')}</strong
          >
        </p>
      </div>
      {#if selected && !isProject}<button
          class="btn-primary"
          disabled={creating}
          onclick={() =>
            guarded(() => {
              assignment = $state.snapshot(selected);
            })}>{t('ux6.use_for_this_project')}</button
        >{/if}
      {#if selected}<button disabled={creating} onclick={() => guarded(duplicate)}
          >{t('oplist.duplicate.title')}</button
        ><button
          disabled={creating}
          onclick={() =>
            guarded(() => {
              deleting = true;
            })}>{t('ux6.delete')}</button
        >{/if}
      {#if isProject && project.data.machineProfileId}<button
          disabled={creating}
          title="Keep all current machine settings and tools; stop profile write-back."
          onclick={() => guarded(() => project.detachMachineProfile())}
          >{t('ux6.detach_profile')}</button
        >{/if}
    </header>
    {#if pending}
      <div class="notice" role="alert">
        <span>Unsaved settings for {title}.</span><button onclick={() => (pending = null)}
          >{t('post.keep_editing')}</button
        ><button onclick={() => finishPending(false)}>{t('ux6.discard_and_continue')}</button
        ><button disabled={conflict} onclick={() => finishPending(true)}
          >{isProject ? 'Apply' : 'Save profile'} and continue</button
        >
      </div>
    {/if}
    {#if deleting}
      <div class="notice" role="alert">
        <span>{t('ux6.delete_this_local_profile_embedded_project_settings_and_tools')}</span><button
          onclick={() => (deleting = false)}>{t('ux6.keep_profile')}</button
        ><button class="btn-danger" onclick={remove}>{t('ux6.delete_local_profile')}</button>
      </div>
    {/if}
    {#if assignment}
      <div class="notice" role="region" aria-label={t('ux6.review_profile_assignment')}>
        <p>
          Replace project machine settings and all {project.data.tools.length} tools with {assignment
            .tools.length} tools from <strong>{assignment.name}</strong>{t(
            'ux6.saved_cutting_values_and_operation_tool_ids_stay_unchanged',
          )}
        </p>
        {#if impacts.length}<ul>
            {#each impacts as impact, i (i)}<li>
                {impact.name}: T{impact.toolId} — {impact.reason === 'missing'
                  ? 'not in this profile'
                  : 'different tool definition'}
              </li>{/each}
          </ul>{/if}
        <button onclick={() => (assignment = null)}>{t('ux6.cancel_assignment')}</button><button
          class="btn-primary"
          disabled={creating}
          onclick={() => {
            if (assignment) project.applyMachineProfile(assignment);
            assignment = null;
          }}>{t('ux6.use_for_this_project_2')}</button
        >
      </div>
    {/if}
    {#if creating}
      <section class="creator" aria-label={t('ux6.new_machine_profile')}>
        <h3>New profile · {step + 1} / 4</h3>
        {#if step === 0}
          <label>{t('ux2.fixture.name')}<input bind:value={fresh.name} /></label>
          <label
            >{t('ux6.primary_process')}<select bind:value={fresh.mode}
              ><option value="mill">{t('ux6.milling')}</option><option value="laser"
                >{t('tools.laser')}</option
              ><option value="drag">{t('ux6.drag_knife')}</option><option value="plasma"
                >{t('tools.plasma')}</option
              ></select
            ></label
          >
        {:else if step === 1}
          <label
            >{t('ux6.program_output')}<select bind:value={fresh.gcodeDialect}
              ><option value="linuxcnc">{t('ux6.linuxcnc')}</option><option value="grbl"
                >GRBL</option
              ><option value="hpgl">HPGL</option></select
            ></label
          >
          <p>{t('ux6.choose_an_existing_output_format_for_shopbot_or_a')}</p>
        {:else if step === 2}
          <p>
            Default workspace — review against your machine. All dimensions in mm; RPM/feed limits
            remain unknown until entered in General.
          </p>
          {#each ['x', 'y', 'z'] as axis (axis)}{@const key = axis as 'x' | 'y' | 'z'}<label
              >Workspace {axis.toUpperCase()}
              <input type="number" min="0.001" bind:value={fresh.workArea![key]} />{t(
                'machine.unit.mm',
              )}</label
            >{/each}
        {:else}
          <p>
            <strong>{fresh.name || suggestMachineName(fresh)}</strong> · {fresh.mode} · {fresh.gcodeDialect}
          </p>
          <p>
            Workspace {fresh.workArea?.x} × {fresh.workArea?.y} × {fresh.workArea?.z} mm. No tools stocked.
            Limits unknown. No machine validation is implied.
          </p>
          <p>{t('ux6.save_creates_a_local_profile_assignment_to_this_project')}</p>
        {/if}
        <div class="actions">
          <button onclick={() => (creating = false)}>{t('ux6.cancel_creation')}</button
          >{#if step > 0}<button onclick={() => step--}>{t('opprops.side.back')}</button
            >{/if}{#if step < 3}<button
              disabled={step === 2 &&
                !Object.values(fresh.workArea ?? {}).every((v) => Number.isFinite(v) && v > 0)}
              onclick={() => step++}>{t('ux6.next')}</button
            >{:else}<button class="btn-primary" onclick={create}>{t('ux6.save_new_profile')}</button
            >{/if}
        </div>
      </section>
    {:else if source}
      <nav aria-label={t('machinews.sections.aria')}>
        {#each [['general', 'General'], ['output', 'Program output'], ['tools', 'Tools'], ['advanced', 'Advanced']] as item (item[0])}<button
            aria-pressed={section === item[0]}
            onclick={() => (section = item[0] as typeof section)}>{item[1]}</button
          >{/each}
      </nav>
      <p class="scope">
        {isProject
          ? assigned
            ? 'Apply updates this project and its linked local profile.'
            : 'Apply updates this project only.'
          : 'Save updates the inspected profile only. The project stays unchanged.'}
      </p>
      {#key selectedId}
        <div class="editor" class:hidden={section === 'tools'}>
          <MachineDialog
            bind:this={editor}
            embedded
            open={false}
            onClose={() => {}}
            sourceMachine={source}
            scopeKey={isProject
              ? `project:${project.data.machineProfileId ?? 'local'}`
              : `profile:${selectedId}`}
            inheritedIdentity={isProject ? project.data.fsContext?.machine_identity : undefined}
            onSaveMachine={save}
            saveLabel={isProject
              ? assigned
                ? 'Apply to project and linked profile'
                : 'Apply to project'
              : 'Save profile'}
            section={section === 'tools' ? 'general' : section}
            onDraftState={(d, c) => {
              dirty = d;
              conflict = c;
              if (d) {
                assignment = null;
                deleting = false;
              }
            }}
          />
        </div>
      {/key}
      {#if section === 'tools'}
        {#if isProject}<MachineTooling />{:else}<section class="tool-summary">
            <h3>Profile tool copies · {selected?.tools.length ?? 0}</h3>
            <p>{t('ux6.these_are_saved_copies_assign_this_profile_to_stock')}</p>
            <ul>
              {#each selected?.tools ?? [] as tool (tool.id)}<li>
                  #{tool.id}
                  {tool.name} · {tool.kind} · Ø{tool.diameter} mm
                </li>{/each}
            </ul>
          </section>{/if}
      {/if}
    {:else}<p class="notice">
        {t('ux6.this_local_profile_is_no_longer_available_select_current')}
      </p>{/if}
  </main>
</div>

<style>
  .machine-ws {
    display: grid;
    grid-template-columns: 220px minmax(0, 1fr);
    flex: 1;
    min-height: 0;
    background: var(--bg-panel);
  }
  aside {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-height: 0;
    padding: 12px;
    border-right: 1px solid var(--border);
  }
  h2 {
    font-size: 0.9rem;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  h3 {
    font-size: 0.85rem;
  }
  p {
    font-size: 0.78rem;
    line-height: 1.45;
    color: var(--text-muted);
    margin: 4px 0;
  }
  button,
  input,
  select {
    min-height: 28px;
    font-size: 0.8rem;
  }
  button {
    cursor: pointer;
  }
  .profile-list {
    overflow: auto;
    flex: 1;
    min-height: 120px;
  }
  .profile-list button {
    text-align: left;
    width: 100%;
    background: transparent;
    border: 1px solid transparent;
    padding: 7px;
    margin: 2px 0;
  }
  .profile-list span,
  small {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  small {
    font-size: 0.72rem;
    color: var(--text-muted);
    margin-top: 3px;
  }
  button[aria-pressed='true'] {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 12%, var(--bg-panel));
  }
  .detail {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
  }
  header {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
    padding: 12px;
    border-bottom: 1px solid var(--border);
  }
  header > div {
    flex: 1 1 300px;
    min-width: 0;
  }
  nav {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 8px 12px;
    border-bottom: 1px solid var(--border);
  }
  .scope {
    padding: 4px 12px;
  }
  .editor {
    display: flex;
    flex: 1;
    min-height: 0;
  }
  .hidden {
    display: none;
  }
  .notice {
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elevated);
    font-size: 0.8rem;
    max-height: 220px;
    overflow: auto;
  }
  .notice button {
    margin: 4px 6px 0 0;
  }
  .creator,
  .tool-summary {
    padding: 16px;
    max-width: 650px;
    overflow: auto;
  }
  .creator label {
    display: flex;
    align-items: center;
    gap: 10px;
    margin: 10px 0;
    font-size: 0.8rem;
  }
  .creator input[type='number'] {
    width: 96px;
  }
  .actions {
    display: flex;
    gap: 8px;
    margin-top: 16px;
  }
  @media (max-width: 1050px) {
    .machine-ws {
      grid-template-columns: 200px minmax(0, 1fr);
    }
  }
  button:not(.btn-primary):not(.btn-danger) {
    background: var(--bg-elevated);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 4px 8px;
  }
  .profile-list button[aria-pressed='true'],
  nav button:not(.btn-primary):not(.btn-danger)[aria-pressed='true'] {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 12%, var(--bg-panel));
  }
  nav button[aria-pressed='true'] {
    box-shadow: inset 0 -2px 0 var(--accent);
    color: var(--text-strong);
  }
  .limitation {
    color: var(--warning);
  }
  header strong {
    margin-left: 4px;
  }
</style>
