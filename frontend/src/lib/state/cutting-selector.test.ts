import { describe, expect, it, vi } from 'vitest';
import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v3.json';
import { loadedPack, type SourcePack } from './feeds-speeds-sources';
import { presetRows } from './preset-library';
import {
  recordSelection,
  unavailableRecord,
  assertSameSources,
  applyReviewedCutting,
} from './cutting-selector';
const pack = () => structuredClone(reference) as unknown as SourcePack;
const rows = () => [loadedPack(pack())];
describe('unified cutting selector identity and application boundary', () => {
  it('pins all original identity dimensions without rewriting the source', () => {
    const p = pack(),
      before = JSON.stringify(p);
    const r = presetRows(
      [p],
      new Set([`${p.manifest.identity.id}@${p.manifest.identity.revision}`]),
      true,
    )[0];
    expect(recordSelection(r)).toEqual({
      pack: p.manifest.identity,
      record_id: r.record!.source_record_id,
      profile: r.profile.identity,
    });
    expect(unavailableRecord(r)).toBe(false);
    expect(unavailableRecord({ ...r, enabled: false })).toBe(true);
    expect(unavailableRecord({ ...r, milling: false })).toBe(true);
    expect(JSON.stringify(p)).toBe(before);
  });
  it('rejects removed, edited and revised records, independently of collection order', () => {
    const original = rows();
    expect(() => assertSameSources(original, rows())).not.toThrow();
    expect(() => assertSameSources(original, [])).toThrow(/changed/);
    const changed = pack();
    changed.records[0].original_classification += 'edited';
    expect(() => assertSameSources(original, [loadedPack(changed)])).toThrow(/changed/);
    changed.manifest.identity.revision += 'next';
    expect(() => assertSameSources(original, [loadedPack(changed)])).toThrow(/changed/);
  });
  it('never validates or commits unacknowledged manual replacements', async () => {
    const validate = vi.fn(),
      commit = vi.fn();
    await expect(
      applyReviewedCutting({
        previewSources: rows(),
        readSources: async () => rows(),
        isCurrent: () => true,
        requiresAcknowledgement: true,
        acknowledged: false,
        validate,
        commit,
      }),
    ).rejects.toThrow(/Confirm/);
    expect(validate).not.toHaveBeenCalled();
    expect(commit).not.toHaveBeenCalled();
  });
  it.each(['operation', 'sources', 'refused'])(
    'preserves values when %s changes during authoritative Apply',
    async (reason) => {
      let current = true,
        changed = false;
      const commit = vi.fn();
      await expect(
        applyReviewedCutting({
          previewSources: rows(),
          readSources: async () => (changed ? [] : rows()),
          isCurrent: () => current,
          requiresAcknowledgement: false,
          acknowledged: false,
          validate: async () => {
            if (reason === 'refused') throw new Error('refused');
            if (reason === 'operation') current = false;
            else changed = true;
            return { feed: 100 };
          },
          commit,
        }),
      ).rejects.toThrow();
      expect(commit).not.toHaveBeenCalled();
    },
  );
  it('both discovery modes commit only the authoritative result through the same boundary', async () => {
    for (const mode of ['recommended', 'browse']) {
      const result = { feed: 100, provenance: mode };
      const commit = vi.fn();
      await applyReviewedCutting({
        previewSources: rows(),
        readSources: async () => rows(),
        isCurrent: () => true,
        requiresAcknowledgement: true,
        acknowledged: true,
        validate: async () => result,
        commit,
      });
      expect(commit).toHaveBeenCalledTimes(1);
      expect(commit).toHaveBeenCalledWith(result);
    }
  });
});
