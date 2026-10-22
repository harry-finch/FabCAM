import { afterEach, beforeEach, expect, it, vi } from 'vitest';
vi.mock('@tauri-apps/plugin-dialog', () => ({ open: vi.fn(), save: vi.fn() }));
vi.mock('@tauri-apps/plugin-fs', () => ({
  stat: vi.fn(),
  readTextFile: vi.fn(),
  writeTextFile: vi.fn(),
}));
vi.mock('../state/preset-collections', () => ({ validateCollection: vi.fn() }));
import { open, save } from '@tauri-apps/plugin-dialog';
import { stat, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { validateCollection, type PresetCollection } from '../state/preset-collections';
import { importCollectionFile, exportCollectionFile } from './preset-collection-files';
const collection: PresetCollection = {
  format: 'fabcam-preset-collection',
  version: 1,
  id: 'qa',
  name: 'QA',
  presets: [],
};
beforeEach(() => {
  vi.resetAllMocks();
  vi.stubGlobal('window', { __TAURI_INTERNALS__: {} });
  vi.mocked(validateCollection).mockResolvedValue(collection);
});
afterEach(() => vi.unstubAllGlobals());
it('cancelled file dialogs never read or write', async () => {
  vi.mocked(open).mockResolvedValue(null);
  vi.mocked(save).mockResolvedValue(null);
  expect(await importCollectionFile()).toBeNull();
  expect(await exportCollectionFile(collection)).toBe(false);
  expect(readTextFile).not.toHaveBeenCalled();
  expect(writeTextFile).not.toHaveBeenCalled();
});
it('oversized selected files are refused before reading and malformed collections never reach a writer', async () => {
  vi.mocked(open).mockResolvedValue('/selected/oversized.json');
  vi.mocked(stat).mockResolvedValue({ size: 2 * 1024 * 1024 + 1 } as Awaited<
    ReturnType<typeof stat>
  >);
  await expect(importCollectionFile()).rejects.toThrow('2 MiB');
  expect(readTextFile).not.toHaveBeenCalled();
  vi.mocked(validateCollection).mockRejectedValue(new Error('Malformed'));
  await expect(exportCollectionFile(collection)).rejects.toThrow('Malformed');
  expect(save).not.toHaveBeenCalled();
  expect(writeTextFile).not.toHaveBeenCalled();
});
it('export writes only the explicitly selected path and propagates filesystem scope refusal', async () => {
  vi.mocked(save).mockResolvedValue('/selected/qa.json');
  expect(await exportCollectionFile(collection)).toBe(true);
  expect(writeTextFile).toHaveBeenCalledWith(
    '/selected/qa.json',
    JSON.stringify(collection, null, 2),
  );
  vi.mocked(writeTextFile).mockRejectedValue(new Error('Scope denied'));
  await expect(exportCollectionFile(collection)).rejects.toThrow('Scope denied');
});
it('import validates exact selected bytes before returning an uncommitted collection', async () => {
  vi.mocked(open).mockResolvedValue('/selected/qa.json');
  vi.mocked(stat).mockResolvedValue({ size: 100 } as Awaited<ReturnType<typeof stat>>);
  vi.mocked(readTextFile).mockResolvedValue('selected bytes');
  expect(await importCollectionFile()).toEqual(collection);
  expect(validateCollection).toHaveBeenCalledWith('selected bytes');
  expect(writeTextFile).not.toHaveBeenCalled();
});

it('native stat or read refusal and malformed imports preserve storage', async () => {
  vi.mocked(open).mockResolvedValue('/selected/qa.json');
  vi.mocked(stat).mockRejectedValueOnce(new Error('Command denied'));
  await expect(importCollectionFile()).rejects.toThrow('Command denied');
  expect(readTextFile).not.toHaveBeenCalled();
  vi.mocked(stat).mockResolvedValue({ size: 100 } as Awaited<ReturnType<typeof stat>>);
  vi.mocked(readTextFile).mockRejectedValueOnce(new Error('Scope denied'));
  await expect(importCollectionFile()).rejects.toThrow('Scope denied');
  expect(validateCollection).not.toHaveBeenCalled();
  vi.mocked(readTextFile).mockResolvedValue('{broken');
  vi.mocked(validateCollection).mockRejectedValueOnce(new Error('Malformed'));
  await expect(importCollectionFile()).rejects.toThrow('Malformed');
  expect(writeTextFile).not.toHaveBeenCalled();
});
