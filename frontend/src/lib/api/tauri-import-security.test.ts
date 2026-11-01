import { expect, it, vi } from 'vitest';
const { invoke } = vi.hoisted(() => ({ invoke: vi.fn().mockResolvedValue({ segments: [] }) }));
vi.mock('@tauri-apps/api/core', () => ({ invoke }));
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }));
import { TauriWiacClient } from './tauri';
it('imports a browser File through bytes without granting a temporary filesystem path', async () => {
  const bytes = new Uint8Array([60, 115, 118, 103, 47, 62]);
  const file = { name: 'synthetic.svg', arrayBuffer: async () => bytes.buffer } as File;
  await new TauriWiacClient().importFile(file, 'svg', 1);
  expect(invoke).toHaveBeenCalledWith('import_bytes_cmd', {
    filename: 'synthetic.svg',
    bytes: Array.from(bytes),
    scale: 1,
  });
});

const { open } = vi.hoisted(() => ({ open: vi.fn() }));
vi.mock('@tauri-apps/plugin-dialog', () => ({ open }));
it('reacquires source permission through a dialog instead of granting a suggested path', async () => {
  invoke.mockRejectedValueOnce('Source is outside dialog-granted filesystem scope.');
  open.mockResolvedValueOnce('/synthetic/selected.svg');
  await new TauriWiacClient().importFromPath('/synthetic/suggestion.svg');
  expect(open).toHaveBeenCalledWith({
    defaultPath: '/synthetic/suggestion.svg',
    multiple: false,
    directory: false,
  });
  expect(invoke).toHaveBeenLastCalledWith('import_path', {
    path: '/synthetic/selected.svg',
    scale: null,
  });
});
