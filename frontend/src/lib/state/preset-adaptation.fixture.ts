import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v3.json';
import { defaultProjectMachine, defaultProjectTools } from './project-defaults';
import { createLabPack } from './lab-presets';
import { presetRows } from './preset-library';
import type { SourcePack } from './feeds-speeds-sources';
import type { AdaptTarget } from './preset-adaptation';
export function adaptationFixture() {
  const pack = structuredClone(reference) as unknown as SourcePack;
  const row = presetRows(
    [pack],
    new Set([`${pack.manifest.identity.id}@${pack.manifest.identity.revision}`]),
    true,
  )[0];
  const machine = defaultProjectMachine();
  machine.fsIdentity = { id: 'sienci-longmill-mk2-reference', revision: '1' };
  const tool = defaultProjectTools()[0];
  tool.fsLocalIdentity = { id: 'physical-tool', revision: '1' };
  tool.diameter = 6.35;
  tool.flutes = 1;
  const target: AdaptTarget = {
    machine,
    tool,
    material: { id: 'material/wood.mdf', revision: '1', label: 'MDF' },
  };
  const destination = createLabPack(
    'SOFTWARE ONLY workshop',
    '2026-10-20T10:00:00.000Z',
    'lab-00000000-0000-4000-8000-000000000001',
  );
  return { row, target, destination };
}
