import { describe, expect, it } from 'vitest';
import source from '../../../../data/feeds-speeds/packs/sienci-longmill.v1.json';
import fixture from '../../../../docs/shopbot-modernization/feeds-speeds/FS3_5-software-manual-saved.ivac-project.json';
import {
  enabledPacks,
  legacyLocalPack,
  importedSource,
  loadedPack,
  packCollection,
  NORMALIZED_MATERIALS,
  type SourcePack,
} from './feeds-speeds-sources';
import { validateProjectFile } from './feeds-speeds-persistence';
import type { ProjectFile } from './project-types';
const pack = source as SourcePack;
describe('FS3.7 source packs and private project boundary', () => {
  it('keeps built-in and local sources distinct with explicit version references', () => {
    const local = structuredClone(pack);
    local.manifest.distribution = 'local_only';
    local.manifest.identity.id = 'local-own-data';
    expect(
      enabledPacks({ sienciEnabled: true, custom: [], local: [] }, [pack], [local]),
    ).toHaveLength(1);
    const rows = enabledPacks(
      {
        sienciEnabled: false,
        custom: [],
        local: [{ id: 'local-own-data', version: local.manifest.identity.revision, enabled: true }],
      },
      [pack],
      [local],
    );
    expect(rows).toHaveLength(1);
    expect(rows[0].pack?.manifest.distribution).toBe('local_only');
  });
  it('refuses duplicate enabled pack identities', () => {
    const local = structuredClone(pack);
    local.manifest.distribution = 'local_only';
    expect(() =>
      enabledPacks(
        {
          sienciEnabled: true,
          custom: [],
          local: [
            {
              id: pack.manifest.identity.id,
              version: pack.manifest.identity.revision,
              enabled: true,
            },
          ],
        },
        [pack],
        [local],
      ),
    ).toThrow();
  });
  it('retains legacy data and provenance during local-only migration', () => {
    const converted = legacyLocalPack(importedSource(JSON.stringify(pack.dataset)));
    expect(converted.manifest.distribution).toBe('local_only');
    expect(converted.manifest.redistributable).toBe(false);
    expect(converted.dataset).toEqual(pack.dataset);
    expect(converted.records).toHaveLength(6);
  });
  it('prepares one multi-pack request', () => {
    expect(JSON.parse(packCollection([loadedPack(pack)])).source_packs).toEqual([pack]);
  });
  it('uses stable vendor-independent material identities', () => {
    expect(
      NORMALIZED_MATERIALS.some(
        (m) => m.identity.id === 'material/plastic.pmma.cast' && m.identity.revision === '1',
      ),
    ).toBe(true);
    expect(NORMALIZED_MATERIALS.some((m) => m.identity.id.includes('sienci'))).toBe(false);
  });
  it('saves a guarded summary and opens without private source content', () => {
    const file = structuredClone(fixture) as ProjectFile;
    const applied = file.operations![0].appliedCutting!;
    delete applied.proposal;
    applied.knowledge = {
      policy: 'fs37-compatible-records-1',
      engine_version: 'fixture',
      selected_pack: { id: 'missing-local', revision: '1' },
      selected_record: 'own-trial',
      evidence: 'tested',
      local_only: true,
      compatible_sources: 1,
    };
    file.kind = 'ivac-project-knowledge';
    file.version = 4;
    expect(file.kind).toBe('ivac-project-knowledge');
    expect(file.version).toBe(4);
    const serialized = JSON.stringify(file);
    expect(serialized).not.toContain('source_packs');
    expect(serialized).not.toContain('profile_set');
    const restored = JSON.parse(serialized);
    expect(() => validateProjectFile(restored)).not.toThrow();
    expect(
      restored.operations[0].appliedCutting.slots.some(
        (s: { feed_mm_min: number }) => s.feed_mm_min === 1500,
      ),
    ).toBe(true);
    expect(() =>
      validateProjectFile({ ...restored, kind: 'ivac-project-cutting', version: 3 }),
    ).toThrow();
    restored.operations[0].appliedCutting.proposal = {
      provenance: {
        profile: { source: { notes: 'PRIVATE-DATA' } },
        dataset: { id: 'private', revision: '1' },
      },
      parameters: {},
    };
    expect(() => validateProjectFile(restored)).toThrow('Private source records');
  });
});
