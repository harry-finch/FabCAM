/** Display only: resolve exact recorded identities; never infer names from IDs or notes. */
import type { components } from '../api/generated';
import type { SourcePack } from './feeds-speeds-sources';
import { identityLabel } from './preset-library';
type Selection = components['schemas']['FsSelectedRecord'];
export function trialPresentation(source: Selection, packs: SourcePack[]) {
  const pack = packs.find(
    (p) =>
      p.manifest.identity.id === source.pack.id &&
      p.manifest.identity.revision === source.pack.revision,
  );
  const record = pack?.records.find(
    (r) =>
      r.source_record_id === source.record_id &&
      r.profile.id === source.profile.id &&
      r.profile.revision === source.profile.revision &&
      !r.lab?.deleted,
  );
  const profile =
    record &&
    pack?.dataset.profile_set.profiles.find(
      (p) => p.identity.id === source.profile.id && p.identity.revision === source.profile.revision,
    );
  if (!pack || !record || !profile) return undefined;
  // Recipe authoring records this exact reference. Do not parse or render free-form source notes.
  const origin = packs
    .flatMap((p) => p.dataset.profile_set.profiles.map((profile) => ({ pack: p, profile })))
    .find(
      ({ pack, profile: p }) =>
        profile.source.reference ===
        `${pack.manifest.identity.id}@${pack.manifest.identity.revision}/${p.identity.id}@${p.identity.revision}`,
    );
  const original = origin && {
    publisher: origin.pack.manifest.publisher,
    material:
      origin.profile.scope.material.kind === 'exact'
        ? identityLabel(origin.profile.scope.material.identity, origin.pack.dataset.materials)
        : undefined,
    mode: origin.profile.aggressiveness,
    pack: origin.pack.manifest.identity,
    profile: origin.profile.identity,
  };
  return {
    name: record.lab?.name || record.original_classification || pack.manifest.name,
    publisher: pack.manifest.publisher,
    original,
  };
}
