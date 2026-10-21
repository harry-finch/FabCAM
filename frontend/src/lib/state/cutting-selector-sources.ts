import { readLocalPacks, validatePack } from './feeds-speeds-pack-store';
import { enabledPacks, type SourceSettings, type SourcePack } from './feeds-speeds-sources';
import { currentLocalPacks } from './lab-presets';
/** Fresh storage read without mutating the visible preview or following a new revision. */
export async function freshCuttingSources(settings: SourceSettings, builtins: SourcePack[]) {
  const local = await Promise.all((await readLocalPacks()).map(validatePack));
  return enabledPacks(settings, builtins, currentLocalPacks(local));
}
