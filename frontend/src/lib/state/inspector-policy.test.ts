import { describe, expect, it } from 'vitest';
import {
  depthFields,
  storedPassDepth,
  operationCapability,
  operationRunState,
} from './inspector-policy';
import type { MachineSettings } from './project-types';

describe('lot A intent and depth consumers', () => {
  it('retains chamfer descent and undercut entry, hides unused schedules', () => {
    expect(depthFields('chamfer')).toMatchObject({
      final: false,
      start: false,
      step: true,
      finishStep: true,
      list: false,
    });
    for (const kind of ['t_slot', 'dovetail'] as const)
      expect(depthFields(kind)).toMatchObject({
        final: true,
        start: true,
        step: false,
        list: false,
        through: false,
      });
    expect(depthFields('thread')).toMatchObject({ final: true, start: true, step: false });
    expect(depthFields('drill')).toMatchObject({
      start: true,
      step: false,
      list: false,
      through: true,
    });
  });
  it('keeps inherited depths separate and rejects malformed/negative UI thickness', () => {
    expect(storedPassDepth('')).toBeNull();
    expect(storedPassDepth('1.25')).toBe(-1.25);
    for (const input of ['-1', '0', 'NaN', 'Infinity', '2abc'])
      expect(storedPassDepth(input)).toBeUndefined();
  });
  it('does not certify newly added, stale, cancelled or diagnostic output', () => {
    expect(operationRunState(true, false, false, 'idle')).toBe('not_generated');
    expect(operationRunState(true, true, false, 'idle')).toBe('stale');
    expect(operationRunState(true, true, true, 'cancelling')).toBe('pending');
    expect(operationRunState(false, true, true, 'idle')).toBe('disabled');
    expect(operationRunState(true, true, true, 'idle')).toBe('current');
  });
  it('distinguishes CPS motion refusal, skipped annotation and bundled ShopBot raw commands', () => {
    const machine = { mode: 'mill' } as MachineSettings;
    machine.cpsPost = { source: 'bundled', bundledId: 'shopbot', properties: {} };
    expect(operationCapability('probe', 'cps', machine)).toBe('uxa.cps_motion_blocked');
    expect(operationCapability('homing', 'linuxcnc', machine)).toBeNull();
    expect(operationCapability('cycle_marker', 'cps', machine)).toBe('uxa.cps_marker_skipped');
    expect(operationCapability('gcode_include', 'cps', machine)).toBe('uxa.shopbot_raw_blocked');
    expect(operationCapability('pause', 'cps', machine)).toBeNull();
    machine.cpsPost = { source: 'file', script: '', properties: {} };
    expect(operationCapability('gcode_include', 'cps', machine)).toBeNull();
  });
});
