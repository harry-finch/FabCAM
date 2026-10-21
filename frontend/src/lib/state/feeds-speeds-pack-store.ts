// Installation-local packs. Never use the HTTP client or include packs in project saves.
import { isLabPackId } from './lab-presets';
import { invoke } from '@tauri-apps/api/core';
import { WasmWiacClient } from '../api/wasm';
import { TauriWiacClient } from '../api/tauri';
import type { components } from '../api/generated';
type SourcePack = components['schemas']['FsSourcePack'];
const wasm = new WasmWiacClient();
export const MAX_PACK_BYTES = 2 * 1024 * 1024;
const native = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
export function cuttingKnowledgeClient() {
  return native() ? new TauriWiacClient() : wasm;
}
async function database(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open('ivac-cutting-source-packs', 1);
    req.onupgradeneeded = () => req.result.createObjectStore('packs');
    req.onsuccess = () => resolve(req.result);
    req.onerror = () => reject(req.error);
  });
}
export async function readLocalPacks(): Promise<string[]> {
  if (native()) return invoke<string[]>('list_source_packs');
  const db = await database();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('packs', 'readonly');
      const store = tx.objectStore('packs'),
        req = store.getAllKeys();
      req.onsuccess = () => {
        const keys = req.result.map(String);
        const heads = new Map<string, string>();
        for (const key of keys.sort()) {
          const id = key.split('@')[0];
          if (isLabPackId(id)) heads.set(id, key);
        }
        const selected = keys.filter(
          (key) => !isLabPackId(key.split('@')[0]) || heads.get(key.split('@')[0]) === key,
        );
        const rows: string[] = [];
        for (const key of selected) {
          const item = store.get(key);
          item.onsuccess = () => rows.push(item.result as string);
        }
        tx.oncomplete = () => resolve(rows);
        tx.onerror = () => reject(tx.error);
      };
      req.onerror = () => reject(req.error);
    });
  } finally {
    db.close();
  }
}
export async function importLocalPack(json: string): Promise<SourcePack> {
  if (new TextEncoder().encode(json).byteLength > MAX_PACK_BYTES)
    throw new Error('Source pack exceeds 2 MiB');
  if (native())
    return JSON.parse(await invoke<string>('import_source_pack', { json })) as SourcePack;
  const pack = await wasm.validateSourcePack(json);
  pack.manifest.distribution = 'local_only';
  const db = await database();
  try {
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction('packs', 'readwrite');
      const count = tx.objectStore('packs').getAllKeys();
      count.onsuccess = () => {
        if (
          new Set([...count.result.map((k) => String(k).split('@')[0]), pack.manifest.identity.id])
            .size > 32 ||
          count.result.length >= 512 ||
          (!isLabPackId(pack.manifest.identity.id) &&
            count.result.filter((k) => !isLabPackId(String(k).split('@')[0])).length >= 32)
        ) {
          tx.abort();
          reject(new Error('At most 512 installed source revisions supported'));
          return;
        }
        if (
          isLabPackId(pack.manifest.identity.id) &&
          count.result.some(
            (k) =>
              String(k).startsWith(`${pack.manifest.identity.id}@`) &&
              String(k).split('@')[1] >= pack.manifest.identity.revision,
          )
        ) {
          tx.abort();
          reject(
            new Error(
              'This or a newer preset pack revision is already installed; current knowledge was kept',
            ),
          );
          return;
        }
        tx.objectStore('packs').add(
          JSON.stringify(pack),
          `${pack.manifest.identity.id}@${pack.manifest.identity.revision}`,
        );
      };
      tx.oncomplete = () => resolve();
      tx.onerror = () =>
        reject(new Error('This pack version is already installed, or storage failed'));
      tx.onabort = () => reject(tx.error);
    });
  } finally {
    db.close();
  }
  return pack;
}
export async function validatePack(json: string): Promise<SourcePack> {
  return wasm.validateSourcePack(json);
}

export const captureLabPreset = (
  project: import('../api/generated').components['schemas']['Project'],
  opId: number,
) => wasm.captureLabPreset(project, opId);
// Optimistic update creates an immutable new revision in the existing local store.
export async function commitLabPack(
  pack: SourcePack,
  expectedRevision: string | null,
): Promise<SourcePack> {
  const json = JSON.stringify(pack);
  const checked = await validatePack(json);
  if (
    !isLabPackId(checked.manifest.identity.id) ||
    checked.manifest.distribution !== 'local_only' ||
    checked.manifest.visibility !== 'private'
  )
    throw new Error('Only private local presets can be authored');
  if (native())
    return JSON.parse(
      await invoke<string>('save_lab_pack', { json, expectedRevision }),
    ) as SourcePack;
  const db = await database();
  try {
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction('packs', 'readwrite'),
        store = tx.objectStore('packs'),
        req = store.getAllKeys();
      let reason: Error | undefined;
      req.onsuccess = () => {
        const rows = req.result.map(String);
        const versions = rows
          .filter((key) => key.startsWith(`${checked.manifest.identity.id}@`))
          .map((key) => key.split('@')[1])
          .sort();
        const head = versions.at(-1) ?? null;
        if (
          head !== expectedRevision ||
          (head !== null && checked.manifest.identity.revision <= head)
        ) {
          reason = new Error('Presets changed in another window; reopen and retry');
          tx.abort();
          return;
        }
        if (
          rows.length >= 512 ||
          (!head && new Set(rows.map((key) => key.split('@')[0])).size >= 32)
        ) {
          reason = new Error(
            'Local source storage is full; export a backup before managing old revisions',
          );
          tx.abort();
          return;
        }
        store.add(json, `${checked.manifest.identity.id}@${checked.manifest.identity.revision}`);
      };
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(reason ?? tx.error);
      tx.onabort = () => reject(reason ?? tx.error);
    });
  } finally {
    db.close();
  }
  return checked;
}

/** Remove an installation-local source and its entire history. Applied project values are separate. */
export async function removeLocalSource(id: string, expectedHead: string): Promise<void> {
  if (!/^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,99}$/.test(id)) throw new Error('Invalid source id');
  if (native()) return invoke<void>('remove_source_pack', { id, expectedHead });
  const db = await database();
  try {
    await new Promise<void>((resolve, reject) => {
      const tx = db.transaction('packs', 'readwrite'),
        store = tx.objectStore('packs'),
        req = store.getAllKeys();
      let reason: Error | undefined;
      req.onsuccess = () => {
        const keys = req.result
          .map(String)
          .filter((key) => key.startsWith(`${id}@`))
          .sort();
        if (keys.at(-1) !== `${id}@${expectedHead}`) {
          reason = new Error('Source changed in another window; refresh and retry');
          tx.abort();
          return;
        }
        for (const key of keys) store.delete(key);
      };
      tx.oncomplete = () => resolve();
      tx.onerror = tx.onabort = () => reject(reason ?? tx.error);
    });
  } finally {
    db.close();
  }
}
