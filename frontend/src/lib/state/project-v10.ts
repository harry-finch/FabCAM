import { validateToolMetadata } from './tool-metadata';
import { isProgramOnlyOp } from './op_types';
import type { ProjectFile } from './project-types';
import { validateCuttingStructure } from './cutting-parameters';
import { validateProjectGeometry } from './project-geometry-validation';
/** Validate before changing any current document state. Null rates remain editable. */
export function validateProjectV10(input: unknown): asserts input is ProjectFile {
  const value = input as ProjectFile;
  if (!value || value.kind !== 'ivac-project' || value.version !== 10)
    throw new Error(
      'This FabCAM requires project format v10. Open historical projects in the previous FabCAM version or recreate them manually; the original file was kept.',
    );
  if (
    !Array.isArray(value.imports) ||
    !Array.isArray(value.operations) ||
    !Array.isArray(value.tools) ||
    !value.machine
  )
    throw new Error('Incomplete v10 project');
  if ('fsContext' in value) throw new Error('Legacy source context is not supported in v10');
  validateProjectGeometry(value);
  const ids = new Set<number>();
  for (const t of value.tools) {
    validateToolMetadata(t);
    if (!Number.isInteger(t.id) || t.id <= 0 || ids.has(t.id))
      throw new Error('Invalid or duplicate tool ID');
    ids.add(t.id);
  }
  const ops = new Set<number>();
  for (const op of value.operations) {
    if (!Number.isInteger(op.id) || op.id <= 0 || ops.has(op.id))
      throw new Error('Invalid or duplicate operation ID');
    ops.add(op.id);
    if (
      'appliedCutting' in op ||
      'fsFullWidth' in op ||
      'appliedContextKey' in op ||
      op.feedRateOverride !== undefined ||
      op.plungeRateOverride !== undefined
    )
      throw new Error('Legacy cutting overrides are unsupported in v10');
    if (isProgramOnlyOp(op.kind)) continue;
    if (!ids.has(op.toolId)) throw new Error(`${op.name}: assigned tool is missing`);
    if (value.machine.mode === 'mill') {
      validateCuttingStructure(op);
      for (const r of op.stages
        ? [op.stages.main, ...(op.stages.finishing ? [op.stages.finishing.cutting] : [])]
        : op.cutting!.rates)
        if (!ids.has(r.tool_id)) throw new Error('Secondary tool is missing');
    } else if (op.cutting || op.stages || op.stagePreset)
      throw new Error('Milling values on non-milling operation');
  }
}
