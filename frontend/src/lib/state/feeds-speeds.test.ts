import { describe, expect, it } from 'vitest';
import {
  cuttingContextKey,
  parseDatasetCatalog,
  preserveManualCutting,
  minimumKnownRpm,
  editOperationRate,
  operationRates,
  referenceContext,
} from './feeds-speeds';
import { hasFeedsSpeedsData, validateProjectFile } from './feeds-speeds-persistence';
import { applyCuttingCommand, type CommandTarget } from './commands';
import type { ProjectFile, MachineSettings, ToolEntry } from './project-types';
import type { OpEntry } from './op_types';
import type { components } from '../api/generated';
import source from '../../../../data/feeds-speeds/sienci-router-reference.v1.json';
type S = components['schemas'];
function op(): OpEntry {
  return {
    id: 1,
    name: 'Profile',
    enabled: true,
    kind: 'profile',
    offset: 'outside',
    toolId: 1,
    sourceLayers: null,
    depth: -2.8,
    startDepth: 0,
    step: -2.8,
  };
}
function applied(): S['AppliedCutting'] {
  const profile = source.profile_set.profiles[0] as S['FsProfile'];
  return {
    slots: ['rough', 'finish'].map((pass) => ({
      tool_id: 1,
      pass: pass as S['FsPass'],
      rpm: 17000,
      feed_mm_min: 1620,
      plunge_mm_min: 810,
      origins: { rpm: 'recommendation', feed: 'recommendation', plunge: 'recommendation' },
    })),
    context_key: 'native:synthetic',
    doc_origin: 'recommendation',
    proposal: {
      parameters: {
        rpm: 17000,
        feed_mm_min: 1620,
        plunge_mm_min: 810,
        doc_mm: 2.8,
        effective_chipload_mm_tooth: 1620 / 17000,
      },
      aggressiveness: 'prudent',
      source_evidence: 'reference_data',
      applicability: 'within_declared_profile',
      diagnostics: [],
      provenance: {
        engine_version: 'test',
        schema_version: 1,
        dataset: source.profile_set.identity,
        profile,
        request: {} as S['FsFeedsSpeedsRequest'],
        selection_tier: 3,
        adjustments: [],
      },
    },
  };
}
function file(): ProjectFile {
  return {
    kind: 'ivac-project',
    version: 1,
    imports: [],
    visibleLayers: [],
    selectedEntities: [],
    operations: [op()],
  };
}
describe('FS3 explicit persistence and edits', () => {
  it('keeps v1 legacy files, refuses unsupported versions and FS data disguised as v1', () => {
    const f = file();
    expect(() => validateProjectFile(f)).not.toThrow();
    expect(hasFeedsSpeedsData(f)).toBe(false);
    expect(() => validateProjectFile({ ...f, version: 99 } as unknown as ProjectFile)).toThrow();
    f.operations![0].appliedCutting = applied();
    expect(() => validateProjectFile(f)).toThrow();
    f.kind = 'ivac-project-fs';
    f.version = 2;
    expect(() => validateProjectFile(f)).not.toThrow();
  });
  it('uses a distinct root kind rejected by pre-FS apps', () => {
    const f = file();
    f.kind = 'ivac-project-fs';
    f.version = 2;
    expect(f.kind).not.toBe('ivac-project');
    expect(() => validateProjectFile({ ...f, kind: 'ivac-project' })).toThrow();
  });
  it('refuses invalid and duplicate role rates at reopen', () => {
    const f = file();
    f.kind = 'ivac-project-fs';
    f.version = 2;
    f.operations![0].appliedCutting = applied();
    f.operations![0].appliedCutting.slots[0].rpm = 0;
    expect(() => validateProjectFile(f)).toThrow();
    f.operations![0].appliedCutting = applied();
    f.operations![0].appliedCutting.slots.push({ ...f.operations![0].appliedCutting.slots[0] });
    expect(() => validateProjectFile(f)).toThrow();
  });
  it('manual feed edits target saved roles and never reopen tool inheritance', () => {
    const o = { ...op(), appliedCutting: applied() };
    const before = JSON.stringify(o);
    const patch = preserveManualCutting(o, { feedRateOverride: 1500 });
    expect(patch.feedRateOverride).toBeUndefined();
    expect(
      patch.appliedCutting!.slots.every(
        (s) => s.feed_mm_min === 1500 && s.origins.feed === 'manual',
      ),
    ).toBe(true);
    expect(JSON.stringify(o)).toBe(before);
    expect(
      preserveManualCutting(o, { feedRateOverride: undefined }).appliedCutting,
    ).toBeUndefined();
    expect(o.appliedCutting.slots[0].feed_mm_min).toBe(1620);
  });
  it('marks native geometry origins on manual step or overlap edits', () => {
    const o = { ...op(), appliedCutting: applied() };
    expect(preserveManualCutting(o, { step: -2 }).appliedCutting!.doc_origin).toBe('manual');
    expect(
      preserveManualCutting(o, { xyOverlap: 0.6 } as Partial<OpEntry>).appliedCutting!
        .stepover_origin,
    ).toBe('manual');
  });
  it('saved manual slots and provenance survive JSON independently of live source data', () => {
    const f = file();
    f.kind = 'ivac-project-fs';
    f.version = 2;
    f.operations![0].appliedCutting = applied();
    f.operations![0].appliedCutting.slots[1].feed_mm_min = 1500;
    f.operations![0].appliedCutting.slots[1].origins.feed = 'manual';
    const reopened = JSON.parse(JSON.stringify(f)) as ProjectFile;
    validateProjectFile(reopened);
    expect(reopened.operations![0].appliedCutting).toEqual(f.operations![0].appliedCutting);
  });
  it('context changes mark stale while manual rate edits do not change the context key', () => {
    const o = { ...op(), appliedCutting: applied() };
    const m = { mode: 'mill' } as MachineSettings;
    const tools = [{ id: 1, diameter: 6.35 }] as ToolEntry[];
    const key = cuttingContextKey(o, m, tools);
    o.appliedCutting.slots[0].feed_mm_min = 1500;
    expect(cuttingContextKey(o, m, tools)).toBe(key);
    expect(cuttingContextKey({ ...o, step: -2 }, m, tools)).not.toBe(key);
    expect(cuttingContextKey(o, m, [{ ...tools[0], diameter: 6 }])).not.toBe(key);
  });
  it('applies context, geometry and rates atomically with undo', () => {
    const o = op();
    const target = { operations: [o], dirty: false } as CommandTarget;
    const block = applied();
    const material = { id: 'material', revision: '1' };
    const context = { machine_identity: { id: 'machine', revision: '1' }, tools: [] };
    const command = applyCuttingCommand(
      1,
      { appliedCutting: block, step: -2.8 },
      material,
      context,
    );
    command.apply(target);
    expect(target.operations[0].appliedCutting).toEqual(block);
    expect(target.material).toEqual(material);
    expect(target.dirty).toBe(true);
    command.revert(target);
    expect(target.operations[0]).toEqual(o);
    expect(target.material).toBeUndefined();
    expect(target.fsContext).toBeUndefined();
  });
  it('catalog parsing does not promote source data or invent identities', () => {
    const d = parseDatasetCatalog(JSON.stringify(source));
    expect(d.tools).toHaveLength(2);
    expect(d.machines[0].identity.id).toBe('sienci-longmill-mk2-reference');
    expect(() => parseDatasetCatalog('{}')).toThrow();
    expect(() => parseDatasetCatalog(JSON.stringify({ ...source, format_version: 2 }))).toThrow();
  });
});

describe('Known cutter constraints', () => {
  it('intersects all known maxima without inventing an unknown limit', () => {
    expect(minimumKnownRpm(undefined, null)).toBeNull();
    expect(minimumKnownRpm(18000, 12000)).toBe(12000);
    expect(minimumKnownRpm(12000, undefined)).toBe(12000);
    expect(minimumKnownRpm(0, 12000)).toBe(0);
  });
});

describe('FS3.5 manual operation ownership', () => {
  const tool = {
    id: 1,
    name: 'Physical cutter',
    kind: 'endmill',
    diameter: 6,
    flutes: 2,
    speed: 18000,
    feedRate: 3600,
    plungeRate: 900,
    coolant: 'off',
  } as ToolEntry;
  it('saves manual-only values without fabricated evidence and rejects old FS readers', () => {
    const o = op();
    o.appliedCutting = editOperationRate(o, [tool], 1, 'finish', 'rpm', 17000);
    o.appliedCutting = editOperationRate(o, [tool], 1, 'finish', 'feed_mm_min', 1500);
    expect(o.appliedCutting.proposal).toBeUndefined();
    const f = {
      ...file(),
      kind: 'ivac-project-cutting',
      version: 3,
      operations: [o],
      material: { id: 'custom-mdf', revision: 'job-1', label: 'MDF atelier' },
    } as ProjectFile;
    const restored = JSON.parse(JSON.stringify(f)) as ProjectFile;
    expect(() => validateProjectFile(restored)).not.toThrow();
    expect(restored.material?.label).toBe('MDF atelier');
    expect(
      operationRates(restored.operations![0], { ...tool, feedRate: 999 }, 'finish')?.feed_mm_min,
    ).toBe(1500);
    expect(restored.operations![0].appliedCutting?.slots[1].origins.feed).toBe('manual');
    expect(() =>
      validateProjectFile({ ...restored, kind: 'ivac-project-fs', version: 2 }),
    ).toThrow();
  });
  it('manual editing keeps the accepted proposal and native geometry', () => {
    const o = { ...op(), appliedCutting: applied() };
    const result = editOperationRate(o, [tool], 1, 'finish', 'plunge_mm_min', 600);
    expect(result.proposal).toEqual(o.appliedCutting.proposal);
    expect(result.slots[1].plunge_mm_min).toBe(600);
    expect(o.step).toBe(-2.8);
    expect(o.appliedCutting.slots[1].plunge_mm_min).toBe(810);
  });
  it('physical reference metadata explicitly overrides or clears legacy bindings', () => {
    const old = {
      machine_identity: { id: 'old-machine', revision: '1' },
      tools: [{ tool_id: 1, identity: { id: 'old-tool', revision: '1' }, max_rpm: 12000 }],
    };
    const result = referenceContext(
      { fsIdentity: null } as MachineSettings,
      [{ ...tool, fsIdentity: null }],
      old,
    );
    expect(result.machine_identity).toBeNull();
    expect(result.tools).toEqual([]);
    expect(referenceContext({} as MachineSettings, [tool], old)).toEqual(old);
  });
});

describe('FS3.5 cutter changes preserve saved authority', () => {
  it('does not silently inherit a different cutter and allows explicit manual resolution', () => {
    const old = { ...op(), appliedCutting: applied(), toolId: 2 };
    const replacement = {
      id: 2,
      name: 'replacement',
      kind: 'endmill',
      diameter: 3,
      flutes: 2,
      speed: 12000,
      feedRate: 800,
      plungeRate: 400,
      coolant: 'off',
    } as ToolEntry;
    const before = JSON.stringify(old.appliedCutting);
    expect(operationRates(old, replacement, 'finish')).toBeNull();
    const resolved = editOperationRate(old, [replacement], 2, 'finish', 'feed_mm_min', 700);
    expect(resolved.slots.find((s) => s.tool_id === 2 && s.pass === 'finish')?.feed_mm_min).toBe(
      700,
    );
    expect(JSON.stringify(old.appliedCutting)).toBe(before);
    expect(resolved.proposal).toEqual(old.appliedCutting.proposal);
  });
});
