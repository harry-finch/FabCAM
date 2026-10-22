import { validateCollection, type PresetCollection } from '../state/preset-collections';
const native = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
const filters = [{ name: 'FabCAM tested preset collection', extensions: ['json'] }];
const maxBytes = 2 * 1024 * 1024;
export async function importCollectionFile(): Promise<PresetCollection | null> {
  let json: string | null;
  if (native()) {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const { stat, readTextFile } = await import('@tauri-apps/plugin-fs');
    const path = await open({ multiple: false, filters });
    if (typeof path !== 'string') return null;
    if ((await stat(path)).size > maxBytes) throw new Error('Collection exceeds 2 MiB');
    json = await readTextFile(path);
  } else {
    json = await new Promise<string | null>((resolve, reject) => {
      const input = document.createElement('input');
      input.type = 'file';
      input.accept = '.json';
      input.oncancel = () => resolve(null);
      input.onchange = async () => {
        const file = input.files?.[0];
        if (!file) {
          resolve(null);
          return;
        }
        if (file.size > maxBytes) {
          reject(new Error('Collection exceeds 2 MiB'));
          return;
        }
        try {
          resolve(await file.text());
        } catch (e) {
          reject(e);
        }
      };
      input.click();
    });
  }
  return json === null ? null : validateCollection(json);
}
export async function exportCollectionFile(collection: PresetCollection): Promise<boolean> {
  const valid = await validateCollection(JSON.stringify(collection));
  const json = JSON.stringify(valid, null, 2);
  const filename = `${valid.id}.fabcam-presets.json`;
  if (native()) {
    const { save } = await import('@tauri-apps/plugin-dialog');
    const { writeTextFile } = await import('@tauri-apps/plugin-fs');
    const path = await save({ defaultPath: filename, filters });
    if (!path) return false;
    await writeTextFile(path, json); // Native plugin enforces the explicit dialog-granted scope.
  } else {
    const url = URL.createObjectURL(new Blob([json], { type: 'application/json' }));
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }
  return true;
}
