import { expect, it } from 'vitest';
import { programTools } from './tool-provenance';
import { toWireTool } from './config';
import type { GenerateResponse } from '../api/types';
function program(): GenerateResponse {
  const tool = {
    id: 41,
    name: 'rough',
    kind: 'endmill',
    diameter: 8,
    flutes: 2,
    speed: 18000,
    feed_rate: 600,
    plunge_rate: 100,
    coolant: 'off',
  };
  return {
    tool_provenance: {
      revision: 1,
      authority: 'resolved',
      transitions: [],
      diagnostics: [],
      bindings: [
        { reference: 1, project_tool_id: 41, program_tool_number: 41, tool },
        {
          reference: 2,
          project_tool_id: 7,
          program_tool_number: 7,
          tool: {
            ...tool,
            id: 7,
            name: 'finish',
            diameter: 2,
            wear_offset_mm: 0.2,
            flute_length_mm: 5,
            holder: { kind: 'cylinder', diameter_mm: 20, length_mm: 20 },
          },
        },
      ],
    },
    toolpath: [1, 2].map((tool_ref) => ({
      op_id: 99,
      tool_ref,
      kind: 'cut',
      from: { x: 0, y: 0, z: -2 },
      to: { x: 10, y: 0, z: -2 },
      gcode_line: tool_ref,
    })),
  } as unknown as GenerateResponse;
}
it('one op resolves separate immutable full bodies; grid uses effective cutting diameter', () => {
  const p = program();
  const r = programTools(p, true);
  expect(r.resolve(p.toolpath[0]).diameter).toBe(8);
  expect(r.resolve(p.toolpath[1]).diameter).toBe(2);
  expect(toWireTool(r.resolve(p.toolpath[1]))).toEqual(p.tool_provenance.bindings[1].tool);
  expect(r.diameter).toBe(1.8);
  p.toolpath.reverse();
  expect(programTools(p).diameter).toBe(1.8);
});
it.each(['missing', 'unknown', 'duplicate', 'contradictory', 'legacy', 'planned'])(
  'refuses %s provenance without op fallback',
  (failure) => {
    const p = program();
    if (failure === 'missing') delete p.toolpath[0].tool_ref;
    if (failure === 'unknown') p.toolpath[0].tool_ref = 99;
    if (failure === 'duplicate') p.tool_provenance.bindings.push(p.tool_provenance.bindings[0]);
    if (failure === 'contradictory') p.tool_provenance.bindings[0].tool.id = 999;
    if (failure === 'legacy') p.tool_provenance.revision = 0;
    if (failure === 'planned') p.tool_provenance.authority = 'planned';
    expect(() => programTools(p, true)).toThrow();
  },
);
it('synthetic op identity is irrelevant when its declared tool binding is present', () => {
  const p = program();
  p.toolpath[0].op_id = 999;
  expect(programTools(p, true).resolve(p.toolpath[0]).diameter).toBe(8);
});
