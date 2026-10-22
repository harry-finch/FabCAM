import type { Command } from './history';
import type { CommandTarget } from './commands';
import { updateOperationCommand } from './commands';
import type { OpEntry, MachineMode } from './op_types';
import type { ToolEntry } from './project-types';
import { stockTool, stockedCopy } from './tool_inventory';
import { effectiveModes, toolCompatibleWithAnyMode } from './tool_family';
import { expectedToolKinds, isToolKindAcceptable } from './op_tool_constraint';

export type ToolSlot = 'toolId' | 'finishToolId' | 'spotToolId';
export function toolFitsOperation(
  op: OpEntry,
  tool: ToolEntry,
  modes: readonly MachineMode[],
  slot: ToolSlot = 'toolId',
): boolean {
  return (
    expectedToolKinds(op.kind).length > 0 &&
    (slot === 'spotToolId'
      ? ['drill', 'endmill', 'v_bit', 'engraver', 'cone'].includes(tool.kind)
      : isToolKindAcceptable(op.kind, tool.kind)) &&
    toolCompatibleWithAnyMode(tool.kind, modes) &&
    Number.isFinite(tool.diameter) &&
    tool.diameter > 0 &&
    !(op.kind === 'planar_surface' && tool.whirl)
  );
}

/** Add to the active machine/project and assign in one undo step. No source match is inferred. */
export function selectOperationToolCommand(
  opId: number,
  slot: ToolSlot,
  tool: ToolEntry,
  origin: 'project' | 'inventory',
): Command {
  let addedId: number | undefined;
  let edit: Command | undefined;
  return {
    label: 'Select operation tool',
    apply(state) {
      const s = state as CommandTarget;
      const op = s.operations.find((o) => o.id === opId);
      if (
        !op ||
        !toolFitsOperation(op, tool, effectiveModes(s.machine), slot) ||
        (slot === 'finishToolId' && op.kind !== 'pocket' && op.kind !== 'drill') ||
        (slot === 'spotToolId' && (op.kind !== 'drill' || !op.spotFirst))
      )
        throw new Error('Tool is incompatible with this operation or machine');
      const chosen =
        origin === 'inventory'
          ? (stockedCopy(tool, s.tools) ?? stockTool(tool, s.tools)!)
          : s.tools.find((t) => t.id === tool.id);
      if (!chosen) throw new Error('Tool is no longer available');
      addedId = s.tools.some((t) => t.id === chosen.id) ? undefined : chosen.id;
      const patch: Partial<OpEntry> =
        slot === 'spotToolId' && op.kind === 'drill' && op.spotFirst
          ? { spotFirst: { ...op.spotFirst, spotToolId: chosen.id } }
          : { [slot]: slot === 'finishToolId' && chosen.id === op.toolId ? undefined : chosen.id };
      // updateOperationCommand rebinds actual v9 roles while preserving authored values.
      if (addedId !== undefined)
        s.tools = [...s.tools, JSON.parse(JSON.stringify(chosen)) as ToolEntry];
      edit = updateOperationCommand(opId, patch);
      edit.apply(s);
    },
    revert(state) {
      const s = state as CommandTarget;
      edit?.revert(s);
      if (addedId !== undefined) s.tools = s.tools.filter((t) => t.id !== addedId);
    },
  };
}
