import type { OpEntry } from './op_types';
import type { components } from '../api/generated';
const id = { id: 'synthetic', revision: '1' };
export function trialOp(): OpEntry {
  const recipe: components['schemas']['ExperimentalRecipe'] = {
    kind: 'theoretical_untested',
    source: {
      pack: { id: 'sorbonne-fablab', revision: '0001791792000000' },
      record_id: 'draft-one',
      profile: id,
    },
    target: {
      machine: id,
      material: id,
      tool: id,
      tool_id: 1,
      operation: 'profile',
      diameter_mm: 6,
      flute_count: 2,
    },
    applied_values: {
      rpm: 18000,
      feed_mm_min: 1200,
      plunge_mm_min: 300,
      doc_mm: 2,
      stepover_ratio: null,
    },
  };
  return {
    id: 1,
    name: 'Synthetic trial',
    enabled: true,
    kind: 'profile',
    offset: 'outside',
    toolId: 1,
    sourceLayers: null,
    depth: -4,
    startDepth: 0,
    step: -2,
    appliedCutting: {
      experimental_recipe: recipe,
      context_key: '',
      doc_origin: 'manual',
      slots: (['rough', 'finish'] as const).map((pass) => ({
        tool_id: 1,
        pass,
        rpm: 18000,
        feed_mm_min: 1200,
        plunge_mm_min: 300,
        origins: { rpm: 'manual', feed: 'manual', plunge: 'manual' },
      })),
    },
  };
}
