import type { ToolEntry } from './project-types';
import type { ToolKind, OpEntry } from './op_types';
import { isAutoToolName, suggestToolName } from './tool_naming';

/** A kind edit preserves authored hidden fields; it does not measure new dimensions. */
export function changeToolKind(tool: ToolEntry, kind: ToolKind): ToolEntry {
  const next = { ...tool, kind };
  if (isAutoToolName(tool)) next.name = suggestToolName(next);
  return next;
}

/** Presentation of the existing same-ID sync contract, not new association provenance. */
export function toolLibraryImpacts(
  inventory: readonly ToolEntry[],
  stocked: readonly ToolEntry[],
  operations: readonly Pick<OpEntry, 'toolId' | 'finishToolId' | 'stages' | 'name'>[],
) {
  return stocked.flatMap((tool) => {
    const replacement = inventory.find((t) => t.id === tool.id);
    if (!replacement || JSON.stringify(tool) === JSON.stringify(replacement)) return [];
    return [
      {
        id: tool.id,
        before: tool.name,
        after: replacement.name,
        kindBefore: tool.kind,
        kindAfter: replacement.kind,
        substrateBefore: tool.substrate,
        substrateAfter: replacement.substrate,
        coatingBefore: tool.coating,
        coatingAfter: replacement.coating,
        diameterBefore: tool.diameter,
        diameterAfter: replacement.diameter,
        operations: operations
          .filter(
            (o) =>
              o.toolId === tool.id ||
              o.finishToolId === tool.id ||
              o.stages?.main.tool_id === tool.id ||
              o.stages?.finishing?.cutting.tool_id === tool.id,
          )
          .map((o) => o.name),
      },
    ];
  });
}
