import { describe, it, expect } from 'vitest';
import { adaptationFixture } from './preset-adaptation.fixture';
import {
  adaptationProblems,
  adaptationWarnings,
  adaptationDifferences,
  duplicatePresetAsDraft,
  localCopyAttestation,
} from './preset-adaptation';
const now = '2026-10-21T10:00:00.000Z';
const form = {
  name: 'SOFTWARE ONLY copy',
  reviewed: true,
  ownershipConfirmed: false,
  notes: 'Not a physical trial',
};
describe('Independent inactive recipe authoring', () => {
  it('preserves values/license/provenance, clears evidence, and leaves inputs unchanged', () => {
    const { row, target, destination } = adaptationFixture();
    const before = JSON.stringify({ row, target, destination });
    expect(adaptationProblems(row, target, false)).toEqual([]);
    const copy = duplicatePresetAsDraft(destination, row, target, form, now, 'new-id');
    const p = copy.dataset.profile_set.profiles[0],
      r = copy.records[0];
    expect(p.data).toEqual(row.profile.data);
    expect(p.identity.id).toBe('new-id');
    expect(p.source.permission_or_license).toBe(row.profile.source.permission_or_license);
    expect(p.source.notes).toContain(row.pack.manifest.attribution);
    expect(p.source.notes).toContain(row.profile.identity.id);
    expect(p.source.evidence_record).toBeNull();
    expect(p.evidence).toBe('theoretical');
    expect(r.lab).toMatchObject({
      state: 'draft',
      enabled: false,
      tested_at: null,
      physically_tested: false,
      no_critical_failure: false,
      review: null,
      operator: '',
      job_reference: '',
    });
    expect(r.lab_history).toEqual([]);
    expect(JSON.stringify({ row, target, destination })).toBe(before);
  });
  it('flags diameter/flutes changes and preserves chipload-only values without inventing feed', () => {
    const { row, target, destination } = adaptationFixture();
    target.tool!.diameter = 3;
    target.tool!.flutes = 2;
    row.profile.data = {
      kind: 'chipload_target',
      rpm: 18000,
      chipload_mm_tooth: 0.025,
      plunge_mm_min: 100,
      doc_mm: 1,
    };
    expect(adaptationDifferences(row, target).map((d) => d.field)).toEqual(
      expect.arrayContaining(['diameter', 'flutes']),
    );
    const copy = duplicatePresetAsDraft(destination, row, target, form, now, 'new-id');
    expect(copy.dataset.profile_set.profiles[0].data).toEqual(row.profile.data);
    expect(copy.dataset.profile_set.profiles[0].data).not.toHaveProperty('feed_mm_min');
    expect(copy.dataset.profile_set.profiles[0].scope.diameter_mm).toEqual({ min: 3, max: 3 });
  });
  it('requires explicit material relationships and exact known taxonomy revisions', () => {
    const { row, target } = adaptationFixture();
    for (const material of [
      { id: 'MDF', revision: '1', label: 'WOOD/MDF' },
      { id: 'material/wood.mdf', revision: '2' },
      { id: 'material/wood.mdf.invented', revision: '1' },
    ])
      expect(adaptationWarnings(row, { ...target, material })).toContain('material');
    expect(adaptationWarnings(row, target)).not.toContain('material');
  });
  it('blocks generic unlicensed references and requires deliberate local authorship authorization', () => {
    const { row, target, destination } = adaptationFixture();
    row.pack.manifest.transformation_permitted = false;
    expect(adaptationProblems(row, target, true)).toContain('rights');
    row.pack.manifest.identity.id = destination.manifest.identity.id;
    row.pack.manifest.visibility = 'private';
    row.pack.manifest.distribution = 'local_only';
    row.record!.lab = {
      name: 'Own trial',
      state: 'tested',
      created_at: now,
      updated_at: now,
      tested_at: '2026-10-20',
      physically_tested: true,
      no_critical_failure: true,
      enabled: true,
      deleted: false,
      review: null,
      operator: 'Synthetic',
      job_reference: 'SOFTWARE ONLY',
      observations: '',
      detailed_notes: '',
      finishing_context: '',
    };
    row.profile.scope.machine = { kind: 'exact', identity: target.machine.fsLocalIdentity! };
    expect(localCopyAttestation(row)).toBe(true);
    expect(adaptationProblems(row, target, false)).toContain('rights');
    expect(adaptationProblems(row, target, true)).toEqual([]);
    const copy = duplicatePresetAsDraft(
      destination,
      row,
      target,
      { ...form, ownershipConfirmed: true },
      now,
      'new-id',
    );
    expect(copy.records[0].lab?.tested_at).toBeNull();
    expect(copy.records[0].lab?.physically_tested).toBe(false);
  });
  it('rejects missing/ambiguous identities, incompatible machines/families/processes, short tools and missing review', () => {
    const { row, target, destination } = adaptationFixture();
    expect(adaptationProblems(row, { ...target, tool: undefined }, false)).toContain('identity');
    const ambiguous = structuredClone(target);
    ambiguous.tool!.fsLocalIdentity = ambiguous.tool!.fsIdentity = { id: 'legacy', revision: '1' };
    expect(adaptationProblems(row, ambiguous, false)).toContain('identity');
    const laser = structuredClone(target);
    laser.machine.mode = 'laser';
    laser.machine.capabilities = ['laser'];
    expect(adaptationProblems(row, laser, false)).toContain('process');
    const other = structuredClone(target);
    other.machine.fsIdentity = { id: 'other', revision: '1' };
    expect(adaptationWarnings(row, other)).toContain('machine');
    expect(adaptationProblems(row, other, false)).not.toContain('machine');
    const ball = structuredClone(target);
    ball.tool!.kind = 'ball_nose';
    expect(adaptationProblems(row, ball, false)).toContain('tool');
    const short = structuredClone(target);
    short.tool!.fluteLengthMm = 0.5;
    expect(adaptationProblems(row, short, false)).toContain('geometry');
    expect(adaptationProblems({ ...row, enabled: false }, target, false)).toContain('source');
    expect(() =>
      duplicatePresetAsDraft(destination, row, target, { ...form, reviewed: false }, now, 'new-id'),
    ).toThrow('Review');
  });
  it('keeps recorded RPM limits and refuses geometry changes under an existing physical identity', () => {
    const { row, target, destination } = adaptationFixture();
    destination.dataset.tools.push({
      identity: target.tool!.fsLocalIdentity!,
      label: 'tool',
      notes: '',
      cutting_family: 'flat_endmill',
      diameter_mm: 6.35,
      flute_count: 1,
      max_rpm: 20000,
    });
    const copy = duplicatePresetAsDraft(destination, row, target, form, now, 'new-id');
    expect(copy.dataset.tools[0].max_rpm).toBe(20000);
    expect(() =>
      duplicatePresetAsDraft(
        destination,
        row,
        target,
        { ...form, data: { ...row.profile.data, rpm: 21000 } },
        now,
        'over-limit',
      ),
    ).toThrow('tool limit');
    target.tool!.diameter = 3;
    expect(() => duplicatePresetAsDraft(destination, row, target, form, now, 'new-id')).toThrow();
  });
});
