import { isProgramOnlyOp } from './op_types';
import type { ProjectFile } from './project-types';
import { validateCuttingStructure } from './cutting-parameters';
import { validateProjectGeometry } from './project-geometry-validation';
/** Validate before changing any current document state. Null rates remain editable. */
export function validateProjectV9(value: ProjectFile): void {
  if (!value || value.kind !== 'ivac-project' || value.version !== 9)
    throw new Error(
      'This FabCAM requires project format v9. Versions 1–8 are unsupported; the original file was kept.',
    );
  if (
    !Array.isArray(value.imports) ||
    !Array.isArray(value.operations) ||
    !Array.isArray(value.tools) ||
    !value.machine
  )
    throw new Error('Incomplete v9 project');
  if (value.fsContext) throw new Error('Legacy source context is not supported in v9');
  validateProjectGeometry(value);
  const ids = new Set<number>();
  for (const t of value.tools) {
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
      op.appliedCutting ||
      op.feedRateOverride !== undefined ||
      op.plungeRateOverride !== undefined
    )
      throw new Error('Legacy cutting overrides are unsupported in v9');
    if (isProgramOnlyOp(op.kind)) continue;
    if (!ids.has(op.toolId)) throw new Error(`${op.name}: assigned tool is missing`);
    if (value.machine.mode === 'mill') {
      validateCuttingStructure(op);
      for (const r of op.cutting!.rates)
        if (!ids.has(r.tool_id)) throw new Error('Secondary tool is missing');
    } else if (op.cutting) throw new Error('Milling values on non-milling operation');
  }
}
