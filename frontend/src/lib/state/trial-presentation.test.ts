import { expect, it } from 'vitest';
import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v1.json';
import { trialPresentation } from './trial-presentation';
import type { SourcePack } from './feeds-speeds-sources';
import { trialOp } from './experimental-recipe.fixture';
import { experimentalModified } from './experimental-recipe';
function fixture() {
  const original = structuredClone(reference) as unknown as SourcePack;
  const draft = structuredClone(original);
  draft.manifest.identity = { id: 'sorbonne-fablab', revision: '1' };
  const profile = draft.dataset.profile_set.profiles[0];
  const record = draft.records[0];
  record.lab = { name: 'Workshop recipe', deleted: false } as NonNullable<typeof record.lab>;
  profile.source.reference = `${original.manifest.identity.id}@${original.manifest.identity.revision}/${profile.identity.id}@${profile.identity.revision}`;
  profile.source.notes = 'PRIVATE NEVER DISPLAY';
  const source = {
    pack: draft.manifest.identity,
    profile: profile.identity,
    record_id: record.source_record_id,
  };
  return { original, draft, source };
}
it('uses authored name and exact upstream labels without leaking notes or narrowing the material category', () => {
  const { original, draft, source } = fixture();
  const view = trialPresentation(source, [original, draft])!;
  expect(view.name).toBe('Workshop recipe');
  expect(view.original?.publisher).toBe('Sienci Labs');
  expect(view.original?.material).toBe(original.dataset.materials[0].label);
  expect(JSON.stringify(view)).not.toContain('PRIVATE');
});
it('does not guess missing, deleted or revised source names; disabled source labels remain usable', () => {
  const { original, draft, source } = fixture();
  expect(trialPresentation(source, [])).toBeUndefined();
  expect(trialPresentation(source, [draft])?.original).toBeUndefined();
  // Availability is independent from display lookup, not permission to apply.
  expect(trialPresentation(source, [draft])?.name).toBe('Workshop recipe');
  draft.records[0].lab!.deleted = true;
  expect(trialPresentation(source, [draft])).toBeUndefined();
  draft.records[0].lab!.deleted = false;
  draft.manifest.identity = { ...draft.manifest.identity, revision: '2' };
  draft.records[0].lab!.name = 'Renamed';
  expect(trialPresentation(source, [original, draft])).toBeUndefined();
});
it('reopened experimental values retain ancestry and modified presentation independent of source availability', () => {
  const op = trialOp();
  op.appliedCutting!.slots[0].feed_mm_min += 1;
  const reopened = JSON.parse(JSON.stringify(op));
  expect(experimentalModified(reopened)).toBe(true);
  expect(reopened.appliedCutting.experimental_recipe).toEqual(
    op.appliedCutting!.experimental_recipe,
  );
  expect(trialPresentation(reopened.appliedCutting.experimental_recipe.source, [])).toBeUndefined();
});
