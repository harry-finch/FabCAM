import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  importedSource,
  loadConfiguredSources,
  normalizeSourceSettings,
} from './feeds-speeds-sources';
import { DEFAULT_SETTINGS, loadSettings, saveSettings } from './project-data.svelte';
import source from '../../../../data/feeds-speeds/sienci-router-reference.v1.json';
afterEach(() => vi.unstubAllGlobals());
describe('FS3.5 global recommendation source settings', () => {
  it('persists enabled sources and custom contents through the actual settings storage', () => {
    const memory = new Map<string, string>();
    vi.stubGlobal('window', {
      localStorage: {
        getItem: (k: string) => memory.get(k) ?? null,
        setItem: (k: string, v: string) => memory.set(k, v),
      },
    });
    const custom = importedSource(JSON.stringify(source));
    saveSettings({
      ...DEFAULT_SETTINGS,
      feedsSpeedsSources: { sienciEnabled: false, custom: [custom] },
    });
    const restored = loadSettings();
    expect(restored.feedsSpeedsSources).toEqual({ sienciEnabled: false, custom: [custom] });
  });
  it('disabled sources load no data and never calculate or apply operation values', async () => {
    const read = vi.fn(async () => JSON.stringify(source));
    expect(await loadConfiguredSources({ sienciEnabled: false, custom: [] }, read)).toEqual([]);
    expect(read).not.toHaveBeenCalled();
  });
  it('loads configured references and custom sources without rewriting their scope', async () => {
    const custom = importedSource(JSON.stringify(source));
    const rows = await loadConfiguredSources({ sienciEnabled: true, custom: [custom] }, async () =>
      JSON.stringify(source),
    );
    expect(rows).toHaveLength(2);
    expect(rows[0].label).toBe('Sienci');
    expect(JSON.parse(rows[1].contents)).toEqual(source);
  });
  it('keeps settings backward compatible and refuses malformed imports', () => {
    expect(normalizeSourceSettings(undefined)).toEqual({ sienciEnabled: true, custom: [] });
    expect(normalizeSourceSettings({ sienciEnabled: false, custom: [{}] })).toEqual({
      sienciEnabled: false,
      custom: [],
    });
    expect(() => importedSource('{}')).toThrow();
  });
});
