import type { components } from '../api/generated';
import { presetCommand } from '../api/wasm';
export type PresetCollection = components['schemas']['PresetCollection'];
export type TestedPreset = components['schemas']['TestedPreset'];
export interface StoredCollection {
  json: string;
  token: string;
}
const native = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
const MAX_BYTES = 2 * 1024 * 1024;
export async function validateCollection(json: string): Promise<PresetCollection> {
  if (new TextEncoder().encode(json).length > MAX_BYTES)
    throw new Error('Collection exceeds 2 MiB');
  return (await presetCommand({ action: 'validate_collection', json })) as PresetCollection;
}
async function database(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open('ivac-preset-collections', 1);
    request.onupgradeneeded = () =>
      request.result.createObjectStore('collections', { keyPath: 'id' });
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}
async function digest(json: string): Promise<string> {
  return Array.from(
    new Uint8Array(await crypto.subtle.digest('SHA-256', new TextEncoder().encode(json))),
    (b) => b.toString(16).padStart(2, '0'),
  ).join('');
}
export async function listCollections(): Promise<StoredCollection[]> {
  if (native()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('list_preset_collections');
  }
  const db = await database();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('collections', 'readonly');
      const req = tx.objectStore('collections').getAll();
      req.onsuccess = () => resolve(req.result);
      req.onerror = () => reject(req.error);
    });
  } finally {
    db.close();
  }
}
export async function saveCollection(
  collection: PresetCollection,
  expected: string | null,
): Promise<StoredCollection> {
  const validated = await validateCollection(JSON.stringify(collection));
  const json = JSON.stringify(validated);
  if (native()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('save_preset_collection', { json, expected });
  }
  const row = { id: validated.id, json, token: await digest(json) };
  const db = await database();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('collections', 'readwrite');
      const store = tx.objectStore('collections');
      let error: Error | undefined;
      const req = store.get(validated.id);
      req.onsuccess = () => {
        if ((req.result?.token ?? null) !== expected) {
          error = new Error('Collection changed; reload before replacing');
          tx.abort();
          return;
        }
        const count = store.count();
        count.onsuccess = () => {
          if (!req.result && count.result >= 32) {
            error = new Error('Maximum 32 collections');
            tx.abort();
          } else store.put(row);
        };
      };
      tx.oncomplete = () => resolve(row);
      tx.onabort = () => reject(error ?? tx.error ?? new Error('Collection write aborted'));
      tx.onerror = () => reject(tx.error);
    });
  } finally {
    db.close();
  }
}
export async function deleteCollection(id: string, expected: string): Promise<void> {
  if (!/^[a-zA-Z0-9_-]{1,96}$/.test(id)) throw new Error('Invalid collection ID');
  if (native()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('delete_preset_collection', { id, expected });
  }
  const db = await database();
  try {
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction('collections', 'readwrite');
      const store = tx.objectStore('collections');
      const req = store.get(id);
      let conflict = false;
      req.onsuccess = () => {
        if (req.result?.token !== expected) {
          conflict = true;
          tx.abort();
        } else store.delete(id);
      };
      tx.oncomplete = () => resolve();
      tx.onabort = () =>
        reject(
          new Error(
            conflict ? 'Collection changed; reload before deleting' : 'Collection deletion aborted',
          ),
        );
    });
  } finally {
    db.close();
  }
}
