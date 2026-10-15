import { describe, it, expect, vi } from 'vitest';
import { defaultToolForMode } from '../state/tool_mode_defaults';
const io = vi.hoisted(() => ({
  text: '',
  cancel: false,
  write: vi.fn(),
  invoke: vi.fn(),
  setError: vi.fn(),
}));
vi.mock('../state/project.svelte', () => ({ project: { setError: io.setError } }));
vi.mock('../state/workspace.svelte', () => ({ workspace: {} }));
vi.mock('../state/confirm.svelte', () => ({ confirmStore: {} }));
vi.mock('../api/env', () => ({ isTauri: () => true, isAndroid: () => false }));
vi.mock('../api/http', () => ({ defaultClient: {} }));
vi.mock('../api/client', () => ({ tryParseStructuredError: () => null }));
vi.mock('../i18n', () => ({ t: (key: string) => key }));
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: async () => (io.cancel ? null : '/tools.json'),
  save: async () => (io.cancel ? null : '/tools.json'),
}));
vi.mock('@tauri-apps/plugin-fs', () => ({
  readTextFile: async () => io.text,
  writeTextFile: io.write,
}));
vi.mock('@tauri-apps/api/core', () => ({ invoke: io.invoke }));
import { importToolset, exportToolset } from './file_ops';

describe('UX6-B staged toolset I/O', () => {
  it('replace renumbers, add skips existing names, neither mutates current tools or invokes project/cache commands', async () => {
    const current = [{ ...defaultToolForMode('mill', 8), name: 'Existing' }];
    const original = structuredClone(current);
    io.text = JSON.stringify({
      kind: 'toolset',
      version: 1,
      payload: [
        { ...defaultToolForMode('mill', 42), name: 'Existing' },
        {
          ...defaultToolForMode('mill', 70),
          name: 'New V-bit',
          kind: 'v_bit',
          tipAngleDeg: 60,
          tipDiameter: 0.2,
        },
      ],
    });
    const replacement = await importToolset('replace', current);
    expect(replacement?.map((t) => t.id)).toEqual([1, 2]);
    expect(replacement?.[1].tipAngleDeg).toBe(60);
    const added = await importToolset('add', current);
    expect(added?.map((t) => t.id)).toEqual([8, 9]);
    expect(added?.[0]).toEqual(original[0]);
    expect(current).toEqual(original);
    expect(io.invoke).not.toHaveBeenCalled();
  });
  it('cancel/invalid import leave the draft unchanged; export writes the draft format', async () => {
    const current = [defaultToolForMode('mill', 5)];
    const original = structuredClone(current);
    io.cancel = true;
    expect(await importToolset('replace', current)).toBeNull();
    io.write.mockClear();
    await exportToolset(current);
    expect(io.write).not.toHaveBeenCalled();
    io.cancel = false;
    io.text = '{"kind":"wrong"}';
    expect(await importToolset('replace', current)).toBeNull();
    expect(io.setError).toHaveBeenCalled();
    await exportToolset(current);
    const saved = JSON.parse(io.write.mock.calls[0][1]);
    expect(saved.kind).toBe('toolset');
    expect(saved.payload).toEqual(current);
    expect(current).toEqual(original);
    expect(io.invoke).not.toHaveBeenCalled();
  });
});
