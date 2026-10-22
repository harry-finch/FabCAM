import { describe, expect, it } from 'vitest';
import { assignmentReview, newMachineDraft } from './machine-profile-review';
import { profileFromCurrent, profilePayload } from './machine_profiles';
import type { ToolEntry } from './project-types';

const tool = (id: number, diameter = 6): ToolEntry => ({
  id,
  physicalId: `physical-tool-${id}`,
  name: `Tool ${id}`,
  kind: 'endmill',
  diameter,
  flutes: 2,
  speed: 12000,
  feedRate: 600,
  plungeRate: 150,
  coolant: 'off',
});

describe('UX6-A profile consequence review', () => {
  it('reports affected operations for missing and same-ID changed physical tools without rewriting anything', () => {
    const current = [tool(1), tool(2), tool(3)];
    const profile = profileFromCurrent(newMachineDraft(), [tool(1), tool(2, 3)], [], 'other');
    const operations = [
      { name: 'Unchanged', toolId: 1 },
      { name: 'Pocket', toolId: 2 },
      { name: 'Finish', toolId: 2 },
      { name: 'Drill', toolId: 3 },
    ];
    const before = JSON.stringify({ profile, current, operations });
    expect(assignmentReview(profile, current, operations)).toEqual([
      { name: 'Pocket', toolId: 2, reason: 'changed' },
      { name: 'Finish', toolId: 2, reason: 'changed' },
      { name: 'Drill', toolId: 3, reason: 'missing' },
    ]);
    expect(JSON.stringify({ profile, current, operations })).toBe(before);
  });
  it('recognizes same-ID changes to legacy cutting defaults and holders, not only diameter', () => {
    const previous = tool(1);
    const next = { ...previous, feedRate: 900, wearOffset: 0.1 };
    const p = profileFromCurrent(newMachineDraft(), [next], [], 'p');
    expect(assignmentReview(p, [previous], [{ name: 'Contour', toolId: 1 }])[0].reason).toBe(
      'changed',
    );
  });
  it('uses migrated copies like assignment and accepts structurally equal definitions', () => {
    const p = profileFromCurrent(newMachineDraft(), [tool(1)], [], 'p');
    const payload = profilePayload(p);
    expect(assignmentReview(p, payload.tools, [{ name: 'Same', toolId: 1 }])).toEqual([]);
    payload.machine.name = 'changed';
    payload.tools[0].diameter = 20;
    expect(p.machine.name).toBe('');
    expect(p.tools[0].diameter).toBe(6);
  });
  it('creates independent drafts with no invented feed/RPM limits or stocked tools', () => {
    const a = newMachineDraft(),
      b = newMachineDraft();
    a.workArea!.x = 900;
    expect(b.workArea!.x).toBe(200);
    expect(b.spindleRpmMin).toBeUndefined();
    expect(b.spindleRpmMax).toBeUndefined();
    expect(b.maxFeedMmMin).toBeUndefined();
    expect(profileFromCurrent(b, [], []).tools).toEqual([]);
  });
});
