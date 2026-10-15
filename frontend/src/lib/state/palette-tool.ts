import type { ToolEntry } from './project-types';
import type { OpEntry, MachineMode } from './op_types';
import { isProgramOnlyOp } from './op_types';
import { expectedToolKinds, isToolKindAcceptable } from './op_tool_constraint';
import { toolCompatibleWithAnyMode } from './tool_family';
import { operationRates } from './feeds-speeds';
export function assignmentProblem(
  op: OpEntry | undefined,
  tool: ToolEntry | undefined,
  modes: readonly MachineMode[],
):
  | 'palette.select_operation'
  | 'palette.stock_tool'
  | 'palette.incompatible'
  | 'palette.review_rates'
  | undefined {
  if (!op || isProgramOnlyOp(op.kind)) return 'palette.select_operation';
  if (!tool) return 'palette.stock_tool';
  if (
    !expectedToolKinds(op.kind).length ||
    !isToolKindAcceptable(op.kind, tool.kind) ||
    !toolCompatibleWithAnyMode(tool.kind, modes) ||
    !(tool.diameter > 0) ||
    !Number.isFinite(tool.diameter) ||
    (op.kind === 'planar_surface' && !!tool.whirl)
  )
    return 'palette.incompatible';
  if (
    op.appliedCutting &&
    (!operationRates(op, tool, 'rough') ||
      (op.kind === 'pocket' &&
        (!op.finishToolId || op.finishToolId === tool.id) &&
        !operationRates(op, tool, 'finish')))
  )
    return 'palette.review_rates';
  return undefined;
}
