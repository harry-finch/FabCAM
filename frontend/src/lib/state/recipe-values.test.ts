import { describe, expect, it } from 'vitest';
import { recipeData, recipeInputs, recipeProblems } from './recipe-values';
import { adaptationFixture } from './preset-adaptation.fixture';
import {
  adaptationProblems,
  adaptationWarnings,
  duplicatePresetAsDraft,
} from './preset-adaptation';
import { readFileSync } from 'node:fs';
import type { SourcePack } from './feeds-speeds-sources';
async function validate(pack: SourcePack): Promise<SourcePack> {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  return wasm.validateSourcePack(JSON.stringify(pack));
}
const form = {
  name: 'SOFTWARE ONLY manual recipe',
  reviewed: true,
  ownershipConfirmed: false,
  notes: 'Synthetic; no physical trial',
};
const now = '2026-10-21T16:00:00.000Z';
describe('manual recipe values', () => {
  it('edits explicitly without diameter/flute scaling, and preserves immutable source provenance', async () => {
    const { row, target, destination } = adaptationFixture();
    const before = JSON.stringify({ row, target, destination });
    target.machine.fsLocalIdentity = { id: 'other-lab-machine', revision: '1' };
    target.machine.fsIdentity = null;
    target.material = { id: 'material/wood.plywood', revision: '1', label: 'Plywood' };
    target.tool!.diameter = 3;
    target.tool!.flutes = 2;
    const inputs = recipeInputs(row.profile.data);
    expect(inputs.rate).toBe(
      row.profile.data.kind === 'exact_values'
        ? row.profile.data.feed_mm_min
        : row.profile.data.chipload_mm_tooth,
    );
    Object.assign(inputs, { rpm: 12000, rate: 600, plunge: 150, depth: 0.5 });
    const data = recipeData(row.profile.data, inputs)!;
    expect(adaptationWarnings(row, target)).toEqual(['machine', 'material']);
    expect(adaptationProblems(row, target, false, data)).toEqual([]);
    const copy = duplicatePresetAsDraft(
      destination,
      row,
      target,
      { ...form, data },
      now,
      'recipe-1',
    );
    const p = copy.dataset.profile_set.profiles[0];
    expect(p.data).toEqual({
      kind: 'exact_values',
      rpm: 12000,
      feed_mm_min: 600,
      plunge_mm_min: 150,
      doc_mm: 0.5,
      stepover_ratio: null,
    });
    expect(p.scope.doc_mm).toEqual({ min: 0.5, max: 0.5 });
    expect(p.scope.material).toEqual({
      kind: 'exact',
      identity: target.material && { id: target.material.id, revision: '1' },
    });
    expect(p.scope.machine).toEqual({ kind: 'exact', identity: target.machine.fsLocalIdentity });
    expect(JSON.parse(p.source.notes!.slice(p.source.notes!.indexOf('{'))).data).toEqual(
      row.profile.data,
    );
    expect(p.source.notes).toContain('manual-inactive-recipe-1');
    expect(copy.records[0].lab).toMatchObject({
      state: 'draft',
      enabled: false,
      physically_tested: false,
      review: null,
      tested_at: null,
    });
    expect(copy.records[0].lab_history).toEqual([]);
    const checked = await validate(copy);
    const { stepover_ratio, ...numeric } = p.data;
    expect(checked.dataset.profile_set.profiles[0].data).toMatchObject(numeric);
    expect(checked.dataset.profile_set.profiles[0].data.stepover_ratio ?? null).toBe(
      stepover_ratio ?? null,
    );
    expect(checked.records[0].lab?.enabled).toBe(false);
    expect(JSON.stringify(row)).toBe(JSON.stringify(JSON.parse(before).row));
    expect(JSON.stringify(destination)).toBe(JSON.stringify(JSON.parse(before).destination));
  });
  it.each([0, -1, NaN, Infinity, undefined])(
    'refuses invalid values %s before any storage write',
    (bad) => {
      const { row } = adaptationFixture();
      for (const key of ['rpm', 'rate', 'plunge', 'depth'] as const) {
        expect(
          recipeData(row.profile.data, { ...recipeInputs(row.profile.data), [key]: bad }),
        ).toBeNull();
      }
    },
  );
  it('retains chipload semantics and synchronizes edited pocket scope without inventing a feed', async () => {
    const { row, target, destination } = adaptationFixture();
    row.profile.scope.operation = 'pocket';
    row.profile.scope.engagement = { kind: 'nominal', ratio: { min: 0.3, max: 0.3 } };
    row.profile.data = {
      kind: 'chipload_target',
      rpm: 12000,
      chipload_mm_tooth: 0.03,
      plunge_mm_min: 150,
      doc_mm: 1,
      stepover_ratio: 0.3,
    };
    const data = recipeData(row.profile.data, {
      rpm: 10000,
      rate: 0.02,
      plunge: 100,
      depth: 0.4,
      stepover: 20,
    })!;
    const p = duplicatePresetAsDraft(
      destination,
      row,
      target,
      { ...form, data },
      now,
      'chipload-recipe',
    );
    expect(p.dataset.profile_set.profiles[0].data).not.toHaveProperty('feed_mm_min');
    expect(p.dataset.profile_set.profiles[0].scope.engagement).toEqual({
      kind: 'nominal',
      ratio: { min: 0.2, max: 0.2 },
    });
    await expect(validate(p)).resolves.toBeDefined();
  });
  it('rejects out-of-range stepover, hard limits and edited depths beyond flute length', () => {
    const { row, target, destination } = adaptationFixture();
    expect(
      recipeData(row.profile.data, { ...recipeInputs(row.profile.data), stepover: 101 }),
    ).toBeNull();
    expect(
      recipeData(row.profile.data, { ...recipeInputs(row.profile.data), stepover: 0 }),
    ).toBeNull();
    target.machine.spindleRpmMax = 10000;
    expect(adaptationProblems(row, target, false)).toContain('limits');
    expect(() => duplicatePresetAsDraft(destination, row, target, form, now, 'bad')).toThrow();
    const data = { ...row.profile.data, rpm: 9000, doc_mm: 0.25 };
    target.tool!.fluteLengthMm = 0.5;
    expect(adaptationProblems(row, target, false, data)).toEqual([]);
    expect(adaptationProblems(row, target, false, { ...data, doc_mm: 1 })).toContain('geometry');
    expect(recipeProblems({ ...data, stepover_ratio: null }, 'pocket', target.machine)).toContain(
      'values',
    );
  });
});
