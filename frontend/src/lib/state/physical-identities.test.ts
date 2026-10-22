import { expect, it } from 'vitest';
import { withPhysicalIdentity } from './physical-identities';
import { defaultProjectMachine, defaultProjectTools } from './project-defaults';
it('gives new machines and tools independent stable physical identities', () => {
  const a = defaultProjectMachine(),
    b = defaultProjectMachine();
  expect(a.physicalId).toMatch(/^lab-machine-/);
  expect(b.physicalId).not.toBe(a.physicalId);
  const tool = defaultProjectTools()[0];
  expect(tool.physicalId).toMatch(/^lab-tool-/);
  expect(withPhysicalIdentity(a, 'machine')).toBe(a);
  expect(withPhysicalIdentity(tool, 'tool')).toBe(tool);
});
it('preserves existing workshop library identities and opaque data without activating reference packs', () => {
  const stored = {
    physicalId: undefined,
    fsLocalIdentity: { id: 'our-lab-tool', revision: '7' },
    fsIdentity: { id: 'official-tool', revision: '2' },
    privateNote: 'keep',
  };
  const before = structuredClone(stored);
  const loaded = withPhysicalIdentity(stored, 'tool');
  expect(loaded).toEqual({ ...stored, physicalId: 'our-lab-tool' });
  expect(stored).toEqual(before);
  expect(withPhysicalIdentity(JSON.parse(JSON.stringify(loaded)), 'tool')).toEqual(loaded);
});
it('never invents a physical match from an official reference or silently repairs authored identities', () => {
  expect(
    withPhysicalIdentity(
      { physicalId: undefined, fsIdentity: { id: 'official', revision: '1' } },
      'machine',
    ).physicalId,
  ).toMatch(/^lab-machine-/);
  for (const physicalId of ['explicit', '']) {
    const stored = { physicalId, fsLocalIdentity: { id: 'different' } };
    expect(withPhysicalIdentity(stored, 'tool')).toBe(stored);
  }
});
