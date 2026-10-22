// Test construction only. Historical files stay unchanged and are rejected by the product.
// Reuse their geometry in a separately authored v9 software fixture with creation-time seeds.
import { seedCutting } from './cutting-parameters';
import { isProgramOnlyOp } from './op_types';
import type { ProjectFile, MachineSettings, ToolEntry } from './project-types';
import type { OpEntry } from './op_types';
import { defaultProjectMachine } from './project-defaults';
type ExplicitFixture = ProjectFile & {
  machine: MachineSettings;
  tools: ToolEntry[];
  operations: OpEntry[];
} & Record<string, unknown>;
export function explicitProjectFixture(source: ProjectFile): ExplicitFixture {
  const p = structuredClone(source);
  p.kind = 'ivac-project';
  p.version = 9;
  delete p.fsContext;
  p.imports ??= [];
  p.operations ??= [];
  p.tools ??= [];
  p.machine = {
    ...defaultProjectMachine(),
    ...p.machine,
    physicalId:
      p.machine?.physicalId ?? p.machine?.fsLocalIdentity?.id ?? 'software-fixture-machine',
  };
  p.tools = p.tools.map((t) => ({
    ...t,
    physicalId: t.physicalId ?? t.fsLocalIdentity?.id ?? `software-fixture-tool-${t.id}`,
  }));
  p.operations = p.operations.map((op) => {
    const next = { ...op };
    delete next.appliedCutting;
    delete next.feedRateOverride;
    delete next.plungeRateOverride;
    if (!isProgramOnlyOp(next.kind) && p.machine?.mode === 'mill')
      next.cutting = seedCutting(next, p.tools!);
    return next;
  });
  return p as ExplicitFixture;
}
