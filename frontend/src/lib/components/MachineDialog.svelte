<script lang="ts">
  /// Machine settings dialog. Project-scoped CNC config — units, mode,
  /// fast move height, comments / arcs / toolchange flags. The op-driven
  /// pipeline reads these from the Project; until that wires up the
  /// values are also mirrored into setup.machine via SetupPanel so the
  /// legacy Generate path keeps working.
  import {
    project,
    type AxisLimits,
    type MachineSettings,
    type PostProfile,
  } from '../state/project.svelte';
  import { untrack } from 'svelte';
  import Modal from './Modal.svelte';
  import FeedsSpeedsCompatibility from './FeedsSpeedsCompatibility.svelte';
  import { deepEqual } from './dialog-draft';
  import PostProcessorEditor from './PostProcessorEditor.svelte';
  import CpsPostConfigEditor from './CpsPostConfig.svelte';
  import { DialogDraft } from './dialog-draft.svelte';
  import * as fileOps from '../services/file_ops';
  import { suggestMachineName } from '../state/tool_naming';
  import { t } from '../i18n';
  import { defaultClient } from '../api/http';
  import type { VersionResponse } from '../api/types';

  interface Props {
    open: boolean;
    onClose: () => void;
    /// Render as a first-class tab panel instead of a modal: no Modal
    /// wrapper, no × / Cancel (the component stays mounted across tab
    /// switches, so an in-progress draft survives), footer becomes
    /// Apply / Revert.
    embedded?: boolean;
    sourceMachine?: MachineSettings;
    onSaveMachine?: (machine: MachineSettings) => void;
    saveLabel?: string;
    section?: 'general' | 'output' | 'advanced' | 'all';
    scopeKey?: string;
    inheritedIdentity?: MachineSettings['fsIdentity'];
    onDraftState?: (dirty: boolean, conflict: boolean) => void;
  }
  let {
    open,
    onClose,
    embedded = false,
    sourceMachine,
    onSaveMachine,
    saveLabel,
    section = 'all',
    scopeKey = 'project',
    inheritedIdentity,
    onDraftState,
  }: Props = $props();
  const source = $derived(sourceMachine ?? project.data.machine);
  let baseline = $state<MachineSettings>(untrack(() => $state.snapshot(source)));
  $effect(() => {
    onDraftState?.(dd.isDirty, conflict);
  });
  export function discardChanges() {
    revert();
  }
  export function saveChanges(): boolean {
    return commit();
  }
  let baselineScope = $state(untrack(() => scopeKey));
  const active = $derived(open || embedded);

  // PostProcessor editor — the heavy editing surface lives in a
  // dedicated dialog with a live preview pane and JSON I/O. We just
  // own the "is it open" flag.
  let editorOpen = $state(false);

  /// The dialog edits two top-level pieces — the machine settings plus
  /// the jerk fields (a single enable checkbox + AxisLimits, default
  /// off: trapezoidal profile only; S-curve refinement is Phase 2) —
  /// so the DialogDraft wraps them as one composite. One draft, one
  /// pristine snapshot, one dirty check covering both.
  interface MachineDraft {
    machine: MachineSettings;
    jerkEnabled: boolean;
    jerk: AxisLimits;
  }

  function compositeOf(m: MachineSettings): MachineDraft {
    return {
      machine: cloneSettings(m),
      jerkEnabled: !!m.jerk,
      jerk: m.jerk ? { ...m.jerk } : { x: 100, y: 100, z: 50 },
    };
  }

  // The .cps option exists only when the active transport's build
  // carries the runtime (wasm ships it opt-in) — probe /version once.
  let cpsSupported = $state(false);
  $effect(() => {
    let cancelled = false;
    void defaultClient()
      .version()
      .then((v: VersionResponse) => {
        if (!cancelled) cpsSupported = (v.capabilities ?? []).includes('post-cps');
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  });

  const dd = new DialogDraft<MachineDraft>();
  const conflict = $derived(
    dd.isDirty && (!deepEqual(source, baseline) || baselineScope !== scopeKey),
  );
  dd.open(compositeOf(untrack(() => source)));
  /// Narrow aliases so the template / commit path keep reading
  /// `draft.*` / `jerkDraft.*` — two-way bindings still mutate the
  /// deeply-reactive dd.draft underneath. The `??` arms are unreachable
  /// (dd is opened at init and never close()d) but keep the types
  /// null-free.
  const composite = $derived(dd.draft ?? compositeOf(source));
  const draft = $derived(composite.machine);
  const isShopBot = $derived(
    draft.gcodeDialect === 'cps' &&
      draft.cpsPost?.source === 'bundled' &&
      draft.cpsPost.bundledId === 'shopbot',
  );
  const jerkDraft = $derived(composite.jerk);

  $effect(() => {
    if (!active) return;
    // Tracked dep: ONLY the committed machine (deep snapshot). The
    // guard + dd.open below run untracked — reading dd.isDirty
    // (deep-reads dd.draft) and then writing dd.draft in the same
    // effect self-invalidates it into an infinite loop (the frozen-
    // tab bug, see ToolLibraryDialog's twin effect).
    const machine = $state.snapshot(source) as MachineSettings;
    const owner = scopeKey;
    untrack(() => {
      // Embedded panels stay mounted, so external machine changes
      // (undo, profile switch) re-run this — refresh a CLEAN draft to
      // stay in sync, but never clobber in-progress edits.
      if (embedded && dd.isDirty) return;
      // try/catch so a throw from cloning can't propagate into the
      // Svelte 5 reactivity scheduler and abort it — a dead scheduler
      // manifests as "every button still fires its onclick, but
      // visible state stops updating" (see App.svelte:117).
      try {
        dd.open(compositeOf(machine));
        baseline = machine;
        baselineScope = owner;
      } catch (e) {
        console.error('MachineDialog.open: init failed', e);
      }
    });
  });

  function cloneSettings(m: MachineSettings): MachineSettings {
    return {
      ...m,
      // Checkbox bindings coerce absent optional booleans to false. Seed the
      // pristine draft with those same values so mounting the hidden panel
      // cannot mark it dirty and prevent later project/profile refreshes.
      optionalStop: m.optionalStop ?? false,
      laserDynamicPower: m.laserDynamicPower ?? false,
      plotModeZ: m.plotModeZ ?? false,
      parkAtHome: m.parkAtHome ?? false,
      rapidSpeed: m.rapidSpeed,
      toolchangeS: m.toolchangeS,
      accel: m.accel ? { ...m.accel } : { x: 250, y: 250, z: 250 },
      jerk: m.jerk ? { ...m.jerk } : undefined,
      workArea: m.workArea ? { ...m.workArea } : { x: 200, y: 300, z: 50 },
      postProfile: m.postProfile ? { ...m.postProfile } : undefined,
    };
  }

  /// Map the current `postProfile` to one of our preset keys so the
  /// preset dropdown reflects what the user picked. Matched by NAME,
  /// not exact equality — the user might tweak templates on top of
  /// 'Mach3 metric' and we still want the dropdown to show Mach3 (so
  /// the next tweak doesn't snap it back to the canonical preset).
  /// Map the current `postProfile` to one of our preset keys. Match
  /// the NAME *and* check that the user hasn't tweaked anything on
  /// top of the preset — if they edited a template, the dropdown
  /// should snap to "custom" so users can tell they've diverged from
  /// the canonical preset (was: name-only match, which silently
  /// claimed a heavily-edited profile was still "LinuxCNC default").
  function profilePreset(p: PostProfile | undefined): string {
    if (!p) return 'none';
    const matches = (a: PostProfile, b: PostProfile): boolean =>
      a.file_extension === b.file_extension &&
      (a.line_ending ?? null) === (b.line_ending ?? null) &&
      (a.program_start ?? null) === (b.program_start ?? null) &&
      (a.program_end ?? null) === (b.program_end ?? null) &&
      (a.tool_change ?? null) === (b.tool_change ?? null) &&
      (a.coolant_flood_on ?? null) === (b.coolant_flood_on ?? null) &&
      (a.coolant_flood_off ?? null) === (b.coolant_flood_off ?? null) &&
      (a.coolant_mist_on ?? null) === (b.coolant_mist_on ?? null) &&
      (a.coolant_mist_off ?? null) === (b.coolant_mist_off ?? null) &&
      !a.axes === !b.axes;
    const presets: { key: string; profile: PostProfile }[] = [
      {
        key: 'linuxcnc',
        profile: { name: 'LinuxCNC default', file_extension: 'nc', line_ending: '\n' },
      },
      {
        key: 'mach3',
        profile: {
          name: 'Mach3 metric',
          file_extension: 'tap',
          line_ending: '\r\n',
          program_start: '%\nN10 G21 G90 (ivac <version>)',
          program_end: 'M30\n%',
        },
      },
      {
        key: 'grbl',
        profile: {
          name: 'GRBL default',
          file_extension: 'nc',
          line_ending: '\n',
          program_start: '; ivac <version> — GRBL',
          program_end: 'M2',
          tool_change: '; toolchange to T<t> (manual on GRBL)',
        },
      },
    ];
    for (const { key, profile } of presets) {
      if (p.name === profile.name && matches(p, profile)) return key;
    }
    return 'custom';
  }

  /// One-line summary of what's been customized in the current
  /// profile, shown as a "X tweaks: header, footer, axes" chip next
  /// to the Edit button so users don't have to open the editor to
  /// see whether they have any overrides.
  function profileTweakSummary(p: PostProfile | undefined): string {
    if (!p) return '';
    const parts: string[] = [];
    if (p.program_start) parts.push(t('machine.pp.tweak.header'));
    if (p.program_end) parts.push(t('machine.pp.tweak.footer'));
    if (p.tool_change) parts.push(t('machine.pp.tweak.toolchange'));
    if (p.coolant_flood_on || p.coolant_flood_off) parts.push(t('machine.pp.tweak.coolant'));
    if (p.coolant_mist_on || p.coolant_mist_off) parts.push(t('machine.pp.tweak.mist'));
    if (p.axes) parts.push(t('machine.pp.tweak.axes'));
    if (p.file_extension || p.line_ending) parts.push(t('machine.pp.tweak.file'));
    return parts.join(', ');
  }

  /// Plain MachineSettings from the composite draft — the exact value
  /// commit() hands to setMachine, also reused by "Save as profile".
  /// $state.snapshot strips the proxy (Svelte 5 proxies can trip
  /// `structuredClone` on some WebKit builds, silently aborting the
  /// commit); jerk folds back in from its enable toggle; a non-empty
  /// capability set is normalized to contain the primary mode (the
  /// emitter targets it; Rust's effective-capability logic reads
  /// capabilities INSTEAD of mode when non-empty — and the disabled
  /// primary checkbox never fires onchange, so a mode change after
  /// customizing capabilities can leave the draft set lacking it).
  function draftSnapshot(): MachineSettings {
    const snap = $state.snapshot(draft) as MachineSettings;
    snap.jerk = composite.jerkEnabled ? { ...jerkDraft } : undefined;
    if (snap.capabilities && snap.capabilities.length > 0) {
      snap.capabilities = [...new Set([...snap.capabilities, snap.mode])];
    }
    return snap;
  }

  function commit(): boolean {
    if (conflict) return false;
    try {
      const next = draftSnapshot();
      if (onSaveMachine) onSaveMachine(next);
      else project.setMachine(next);
      baseline = $state.snapshot(next);
      dd.markClean();
      if (!embedded) onClose();
      return true;
    } catch (e) {
      project.setError(`Machine settings: ${String(e)}`);
      return false;
    }
  }

  function revert() {
    baseline = $state.snapshot(source);
    baselineScope = scopeKey;
    dd.open(compositeOf(untrack(() => source)));
  }

  /// Close protocol (dd.requestClose): the first attempt on a dirty
  /// draft arms the inline "Discard / Keep editing" footer pair; the
  /// second confirms. The inline bar replaces the prior `window.confirm`
  /// prompt, which silently returns false in some Tauri / WebKitGTK
  /// builds — the project's audit-C10 note already flagged
  /// `window.confirm` as unreliable for this reason.
  function close() {
    if (dd.requestClose()) onClose();
  }
</script>

{#snippet shell()}
  {#if !embedded}
    <header>
      <h2 id="machine-title">{t('machine.title')}</h2>
      <button class="dlg-close" onclick={close} aria-label={t('common.close')}>×</button>
    </header>
  {/if}

  <p class="storage-note">{t('ux6.dimensions_stay_in_mm_output_units_are_set_in')}</p>

  {#if conflict}
    <div class="conflict" role="alert">
      {t('ux6.committed_settings_changed_while_you_were_editing_reload_current')}<button
        class="btn-secondary"
        onclick={revert}>{t('ux6.reload_current_settings')}</button
      >
    </div>
  {/if}
  <div class="grid">
    {#if section === 'general' || section === 'all'}
      <label title={t('machine.name.label_title')}>
        {t('machine.name')}
        <input
          type="text"
          placeholder={suggestMachineName(draft)}
          value={draft.name ?? ''}
          title={t('machine.name.input_title')}
          oninput={(e) => (draft.name = (e.currentTarget as HTMLInputElement).value)}
        />
      </label>
      <label title={t('machine.mode.title')}>
        {t('machine.mode')}
        <select bind:value={draft.mode}>
          <option value="mill">{t('machine.mode.mill')}</option>
          <option value="laser">{t('machine.mode.laser')}</option>
          <option value="drag">{t('machine.mode.drag')}</option>
          <option value="plasma">{t('machine.mode.plasma')}</option>
        </select>
      </label>
      <fieldset class="capabilities">
        <legend title={t('machine.capabilities.legend_title')}>
          {t('machine.capabilities')}
        </legend>
        {#each ['mill', 'laser', 'drag', 'plasma'] as cap (cap)}
          {@const isPrimary = cap === draft.mode}
          <!-- The primary mode is always a capability (the gcode
               emitter targets it), so its checkbox is DISABLED rather
               than silently re-added on commit — unchecking it and
               watching it come back on reopen reads as a bug. -->
          <label class="cap-toggle">
            <input
              type="checkbox"
              disabled={isPrimary}
              title={isPrimary ? t('machine.capabilities.primary_title') : ''}
              checked={isPrimary ||
                (draft.capabilities ?? [draft.mode]).includes(
                  cap as 'mill' | 'laser' | 'drag' | 'plasma',
                )}
              onchange={(e) => {
                const on = (e.currentTarget as HTMLInputElement).checked;
                const cur = new Set(draft.capabilities ?? [draft.mode]);
                if (on) cur.add(cap as 'mill' | 'laser' | 'drag' | 'plasma');
                else cur.delete(cap as 'mill' | 'laser' | 'drag' | 'plasma');
                // Belt-and-braces: keep the primary mode in the set so
                // the gcode emitter never targets a removed capability
                // (the checkbox above is disabled, but a stale draft
                // could still lack it).
                cur.add(draft.mode);
                draft.capabilities = [...cur];
              }}
            />
            <span
              >{cap === 'mill'
                ? t('machine.cap.mill')
                : cap === 'laser'
                  ? t('machine.cap.laser')
                  : cap === 'drag'
                    ? t('machine.cap.drag')
                    : t('machine.cap.plasma')}</span
            >
          </label>
        {/each}
      </fieldset>
      <label title={t('machine.fast_move_z.title')}>
        {t('machine.fast_move_z')}
        <span class="field"
          ><input type="number" bind:value={draft.fastMoveZ} step="0.1" /><span class="unit"
            >mm</span
          ></span
        >
      </label>
      <fieldset class="work-area">
        <legend title={t('machine.work_area.legend_title')}>{t('machine.work_area')}</legend>
        <label
          >X
          <span class="field"
            ><input
              type="number"
              min="1"
              step="10"
              value={draft.workArea?.x ?? 200}
              oninput={(e) => {
                const v = (e.target as HTMLInputElement).valueAsNumber;
                if (Number.isFinite(v) && v > 0) {
                  draft.workArea = {
                    x: v,
                    y: draft.workArea?.y ?? 300,
                    z: draft.workArea?.z ?? 50,
                  };
                }
              }}
            /><span class="unit">mm</span></span
          >
        </label>
        <label
          >Y
          <span class="field"
            ><input
              type="number"
              min="1"
              step="10"
              value={draft.workArea?.y ?? 300}
              oninput={(e) => {
                const v = (e.target as HTMLInputElement).valueAsNumber;
                if (Number.isFinite(v) && v > 0) {
                  draft.workArea = {
                    x: draft.workArea?.x ?? 200,
                    y: v,
                    z: draft.workArea?.z ?? 50,
                  };
                }
              }}
            /><span class="unit">mm</span></span
          >
        </label>
        <label
          >Z
          <span class="field"
            ><input
              type="number"
              min="1"
              step="5"
              value={draft.workArea?.z ?? 50}
              oninput={(e) => {
                const v = (e.target as HTMLInputElement).valueAsNumber;
                if (Number.isFinite(v) && v > 0) {
                  draft.workArea = {
                    x: draft.workArea?.x ?? 200,
                    y: draft.workArea?.y ?? 300,
                    z: v,
                  };
                }
              }}
            /><span class="unit">mm</span></span
          >
        </label>
      </fieldset>
      <div class="section-title">{t('fs391.known_limits')}</div>
      <label title={t('machine.spindle_min.title')}>
        {t('machine.spindle_min')}
        <span class="field"
          ><input
            type="number"
            min="0"
            step="100"
            placeholder="—"
            value={draft.spindleRpmMin ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              if (raw === '') {
                draft.spindleRpmMin = undefined;
                return;
              }
              const v = parseInt(raw, 10);
              draft.spindleRpmMin = isFinite(v) && v >= 0 ? v : undefined;
            }}
          /><span class="unit">RPM</span></span
        >
      </label>
      <label title={t('machine.spindle_max.title')}>
        {t('machine.spindle_max')}
        <span class="field"
          ><input
            type="number"
            min="0"
            step="500"
            placeholder="—"
            value={draft.spindleRpmMax ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              if (raw === '') {
                draft.spindleRpmMax = undefined;
                return;
              }
              const v = parseInt(raw, 10);
              draft.spindleRpmMax = isFinite(v) && v >= 0 ? v : undefined;
            }}
          /><span class="unit">RPM</span></span
        >
      </label>
      <label title={t('machine.max_feed.title')}>
        {t('machine.max_feed')}
        <span class="field"
          ><input
            type="number"
            min="0"
            step="100"
            placeholder="—"
            value={draft.maxFeedMmMin ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              if (raw === '') {
                draft.maxFeedMmMin = undefined;
                return;
              }
              const v = parseInt(raw, 10);
              draft.maxFeedMmMin = isFinite(v) && v >= 0 ? v : undefined;
            }}
          /><span class="unit">mm/min</span></span
        >
      </label>

      <p class="help">
        {t('ux6.unknown_limits_stay_unset_workspace_dimensions_and_clearance_are')}
      </p>
      <FeedsSpeedsCompatibility
        kind="machines"
        value={draft.fsIdentity !== undefined ? draft.fsIdentity : inheritedIdentity}
        onChange={(identity) => (draft.fsIdentity = identity)}
      />
    {/if}
    {#if section === 'output' || section === 'all'}
      {#if isShopBot}<p class="help">{t('ux6.shopbot_context')}</p>
        {#if draft.unit !== 'mm' || draft.mode !== 'mill' || draft.toolchangeStrategy !== 'manual_m0_pause' || draft.parkAtHome || draft.parkXy || draft.postProfile}<p
            class="conflict"
            role="status"
          >
            {t('ux6.shopbot_retained')}
          </p>{/if}{/if}
      <div class="section-title">{t('machine.section.gcode_formatting')}</div>
      <label title={t('machine.dialect.title')}>
        {t('machine.dialect')}
        <span class="field">
          <select
            value={draft.gcodeDialect ?? ''}
            onchange={(e) => {
              const v = (e.currentTarget as HTMLSelectElement).value;
              draft.gcodeDialect =
                v === '' ? undefined : v === 'grbl' || v === 'hpgl' || v === 'cps' ? v : 'linuxcnc';
            }}
          >
            <option value="">{t('ux6.workspace_output_default')}</option>
            <option value="linuxcnc">LinuxCNC</option>
            <option value="grbl">GRBL</option>
            <option value="hpgl">HPGL</option>
            {#if cpsSupported || draft.gcodeDialect === 'cps'}
              <option value="cps">{t('machine.dialect.cps')}</option>
            {/if}
          </select>
        </span>
      </label>
      {#if draft.gcodeDialect === 'cps'}
        {#if !cpsSupported}<p class="conflict" role="status">
            CPS is unavailable on this backend. Saved selection retained; generation remains
            blocked.
          </p>{/if}
        <CpsPostConfigEditor
          value={draft.cpsPost}
          onchange={(next) => {
            draft.cpsPost = next;
          }}
        />
      {/if}

      <label title={t('machine.unit.title')}>
        {t('machine.unit')}
        <select bind:value={draft.unit}>
          <option value="mm">{t('machine.unit.mm')}</option>
          <option value="inch">{t('machine.unit.inch')}</option>
        </select>
      </label>
      <label class="check" title={t('machine.emit_comments.title')}>
        <input type="checkbox" bind:checked={draft.comments} />
        {t('machine.emit_comments')}
      </label>
      {#if isShopBot}<p class="help">{t('ux6c.shopbot_arcs')}</p>{/if}
      <details class="field-group">
        <summary>{t('ux6c.arc_settings')}</summary>
        <label
          class="check"
          title={isShopBot ? t('ux6c.shopbot_arcs') : t('machine.emit_arcs.title')}
        >
          <input type="checkbox" bind:checked={draft.arcs} />
          {isShopBot ? t('ux6c.fit_before_post') : t('machine.emit_arcs')}
        </label>
        <label class:disabled={!draft.arcs} title={t('machine.arc_tol.title')}>
          {t('machine.arc_tol')}
          <span class="field"
            ><input
              type="number"
              min="0"
              step="0.001"
              disabled={!draft.arcs}
              value={draft.arcFitToleranceMm ?? 0.01}
              oninput={(e) => {
                const v = (e.target as HTMLInputElement).valueAsNumber;
                draft.arcFitToleranceMm = isFinite(v) && v >= 0 ? v : undefined;
              }}
            /><span class="unit">mm</span></span
          >
        </label>
      </details>
      <label title={t('machine.toolchange.title')}>
        {t('machine.toolchange')}
        <select bind:value={draft.toolchangeStrategy}>
          <option value="atc"
            >{isShopBot ? t('ux6.atc_unsupported') : t('machine.toolchange.atc')}</option
          >
          <option value="manual_m6_prompt"
            >{isShopBot ? t('ux6.m6_unsupported') : t('machine.toolchange.m6_prompt')}</option
          >
          <option value="manual_m0_pause"
            >{isShopBot ? t('ux6.manual_shopbot') : t('machine.toolchange.m0_pause')}</option
          >
          <option value="ignore"
            >{isShopBot ? t('ux6.ignore_unsupported') : t('machine.toolchange.ignore')}</option
          >
        </select>
      </label>
      <!-- z9zh: GRBL dynamic-power laser mode (M4). -->
      <details
        class="field-group"
        open={draft.mode === 'laser' ||
          draft.mode === 'drag' ||
          draft.laserDynamicPower ||
          draft.plotModeZ}
      >
        <summary
          >Specialized process settings{draft.laserDynamicPower || draft.plotModeZ
            ? ' — saved values enabled'
            : ''}</summary
        >
        <p class="help">
          Dynamic power applies to GRBL laser output. Plot mode changes Z cutting behavior; it is
          not a preview setting. Unsupported combinations remain subject to generation checks.
        </p>
        <label class="check" title={t('machine.laser_dynamic.title')}>
          <input type="checkbox" bind:checked={draft.laserDynamicPower} />
          {t('machine.laser_dynamic')}
        </label>
        <label class="check" title={t('machine.plot_mode.title')}>
          <input type="checkbox" bind:checked={draft.plotModeZ} />
          {t('machine.plot_mode')}
        </label>
      </details>
      <details class="field-group" open={draft.gcodeDialect !== 'cps'}>
        <summary
          >Built-in output formatting{draft.gcodeDialect === 'cps'
            ? ' — retained, not CPS properties'
            : ''}</summary
        >
        <p class="help">
          Decimal separator, line numbers and templates configure built-in output. CPS posts use
          their own properties above; stored values are preserved.
        </p>
        <!-- Optional-stop M1 alternative to M0 at program pauses. -->
        <label class="check" title={t('machine.optional_stop.title')}>
          <input type="checkbox" bind:checked={draft.optionalStop} />
          {t('machine.optional_stop')}
        </label>
        <label title={t('machine.decimal_sep.title')}>
          {t('machine.decimal_sep')}
          <span class="field">
            <select
              value={draft.decimalSeparator ?? '.'}
              onchange={(e) => {
                const v = (e.currentTarget as HTMLSelectElement).value;
                draft.decimalSeparator = v === ',' ? ',' : '.';
              }}
            >
              <option value=".">{t('machine.decimal_sep.period')}</option>
              <option value=",">{t('machine.decimal_sep.comma')}</option>
            </select>
          </span>
        </label>
        <label title={t('machine.line_num.title')}>
          {t('machine.line_num')}
          <span class="field">
            <input
              type="number"
              min="0"
              step="10"
              placeholder={t('machine.line_num.placeholder')}
              value={draft.lineNumberStart ?? ''}
              oninput={(e) => {
                const raw = (e.target as HTMLInputElement).value;
                if (raw === '') {
                  draft.lineNumberStart = undefined;
                  return;
                }
                const v = parseInt(raw, 10);
                draft.lineNumberStart = isFinite(v) && v > 0 ? v : undefined;
              }}
            />
            <span class="unit">N</span>
          </span>
        </label>

        <div class="section-title">{t('machine.section.post_profile')}</div>
        {#if draft.mode === 'drag'}
          <p class="hpgl-note">{t('machine.hpgl_note')}</p>
        {/if}
        <label title={t('machine.profile_preset.title')}>
          {t('machine.profile_preset')}
          <span class="field">
            <select
              value={profilePreset(draft.postProfile)}
              onchange={(e) => {
                const v = (e.currentTarget as HTMLSelectElement).value;
                if (v === 'none') {
                  draft.postProfile = undefined;
                } else if (v === 'linuxcnc') {
                  draft.postProfile = {
                    name: 'LinuxCNC default',
                    file_extension: 'nc',
                    line_ending: '\n',
                  };
                } else if (v === 'mach3') {
                  draft.postProfile = {
                    name: 'Mach3 metric',
                    file_extension: 'tap',
                    line_ending: '\r\n',
                    program_start: '%\nN10 G21 G90 (ivac <version>)',
                    program_end: 'M30\n%',
                  };
                } else if (v === 'grbl') {
                  draft.postProfile = {
                    name: 'GRBL default',
                    file_extension: 'nc',
                    line_ending: '\n',
                    program_start: '; ivac <version> — GRBL',
                    program_end: 'M2',
                    tool_change: '; toolchange to T<t> (manual on GRBL)',
                  };
                } else if (v === 'custom') {
                  draft.postProfile = draft.postProfile ?? { name: 'Custom' };
                }
              }}
            >
              <option value="none">{t('machine.profile_preset.none')}</option>
              <option value="linuxcnc">{t('machine.profile_preset.linuxcnc')}</option>
              <option value="grbl">{t('machine.profile_preset.grbl')}</option>
              <option value="mach3">{t('machine.profile_preset.mach3')}</option>
              <option value="custom">{t('machine.profile_preset.custom')}</option>
            </select>
          </span>
        </label>
        {#if draft.postProfile}
          <div class="pp-summary">
            <span class="pp-summary-tweaks">
              {#if profileTweakSummary(draft.postProfile)}
                {t('machine.pp.overrides')} <em>{profileTweakSummary(draft.postProfile)}</em>
              {:else}
                {t('machine.pp.no_overrides')}
              {/if}
            </span>
            <button type="button" class="pp-edit-btn" onclick={() => (editorOpen = true)}
              >{t('machine.pp.edit_btn')}</button
            >
          </div>
        {/if}
      </details>
      <label title={t('machine.spindle_start_dwell.title')}>
        {t('machine.spindle_start_dwell')}
        <span class="field"
          ><input
            type="number"
            min="0"
            step="0.1"
            placeholder="0.5"
            value={draft.spindleStartDwellSec ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              if (raw === '') {
                draft.spindleStartDwellSec = undefined;
                return;
              }
              const v = parseFloat(raw);
              draft.spindleStartDwellSec = isFinite(v) && v >= 0 ? v : undefined;
            }}
          /><span class="unit">s</span></span
        >
      </label>
      <label title={t('machine.spindle_stop_dwell.title')}>
        {t('machine.spindle_stop_dwell')}
        <span class="field"
          ><input
            type="number"
            min="0"
            step="0.1"
            placeholder="0.5"
            value={draft.spindleStopDwellSec ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              if (raw === '') {
                draft.spindleStopDwellSec = undefined;
                return;
              }
              const v = parseFloat(raw);
              draft.spindleStopDwellSec = isFinite(v) && v >= 0 ? v : undefined;
            }}
          /><span class="unit">s</span></span
        >
      </label>
      {#if isShopBot}<p class="help">{t('uxa.shopbot_parking')}</p>{/if}
      <label
        class="check"
        title={isShopBot ? t('uxa.shopbot_parking') : t('machine.park_home.title')}
      >
        <input type="checkbox" bind:checked={draft.parkAtHome} />
        {t('machine.park_home')}
      </label>
      {#if !draft.parkAtHome}
        <div class="triplet-label">
          {t('machine.park_xy_label')} <span class="unit">mm, WCS</span>
          <small class="park-help">{t('machine.park_xy.help')}</small>
        </div>
        <div class="park-pair">
          <input
            type="number"
            step="1"
            aria-label={t('machine.park.x.aria')}
            placeholder="X"
            value={draft.parkXy?.[0] ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              const cur = draft.parkXy ?? [0, 0];
              if (raw === '') {
                // Clearing X drops the whole pair — both must be set
                // for park_xy to mean anything on the wire.
                draft.parkXy = undefined;
                return;
              }
              const v = parseFloat(raw);
              if (!isFinite(v)) return;
              draft.parkXy = [v, cur[1]];
            }}
          />
          <input
            type="number"
            step="1"
            aria-label={t('machine.park.y.aria')}
            placeholder="Y"
            value={draft.parkXy?.[1] ?? ''}
            oninput={(e) => {
              const raw = (e.target as HTMLInputElement).value;
              const cur = draft.parkXy ?? [0, 0];
              if (raw === '') {
                draft.parkXy = undefined;
                return;
              }
              const v = parseFloat(raw);
              if (!isFinite(v)) return;
              draft.parkXy = [cur[0], v];
            }}
          />
        </div>
      {/if}
    {/if}
    {#if section === 'advanced' || section === 'all'}
      <p class="help">{t('ux6.time_estimate_assumptions_only_these_values_do_not_configure')}</p>
      <div class="section-title">{t('machine.section.kinematics')}</div>
      <label title={t('machine.rapid.title')}>
        {t('machine.rapid')}
        <span class="field"
          ><input type="number" min="0" step="100" bind:value={draft.rapidSpeed} /><span
            class="unit">mm/min</span
          ></span
        >
      </label>
      <label title={t('machine.toolchange_time.title')}>
        {t('machine.toolchange_time')}
        <span class="field"
          ><input type="number" min="0" step="0.5" bind:value={draft.toolchangeS} /><span
            class="unit">s</span
          ></span
        >
      </label>
      <div class="triplet-label">{t('machine.accel_label')} <span class="unit">mm/s²</span></div>
      <div class="triplet">
        <input
          type="number"
          min="0"
          step="10"
          aria-label={t('machine.accel.x.aria')}
          value={draft.accel?.x ?? 250}
          oninput={(e) => {
            const v = (e.target as HTMLInputElement).valueAsNumber;
            draft.accel = {
              ...(draft.accel ?? { x: 250, y: 250, z: 250 }),
              x: isFinite(v) ? v : 250,
            };
          }}
        />
        <input
          type="number"
          min="0"
          step="10"
          aria-label={t('machine.accel.y.aria')}
          value={draft.accel?.y ?? 250}
          oninput={(e) => {
            const v = (e.target as HTMLInputElement).valueAsNumber;
            draft.accel = {
              ...(draft.accel ?? { x: 250, y: 250, z: 250 }),
              y: isFinite(v) ? v : 250,
            };
          }}
        />
        <input
          type="number"
          min="0"
          step="10"
          aria-label={t('machine.accel.z.aria')}
          value={draft.accel?.z ?? 250}
          oninput={(e) => {
            const v = (e.target as HTMLInputElement).valueAsNumber;
            draft.accel = {
              ...(draft.accel ?? { x: 250, y: 250, z: 250 }),
              z: isFinite(v) ? v : 250,
            };
          }}
        />
      </div>
      <label
        class="check"
        title="Retained compatibility field; jerk is not used in current time estimates."
      >
        <input type="checkbox" bind:checked={composite.jerkEnabled} />
        {t('machine.jerk_enable')}
      </label>
      {#if composite.jerkEnabled}
        <div class="triplet-label">{t('machine.jerk_label')} <span class="unit">mm/s³</span></div>
        <div class="triplet">
          <input
            type="number"
            min="0"
            step="10"
            aria-label={t('machine.jerk.x.aria')}
            bind:value={jerkDraft.x}
          />
          <input
            type="number"
            min="0"
            step="10"
            aria-label={t('machine.jerk.y.aria')}
            bind:value={jerkDraft.y}
          />
          <input
            type="number"
            min="0"
            step="10"
            aria-label={t('machine.jerk.z.aria')}
            bind:value={jerkDraft.z}
          />
        </div>
      {/if}
    {/if}
  </div>

  <footer>
    {#if dd.confirmingDiscard}
      <span>{t('common.discard_unsaved')}</span>
      <button onclick={() => dd.cancelDiscard()}>{t('common.keep_editing')}</button>
      <button onclick={close}>{t('common.discard')}</button>
    {:else}
      <button
        class="btn-secondary"
        title={t('ux6.export_the_displayed_draft_only_does_not_apply_changes')}
        onclick={() => fileOps.saveMachine(draftSnapshot())}>{t('ux6.export_settings')}</button
      >
      <button
        class="btn-secondary"
        disabled={dd.isDirty}
        title="Import into this draft; Apply or Save is still required. Resolve unsaved changes first."
        onclick={async () => {
          const m = await fileOps.readMachine();
          if (m) dd.draft = compositeOf(m);
        }}>{t('ux6.import_settings')}</button
      >
      <span class="sep"></span>
      <button class="btn-secondary" onclick={revert} disabled={!dd.isDirty && !conflict}
        >{t('common.revert')}</button
      >
      <button class="btn-primary" onclick={commit} disabled={!dd.isDirty || conflict}
        >{saveLabel ?? 'Apply to project'}</button
      >
    {/if}
  </footer>
{/snippet}

{#if embedded}
  <section class="embedded-shell">{@render shell()}</section>
{:else if open}
  <Modal
    onClose={close}
    modalClass="machine-modal"
    persistKey="machine"
    width="min(880px, 96vw)"
    draggable
    resizable
    ariaLabelledBy="machine-title"
  >
    {@render shell()}
  </Modal>
{/if}
{#if active}
  <PostProcessorEditor
    open={editorOpen}
    initial={draft.postProfile ?? { name: 'Custom' }}
    onSave={(next) => {
      draft.postProfile = next;
      editorOpen = false;
    }}
    onClose={() => (editorOpen = false)}
  />
{/if}

<style>
  .field-group {
    display: grid;
    gap: var(--ui-space-2);
    border-top: 1px solid var(--border);
    font-size: 0.8rem;
  }
  .field-group summary {
    cursor: pointer;
    padding: 6px 0;
  }
  .field-group label {
    margin-top: var(--ui-space-2);
  }
  .help {
    margin: 4px 0;
    font-size: 0.78rem;
    color: var(--text-muted);
    line-height: 1.45;
  }
  .conflict {
    padding: 8px 12px;
    color: var(--warn);
    font-size: 0.8rem;
  }
  .grid {
    max-width: 650px;
    width: 100%;
    box-sizing: border-box;
  }
  .grid input[type='number'] {
    width: 96px;
    flex: 0 0 96px;
  }
  .grid .triplet {
    max-width: 330px;
  }
  .grid .triplet input[type='number'] {
    width: 100%;
  }
  footer {
    flex-wrap: wrap;
    position: sticky;
    bottom: 0;
    z-index: 1;
    margin-top: auto;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.7rem;
    border-bottom: 1px solid var(--border);
    background: var(--bg-elevated);
  }
  h2 {
    font-size: 0.95rem;
    margin: 0;
    color: var(--text-strong);
  }
  /* `.dlg-close` lives in Modal.svelte; `.btn-primary|secondary|danger`
     + `.discard-prompt` in app.css (audit hbi7). */
  /* Tab-panel (embedded) shell — fills the main area; the grid scrolls
     between the sticky header and footer. */
  .embedded-shell {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    background: var(--bg-panel);
    overflow: auto;
  }
  .storage-note {
    margin: 0;
    padding: 0.45rem 0.7rem;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, var(--accent) 4%, var(--bg-panel));
    color: var(--text-muted);
    font-size: 0.78rem;
    line-height: 1.4;
  }
  /* `<strong>` arrives via {@html} (translated string), so it needs
     :global to escape Svelte's scoped-CSS hashing. */
  .storage-note :global(strong) {
    color: var(--text);
  }
  .grid {
    padding: 0.7rem;
    display: grid;
    gap: 0.5rem;
  }
  label {
    display: grid;
    grid-template-columns: minmax(0, 9rem) minmax(0, 1fr);
    align-items: center;
    gap: 0.6rem;
    font-size: 0.8rem;
  }
  label.check {
    grid-template-columns: auto 1fr;
    gap: 0.4rem;
  }
  label.disabled {
    color: var(--text-muted);
    opacity: 0.6;
  }
  input[type='number'],
  select {
    background: var(--bg-input);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 3px;
    padding: 0.2rem 0.4rem;
    font-size: 0.8rem;
  }
  .field {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
  }
  .field input[type='number'] {
    flex: 1;
    min-width: 0;
  }
  input[type='checkbox'] {
    accent-color: var(--accent);
  }
  .section-title {
    grid-column: 1 / -1;
    margin-top: 0.4rem;
    padding-top: 0.4rem;
    border-top: 1px solid var(--border);
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }
  .hpgl-note {
    grid-column: 1 / -1;
    margin: 0.3rem 0;
    padding: 0.4rem 0.6rem;
    border: 1px dashed var(--border);
    border-radius: 4px;
    background: color-mix(in srgb, var(--accent) 4%, var(--bg-panel));
    color: var(--text-muted);
    font-size: 0.78rem;
    line-height: 1.4;
  }
  .triplet-label {
    font-size: 0.8rem;
    color: var(--text);
  }
  .triplet {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.3rem;
  }
  .triplet input[type='number'] {
    width: 100%;
  }
  .park-pair {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.3rem;
  }
  .park-pair input[type='number'] {
    width: 100%;
  }
  .park-help {
    display: block;
    color: var(--text-muted);
    font-size: 0.7rem;
    font-weight: normal;
    margin-top: 0.15rem;
  }
  footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.4rem;
    padding: 0.5rem 0.7rem;
    border-top: 1px solid var(--border);
    background: var(--bg-elevated);
  }
  .pp-summary {
    grid-column: 1 / -1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.6rem;
    padding: 0.3rem 0.6rem;
    border: 1px dashed var(--border);
    border-radius: 4px;
    background: var(--bg-elevated);
    font-size: 0.82rem;
  }
  .pp-summary-tweaks em {
    color: var(--accent);
    font-style: normal;
  }
  .pp-edit-btn {
    background: transparent;
    color: var(--text);
    border: 1px solid var(--border);
    padding: 0.25rem 0.6rem;
    border-radius: 3px;
    cursor: pointer;
    font-size: 0.82rem;
  }
  .pp-edit-btn:hover {
    background: var(--bg);
  }
  .grid .field input[type='number'] {
    width: 96px;
    flex: 0 0 96px;
  }
  .grid .capabilities {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }
  .grid .capabilities .cap-toggle {
    display: flex;
    gap: 5px;
  }
  .grid .work-area label {
    grid-template-columns: minmax(0, 9rem) auto;
    margin: 5px 0;
  }
</style>
