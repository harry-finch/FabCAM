import { describe, expect, it } from 'vitest';
import { History } from './history';
import { updateOperationCommand } from './commands';
import { incompatibleVCarveSettings, isVCarveRefusal } from './vcarve-controls';
import type { VCarveOp } from './op_types';
import { buildOpEntry } from './op_defaults';
import { pipelineWarningSeverity } from '../api/pipeline-warnings';

function fixture(): VCarveOp {
  return buildOpEntry('vcarve', {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  }) as VCarveOp;
}
describe('V-Carve compatibility presentation', () => {
  it('does not mutate valid settings in either saved mode', () => {
    for (const fullMedialAxis of [false, true]) {
      const op = {
        ...fixture(),
        fullMedialAxis,
        depth: -1,
        step: -0.2,
        carveMaxWidthMm: 2,
        sourceInsetMm: 0.1,
      };
      const before = structuredClone(op);
      expect(incompatibleVCarveSettings(op)).toEqual([]);
      expect(op).toEqual(before);
    }
  });
  it('reveals every persisted unsupported setting without normalizing it', () => {
    const op = {
      ...fixture(),
      startDepth: -1,
      throughDepth: 0.2,
      depthList: [-0.3, -1],
      finishStep: -0.1,
      multiPassRefine: true,
    };
    const restored = JSON.parse(JSON.stringify(op));
    expect(incompatibleVCarveSettings(restored).map((s) => s.field)).toEqual([
      'startDepth',
      'throughDepth',
      'depthList',
      'finishStep',
      'multiPassRefine',
    ]);
    expect(restored).toEqual(op);
    // Only explicit per-setting actions produce patches; unrelated authored values remain.
    for (const setting of incompatibleVCarveSettings(restored)) {
      const next = { ...restored, [setting.field]: setting.reset };
      expect(incompatibleVCarveSettings(next)).toHaveLength(4);
      expect(next.depth).toBe(op.depth);
      expect(next.step).toBe(op.step);
    }
  });
  it('does not treat an explicit finish step of zero as an absent setting', () => {
    expect(incompatibleVCarveSettings({ ...fixture(), finishStep: 0 }).map((s) => s.field)).toEqual(
      ['finishStep'],
    );
  });
  it('keeps V-Carve operation refusals visible without affecting other errors', () => {
    const ops = [
      { id: 1, kind: 'vcarve' },
      { id: 2, kind: 'pocket' },
    ];
    expect(isVCarveRefusal({ kind: 'misconfigured', params: { op_id: '1' } }, ops)).toBe(true);
    expect(isVCarveRefusal({ kind: 'misconfigured', params: { op_id: '2' } }, ops)).toBe(false);
    expect(isVCarveRefusal('other error', ops)).toBe(false);
    expect(isVCarveRefusal({ kind: 'misconfigured' }, ops)).toBe(false);
  });
  it('preserves the audited warning severities and blocking errors', () => {
    for (const kind of ['vcarve_scope', 'vcarve_target_capped', 'cps_post_warning']) {
      expect(pipelineWarningSeverity({ kind, message: '' })).toBe('warning');
    }
    expect(pipelineWarningSeverity({ kind: 'vcarve_below_tip_radius', message: '' })).toBe(
      'critical',
    );
    expect(pipelineWarningSeverity({ kind: 'fixture_path_collision', message: '' })).toBe(
      'critical',
    );
  });
});

it('explicit clearing is undoable and changing the operation kind preserves authored fields', () => {
  const op = { ...fixture(), startDepth: -0.5, finishStep: -0.1, multiPassRefine: true };
  const state = { operations: [op], dirty: false };
  const history = new History();
  history.exec(updateOperationCommand(op.id, { startDepth: 0 }), state);
  expect(state.operations[0].startDepth).toBe(0);
  history.undo(state);
  expect(state.operations[0]).toEqual(op);
  history.redo(state);
  expect(state.operations[0].startDepth).toBe(0);
  history.exec(updateOperationCommand(op.id, { kind: 'pocket' }), state);
  expect(state.operations[0]).toMatchObject({ finishStep: -0.1, multiPassRefine: true });
  history.undo(state);
  expect(state.operations[0]).toMatchObject({
    kind: 'vcarve',
    finishStep: -0.1,
    multiPassRefine: true,
  });
});
