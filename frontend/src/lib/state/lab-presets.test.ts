import { describe, it, expect } from 'vitest';
import {
  captureScope,
  saveCapturedPreset,
  revisePreset,
  closePresets,
  newestLabPack,
  LAB_POLICY,
  type LabCapture,
  type CaptureForm,
} from './lab-presets';
import source from '../../../../data/feeds-speeds/packs/sienci-longmill.v1.json';
const now = '2026-10-12T08:00:00.000Z',
  later = '2026-10-12T09:00:00.000Z';
export function capture(): LabCapture {
  return {
    request: {
      schema_version: 1,
      dataset: { id: 'sorbonne-fablab', revision: 'capture' },
      machine: { identity: { id: 'machine-shopbot-software', revision: '1' } },
      material: {
        identity: { id: 'material/wood.plywood.birch', revision: '1' },
        thickness_mm: 12,
      },
      tool: {
        identity: { id: 'tool-6mm-software', revision: '1' },
        diameter_mm: 6,
        flute_count: 2,
        cutting_family: 'flat_endmill',
      },
      operation: {
        kind: 'full_slot',
        doc_mm: 2,
        entry: 'direct',
        engagement: { kind: 'known_full_width' },
      },
      aggressiveness: 'prudent',
    },
    rpm: 10000,
    feed_mm_min: 4000,
    plunge_mm_min: 900,
    machine_name: 'SOFTWARE ONLY ShopBot',
    tool_name: 'SOFTWARE ONLY 6 mm',
    operation_name: 'SOFTWARE ONLY Profile',
    finishing_context: 'No separately qualified finishing context',
  };
}
const form: CaptureForm = {
  name: 'SOFTWARE ONLY birch plywood',
  state: 'tested',
  tested_at: '2026-10-12',
  operator: 'Synthetic operator — no physical test',
  job_reference: 'SOFTWARE TEST',
  observations: 'Synthetic successful-use attestation; not machining evidence',
  detailed_notes: '',
  physically_tested: true,
  no_critical_failure: true,
};
function saved() {
  return saveCapturedPreset(undefined, capture(), form, now, 'sorbonne-software-1');
}
describe('FS3.8 local authoring', () => {
  it('captures actual manually overridden values and all known exact context without mutations', () => {
    const c = capture(),
      before = structuredClone(c),
      sienci = structuredClone(source);
    const p = saveCapturedPreset(undefined, c, form, now, 'preset');
    const profile = p.dataset.profile_set.profiles[0];
    expect(profile.data).toMatchObject({
      feed_mm_min: 4000,
      rpm: 10000,
      plunge_mm_min: 900,
      doc_mm: 2,
    });
    expect(profile.scope).toMatchObject({
      tool: { id: 'tool-6mm-software', revision: '1' },
      thickness_mm: { min: 12, max: 12 },
      operation: 'full_slot',
      engagement: { kind: 'full_width' },
    });
    expect(c).toEqual(before);
    expect(source).toEqual(sienci);
    expect(p.manifest).toMatchObject({
      visibility: 'private',
      distribution: 'local_only',
      redistributable: false,
    });
    expect(JSON.parse(JSON.stringify(p))).toEqual(p);
  });
  it('requires deliberate physical-use attestation for Tested', () => {
    expect(() =>
      saveCapturedPreset(undefined, capture(), { ...form, physically_tested: false }, now, 'a'),
    ).toThrow();
    expect(() =>
      saveCapturedPreset(undefined, capture(), { ...form, no_critical_failure: false }, now, 'a'),
    ).toThrow();
    expect(() =>
      saveCapturedPreset(undefined, capture(), { ...form, tested_at: '2026-10-13' }, now, 'a'),
    ).toThrow();
  });
  it('keeps unknown engagement as an inactive Draft, never an exact tested claim', () => {
    const c = capture();
    c.request.operation.kind = 'profile';
    c.request.operation.engagement = { kind: 'unknown' };
    expect(() => saveCapturedPreset(undefined, c, form, now, 'a')).toThrow();
    const p = saveCapturedPreset(undefined, c, { ...form, state: 'draft' }, now, 'a');
    expect(p.records[0].lab).toMatchObject({
      state: 'draft',
      enabled: false,
      physically_tested: false,
    });
    expect(p.dataset.profile_set.profiles[0].scope.engagement.kind).toBe('unknown_safe');
    expect(() => revisePreset(p, 'a', later, { enabled: true })).toThrow();
  });
  it('detects duplicates and retains stable identity, creation date and previous values on explicit update', () => {
    const p = saved(),
      before = structuredClone(p),
      c = capture();
    expect(closePresets(p, c)).toHaveLength(1);
    c.feed_mm_min = 4200;
    const next = saveCapturedPreset(p, c, form, later, 'unused', 'sorbonne-software-1');
    expect(next.records[0].profile.id).toBe(p.records[0].profile.id);
    expect(next.records[0].lab?.created_at).toBe(now);
    expect(next.records[0].lab_history?.[0].profile.data).toMatchObject({ feed_mm_min: 4000 });
    expect(next.dataset.profile_set.profiles[0].data).toMatchObject({ feed_mm_min: 4200 });
    expect(p).toEqual(before);
    expect(() => saveCapturedPreset(p, c, form, later, 'sorbonne-software-1')).toThrow();
  });
  it('does not generalize duplicate detection to other machine/material/tool/engagement', () => {
    const p = saved();
    for (const key of ['machine', 'material', 'tool'] as const) {
      const c = capture();
      c.request[key].identity = { id: 'other', revision: '1' };
      expect(closePresets(p, c)).toHaveLength(0);
    }
    const c = capture();
    c.request.operation.engagement = { kind: 'nominal', ratio: 0.25 };
    expect(closePresets(p, c)).toHaveLength(0);
  });
  it('requires all review criteria for a later Validated revision and retains Tested history', () => {
    const p = saved(),
      review = {
        policy: LAB_POLICY,
        reviewed_at: later,
        notes: 'Synthetic software review, no physical evidence',
        acceptable_finish_and_behavior: true,
        no_overheating_chatter_or_overload: true,
        suitable_for_lab_use: true,
      };
    expect(() =>
      revisePreset(p, 'sorbonne-software-1', later, {
        review: { ...review, suitable_for_lab_use: false },
      }),
    ).toThrow();
    const n = revisePreset(p, 'sorbonne-software-1', later, { review });
    expect(n.records[0].lab?.state).toBe('validated');
    expect(n.records[0].lab_history?.[0].metadata.state).toBe('tested');
    expect(
      n.dataset.profile_set.profiles[0].source.evidence_record?.validation_criteria_reference,
    ).toBe(LAB_POLICY);
  });
  it('disables and reversibly deletes without destructive record removal', () => {
    const p = saved();
    const n = revisePreset(p, 'sorbonne-software-1', later, { deleted: true });
    expect(n.records[0].lab).toMatchObject({ deleted: true, enabled: false });
    expect(n.records[0].lab_history).toHaveLength(1);
    const restored = revisePreset(n, 'sorbonne-software-1', '2026-10-12T10:00:00.000Z', {
      deleted: false,
    });
    expect(restored.records[0].lab).toMatchObject({ deleted: false, enabled: true });
  });
  it('rejects invalid values, missing identities and public pack authoring', () => {
    for (const val of [0, -1, NaN]) {
      const c = capture();
      c.rpm = val;
      expect(() => captureScope(c)).toThrow();
    }
    const c = capture();
    c.request.tool.identity = null;
    expect(() => captureScope(c)).toThrow();
    expect(() => saveCapturedPreset(source as never, capture(), form, later, 'a')).toThrow();
  });
  it('selects one current immutable Sorbonne revision without losing old revision objects', () => {
    const a = saved(),
      b = revisePreset(a, 'sorbonne-software-1', later, { notes: 'updated' });
    expect(newestLabPack([a, b])).toBe(b);
    expect(a.records[0].lab?.observations).toBe(form.observations);
  });
});

it('uses stable identity labels when a legacy machine or tool has no display name', () => {
  const c = capture();
  c.machine_name = '';
  c.tool_name = '';
  const p = saveCapturedPreset(
    undefined,
    c,
    { ...form, state: 'draft' },
    now,
    'sorbonne-empty-label',
  );
  expect(p.dataset.machines[0].label).toBe(c.request.machine.identity!.id);
  expect(p.dataset.tools[0].label).toBe(c.request.tool.identity!.id);
});

it('keeps physical identity through a display rename', () => {
  const c = capture();
  const p = saveCapturedPreset(undefined, c, form, now, 'sorbonne-rename');
  c.machine_name = 'Renamed machine';
  c.tool_name = 'Renamed cutter';
  const next = saveCapturedPreset(p, c, form, later, 'sorbonne-rename-new');
  expect(next.dataset.machines).toHaveLength(1);
  expect(next.dataset.tools[0].identity).toEqual(p.dataset.tools[0].identity);
  expect(next.dataset.tools[0].label).toBe('Renamed cutter');
});

it('can revise a reactive proxy-backed JSON pack without mutating it', () => {
  const original = saveCapturedPreset(undefined, capture(), form, now, 'sorbonne-proxy');
  const reactive = new Proxy(original, {});
  const next = revisePreset(reactive, 'sorbonne-proxy', later, { notes: 'updated observations' });
  expect(next.records[0].lab!.observations).toBe('updated observations');
  expect(original.records[0].lab!.observations).not.toBe('updated observations');
  expect(next.records[0].lab_history).toHaveLength(1);
});

it('detects duplicates after Rust JSON normalization changes property order', () => {
  const c = capture();
  const p = saveCapturedPreset(undefined, c, form, now, 'sorbonne-order');
  p.dataset.profile_set.profiles[0].scope = Object.fromEntries(
    Object.entries(p.dataset.profile_set.profiles[0].scope).reverse(),
  ) as (typeof p.dataset.profile_set.profiles)[0]['scope'];
  delete p.dataset.tools[0].max_rpm;
  expect(closePresets(p, c)).toHaveLength(1);
  const next = saveCapturedPreset(p, c, form, later, 'unused', 'sorbonne-order');
  expect(next.records).toHaveLength(1);
  expect(next.records[0].lab_history).toHaveLength(1);
});
