import { describe, expect, it } from 'vitest';
import { fixtureCollisions } from './fixture-collisions';
import type { Fixture } from './project-types';
import type { PipelineWarning } from '../api/pipeline-warnings';
const fixtures: Fixture[] = [
  {
    id: 4,
    name: 'Clamp 4',
    origin: [7, 60],
    z_bottom: 0,
    z_top: 10,
    color: 0,
    kind: { shape: 'box', width: 20, depth: 10 },
  },
];
const warnings: PipelineWarning[] = [
  {
    kind: 'fixture_path_collision',
    message: 'Candidate collision',
    params: { fixture_id: '4', op_id: '1' },
  },
];
describe('fixture collision presentation', () => {
  it('uses existing real fixture identity and exposes no fabricated point', () => {
    expect(fixtureCollisions(fixtures, warnings, [], false)).toEqual([
      { fixtureId: 4, name: 'Clamp 4', opId: 1 },
    ]);
  });
  it('drops stale verdicts, unknown objects and warning-only clearance uncertainty', () => {
    expect(fixtureCollisions(fixtures, warnings, [], true)).toEqual([]);
    expect(fixtureCollisions([], warnings, [], false)).toEqual([]);
    expect(
      fixtureCollisions(
        fixtures,
        [{ ...warnings[0], kind: 'fixture_clearance_unknown' }],
        [],
        false,
      ),
    ).toEqual([]);
  });
});
