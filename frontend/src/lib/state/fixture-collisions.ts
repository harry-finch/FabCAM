import type { Fixture } from './project-types';
import type { PipelineWarning } from '../api/pipeline-warnings';
import type { SimWarning } from '../api/types';

export interface FixtureCollision {
  fixtureId: number;
  name: string;
  opId?: number;
}
/** Pipeline gives fixture/op identity, not a precise collision point. */
export function fixtureCollisions(
  fixtures: readonly Fixture[],
  pipeline: readonly PipelineWarning[],
  sim: readonly SimWarning[],
  stale: boolean,
): FixtureCollision[] {
  if (stale) return [];
  const found = new Map<number, FixtureCollision>();
  for (const w of pipeline) {
    if (w.kind !== 'fixture_path_collision') continue;
    const id = Number(w.params?.fixture_id);
    const f = fixtures.find((f) => f.id === id);
    if (f)
      found.set(id, { fixtureId: id, name: f.name, opId: Number(w.params?.op_id) || undefined });
  }
  for (const w of sim) {
    if (w.kind !== 'fixture_collision') continue;
    const f = fixtures.find((f) => f.id === w.fixture_id);
    if (f && !found.has(f.id)) found.set(f.id, { fixtureId: f.id, name: f.name });
  }
  return [...found.values()];
}
