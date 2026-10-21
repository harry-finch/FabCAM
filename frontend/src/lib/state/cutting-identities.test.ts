import { describe, expect, it } from 'vitest';
import { withLocalIdentity, referenceIdentity } from './cutting-identities';
import { defaultProjectMachine, defaultProjectTools } from './project-defaults';
import { defaultToolForMode } from './tool_mode_defaults';
import { referenceContext } from './feeds-speeds';
import { parseWorkspace } from './workspace';
import { hasFeedsSpeedsData } from './feeds-speeds-persistence';
const reference = { id: 'official-model', revision: '1' };
describe('independent lab and pack identities', () => {
  it('creates independent stable identities without activating presets', () => {
    const a = defaultProjectMachine(),
      b = defaultProjectMachine();
    const tool = defaultToolForMode('mill', 1);
    expect(a.fsLocalIdentity).toBeTruthy();
    expect(a.fsLocalIdentity).not.toEqual(b.fsLocalIdentity);
    expect(defaultProjectTools()[0].fsLocalIdentity).toBeTruthy();
    expect(withLocalIdentity(a, 'machine')).toBe(a);
    const linked = { ...a, fsIdentity: reference };
    expect(withLocalIdentity(linked, 'machine').fsLocalIdentity).toEqual(a.fsLocalIdentity);
    const context = referenceContext(linked, [{ ...tool, fsIdentity: reference }]);
    expect(context.machine_identity).toEqual(reference);
    expect(context.machine_local_identity).toEqual(a.fsLocalIdentity);
    expect(context.tools[0]).toMatchObject({
      identity: reference,
      local_identity: tool.fsLocalIdentity,
    });
    expect(
      referenceContext({ ...linked, fsIdentity: null }, [tool]).machine_local_identity,
    ).toEqual(a.fsLocalIdentity);
    expect(hasFeedsSpeedsData({ machine: a })).toBe(true);
  });
  it('preserves legacy captures and explicit unlinking without reviving a reference', () => {
    const old = withLocalIdentity({ fsIdentity: reference }, 'machine');
    expect(old.fsLocalIdentity).toEqual(reference);
    expect(withLocalIdentity(JSON.parse(JSON.stringify(old)), 'machine')).toEqual(old);
    expect(
      withLocalIdentity({ fsIdentity: null }, 'machine', reference).fsLocalIdentity,
    ).not.toEqual(reference);
    expect(referenceIdentity({ id: 'lab-machine-legacy', revision: '1' })).toBeNull();
    expect(referenceIdentity(reference)).toEqual(reference);
    const physical = { id: 'physical-from-native-context', revision: '1' };
    expect(
      withLocalIdentity({ fsIdentity: reference }, 'machine', reference, physical).fsLocalIdentity,
    ).toEqual(physical);
  });
  it('does not inherit a previous project reference when selecting another machine or tool', () => {
    const machine = defaultProjectMachine(),
      tool = defaultToolForMode('mill', 1);
    const old = { machine_identity: reference, tools: [{ tool_id: 1, identity: reference }] };
    const fresh = referenceContext(machine, [tool], old);
    expect(fresh.machine_identity).toBeNull();
    expect(fresh.tools[0].identity).toEqual(tool.fsLocalIdentity);
    const migratedMachine = withLocalIdentity(machine, 'machine', reference);
    const migratedTool = withLocalIdentity(tool, 'tool', reference);
    const migrated = referenceContext(migratedMachine, [migratedTool], old);
    expect(migrated.machine_identity).toEqual(reference);
    expect(migrated.tools[0].identity).toEqual(reference);
    expect(migrated.machine_local_identity).toEqual(machine.fsLocalIdentity);
    expect(migrated.tools[0].local_identity).toEqual(tool.fsLocalIdentity);
  });
  it('persists both links through workspace reload without replacing IDs', () => {
    const machine = { ...defaultProjectMachine(), fsIdentity: reference };
    const tool = { ...defaultToolForMode('mill', 1), fsIdentity: reference };
    const workspace = parseWorkspace(
      JSON.stringify({
        workspace_schema_version: 1,
        machine_profiles: [{ id: 'mp-1', name: 'Lab', machine, tools: [tool] }],
        tool_inventory: [tool],
      }),
    );
    const reopened = parseWorkspace(JSON.stringify(workspace));
    expect(reopened.machine_profiles[0].machine).toEqual(machine);
    expect(reopened.machine_profiles[0].tools).toEqual([tool]);
    expect(reopened.tool_inventory).toEqual([tool]);
  });
});
