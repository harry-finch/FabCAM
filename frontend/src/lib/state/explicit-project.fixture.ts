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
export function explicitProjectFixture(source: unknown): ExplicitFixture {
  const p = structuredClone(source) as ProjectFile;
  p.kind = 'ivac-project';
  p.version = 9;
  delete (p as unknown as Record<string, unknown>).fsContext;
  p.imports ??= [];
  p.operations ??= [];
  p.tools ??= [];
  p.machine = {
    ...defaultProjectMachine(),
    ...p.machine,
    physicalId: p.machine?.physicalId ?? 'software-fixture-machine',
  };
  p.tools = p.tools.map((t) => ({
    ...t,
    physicalId: t.physicalId ?? `software-fixture-tool-${t.id}`,
  }));
  p.operations = p.operations.map((op) => {
    const next = { ...op };
    delete (next as unknown as Record<string, unknown>).appliedCutting;
    delete (next as unknown as Record<string, unknown>).fsFullWidth;
    delete (next as unknown as Record<string, unknown>).appliedContextKey;
    delete next.feedRateOverride;
    delete next.plungeRateOverride;
    if (!isProgramOnlyOp(next.kind) && p.machine?.mode === 'mill')
      next.cutting = seedCutting(next, p.tools!);
    return next;
  });
  return p as ExplicitFixture;
}
