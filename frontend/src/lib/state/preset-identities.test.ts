import { expect, it } from 'vitest';
import { missingPresetIdentities } from './preset-identities';
import type { components } from '../api/generated';
it('lists missing identities separately and checks the selected tool binding', () => {
  const p = {
    fs_context: {
      machine_identity: { id: 'm', revision: '1' },
      tools: [{ tool_id: 2, identity: { id: 't', revision: '1' } }],
    },
    material: { id: 'wood', revision: '1' },
  } as components['schemas']['Project'];
  expect(missingPresetIdentities(p, 2)).toEqual([]);
  expect(missingPresetIdentities(p, 3)).toEqual(['tool']);
  p.material = undefined;
  expect(missingPresetIdentities(p, 2)).toEqual(['material']);
  p.fs_context = undefined;
  expect(missingPresetIdentities(p, 2)).toEqual(['machine', 'material', 'tool']);
});
