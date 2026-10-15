import { describe, expect, it } from 'vitest';
import { beginGenerationProbe, GenerationProbe } from './performance-probe';

describe('opt-in generation diagnostics', () => {
  it('does not activate outside an explicitly enabled browser', () => {
    expect(beginGenerationProbe('cps')).toBeUndefined();
  });
  it('rejects stale preview work and frames before current buffers exist', () => {
    let clock = 10;
    const events: Record<string, unknown>[] = [];
    const probe = new GenerationProbe(
      1,
      () => clock,
      (event) => events.push(event),
    );
    clock = 20;
    probe.mark('request_sent');
    probe.preview('wire_buffers_ready', 3);
    probe.bind(4);
    probe.preview('first_3d_frame', 4);
    probe.preview('wire_buffers_ready', 3);
    probe.preview('wire_buffers_ready', 4);
    clock = 25;
    probe.preview('first_3d_frame', 4);
    probe.preview('first_3d_frame', 4);
    expect(events.map((e) => e.stage)).toEqual([
      'request_sent',
      'state_applied',
      'wire_buffers_ready',
      'first_3d_frame',
    ]);
    expect(events.at(-1)).toMatchObject({ id: 1, version: 4, elapsed_ms: 15 });
  });
  it('marks the rendered relief only after current simulation and LOD', () => {
    const events: Record<string, unknown>[] = [];
    const probe = new GenerationProbe(
      2,
      () => 0,
      (event) => events.push(event),
    );
    probe.bind(5);
    probe.preview('first_3d_frame_after_simulation', 5);
    probe.preview('simulation_lod_ready', 4);
    probe.preview('first_3d_frame_after_simulation', 5);
    expect(events).toHaveLength(1);
    probe.preview('simulation_lod_ready', 5);
    probe.preview('first_3d_frame_after_simulation', 5);
    expect(events.at(-1)?.stage).toBe('first_3d_frame_after_simulation');
  });
});
