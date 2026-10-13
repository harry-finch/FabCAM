<script lang="ts">
  import { project } from '../state/project.svelte';
  import type { Fixture } from '../state/project-types';
  import { computeFootprint } from '../sim/footprint';
  import { parseFixturePolygon } from '../state/fixture-editor';
  import { t } from '../i18n';
  let error = $state<string | null>(null);
  const selected = $derived(
    project.data.fixtures.find((f) => f.id === project.sel.selectedFixtureId),
  );
  function add(shape: 'box' | 'cylinder' | 'polygon') {
    const fp = computeFootprint(
      project.stockSizingImport,
      project.data.stock,
      project.data.machine.workArea,
    );
    project.addFixture(
      shape === 'box'
        ? { shape, width: 20, depth: 10 }
        : shape === 'cylinder'
          ? { shape, radius: 5 }
          : {
              shape,
              vertices: [
                [-10, -5],
                [10, -5],
                [10, 5],
                [-10, 5],
              ],
            },
      [fp.minX - 15, fp.minY + 15],
      project.data.stock.offsetZ ?? 0,
      (project.data.stock.offsetZ ?? 0) + 10,
    );
    error = null;
  }
  function number(
    f: Fixture,
    key: 'x' | 'y' | 'bottom' | 'top' | 'width' | 'depth' | 'radius',
    raw: string,
  ) {
    const value = Number(raw);
    if (
      !raw.trim() ||
      !Number.isFinite(value) ||
      (['width', 'depth', 'radius'].includes(key) && value <= 0) ||
      (key === 'bottom' && value >= f.z_top) ||
      (key === 'top' && value <= f.z_bottom)
    ) {
      error = t('ux2.fixture.invalid');
      return;
    }
    error = null;
    const patch: Partial<Fixture> =
      key === 'x' || key === 'y'
        ? { origin: key === 'x' ? [value, f.origin[1]] : [f.origin[0], value] }
        : key === 'bottom'
          ? { z_bottom: value }
          : key === 'top'
            ? { z_top: value }
            : { kind: { ...f.kind, [key]: value } };
    project.updateFixture(f.id, patch);
  }
</script>

<details class="fixtures" open={project.data.fixtures.length > 0}>
  <summary>{t('ux2.fixtures')} <span>{project.data.fixtures.length}</span></summary>
  <p class="hint">{t('ux2.fixture.help')}</p>
  <div class="actions">
    <button onclick={() => add('box')}>{t('ux2.fixture.box')}</button>
    <button onclick={() => add('cylinder')}>{t('ux2.fixture.cylinder')}</button>
    <button onclick={() => add('polygon')}>{t('ux2.fixture.polygon')}</button>
  </div>
  <div class="items" aria-label={t('ux2.fixtures')}>
    {#each project.data.fixtures as f (f.id)}
      <button
        class:active={f.id === selected?.id}
        aria-pressed={f.id === selected?.id}
        onclick={() => project.selectFixture(f.id)}>{f.name}</button
      >
    {/each}
  </div>
  {#if selected}
    {@const f = selected}
    <label
      >{t('ux2.fixture.name')}<input
        value={f.name}
        onchange={(e) => project.updateFixture(f.id, { name: e.currentTarget.value })}
      /></label
    >
    <div class="grid">
      {#each ['x', 'y', 'bottom', 'top'] as key (key)}
        <label
          >{key === 'x'
            ? 'X'
            : key === 'y'
              ? 'Y'
              : key === 'bottom'
                ? t('ux2.fixture.bottom')
                : t('ux2.fixture.top')}
          <span class="field"
            ><input
              type="number"
              step="0.5"
              value={key === 'x'
                ? f.origin[0]
                : key === 'y'
                  ? f.origin[1]
                  : key === 'bottom'
                    ? f.z_bottom
                    : f.z_top}
              onchange={(e) =>
                number(f, key as 'x' | 'y' | 'bottom' | 'top', e.currentTarget.value)}
            /><span>{t('machine.unit.mm')}</span></span
          >
        </label>
      {/each}
      {#if f.kind.shape === 'box'}
        {#each ['width', 'depth'] as key (key)}
          <label
            >{key === 'width' ? t('ux2.fixture.width') : t('ux2.fixture.depth')}
            <span class="field"
              ><input
                type="number"
                min="0.01"
                step="0.5"
                value={key === 'width' ? f.kind.width : f.kind.depth}
                onchange={(e) => number(f, key as 'width' | 'depth', e.currentTarget.value)}
              /><span>{t('machine.unit.mm')}</span></span
            >
          </label>
        {/each}
      {:else if f.kind.shape === 'cylinder'}
        <label
          >{t('ux2.fixture.radius')}<span class="field"
            ><input
              type="number"
              min="0.01"
              step="0.5"
              value={f.kind.radius}
              onchange={(e) => number(f, 'radius', e.currentTarget.value)}
            /><span>{t('machine.unit.mm')}</span></span
          ></label
        >
      {/if}
    </div>
    {#if f.kind.shape === 'polygon'}
      <label
        >{t('ux2.fixture.vertices')}
        <textarea
          rows="4"
          value={f.kind.vertices.map((p) => p.join(', ')).join('\n')}
          onchange={(e) => {
            const vertices = parseFixturePolygon(e.currentTarget.value);
            if (!vertices) {
              error = t('ux2.fixture.invalid');
              return;
            }
            error = null;
            project.updateFixture(f.id, { kind: { shape: 'polygon', vertices } });
          }}
        ></textarea>
      </label>
    {/if}
    <button class="remove" onclick={() => project.removeFixture(f.id)}
      >{t('ux2.fixture.remove')}</button
    >
  {/if}
  {#if error}<p class="error" role="alert">{error}</p>{/if}
</details>

<style>
  .fixtures {
    border-top: 1px solid var(--border);
    padding: var(--ui-space-2) 0;
  }
  summary {
    font-size: 0.78rem;
    cursor: pointer;
  }
  summary span {
    color: var(--text-muted);
    margin-left: var(--ui-space-2);
  }
  .hint {
    font-size: 0.72rem;
    color: var(--text-muted);
    line-height: 1.4;
  }
  .actions,
  .items {
    display: flex;
    flex-wrap: wrap;
    gap: var(--ui-space-1);
    margin: var(--ui-space-2) 0;
  }
  button {
    font-size: 0.72rem;
  }
  .active {
    border-color: var(--accent);
  }
  .grid {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: var(--ui-space-2);
  }
  label {
    display: flex;
    flex-direction: column;
    gap: var(--ui-space-1);
    font-size: 0.72rem;
    color: var(--text-muted);
    margin: var(--ui-space-1) 0;
  }
  input,
  textarea {
    min-width: 0;
    width: 100%;
    box-sizing: border-box;
    font: inherit;
    color: var(--text);
  }
  .field {
    display: flex;
    align-items: center;
    gap: var(--ui-space-1);
  }
  .field span {
    font-size: 0.68rem;
  }
  .remove {
    margin-top: var(--ui-space-2);
  }
  .error {
    font-size: 0.72rem;
    color: var(--error);
  }
</style>
